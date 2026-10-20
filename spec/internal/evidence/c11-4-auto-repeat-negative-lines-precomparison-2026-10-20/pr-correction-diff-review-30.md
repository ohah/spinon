# C11.4 auto-repeat 계획 정정 PR diff 적대 검토 30개

**검토 범위:** 수정된 계획, 상위 계획 링크, 구현 전 code-path 근거, 초기 검토 판정의 무효화 기록, Chrome oracle의 기존 18-case 자료와 SHA index. 이번 기록은 auto-repeat branch dispatch를 다시 소스와 대조했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 문서가 일반 normalizer를 auto-repeat 분기에도 적용되는 것처럼 기술하는가 | 전용 `normalize_auto_repeat_grid_items` 분기를 분명히 쓰고 일반 normalizer 주장을 제거했다. 통과. |
| 2 | Taffy line resolution 순서를 repeat count보다 앞이라고 적는가 | source 순서대로 axis count, explicit count, `into_origin_zero`, item placement를 기록한다. 통과. |
| 3 | 이미 동작할 수 있는 negative line resolver를 중복 구현 대상으로 삼는가 | 별도 resolver를 제외하고 기존 Taffy 경로에 regression을 추가한다. 통과. |
| 4 | 실제 확인된 거부 오류가 계획에서 빠지는가 | auto-repeat과 `GridAutoRows/Columns::TrackList`의 명시 거부를 양 축에 기록한다. 통과. |
| 5 | 거부문 제거가 adapter 전달을 대신할 수 있다고 전제하는가 | Taffy Style auto-track field의 누락을 별도 연결 작업으로 명시한다. 통과. |
| 6 | scalar px control을 검증하지 않아 기존 input 의미가 무시되는가 | `LengthPx` control을 추가하고 지원 후보에 포함했다. 통과. |
| 7 | Auto default를 adapter 수정으로 바꾸는가 | 기본 Auto behavior를 control로 보존한다. 통과. |
| 8 | generic Taffy style mutation으로 non-grid node에 auto tracks가 생기는가 | 변경 범위를 GridContainerInput이 있는 grid container의 style conversion으로 한정한다. 통과. |
| 9 | columns와 rows가 동일 track list slice를 재사용하는가 | 각 field 및 axis에 별도 conversion과 two-axis regression을 요구한다. 통과. |
| 10 | leading implicit cycle list 순서가 CSS 규칙과 반대인가 | Taffy source의 reverse offset 및 `[31,47]` case를 근거로 넣었다. 통과. |
| 11 | trailing side도 reverse cycle해야 한다고 오해하는가 | positive/trailing control의 forward order를 별도 검증한다. 통과. |
| 12 | expanded explicit count가 implicit count와 합쳐져 negative line을 이동시키는가 | Taffy negative line basis는 explicit count로 고정하고 leading implicit offset은 track vector/frame 확인으로 분리한다. 통과. |
| 13 | repeat-only axis에서 prefix/suffix 없는 count가 잘못되는가 | 기존 독립 fixture와 repeat-only line controls를 유지한다. 통과. |
| 14 | two-axis auto-fit이 서로 다른 count/collapse를 섞는가 | per-axis expected count/collapse 및 independent controls를 유지한다. 통과. |
| 15 | definite available size 산식의 gap 수가 수정 중 바뀌는가 | 기존 Chrome 입력과 기대 JSON 산식은 유지하고, review 기록을 바꾼 뒤 hash를 다시 계산한다. 통과. |
| 16 | fixed `minmax()` support guard보다 넓은 grammar를 어댑터가 받아들이는가 | profile guard를 그대로 유지하고 px bounded cases만 대상으로 둔다. 통과. |
| 17 | grid-auto list empty/invalid/over-cap cases fail open되는가 | 기존 validator와 layout error types를 보존하며 empty, invalid, cap boundary를 추가 regression으로 명시한다. 통과. |
| 18 | 0px size가 다른 track과 동일하게 falsy/Auto 처리되는가 | zero-valued px regression을 요구하고 fixed `0px`와 Auto를 구분한다. 통과. |
| 19 | negative line invalid `0`이 Taffy conversion에서 Auto로 바뀌는가 | Spinon validation이 zero line을 거부하는 existing contract을 regression으로 유지한다. 통과. |
| 20 | large signed line/span이 `i16/u16` conversion에서 잘리거나 overflow하는가 | range rejection, checked arithmetic, overlarge grid cap을 검증 대상으로 남긴다. 통과. |
| 21 | hidden item 또는 out-of-flow item이 auto-fit occupied track을 만들지 않는가 | visible in-flow occupancy 기준을 명시하고 hidden/out-of-flow controls를 추가한다. 통과. |
| 22 | track-size trace가 final expanded auto-fit list와 맞지 않는가 | final track sizes, collapsed state, item frame을 같은 case ID에 연결한다. 통과. |
| 23 | Chrome gutter 차이가 수정 뒤 규범 기대값에 섞이는가 | 기존 Blink 차이를 유지하고 validator가 auto-fit frame을 normative expectation으로 사용하지 않는다고 적었다. 통과. |
| 24 | Chrome screenshot을 Spinon runtime evidence로 게시하는가 | 캡처는 browser oracle로 유지하고 simulator screenshots는 구현 PR에서 별도로 첨부한다. 통과. |
| 25 | 기존 plan review가 정정 후에도 유효 승인으로 남는가 | 초기 review 파일의 verdict superseded note와 새 20-case plan re-review 링크가 있다. 통과. |
| 26 | 기존 PR diff review의 잘못된 #26 판정이 승인으로 남는가 | 해당 기록을 명시적으로 invalidated/superseded 처리하고 새 30-case review를 추가한다. 통과. |
| 27 | SHA table에 수정된 review 또는 새 review가 빠지는가 | plan review, PR review, correction reviews의 hash 목록을 재생성한다. validator와 screenshot fixture hashes는 유지한다. 통과. |
| 28 | 내부 계약 0059나 `spec/STATUS.md`가 runtime 구현 없이 바뀌는가 | 지원 계약·완료 표시는 건드리지 않는다. 통과. |
| 29 | 수정된 내부 상대 링크가 잘못된 fixture/review 파일을 가리키는가 | 계획·README에서 정정 재검토와 Chrome evidence 경로가 저장소 내 실제 파일명과 일치한다. 통과. |
| 30 | 문서가 check 부재를 green CI 또는 완료 상태로 과장하는가 | plan은 구현 미완료와 최신 checks 확인 절차를 요구하며, `spec/STATUS.md`는 건드리지 않는다. 통과. |

## 판정

정정 PR diff에서 미해결 결함은 발견되지 않았다. 이번 변경은 계획과 근거만 고치며, Spinon numeric placement나 implicit track runtime 결과를 주장하지 않는다.
