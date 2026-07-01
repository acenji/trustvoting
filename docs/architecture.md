
# TrustVoting — Software Architecture













---

## System Map

Five trust zones. Data crosses zone boundaries only on signed, controlled media.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  VENDOR HQ                                                                  │
│  ─────────                                                                  │
│                                                                             │
│  ┌──────────────────┐    ┌──────────────────────┐    ┌───────────────────┐  │
│  │  Secure Build CI  │───▶│  Release Signing      │───▶│  Artifact Vault   │  │
│  │  (reproducible)   │    │  Station (offline)     │    │  (SAC + bundles)  │  │
│  │                   │    │                        │    │                   │  │
│  │  • Yocto builds   │    │  • 2-of-3 key holders  │    │  • Signed images  │  │
│  │  • SBOM gen       │    │  • QA attestation      │    │  • SAC certs      │  │
│  │  • Test suite     │    │  • Security attestation │    │  • SBOM archive   │  │
│  │  • Security scans │    │  • SAC creation         │    │                   │  │
│  └──────────────────┘    └──────────────────────┘    └────────┬──────────┘  │
│                                                                │             │
│  ┌──────────────────┐                                          │             │
│  │  Support &        │    Signed update media                  │             │
│  │  Incident Response│◀── (exported to counties) ◀─────────────┘             │
│  │  Platform         │                                                       │
│  └──────────────────┘                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
         │
         │  Signed update bundles + SAC (on controlled media)
         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  COUNTY HQ  (offline network or standalone workstations)                    │
│  ──────────                                                                 │
│                                                                             │
│  ┌──────────────────┐    ┌──────────────────────┐    ┌───────────────────┐  │
│  │  EMS Core         │───▶│  Definition Signing   │───▶│  Update Station   │  │
│  │                   │    │  Station (offline)     │    │                   │  │
│  │  • Ballot design  │    │                        │    │  • Installs SAC-  │  │
│  │  • Contest setup  │    │  • Definition tests    │    │    validated      │  │
│  │  • Precinct       │    │  • DTR generation      │    │    images         │  │
│  │    assignment     │    │  • EDC signing          │    │  • Two-person     │  │
│  │  • Tabulation     │    │  • Dual control         │    │    rule           │  │
│  │    rules          │    │    (2 officials sign)   │    │  • Logs update    │  │
│  └──────────────────┘    └──────────────────────┘    └───────────────────┘  │
│                                                                             │
│  ┌──────────────────┐    ┌──────────────────────┐                           │
│  │  Results          │    │  County Ops Dashboard │                          │
│  │  Aggregation      │    │                        │                         │
│  │                   │    │  • Precinct status map  │                         │
│  │  • Import signed  │    │  • Alert feed           │                         │
│  │    export bundles │    │  • Queue estimates      │                         │
│  │  • Verify sigs    │    │  • Incident tracking    │                         │
│  │  • Verify EDC/SAC │    │  • AI anomaly detection │                         │
│  │  • Aggregate      │    │  • Spare logistics      │                         │
│  │    totals         │    │                        │                          │
│  │  • Generate       │    │  (optional controlled   │                         │
│  │    canvass pkg    │    │   internet for public   │                         │
│  └──────┬───────────┘    │   ops feed only)        │                         │
│         │                └──────────────────────┘                            │
│         │                                                                    │
└─────────┼────────────────────────────────────────────────────────────────────┘
          │                        │
          │  Signed results +      │  Signed election definition +
          │  PVP package           │  EDC (on controlled media)
          │                        │
          │                        ▼
          │  ┌─────────────────────────────────────────────────────────────────┐
          │  │                                                                 │
          │  │  PRECINCT / POLLING PLACE  (fully air-gapped — NO network)      │
          │  │  ────────────────────────                                       │
          │  │                                                                 │
          │  │  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐   │
          │  │  │  BMD #1     │ │  BMD #2     │ │  BMD #3     │ │  BMD #4     │  │
          │  │  │             │ │             │ │             │ │             │  │
          │  │  │ • Ballot UI │ │ • Ballot UI │ │ • Ballot UI │ │ • Ballot UI │  │
          │  │  │ • Printer   │ │ • Printer   │ │ • Printer   │ │ • Printer   │  │
          │  │  │ • Tamper    │ │ • Tamper    │ │ • Tamper    │ │ • Tamper    │  │
          │  │  │   sensors   │ │   sensors   │ │   sensors   │ │   sensors   │  │
          │  │  │ • Audit log │ │ • Audit log │ │ • Audit log │ │ • Audit log │  │
          │  │  └──────┬─────┘ └──────┬─────┘ └──────┬─────┘ └──────┬─────┘   │
          │  │         │              │              │              │          │
          │  │         │     prints paper ballots    │              │          │
          │  │         ▼              ▼              ▼              ▼          │
          │  │  ┌──────────────────────────────────────────────────────────┐   │
          │  │  │                    BALLOT BOX                            │   │
          │  │  │               (paper = ground truth)                     │   │
          │  │  └────────────────────────┬────────────────────────────────┘   │
          │  │                           │                                    │
          │  │                    voter feeds paper ballot                    │
          │  │                           │                                    │
          │  │                           ▼                                    │
          │  │  ┌──────────────────────────────────────────┐                  │
          │  │  │  SCANNER / TABULATOR (main)               │                 │
          │  │  │                                            │                │
          │  │  │  • Scans paper ballots                     │                │
          │  │  │  • Records CVRs (encrypted, SQLCipher)     │                │
          │  │  │  • Tabulates totals                        │                │
          │  │  │  • Hash-chained audit log                  │                │
          │  │  │  • Tamper sensors + quarantine              │                │
          │  │  │  • Signed export bundle at close            │                │
          │  │  └──────────────────────────────────────────┘                  │
          │  │                                                                 │
          │  │  ┌─────────────────┐    ┌──────────────────┐                    │
          │  │  │ SCANNER (backup) │    │  ADMIN CONSOLE    │                   │
          │  │  │ (failover unit)  │    │                    │                   │
          │  │  └─────────────────┘    │  • Open/close polls│                   │
          │  │                          │  • Export trigger   │                  │
          │  │  ┌─────────────────┐    │  • Device status    │                  │
          │  │  │ POLLBOOK #1      │    │  • Chain-of-custody│                   │
          │  │  │                   │    │    forms           │                   │
          │  │  │ • Check-in       │    └──────────────────┘                    │
          │  │  │ • Token issuance │                                            │
          │  │  │ • Thermal printer│                                            │
          │  │  │ • Audit log      │                                            │
          │  │  └─────────────────┘                                            │
          │  │  ┌─────────────────┐                                            │
          │  │  │ POLLBOOK #2      │                                           │
          │  │  │                   │                                           │
          │  │  │ • Check-in       │                                           │
          │  │  │ • Token issuance │                                           │
          │  │  │ • Thermal printer│                                           │
          │  │  │ • Audit log      │                                           │
          │  │  └─────────────────┘                                            │
          │  │                                                                 │
          │  └─────────────────────────────────────────────────────────────────┘
          │
          │  Signed results + audit bundles (on controlled media)
          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  STATE HQ  (optional, depends on state structure)                           │
