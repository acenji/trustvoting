# TrustVoting — Licensing & IP Protection Strategy

> **Status: committed decision.** This document records *why* TrustVoting is licensed the way it is,
> what each layer of protection does, and — importantly — what licensing does **not** protect, so the
> moat is built in the right place (certification, keys, trust), not assumed to come from the license.

## TL;DR

- **Core system (device + county software): AGPL-3.0** — strongest copyleft; forks must stay open and
  attributed.
- **Public verifier + transparency tools (`tv-verifier`, `tv-publisher`, `tv-transparency-log`):
  Apache-2.0** — permissive, with an explicit patent grant, so any party can freely build independent
  verification tools.
- **Trademark "TrustVoting" + logo** — separate registration; this is what stops a competitor from
  *being* us.
- **Root keys, Device CA, certification: never open-sourced** — this is the actual moat.

No license makes you "fully protected." Protection is a **bundle**: license + trademark + provenance +
keys/certification. The license is one of four pillars.

## Why AGPL-3.0 for the core

The entire TrustVoting value proposition is *"this machine verifiably runs the published, reviewed
code."* That only holds if no one can take the code, secretly modify the tabulator, and ship it
closed. AGPL-3.0 is the strongest available defense of that promise:

- **Cannot be closed.** Anyone who distributes *or runs* a modified version (including as a hosted
  service — the "A" in AGPL closes the SaaS loophole) **must publish their source**. A competitor
  cannot turn our open code into a proprietary product. The thing that would make them a differentiated
  vendor is exactly what AGPL strips away.
- **Cannot erase authorship.** AGPL requires preserving copyright notices and license headers.
  Removing "Copyright Scale Campaign LLC" is a license violation = copyright infringement, enforceable.
- **Forks are permanently downstream.** A fork always chases upstream. We set direction; they react.

Precedent: VotingWorks, Belenios (INRIA), and CHVote (Geneva) all use AGPL/GPL-family copyleft for the
same transparency reason. ElectionGuard (Microsoft) uses permissive MIT — but its goal is mass
adoption by *other* vendors, a different strategy than ours.

## Why Apache-2.0 (not MIT) for the verifier

We *want* the maximum number of independent parties — journalists, researchers, rival vendors,
election officials — to build tools that verify our published artifacts (EDC, SAC, PVP). Friction
there is bad. So the verification layer is permissive.

Apache-2.0 over MIT because Apache-2.0 includes an **explicit patent grant and retaliation clause**,
which protects both adopters and us; MIT is silent on patents.

## What licensing does NOT protect (and what does)

| Threat | Protected by | Is it the license? |
|--------|--------------|--------------------|
| Competitor closes/relicenses our code | **AGPL-3.0** | ✅ Yes |
| Competitor strips our name off the files | **AGPL-3.0** attribution clause | ✅ Yes |
| Competitor claims they invented it | **Public git history + transparency log** (timestamped, immutable provenance) | ❌ Provenance, not license |
| Competitor brands their fork "TrustVoting" | **Trademark registration** | ❌ Trademark, not license |
| Competitor's devices trusted as ours | **Vendor Root Key, Device CA, signing ceremony** | ❌ Keys, not license |
| Competitor wins on a code copy alone | **Certification + relationships + hardware + support** | ❌ Market moat, not license |

## The "copycat in a month" concern — addressed

A competitor *can* legally fork AGPL code. What they have after doing so:

- An **open, attributed, publicly-a-fork** version — they cannot close it or claim sole authorship.
- **No certification.** EAC + state-by-state certification takes 2–4 years and millions of dollars per
  system, and does not transfer to a fork.
- **No root of trust.** They can copy the code that *verifies* signatures; they cannot copy *our keys*.
  Every TrustVoting device trusts our root, not theirs.
- **No track record, relationships, hardware supply chain, or support org.**

Code is roughly 6 months of a multi-year, multi-million-dollar moat. And open-source-ness is itself the
sales pitch: closed competitors are distrusted *because* they are closed. Closing our code to deter
copycats would delete our core differentiation — so the license is load-bearing for the product, not a
liability.

Provenance note: open source is the **best** proof of authorship that exists. Our timestamped git
history and append-only transparency log prove who published what, when. Faking "we invented it" is
*easier* against secret code, not public code.

## Action checklist

- [ ] Add `LICENSE` (AGPL-3.0) at repo root for core modules.
- [ ] Add `LICENSE-APACHE` (Apache-2.0) scoped to `tv-verifier` / `tv-publisher` /
      `tv-transparency-log` directories, with a clear NOTICE on which dirs are permissive.
- [ ] Per-file SPDX headers (`SPDX-License-Identifier: AGPL-3.0-only` or `Apache-2.0`).
- [ ] Register "TrustVoting" word mark + logo (USPTO; consider EU/Madrid for international).
- [ ] Keep Vendor Root Key, Device CA, and signing-ceremony material out of the repo permanently
      (already required by the architecture's offline-HSM design).
- [ ] (Optional) Evaluate a defensive patent on the BAT protocol with a patent grant — weigh against
      the transparency ethos.
- [ ] Add a `CONTRIBUTING.md` with a DCO or CLA so inbound contributions are cleanly licensed.

## Open question

CLA vs DCO for contributors: a **CLA** (contributor license agreement) lets Scale Campaign relicense
later (e.g., offer a commercial license) but adds contributor friction; a **DCO** (developer
certificate of origin) is lighter-weight but locks us into AGPL. Decide before accepting external
contributions.
