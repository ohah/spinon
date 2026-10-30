# C11.4 `auto <ratio>` PR diff 실패 관점 검토 30개

이 검토는 구현 PR에 제출할 변경 diff를 대상으로 하며 계획 문서 review나 runtime 20관점 검토와 합산하지 않는다. 기본 비교 기준은 branch 시작점 `74dce30b5be51c1d34dc977b8b64a2fdf08d101d`; 실행 영수증은 source hash map으로 현재 수정 파일 bytes에 결속했다.

| # | diff 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | 타입 변경으로 기존 ratio 호출자가 누락됨 | `layout_aspect_ratio` 사용처와 fixture/style/layout consumer를 갱신했고 workspace compile 및 targeted clippy를 통과했다. | 통과 |
| 2 | typed sizing-box enum이 불필요하게 public API가 됨 | 타입은 기존 내부 style snapshot 경계에 있고 public browser API를 추가하지 않는다. | 통과 |
| 3 | `ComputedLayoutAspectRatio` 값이 복사/비교를 지원하지 않음 | `Copy`, `Clone`, `PartialEq`를 갖고 snapshot 비교에 들어간다. | 통과 |
| 4 | ratio extraction의 호출부가 profile 인자를 잘못 전달 | cascade extraction 호출부를 profile과 함께 갱신했고 style ratio tests가 통과했다. | 통과 |
| 5 | runtime profile의 cascade allowlist가 무제한 확장 | `RuntimeBlockFlexGridTrackSizingV1` 분기만 조정하고 기존 unsupported runtime checks를 유지한다. | 통과 |
| 6 | degenerate ratio가 `None`인데 computed serialization은 숫자로 남음 | extractor 및 collector의 degenerate/serialization control이 이를 거부한다. | 통과 |
| 7 | `auto <ratio>`를 malformed serialization에서 승인 | typed ratio와 serialized numerator/denominator 및 auto token을 일치시킨다. | 통과 |
| 8 | f32 overflow/underflow 값이 layout으로 진입 | ratio와 파생 geometry의 finite/range validation 및 synthetic negative test를 확인했다. | 통과 |
| 9 | legacy direct-text `2/1` 전용 경로 회귀 | 기존 16 scene을 새 23-scene matrix에 포함해 양 플랫폼에서 다시 실행했다. | 통과 |
| 10 | generic direct-text 경로가 aspect-ratio fixture와 혼합 | `DirectTextProfile::AspectRatio` 분기로 좁히고 generic caller는 optional owner를 비운다. | 통과 |
| 11 | text node가 아닌 element를 ratio owner로 변경 | collector의 eligible HTML block/direct text shape와 exact node ownership 검사를 유지한다. | 통과 |
| 12 | `A` item에 ratio를 허용하거나 owner 위치 변경 | target `AAAA` item 및 한 owner 조건을 fixture validator와 negative tests가 고정한다. | 통과 |
| 13 | extra/nested/replaced Grid item이 새 route에서 허용 | profile 검사와 wrong-shape/extra-item tests가 fail-closed한다. | 통과 |
| 14 | 두 child의 source order 또는 tracks가 scene 전반에 바뀜 | 고정 23 scene registry 및 trace 기대값에서 item/order/track을 검사한다. | 통과 |
| 15 | nonzero margin 또는 변경된 edge가 ratio 계산을 오염 | 제한 profile은 허용 edge를 고정하고 margin/edge negative controls가 거부된다. | 통과 |
| 16 | min/max나 explicit size가 helper 전제와 불일치 | auto dimensions 및 min/max 기본값을 검사하며 비지원 조합을 거부한다. | 통과 |
| 17 | row/column track 또는 writing mode 변경이 승인 | root/track/row validator와 fixed scene checks가 범위 밖 입력을 거부한다. | 통과 |
| 18 | projection option이 모든 caller에서 임의 content-box mutation 허용 | `None` 기본값을 유지하고 전용 function만 owner ID를 제공한다. | 통과 |
| 19 | owner ID가 snapshot 밖의 stale style에 대응 | gate는 현재 snapshot 내 node/style 및 ratio basis 일치를 요구한다. | 통과 |
| 20 | projection에서 ratio 값은 맞지만 sizing basis가 틀림 | numeric `BoxSizing`와 auto `ContentBox`를 각각 확인하는 negative/positive tests가 있다. | 통과 |
| 21 | Taffy content-box helper가 padding/border를 중복 차감 | helper의 basis adjustment와 output edge restore를 alignment/item sizing 경로에서 확인했다. | 통과 |
| 22 | `ContentBox` 외 아이템이 새 Taffy helper로 영향받음 | 기존 helper로 위임하는 분기가 있으며 Grid ratio oracle 6/6 통과했다. | 통과 |
| 23 | Grid sizing의 두 계산 경로 중 한 쪽만 ratio 수정 | alignment 및 Grid item sizing 경로가 같은 helper를 사용한다. | 통과 |
| 24 | 실패 시 projection이 일부만 적용된 상태로 layout 진행 | validator가 style projection 이전에 불일치를 반환하고 negative tests가 pre-layout rejection을 확인한다. | 통과 |
| 25 | Android scene matrix가 iOS와 다른 scene을 실행 | 양 플랫폼 receipt의 scene count/IDs와 27 source hashes를 비교했다. | 통과 |
| 26 | Android pixel sample 좌표가 root 밖/clip 안을 혼동 | item-edge sample은 첫 item 내부를, blue sample은 root 외부 overflow를 검사한다. | 통과 |
| 27 | iOS pixel pass를 OS 화면 렌더링 보증으로 과장 | 문서는 drawable readback이 compositor 제출 전임을 명시한다. | 통과 |
| 28 | 기본 FFI archive 검사로 앱 번들 제외를 과장 | 보고서는 static archive marker 부재만 말하고 APK/IPA 전체는 미확인으로 남긴다. | 통과 |
| 29 | 문서가 제한 구현을 C11.4 완료로 승격 | 계획·STATUS·0059 모두 C11.4/C11 unchecked, 72/213 유지 및 미지원 범위를 기록한다. | 통과 |
| 30 | 주변 C07.3 실패를 green으로 숨김 | 실행 README와 review에는 4/5 및 exact string mismatch를 별도 open item으로 남겼다. | 통과(미해결 항목 공개) |

## 병합 전 주의

가장 중요한 잔여 항목은 인접 `c07_3_aspect_ratio_tests`의 exact computed-style serialization 비교 1건이다. 현재 구현은 해당 serializer를 변경하지 않았지만 targeted suite는 4/5다. 이 차이를 통과로 세거나 C11.4/C11 전체 지원으로 표시하지 않는다. Simulator는 AVD software renderer 및 iOS compositor 이전 drawable 범위에 한정된다.