│  ─────────                                                                  │
│                                                                             │
│  ┌──────────────────┐    ┌──────────────────────┐                           │
│  │  State Reporting   │    │  Audit / RLA Tooling  │                         │
│  │  Gateway           │    │                        │                        │
│  │                    │    │  • Risk-limiting audit  │                        │
│  │  • Collect county  │    │  • Random seed ceremony│                         │
│  │    certified       │    │  • Sample selection     │                        │
│  │    results         │    │  • Audit transcript     │                        │
│  │  • Publish         │    │  • Paper vs digital     │                        │
│  │    official        │    │    comparison           │                        │
│  │    results         │    └──────────────────────┘                          │
│  └──────────────────┘                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
          │
          │  EDC + SAC + PVP + audit transcripts
          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  PUBLIC TRANSPARENCY LAYER  (internet-facing, read-only publishing)          │
│  ──────────────────────────                                                 │
│                                                                             │
│  ┌──────────────────┐    ┌──────────────────────┐    ┌───────────────────┐  │
│  │  trustvoting.com   │    │  Transparency Log     │    │  Public Verifier  │  │
│  │                    │    │  (Merkle tree)         │    │  Tool (open src)  │  │
│  │  • EDC published   │    │                        │    │                   │  │
│  │  • SAC published   │    │  • EDC leaf hashes     │    │  • Verify EDC     │  │
│  │  • PVP download    │    │  • SAC leaf hashes     │    │  • Verify SAC     │  │
│  │  • Ops status feed │    │  • Inclusion proofs    │    │  • Verify PVP     │  │
│  │  • Audit reports   │    │  • Merkle roots        │    │  • Verify device  │  │
│  │                    │    │    (append-only)        │    │    records        │  │
│  └──────────────────┘    └──────────────────────┘    └───────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Trust Boundaries

```
BOUNDARY 1: AIR GAP (physical)
─────────────────────────────────
Precinct devices have ZERO network capability.
Data crosses this boundary ONLY on signed, sealed, controlled media.
Human carries media. Two-person chain-of-custody.

BOUNDARY 2: OFFLINE NETWORK (county HQ)
─────────────────────────────────
County HQ systems are offline or on isolated local network.
No connection to internet (except optional ops dashboard feed).
EMS, signing station, aggregation — all offline.

BOUNDARY 3: PUBLISH GATEWAY (one-way)
─────────────────────────────────
Only the publishing layer touches the internet.
It receives signed artifacts from county/state — NEVER sends commands back.
Read-only publishing. No control channel to any voting device.
```

---

## Data Flow: Election Lifecycle

### Phase 1: Software Release (Vendor → County)

```
Vendor Build CI
    │
    │  reproducible build + SBOM + tests + security scans
    ▼
Release Signing Ceremony (offline, 2-of-3 key holders)
    │
    │  SAC created (binds: image hash + test evidence + security evidence)
    ▼
Artifact Vault
    │
    │  signed update bundle + SAC written to controlled media
    ▼
County Update Station (offline)
    │
    │  two officials verify signatures + SAC
    │  device in Maintenance Mode (physical key)
    ▼
Device installs to inactive A/B partition
    │
    │  device verifies: image sig ✓  SAC sig ✓  hash match ✓  version monotonic ✓
    │  writes Update Record to audit log (SAC fingerprint)
    ▼
Device reboots into new image (or rolls back on failure)
```

### Phase 2: Election Definition (County → Precinct)

