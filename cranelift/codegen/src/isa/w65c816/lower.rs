use crate::MachLabel;
use crate::ir::Inst as IRInst;
use crate::isa::w65c816::W65C816Backend;
use crate::isa::w65c816::inst::Inst;
use crate::machinst::InstOutput;
use crate::machinst::Lower;
use crate::machinst::LowerBackend;

pub mod isle;

//=============================================================================
// Lowering-backend trait implementation.

impl LowerBackend for W65C816Backend {
    type MInst = Inst;

    fn lower(&self, ctx: &mut Lower<Inst>, ir_inst: IRInst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Inst>,
        ir_inst: IRInst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }
}
