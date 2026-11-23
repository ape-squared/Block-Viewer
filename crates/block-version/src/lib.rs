//! One compiled version for the viewer, its logs and its network clients.
//! Like LLVersionInfo (indra/newview/llversioninfo.cpp), every consumer
//! reads the same build information. Block's release tag is authoritative.

/// Release tag without `v`, with commit distance/hash metadata between tags.
pub const fn version() -> &'static str {
    env!("BLOCK_COMPILED_VERSION")
}

/// Synthetic version binaries are permitted only in disposable update fixtures.
pub const fn update_test_build() -> bool {
    env!("BLOCK_UPDATE_TEST_BUILD").as_bytes()[0] == b'1'
}

/// Build revision for a diagnostic report, including builds at an exact tag.
pub const fn commit() -> &'static str {
    env!("BLOCK_BUILD_COMMIT")
}

/// HTTP clients use the same version as the interface.
pub const fn user_agent() -> &'static str {
    concat!("BlockViewer/", env!("BLOCK_COMPILED_VERSION"))
}

/// SL's login uses four numeric components, as LLVersionInfo::getVersion:
/// major.minor.patch.build. The build is the distance from the release tag.
pub const fn login_version() -> &'static str {
    env!("BLOCK_LOGIN_VERSION")
}
