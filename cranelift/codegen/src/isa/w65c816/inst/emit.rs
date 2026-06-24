use std::fmt::Display;
use std::string::String;
use std::string::ToString;

use cranelift_control::ControlPlane;
use itertools::Itertools;

use crate::FrameLayout;
use crate::MachBuffer;
use crate::MachInstEmit;
use crate::MachInstEmitState;
use crate::Reg;
use crate::Writable;
use crate::ir::UserStackMap;
use crate::isa::w65c816::abi::W65C816Callee;
use crate::isa::w65c816::inst::AModeDirect;
use crate::isa::w65c816::inst::AModeIndexed;
use crate::isa::w65c816::inst::AModeLoad;
use crate::isa::w65c816::inst::AModeMem;
use crate::isa::w65c816::inst::AModeStore;
use crate::isa::w65c816::inst::Inst;
use crate::isa::w65c816::inst::W65C816Reg;
use crate::isa::w65c816::lower::isle::generated_code::AluOp;
use crate::isa::w65c816::lower::isle::generated_code::StackOffset;
use crate::isa::w65c816::settings as w65c816_settings;
use crate::machinst::ArgPair;
use crate::machinst::RetPair;
use crate::settings as shared_settings;

pub struct EmitInfo {
    #[expect(dead_code, reason = "may want to be used in the future")]
    shared_flag: shared_settings::Flags,
    isa_flags: w65c816_settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(
        shared_flag: shared_settings::Flags,
        isa_flags: w65c816_settings::Flags,
    ) -> Self {
        Self {
            shared_flag,
            isa_flags,
        }
    }
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
    /// The user stack map for the upcoming instruction, as provided to
    /// `pre_safepoint()`.
    user_stack_map: Option<UserStackMap>,

    /// Only used during fuzz-testing. Otherwise, it is a zero-sized struct and
    /// optimized away at compiletime. See [cranelift_control].
    ctrl_plane: ControlPlane,

    frame_layout: FrameLayout,
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(abi: &W65C816Callee, ctrl_plane: ControlPlane) -> Self {
        EmitState {
            user_stack_map: None,
            ctrl_plane,
            frame_layout: abi.frame_layout().clone(),
        }
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<UserStackMap>) {
        self.user_stack_map = user_stack_map;
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        &mut self.ctrl_plane
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        self.ctrl_plane
    }

    fn frame_layout(&self) -> &crate::FrameLayout {
        &self.frame_layout
    }
}

impl AModeLoad {
    fn fixup_stack_vregs(&mut self) {
        let Self::VReg { vreg } = self else {
            return;
        };
        let Some(spillslot) = vreg.to_spillslot() else {
            return;
        };
        *self = Self::Mem {
            mem: AModeMem::Direct {
                direct: AModeDirect::StackOffset {
                    offset: StackOffset::Slot {
                        offset: spillslot.index().try_into().unwrap(),
                    },
                },
            },
        }
    }
}

impl AModeStore {
    fn fixup_stack_vregs(&mut self) {
        let Self::VReg { vreg } = self else {
            return;
        };
        let Some(spillslot) = vreg.to_reg().to_spillslot() else {
            return;
        };
        *self = Self::Mem {
            mem: AModeMem::Direct {
                direct: AModeDirect::StackOffset {
                    offset: StackOffset::Slot {
                        offset: spillslot.index().try_into().unwrap(),
                    },
                },
            },
        }
    }
}

impl Inst {
    pub(super) fn fixup_stack_vregs(&mut self) {
        match self {
            Inst::Store { mem, reg } => mem.fixup_stack_vregs(),
            Inst::Load { mem, reg } => mem.fixup_stack_vregs(),
            Inst::Alu { op, rd, rs1, rs2 } => rs2.fixup_stack_vregs(),
            _ => {}
        }
    }
}

