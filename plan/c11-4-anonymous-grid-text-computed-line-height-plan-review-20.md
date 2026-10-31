# C11.4 anonymous Grid text computed `line-height` 계획 적대 검토 20개

계획 PR 전에 서로 다른 실패 경로로 범위·기준·성공 조건을 검토했다. 이 문서는 계획 검토이며 구현/runtime 결과를 대신하지 않는다.

| # | 실패 관점 | 계획의 방어 또는 처리 | 판정 |
| ---: | --- | --- | --- |
| 1 | 제한 fixture 결과를 일반 authored CSS 지원으로 확대 | opt-in `RuntimeBlockFlexGridTrackSizingV1` anonymous direct-text scene에만 허용하고 일반 CSS admission을 명시적으로 제외 | 통과 |
| 2 | 장면 ID가 입력 CSS 값을 대체해 값 변경이 무시됨 | computed style snapshot에서 정확한 Grid root의 `line-height`를 읽고 scene-selected scalar 제거 | 통과 |
| 3 | Stylo의 number 직렬화가 예상과 다른데 px parser가 오판 | 구현 전 style projection test로 `computed_value_to_string` 출력을 고정하고 number·px grammar를 분리 | 통과 |
| 4 | `%`·`em`·`calc()`를 CSS source 문자열에서 잘못 재계산 | source parser를 만들지 않고 Stylo가 resolve한 px 값만 받고 변환 결과를 test | 통과 |
| 5 | `normal`을 모든 플랫폼에서 10px로 강제 | CSS font-metric 동작과 Chrome Ahem 관찰을 분리하고 Simulator Ahem 결과를 별도 기록 | 통과 |
| 6 | CSS가 허용하는 zero 값이 text model에서 계속 거부 | C11.4 direct-text 경로의 `ParagraphStyle`에만 finite zero 허용, `InlineRunStyle`은 범위 밖으로 고정 | 통과 |
| 7 | 음수·NaN·Infinity 또는 overflow가 Parley/track으로 유입 | finite/nonnegative 검사와 변환 후 finite 검사를 거쳐 layout 시작 전에 거부 | 통과 |
| 8 | `line-height:0`에서 두 forced line을 한 line으로 합쳐버림 | zero forced-break scene도 두-line structure·intrinsic result·row track으로 검증하고 glyph overlap과 분리 | 통과 |
| 9 | number가 font size가 아닌 em-relative length처럼 계산됨 | 고정 computed `font-size:10px`; unitless number의 독립식 `n × 10px`를 명시 | 통과 |
| 10 | root font size가 변하면 fixture 기대값만 통과하고 adapter는 오래된 값 사용 | Ahem/10px style invariant를 유지하고 root computed value와 ParagraphStyle input을 같은 snapshot으로 검사 | 통과 |
| 11 | forced break와 soft wrap이 같은 분기만 검사 | 12 forced-break cases에 single-line 및 20px soft-wrap controls를 추가 | 통과 |
| 12 | 단일 `auto` row가 아닌 고정 row가 계산을 가림 | reference와 runtime scene 모두 auto row를 사용하고 final row contribution을 비교 | 통과 |
| 13 | root height나 Range rect를 track size로 잘못 사용 | gridTemplateRows/track trace를 판정값으로, root frame·Range·glyph paint를 별도 관찰로 분리 | 통과 |
| 14 | sub-font line-height에서 glyph가 겹친 값을 layout failure로 오인 | line box/track 높이와 10px glyph Range rect를 별개로 기록하고 pixel equality를 주장하지 않음 | 통과 |
| 15 | anonymous item이 grid root의 computed value를 상속하는 연결이 누락 | 정확한 root owner·direct text source·same style revision gate를 계획에 명시 | 통과 |
| 16 | number와 length의 자식 상속 차이가 이번 fixture에서 검증된 것처럼 표시 | 자식 font-size와 inline span이 없는 profile로 제한하고 상속 차이는 범위 밖으로 기록 | 통과 |
| 17 | 값 불일치나 stale snapshot 이후 일부 frame/trace가 남음 | revision 일치와 computed-value 검증을 layout publication 전 gate로 두고 rejection controls 추가 | 통과 |
| 18 | 기존 10px anonymous text 장면에 회귀 | 변경 전 fixed 10px matrix를 신규 values와 함께 회귀 실행하고 기존 scene 결과 exact 비교 | 통과 |
| 19 | fixture FFI ABI/payload가 제품 artifact로 새어 나감 | default artifact negative 및 fixture-test positive control을 별도 검사 | 통과 |
| 20 | Simulator 결과나 계획 PR을 public/WPT/C11 완료로 승격 | AVD/iOS Simulator 한계·artifact 한계 명시, plan PR에서 `STATUS.md` checkbox와 `72/213` 유지 | 통과 |

검토 중 `InlineRunStyle`까지 zero validation을 바꾸던 초안은 direct Grid paragraph와 무관하게 C15.2b span 입력 경계를 넓히는 것으로 판정했다. 계획을 `ParagraphStyle`만 변경하도록 좁혔고, `InlineRunStyle` 및 C15.2b 계약은 그대로 둔다. 이 계획의 20개 관점은 구현 검토에 재사용하지 않는다.
