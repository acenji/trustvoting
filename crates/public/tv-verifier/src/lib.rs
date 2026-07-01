// SPDX-License-Identifier: Apache-2.0
// Copyright (c) Scale Campaign LLC and the TrustVoting contributors.
//! `tv-verifier` — Public CLI/library to verify EDC, SAC, PVP, and device records — trust the math.
//!
//! TrustVoting module (public zone). This is a scaffold stub; implementation
//! lands incrementally. See the repository README for the full module map.

/// Returns this module's stable identifier. Placeholder until the module is implemented.
pub fn module_id() -> &'static str {
    "tv-verifier"
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_id() {
        assert_eq!(super::module_id(), "tv-verifier");
    }
}