```
EMS Core (county, offline)
    │
    │  ballot styles, contests, candidates, precinct mapping, tabulation rules
    ▼
Definition Signing Station (offline)
    │
    │  runs Definition Test Suite:
    │    • schema validation
    │    • render determinism
    │    • contest mapping sanity
    │    • tabulation test vectors
    │    • import round-trip test
    │
    │  generates DTR (Definition Test Report)
    │  creates EDC (Election Definition Certificate)
    │  signed by: election authority + optional independent auditor
    │
    │  generates TAK_seed (Token Authentication Key seed):
    │    • 256-bit CSPRNG random value
    │    • included in election definition bundle (encrypted)
    │    • SHA-384(TAK_seed) committed in EDC
    ▼
EDC published (pre-election, public)
    │
    │  definition bundle + EDC written to controlled media
    ▼
Precinct devices accept definition ONLY if:
    • EDC signatures valid
    • bundle hash matches EDC
    • device role permitted by EDC
    • device logs EDC fingerprint in audit chain
    • (pollbooks + BMDs extract TAK_seed for BAT protocol)
```

### Phase 3: Election Day (precinct, air-gapped)

```
POLL OPEN
    │
    │  Admin Console triggers Open Polls
    │  Each device writes signed Poll Open Record:
    │    device_id + timestamp + EDC fingerprint + SAC fingerprint + tamper status
    │  Pollbooks + BMDs derive TAK from TAK_seed:
    │    TAK = HKDF-SHA-384(TAK_seed, election_id || precinct_id, "TrustVoting-BAT-v1")
    ▼
VOTING (per voter session)
    │
    │  Pollbook (check-in + token issuance):
    │    poll worker verifies voter identity
    │    → pollbook marks voter checked-in
    │    → pollbook generates Ballot Activation Token (BAT):
    │        CBOR payload: election_id, precinct_id, ballot_style, token_id,
    │                       pollbook_id, sequence_num, issued_at, expiry_at
    │        HMAC-SHA-384 tag computed with TAK
    │    → prints token slip (QR code + alphanumeric fallback)
    │    → voter takes slip to BMD
    │
    │  BMD (token verification + ballot session):
    │    voter presents token slip → BMD scans QR
    │    → verify HMAC tag (constant-time)
    │    → validate: election_id, precinct_id, ballot_style
    │    → check expiry (1 hour default)
    │    → check replay (consumed_tokens table in SQLCipher)
    │    → on success: start ballot session with voter's ballot style
    │    voter selects on touchscreen
    │    → machine prints paper ballot (human-readable + machine-readable)
    │    → voter verifies paper
    │    → voter deposits paper in ballot box
    │    → BMD logs: session event (token_id + sequence + timestamp, no vote content)
    │
    │  Spoiled ballot:
    │    voter surrenders ballot → poll worker spoils it
    │    → pollbook issues replacement token (new token_id, same voter)
    │    → max 3 tokens per voter (configurable), logged with reason + poll_worker_id
    │
    │  Scanner/Tabulator:
    │    voter feeds paper ballot
    │    → scanner reads ballot image
    │    → tabulation engine interprets marks
    │    → CVR created (encrypted, stored in SQLCipher)
    │    → running totals updated
    │    → hash-chained audit log entry written
    │
    │  Telemetry (operational only, no vote content):
    │    device health, printer/scanner events, ballot count, errors,
    │    BAT rate monitoring alerts
    │    → signed telemetry bundle (exportable to county ops dashboard)
    │
    │  TAMPER EVENT (if any):
    │    hardware security controller detects forbidden event
    │    → irreversible tamper flag set
    │    → device enters QUARANTINE
    │    → screen: "COMPROMISED — REMOVE FROM SERVICE"
    │    → signed incident record created
    │    → red LED on
    ▼
POLL CLOSE
    │
    │  Admin Console triggers Close Polls
    │  Each device writes signed Poll Close Record:
    │    device_id + timestamp + EDC fingerprint + totals hash + audit chain head
    │
    │  Precinct reconciliation check:
    │    tokens_issued (pollbooks) ≈ tokens_consumed (BMDs) + unused
    │    ballots_printed (BMDs) = ballots_scanned + spoiled + provisional
    │    discrepancies flagged in reconciliation report
    │
    │  Pollbook generates signed Export Bundle:
    │    • issued_tokens table
    │    • voter_roster snapshot
    │    • reissue log (spoiled ballot tokens)
    │    • audit log chain
    │
    │  BMD generates signed Export Bundle:
    │    • consumed_tokens table
    │    • session log + rate monitoring alerts
    │    • audit log chain
    │
    │  Scanner/Tabulator generates signed Export Bundle:
    │    • totals
    │    • CVRs (encrypted)
    │    • audit log chain
    │    • EDC fingerprint
    │    • SAC fingerprint
    │    • tamper status
    │    • all poll open/close records
    │
    │  All bundles written to controlled export media (USB, sealed)
    ▼
Media transported to County HQ (chain-of-custody logged)
```

### Phase 4: Results Aggregation (County HQ)

