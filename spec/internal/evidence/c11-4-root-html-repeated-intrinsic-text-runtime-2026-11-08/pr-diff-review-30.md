# C11.4 repeated intrinsic text PR diff 30관점 실패 검토

**대상:** candidate diff 전체(code, tests, fixture, Chrome oracle, 내부 계약, 계획, Android/iOS simulator evidence). 계획 검토 및 구현/runtime 검토와 별도의 30개 관점이다. **결과:** diff 일관성 30/30 통과. Android runtime은 수정된 API 37.0 AVD에서 통과했으며 제한은 screenshot visual smoke다.

| # | 독립 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | 반복 profile의 anchor 위치가 임의 순서로 열리는가 | Style/Layout 모두 첫 member를 bounded `fit-content()`로 고정하고 정확한 6-member list만 받는다. |
| 2 | tail multiset count가 잘못되는가 | helper가 `min-content×2`, `max-content×2`, `auto×1`만 인정하며 test matrix가 30개 고유 order를 생성한다. |
| 3 | unsupported cap이 typed cascade에서 유실되는가 | finite non-negative px/direct percentage typed values만 인정하며 `calc`, negative, non-finite, indefinite basis는 profile 밖이다. |
| 4 | computed CSS serialization이 typed order와 다르게 기록되는가 | 270-scene test가 typed list와 `grid-auto-rows` computed string을 각각 확인한다. |
| 5 | text profile이 Grid geometry를 과도하게 넓히는가 | 단일 40px column, 10px explicit row, definite 100px overflow 또는 220px comparison height로 제한된다. |
| 6 | Grid placement·gap·edge·overflow 차이가 허용되는가 | row-sparse, zero gaps/edges, static/LTR/visible overflow 계약을 projection과 Layout이 검사한다. |
| 7 | 추가·누락·재정렬 item이 같은 fixture로 판정되는가 | direct child 9개와 payload 순서를 고정한다. 30 scene에서 DOM item order와 expected array order를 유지한다. |
| 8 | 첫 explicit item에 descendant나 크기 변이가 허용되는가 | empty 10px item shape를 확인한다. 기존 profile의 다른 item shape는 변경하지 않았다. |
| 9 | 첫 item `min-height:0` 예외가 새 profile 밖으로 새는가 | 예외 인자를 repeated-text 판정에만 연결했다. 기존 4-member negative regression을 복구 후 재실행해 통과했다. |
| 10 | payload wrapper의 중첩 HTML이 text source identity를 흐리는가 | div payload 직계 text node 하나만 받는다. inline span mutation은 실패하고 partial output이 없다. |
| 11 | payload 문장별 order/punctuation 변경이 잘못 받아들여지는가 | 여덟 고정 문자열을 DOM order로 매핑하고 wrong-text mutation이 fail-closed한다. |
| 12 | font family/size/weight/style/line-height가 변해도 같은 geometry로 처리되는가 | computed-style profile이 각 값을 exact match한다. wrong family/size/line-height mutation을 확인했다. |
| 13 | whitespace/word-break/direction/spacing/writing-mode 차이가 섞이는가 | normal wrapping·LTR horizontal 및 zero spacing contract를 검사한다. `nowrap` mutation이 거부된다. |
| 14 | font fallback line wrapping이 기대와 다른데도 pass하는가 | Rust matrix에서 payload별 glyph 수와 baseline line 수를 exact match한다. iOS·Android screenshot은 visual smoke로만 적었고 Android 독립 line-wrap oracle 검증은 주장하지 않는다. |
| 15 | intrinsic row 높이가 per-order constant로 강제되는가 | 반복 text 경로는 existing paragraph measurement/contribution을 사용하고 matrix가 실제 row/item/payload frames를 비교한다. |
| 16 | cap `0%`가 zero-height frame을 잃거나 payload paint를 지우는가 | 270-scene matrix가 zero item frame presence와 payload paint snapshot을 검사한다. |
| 17 | cap `120%` 또는 percentage basis가 잘못되는가 | 220px definite basis의 120% 장면을 네 cap cohort에 넣어 Chrome frame과 대조한다. |
| 18 | `align-content:start`의 free space가 implicit row에 들어가는가 | start 장면의 track/frame expected values를 exact track order 및 0.01px geometry 기준으로 비교한다. |
| 19 | stretch가 intrinsic track 전체에 잘못 퍼지는가 | computed expected rows로 auto-track positive free-space distribution을 각 tail order에서 확인한다. |
| 20 | Grid 밖 visible overflow가 clip/clamp되는가 | 100px Grid overflow 30 scene을 별도 비교하고 outer frame 아래 payload geometry를 검사한다. |
| 21 | 1/64 quantization이 unrelated profile에 적용되는가 | helper는 RootHtml fit-content repeated-list shape와 고정 text source가 맞을 때만 실행한다. Root HTML fit-content RuntimeSession regression 23개와 Layout 14개가 통과했다. |
| 22 | float error 때문에 경계 바로 아래 1/64 unit으로 내려가는가 | `f32` epsilon guard를 두고 270 scene 전체를 ±0.01px로 비교한다. |
| 23 | 누적 row boundary가 자체 float height 합으로 드리프트하는가 | 다음 sibling top을 다음 경계로 사용하며 마지막 row만 자체 frame bottom을 사용한다. |
| 24 | frame normalization 뒤 payload/flow frame이 item과 어긋나는가 | item subtree의 `frames`와 `flow_frames`를 함께 이동하고 RuntimeSession matrix에서 payload paint/layout frames를 비교한다. |
| 25 | ordinary layout마다 node map/hidden tree를 추가 계산하는가 | repeated profile 존재 여부를 먼저 검사해 다른 layout은 allocation 이전에 early return한다. |
| 26 | style/layout generation identity가 바뀌거나 섞이는가 | expected test에서 completed layout key와 cascade key의 동일성을 확인한다. |
| 27 | invalid text/style이 partial draw/layout을 publish하는가 | mutation suite는 node/property 진단, Failed state, `completed=None`을 검사한다. |
| 28 | same-session recovery가 실패하거나 stale state를 재사용하는가 | dark mode reject 뒤 light mode로 돌아오는 동일 session 회귀가 Ready를 확인한다. |
| 29 | oracle·fixture·screenshot가 서로 다른 revision을 설명하는가 | fixture SHA `923a9d831a6509c1eb577428296535369146034336ac5af6658af0506157b8a9`, 270 scene JSON, precomparison README, iOS screenshot, Android APK asset과 Android screenshot을 대조했다. Chrome expected matrix는 30 order·8 text descriptor다. |
| 30 | PR 문서가 미검증 플랫폼이나 완료 상태를 과장하는가 | iOS와 Android 모두 build + layout-ready + OS screenshot visual smoke로 기록하고 Metal/PixelCopy readback 또는 hardware GPU 주장을 하지 않는다. 공식 `78/214`, C11.4/C11 checkbox, 내부 version `0.1.0`은 변경하지 않는다. 실패 AVD는 근거 보존 후 제거하고 검증용 API 37 AVD는 종료 상태로 유지한다. |

변경 crate의 3/23 RuntimeSession, 14 Layout, 7 Style projection focused tests, Clippy `-D warnings`, rustfmt, Android API 37.0 runtime 및 iOS Simulator 결과는 [runtime evidence](./README.md)에 있다. GitHub PR에는 현재 status check이 없다.
