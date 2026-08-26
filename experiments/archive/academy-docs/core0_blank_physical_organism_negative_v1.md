# CORE0 v1 immutable evaluator negative

The sole CORE0 v1 execution compiled the frozen candidate and then stopped
before publishing the 60-row matrix.

The failure was evaluator timing, not organism physics. CORE-A/E2 generated a
two-hop contact relation at tick zero. Its ordinary delays advanced the body to
tick two. The evaluator then attempted to admit the second experience at the
hard-coded absolute tick one, and the runtime correctly rejected an arrival
that preceded current physical time.

No CORE profile failed a capability predicate. No result CSV or report was
published, no downstream row was reached, and the v1 execution will not be
rerun or relabeled.

The only lawful successor correction is to admit E2's second experience at the
body's then-current tick and its consequence at the body's then-current tick
after that propagation. This preserves immediate sequential experience while
making no assumption about how many lawful ticks the generated topology used.

All runtime bytes, four profiles, capability worlds, experience order,
predicates, comparators and prefix rules remain frozen.