```
County Results Aggregation (offline)
    │
    │  import signed export bundles from all precincts
    │
    │  for each bundle:
    │    verify device signature ✓
    │    verify EDC fingerprint matches published EDC ✓
    │    verify SAC fingerprint matches known release ✓
    │    check tamper status (quarantined devices flagged + excluded) ✓
    │    verify audit chain integrity ✓
    │
    │  aggregate totals across precincts
    │  generate county canvass package (signed)
    ▼
County certifies results
    │
    │  signed results → State Reporting Gateway
    │  PVP assembled → Public Transparency Layer
    ▼
Public Verification Package published
    anyone can download + verify independently
```

---

## Ballot Activation Token (BAT) Protocol

The BAT protocol links voter check-in (pollbook) to ballot session activation (BMD). Without this link, a BMD could print unlimited ballots without pollbook authorization — enabling ballot stuffing or use of an unattended device to produce ballots assigned to inactive voters.

**Transport:** Printed slip with QR code + human-readable alphanumeric fallback. Voter carries slip from pollbook to BMD. No network between devices — air-gap consistent.

### Token Authentication Key (TAK) Derivation

All precinct devices derive a shared symmetric key from a seed distributed in the election definition bundle. No device-to-device communication is required.

```
TAK_seed:     generated at Definition Signing Station (county, offline)
              included in election definition bundle (encrypted, EDC-bound)
              SHA-384 hash of TAK_seed committed in EDC

TAK derivation (per precinct):

  TAK = HKDF-SHA-384(
    ikm  = TAK_seed,
    salt = election_id || precinct_id,
    info = "TrustVoting-BAT-v1"
  )

  Every pollbook and BMD in the precinct derives the same TAK
  from the election definition bundle at POLLS_OPEN.
```

### Token Payload (CBOR-serialized)

```
{
  "version":      1,                          // protocol version
  "election_id":  <32 bytes>,                 // SHA-384 truncated
  "precinct_id":  <string>,                   // precinct identifier
  "ballot_style": <string>,                   // voter's assigned ballot style
  "token_id":     <16 bytes>,                 // 128-bit random (unique per token)
  "pollbook_id":  <string>,                   // issuing pollbook device ID
  "sequence_num": <uint32>,                   // monotonic per pollbook session
  "issued_at":    <uint64>,                   // Unix timestamp (seconds)
  "expiry_at":    <uint64>                    // issued_at + 3600 (1 hour default)
}
```

### Token Generation (Pollbook)

```
1. Poll worker confirms voter identity and eligibility
2. Pollbook marks voter as checked-in in voter_roster table
3. Pollbook generates token payload:
     - token_id = 128-bit CSPRNG
     - sequence_num = next monotonic value for this pollbook
     - issued_at = current device time
     - expiry_at = issued_at + 3600
4. CBOR-serialize payload → token_bytes
5. Compute HMAC:
     tag = HMAC-SHA-384(TAK, token_bytes)
6. Encode for QR: Base45(token_bytes || tag)
7. Print slip: QR code + human-readable alphanumeric fallback
8. Log in issued_tokens table:
     token_id, voter_id (hashed), ballot_style, issued_at, sequence_num
9. Voter takes slip to BMD
```

### Token Verification (BMD)

```
1. Voter presents slip → BMD scans QR (or poll worker enters alphanumeric code)
2. Decode Base45 → token_bytes || tag
3. CBOR-deserialize token_bytes → payload
4. Verify HMAC:
     expected = HMAC-SHA-384(TAK, token_bytes)
     constant-time compare(expected, tag)
     FAIL → reject, log INVALID_TOKEN event
5. Validate fields:
     election_id matches loaded election     → FAIL = reject
     precinct_id matches this precinct       → FAIL = reject
     ballot_style is valid for this precinct → FAIL = reject
6. Check expiry:
     current_time > expiry_at               → FAIL = reject, log EXPIRED_TOKEN
7. Check replay:
     SELECT token_id FROM consumed_tokens WHERE token_id = ?
     EXISTS → reject, log REPLAY_DETECTED
8. Accept token:
     INSERT INTO consumed_tokens (token_id, consumed_at, ballot_style)
     Start ballot session with payload.ballot_style
     Log TOKEN_ACCEPTED event in audit chain
```

### Replay Prevention

**Per-BMD (real-time):** Each BMD maintains a `consumed_tokens` table in its SQLCipher database. A token accepted by one BMD cannot be replayed on the same BMD.

**Cross-BMD (poll close):** A token accepted by BMD #1 could theoretically be presented to BMD #2 (since BMDs do not communicate). This is detected at poll-close reconciliation when county aggregation cross-checks `token_id` values across all device exports. Duplicate `token_id` across devices triggers an alert and investigation.

### Spoiled Ballot Handling

```
Voter requests new ballot (paper damaged, marking error, etc.)
    │
    │  Poll worker verifies: original ballot exists and is surrendered
    │  Poll worker spoils original ballot (physical process)
    ▼
Pollbook issues replacement token:
    • Same voter, same ballot_style
    • New token_id, new sequence_num, new timestamps
    • Logged with: reason, poll_worker_id, original_token_id
    • Previous token NOT invalidated (BMD-side expiry handles it)

Maximum tokens per voter: 3 (configurable in election definition)
    • Pollbook enforces: count(issued_tokens WHERE voter_id = ?) < max_tokens
    • Exceeding max requires poll worker override + supervisor authorization
    • Override logged in audit chain
```

### Rate Monitoring (BMD-side)

BMDs monitor token acceptance rate to detect abnormal patterns:

