# C11.2.2 Grid order 계획 PR 변경 검토 30관점

대상: 새 계획, 기존 C11.2.2 계획의 후속 링크, 계획 자체의 20관점 검토, Chrome 156 사전 비교 입력·관측·이미지. 구현 변경은 포함하지 않는다. 각 관점은 현재 staged PR diff를 기준으로 검사했다.

| # | 변경 실패 관점 | 결과 |
| --- | --- | --- |
| 1 | 새 계획이 공식 C11.2.2 계획의 `order:0` 경계와 충돌하는가? | PASS — 명시적인 후속 제한이며 기존 profile 경계·상위 완료 표시는 바뀌지 않는다. |
| 2 | CSS Grid 규범 링크가 pinned version과 section을 고정하는가? | PASS — CSS Grid 2025 CRD §6.3/§8.5를 직접 연결했다. |
| 3 | 구현 목표가 auto-placement 외 paint order 전체를 약속하는가? | PASS — 비겹침 fixed scene만 사용하고 overlap/stacking paint order를 제외한다고 적었다. |
| 4 | 양수와 음수 computed `order`가 모두 비교되는가? | PASS — 혼합 scene의 A=2, B=-1, C=0을 고정했다. |
| 5 | 동률의 상대 순서가 독립 기대값에 있는가? | PASS — 두 번째 scene은 B/C가 -1이며 B가 앞서는 위치를 기록한다. |
| 6 | DOM 순서 보존과 계산 순서 정렬을 혼동하는가? | PASS — 두 순서를 분리하고 HostDocument child list를 보존한다. |
| 7 | 자동 배치가 stable sort를 요구하는가? | PASS — 안정 정렬과 stable tie scene을 계획에 적었다. |
| 8 | 부분-auto placement까지 완전 auto로 오인할 수 있는가? | PASS — authored placement 입력의 `None | is_auto()`를 사용하고 normalized partial-auto set은 사용하지 않는다. |
| 9 | `display:none` child가 item order/cursor에 끼어드는가? | PASS — hidden filtering 후 visible children만 order 대상으로 삼는다. |
| 10 | definite/mixed item의 기존 placement 순서를 바꾸는가? | PASS — order를 가진 definite/mixed placement는 reject하고 기존 definite-first 동작을 고정한다. |
| 11 | `SparseRow` 밖에서 order가 허용되는가? | PASS — Dense, template, track-sizing 및 기타 mode의 rejection을 유지한다. |
| 12 | ComputedStyle parser를 건너뛰어 authored CSS를 직접 읽는가? | PASS — Stylo computed string에서 typed `LayoutStyle.order`로 흐르는 경로를 검사한다. |
| 13 | integer parse overflow가 성공값으로 바뀌는가? | PASS — 현재 `i32` parser의 unsupported-value 오류를 negative로 고정한다. |
| 14 | Taffy child sequence와 custom placement sequence가 서로 다른가? | PASS — `grid_auto_placement`와 `calc_tree` 두 경계를 따로 변경·검증한다. |
| 15 | child reorder가 `NodeId` frame 매핑을 깨뜨리는가? | PASS — frame/node identity를 명시 검사한다. |
| 16 | Taffy track-sizing source order를 DOM source order로 오인하는가? | PASS — Taffy 내부 child vector만 order-modified 순서로 만들고 source tree는 유지한다. |
| 17 | synthetic anonymous text item이 다른 order를 받는가? | PASS — default order 0과 기존 text regression control을 포함한다. |
| 18 | fixed items가 overlapping grid area를 만들 수 있는가? | PASS — 10px tracks와 10×10px leaf, 0 edges인 두 고정 scene으로 한정한다. |
| 19 | negative margin 또는 transform이 no-overlap premise를 무효화하는가? | PASS — scene 입력은 margin/padding/border 0이며 transform이 없다. |
| 20 | 기존 C11.2.2 16-scene matrix의 hash/evidence를 바꾸는가? | PASS — 별도 order matrix를 만들고 기존 JSON을 수정하지 않는다. |
| 21 | 신규 scene ID와 source가 FFI route allowlist 밖에서 실행되는가? | PASS — 기존 opt-in C11.2.2 fixture host 경로에만 연결한다. |
| 22 | Android/iOS 실행을 실기기 검증처럼 보고할 수 있는가? | PASS — AVD와 Simulator, surface sample, compositor 경계를 분리한다. |
| 23 | screenshots가 glyph/pixel equality를 과장하는가? | PASS — box interior 표본은 colored box 위치만 판정한다. |
| 24 | Chrome 관측을 Spinon 기능 근거로 쓸 수 있는가? | PASS — 사전 비교 문서가 browser oracle와 구현/runtime evidence를 분리한다. |
| 25 | Agent Browser에서 얻은 값의 환경 버전이 빠졌는가? | PASS — HeadlessChrome 156 UA를 JSON에 보존한다. |
| 26 | HTML source와 observation JSON이 일치하는가? | PASS — 두 scene의 computed order와 frame이 입력 source 및 기대값과 맞고 JSON 구조 검사를 통과했다. |
| 27 | source hash가 현재 input file hash와 다른가? | PASS — staged HTML SHA-256과 observation JSON/README의 hash가 일치한다. |
| 28 | 브라우저 세션이 종료되지 않고 남아있는가? | PASS — close 뒤 process table에서 Agent Browser/HeadlessChrome/Chromium 이름이 0개였다. |
| 29 | 이 slice를 완료된 C11.2.2 또는 C11로 승격하는가? | PASS — 상태 대장 checkbox와 공식 `72/213` 집계는 구현 뒤에도 유지하도록 명시한다. |
| 30 | 계획 리뷰 20개를 PR diff 리뷰 30개로 중복 계산하는가? | PASS — 계획 리뷰와 이 PR diff 보고서는 서로 다른 관점을 따로 기록한다. |

결론: staged 계획 PR diff에서 blocker를 찾지 못했다. 구현 PR 전에 plan review의 authored-input gate와 `i32` overflow 확인을 새 runtime/implementation 관점으로 다시 검증해야 한다.
