---
name: categorical-rust
description: Model, implement, or refactor Rust code using category-theoretic structure while keeping the result idiomatic and readable. Use for typed pipelines, composable transformations, algebraic data types, state transitions, effect boundaries, and law-driven API design, even when the user does not explicitly mention category theory.
---

# Categorical Rust

```text
Domain problem
     |
     v
Identify objects, arrows,
composition, and laws
     |
     v
Translate into ordinary Rust
 types    -> structs, enums, newtypes
 arrows   -> functions
 failure  -> Option or Result
 effects  -> explicit boundaries
     |
     v
Compose the smallest useful API
     |
     v
Test laws and explain it
in domain language
```

## Rules

- Use category theory as a modeling lens, not as naming or ceremony.
- Start with the domain states and transformations between them.
- Make invalid states unrepresentable with structs, enums, and newtypes.
- Prefer total functions; represent partiality with `Option` or `Result`.
- Keep transformations pure and move I/O, time, randomness, and mutation to boundaries.
- Prefer functions over traits and concrete types over generic machinery.
- Add an abstraction only when it removes real duplication or enforces a useful law.
- Test identity, associativity, or preservation laws when the design depends on them.
- Explain the result using domain names, not category-theory jargon.
- Stop when the abstraction makes ownership, lifetimes, or errors harder to understand.