| Condition | Threshold | Alert Level |
|-----------|-----------|-------------|
| Tokens in 10-minute window | > 6 | WARNING |
| Tokens in 10-minute window | > 10 | CRITICAL |
| Tokens in 60-second burst | > 3 | BURST |

Alerts are:
- Logged in the device's hash-chained audit log
- Displayed on the Admin Console status feed
- Included in the device's export bundle

Rate monitoring is advisory — it does not block valid tokens. Blocking would create a denial-of-service vector.

---

## Ballot Accounting & Reconciliation Protocol

Reconciliation ensures that every ballot is accounted for. Discrepancies are detected at poll close (precinct-level) and during county aggregation (cross-device).

### Reconciliation Equations

```
Equation 1 — Token accounting:
  tokens_issued (all pollbooks) = tokens_consumed (all BMDs) + tokens_unused

Equation 2 — Ballot accounting:
  ballots_printed (all BMDs) = ballots_scanned + ballots_spoiled + ballots_provisional

Equation 3 — Voter accounting:
  unique_voters_checked_in (pollbooks) = unique voters with consumed tokens (BMDs)

Equation 4 — Cross-device token integrity:
  COUNT(DISTINCT token_id across all device exports) = tokens_issued (all pollbooks)
  (any duplicate token_id across BMDs → investigation)
```

### Precinct-Level Procedure (Poll Close)

```
1. Admin Console triggers Close Polls on all devices
2. Each pollbook produces signed export bundle:
     • issued_tokens table (token_id, voter_id hash, ballot_style, timestamps)
     • voter_roster snapshot (checked-in count, not-voted count)
     • reissue log (spoiled ballot replacement tokens)
     • device audit log
3. Each BMD produces signed export bundle:
     • consumed_tokens table (token_id, consumed_at, ballot_style)
     • session log (ballot_printed events, cancelled sessions)
     • rate monitoring alerts
     • device audit log
4. Scanner/Tabulator produces signed export bundle (existing):
     • CVRs, totals, audit log, EDC/SAC fingerprints, tamper status
5. Admin Console runs precinct reconciliation check:
     • Compare token counts (Equations 1–3)
     • Flag discrepancies > threshold (configurable, default: 0)
     • Print reconciliation report (signed)
     • Include reconciliation status in precinct export
```

### County-Level Cross-Device Verification

```
County Results Aggregation imports all device bundles:

1. Verify all signatures on all device exports
2. Cross-check token_ids:
     • Collect all token_ids from all pollbook exports (issued)
     • Collect all token_ids from all BMD exports (consumed)
     • Verify: every consumed token_id exists in exactly one pollbook's issued list
     • Flag: any token_id consumed but not issued (CRITICAL)
     • Flag: any token_id consumed on multiple BMDs (CRITICAL)
3. Aggregate ballot accounting:
     • Sum ballots_printed across all BMDs
     • Sum ballots_scanned across all scanners
     • Compare with spoiled + provisional counts
4. Generate county reconciliation report (signed)
     • Per-precinct status
     • Flagged discrepancies with severity
     • Included in PVP package
```

### Discrepancy Handling

| Discrepancy | Severity | Action |
|-------------|----------|--------|
| Consumed token not in any pollbook's issued list | CRITICAL | Quarantine affected device, investigate |
| Same token_id consumed on multiple BMDs | CRITICAL | Flag both devices, investigate |
| tokens_issued - tokens_consumed ≠ tokens_unused | WARNING | Review for uncounted spoiled slips |
| ballots_printed ≠ ballots_scanned + spoiled + provisional | WARNING | Physical ballot count reconciliation |
| Rate monitoring CRITICAL alert in export | REVIEW | Investigate device usage pattern |

---

## Device Authorization & Rogue Device Prevention

A rogue BMD — set up in a warehouse or back room — could theoretically print unauthorized ballots at scale. The system must prove that every device in use is an authorized device, without exposing a public device inventory that becomes an attack target list.

**Design principle:** The device whitelist is internal to the EDC (not published as a separate public registry). Cryptographic controls prove device legitimacy at every stage without revealing device IDs, TPM keys, or precinct assignments to adversaries.

### Why a Public Device Registry Is Not Used

Publishing device IDs, TPM public keys, and precinct assignments would give adversaries:
- A target list (which devices are where, how many per precinct)
- Key material to probe (TPM public keys)
- Social engineering vectors (device-to-precinct mapping)

This exposure is unnecessary because the same assurance is achieved through cryptographic controls that keep the device inventory internal.

### Defense Layers (how rogue devices are prevented and detected)

