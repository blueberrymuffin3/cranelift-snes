use core::cell::OnceCell;
use std::prelude::v1::Vec;
use std::sync::Once;
use std::sync::OnceLock;

use regalloc2::MachineEnv;
use regalloc2::PRegSet;
use smallvec::SmallVec;
use smallvec::smallvec;

use crate::CodegenResult;
use crate::FrameLayout;
use crate::RealReg;
use crate::Reg;
use crate::Writable;
use crate::ir::AbiParam;
use crate::ir::Signature;
use crate::ir::Type;
use crate::isa::CallConv;
use crate::isa::w65c816::inst::AModeDirect;
use crate::isa::w65c816::inst::AModeLoad;
use crate::isa::w65c816::inst::AModeMem;
use crate::isa::w65c816::inst::AModeStore;
use crate::isa::w65c816::inst::Inst;
use crate::isa::w65c816::inst::W65C816FixedStackSlot;
use crate::isa::w65c816::inst::W65C816Reg;
use crate::isa::w65c816::inst::make_machine_env;
use crate::isa::w65c816::lower::isle::generated_code::StackOffset;
use crate::isa::w65c816::settings::Flags as W65C816Flags;
use crate::machinst::ABIArg;
use crate::machinst::ABIArgSlot;
use crate::machinst::ABIMachineSpec;
use crate::machinst::ArgsAccumulator;
use crate::machinst::ArgsOrRets;
use crate::machinst::Callee;
use crate::machinst::FunctionCalls;
use crate::machinst::IsaFlags;
use crate::machinst::RetPair;
use crate::machinst::SmallInstVec;
use crate::machinst::StackAMode;
use crate::opts::I16;
use crate::settings::Flags;

/// Support for the W65C816 ABI from the callee side (within a function body).
pub(crate) type W65C816Callee = Callee<W65C816MachineDeps>;

/// W65C816-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct W65C816MachineDeps;

const DEFAULT_CLOBBERS: PRegSet = PRegSet::empty()
    .with(W65C816Reg::X.p_reg())
    .with(W65C816Reg::Y.p_reg());

impl IsaFlags for W65C816Flags {}

impl ABIMachineSpec for W65C816MachineDeps {
    type I = Inst;

    type F = W65C816Flags;

    const STACK_ARG_RET_SIZE_LIMIT: u32 = 256;

    fn word_bits() -> u32 {
        16
    }

    fn stack_align(call_conv: CallConv) -> u32 {
        1
    }

