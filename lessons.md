# Lessons for Future Build Outs

0a. **Establish the arrow before testing composition or naturality.**
    We saw that “ownership changed” was too vague to identify the broken law. The category-theory lens forced three questions in order: does each choice produce exactly one physical arrow, do completed arrows compose, and does a change of learner representation preserve the same arrow? The solve was to test functionality before composition and naturality. This ruled out multi-target choice and localized the first non-commuting square to a missing completed-cycle witness.

0b. **Use the trace you already have before adding another diagnostic.**
    We saw that the runtime already recorded path participation, consequence writes, learner construction, ownership, candidate decisions, and return lifetime, but the frozen artifact reduced away the surrounding evidence needed for the next question. The solve is to audit the existing trace first and preserve the decisive slice losslessly; add a new event only when the required physical fact is genuinely absent.

1. **Find the first missing physical step.**
   We saw failures that looked like missing high-level capabilities but were caused by one broken step in `input → junction → path → output`. The solve was to locate the first broken transition before proposing a larger mechanism.

2. **Temporary structure can perform real computation.**
   We saw interactions succeed through links and paths that existed only briefly. The solve was to treat transient structure as part of the computation and require it to live only as long as its physical role requires.

3. **A correct path needs the correct lifetime.**
   We saw valid paths disappear before their output fired or consequence returned. The solve was to preserve each path through the physical events that depend on it, then let it decay normally.

4. **Credit actual participation.**
   We saw retrospective reconstruction risk strengthening links that might have participated rather than those that actually fired. The solve was to let real firing record the exact links eligible for later strengthening.

5. **Create the return route while the used path is live.**
   We saw outputs fire without leaving a reliable route for their consequences to return. The solve was to form the temporary return path at the moment of actual output participation.

6. **Record causality during participation, not afterward.**
   We saw that the links and junctions needed to reconstruct an interaction could disappear before its consequence arrived. The solve was to create the necessary causal structure while those physical participants were still live.

7. **Keep output completion separate from consequence completion.**
   We saw output firing finish before its physical consequence returned. The solve was to let the output complete while keeping its used return structure reachable until the consequence completed.

8. **Fix consequence delivery before changing credit propagation.**
   We saw strength return successfully across very long paths once a real return route existed. The solve was to repair return opening and delivery rather than invent a stronger credit rule.

9. **Try ordinary consequence learning before adding curiosity.**
   We saw that an ordinary attempted path could become preferred after its real consequence returned and strengthened it. The solve was to use normal variation, return, and reuse first, adding curiosity only if evidence shows they are insufficient.

10. **Separate faster learning from necessary learning.**
    We saw that a special preference could accelerate path choice while ordinary consequence learning eventually reached the same result. The solve was to retain the simpler mechanism unless the special preference proved necessary.

11. **Measure alternative selection, not repeated firing.**
    We saw junctions fire repeatedly while selecting the same path every time. The solve was to diagnose whether another path actually participated rather than counting repeated activity as variation.

12. **Variation must expose another path to consequence.**
    We saw that randomness had no learning value unless it caused a different physical path to participate. The solve was to use bounded variation specifically to let another available path fire and receive a real consequence.

13. **Separate alternative selection from alternative lifetime.**
    We saw alternative paths get selected but disappear before their consequence could return. The solve was to diagnose selection and lifetime independently and require both transitions to survive.

14. **Path formation does not prove path execution.**
    We saw links form a complete-looking path that later input could not use. The solve was to test formation, traversal, output firing, and later reuse as separate physical transitions.

15. **Strengthened structure must still execute later.**
    We saw used paths strengthen correctly but fail to fire on the next matching input. The solve was to verify consolidation and autonomous reuse separately instead of treating strength change as learned behavior.

16. **Let physical reuse embody the learned choice.**
    We saw later input reuse the path that actual consequence had strengthened. The solve was to let that changed physical path express the policy rather than adding a separate policy representation.

