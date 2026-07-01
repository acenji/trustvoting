# TrustVoting

**Verifiable elections by design.** An open-source, air-gapped, paper-backed precinct voting system where anyone — voter, journalist, or party — can independently *check* the result instead of *trusting* it.

[![Core: AGPL-3.0](https://img.shields.io/badge/core-AGPL--3.0-blue.svg)](LICENSE) [![Verifier: Apache-2.0](https://img.shields.io/badge/verifier-Apache--2.0-green.svg)](LICENSE-APACHE) · [trustvoting.com](https://trustvoting.com) · [How it works](https://trustvoting.com/how-it-works)

---

The entire premise is simple: **replace trust with proof.** Paper ballots are the ground truth; every machine is offline and tamper-evident; every step is cryptographically signed; and the tools to verify the outcome are free and open for everyone. A voting machine you cannot inspect is a promise. A voting machine whose code, builds, and results anyone can check is *evidence*.

> This repository is the platform implementation. It is early — most modules are scaffolds today and land incrementally in the open, with public git history and (eventually) an append-only transparency log as timestamped proof of provenance.

## The promise, in three links

1. **Cast as intended** — you see and check your own printed paper ballot before it goes in the box.
2. **Recorded as cast** — the paper ballot itself is the record; the machine only reads and counts it.
3. **Counted as recorded** — every total is signed and can be re-checked against the paper by anyone.

## How the code is organized (and licensed)

The system spans four trust zones. Data crosses a zone boundary only on **signed, sealed, hand-carried media** — never over a network. The repository is a Rust [Cargo workspace](Cargo.toml), one crate per module, grouped by zone. It is **dual-licensed by directory**:

| Zone | Path | License | Why |
|------|------|---------|-----|
| **Core — device** | `crates/device/` | **AGPL-3.0** | The code that runs the election must stay open and attributed — forks (even hosted) must publish source. |
| **Core — county** | `crates/county/` | **AGPL-3.0** | Same guarantee for the back-end that aggregates and certifies. |
| **Public verification** | `crates/public/` | **Apache-2.0** | We *want* the widest possible independent adoption — permissive, with an explicit patent grant. |
| **Shared formats** | `crates/shared/tv-formats` | **Apache-2.0** | The open wire schemas any third party needs to build their own verifier. |

Every source file carries an `SPDX-License-Identifier` header; when in doubt it is authoritative. See [docs/licensing-strategy.md](docs/licensing-strategy.md) for the full rationale, and [`NOTICE`](NOTICE).

## The modules

### 🗳️ Device — inside the voting center · AGPL-3.0 · `crates/device/`
| # | Crate | Role |
|---|-------|------|
| 1 | `tv-boot` | Secure boot chain, TPM measurement, key unsealing. |
| 2 | `tv-os` | Immutable base OS integration, SELinux policy, namespace setup. |
| 3 | `tv-ballot-ui` | Touchscreen ballot interface (BMD); requires a valid BAT to start a session. |
| 4 | `tv-bat` | Ballot Activation Token verification: QR scan, HMAC-SHA-384, replay + rate checks. |
| 5 | `tv-pollbook` | Voter check-in, roster, BAT generation + signing, slip printing. |
| 6 | `tv-printer` | Ballot rendering and thermal print / paper-path control. |
| 7 | `tv-scanner` | Optical scan pipeline: image capture and mark interpretation. |
| 8 | `tv-tabulate` | Tabulation engine: contest rules and Cast Vote Record (CVR) creation. |
| 9 | `tv-audit-log` | Append-only, hash-chained audit log with signed checkpoints. |
| 10 | `tv-export` | Signed export-bundle creation and sealed-media write. |
| 11 | `tv-admin` | Admin console: open/close polls, device status, export trigger. |
| 12 | `tv-update` | Offline update verifier (maintenance mode only). |
| 13 | `tv-tamper` | Hardware security-controller interface and irreversible quarantine logic. |

### 🏛️ County HQ — offline back-end · AGPL-3.0 · `crates/county/`
| # | Crate | Role |
|---|-------|------|
| 14 | `tv-ems` | Election definition authoring, ballot design, precinct management. |
| 15 | `tv-def-sign` | Definition test suite, DTR generation, EDC signing. |
| 16 | `tv-aggregator` | Import + verify precinct bundles, aggregate totals, build canvass package. |
| 17 | `tv-ops-dashboard` | Real-time precinct status, alerts, logistics, anomaly detection (advisory). |
| 18 | `tv-update-station` | Install signed updates to devices (maintenance ceremony). |

### 🌍 Public verification — anyone, anywhere · Apache-2.0 · `crates/public/`
| # | Crate | Role |
|---|-------|------|
| 19 | `tv-verifier` | Public CLI/library to verify EDC, SAC, PVP, and device records. |
| 20 | `tv-transparency-log` | Append-only Merkle log for EDC/SAC commitments and inclusion proofs. |
| 21 | `tv-publisher` | Publish verification artifacts to trustvoting.com (one-way; no control channel). |

### 🔗 Shared — `crates/shared/`
`tv-formats` (Apache-2.0) — the open, versioned wire schemas (EDC, SAC, PVP, CVR, audit entries) shared by the core and by any independent verifier. Kept permissive and dependency-light on purpose, so anyone can build their own checking tools without touching AGPL code.

> **EDC** = Election Definition Certificate · **SAC** = Software Assurance Certificate · **PVP** = Public Verification Package · **CVR** = Cast Vote Record · **BAT** = Ballot Activation Token. Full system design in [docs/architecture.md](docs/architecture.md).

## Build

```bash
cargo build            # build the whole workspace
cargo test             # run all crate tests
cargo build -p tv-verifier   # build just the public verifier
```

Requires stable Rust (see [`rust-toolchain.toml`](rust-toolchain.toml)).

## What licensing does — and does not — protect

Open source is the sales pitch, not a liability: closed voting systems are distrusted *because* they are closed. The license is one of four pillars of protection — the others are **trademark** (stops someone *being* TrustVoting), **provenance** (public git history + transparency log prove who published what, when), and **keys/certification** (the Vendor Root Key, Device CA, and signing ceremonies are the real moat and are **never** in this repo). Details in [docs/licensing-strategy.md](docs/licensing-strategy.md).

## Contributing & security

- Read [CONTRIBUTING.md](CONTRIBUTING.md) — contributions to `crates/device` and `crates/county` are under AGPL-3.0; contributions to `crates/public` and `crates/shared` under Apache-2.0.
- Report vulnerabilities privately per [SECURITY.md](SECURITY.md). Please do **not** open public issues for security reports.

## License

Dual-licensed: **AGPL-3.0-only** for the core ([LICENSE](LICENSE)) and **Apache-2.0** for the public verification layer and shared formats ([LICENSE-APACHE](LICENSE-APACHE)). "TrustVoting" and the logo are trademarks of Scale Campaign LLC and are not licensed for use under either license.