```
Layer 1 — EDC Device Whitelist (pre-election)
──────────────────────────────────────────────
  EDC contains authorized device list per precinct:
    • device_id + device_role (BMD, pollbook, scanner, admin)
    • assigned precinct
    • signed by Election Authority Key

  At ELECTION_LOADED:
    device checks its own device_id against EDC whitelist
    → NOT on list = refuse to operate, log UNAUTHORIZED_DEVICE event

Layer 2 — TAK_seed Access Control (poll open)
──────────────────────────────────────────────
  TAK_seed is encrypted in the election definition bundle
  Only devices that:
    • pass secure boot (SAC-validated image)
    • unseal TPM keys (boot measurements + no tamper flag)
    • match EDC device whitelist
  can extract TAK_seed and derive TAK

  A rogue device without a valid TPM-bound key CANNOT derive TAK
  → cannot generate valid BAT tokens (as pollbook)
  → cannot verify BAT tokens (as BMD)
  → ballots printed without BAT have no matching tokens in any
     pollbook's issued_tokens table

Layer 3 — Multi-Party Verification (procedural, poll open)
──────────────────────────────────────────────────────────
  Before polls open, party representatives verify:
    • device count matches EDC-specified count for precinct
    • device IDs displayed on each device screen match
      the sealed device manifest provided to observers
    • tamper seals intact on all devices
    • all devices show valid SAC + EDC fingerprints

  Device manifest is distributed to party observers
  under controlled conditions (not published online).
  Observers sign attestation form confirming verification.

Layer 4 — Device Signature Chain (post-election)
──────────────────────────────────────────────────
  Every export bundle is signed by the device's TPM-bound key
  Device key chains to: Device CA → Vendor Root Key

  County Results Aggregation verifies:
    • bundle signature is valid
    • signing key chains to known Device CA
    • device_id in bundle matches EDC whitelist for that precinct
    • reject any bundle from unknown/unauthorized device

Layer 5 — BAT Reconciliation (post-election)
──────────────────────────────────────────────
  Even if a rogue device somehow produced ballots:
    • no matching token_ids in any pollbook's issued_tokens
    • tokens_issued ≠ tokens_consumed + unused (reconciliation fails)
    • ballots_printed ≠ ballots_scanned + spoiled + provisional
    • county cross-device check flags orphaned token_ids

  Ballots from a rogue device create detectable discrepancies
  across multiple independent reconciliation equations.
```

### Rogue Device Attack Scenarios

| Scenario | Blocked By | Detection |
|----------|-----------|-----------|
| Rogue BMD without election definition | Layer 1 — no EDC, device cannot operate | Immediate (device refuses to boot into voting mode) |
| Rogue BMD with stolen election definition | Layer 2 — no valid TPM keys, cannot extract TAK_seed or derive TAK | Immediate (cannot verify BAT tokens, cannot start ballot sessions) |
| Rogue BMD with cloned TPM keys (extremely difficult) | Layer 3 — party observers verify device count and IDs at poll open | Pre-election (observer attestation) |
| Rogue device produces ballots offline, smuggles paper into ballot box | Layer 5 — ballots have no matching consumed tokens; ballot count > token count | Poll close reconciliation |
| Insider adds unauthorized device to precinct | Layers 1+3+4 — EDC whitelist rejects unknown device_id; observers verify device count; county rejects unknown device signature | Multiple layers |

### Summary

No single control prevents every rogue device scenario. The five layers create **defense in depth**:

- Layers 1–2 make it **technically infeasible** (EDC whitelist + TPM-sealed TAK access)
- Layer 3 makes it **procedurally visible** (multi-party observer verification)
- Layers 4–5 make it **cryptographically detectable** (device signature chain + BAT reconciliation)

A rogue device would need to simultaneously: possess a valid TPM-bound key signed by the Device CA, appear on the EDC device whitelist, survive multi-party physical inspection, produce ballots with matching BAT tokens from an authorized pollbook, and generate an export bundle that passes county signature verification. Compromising all five layers simultaneously is not a credible attack.

---

## Device Internal Architecture

### Common Security Core (shared by all device types)

