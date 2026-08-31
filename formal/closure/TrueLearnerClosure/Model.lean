import Lean.Data.Json

open Lean

namespace TrueLearnerClosure

abbrev EventId := Nat
abbrev WitnessId := Nat
abbrev LinkId := Nat

structure Event where
  id : EventId
  time : Nat
  parents : Array EventId
deriving BEq, FromJson, Repr, ToJson

structure Witness where
  id : WitnessId
  crossing : EventId
  support : Array LinkId
  openedAt : Nat
  expiresAt : Nat
deriving BEq, FromJson, Repr, ToJson

structure Claim where
  resolution : String
  witness : Option WitnessId
deriving BEq, FromJson, Repr, ToJson

structure CheckRequest where
  schema : String
  events : Array Event
  witnesses : Array Witness
  returned : EventId
  claim : Claim
deriving BEq, FromJson, Repr, ToJson

structure CheckReceipt where
  schema : String
  accepted : Bool
  resolution : String
  witness : Option WitnessId
  explanations : Array WitnessId
  persistentLinks : Array LinkId
  message : String
deriving BEq, FromJson, Repr, ToJson

inductive Resolution where
  | noClaim
  | closed (witness : WitnessId)
  | ambiguous
deriving BEq, Repr

def requestSchema : String := "truelearner-causal-check/v1"

def receiptSchema : String := "truelearner-causal-receipt/v1"

def findEvent? (events : Array Event) (id : EventId) : Option Event :=
  events.find? (fun event => event.id == id)

def findWitness? (witnesses : Array Witness) (id : WitnessId) : Option Witness :=
  witnesses.find? (fun witness => witness.id == id)

def allUnique (values : Array Nat) : Bool :=
  values.all fun value => values.toList.count value == 1

def eventIds (events : Array Event) : Array EventId :=
  events.map (·.id)

def witnessIds (witnesses : Array Witness) : Array WitnessId :=
  witnesses.map (·.id)

def eventWellFormed (events : Array Event) (event : Event) : Bool :=
  event.parents.all fun parentId =>
    parentId < event.id &&
      match findEvent? events parentId with
      | some parent => parent.time ≤ event.time
      | none => false

def witnessWellFormed (events : Array Event) (witness : Witness) : Bool :=
  !witness.support.isEmpty &&
    allUnique witness.support &&
    witness.openedAt ≤ witness.expiresAt &&
    match findEvent? events witness.crossing with
    | some crossing => crossing.time == witness.openedAt
    | none => false

def requestWellFormed (request : CheckRequest) : Bool :=
  request.schema == requestSchema &&
    !request.events.isEmpty &&
    allUnique (eventIds request.events) &&
    allUnique (witnessIds request.witnesses) &&
    request.events.all (eventWellFormed request.events) &&
    request.witnesses.all (witnessWellFormed request.events) &&
    (findEvent? request.events request.returned).isSome

def parentFrontier (events : Array Event) (frontier : List EventId) : List EventId :=
  frontier.flatMap fun id =>
    match findEvent? events id with
    | some event => event.parents.toList
    | none => []

def descendsFrom (events : Array Event) (ancestor current : EventId) : Bool :=
  let rec visit (fuel : Nat) (frontier : List EventId) : Bool :=
    match fuel with
    | 0 => false
    | remaining + 1 =>
        if frontier.any (· == ancestor) then
          true
        else
          visit remaining (parentFrontier events frontier)
  visit (events.size + 1) [current]

def explains (events : Array Event) (returned : Event) (witness : Witness) : Bool :=
  witness.openedAt ≤ returned.time &&
    returned.time ≤ witness.expiresAt &&
    descendsFrom events witness.crossing returned.id

def explanationIds (request : CheckRequest) : Array WitnessId :=
  match findEvent? request.events request.returned with
  | none => #[]
  | some returned =>
      (request.witnesses.filter (explains request.events returned)).map (·.id)

def resolveIds : List WitnessId → Resolution
  | [] => .noClaim
  | [witness] => .closed witness
  | _ :: _ :: _ => .ambiguous

def resolve (request : CheckRequest) : Resolution :=
  resolveIds (explanationIds request).toList

def persistentLinks (request : CheckRequest) (resolution : Resolution) : Array LinkId :=
  match resolution with
  | .closed witness => (findWitness? request.witnesses witness).map (·.support) |>.getD #[]
  | .noClaim | .ambiguous => #[]

def resolutionName : Resolution → String
  | .noClaim => "no_claim"
  | .closed _ => "closed"
  | .ambiguous => "ambiguous"

def resolutionWitness : Resolution → Option WitnessId
  | .closed witness => some witness
  | .noClaim | .ambiguous => none

def check (request : CheckRequest) : CheckReceipt :=
  if !requestWellFormed request then
    {
      schema := receiptSchema
      accepted := false
      resolution := "invalid"
      witness := none
      explanations := #[]
      persistentLinks := #[]
      message := "request is not a well-formed causal trace"
    }
  else
    let explanations := explanationIds request
    let resolution := resolveIds explanations.toList
    let witness := resolutionWitness resolution
    let accepted :=
      request.claim.resolution == resolutionName resolution &&
        request.claim.witness == witness
    {
      schema := receiptSchema
      accepted
      resolution := resolutionName resolution
      witness
      explanations
      persistentLinks := persistentLinks request resolution
      message := if accepted then "claim follows from the causal trace" else "claim differs from the causal trace"
    }

end TrueLearnerClosure