17. **Diagnose output physics before blaming learning.**
    We saw valid paths reach an output that still failed because of timing, signed cancellation, strength, decay, or threshold. The solve was to instrument these physical quantities at the output decision itself.

18. **Do not let cancellation erase the experiment.**
    We saw two valid paths cancel at the output before either could participate and receive a distinguishing consequence. The solve was to give them a bounded opportunity to compete before cancellation.

19. **Compose behavior from the complete local learning loop.**
    We saw useful behavior emerge when real input formed a path, a junction chose, output fired, consequence returned, used links strengthened, and later input reused them. The solve was to compose these local transitions without inserting a planner or policy object:

    ```text
    input fires
    → links form a path
    → junction chooses
    → output fires
    → outcome returns
    → used links strengthen
    → later input reuses the path
    ```

20. **Change strengthening only after the causal chain reaches it.**
    We saw failures attributed to PQLC even though the path had not formed, fired, survived, received consequence, or become reusable. The solve was to inspect that physical sequence first and change strengthening only when strengthening itself was the first break.

# Build Lens

For every new capability, ask these questions in order:

```text
Did the input fire?

Did the needed links form?

Did they meet at the right junction?

Did a path become available?

Did the junction choose a path?

Did the path stay live long enough?

Did the output fire?

Did the outcome return along the used path?

Did the used links strengthen?

Could later input reuse the strengthened path?
```

Do not name the missing capability until you know which of these steps failed.

# Default Design Rule

Prefer:

```text
small local state
small local links
real firing
real return
local strengthening
later reuse
```

over:

```text
planner
policy object
episode memory
action history
curiosity score
global search
reconstructed causal history
```

Add a larger mechanism only when the smaller physical story has been shown not to work.

# Hand-development additions

21. **Require consequence to be born after its output.**
    We saw nearby surfaces treated as consequences even though they were already present before the output opened its return. The solve was to admit closure evidence only when that physical incidence began strictly after the exact return opened.

22. **Close sibling returns as one output cohort.**
    We saw an old unanswered sibling survive and later make unrelated coactivity look causal. The solve was to process the complete reply moment and then close the sibling returns opened by that same output.

23. **Give valid replies priority within the same physical moment.**
    We saw a newer ineligible reply prevent an older eligible consequence from being processed. The solve was to process eligible returns first while preserving stable order within each eligibility group.

24. **One physical outcome must produce one learning effect.**
    We saw simultaneous surfaces multiply strengthening and change results when their input order was reversed. The solve was to let those surfaces compete as evidence while applying only one physical consequence effect.

25. **Require physical novelty before constructing another learner boundary.**
    We saw repeated evidence from one already-owned surface/output pair create false nested learners. The solve was to require a genuinely new participating physical member before counting developmental progress.

26. **Clear the prerequisite before rerunning embodiment.**
    We saw unchanged hand trials remain uninterpretable while the learner was still constructing false boundaries. The solve was to establish temporal closure and physical boundary novelty first, then retry the identical hand.

27. **Define boundary novelty by stable physical membership.**
    We saw regenerated temporary links masquerade as new developmental boundaries. The solve was to require the child to add a participating physical junction outside the boundary already owned by its parent.

28. **Trace action after consequence writing.**
    We saw the hand construct real boundaries and write owner-local consequence but still fail to act. The solve was to stop changing memory and separately inspect reverse-path survival, motor-candidate formation, owner resolution, memory read, and output firing.

29. **Instrument the entire decision in one pass.**
    We saw live reverse paths and strong motor candidates, but narrow diagnostics repeatedly missed the later ownership failure. The solve was to record every candidate, path, origin, owner, drive, threshold, decision, and output together.

30. **Keep physical origin and learner ownership separate.**
    We saw motor candidates with two origins but only one known learner owner, making a two-owner rule inapplicable. The solve was to count origins and actual owners independently and apply ownership rules only to the ownership structure truly present.

