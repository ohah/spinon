# C11.4 generic mixed text/image runtime · 계획 PR diff 검토 20

검토 대상 diff: 계획 [`c11-4-generic-mixed-direct-text-image-intrinsic.md`](./c11-4-generic-mixed-direct-text-image-intrinsic.md) 및 독립 실패 관점 보고서 [`c11-4-generic-mixed-direct-text-image-intrinsic-plan-review-20.md`](./c11-4-generic-mixed-direct-text-image-intrinsic-plan-review-20.md).

검토는 staged PR 변경의 파일 범위와 주장·근거·게이트 정합성을 대상으로 한다. 구현/test/build/simulator 결과를 검토 통과로 혼동하지 않는다.

| # | PR diff 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 계획 문서가 변경 목적을 C11.4 parent 완료로 과장함 | 문서는 제한 fixture 경로를 명시하고 parent checkbox 및 `72/213` 유지 조건을 적었다. PASS |
| 2 | 관련 계획·내부 계약 링크가 잘못되거나 누락됨 | C11.4 상위 계획, 기존 fixed mixed slice, generic text collector, 내부 계약 0059 링크가 저장소 경로에 맞는다. PASS |
| 3 | 기준 Chrome observation이 잘못된 mixed fixture를 가리킴 | 링크는 2026-10-25 direct mixed non-spanning precomparison이며 README는 동일 `XX XXXXX`와 PNG, 네 scene을 설명한다. PASS |
| 4 | top-level 상태가 fixed adapter coverage를 잘못 설명함 | span, direct column, direct row static routes를 구별하고 신규 범위를 direct column 입력 네 개로 한정했다. PASS |
| 5 | 새 generic adapter가 known static scene을 바꿈 | 구현 순서는 static scene ID hit 뒤에 generic 경로를 두며 fixed validation 보존을 요구한다. PASS |
| 6 | unknown root/item HostNode ID가 다시 expected scene 선택에 쓰임 | 새 route는 새 HostNode IDs를 사용하고 ID-independent source/style decision을 요구한다. PASS |
| 7 | generic mixed path가 generic text collector 뒤에서 실행됨 | mixed path는 static lookup miss 후 `prepare`가 None을 내기 전 실행한다고 명시했다. PASS |
| 8 | fallback과 거부 경계가 상충함 | one-item legacy fallback과 two-text-plus-image collector rejection을 유지하고 둘 다 직접 regression control로 요구한다. PASS |
| 9 | positive topology가 모호하거나 child가 빠짐 | 정확히 두 HTML direct element, 하나의 단일-text `div`, 하나의 leaf `img`를 요구한다. PASS |
| 10 | unsupported mixed 입력이 partial contribution으로 흘러감 | signature 발견 후 source/style/revision 등 mismatch는 error이며 text-only/image-only partial result와 fallthrough을 금지한다. PASS |
| 11 | min-content 기대값이 Chrome output에 종속됨 | 5 Ahem glyph × 10px와 PNG 24px에서 `max=50px`로 계산하고 Chrome을 관찰값으로 분리한다. PASS |
| 12 | max-content 계산에서 space glyph를 누락함 | 여덟 advance × 10px에서 `max=80px`로 계산한다. PASS |
| 13 | reversed order가 placement 또는 text/image frame을 바꿈 | source order는 바꾸되 fixed row placement를 유지하고 네 장면 모두 동일 track 기대값을 갖는다. PASS |
| 14 | computed style admission이 중요한 box 조건을 빠뜨림 | root dimensions/track/gap/edges/alignment/writing/direction과 item sizing/placement/edges를 열거했다. PASS |
| 15 | stale computed style 또는 image decode identity가 섞임 | snapshot/style revision consistency와 decoded natural size, same-revision trace 검사를 완료 조건에 포함했다. PASS |
| 16 | app fixture scene ID와 HostNode ID가 같은 routing boundary로 취급됨 | 네 FFI route ID와 새로운 HostNode IDs를 분리하고 Android/iOS matcher에서 source/order/frame을 검사한다. PASS |
| 17 | test/build 범위가 부족하거나 비목표가 pass로 오해됨 | focused Rust/FFI, formatting/lint, default/fixture artifact check와 두 simulator matrix를 적고 glyph/PNG pixel·physical device 등은 제외했다. PASS |
| 18 | evidence provenance와 프로세스 cleanup이 빠짐 | source commit, artifact hashes, logs/screenshots, 실행한 app 종료, Agent Browser screenshot-link 확인 및 session 종료를 요구한다. PASS |
| 19 | CI/merge 상태가 근거 없이 pass 처리됨 | empty GitHub check rollup은 CI pass가 아니라고 명시하고 Rebase and merge 및 실제 merge state를 새로 확인한다. PASS |
| 20 | PR에 구현을 섞거나 순차 작업 선호를 놓침 | plan PR 병합 후 같은 checkout에서 순차 구현하며 worktree/subagent lane을 금지한다. 변경 파일은 계획과 두 검토 문서로 제한했다. PASS |

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0.** 이 검토는 계획 PR diff만 대상으로 하며 구현 준비가 됐다는 의미다. 구현은 계획 PR 병합 후 시작한다.
