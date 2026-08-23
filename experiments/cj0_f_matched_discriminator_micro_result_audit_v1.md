# CJ0-F matched discriminator MICRO result audit v1

Status: **INTERPRETABLE DEVELOPMENT MICRO; BOTH FAIL; SHARED BOUNDARY ROBUST**.

The unchanged corrected comparator executed `micro` once after the frozen
PROBE audit. It independently reconstructed 1,920 CJ-B and 1,920 CJ-E worlds
from equal row serializations across seeds 211/223, normal/mirror layouts,
thresholds 2/3, coupling 1/2, loads 0/4, sparse/dense allocations, and every
preregistered timing transfer.

| artifact | rows / bytes | SHA-256 |
|---|---:|---|
| CJ-B CSV | 1,920 / 549,511 | `0c58cada8a18d146e9ed9fd81ce03d02528db33f0096a78ece9dfbb5c2e3d0ab` |
| CJ-E CSV | 1,920 / 551,410 | `ceb79ee889301230c0512fcdb1777a873124675381e45d0b7f8819f603dff354` |
| paired CSV | 1,920 / 277,938 | `8702b2c546c12443eeba5e0e4064f30d1dd42a9ac6e038349006ce068aa54795` |
| report | 962 bytes | `4c3419a4716e56be31df4697b43364c3d807f7da5008bd9b2b9869ae213adfdc` |

| family | rows | CJ-B pass | CJ-E pass | differences | B false conjunction | E false conjunction |
|---|---:|---:|---:|---:|---:|---:|
| same-source bursts | 544 | 416 | 456 | 64 | 80 | 80 |
| amplitude vs multiplicity | 128 | 64 | 96 | 32 | 64 | 32 |
| dense return topology | 192 | 192 | 192 | 0 | 0 | 0 |
| timing transfer | 672 | 528 | 648 | 120 | 0 | 0 |
| shared controls | 384 | 360 | 384 | 24 | 24 | 0 |
| **total** | **1,920** | **1,560** | **1,776** | **240** | **168** | **112** |

The PROBE discriminator reproduces without seed, mirror, threshold or load
rescue. CJ-E rejects every matched strong singleton but still accepts
same-source paired weak arrivals. CJ-B accepts both same-source paired weak
arrivals and matched strong singleton matter; mature coupling additionally
creates source-alone/crossed effects. Both retain correct dense attribution in
all 192 rows. Timing differences remain candidate-law predictions rather than
selection evidence because both already fail multiplicity.

| measure | CJ-B | CJ-E |
|---|---:|---:|
| total native work | 64,512 | 59,248 |
| summed per-row persistent bytes | 651,776 | 651,776 |
| per-row persistent range | 160..1,024 | 160..1,024 |
| summed temporary-byte lower bound | 172,800 | 172,800 |
| maximum temporary-byte lower bound | 240 | 240 |
| naturally quiescent | 1,920/1,920 | 1,920/1,920 |
| runaway | 0 | 0 |

Classification remains **both fail; shared boundary frozen**. Work cannot
select after scientific insufficiency. The result is interpretable, all CSV
shapes and paired identifiers reconcile, and no staging artifact remains.
The terminal preregistered development GATE may execute once for the full
fresh-seed/load/threshold robustness surface. PX3 remains absent and no
definitive/authority evidence exists.
