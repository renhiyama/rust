use crate::spec::{Cc, LinkerFlavor, Lld, TargetOptions, base, cvs};

pub(crate) fn opts() -> TargetOptions {
    // RunixOS is a Linux-kernel-based OS with its own userland and filesystem hierarchy.
    // Uses LLVM runtime stack: compiler-rt, libunwind, libc++.
    let mut base = TargetOptions {
        os: "runixos".into(),
        env: "gnu".into(),
        vendor: std::borrow::Cow::Borrowed("rovelstars"),
        ..base::linux::opts()
    };

    // RunixOS uses LLVM's libunwind instead of libgcc_s.
    // Add -lunwind to default link args so Rust programs link correctly.
    base.add_post_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-lunwind"]);
    base.add_post_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::Yes), &["-lunwind"]);

    // Default to using lld as the linker
    base.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::Yes);
    base.link_self_contained = crate::spec::LinkSelfContainedDefault::with_linker();

    // RunixOS uses .rdl for shared libraries
    base.dll_prefix = "lib".into();
    base.dll_suffix = ".rdl".into();
    base.staticlib_prefix = "lib".into();
    base.staticlib_suffix = ".ral".into();

    base
}
