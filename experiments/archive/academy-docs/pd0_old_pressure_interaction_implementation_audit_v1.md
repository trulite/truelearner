# PD0 old pressure interaction implementation audit v1

Status: frozen before PD0 matrix execution.

Parent protocol: `bde462a`, tagged
`pd0-old-pressure-interaction-protocol-v1`.

## Runtime surface

PD0 changes no runtime, constant, checkpoint, or substrate source. The complete
PQLC1/PQLC0 core remains byte-identical:

```text
core Cargo.toml
14d45bc379a5220d33b028b48f38319cb888f732d0b34655fda02b3941a829a8

core lib.rs
c5173e8d43d109465252813fba411288c59e3bfa274f790519747eb34314e894

core mechanics.rs
266b713130be6b221432022c7518cc413a0def30ca00371422af6aceeda900da
```

The evaluator observes only public durable state, CPC1 participation/support,
physical events, work, clock, and canonical body bytes. It reconstructs the
old rectangular eligibility state from candidate `Eligible` events and the
ordinary Modulatory delivery that consumes it. It does not read or mutate the
private eligibility field.

Every CELL is ten position units from its neighbor, outside the fixed local
proposal radius. The evaluator requires zero structural proposals.

## Frozen matrix surface

The unconditional inventory is:

```text
dormant                          30
used without consequence        30
timed consequence              240
unrelated activity             240
same-path renewal              210
                                ---
physical cases                 750
mechanics rows                1500
same-mechanics replay runs    3000
```

Every case records a per-tick path trajectory through sixty physical ticks,
including liveness, durable resistance/coupling, reconstructed rectangular
eligibility, continuous participation, and CPC1 support. Pressure epochs and
the state immediately before/after each delayed event are serialized
independently.

Reference and Production complete observations are compared exactly before
summary aggregation. Characterization has no desired-behavior predicate.

## Frozen hashes

```text
evaluator Cargo.toml
39383a42007814eeefd69220f17e50e16c8de09d6cd2e044c9d7761f0eaa3a17

evaluator main.rs
0066d6a9902ab79f6d72ed1f1ec7fe413c14819b4b0ab17220e182f0641d4720

static audit
43e0cdf3cb152ee21b522b77589a4f84ed3acd58ba7ce234741df12674a59f8a

protocol
23210c9cfc324c9f55cea630f1849617498318efab355400afb44675b7392de0
```

## Targeted E2B validation

Reusable sandbox `il7q2g6fbjh6lmrgnbudz` ran only:

- evaluator formatting;
- targeted evaluator release check;
- strict evaluator Clippy with `-D warnings`;
- shell-audit syntax validation;
- exact frozen core hash checks;
- evaluator hidden-state mutation/private-function scans.

No physical world or matrix ran. No pre-freeze scientific or fixture repair
was required.

## Boundary

PD0 introduces no PD1 candidate, pressure equation, constant, ARC input,
authority, oracle, or `arch.md` change. No comparator repair or fixture change
is authorized after evidence begins.