31. **Require useful movement to remain naturally quiet.**
    We saw origin-based splitting increase hand motion while creating an endless internal feedback wave that mixed private paths. The solve was to reject movement-only success, require natural quiescence, and localize the first repeated physical cycle before adding another action rule.

32. **Represent runaway activity as its actual physical loop.**
    We saw a six-edge cycle in which one motor effect formed a path to the other motor and returned again. The solve was to record ordered edges, regions, carried origins, origin owners, and path owners until the first graph-contiguous repeated cycle became visible.

33. **Separate outward effect delivery from effect-born path creation.**
    We saw a legitimate motor effect immediately form a new path back into the organism and sustain the runaway. The solve was to preserve the outward effect while suppressing path genesis only for the firing caused by that boundary crossing.

34. **Use the same eligibility fact for every consequence consumer.**
    We saw pre-opening surfaces correctly rejected for closure but still admitted for consequence writing, allowing them to refresh the incumbent indefinitely. The solve was to apply birth-after-opening eligibility before admission, writing, renewal, consolidation, and closure.

35. **Carry physical change instead of inferring it from time.**
    We saw unchanged surfaces sampled after a return opened, so they truthfully passed the timing test and looked consequential. The solve was to classify external incidence as `Sample` or `PhysicalTransition`, carry the transition through causal lineage, and admit consequence only for a post-output transition. This rejected seventy-six unchanged samples without exposing position, direction, or hand knowledge.

36. **Use actual ancestry at an ownership boundary.**
    We saw a safe fresh-opportunity law fail because its organism-owned donor return and root-learner-owned recipient were unequal even though they met at one local physical competition. The solve was to classify their actual learner relation and admit only the typed `OrganismToRoot` case. One exact transfer then fired the silent motor and released upper contact without allowing arbitrary cross-owner transfer.

37. **Boundary release is not cumulative control.**
    We saw one safe alternative fire, produce both directions, and leave upper contact while the hand still never reached the lower limit. The solve was to preserve the whole step trajectory and inspect the first loss of continuation rather than treating release as a completed joint controller.

38. **Do not confuse complete candidates with effective competition.**
    We saw complete paths, executable owner-local candidates, consequence reads, and both motor outputs at the first post-release stall; the opposing effects canceled inside one external cycle. The attempted solve was a transient preference for the unique unanswered incumbent carrying a current owner-local physical transition. It reduced opposing cycles from six to four but did not increase real movement, proving that pre-write continuation alone is insufficient.

39. **Preserve world-effect order, but keep counterfactual adapters out of capability claims.**
    We saw outputs from separate naturally quiet learner runs combined into one batched force update, erasing the fact that the first effect should change what the next run senses. The solve was an explicit sequential composition arm that applied each quiescent run before resampling; it increased actual transitions from nine to twenty-one. Because Academy defines the official hand force as batched, this result diagnoses a composition mismatch but does not prove the official hand capability.

40. **More physical transitions are not cumulative control.**
    We saw sequential composition and unresolved-effect coherence compose into twenty real transitions with only three opposing-output steps, yet nineteen positive phase effects versus eight negative effects still saturated the upper side and never reached the lower limit. The solve was to retain the full ordered phase trace and require both contacts and escapes, revealing that the remaining failure is sustained sign coherence rather than missing activity.

41. **A composable action–effect cycle is not yet a coherent trajectory.**
    We saw a truthful movement return lose its successor when its transaction closed or learner ownership changed. The solve was to continue exactly one candidate carrying the uniquely latest recent real-transition consequence: it composed nine cycles, crossed ownership views twice, and raised official batched movement from eleven to twelve while samples, ambiguity, and stale evidence still released. But the hand reached neither limit, showing that this solves local cycle transport, not the longer problem of keeping successive physical arrows aligned.

