# C06.3 계획의 독립 실패 경로 검토

- **대상:** [C06.3 절대 길이 단위 계획](../../../plan/c06-3-absolute-lengths.md)
- **기준 소스:** Stylo `0.22.0` `values/specified/length.rs`, `values/computed/length.rs`; 현재 Spinon dimension·spacing extraction과 Taffy projection
- **계획 단계 관찰:** Chrome `154.0.8037.98` 고정 oracle의 26개 노드에서 `px`, `in`, `cm`, `mm`, `Q`, `pt`, `pc` 값이 같은 CSS px serialization·geometry를 만들며, `-4.5pt`는 `-6px`, `2.54cm` custom property는 `96px`였다.
- **수정된 지적:** 초기 fixture는 9개 row를 10 CSS px 높이와 2px 간격으로 두어 마지막 row가 100px root 밖으로 나갔다. 8px 높이로 줄여 마지막 row가 `y=80, height=8`에 들어오는 것을 다시 캡처했다. 기준 fixture는 이제 지원 대상 전부를 viewport 안에서 관찰한다.
- **계획 중 추가 보정:** Stylo는 상수 `calc(1in)`을 `CSSPixelLength(96px)`로 계산해 내보내므로 이를 무조건 미지원으로 닫는 계획은 Stylo 계산을 어댑터가 뒤집는 오류였다. 상수식의 이미 해소된 typed 값은 사용하고, percentage가 남은 `calc(1in + 10%)`만 C06.5 전까지 거부하도록 계획과 검사를 바꿨다.

## 검토 항목

| # | 공격 관점 | 계획의 대응·증거 |
|---:|---|---|
| 1 | `in`을 CSS px/in이 아닌 물리 DPI로 계산할 수 있는가 | 96 CSS px/in으로 고정하고 Chrome·Stylo typed CSS px를 비교한다. |
| 2 | `cm`에서 2.54 conversion을 빠뜨리거나 잘못된 방향으로 나눌 수 있는가 | `2.54cm = 96px` 동치와 `96/2.54` factor를 fixture에서 확인한다. |
| 3 | `mm` conversion이 `cm` 기준과 불일치할 수 있는가 | `25.4mm = 96px` 및 fractional `1.5875mm = 6px`를 확인한다. |
| 4 | Q 단위를 pica 또는 다른 quarter 의미로 오독할 수 있는가 | CSS quarter-millimeter `Q` 및 `101.6Q = 96px`를 명시하고 대문자 입력도 Chrome에서 캡처한다. |
| 5 | `pt`를 플랫폼 typography point로 잘못 바꿀 수 있는가 | CSS `pt = 1/72in`만 사용하고 `72pt = 96px`를 비교한다. |
| 6 | `pc`가 `pt`의 12배와 다르게 처리될 수 있는가 | `1pc = 12pt`, `6pc = 96px`를 CSS reference에 고정한다. |
| 7 | `px`가 device pixel이나 Android dp와 동일시될 수 있는가 | CSS px 기준과 raster pixel·native point/dp 변환을 분리하고 동일 CSS frame 비교를 둔다. |
| 8 | Stylo가 이미 환산한 값에 adapter가 두 번째 factor를 적용할 수 있는가 | computed `CSSPixelLength::px()`를 읽고 별도 factor table을 만들지 않는 책임을 명시한다. |
| 9 | width·height typed extraction은 맞지만 margin·padding은 다른 단위를 남길 수 있는가 | dimension과 spacing paths를 한 reference에서 모두 검사하고 CSSOM과 frame을 대조한다. |
| 10 | `flex-basis`가 width와 다른 canonical value로 투영될 수 있는가 | `flex-basis`를 독립 관찰하고 각 절대 단위에서 48 CSS px typed value를 비교한다. |
| 11 | margin signed value에서 `pt` factor나 음수 부호가 사라질 수 있는가 | `-4.5pt`를 `-6px` typed/layout 결과로 별도 검증한다. |
| 12 | padding/gap의 유효하지 않은 음수 declaration이 내부 clamp와 섞일 수 있는가 | CSS authored invalid 처리와 내부 DTO 방어를 구분하고 기존 error contract를 유지한다. |
| 13 | 분수 단위 변환에서 오차가 평균에 가려질 수 있는가 | 각 typed 값과 node/axis frame을 각각 보고하고 0.5 CSS px별 상한을 둔다. |
| 14 | 큰 authored value가 f32 overflow 후 유한값처럼 전파될 수 있는가 | computed finite guard와 layout `NonFiniteFrame` 실패를 요구한다. |
| 15 | box-sizing/content dimension이 absolute width를 재해석할 수 있는가 | 고정 border-box row와 child box를 Chrome·Taffy geometry로 비교한다. |
| 16 | Chrome CSSOM serialization과 Stylo internal computed representation이 다를 수 있는가 | serialized CSS value와 typed DTO·used frame을 별도 assertion으로 비교한다. |
| 17 | device scale factor 1과 3에서 CSS 단위 길이가 달라질 수 있는가 | 같은 viewport의 Rust runtime 두 입력에서 CSS frame이 bit/허용치 수준으로 같은지 확인한다. |
| 18 | custom property 안의 absolute unit이 substitution 뒤 변환되지 않을 수 있는가 | `--physical-length: 2.54cm; width:var(--physical-length)`을 Chrome·V8 fixture에 넣는다. |
| 19 | resolved `calc(1in)`을 거부하거나 unresolved `calc(1in + 10%)`을 px로 오독할 수 있는가 | Stylo가 이미 `CSSPixelLength`로 계산한 상수식은 허용하고, `Unpacked::Calc`에 남은 표현식은 C06.5 전까지 거부한다. font-relative 단위는 C06.4로 분리한다. |
| 20 | reference fixture에서 보이지 않는 마지막 row가 검증에서 빠질 수 있는가 | 9-row 합계 높이를 계산하고 마지막 `pc` row가 root viewport 안에 있는지 reference geometry로 확인한다. |

**판정:** 계획을 구현 전에 고쳤다. 이 검토는 계획 위험만 다루며 C06.3 코드 구현 검토를 대신하지 않는다.
