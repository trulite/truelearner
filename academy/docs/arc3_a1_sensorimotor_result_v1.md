# ARC3-A1 sensorimotor development result v1

ARC3-A1 is development-positive for the first closed raster/action learning
loop. It is not an ARC-AGI-3 task-solving result.

The official `ls20` environment supplied every 64x64 raster. A persistent Rust
agent admitted a fixed, label-free dominant-palette sensor into the unchanged
TrueLearner physical runtime. Every executed ARC action originated in an
ordinary outward motor crossing. The external bridge only mapped that crossing
to an available ARC actuator after it left the organism.

## Observed physical sequence

```text
fresh raster context
    weak candidate coupling 1
    motor threshold 2
    no babbling
        -> no action

same raster + one ordinary motor-babbling pulse
        -> motor 0 crosses outward
        -> Academy executes ARC action 1

official next raster changes by 52 cells
context trace + actual motor trace + changed-raster return
        -> exactly one Modulatory arrival
        -> candidate resistance 1 -> 4
        -> candidate coupling 1 -> 2
pressure
        -> retained resistance 3

reset official environment, no babbling, no modulation
same coarse raster context
        -> motor 0 crosses outward
        -> ARC action 1
```

A second completed physical loop raised retained resistance to 5. After the
frozen learned probe and shuffled-map control applied ordinary pressure, the
route still survived a further ten-tick retention gap at resistance 2 and
produced the same unaided motor crossing.

## Episode matrix

| Episode | Result |
|---|---|
| untrained raster, no babbling | zero action |
| initial exploration | one scaffolded motor crossing |
| changed-raster development | two qualified updates; coupling reaches 2 |
| frozen learned probe | one action; no babbling; zero update |
| shuffled boundary map | motor 0 unchanged; external action becomes 2 |
| retention | one unaided action after physical gap |
| adjacent actions, return blocked | zero updates; route deallocates; silence |

Across the seven episodes there were 11 organism observations, nine outward
crossings, two qualified plasticity updates, 11,109 total units of physical
work, and a maximum of 3,511 work in one observation. Every observation
naturally quiesced. A complete second run against freshly created official
environments produced byte-identical normalized evidence.

## Firewall and limitations

TrueLearner received no game ID, ARC action ID, score, terminal state, level
count, goal, expected action, or evaluator result. The shuffled-map control
shows that action meaning remained outside the body: the motor crossing was
unchanged while the external action changed.

This result establishes one retained action-effect relation in one coarse
visual context. It does not establish exploration policy, object perception,
goal discovery, planning, transfer to a new ARC game, or level completion.
Those require subsequent developmental gates.

## Evidence

- official game: `ls20`, version `9607627b`
- seed: `205`
- E2B sandbox: `i0sx3yhxt5wtfvhxb0j0b`
- canonical suite SHA-256:
  `bfc9a1cb945d3ad5864d7a3ef12e7c648349942813c33aa328bcb695afc04198`
- episode catalog SHA-256:
  `814c1db81c7a24e8eb20abc0ea46ad882cc06282c9cad6c58dc88b10c824ee96`
- development video SHA-256:
  `f2fcdd5de8cf17b655cae7264e7e0677c05805d764bed22fe5d2309c6630f68d`
- learned-probe video SHA-256:
  `50cebe4fdcfe189019801ef48f02a647a094671eff2cc976b647fd9d1e5f5943`
- blocked-return video SHA-256:
  `3cd2fb0714ae172467bb49bc2a89bda5bbef5ef66217f6a608ac94f77bce034e`

The official live suite ran twice. Rendering occurred afterward and did not
rerun or feed media back into the organism. No file under `truelearner/` was
modified.
