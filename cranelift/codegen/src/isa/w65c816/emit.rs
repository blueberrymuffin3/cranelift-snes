//! Riscv64 ISA: binary code emission.

use crate::ir::{self, LibCall, TrapCode};
use cranelift_control::ControlPlane;

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