42. **Check that a choice is one arrow before asking whether it stays natural.**
    We saw twenty-three local hand choices and first checked whether any choice admitted several physical targets. None did. The solve was a group-level admission trace and a pure comparison across ownership views. It found the first break exactly: an organism-owned choice of target eleven became learner-two's target ten because the new view had no completed-cycle evidence and used a fresh alternative. Other ownership changes preserved their target, so the problem is the missing construction-boundary witness—not ownership change by itself, and not permission to copy the parent's memory into the child.

43. **Compose trace arrows instead of equating their endpoints.**
    We saw the tick-eight consequence written on links thirty-five and thirty-six at internal junction eighteen, while the completed output chosen later was target eleven. A direct `junction == output` diagnostic therefore found no witness even though the retained trace showed link thirty-six carrying junction eighteen into target eleven and still completing that path at tick twenty-three. The solve was to preserve the failed frozen arm and define the next diagnostic as a pure composition of consequence write, uninterrupted link identity, downstream drive, construction, and owner-local read—not to add another event or rerun the organism.

44. **Construction can hide a witness without destroying it.**
    We saw link thirty-six keep generation one, avoid deallocation, and complete target eleven at ticks eleven, fifteen, nineteen, and twenty-three. At tick twenty-three it still physically participated under learner two, but learner two had no matching private consequence write and read no consequence for target eleven. The solve was the compositional offline fold that separated physical lifetime from owner-local visibility. This proves an owner-projection gap; it does not justify copying the parent's memory. The next solve must transport only the same currently participating physical witness across the construction view change.

45. **Compose the existing outcome through construction before adding state.**
    We saw links thirty-six, forty-four, and forty-five already carry tick-sixteen consequence when learner two formed in that same physical moment, but the owner-local write had happened before learner two existed. The solve was to record only live links in the learner's exact construction lineage whose existing consequence tick equaled the construction tick, using the existing learner memory and preserving tick sixteen. At tick twenty-three target eleven changed from `Missing` to `Stale`, proving that no held path, new outcome, or new memory type was needed for this local square.

46. **A repaired local square can expose a later composition change.**
    We saw the construction projection repair the exact tick-twenty-three witness while target ten and the external hand trajectory stayed unchanged, yet completed-cycle admissions increased from nine to ten later in the run. The solve was to reject clean integration, freeze both failed arms, and require the next artifact to retain every existing construction projection and completed-cycle decision until the first extra admission is identified. Do not change recency, selection, or lifetime before that first downstream divergence is known.

47. **Retain the evidence before assuming how two traces align.**
    We saw the extra admission first appear at tick forty-seven after learner three formed at tick forty-four, but it changed the chosen arrow from parent target eleven to candidate target ten and added a twenty-fourth later choice. The comparator correctly refused to pretend the unequal sequences were identical, and the retained choices still exposed the first behavioral change without another hand run. The attempted solve retained the physical trace only after successful alignment, so it lost the exact completing link on this failure path. The solve now is to retain the full existing trace unconditionally, then align choices only by shared physical tick and phase before composing links thirty-three, thirty-four, and forty-one into target ten.

48. **Judge a projected outcome by the arrow it actually completes.**
    We saw learner three receive tick-forty-four consequence on links thirty-three, thirty-four, and forty-one, while several links later drove target ten. Only link thirty-four kept generation three through construction, completed target ten at tick forty-seven, and appeared in that admitted arrow's causal lineage. The solve was to intersect construction projection, completing drive, and same-link same-generation lineage over the unconditionally retained 2,109-event snapshot. This localized the tenth admission as a real recurrence of the construction-naturality repair, not unrelated leakage. The old nine-admission equality check is therefore a stale parent contract to revise in a separate campaign, not a reason to suppress link thirty-four.

