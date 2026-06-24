use crate::isa::w65c816::W65C816Backend;
use crate::isa::w65c816::settings as w65c816_settings;
use crate::machinst::Reg;
use crate::machinst::{CallInfo, MachInst, isle::*};
use crate::machinst::{VCodeConstant, VCodeConstantData};
use crate::settings as shared_settings;
use crate::{
    ir::{
        AtomicRmwOp, BlockCall, ExternalName, Inst, InstructionData, MemFlags, Opcode, TrapCode,
        Value, ValueList, condcodes::*, immediates::*, types::*,
    },
    isa::w65c816::inst::*,
    machinst::{ArgPair, CallArgList, CallRetList, InstOutput},
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use regalloc2::PReg;

type VecArgPair = Vec<ArgPair>;

pub mod generated_code;

use generated_code::MInst;
use target_lexicon::triple;

pub(crate) struct W65C816IsleContext<'a, 'b, I, B>
where
    I: VCodeInst,
    B: LowerBackend,
{
    pub lower_ctx: &'a mut Lower<'b, I>,
    pub backend: &'a B,
}

impl<'a, 'b> W65C816IsleContext<'a, 'b, MInst, W65C816Backend> {
    fn new(lower_ctx: &'a mut Lower<'b, MInst>, backend: &'a W65C816Backend) -> Self {
        Self { lower_ctx, backend }
    }

    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.lower_ctx.f.dfg
    }
}

impl generated_code::Context for W65C816IsleContext<'_, '_, MInst, W65C816Backend> {
    isle_lower_prelude_methods!();

    #[inline]
    fn emit(&mut self, inst: &MInst) -> Unit {
        self.lower_ctx.emit(inst.clone());
    }

    // fn spillslot_mem_from_reg(&mut self, reg: Reg) -> Option<AModeMem> {
    //     let spillslot = reg.to_spillslot()?;
    //     let mem = AModeMem::Direct {
    //         direct: AModeDirect::StackOffset {
    //             offset: generated_code::StackOffset::Slot {
    //                 offset: spillslot.index().try_into().expect("Stack size overflow"),
    //             },
    //         },
    //     };
    //     Some(mem)
    // }
    //
    // #[inline]
    // fn reg_from_writable_reg(&mut self, writable: WritableReg) -> Reg {
    //     writable.to_reg()
    // }
}

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &W65C816Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = W65C816IsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

/// The main entry point for branch lowering with ISLE.
pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &W65C816Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = W65C816IsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}
