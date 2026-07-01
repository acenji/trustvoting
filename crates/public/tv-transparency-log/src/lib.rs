// SPDX-License-Identifier: Apache-2.0
// Copyright (c) Scale Campaign LLC and the TrustVoting contributors.
//! `tv-transparency-log` — Append-only Merkle log for EDC/SAC commitments and inclusion proofs.
//!
//! TrustVoting module (public zone). This is a scaffold stub; implementation
//! lands incrementally. See the repository README for the full module map.

/// Returns this module's stable identifier. Placeholder until the module is implemented.
pub fn module_id() -> &'static str {
    "tv-transparency-log"
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_id() {
        assert_eq!(super::module_id(), "tv-transparency-log");
    }
}
