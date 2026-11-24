# Preliminary dependency and overlap map

Snapshot retrieved 2026-11-24T11:51:11.9051768Z through 2026-11-24T11:51:36.6100229Z.

Source queries: `GET repos/Marvisak/VIctory/pulls?state=open&per_page=100` (5 exhausted pages: 100/100/100/100/45; 445 open PRs) and `GET repos/Marvisak/VIctory/issues?state=open&per_page=100` (7 exhausted pages: 100/100/100/100/100/100/34; 189 issues after filtering PR entries). Retrieval used gh api --paginate --slurp. Membership is complete for this snapshot, not a guarantee of current state.

This map records prioritization leads from public PR titles/descriptions. It does not establish code ancestry, patch equivalence, supersession, acceptance, test results, or integration. Every relationship below remains unverified against complete diffs and current exact SHAs. References may name closed items absent from the open inventory. No discussion instructions were executed.

## Explicit dependency claims

An arrow means the later PR explicitly describes a dependency or stacked base; these are author claims requiring verification.

| Claimed prerequisite | Dependent | Claim source |
|---|---|---|
| #175 | #362 | Dependent title says stacked on #175 |
| #238 | #558 | Dependent title says depends on #238 |
| #302 | #489 | Dependent title says stacked on #302 |
| #489 | #710 | Dependent title says stacked on #489 |
| #294 | #709 | Dependent title says stacked on #294 |
| #193 | #704 | Dependent title says stacked on #193 |

## Overlap hints

These are topic similarities, not established dependencies or duplicate dispositions. Compare source diffs, discussion, tests, and ancestry before choosing an implementation. A clean application alone is insufficient.

| Area | PR identifiers | Review question |
|---|---|---|
| LDS byte/half access | #101, #116, #383, #571, #632 | Equivalent lowering or competing semantic choices? |
| Wave emulation | #189, #192, #223, #233, #320, #439, #578 | Broad emulation alternatives versus refusal of unsupported wave64 programs? |
| Image sampling | #73, #138, #160, #306, #318, #360, #563, #569, #694 | Shared opcode support and coordinate/gradient semantics? |
| 64-bit image atomics | #305, #539, #705 | Equivalent implementations or distinct fixes? |
| Per-lane instructions | #98, #404, #498, #576, #627 | Instruction coverage and overlapping implementations? |
| Heap-index bounds | #483, #550, #584 | Host bounds shared across shader and GPU changes? |
| Typed null slots | #568, #581 | Same descriptor fallback with broader bundled changes? |
| Depth bounds | #158, #582 | Same host plumbing or complementary paths? |
| Indirect shader compilation | #178, #190 | Compiler fallback versus replacement writer? |
| Indirect count handling | #242, #583 | Same count-buffer behavior and bounds guards? |
| Rectangle indirect draws | #282, #346 | Alternative expansion paths and prerequisites? |
| Resource retirement | #431, #575, #609, #595 | Lifetime changes interacting with asynchronous submissions? |
| Adapter selection | #94, #208 | Same selection and capability behavior? |
| Agility runtime | #175, #463, #486, #362 | App/runtime setup versus test setup and explicit stack? |
| Linux builds/runtime | #112, #235, #523, #526, #232, #438, #505 | Alternative platform approaches and test-only changes? |
| Pipeline-cache write failures | #72, #640, #358, #389 | Retry persistence versus translation despite failed caching? |
| Config data paths | #241, #516, #708, #363 | Persistence/cache separation and overlapping config edits? |
| Narration | #82, #691 | Alternative host speech backends? |
| Formatting/QA | #17, #502, #540, #545, #547, #692 | Competing workspace churn and workflow scope? |
| PE/loader hardening | #216, #219, #324, #396, #406, #670 | Bundled versus bounded fixes and parser compatibility? |
| Save integrity | #399, #408, #570, #590 | Distinct failure phases or overlapping storage logic? |

## Preliminary queue notes

The inventory workflow labels are routing states only. #449, #480, and #708 carry `review-passed-tests-pending`; #689 carries `candidate-validation`; #629 and #396 carry `coordinator-review`; #699 carries `likely-already-present-needs-confirmation`. None denotes integration or upstream resolution. All other PRs remain `queued-unreviewed`; issues remain `awaiting-PR-pass`.
