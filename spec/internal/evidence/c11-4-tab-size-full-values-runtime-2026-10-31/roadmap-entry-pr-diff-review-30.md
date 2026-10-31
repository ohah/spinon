# C11.4 full computed `tab-size` roadmap entry diff review · 30 checks

**Scope:** the `spec/public/roadmap.html` entry added for already merged PR #893. This review checks the public-facing roadmap copy and its links; it does not claim CSS or C11 completion.

| # | Failure path examined | Finding |
| --- | --- | --- |
| 1 | New item is inserted below older C11.4 notes | The PR #893 item is inserted before PR #885 as the newest entry. |
| 2 | The linked PR number is wrong | It links to PR #893. |
| 3 | Merge timestamp is inaccurate | It records `2026-10-31T02:54:29Z`. |
| 4 | Merge SHA is inaccurate or abbreviated ambiguously | It records full SHA `ae292fff135a925ef30a3e2437082e96c2386e9c`. |
| 5 | Merge method differs from GitHub | It states Rebase merge. |
| 6 | Empty check rollup is described as CI success | It says the rollup is empty and does not claim CI passed. |
| 7 | Required-check configuration is omitted | It says main required checks are disabled. |
| 8 | Number/length values are presented as full CSS support | It calls the change a limited opt-in fixture. |
| 9 | Scene count is inflated | It states 14 values × 10 scenes = 140 per platform. |
| 10 | Android and iOS are conflated into one matrix | It says each platform passed 140/140. |
| 11 | AVD is described as physical-device evidence | It explicitly names the ARM64 16 KB AVD. |
| 12 | iOS simulator model/runtime is missing | It names iPhone 17 Pro/iOS 26.2 Simulator. |
| 13 | Rejections or retries are omitted | It records zero rejected and zero retries. |
| 14 | Failed attempts are incorrectly included in the pass count | Accepted count refers to the successful full matrices; README separately records failed attempts. |
| 15 | Representative PNGs are portrayed as glyph proof | The text says they show route/present state only and exclude glyph pixel equality. |
| 16 | General authored CSS admission is implied | It explicitly remains unsupported. |
| 17 | Public CSS API support is implied | It explicitly remains unsupported. |
| 18 | Spinon WPT completion is implied | It explicitly remains incomplete. |
| 19 | C11.4 is marked complete | The entry says C11.4 remains incomplete. |
| 20 | C11 is marked complete | The entry says C11 remains incomplete. |
| 21 | Official implementation count changes | It retains `72/213`. |
| 22 | Plan route is broken | Link targets the RSPress `plan/c11-4-anonymous-grid-text-pre-tab-size-values.html` route. |
| 23 | Contract route is broken | Link targets `internal/0059-c11-grid-layout.html`. |
| 24 | Runtime README route is broken | Link targets the candidate evidence README route. |
| 25 | Android image route is broken | Link includes the selected run and representative PNG path. |
| 26 | iOS image route is broken | Link includes the selected run and representative PNG path. |
| 27 | Implementation PR review route is broken | Link targets the existing implementation PR diff review HTML. |
| 28 | Official status route is broken | Link targets the CSS implementation checklist anchor. |
| 29 | HTML code spans or link markup are malformed | The entry uses balanced tags within one paragraph. |
| 30 | The roadmap page gains a misleading feature/API commitment | The new paragraph records a merged candidate and its limits without changing checklist data or public API support. |

**Disposition:** no blocking issue found. The roadmap history now reflects PR #893 while preserving the official incomplete state and count.
