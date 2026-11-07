# PR #1254 변경분 실패 관점 30개

현재 PR #1254 head `c428d79979830aed43535faa94a1f69d219f632f`의 실제 source, tests, 문서, Simulator artifacts, PR 본문을 기준으로 검토했다. 계획 검토 20개 및 implementation/runtime 검토 20개와 따로 진행했다.

| # | 독립 검토 관점 | 판정 |
|---:|---|---|
| 1 | 지원 범위가 start position의 두 safety 값으로만 제한되는가 | Layout allowlist는 `Start + Safe/Unsafe` 두 typed value만 추가한다. 통과. |
| 2 | Style projection과 Layout allowlist가 같은 값 집합인가 | Computed-value allowlist와 layout predicate에 같은 두 exact value만 추가했다. 통과. |
| 3 | `safe`와 `unsafe` modifier가 computed snapshot에서 보존되는가 | Style-to-layout matrix가 각 scene의 `align-content` 문자열 exact equality를 확인한다. 통과. |
| 4 | 잘못된 CSS grammar/control이 두 accepted value를 흉내 내는가 | `safe flex-start` control은 unsupported value로 남고 `align-content` 오류를 낸다. 통과. |
| 5 | Baseline 정렬이 profile 안에 섞이는가 | Layout baseline control은 `align-content` property rejection을 확인한다. 통과. |
| 6 | Distributed value의 safety modifier까지 넓어지는가 | 기존 baseline/safety/unknown allowlist control을 유지하고 `safe flex-start`도 거부한다. 통과. |
| 7 | 기존 plain `start` positive geometry가 바뀌는가 | Precomparison·Layout·style projection·RuntimeSession의 six-scene matrix에서 `[0,10,20]px` control을 확인한다. 통과. |
| 8 | Positive-space `safe start`가 잘못 분배되는가 | `safe start`의 positive item y는 `[0,10,20]px`; Simulator bounds도 1 CSS px 이내다. 통과. |
| 9 | Positive-space `unsafe start`가 다른 위치로 분기되는가 | Computed modifier는 보존하면서 Layout/runtime y를 `[0,10,20]px`로 판정한다. 통과. |
| 10 | Overflow `safe start`가 container start에서 밀려나는가 | Chrome·독립 기대값과 두 Simulator screenshot bounds `[0,56,112]px`가 일치한다. 통과. |
| 11 | Overflow `unsafe start`가 end/negative offset으로 해석되는가 | 최종 corrected pass04에서 두 Simulator bound가 `[0,56,112]px`다. 통과. |
| 12 | Content alignment이 row track base를 키우거나 줄이는가 | Track-sizing trace는 세 row `[10,10,10]px`를 검사한다. 통과. |
| 13 | Alignment 변경으로 fixed column/explicit row/item dimensions가 바뀌는가 | Layout/style/runtime tests가 40px width와 10px height를 고정값과 대조한다. 통과. |
| 14 | `calc()` percentage가 definite block axis가 아닌 viewport를 쓰는가 | 40px definite basis에서 gap 46px 두 개와 item y `[0,56,112]px`를 확인한다. 통과. |
| 15 | Gap sizing 자체가 alignment modifier에 따라 달라지는가 | Layout gutter trace는 positive gap 0px, overflow gap 46px 두 개를 검사한다. 통과. |
| 16 | `justify-content`로 allowlist 변경이 번지는가 | 변경은 content alignment path에만 있다. Grid fixture의 other-axis computed values는 그대로다. 통과. |
| 17 | Text item 순서나 parent ownership이 바뀌는가 | Fixed empty item 다음 direct `A`, `B` frame과 paint colors를 순서대로 확인한다. 통과. |
| 18 | 다른 track/item topology가 새 지원 경로로 오분류되는가 | 기존 definite 40×40, single explicit tracks, two implicit auto rows와 child shape gates를 유지한다. 통과. |
| 19 | CSS parser/global computed admission/public CSS API가 넓어지는가 | Parser, generic CSS admission, public API와 FFI 파일은 변경되지 않았다. 통과. |
| 20 | 지원 scene의 style/layout 결과가 서로 다른 revision에서 합쳐지는가 | RuntimeSession completed snapshot과 cascade key의 revision equality를 검사한다. 통과. |
| 21 | Unsupported scene 실패 뒤 partial frame/paint snapshot이 남는가 | Rejection test는 failed state에서 completed snapshot 부재를 검사한다. 통과. |
| 22 | Failure 뒤 동일 session에서 valid alignment로 회복되는가 | Dark media rejection 다음 light media의 supported alignment가 Ready로 publish된다. 통과. |
| 23 | 기존 safe/unsafe center 동작이 함께 바뀌는가 | Layout 18/18, RuntimeSession safety 6/6에 center overflow 회귀가 포함된다. 통과. |
| 24 | 기존 safe/unsafe end 동작이 함께 바뀌는가 | Layout 18/18, style-to-layout 8/8, RuntimeSession safety 6/6에 end control이 포함된다. 통과. |
| 25 | Percentage/`calc()` row-gap regressions가 빠지는가 | Layout matrix와 RuntimeSession calc-gap 2/2가 그대로 통과한다. 통과. |
| 26 | Mobile app이 실수로 fixture-only ABI/route를 호출하는가 | 두 플랫폼 모두 기본 parsed Root HTML route에서 C11 fixture flags를 끈 상태로 실행한다. 통과. |
| 27 | Android APK가 source fixture와 다른 HTML을 포함하는가 | Build script가 APK `assets/index.html` SHA-256을 fixture SHA와 비교한다. 네 pass에서 통과. |
| 28 | iOS Simulator app이 다른 HTML이나 다른 device를 사용했는가 | App bundle HTML hash와 pinned iPhone 17 Pro/iOS 26.2 Simulator UDID를 기록한다. 네 pass에서 통과. |
| 29 | OS screenshots가 기대 위치를 틀린 root origin에서 측정하는가 | Positive start에서 fixed item top을 y=0 anchor로 사용한다. 배경색 occlusion 오류를 수정했고 Android/iOS 1 CSS px 경계가 통과한다. |
| 30 | PR에 잘못된 화면·상태가 게시되거나 공식 완료/CI가 부풀려지는가 | Agent Browser에서 PR 본문의 4개 이미지가 모두 load 완료·nonzero dimensions임을 봤다. C11.4/C11과 `78/214`는 유지하고 GitHub check/review가 없는 상태를 green으로 부르지 않는다. |

검토에서 보완한 점은 computed-style exact-preservation assertion 추가, start 화면에서 Grid 배경이 child로 가려지는 analyzer 기준점 수정, unsafe fixture title 오타 수정이다. 마지막 항목은 corrected fixture로 pass04를 재실행했다. Focused suites, Clippy, rustfmt와 full PR `git diff --check`는 통과했다. PR #1254 head에서 workflow/check/review rollup은 비어 있다.
