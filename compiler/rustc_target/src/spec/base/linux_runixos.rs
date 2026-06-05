use crate::spec::{Cc, LinkerFlavor, Lld, LinkSelfContainedDefault, TargetOptions, add_link_args, base};

pub(crate) fn opts() -> TargetOptions {
    // RunixOS is a glibc-based Linux userland with its own filesystem hierarchy.
    // Its libc ABI is glibc, so the Rust environment is "gnu": that reuses every
    // glibc code path in std and libc at no cost. RunixOS identity comes from the
    // rovelstars vendor and the runixos triple, not the environment. The RunixOS
    // specifics are the LLVM runtime stack (compiler-rt, libunwind, libc++) and
    // the .rdl/.ral library naming; the toolchain (our clang/lld) supplies the
    // /Core paths and the ld-runixos interpreter.
    let mut base = TargetOptions {
        vendor: "rovelstars".into(),
        // Always link with lld and let the toolchain supply its runtime objects.
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
        link_self_contained: LinkSelfContainedDefault::with_linker(),
        // RunixOS library naming: .rdl shared, .ral static.
        dll_prefix: "lib".into(),
        dll_suffix: ".rdl".into(),
        staticlib_prefix: "lib".into(),
        staticlib_suffix: ".ral".into(),
        ..base::linux_gnu::opts()
    };

    // RunixOS unwinds with LLVM's libunwind, not libgcc_s. Adding to Lld::No
    // also populates the Lld::Yes flavor.
    add_link_args(&mut base.post_link_args, LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-lunwind"]);

    base
}