```
┌─────────────────────────────────────────────────────────────┐
│  HARDWARE LAYER                                              │
│                                                              │
│  ┌────────────────┐  ┌──────────┐  ┌──────────────────────┐ │
│  │ Industrial SBC  │  │ TPM 2.0  │  │ Security             │ │
│  │ (fanless x86,   │  │          │  │ Microcontroller      │ │
│  │  no Wi-Fi/BT/   │  │ • PCR    │  │ (out-of-band)        │ │
│  │  no Ethernet)   │  │   storage│  │                      │ │
│  │                 │  │ • key    │  │ Monitors:            │ │
│  │ • CPU           │  │   sealing│  │ • case open switch   │ │
│  │ • RAM           │  │ • tamper │  │ • port cover seal    │ │
│  │ • SSD/eMMC      │  │   flag   │  │ • USB insertion      │ │
│  │   (encrypted)   │  │          │  │ • debug headers      │ │
│  └────────────────┘  └──────────┘  │ • link-up detect     │ │
│                                     │                      │ │
│  ┌────────────────┐                 │ On tamper:           │ │
│  │ Battery + UPS   │                 │ • set eFuse flag     │ │
│  │ (4-10 hr)       │                 │ • force quarantine   │ │
│  └────────────────┘                 │ • red LED on         │ │
│                                     │ • log to WORM        │ │
│  ┌────────────────┐                 └──────────────────────┘ │
│  │ Locked USB port │                                         │
│  │ (import/export  │                                         │
│  │  only, sealed)  │                                         │
│  └────────────────┘                                         │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│  BOOT CHAIN                                                  │
│                                                              │
│  UEFI Secure Boot → signed bootloader → signed kernel        │
│  → signed initramfs → measured boot (PCRs into TPM)          │
│  → TPM unseal keys ONLY IF:                                  │
│      PCRs match known-good ✓  AND  tamper flag not set ✓     │
│  → mount read-only root (dm-verity integrity)                │
│  → launch single-purpose application                         │
│                                                              │
│  If ANY check fails → QUARANTINE BOOT                        │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│  OS LAYER (Yocto Linux, immutable)                           │
│                                                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────────┐  │
│  │ A/B      │ │ SELinux   │ │ seccomp  │ │ No package     │  │
│  │ partition│ │ enforcing │ │ filters  │ │ manager        │  │
│  │ scheme   │ │           │ │          │ │ No shell       │  │
│  └──────────┘ └──────────┘ └──────────┘ │ No SSH         │  │
│                                          │ No NTP         │  │
│                                          │ No listeners   │  │
│                                          └────────────────┘  │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│  APPLICATION LAYER (sandboxed per function)                   │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Namespace: UI / Ballot Rendering / Printing            │  │
│  │  (BMD only — requires valid BAT to start session)       │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: BAT Verifier (BMD only)                     │  │
│  │  (QR scan, HMAC verify, replay check, rate monitor)     │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: Pollbook (check-in, token issuance)         │  │
│  │  (Pollbook only — voter roster, BAT generation, print)  │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: Scanner Pipeline / Tabulation Engine        │  │
│  │  (Tabulator only)                                       │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: Export Bundler                              │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: Logging Service (append-only, hash-chain)   │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │  Namespace: Update Verifier (maintenance mode only)     │  │
│  └────────────────────────────────────────────────────────┘  │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│  DATA LAYER                                                  │
│                                                              │
│  ┌──────────────────┐  ┌──────────────────────────────────┐  │
│  │ SQLCipher DB      │  │ Audit Log (separate partition)    │  │
│  │ (AES-256-GCM)     │  │                                   │  │
│  │                   │  │ • Append-only                     │  │
│  │ • Election def    │  │ • Hash-chained entries            │  │
│  │   metadata        │  │ • Signed checkpoints              │  │
│  │ • Device state    │  │ • Encrypted at rest               │  │
│  │ • CVRs (tabulator)│  │ • Redundant copy on microSD       │  │
│  │ • Counters        │  │                                   │  │
│  │ • consumed_tokens │  │ Key sealed by TPM                 │  │
│  │   (BMD — replay   │  │ (unseals only if boot OK +        │  │
│  │    prevention)    │  │  tamper flag not set)              │  │
│  │ • issued_tokens   │  │                                   │  │
│  │   (Pollbook)      │  │                                   │  │
│  │ • voter_roster    │  │                                   │  │
│  │   (Pollbook)      │  │                                   │  │
│  │                   │  │                                   │  │
│  │ Key sealed by TPM │  │                                   │  │
│  └──────────────────┘  └──────────────────────────────────┘  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

---

## Device State Machine

```
                    ┌─────────────────────┐
                    │ FACTORY_PROVISIONED  │
                    │                     │
                    │ • Device ID assigned │
                    │ • TPM keys created   │
                    │ • Secure boot enabled│
                    └──────────┬──────────┘
                               │
                    load signed software (SAC required)
                               │
                               ▼
                    ┌─────────────────────┐
                    │  SOFTWARE_LOADED     │
                    │                     │
                    │ • SAC fingerprint    │
                    │   logged             │
                    └──────────┬──────────┘
                               │
                    load signed election definition (EDC required)
                               │
                               ▼
                    ┌─────────────────────┐
                    │  ELECTION_LOADED     │
                    │                     │
                    │ • EDC fingerprint    │
                    │   logged             │
                    └──────────┬──────────┘
                               │
                    admin triggers "Open Polls"
                               │
                               ▼
                    ┌─────────────────────┐
                    │  POLLS_OPEN          │
                    │                     │◀──── voter sessions cycle here
                    │ • Poll Open Record   │
                    │   signed + logged    │
                    │ • TAK derived from   │
                    │   TAK_seed (HKDF)    │
                    │   (pollbooks + BMDs) │
                    │ • Accepting ballots  │
                    │   (BAT required)     │
                    └──────────┬──────────┘
                               │
                    admin triggers "Close Polls"
                               │
                               ▼
                    ┌─────────────────────┐
                    │  POLLS_CLOSED        │
                    │                     │
                    │ • Poll Close Record  │
                    │   signed + logged    │
                    │ • No more ballots    │
                    │ • Reconciliation     │
                    │   check run          │
                    └──────────┬──────────┘
                               │
                    admin triggers "Export"
                               │
                               ▼
                    ┌─────────────────────┐
                    │  EXPORTED            │
                    │                     │
                    │ • Signed export      │
                    │   bundle on media    │
                    │   (all device types: │
                    │    pollbook, BMD,    │
                    │    scanner, admin)   │
                    │ • Chain-of-custody   │
                    │   sealed             │
                    └──────────┬──────────┘
                               │
                    device returned to warehouse
                               │
                               ▼
                    ┌─────────────────────┐
                    │  ARCHIVED            │
                    │                     │
                    │ • Logs preserved     │
                    │ • Awaits next cycle  │
                    └─────────────────────┘


    ╔═══════════════════════════════════════════════════╗
    ║  TAMPER EVENT (from ANY state above)              ║
    ║                                                   ║
    ║  hardware controller detects forbidden event      ║
    ║           │                                       ║
    ║           ▼                                       ║
    ║  ┌─────────────────────────────┐                  ║
    ║  │  QUARANTINED_COMPROMISED     │                 ║
    ║  │                              │                 ║
    ║  │  • Irreversible tamper flag  │                 ║
    ║  │  • Voting/tabulation OFF     │                 ║
    ║  │  • Screen: "COMPROMISED"     │                 ║
    ║  │  • Red LED on                │                 ║
    ║  │  • Signed incident record    │                 ║
    ║  │  • NO admin can clear this   │                 ║
    ║  │  • Requires factory reimage  │                 ║
    ║  │    + rekey to recover        │                 ║
    ║  └─────────────────────────────┘                  ║
    ╚═══════════════════════════════════════════════════╝