49. **Keep the cause of every candidate, not only the final winner.**
    We saw three completed cycles born directly from construction: learner three used link thirty-four generation three at tick forty-seven, learner four used link forty-six generation seven at tick ninety-five, and learner five used link thirty-four generation three at tick one-hundred-three. The last completed-cycle candidate was physically valid even though coherent-effect priority selected the other output, so final action alone would have erased its cause. The solve was to attach the sorted live completing link generations that supplied each candidate's existing consequence tick to the diagnostic event, without changing ranking or memory. All three construction arrows then factored exactly, the revised ten-admission/twenty-four-choice contract survived, and the old nine-admission contract was retired.

50. **A repaired transition can still fail the complete cost contract.**
    We saw bounded first-use construction continuation do exactly what it predicted at the first wall: learner two used link thirty-six generation one with the original tick-sixteen outcome, target eleven won at tick twenty-three, the hand moved from plus three to plus four, and the held state became ordinary without later stale reuse. The same hand reached and escaped the upper boundary with replay, natural quiet, and zero exhaustion, but comparisons rose from 5,370 to 7,320 and failed the frozen cost bound. The solve was to preserve the negative complete verdict, keep the law opt-in, and localize parent-versus-candidate work on the shared prefix before testing another behavior rule or claiming integration.

51. **Distinguish a work difference from positive overhead.**
    We saw parent and candidate match exactly through the first three shared steps, then differ by minus one comparison and minus eleven scans on the repaired step before the external inputs diverged. A frozen classifier treated any nonzero delta as direct overhead and therefore falsified its exact-match arm, even though the mechanism was cheaper at that step. The solve was to preserve that negative arm while composing signed regional deltas: `-1 + 1,951 = +1,950` comparisons and `-11 + 266 = +255` scans. This proved the failed positive excess begins in the changed downstream trajectory, not in the bounded read and consume.

52. **Measure new work against the activity that caused it.**
    We saw the repaired hand reach upper contact and receive more real inputs than its parent. Those inputs created larger but finite causal waves: the four major spikes each had more drive delivery and physical work, while step eleven also constructed two learners. The solve was to split every hand step into return and current-input phases and partition every comparison into two exact scheduler sources. All downstream excess became `1,274` searches for the next causal event plus `677` choices inside equal-time buckets, with natural quiet and zero exhaustion. This ruled out hidden memory overhead, dormant scanning, and runaway; the remaining integration question is scaling per real causal activity, not equality with a parent that never reached those inputs.

53. **Let the current physical arrow finish before an old path answers again.**
    We saw the scheduler repeatedly search an already identified causal wave, while the hand's real movement return let an older motor path answer before the new consequence could guide reuse. The solve was to extract each causal wave once, admit only the balanced path carrying the current owner-local physical transition, prefer its unique current return or latest actual outcome, and release an unanswered path at unchanged contact only when several anonymous path origins expose an executable alternative. Comparisons then scaled linearly, and the unchanged anonymous joint reached and left both limits, recovered from perturbation, replayed exactly, and became naturally quiet. This is development evidence for the opt-in law, not frozen authority or accepted-default adoption.

54. **Factor choices by connected causal world, not by the labels currently riding an arrow.**
    We saw two anonymous joints work alone but suppress one another inside one harness. Immediate-origin and completed-path-origin partitions then split the two opposing alternatives of one unchanged joint and made them cancel at contact. Connected outcome topology preserved those alternatives, but the trace exposed one remaining global fresh-opportunity choice at the shared reversal. The solve was to use the same existing connected outcome-source component for both output competition and opportunity allocation: choices remain ordinary inside one component and compose across disconnected components. Two joints then closed before five ran; all five reached and left both limits, recovered under proximal perturbation, replayed exactly, quiesced naturally, and stayed activity bounded. This is development evidence, not adopted authority.

55. **A product law needs every independently acting factor.**
    We saw that removing output factorization made the proximal joint disappear at the first two-joint choice, while removing fresh-opportunity factorization let both joints start but made the distal joint lose its reversal at step five. Both removals still preserved the exact one-joint hand, so these were the intended multi-component failures rather than broken inheritance. The solve was to keep both choices local to the same connected outcome component: each disconnected component selects an output and receives its own bounded opportunity, while alternatives inside one component still compete normally. The complete law alone closed two and then five joints; the two failures show that both halves are separately necessary in this world.

