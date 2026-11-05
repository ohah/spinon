# 계획 PR diff 적대 검토 30개

**검토 대상:** `plan/c11-root-html-grid-text-aspect-ratio-min-max.md`, 그 전용 계획 검토, C11 fixture, Chrome 156 사전 비교 evidence 전체. 이 검토는 계획 자체의 20개 검토 및 이후 구현 검토와 별도다.

| # | 변경 diff에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | C11.1의 기본 Root HTML route와 C11.4 opt-in fixture가 섞이는가 | 통과 — 계획과 README가 evidence 경계를 분리한다. |
| 2 | 계획이 기존 finite-positive ratio 기능을 회귀시키는가 | 통과 — unconstrained ratio subset을 회귀 경계로 명시한다. |
| 3 | 제약 확장이 정확히 2:1 ratio로 제한되는가 | 통과 — constrained item의 serialization과 typed value를 함께 고정한다. |
| 4 | owner 밖 Grid 또는 root node의 `aspect-ratio`가 허용되는가 | 통과 — non-nested direct owner 조건 및 owner 오류를 요구한다. |
| 5 | 같은 owner의 두 ratio item을 partial 성공시키는가 | 통과 — 하나만 허용하며 다중 소유자는 전체 owner를 거부한다. |
| 6 | ratio item을 `A` sibling으로 바꿔 소유자 판정을 혼란시키는가 | 통과 — source order, exact text 및 ratio NodeId를 고정한다. |
| 7 | definite width에서 wrong-axis min-height 반례가 없는가 | 통과 — 60×30 baseline과 40px minimum row를 비교한다. |
| 8 | definite width에서 max-height가 무시되는가 | 통과 — max-height 20px 장면이 있다. |
| 9 | definite height에서 min-width가 무시되는가 | 통과 — min-width 70px와 visible overflow를 고정한다. |
| 10 | definite height에서 max-width가 무시되는가 | 통과 — max-width 50px 장면이 있다. |
| 11 | min-over-max precedence가 한 축에만 우연히 맞는가 | 통과 — height와 width 모두 contradictory min/max scene이 있다. |
| 12 | no-ratio control이 computed `aspect-ratio:auto`를 유지하는가 | 통과 — 두 sizing 방향에 control을 기록한다. |
| 13 | Ahem 전용 계산을 기본 앱 font availability로 오인하는가 | 통과 — Agent Browser와 앱은 system `monospace`, Ahem은 필요 시 Rust-only로 분리한다. |
| 14 | system font control 폭을 모바일과 exact 비교하는가 | 통과 — intrinsic control 폭은 platform equality gate에서 제외한다. |
| 15 | `min-width:70px`의 overflow를 root width에 clamp하는가 | 통과 — target right edge 80px, root right edge 70px를 독립 expected로 둔다. |
| 16 | browser output을 독립 expected로 복사해 circular 판정하는가 | 통과 — expected JSON은 ratio 산식·min/max precedence로 별도 생성하고 비교 결과를 저장했다. |
| 17 | CSS Grid CRD와 Sizing WD의 상태를 안정된 표준으로 과장하는가 | 통과 — CRD/WD를 명시하고 고정 profile 비교로 한정한다. |
| 18 | browser screenshot을 pixel equality 또는 WPT로 설명하는가 | 통과 — README가 visual-only 한계를 적는다. |
| 19 | computed string과 typed `CssDimension`의 불일치를 놓치는가 | 통과 — same Node/owner/style revision과 property coherence를 요구한다. |
| 20 | CSS-invalid 선언 fallback과 valid-but-unsupported 값을 혼동하는가 | 통과 — 두 오류 경계를 분리한다. |
| 21 | `auto` min과 explicit zero minimum을 합쳐 처리하는가 | 통과 — typed defaults와 min/max longhand를 검사한다. |
| 22 | finite/range guard가 negative, NaN, infinity 또는 상한 초과를 허용하는가 | 통과 — 모든 값은 `0..=70px` bounded finite px여야 한다. |
| 23 | percentage, math, intrinsic keyword가 우회 경로로 통과하는가 | 통과 — supported-value subset과 atomic rejection을 요구한다. |
| 24 | stale style revision에서 이전 frame이 새 장면처럼 보이는가 | 통과 — partial publication 거부와 다음 valid revision recovery가 관문이다. |
| 25 | alignment, writing mode, box edges나 gap이 다른 sizing 변수를 바꾸는가 | 통과 — 기존 zero-edge, zero-gap, horizontal-tb/LTR, start 조건을 유지한다. |
| 26 | Android/iOS 실행 근거의 source identity가 다른가 | 통과 — 같은 parsed HTML source hash와 revision-bound output을 요구한다. |
| 27 | software AVD를 hardware GPU 또는 physical device로 보고하는가 | 통과 — AVD software renderer임을 명시한다. |
| 28 | iOS drawable을 compositor 제출 뒤 표면으로 과장하는가 | 통과 — pre-compositor 한계를 명시한다. |
| 29 | readback 좌표가 overflow scene의 item/background 위치와 무관한가 | 통과 — target frame, scene revision, outside-root 표본을 함께 연결한다. |
| 30 | 계획 PR만으로 기능/공식 `78/214` 상태를 승격하는가 | 통과 — 계획 PR에는 STATUS 체크를 바꾸지 않고 C11.1/C11 gate를 유지한다. |

**결과:** 변경 diff에서 수정이 필요한 결함은 발견되지 않았다. 이 30관점은 계획 PR 검토이며 구현 PR diff의 30관점 검토를 대체하지 않는다.
