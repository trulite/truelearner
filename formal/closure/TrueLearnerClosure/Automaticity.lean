namespace TrueLearnerClosure

/- Observer-only model of the law implemented by the Rust body. A segment has
   no level or task label, so the result of composition can enter `compose`
   again without a separate macro operation. -/
structure Segment where
  source : Nat
  target : Nat
  duration : Nat
  boundaryCrossings : Nat
deriving BEq, DecidableEq, Repr

structure Interior where
  visibleEffects : Nat
  boundaryCrossings : Nat
  pendingInputs : Nat
deriving BEq, DecidableEq, Repr

def Interior.transparent (interior : Interior) : Prop :=
  interior.visibleEffects = 0 ∧
    interior.boundaryCrossings = 0 ∧
    interior.pendingInputs = 0

def composable (left right : Segment) (interior : Interior) : Prop :=
  left.target = right.source ∧ interior.transparent

def compose (left right : Segment) : Segment :=
  {
    source := left.source
    target := right.target
    duration := left.duration + right.duration
    boundaryCrossings := left.boundaryCrossings + right.boundaryCrossings
  }

structure Observation where
  source : Nat
  target : Nat
  duration : Nat
  boundaryCrossings : Nat
deriving BEq, DecidableEq, Repr

def observe (segment : Segment) : Observation :=
  {
    source := segment.source
    target := segment.target
    duration := segment.duration
    boundaryCrossings := segment.boundaryCrossings
  }

theorem composition_is_level_free_and_associative
    (first second third : Segment) :
    compose (compose first second) third = compose first (compose second third) := by
  simp [compose, Nat.add_assoc]

theorem composition_preserves_ordered_endpoints
    (left right : Segment) :
    (compose left right).source = left.source ∧
      (compose left right).target = right.target := by
  simp [compose]

theorem composition_preserves_total_time
    (left right : Segment) :
    (compose left right).duration = left.duration + right.duration := by
  rfl

theorem composition_preserves_boundary_crossings
    (left right : Segment) :
    (compose left right).boundaryCrossings =
      left.boundaryCrossings + right.boundaryCrossings := by
  rfl

theorem a_boundary_crossing_is_not_transparent
    (interior : Interior)
    (crosses : 0 < interior.boundaryCrossings) :
    ¬interior.transparent := by
  intro transparent
  simp [Interior.transparent] at transparent
  omega

theorem a_pending_input_is_not_transparent
    (interior : Interior)
    (pending : 0 < interior.pendingInputs) :
    ¬interior.transparent := by
  intro transparent
  simp [Interior.transparent] at transparent
  omega

theorem independent_work_is_construction_order_invariant
    (left right : Nat) :
    left + right = right + left := by
  omega

end TrueLearnerClosure
