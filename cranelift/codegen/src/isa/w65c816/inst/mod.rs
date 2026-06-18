use std::vec::Vec;

use crate::MachInst;

use crate::binemit::CodeOffset;
use crate::isa::w65c816::abi::W65C816MachineDeps;
pub use crate::isa::w65c816::lower::isle::generated_code::MInst as Inst;
use crate::machinst::MachInstLabelUse;

mod emit;

pub use emit::EmitInfo;

impl MachInst for Inst {
    type ABIMachineSpec = W65C816MachineDeps;
    type LabelUse = LabelUse;

    const TRAP_OPCODE: &'static [u8] = &[0xDB];

    fn get_operands(&mut self, collector: &mut impl crate::machinst::OperandVisitor) {
        todo!()
    }

    fn is_move(&self) -> Option<(crate::Writable<crate::Reg>, crate::Reg)> {
        todo!()
    }

    fn is_term(&self) -> crate::machinst::MachTerminator {
        todo!()
    }

    fn is_trap(&self) -> bool {
        todo!()
    }

    fn is_args(&self) -> bool {
        todo!()
    }

    fn call_type(&self) -> crate::machinst::CallType {
        todo!()
    }

    fn is_included_in_clobbers(&self) -> bool {
        todo!()
    }

    fn is_mem_access(&self) -> bool {
        todo!()
    }

    fn gen_move(
        to_reg: crate::Writable<crate::Reg>,
        from_reg: crate::Reg,
        ty: crate::ir::Type,
    ) -> Self {
        todo!()
    }

    fn gen_dummy_use(reg: crate::Reg) -> Self {
        todo!()
    }

    fn rc_for_type(
        ty: crate::ir::Type,
    ) -> crate::CodegenResult<(
        &'static [crate::machinst::RegClass],
        &'static [crate::ir::Type],
    )> {
        todo!()
    }

    fn canonical_type_for_rc(rc: crate::machinst::RegClass) -> crate::ir::Type {
        todo!()
    }

    fn gen_jump(target: crate::MachLabel) -> Self {
        todo!()
    }

    fn gen_nop(preferred_size: usize) -> Self {
        todo!()
    }

    fn gen_nop_units() -> Vec<Vec<u8>> {
        todo!()
    }

    fn worst_case_size() -> crate::binemit::CodeOffset {
        todo!()
    }

    fn worst_case_island_growth() -> crate::binemit::CodeOffset {
        todo!()
    }

    fn ref_type_regclass(_flags: &crate::settings::Flags) -> crate::machinst::RegClass {
        todo!()
    }

    fn is_safepoint(&self) -> bool {
        todo!()
    }

    fn function_alignment() -> crate::isa::FunctionAlignment {
        todo!()
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
        todo!()
    }

    fn generate_veneer(self, buffer: &mut [u8], veneer_offset: CodeOffset) -> (CodeOffset, Self) {
        todo!()
    }

    fn from_reloc(reloc: crate::binemit::Reloc, addend: crate::binemit::Addend) -> Option<Self> {
        todo!()
    }
}
