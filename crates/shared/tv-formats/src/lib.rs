// SPDX-License-Identifier: Apache-2.0
// Copyright (c) Scale Campaign LLC and the TrustVoting contributors.
//! `tv-formats` — Open, versioned wire schemas/types (EDC, SAC, PVP, CVR, audit entries) shared by the core and any independent verifier.
//!
//! TrustVoting module (shared zone). This is a scaffold stub; implementation
//! lands incrementally. See the repository README for the full module map.

/// Returns this module's stable identifier. Placeholder until the module is implemented.
pub fn module_id() -> &'static str {
    "tv-formats"
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_id() {
        assert_eq!(super::module_id(), "tv-formats");
    }
}
