use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let settings = SettingGroupBuilder::new("w65c816");

    TargetIsa::new("w65c816", settings.build())
}
