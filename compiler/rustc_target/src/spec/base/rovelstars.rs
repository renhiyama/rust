use crate::spec::{Cc, LinkerFlavor, Lld, TargetOptions, base};

pub(crate) fn opts() -> TargetOptions {
    // RunixOS is a Linux-kernel-based OS with its own userland and filesystem hierarchy.
    // Inherit Linux base config but override OS identity.
    let mut base = TargetOptions {
        os: "runixos".into(),
        env: "gnu".into(),
        vendor: std::borrow::Cow::Borrowed("rovelstars"),
        ..base::linux::opts()
    };

    // When we're asked to use the `rust-lld` linker by default, set the appropriate lld-using
    // linker flavor, and self-contained linker component.
    if option_env!("CFG_DEFAULT_LINKER_SELF_CONTAINED_LLD_CC").is_some() {
        base.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::Yes);
        base.link_self_contained = crate::spec::LinkSelfContainedDefault::with_linker();
    }

    base
}
