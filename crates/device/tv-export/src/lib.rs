// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) Scale Campaign LLC and the TrustVoting contributors.
//! `tv-export` — Signed export-bundle creation and sealed-media write.
//!
//! TrustVoting module (device zone). This is a scaffold stub; implementation
//! lands incrementally. See the repository README for the full module map.

/// Returns this module's stable identifier. Placeholder until the module is implemented.
pub fn module_id() -> &'static str {
    "tv-export"
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_id() {
        assert_eq!(super::module_id(), "tv-export");
    }
}