    fn compute_arg_locs(
        call_conv: CallConv,
        flags: &Flags,
        params: &[AbiParam],
        args_or_rets: ArgsOrRets,
        add_ret_area_ptr: bool,
        mut args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        if add_ret_area_ptr {
            return Err(crate::CodegenError::Unsupported(
                "Return area pointer not supported".into(),
            ));
        }

        let mut arg_registers = Iterator::chain(
            W65C816Reg::index_regs().into_iter(),
            W65C816FixedStackSlot::fixed_stack_slots().into_iter(),
        );

        for param in params {
            match param.purpose {
                crate::ir::ArgumentPurpose::Normal => {
                    let Some(reg) = arg_registers.next() else {
                        return Err(crate::CodegenError::Unsupported(
                            "Too many arguments".into(),
                        ));
                    };
                    let slot = ABIArgSlot::Reg {
                        reg: reg.into(),
                        ty: I16,
                        extension: param.extension,
                    };
                    args.push(ABIArg::Slots {
                        slots: [slot].into(),
                        purpose: param.purpose,
                    });
                }
                _ => {
                    return Err(crate::CodegenError::Unsupported(
                        "Unsupported argument type".into(),
                    ));
                }
            }
        }

        Ok((0, None))
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Self::I {
        let offset = match mem {
            StackAMode::Slot(slot) => StackOffset::Slot {
                offset: slot.try_into().unwrap(),
            },
            _ => todo!("Stack arguments"),
        };
        Inst::Load {
            mem: AModeLoad::Mem {
                mem: AModeMem::Direct {
                    direct: AModeDirect::StackOffset { offset },
                },
            },
            reg: into_reg,
        }
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Self::I {
        let offset = match mem {
            StackAMode::Slot(slot) => StackOffset::Slot {
                offset: slot.try_into().unwrap(),
            },
            _ => todo!("Stack arguments"),
        };
        Inst::Store {
            mem: AModeStore::Mem {
                mem: AModeMem::Direct {
                    direct: AModeDirect::StackOffset { offset },
                },
            },
            reg: from_reg,
        }
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Self::I {
        Inst::Move {
            rd: to_reg,
            rs: from_reg,
        }
    }

    fn gen_extend(
        to_reg: Writable<Reg>,
        from_reg: Reg,
        is_signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Self::I {
        todo!()
    }

    fn gen_args(args: Vec<crate::machinst::ArgPair>) -> Self::I {
        Inst::Args { args }
    }

    fn gen_rets(rets: Vec<RetPair>) -> Self::I {
        Inst::Rets { rets }
    }

    fn gen_add_imm(
        call_conv: CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Self::I {
        todo!()
    }

    fn get_stacklimit_reg(call_conv: CallConv) -> Reg {
        todo!()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Self::I {
        todo!()
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Self::I {
        todo!()
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Self::I> {
        todo!()
    }

    fn compute_frame_layout(
        call_conv: CallConv,
        flags: &Flags,
        sig: &Signature,
        regs: &[Writable<RealReg>],
        function_calls: FunctionCalls,
        incoming_args_size: u32,
        tail_args_size: u32,
        stackslots_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> FrameLayout {
        assert_eq!(incoming_args_size, 0, "non-fixed stack args not supported");
        assert_eq!(outgoing_args_size, 0, "non-fixed stack args not supported");
        assert_eq!(tail_args_size, 0, "non-fixed stack args not supported");

        FrameLayout {
            word_bytes: 1,
            incoming_args_size,
            tail_args_size,
            setup_area_size: 0,
            clobber_size: 0, // TODO: Callee-saved regs
            fixed_frame_storage_size,
            stackslots_size,
            outgoing_args_size,
            clobbered_callee_saves: vec![],
            function_calls,
        }
    }

    fn gen_prologue_frame_setup(
        call_conv: CallConv,
        flags: &Flags,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        assert_eq!(frame_layout.stackslots_size, 0);
        smallvec![] // TODO: What do we need to do here?
    }

    fn gen_epilogue_frame_restore(
        call_conv: CallConv,
        flags: &Flags,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        assert_eq!(frame_layout.stackslots_size, 0);
        smallvec![] // TODO: What do we need to do here?
    }

    fn gen_return(
        call_conv: CallConv,
        isa_flags: &Self::F,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        smallvec![Inst::Ret]
    }

    fn gen_probestack(insts: &mut SmallInstVec<Self::I>, frame_size: u32) {
        todo!()
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        call_conv: CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        todo!()
    }

    fn gen_clobber_save(
        call_conv: CallConv,
        flags: &Flags,
        frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        smallvec![] // TODO: What do we need to do here?
    }

    fn gen_clobber_restore(
        call_conv: CallConv,
        flags: &Flags,
        frame_layout: &FrameLayout,
    ) -> smallvec::SmallVec<[Self::I; 16]> {
        smallvec![] // TODO: What do we need to do here?
    }

    fn gen_memcpy<F: FnMut(Type) -> Writable<Reg>>(
        call_conv: CallConv,
        dst: Reg,
        src: Reg,
        size: usize,
        alloc_tmp: F,
    ) -> smallvec::SmallVec<[Self::I; 8]> {
        todo!()
    }

    fn get_number_of_spillslots_for_value(
        rc: crate::machinst::RegClass,
        target_vector_bytes: u32,
        isa_flags: &Self::F,
    ) -> u32 {
        match rc {
            regalloc2::RegClass::Int => 1,
            _ => unreachable!(),
        }
    }

    fn get_machine_env(flags: &Flags, call_conv: CallConv) -> &MachineEnv {
        // FIXME: This is the best way I've found to do this
        // since you can't construct a vec in const code
        static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
        MACHINE_ENV.get_or_init(make_machine_env)
    }

    fn get_regs_clobbered_by_call(
        call_conv_of_callee: CallConv,
        is_exception: bool,
    ) -> regalloc2::PRegSet {
        DEFAULT_CLOBBERS
    }

    fn get_ext_mode(
        call_conv: CallConv,
        specified: crate::ir::ArgumentExtension,
    ) -> crate::ir::ArgumentExtension {
        specified // TODO: How do we want to handle 8-bit args?
    }

    fn retval_temp_reg(call_conv_of_callee: CallConv) -> Writable<Reg> {
        todo!()
    }
}
