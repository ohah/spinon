# C11 기본 Root HTML direct-text aspect-ratio 계획 보정 검토 20개

2026-11-05 보정된 입력·기대값·실행 증거 경계를 서로 다른 실패 관점으로 다시 확인했다. 이는 기존 구현 전 계획 검토의 재사용이 아니며 제품 코드나 PR diff 검토를 대신하지 않는다.

| # | 적대 관점 | 판정·반영 |
|---:|---|---|
| 1 | 앱 fixture의 Ahem `@font-face`가 실제 기본 route에서 거부될 가능성 | 통과 — 이전 시도가 기본 CSS input profile에서 거부된 로그가 보존되어 있고, 성공 fixture는 `@font-face` 없이 시스템 `monospace`를 쓴다. |
| 2 | 기본 앱에서 Ahem이 내장·등록된다고 잘못 가정 | 통과 — 계획은 Ahem을 제품 입력에서 제외하고, 직접 등록하는 Rust 단위 검사에만 결정적 font로 남긴다. |
| 3 | Chrome monospace 폭을 Android/iOS 시스템 폭으로 단정 | 통과 — Chrome 관측 `AAAA=24.09375px`와 플랫폼 intrinsic width를 분리한다. |
| 4 | 바뀐 font width가 definite-width ratio scene의 결론을 바꿈 | 통과 — ratio 양성 장면은 60px definite width와 2:1에서 60×30px이며, 글자 intrinsic width와 독립이다. |
| 5 | 바뀐 font width가 definite-height ratio scene의 결론을 바꿈 | 통과 — 30px definite height와 2:1에서 target width 60px이며, height-control의 자연 폭만 별도 취급한다. |
| 6 | no-ratio width control을 ratio target과 혼동 | 통과 — width control은 60×10px이고 target ratio가 적용되지 않는 별도 장면이다. |
| 7 | height-control의 자연 너비를 고정 40px로 계속 판정 | 통과 — Chrome fixture는 24.09375px, Ahem deterministic Rust fixture는 40px이며 native monospace 폭은 플랫폼별로 기록한다. |
| 8 | font metric 차이가 auto row 높이까지 바꾼다고 가정 | 통과 — 한 줄 높이는 각 fixture에서 10px로 고정하고 ratio 장면의 30px row contribution과 분리한다. |
| 9 | 짧은 `A` item의 line-height가 width-ratio row에서 무시됨 | 통과 — 기대 row는 `max(10px,30px)=30px`; width-control은 `max(10px,10px)=10px`이다. |
| 10 | fixture가 글자를 줄바꿈해 ratio geometry 외 요인을 추가 | 통과 — 두 item 모두 `white-space:nowrap`, `10px` font size와 `10px` line-height를 고정한다. |
| 11 | Grid width/track/gap 변경이 ratio 효과와 섞임 | 통과 — width `70px`, columns `[10px,60px]`, gap `0`, 단일 auto row를 기대값·hash 고정 fixture에서 사용한다. |
| 12 | 색상 식별 배경이 sizing에 기여 | 통과 — 색상은 장면 식별 전용이고 padding·border·margin은 0으로 둔다. |
| 13 | Chrome 값이 표준 기대값 또는 유일한 normative 근거로 과장 | 통과 — CSS Grid 2025 CRD 계산 근거와 CSS Sizing 2026 Working Draft, Chrome 관측을 구분한다. |
| 14 | 기대값 JSON이 수정된 fixture와 분리되어 낡음 | 통과 — 원본 fixture와 evidence copy가 SHA-256 `d44069e298552707307ddcb2116dd7758c630af28a8c943e527984c8a5f80005`로 일치하는지 manifest에서 확인한다. |
| 15 | 이미지 screenshot만으로 수치 일치를 주장 | 통과 — screenshot은 장면 확인용이며 0 CSS px geometry 판단은 observation/JSON/Rust frame assertion에 둔다. |
| 16 | 모바일 screenshot의 작은 content가 ratio 실패로 오인 | 통과 — viewport meta가 없는 기본 route의 980px layout viewport와 page scale을 별도 기록하고 CSS frame으로 판정한다. |
| 17 | simulator summary의 `boxes=14`를 개별 item frame으로 주장 | 통과 — 앱 로그의 root frame/draw 제출과 Rust default-runtime의 ID별 frame assertion을 분리한다. |
| 18 | Android software renderer를 GPU hardware 근거로 확대 | 통과 — Android backend를 ANGLE/Vulkan SwiftShader로 기록하고 hardware GPU 증거로 보지 않는다. |
| 19 | iOS pre-compositor draw를 최종 표시 pixel 증거로 확대 | 통과 — Metal backend·present log와 screenshot을 남기되 Metal readback/compositor 후 pixel 검증을 주장하지 않는다. |
| 20 | 이 후보가 C11 또는 공식 완료 집계를 올린다고 오해 | 통과 — C11 parent는 미완료이고 78/213 공식 표시 수는 이 제한 slice로 변경하지 않는다. |

**판정:** 기본 Root HTML 경로에서 실제 사용할 수 있는 입력과 Chrome/native 글꼴 비교 한계를 계획에 반영했다. 다음 gate는 변경 코드·runtime의 독립 20관점 검토와 실제 PR diff 검토다.
