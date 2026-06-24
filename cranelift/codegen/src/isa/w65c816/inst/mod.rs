use std::vec::Vec;

use regalloc2::OperandKind;
use regalloc2::OperandPos;

use crate::MachInst;
use crate::MachLabel;
use crate::Reg;
use crate::Writable;
use crate::binemit::CodeOffset;
use crate::ir::Type;
use crate::isa::FunctionAlignment;
use crate::isa::w65c816::abi::W65C816MachineDeps;
pub use crate::isa::w65c816::lower::isle::generated_code::AModeDirect;
pub use crate::isa::w65c816::lower::isle::generated_code::AModeIndexed;
pub use crate::isa::w65c816::lower::isle::generated_code::AModeLoad;
pub use crate::isa::w65c816::lower::isle::generated_code::AModeMem;
pub use crate::isa::w65c816::lower::isle::generated_code::AModeStore;
pub use crate::isa::w65c816::lower::isle::generated_code::MInst as Inst;
use crate::machinst::ArgPair;
use crate::machinst::CallType;
use crate::machinst::MachInstLabelUse;
use crate::machinst::MachTerminator;
use crate::machinst::OperandVisitorImpl;
use crate::machinst::RegClass;
use crate::opts::I16;
use crate::settings::Flags;

mod args;
mod emit;
mod fmt;
mod regs;

pub use emit::EmitInfo;
pub use regs::W65C816FixedStackSlot;
pub use regs::W65C816Reg;
pub use regs::make_machine_env;

impl MachInst for Inst {
    type ABIMachineSpec = W65C816MachineDeps;
    type LabelUse = LabelUse;

    const TRAP_OPCODE: &'static [u8] = &[0xDB];

    fn get_operands(&mut self, collector: &mut impl crate::machinst::OperandVisitor) {
        match self {
            Inst::DummyUse { reg } => {
                collector.reg_use(reg);
            }
            Inst::Move { rd, rs } => {
                collector.reg_use(rs);
                collector.reg_def(rd);
            }
            Inst::Store { mem, reg } => {
                // TODO: Clobber A when needed
                collector.reg_use(reg);
            }
            Inst::Load { mem, reg } => {
                // TODO: Clobber A when needed?
                collector.reg_def(reg);
            }
            Inst::Nop => {}
            Inst::Alu { rd, rs1, rs2, .. } => {
                collector.add_operand(
                    rs1,
                    regalloc2::OperandConstraint::Any,
                    OperandKind::Use,
                    OperandPos::Early,
                );
                if let AModeLoad::VReg { vreg } = rs2 {
                    collector.add_operand(
                        vreg,
                        regalloc2::OperandConstraint::Stack,
                        OperandKind::Use,
                        OperandPos::Late,
                    );
                }
                collector.any_def(rd);
            }
            Inst::Args { args } => {
                for ArgPair { vreg, preg } in args {
                    collector.reg_fixed_def(vreg, *preg);
                }
            }
            Inst::Rets { rets } => {
                for crate::machinst::RetPair { vreg, preg } in rets {
                    collector.reg_fixed_use(vreg, *preg);
                }
            }
            Inst::Bra { .. } => {}
            Inst::Ret { .. } => {}
        }
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        None
    }

    fn is_term(&self) -> MachTerminator {
        match self {
            Inst::Bra { .. } => MachTerminator::Branch,
            Inst::Rets { .. } => MachTerminator::Ret,
            _ => MachTerminator::None,
        }
    }

    fn is_trap(&self) -> bool {
        false
    }

    fn is_args(&self) -> bool {
        matches!(self, Inst::Args { .. })
    }

    fn call_type(&self) -> CallType {
        CallType::None
    }

    fn is_included_in_clobbers(&self) -> bool {
        match self {
            Inst::Args { .. } => false,
            _ => true,
        }
    }

    fn is_mem_access(&self) -> bool {
        false
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Self {
        Inst::Move {
            rd: to_reg,
            rs: from_reg,
        }
    }

    fn gen_dummy_use(reg: Reg) -> Self {
        Inst::DummyUse { reg }
    }

    fn rc_for_type(ty: Type) -> crate::CodegenResult<(&'static [RegClass], &'static [Type])> {
        if ty.bytes() > 2 {
            return Err(crate::CodegenError::Unsupported(
                "Only 8 and 16 bit values supported".into(),
            ));
        }
        Ok((&[RegClass::Int], &[I16]))
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        I16
    }

    fn gen_jump(target: MachLabel) -> Self {
        Inst::Bra { target }
    }

    fn gen_nop(preferred_size: usize) -> Self {
        Inst::Nop
    }

    fn gen_nop_units() -> Vec<Vec<u8>> {
        vec![vec![0xEA]]
    }

    fn worst_case_size() -> CodeOffset {
        4
    }

    fn ref_type_regclass(_flags: &Flags) -> RegClass {
        RegClass::Int
    }

    fn is_safepoint(&self) -> bool {
        false // TODO: True for calls
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            preferred: 1,
        }
    }
}

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {}

impl MachInstLabelUse for LabelUse {
    const ALIGN: CodeOffset = 1;

    fn max_pos_range(self) -> CodeOffset {
        todo!()
    }

    fn max_neg_range(self) -> CodeOffset {
        todo!()
    }

    fn patch_size(self) -> CodeOffset {
        todo!()
    }

    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        todo!()
    }

    fn supports_veneer(self) -> bool {
        todo!()
    }

    fn veneer_size(self) -> CodeOffset {
        todo!()
    }

    fn worst_case_veneer_size() -> CodeOffset {
        3 // BRL $abcd
    }

    fn generate_veneer(self, buffer: &mut [u8], veneer_offset: CodeOffset) -> (CodeOffset, Self) {
        todo!()
    }

    fn from_reloc(reloc: crate::binemit::Reloc, addend: crate::binemit::Addend) -> Option<Self> {
        todo!()
    }
}
