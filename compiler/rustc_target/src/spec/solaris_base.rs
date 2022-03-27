use crate::spec::TargetOptions;

use super::cvs;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "solaris".into(),
        dynamic_linking: true,
        executables: true,
        has_rpath: true,
        families: cvs!["unix"],
        is_like_solaris: true,
        linker_is_gnu: false,
        limit_rdylib_exports: false, // Linker doesn't support this
        eh_frame_header: false,

        ..Default::default()
    }
}
