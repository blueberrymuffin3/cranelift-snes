use std::fmt;
use std::vec::Vec;

use crate::dominator_tree::DominatorTree;
use crate::ir::ArgumentExtension;
use crate::ir::Function;
use crate::isa::Builder as IsaBuilder;
use crate::isa::IsaFlagsHashKey;
use crate::isa::OwnedTargetIsa;
use crate::isa::TargetIsa;
use crate::isa::w65c816::settings as w65c816_settings;
use crate::machinst::CompiledCodeStencil;
use crate::machinst::SigSet;
use crate::machinst::VCode;
use crate::machinst::compile;
use crate::result::CodegenResult;
use crate::settings as shared_settings;
use cranelift_control::ControlPlane;
use target_lexicon::{Architecture, Triple};

use super::FunctionAlignment;

mod abi;
mod emit;
mod inst;
mod lower;
mod settings;

pub struct W65C816Backend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: w65c816_settings::Flags,
}

impl W65C816Backend {
    fn compile_vcode(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<(VCode<inst::Inst>, regalloc2::Output)> {
        // This performs lowering to VCode, register-allocates the code, computes
        // block layout and finalizes branches. The result is ready for binary emission.
        let emit_info = inst::EmitInfo::new(self.flags.clone(), self.isa_flags.clone());
        let sigs = SigSet::new::<abi::W65C816MachineDeps>(func, &self.flags)?;
        let abi = abi::W65C816Callee::new(func, self, &self.isa_flags, &sigs)?;
        let (vcode, regalloc) =
            compile::compile::<Self>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)?;

        Ok((vcode, regalloc))
    }
}

impl TargetIsa for W65C816Backend {
    fn name(&self) -> &'static str {
        "w65c816"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.isa_flags.hash_key())
    }

    fn dynamic_vector_bytes(&self, dynamic_ty: crate::ir::Type) -> u32 {
        todo!()
    }

    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let (vcode, regalloc_result) = self.compile_vcode(func, domtree, ctrl_plane)?;

        let emit_result = vcode.emit(&regalloc_result, want_disasm, &self.flags, ctrl_plane);
        let value_labels_ranges = emit_result.value_labels_ranges;
        let buffer = emit_result.buffer;

        if let Some(disasm) = emit_result.disasm.as_ref() {
            crate::trace!("disassembly:\n{}", disasm);
        }

        Ok(CompiledCodeStencil {
            buffer,
            vcode: emit_result.disasm,
            value_labels_ranges,
            bb_starts: emit_result.bb_offsets,
            bb_edges: emit_result.bb_edges,
        })
    }

    fn emit_unwind_info(
        &self,
        result: &crate::machinst::CompiledCode,
        kind: super::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        todo!()
    }

    fn text_section_builder(
        &self,
        num_labeled_funcs: usize,
    ) -> std::prelude::v1::Box<dyn crate::TextSectionBuilder> {
        todo!()
    }

    fn function_alignment(&self) -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            preferred: 1,
        }
    }

    fn page_size_align_log2(&self) -> u8 {
        todo!()
    }

    fn pretty_print_reg(&self, reg: crate::Reg, size: u8) -> std::string::String {
        todo!()
    }

    fn has_native_fma(&self) -> bool {
        false
    }

    fn has_round(&self) -> bool {
        false
    }

    fn has_blendv_lowering(&self, ty: crate::ir::Type) -> bool {
        todo!()
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
    }

    fn default_argument_extension(&self) -> ArgumentExtension {
        ArgumentExtension::None
    }
}

impl fmt::Display for W65C816Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MachBackend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

/// Create a new `isa::Builder`.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    match triple.architecture {
        Architecture::Unknown => {}
        _ => unreachable!(),
    }
    IsaBuilder {
        triple,
        setup: w65c816_settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: shared_settings::Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    let isa_flags = w65c816_settings::Flags::new(&shared_flags, builder);

    let backend = W65C816Backend {
        triple,
        flags: shared_flags,
        isa_flags,
    };
    Ok(backend.wrapped())
}
