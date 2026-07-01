# Security Policy

TrustVoting builds election infrastructure. We take security reports seriously and welcome coordinated disclosure from researchers, election officials, and the public.

## Reporting a vulnerability

**Please do not open a public issue for security reports.**

- Preferred: use **GitHub → Security → "Report a vulnerability"** (private advisory) on this repository.
- Or email **security@trustvoting.com** with details and, if possible, a proof of concept.

We aim to acknowledge reports within 3 business days and to keep you updated as we investigate.

## Scope

In scope: anything in this repository — the device, county, public-verification, and shared-format crates, the build/reproducibility tooling, and the specifications in `docs/`.

Out of scope for *code* reports (but still tell us if you find issues): operational key material, signing ceremonies, and certification processes are deliberately **not** in this repository by design.

## Coordinated disclosure

We ask for reasonable time to remediate before public disclosure. We are happy to credit reporters in the release notes and advisory unless you prefer to remain anonymous. Because verifiability is the whole point of this project, we will publish a clear write-up of any confirmed vulnerability and its fix.