56. **Promote the physical law, not the shape of its test.**
    We saw the clean complete candidate reproduce the generic connected-component fixture, the exact one-joint arrow, two closed joints, and then five closed joints in one fresh frozen run. The two single removals also repeated their distinct failures, while the old origin partitions still failed one-joint identity. The solve was a narrow authority boundary: accept connected-component output and opportunity composition as the parent law for the next hand ladder, but do not turn the serial test harness into a claim about fingers, grasping, keyboards, touchpads, monitors, or arbitrary bodies.

57. **State one physical opportunity once before adding another choice law.**
    We saw the workstation describe one generic chance to move as thirty separate phases and physical origins, so the accepted product truthfully composed them and moved all five fingers together on forty-six of forty-eight steps. The proposed solve was a new wave-scoped component selector, but the downward arm showed the existing causal-origin choice already provided the needed coproduct. The solve was only to give the generic opportunity one phase and one origin. The unchanged learner then produced ten isolated-finger steps spanning all five digits, zero five-finger steps, exact replay, natural quiet, and lower work; the larger core mechanism was stopped.

58. **Carry the output cause through the receptor that changed.**
    We saw stable receptor identity and delta incidence still choose a fresh opposite on the second palm-depth step: the receptor said where the change was observed, but not which output caused it. The solve was to use the existing physical input span with the output-specific outcome as origin and the changed proprioceptor as target. The second step then continued forward, the first boundary released, replay stayed exact, and no held path, new learner memory, desired direction, or hidden pose was added.

59. **A shared physical object needs both connected topology and local controls.**
    We saw palm horizontal, vertical, and depth move as three disconnected products even though all three arrows act on one palm. A shared upstream anchor alone changed nothing because the controls remained eight locality units apart. The solve was a research morphology with one palm anchor and co-located translation controls and proprioceptors. Ordinary connected-component competition then emitted one palm translation arrow instead of three; the core choice law did not change.

60. **One boundary-release step is not sustained reversal.**
    We saw the palm leave depth zero for depth sixteen and immediately fall back to zero. At the next choice, the new increase path had the newer real consequence at tick 424, but the older decrease path from tick 412 was much stronger and won at tick 429. The solve was to replace the weak “left the boundary once” check with a retained two-step reversal witness and a compact candidate/choice projection. This localized the remaining wall to transport of the returned outcome, not movement, opportunity, morphology, or receptor delta.

61. **Do not let organism transition ancestry create control.**
    We saw a broad organism-view continuation freeze the palm at `(528, 768, 256)`. Restricting it to executable candidates still drove a boundary scan that ended permanently at `(512, 0, 1023)` outside every device surface. The solve was to reject and remove both arms: organism ancestry may not supply opportunity or globally override ordinary choice merely because it is recent. Both counterexamples remain as negative plans.

62. **Keep an actual outcome until its first executable choice, not for an arbitrary tick count.**
    We saw the reversed movement write a real outcome at tick 424, encounter only blocked candidates at tick 426, and reach its first executable competition at tick 429. The transition lineage had correctly ended, while the completed-cycle resolver discarded the outcome because its fixed four-tick window expired one tick earlier. The solve was to stop transition-ranking patches and state the next required object precisely: an outcome must remain available until its first eligible local choice and then be consumed exactly once, without extending stale reuse or guessing another raw-tick window.

63. **Outcome lifetime must cross the boundary that the outcome actually crosses.**
    We saw a run-local hold emit no witness because tick 424 and tick 429 belonged to different `send_physical` runs. The retained trace then showed the real composition directly: consequence recording wrote link 2509 at tick 424, and link 2509 generation 24 was still the increase candidate's exact completed-cycle witness at tick 429, but `unique_latest_tick` was empty only because age-based eligibility had expired. The solve was to remove the run-local arm and keep the consequence/completed-cycle projection. The next mechanism must attach one-use availability to the existing link outcome across runs, not hold a path or copy an outcome into transient adapter state.