impl AluOp {
    fn opcode_bits(self) -> u8 {
        match self {
            AluOp::Ora => 0x00,
            AluOp::And => 0x20,
            AluOp::Adc => 0x60,
            AluOp::Sbc => 0xE0,
        }
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, code: &mut MachBuffer<Self>, info: &Self::Info, state: &mut Self::State) {
        let mut inst = self.clone();
        inst.fixup_stack_vregs();
        match inst {
            Inst::DummyUse { reg } => {}
            Inst::Store { mem, reg } => match reg.to_real_reg().unwrap().into() {
                W65C816Reg::A => {
                    let AModeStore::Mem { mem } = mem else {
                        panic!("Stored to virtual register");
                    };
                    emit_mem_op(&mem, code, 0x90);
                }
                xy => {
                    // FIXME: Store directly from xy when possible
                    Inst::Move {
                        rd: Writable::from_reg(W65C816Reg::A.reg()),
                        rs: reg,
                    }
                    .emit(code, info, state);
                    Inst::Store {
                        mem,
                        reg: W65C816Reg::A.reg(),
                    }
                    .emit(code, info, state);
                }
            },
            Inst::Load { mem, reg } => match reg.to_reg().to_real_reg().unwrap().into() {
                W65C816Reg::A => match mem {
                    AModeLoad::Mem { mem } => emit_mem_op(&mem, code, 0xA0),
                    AModeLoad::Immediate { value } => todo!(),
                    AModeLoad::VReg { vreg: rs } => {
                        match rs.to_real_reg().expect("Load to virtual reg").into() {
                            W65C816Reg::A => {} // No Op
                            W65C816Reg::X | W65C816Reg::Y => {
                                Inst::Move { rd: reg, rs }.emit(code, info, state)
                            }
                        }
                    }
                },
                xy => {
                    // FIXME: Load directly to xy when possible
                    Inst::Move {
                        rd: Writable::from_reg(W65C816Reg::A.reg()),
                        rs: reg.to_reg(),
                    }
                    .emit(code, info, state);
                }
            },
            Inst::Move { rd, rs } => {
                let rd_real = rd.map(|reg| reg.to_real_reg().expect("Move to virual reg"));
                let rs_real = rs.to_real_reg().expect("Move from virtual reg");
                emit_move(rd_real, rs_real, code);
            }
            Inst::Nop => todo!(),
            Inst::Alu { op, rd, rs1, rs2 } => {
                match &rs2 {
                    AModeLoad::Mem { mem } => emit_mem_op(mem, code, op.opcode_bits()),
                    _ => todo!(),
                };
                Inst::Move {
                    rd,
                    rs: W65C816Reg::A.reg(),
                }
                .emit(code, info, state);
            }
            Inst::Bra { target } => todo!(),
            Inst::Args { args } => {
                for ArgPair { vreg, preg } in args {
                    assert_eq!(
                        vreg.to_reg(),
                        preg,
                        "Oopsie, I thought regalloc handled this"
                    )
                }
            }
            Inst::Rets { rets } => {
                for RetPair { vreg, preg } in rets {
                    assert_eq!(vreg, preg, "Oopsie, I thought regalloc handled this")
                }
            }
            Inst::Ret => code.put1(0x60),
        }
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.to_string()
    }
}

fn emit_move(
    rd: Writable<impl Into<W65C816Reg>>,
    rs: impl Into<W65C816Reg>,
    code: &mut MachBuffer<Inst>,
) {
    let opcode = match (rd.to_reg().into(), rs.into()) {
        (W65C816Reg::X, W65C816Reg::A) => 0xA9,
        (W65C816Reg::Y, W65C816Reg::A) => 0xA8,
        (W65C816Reg::A, W65C816Reg::X) => 0x8A,
        (W65C816Reg::Y, W65C816Reg::X) => 0x9B,
        (W65C816Reg::A, W65C816Reg::Y) => 0x98,
        (W65C816Reg::X, W65C816Reg::Y) => 0x9B,
        (_same1, _same2) => return,
    };
    code.put1(opcode);
}

fn emit_immediate_op(opcode: u8, immediate: u16, code: &mut MachBuffer<Inst>) {
    code.put1(opcode);
    code.put2(immediate);
}

fn emit_mem_op(mode: &AModeMem, code: &mut MachBuffer<Inst>, opcode_bits: u8) {
    let mode_bits = match mode {
        AModeMem::Direct {
            direct: AModeDirect::Absolute { .. },
        } => 0x0D,
        AModeMem::Direct {
            direct: AModeDirect::DirectPage { .. },
        } => 0x05,
        AModeMem::Direct {
            direct: AModeDirect::DirectPageIndirect { .. },
        } => 0x12,
        AModeMem::Direct {
            direct: AModeDirect::StackOffset { .. },
        } => 0x03,
        AModeMem::Indexed {
            base: AModeIndexed::AbsoluteIndexed { .. },
            ..
        } => todo!("varies for X and Y"),
        AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndexed { .. },
            ..
        } => 0x15,
        AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndexedIndirect { .. },
            ..
        } => 0x01,
        AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndirectIndexed { .. },
            ..
        } => 0x11,
        AModeMem::Indexed {
            base: AModeIndexed::StackOffsetIndirectIndexed { .. },
            ..
        } => 0x13,
    };
    assert_eq!(opcode_bits & mode_bits, 0);
    let opcode = mode_bits | opcode_bits;
    code.put1(opcode);

    match mode {
        AModeMem::Direct {
            direct: AModeDirect::Absolute { addr },
        }
        | AModeMem::Indexed {
            base: AModeIndexed::AbsoluteIndexed { addr },
            ..
        } => {
            code.put2(*addr);
        }

        AModeMem::Direct {
            direct: AModeDirect::DirectPage { addr },
        }
        | AModeMem::Direct {
            direct: AModeDirect::DirectPageIndirect { addr },
        }
        | AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndexed { addr },
            ..
        }
        | AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndexedIndirect { addr },
            ..
        }
        | AModeMem::Indexed {
            base: AModeIndexed::DirectPageIndirectIndexed { addr },
            ..
        } => {
            code.put1(*addr);
        }
        AModeMem::Direct {
            direct: AModeDirect::StackOffset { offset },
        }
        | AModeMem::Indexed {
            base: AModeIndexed::StackOffsetIndirectIndexed { offset },
            ..
        } => {
            code.put1(match offset {
                StackOffset::Arg { offset } => *offset,
                StackOffset::Slot { offset } => *offset,
            });
            log::warn!("Do Relocations");
        }
    };
}
