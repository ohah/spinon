# C11.4 hidden-item 병합 문서 동기화 PR diff 검토 30개

이 검토는 구현 PR #734의 post-merge source documentation 변경을 대상으로 한다. 앱 구현 20관점 및 구현 PR diff 30관점과 구별해 상태 대장, 상위·집중 계획, 내부 계약, 실행 증거 사이의 연결과 미완료 상태가 정확한지 각각 대조했다.

| # | 별도 문서 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 제한 hidden-item slice가 C11.4 parent checkbox를 완료로 바꾼다 | `spec/STATUS.md` checkbox는 그대로 `- [ ]`이며 diff에도 checkbox 변경이 없다. |
| 2 | 상세 완료 표시를 공식 `72/213` 카운트에 추가한다 | status, contract, plans와 README에서 `72/213`을 그대로 유지한다. |
| 3 | implementation entry가 계획 단계를 아직 미구현이라고 남긴다 | status에는 PR #734 merged time·SHA와 두-platform 실행 결과를 기록하고 계획 항목을 완료 결과에 연결한다. |
| 4 | 숨겨진 text가 미지원 `visibility:hidden` 의미로 설명된다 | status·contract는 정확한 `display:none` box-generation 사례로 한정하고 `visibility:hidden` 지원을 주장하지 않는다. |
| 5 | `display:none` node의 zero frame을 생성된 CSS box로 오해한다 | 세 문서는 internal zero-size bookkeeping frame과 CSS box/draw를 명시적으로 구분한다. |
| 6 | visible box 수와 raw frame record 수를 같은 수라고 기술한다 | Android/iOS evidence는 4 visible/drawn boxes와 hidden bookkeeping을 포함한 5 frame records로 분리한다. |
| 7 | track 기대값에 hidden text 150px contribution이 반영된다 | status·contract·plan은 visible A의 100px base와 최종 `[100,40,140]px`를 사용한다. |
| 8 | Columns와 Rows의 scene 수 또는 실행 축이 바뀐다 | evidence README의 `column-hidden`, `row-hidden` 및 각 플랫폼 10/10 결과를 대조했다. |
| 9 | 기존 8 mixed-track scene이 follow-up에서 사라진다 | 계획/상태/contract는 기존 8개에 두 hidden scene을 더한 10개 matrix라고 적는다. |
| 10 | simulator 증거를 실기기나 hardware GPU 결과로 확대한다 | 각 문서는 API 37 AVD와 iOS Simulator를 구체적으로 표기하고 real-device/hardware 범위를 미확인으로 남긴다. |
| 11 | simulator screenshot을 full pixel parity 증거라고 설명한다 | README는 screenshot geometry/presentation 범위를 명시하고 platform pixel equality나 compositor-after parity는 주장하지 않는다. |
| 12 | 미등록 scene, visible-control 또는 profile conflict 거부가 성공으로 기록된다 | status와 evidence는 세 route를 거부 결과로 적고 exact accepted scene ID를 둘로 제한한다. |
| 13 | WPT 또는 임의 CSS 지원이 완료된 것으로 읽힌다 | `STATUS.md`, 집중 계획과 contract 모두 WPT·general CSS admission을 남은 범위로 유지한다. |
| 14 | 공개 API가 추가됐거나 CSS 지원 계약이 선언된다 | `spec/STATUS.md`와 PR link는 runtime fixture만 바뀌었고 public API 변경이 없음을 유지한다. |
| 15 | 내부 contract numeric version이 불필요하게 올라간다 | 0059는 `0.1.0` 고정 상태를 유지하고 hidden slice를 기존 internal fixture 범위에 덧붙인다. |
| 16 | plan PR #733의 날짜나 merge SHA가 틀린다 | GitHub PR metadata와 계획 기록을 대조했다: `d73f71c950ae4ba64d27ebfd0c70f6e81475ec8c`, 2026-10-28T02:51:37Z. |
| 17 | 구현 PR #734의 merge time·SHA가 틀린다 | GitHub PR metadata와 status/contract/plan/evidence의 값이 `50e6286cf7b1c86a8c0398221751a42db105cb05`, 2026-10-28T04:03:05Z로 같다. |
| 18 | run ID에서 Android와 iOS를 맞바꾸거나 stale run을 인용한다 | README와 matrix JSON의 Android AVD `035340076Z-8be33a99`, iOS Simulator `035420142Z-8d77bf8f`를 대조했다. |
| 19 | 앱 hash가 다른 run 또는 플랫폼의 값으로 기록된다 | run manifest의 APK SHA-256과 iOS app SHA-256을 README 표의 platform row와 비교했다. |
| 20 | V8 dependency revision이 branch source와 불일치한다 | 두 current manifest가 required/actual V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`로 일치한다. |
| 21 | matrix JSON이 이전 8-scene run을 가리킨다 | `matrix-verification-hidden-2026-10-28.json`은 expected scene IDs 10개, platform row 10개씩과 hidden frame count 5를 포함한다. |
| 22 | self-test negative controls가 hidden case를 포함하지 않는다 | verification JSON에 일반 control과 hidden source order/text input/track contribution을 포함한 11 controls가 있다. |
| 23 | 새 module path가 근거 목록·artifact record에서 누락된다 | capture source set에는 FFI/runtime hidden modules와 projection registry가 포함되고 static artifact record에는 symbol/payload 경계를 적었다. |
| 24 | release archive 검사를 APK/IPA fixture exclusion 증명으로 과장한다 | artifact-boundary는 macOS ARM64 static FFI archive 범위와 앱 bundle 제외를 증명하지 않는다는 한계를 함께 기록한다. |
| 25 | 기본 product와 fixture positive-control 결과가 뒤바뀐다 | `llvm-nm`·`strings` 결과 hash 표에서 기본 archive는 symbol/marker 부재, opt-in archive는 symbol과 marker 10회를 나타낸다. |
| 26 | 현재 feature archive 결과를 이전 PR #729 산출물과 혼동한다 | artifact-boundary에 PR #729 historic hashes와 PR #734 follow-up hashes를 별도 절로 분리했다. |
| 27 | Clippy finding을 숨기거나 현재 strict Clippy가 실패했다고 잘못 기록한다 | 구현 review는 최초 발견한 type/borrow lint와 수정 뒤 기본·fixture Clippy 통과를 구분한다. PR #729의 과거 lint 문구는 historical context로 남긴다. |
| 28 | 계획·상위 계획·status·contract의 링크가 존재하지 않는 경로를 가리킨다 | 새 relative evidence links와 focused plan/contract anchors를 repository files에서 확인했다. |
| 29 | Tailnet roadmap이 구현 merge 이전 상태로 방치된다 | 문서 source merge 뒤 selected roadmap build/copy와 Agent Browser 검증을 별도 순서로 수행하도록 plan에 남겼다. |
| 30 | Tailnet 공개 preview 동기화를 Pages 배포나 CI 통과로 잘못 표현한다 | 후속 작업은 사용자 지정 Tailnet preview만 대상으로 하고 GitHub Pages/CI 상태는 이 PR 문서로 주장하지 않는다. |

**판정:** C11.4와 `72/213`을 바꾸지 않은 채 PR #734의 현재 source·앱 근거·한계를 문서에 반영했다. Tailnet preview는 문서 PR이 main에 반영된 뒤 별도로 확인한다.