```

---

## Software Modules

### On-Device Software (Yocto Linux)

| Module | Runs on | Role | Human/Machine |
|--------|---------|------|---------------|
| `tv-boot` | All devices | Secure boot chain, TPM measurement, key unsealing | Machine-only |
| `tv-os` | All devices | Immutable base OS, SELinux policy, namespace setup | Machine-only |
| `tv-ballot-ui` | BMD | Touchscreen ballot interface, accessibility, language (requires valid BAT to start session) | Machine (voter interacts) |
| `tv-bat` | BMD | BAT verification: QR scan, HMAC-SHA-384 verify, replay check (consumed_tokens), rate monitoring | Machine-only |
| `tv-pollbook` | Pollbook | Voter check-in, roster management, BAT generation, HMAC-SHA-384 signing, thermal slip printing | Machine (poll worker interacts) |
| `tv-printer` | BMD | Ballot rendering, thermal print control, paper path | Machine-only |
| `tv-scanner` | Tabulator | Optical scan pipeline, image capture, mark interpretation | Machine-only |
| `tv-tabulate` | Tabulator | Tabulation engine, contest rules, CVR creation | Machine-only |
| `tv-audit-log` | All devices | Append-only hash-chained log, signed checkpoints | Machine-only |
| `tv-export` | All devices | Bundle creation, signing, media write | Machine-only |
| `tv-admin` | Admin console | Open/close polls, status display, export trigger | Human-driven |
| `tv-update` | All devices | Offline update verifier (maintenance mode only) | Human-triggered |
| `tv-tamper` | All devices | Security controller interface, quarantine logic | Machine-only |

### County HQ Software (Debian Linux)

| Module | Role | Human/Machine |
|--------|------|---------------|
| `tv-ems` | Election definition authoring, ballot design, precinct management | Human-driven |
| `tv-def-sign` | Definition test suite, DTR generation, EDC signing | Human ceremony + machine checks |
| `tv-aggregator` | Import + verify precinct bundles, aggregate totals, canvass package | Machine verify + human oversight |
| `tv-ops-dashboard` | Real-time precinct status, alerts, logistics, AI anomaly detection | Human-driven |
| `tv-update-station` | Install signed updates to devices (maintenance mode) | Human ceremony |

### Public / Verification Software (open-source)

| Module | Role | Human/Machine |
|--------|------|---------------|
| `tv-verifier` | CLI/web tool: verify EDC, SAC, PVP, device records | Human-run |
| `tv-transparency-log` | Merkle tree append-only log for EDC/SAC commitments | Machine-maintained |
| `tv-publisher` | Publish verification artifacts to trustvoting.com | Human-approved + machine-published |

---

## Cryptographic Primitives

| Purpose | Algorithm | Notes |
|---------|-----------|-------|
| Data at rest | AES-256-GCM | SQLCipher DB + audit log partition |
| Full-disk | AES-XTS-256 | Where applicable |
| Hashing | SHA-384 | All integrity checks, hash chains |
| Signatures | Ed25519 | Release, EDC, device, export bundles |
| Signatures (alt) | ECDSA P-384 | If compliance requires NIST curves |
| Key sealing | TPM 2.0 | Bound to PCR values + tamper flag |
| Key derivation | HKDF-SHA-384 | Per-election, per-device key derivation; TAK derivation for BAT |
| Token authentication | HMAC-SHA-384 | Ballot Activation Token (BAT) generation + verification |
| Token serialization | CBOR (RFC 8949) | BAT payload encoding — compact binary, deterministic |
| TLS (publish only) | TLS 1.3 | Public transparency layer only |

---

## Certificate Chain

```
Vendor Root Key (offline HSM, 2-of-3 threshold)
    │
    ├── Release Key ──── signs SAC (software releases)
    │
    ├── Device CA ──── signs per-device identity certificates
    │
    └── (optional) Transparency Log Key

Election Authority Key (county, offline)
    │
    ├── Definition Key ──── signs EDC (election definitions)
    │                        EDC contains:
    │                          • election definition hash
    │                          • device whitelist (per precinct)
    │                          • TAK_seed hash
    │
    ├── TAK_seed ──── BAT token authentication (HMAC-SHA-384)
    │                  generated at Definition Signing Station
    │                  SHA-384 hash committed in EDC
    │                  distributed in election definition bundle
    │
    └── Results Key ──── signs canvass packages

Independent Auditor Key (optional, offline)
    │
    └── Attestation Key ──── co-signs EDC for additional trust

Device Key (per-device, TPM-bound)
    │
    ├── signs poll open/close records
    ├── signs export bundles
    ├── signs audit log checkpoints
    └── signs telemetry bundles
```
