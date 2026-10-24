# C11.4 calc neighbor 계획 실패 관점 검토 20개

**대상:** [auto-repeat `calc()` neighbor 계획](./c11-4-auto-repeat-calc-neighbors.md) · [Chrome/규범 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/README.md) · 검토 기준 source `67ccd0c13546f15e986fbf41afeb770678e84169`

검토는 product code 수정 전에 계획의 서로 다른 실패 경로를 대조했다. 처음 초안에는 calc 결과 음수를 거부할 수 있는 문구가 있었으나, CSS Values 4 §10.12와 Chrome `calc(-20px + 5%)` 관측을 재확인해 지원 shape의 음수 최종값을 0px로 clamp하도록 계획을 고쳤다. 최초 계획은 음수 percentage AST, 추가 sum term, indefinite basis를 CSS-valid negative control로 분리했다. 구현 착수 전 Stylo canonicalization을 대조한 보정 내용은 아래 후속 기록에 있다.

| # | 실패 관점 | 적대 확인 | 판정·반영 |
|---:|---|---|---|
| 1 | auto-repeat 내부 grammar 혼동 | `<auto-repeat>`의 fixed-size 제한과 fragment 밖 neighbor를 구분했다. 계획은 반복 fragment 내부 sizing syntax를 확장하지 않는다. | 통과; 내부 track admission은 기존 경계 유지 |
| 2 | calc 전체 평가기로 범위가 새는 경우 | Math 함수·중첩·Product·Min/Max·Clamp와 임의 Sum을 받아들이는지 확인했다. | 통과; canonicalization 뒤에도 두 typed term이 length+percentage인지 확인하고 나머지는 reject |
| 3 | 다른 property에서 온 계산식 재사용 | Layout calc ID의 node·property·axis 출처 일치를 확인했다. | 통과; mismatch는 LayoutError, partial output 없음 |
| 4 | AST 항 순서에 따른 누락 | Stylo가 length와 percentage 항을 반대 순서로 정규화할 수 있는지 확인했다. | 통과; 두 order를 whitelist에 명시 |
| 5 | 음수 length term을 미리 잘라내는 오류 | `calc(-2px + 5%)`가 240px basis에서 10px이 되는 fixture와 grammar를 대조했다. | 통과; finite signed length를 보존하고 계산 뒤 range check |
| 6 | 최종 음수를 invalid로 처리하는 오류 | `calc(-20px + 5%)`에서 CSS Values range clamp 및 Chrome 0px track을 비교했다. | 초안 수정; 지원 shape의 최종 음수만 0px로 clamp |
| 7 | 0px clamp를 이웃 부재로 축약 | `col-fill-negative-final`에 명시된 0px prefix와 track count/gap을 비교했다. | 통과; zero-sized explicit neighbor와 부재를 구분 |
| 8 | 음수 percentage AST를 묵시적으로 허용 | `calc(20px - 5%)`은 CSS-valid이며 240px에서 8px이나 계산식 AST 형태는 계획 whitelist 밖이다. | 통과; 별도 negative control, 조용히 양수 percentage로 취급하지 않음 |
| 9 | column basis를 row에 재사용 | 양축 fixture를 독립적으로 추적했다. | 통과; columns inline content size, rows block content size 사용 |
| 10 | border-box outer size를 percentage basis로 사용 | `136px - 16px padding - 4px border = 116px` 계산과 Chrome quantization을 대조했다. | 통과; content box만 basis |
| 11 | neighbor/gutter를 repeat count에서 누락 | 양쪽, prefix-only, suffix-only와 4px gutter의 candidate count를 validator에서 재계산했다. | 통과; non-repeated track과 모든 relevant gutter 포함 |
| 12 | 어떤 수에서도 content box에 맞지 않음 | 196px + 74px neighbor와 40px minimum을 합쳐 minimum one repetition을 확인했다. | 통과; count 1, 78px grid overflow를 보존 |
| 13 | auto-fit 점유 track의 gutter collapse | occupied-first scene에서 빈 track과 gutter collapse를 반복횟수 산정 이후 검사했다. | 통과; count 4와 used `[22,200,0,0,0,10]` 분리 |
| 14 | 모든 auto-fit track이 비어 있음 | item 없는 scene과 zero track list를 대조했다. | 통과; repeat count 4를 유지하고 빈 반복 track은 0 |
| 15 | count용 minimum과 final flex used size 혼동 | `minmax(40px, 1fr)`의 count floor 40px, definite free-space 배분을 따로 계산했다. | 통과; count basis와 final `1fr` 분배 분리 |
| 16 | grid 밖 item을 frame만으로 검증 | overflow scene item은 root right edge보다 4px 밖이며 surface 표본을 plan acceptance에 넣었다. | 통과; PixelCopy·Metal drawable에서 item center와 양쪽 boundary sample 필수 |
| 17 | indefinite axis를 definite로 간주 | max-content scene의 CSS-valid output과 CSS Grid repeat-once 규칙을 비교했다. | 통과; basis 미정은 fail-closed negative control |
| 18 | f32 곱·neighbor 합 overflow 뒤 partial layout | percentage basis multiplication, neighbor aggregate, non-finite input과 Taffy representability gate를 점검했다. | 통과; f64 계산, 표현 불가·aggregate overflow 시 partial output 없이 실패하는 별도 unit cases 계획 |
| 19 | negative control을 CSS syntax 오류로 잘못 보고 | 단순 `calc(10px + 5% + 1px)`는 Stylo computed AST에서 `[LengthPx(11), Percentage(0.05)]`로 정규화되므로 원문 항 개수만으로 거부할 수 없다. nested `clamp()` 항, negative percentage term, indefinite calc neighbor는 Chrome에서 유효하게 계산되는 AST/basis 경계 밖 control이다. | 계획 보정; 두 항 whitelist는 projected typed AST 기준이며 nested `clamp()` fixture를 사용 |
| 20 | fixture가 제품 artifact나 완료 수를 오염 | 14 cases 중 11 positive/3 negative, default-vs-opt-in artifact, simulator 경계와 공식 `72/213`을 대조했다. | 통과; fixture-test 전용, 상위 C11.4 미완료 유지 |

**결론:** 계획 범위는 현재 typed cascade/layout 경계와 독립 CSS oracle에 맞는다. 앞의 range-clamp 문구와 음수 percentage/unsupported controls는 수정 후 재검토했다. 계획 검토가 끝났으며, 이 표를 구현/runtime 검토나 PR diff 검토 횟수에 재사용하지 않는다. 구현이 시작되면 새로운 20개 implementation/runtime 관점과 별도의 30개 PR diff 관점을 작성한다.

**구현 착수 전 보정 · 2026-10-24:** focused Stylo cascade test에서 단순 authored 3-term calc가 computed 단계에서 compatible length 항을 합치는 것을 발견했다. 거부 기준은 처음부터 명시한 projected typed AST shape로 유지하고, 해당 원문을 3-term 거부 control로 주장하지 않도록 fixture와 기대값을 고쳤다. `clamp()`가 남아 있는 CSS-valid nested math control을 사용한다. 이 보정은 지원 typed shape를 넓히지 않으며 Chrome 사전 비교와 계획 기대값도 다시 생성한다.
