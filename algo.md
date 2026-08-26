# Algorithm

```text
TIME
 │
 t1    Input fires
 │
 t2    Links meet at a junction
 │
 t3    Links form paths
 │
 t4    The junction chooses one path
 │
 t5    Output fires
 │
 t6    Outcome returns along the used path
 │
 t7    Used links strengthen
 │
 t8    Later input reuses the path
 ▼
```

```text
t1–t5: fire, form, choose

                 ┌──── path ────┐
                 │              │
[input] ──link──▶(junction)──link──▶[output]
                 │
                 └── chooses one path


t6–t7: return and strengthen

[input] ══link══▶(junction)══link══▶[output]
                    ◀──────── [outcome]
                       returns

         ══ used links strengthen ══


t8: reuse

[later input] ══link══▶(junction)══link══▶[output]
                         reused path
```
