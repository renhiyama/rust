use crate::spec::{
    FramePointer, SanitizerSet, StackProbeType, Target, TargetMetadata, TargetOptions, base,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "aarch64-rovelstars-linux-runixos".into(),
        metadata: TargetMetadata {
            description: Some("ARM64 RunixOS (RovelStars, Linux environment)".into()),
            tier: Some(2),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            features: "+v8a,+outline-atomics".into(),
            // The AAPCS64 expects non-leaf frame pointers (see the ARM ABI docs),
            // and AArch64 unwinding hits interesting bugs without them.
            frame_pointer: FramePointer::NonLeaf,
            mcount: "\u{1}_mcount".into(),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            supported_sanitizers: SanitizerSet::ADDRESS
                | SanitizerSet::CFI
                | SanitizerSet::KCFI
                | SanitizerSet::LEAK
                | SanitizerSet::MEMORY
                | SanitizerSet::MEMTAG
                | SanitizerSet::THREAD
                | SanitizerSet::HWADDRESS,
            supports_xray: true,
            ..base::linux_runixos::opts()
        },
    }
}