64. **Give first-choice lifetime only to outcomes caused by physical change.**
    We saw a broad link lifetime change the very first workstation step because initial ordinary samples created many consequences and were incorrectly carried into later choices. The solve was to use the already-retained physical-incidence lineage: only a consequence whose causal lineage contains a real `PhysicalTransition` receives one-use availability. Samples may update history, but they cannot create, refresh, or erase that available action outcome.

65. **Consume the action outcome at the next real choice.**
    We saw the transition-bearing outcome survive natural quiescence and separate `send_physical` calls, while the parent protocol still lost link 2509's tick-424 witness before tick 429. The solve was to keep the exact outcome tick on its live link generation until the first participating executable local choice, then clear only the availability and retain the history. The candidate moved the palm straight from depth 256 to 560, reached real surface contact at sequence 19 with four fingertip pressures of eight, pressed four proper keys at sequence 26, changed text to `]\\`, began releasing at sequence 29, and released every key by sequence 61. It repeated no full session state and exactly replayed both contact and final release from checkpoints.

66. **A visible world difference must survive the organism's sensory map.**
    We saw two real illuminated keys change both binocular images from sequence zero while the sparse retina returned the same twenty-four values for sixty-four steps, so every later choice, movement, and accidental key press was identical. The solve was a research-only symmetric wide retinal lattice plus one retained paired projection from image hashes through exact retinal values, learner fingerprints, choices, movements, and device events. The two cues then diverged in the retina and learner at sequence one while the accepted hand kept its exact sequence-61 release. This solved visibility without leaking a key ID, coordinate, target, direction, or verdict; it also exposed the next wall honestly: the learned visual distinction still does not participate in an executable motor choice.

67. **Preserve ordinal change through the receptor threshold.**
    We saw real ANSI keys 26 and 87 brighten receptors on opposite sides of each eye, yet unit retinal transitions produced identical candidates; sending impulses one and three changed the learner but still produced no candidate, choice, or movement divergence through sixty-four steps because the receptor threshold collapsed both impulses to one firing. The solve was to preserve signed retinotopy and factor each ordinal change through only the intermediate bin junctions it physically crossed. The final-bin incidence carried the output-specific transition once, while intermediate thresholds remained ordinary samples. Retina, learner, candidate, resolved choice, and movement then all diverged at sequence one; removing visual return, retinotopy, or threshold factorization removed the result. This proves cue-dependent visual motor choice, not intended key selection or a successful key press.

68. **Name learner change with a learner-only fingerprint.**
    We saw a diagnostic called learner divergence change when only retained retinal adapter state differed because it hashed the complete workstation checkpoint. The solve was a separate fingerprint of the canonical core learner checkpoint while retaining the whole-body fingerprint for replay. The paired trace can now distinguish sensory or adapter divergence from an actual learner-state change without hidden position, direction, key, or evaluator knowledge.

69. **Project one retinal cause through the body instead of duplicating the world input.**
    We saw opposite real-key cues choose opposite eye movements while leaving the palm trajectory identical, because every retinal junction was local only to its own eye axis. The solve was one fixed body-internal anatomical link from each signed horizontal retinal junction to the matching palm-horizontal local field. The same admitted retinal firing then fanned out into eye and palm paths, moving the palm left for key 26 and right for key 87 without admitting the cue twice or exposing a key identity, coordinate, target, or desired direction.

70. **Seeing the target centered is not the same as the hand reaching it.**
    We saw both palms enter the intended real key's horizontal span and then escape to a body boundary; adding an ordinary `(0, 0)` foveal receptor made the aligned surface visible but did not close the reach. The solve was to preserve that arm as a killing falsifier and narrow the claim to visual-reach initiation. A later closed-loop solve needs a physical relation between the seen surface and the hand, not a hidden stop rule or another copy of target position.

