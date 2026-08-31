import TrueLearnerClosure

open Lean
open TrueLearnerClosure

def invalidReceipt (message : String) : CheckReceipt :=
  {
    schema := receiptSchema
    accepted := false
    resolution := "invalid"
    witness := none
    explanations := #[]
    persistentLinks := #[]
    message
  }

def emit (receipt : CheckReceipt) : IO Unit :=
  IO.println (toJson receipt).compress

def main : IO UInt32 := do
  let stdin ← IO.getStdin
  let input ← stdin.readToEnd
  match Json.parse input with
  | .error message =>
      emit (invalidReceipt s!"invalid JSON: {message}")
      return 2
  | .ok json =>
      match fromJson? json with
      | .error message =>
          emit (invalidReceipt s!"invalid request: {message}")
          return 2
      | .ok request =>
          emit (check request)
          return 0
