import TrueLearnerClosure.Model

namespace TrueLearnerClosure

theorem no_explanation_makes_no_claim :
    resolveIds [] = .noClaim := by
  rfl

theorem one_explanation_closes_exactly_that_witness (witness : WitnessId) :
    resolveIds [witness] = .closed witness := by
  rfl

theorem closed_resolution_has_one_explanation
    (explanations : List WitnessId) (witness : WitnessId)
    (closed : resolveIds explanations = .closed witness) :
    explanations = [witness] := by
  cases explanations with
  | nil => simp [resolveIds] at closed
  | cons first rest =>
      cases rest with
      | nil => simpa [resolveIds] using closed
      | cons second rest => simp [resolveIds] at closed

theorem several_explanations_are_ambiguous
    (first second : WitnessId) (rest : List WitnessId) :
    resolveIds (first :: second :: rest) = .ambiguous := by
  rfl

theorem ambiguous_return_persists_nothing (request : CheckRequest) :
    persistentLinks request .ambiguous = #[] := by
  rfl

theorem no_claim_persists_nothing (request : CheckRequest) :
    persistentLinks request .noClaim = #[] := by
  rfl

theorem closed_return_persists_exact_support
    (request : CheckRequest) (witnessId : WitnessId) (witness : Witness)
    (found : findWitness? request.witnesses witnessId = some witness) :
    persistentLinks request (.closed witnessId) = witness.support := by
  simp [persistentLinks, found]

def ContextEquivalent
    {Context History Observation : Type}
    [BEq Observation]
    (observe : Context → History → Observation)
    (contexts : List Context)
    (left right : History) : Prop :=
  ∀ context, context ∈ contexts → observe context left = observe context right

theorem context_equivalent_refl
    {Context History Observation : Type}
    [BEq Observation]
    (observe : Context → History → Observation)
    (contexts : List Context)
    (history : History) :
    ContextEquivalent observe contexts history history := by
  intro _ _
  rfl

theorem context_equivalent_symm
    {Context History Observation : Type}
    [BEq Observation]
    (observe : Context → History → Observation)
    (contexts : List Context)
    (left right : History)
    (equivalent : ContextEquivalent observe contexts left right) :
    ContextEquivalent observe contexts right left := by
  intro context member
  exact (equivalent context member).symm

theorem context_equivalent_trans
    {Context History Observation : Type}
    [BEq Observation]
    (observe : Context → History → Observation)
    (contexts : List Context)
    (left middle right : History)
    (leftEquivalent : ContextEquivalent observe contexts left middle)
    (rightEquivalent : ContextEquivalent observe contexts middle right) :
    ContextEquivalent observe contexts left right := by
  intro context member
  exact (leftEquivalent context member).trans (rightEquivalent context member)

theorem adding_contexts_only_refines
    {Context History Observation : Type}
    [BEq Observation]
    (observe : Context → History → Observation)
    (smaller larger : List Context)
    (left right : History)
    (included : ∀ context, context ∈ smaller → context ∈ larger)
    (equivalent : ContextEquivalent observe larger left right) :
    ContextEquivalent observe smaller left right := by
  intro context member
  exact equivalent context (included context member)

structure TimedEvent where
  id : EventId
  time : Nat
deriving BEq, DecidableEq

def eraseAncestry (events : Array Event) : Array TimedEvent :=
  events.map fun event => { id := event.id, time := event.time }

def ancestryWorldA : Array Event := #[
  { id := 1, time := 10, parents := #[] },
  { id := 2, time := 12, parents := #[1] },
  { id := 3, time := 12, parents := #[] }
]

def ancestryWorldB : Array Event := #[
  { id := 1, time := 10, parents := #[] },
  { id := 2, time := 12, parents := #[] },
  { id := 3, time := 12, parents := #[1] }
]

theorem timing_alone_does_not_identify_ancestry :
    eraseAncestry ancestryWorldA = eraseAncestry ancestryWorldB ∧
    descendsFrom ancestryWorldA 1 2 = true ∧
    descendsFrom ancestryWorldB 1 2 = false := by
  constructor
  · simp [eraseAncestry, ancestryWorldA, ancestryWorldB]
  · constructor <;> rfl

def sameTickEvents : Array Event := #[
  { id := 1, time := 10, parents := #[] },
  { id := 2, time := 10, parents := #[1] }
]

def sameTickWitness : Witness := {
  id := 1
  crossing := 1
  support := #[1, 2]
  openedAt := 10
  expiresAt := 10
}

def sameTickReturned : Event := { id := 2, time := 10, parents := #[1] }

theorem explicit_ancestry_orders_events_within_one_tick :
    explains sameTickEvents sameTickReturned sameTickWitness = true := by
  rfl

end TrueLearnerClosure