71. **Use the smallest factors that commute under a mirror.**
    We saw an outward-only binocular fixture reward the body's default first eye movement, while the preregistered product mixed spatial placement, movement-caused return, and light-threshold factorization and then failed the mirrored near relation. The solve was to mirror the external eye relation, measure separation only at best joint alignment, and remove the unrelated intensity factor. Signed retinal placement composed with each eye's own movement-caused visual return then reached exact joint alignment in all six outward and inward far, middle, and near cases; collapsing placement or removing visual return broke the closed product. This proves bounded alignment acquisition, not stable fixation or general depth perception.

72. **Put an identity after every source that can produce the effect.**
    We saw exact binocular alignment depart through ordinary generic opportunity, then saw an opportunity gate still fail because a learned retinal path emitted the same directional eye output with zero opportunity. Removing the centered visual return also left that path intact. The solve was to compose at the common physical boundary: when an eye's own ordinary center receptor is bright, every horizontal output for that eye produces the identity body effect. Internal choices remain visible, but the gaze does not move and no false movement consequence returns. Both eyes then held all six mirrored far, middle, and near alignments for the remaining twenty-nine to forty-seven steps, while dark eyes and an off-center other eye still explored. This proves bounded fixation on a centered bright feature, not natural-image correspondence or general stereo vision.

73. **Separate “still needs action” from “changed because of action.”**
    We saw exact action-return closure solve one small sensor cycle but leave the containing body stuck: one disturbance stopped at two and the other at minus four. We then saw a worsening change get false credit when the persistent drive copied the motor-caused `Transition`. The solve was to let the body supply a local normal relation, curry it into one typed-observation-to-residual transformation, and give the residual physical memory in attached tissue. Nonzero persistence enters the drive surface as an ordinary `Sample`; a smaller residual returns through threshold falls; a larger residual stays on a separate rise path; zero adds no drive. The same transformation covered finger, hand, binocular, acoustic, vocal, mixed, and held-out spatial values, and ordinary learner physics regulated both directions even when motor effects were swapped and the body's normal band moved. This proves the reusable calibration shape and bounded scalar regulation, not real sensory or morphological competence.

74. **Prove each calibration factor by removing it, and distinguish transit from holding.**
    We saw four independent removals cause four distinct failures. Without persistent drive, the body moved from three to two once and stopped with residual one. When worsening and improving change shared the return surface, the move from minus three to minus four received false credit. When zero still drove, an already-normal body acted. With the wrong fixed body context, the organism eventually settled at one instead of its shifted norm. The first context oracle was itself wrong because the transient path `[3, 3, 2, 2]` spent four observations inside the shifted band before continuing to one. The solve was to preserve that failed arm and add a terminal-residence successor: complete calibration ended inside `[2,3]`, while fixed calibration ended `[1,1,1,1]` inside its old `[-1,1]` relation. All four factors are therefore separately necessary in this bounded scalar world; this still does not prove real morphology competence.

75. **When an authority check is wrong, preserve it and replace the contract—not the result.**
    We saw the first clean authority run pass every calibration law, all six complete regulation cases, all four removal walls, replay, natural quiet, returned causes, and the workspace regression. It still stopped because a raw text search found the letters `ear` inside `CLEAR` and `truelearner`; that was not hidden ear knowledge, but the frozen check could not tell the difference. The solve was to record that run as inconclusive, leave it untouched, and freeze a new one-shot protocol that looked for exact identifier components. The unchanged clean subject passed again, including controls proving that real forbidden terms are found while `TRACE_CLEAR_PHASE` and `truelearner_core` are not. This makes the four-factor calibration law authoritative only for its generic transformation and bounded scalar regulation world; real hands, eyes, ears, voice, learned norms, and collective sensor competence still need their own evidence.
