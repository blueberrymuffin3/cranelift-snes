use std::string::String;

use cranelift_control::ControlPlane;

use crate::FrameLayout;
use crate::MachInstEmit;
use crate::MachInstEmitState;
use crate::ir;
use crate::isa::w65c816::abi::W65C816Callee;
use crate::isa::w65c816::inst::Inst;
use crate::isa::w65c816::settings as w65c816_settings;
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
    user_stack_map: Option<ir::UserStackMap>,

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

    fn pre_safepoint(&mut self, user_stack_map: Option<crate::ir::UserStackMap>) {
        todo!()
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        todo!()
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        todo!()
    }

    fn frame_layout(&self) -> &crate::FrameLayout {
        todo!()
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, code: &mut crate::MachBuffer<Self>, info: &Self::Info, state: &mut Self::State) {
        todo!()
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        format!("{self:?}")
    }
}
