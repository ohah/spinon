# C11.4 auto-repeat calc 최소값 계획 실패 관점 20개

구현 전에 계획과 독립 비교 기준만 검토했다. 구현 검토나 PR diff 검토 횟수에 포함하지 않는다.

| # | 독립 실패 관점 | 공격 검토와 판정 |
| --- | --- | --- |
| 1 | `<fixed-size>` 문법을 일반 `track-size`로 오인해 범위를 넓힌다. | `minmax(calc(abs),1fr)` 단일 track만 허용하고 고정 CSS Grid 문법과 대조한다. |
| 2 | `calc()` 문자열을 재파싱해 computed typed 값과 다르게 계산한다. | `ComputedCssMath` AST를 검증하고 기존 typed `Calc`를 보존하도록 고정한다. |
| 3 | percentage를 절대 길이처럼 추정한다. | 모든 percentage leaf를 unresolved로 취급해 fail-closed하도록 계획한다. |
| 4 | `Number`와 `LengthPx` 차원을 합쳐 잘못된 track 최소값을 허용한다. | evaluator에 scalar/length 차원을 유지하고 최종 결과가 길이인지 판정한다. |
| 5 | `Length × Length`를 CSS 길이로 취급한다. | 곱셈에 한 개 이상의 길이 operand가 생기면 거부하도록 관문에 명시했다. |
| 6 | 0 또는 음수 최소값으로 반복 수가 폭증하거나 CSS 비음수 조건을 위반한다. | calc 결과는 엄격히 양수만 수용하며 0/음수는 범위 밖 rejection으로 둔다. |
| 7 | 중간 산술 overflow가 최종 값 검사를 통과한다. | 모든 AST node 결과의 유한성을 검사하고 중간 비유한 값은 즉시 거부한다. |
| 8 | NaN, infinity, f32 범위 밖 변환이 Taffy 입력으로 유입된다. | f32 경계와 변환 후 유한성을 판정하고 overflow 값을 거부한다. |
| 9 | `min()`, `max()`, `clamp()`와 합·곱의 dimension rule이 누락된다. | 각 연산자 positive/negative unit case를 cascade focused 검증에 포함한다. |
| 10 | 지나치게 깊거나 큰 AST가 preflight 비용을 무제한 늘린다. | 기존 computed math depth/node cap을 유지하고 초과값을 거부한다. |
| 11 | `1fr` maximum을 repeat count의 definite 크기로 잘못 사용한다. | 독립 기준은 definite `calc` minimum을 repeat count에 쓰고 남은 공간만 fr에 배분한다. |
| 12 | auto-fill track 수와 free-space 분배를 섞어 잘못된 used size를 기대한다. | 184/40으로 네 반복을 먼저 산출하고 24/4를 별도 계산해 46px을 고정한다. |
| 13 | auto-fit을 placement 전에 collapse한다. | 한 in-flow item이 첫 track을 점유한 뒤 세 빈 track을 접는 관찰 순서를 검사한다. |
| 14 | auto-fit suffix gutter를 이 slice의 0 gap 조건과 혼동한다. | 모든 fixed scene gap을 0으로 고정하고 gutter 변형은 별도 범위로 뺀다. |
| 15 | rows와 columns가 서로 다른 cascade path를 지나 한 축만 허용된다. | 동일 six-scene model에서 두 축을 분리 실행해 대칭성을 확인한다. |
| 16 | 기존 line-name/area placement나 양축 repeat의 admission gate가 넓어진다. | 변경은 sizing breadth preflight에 한정하고 existing targeted matrix를 회귀시킨다. |
| 17 | 다른 runtime profile이 `Calc` auto-repeat을 우연히 받는다. | `RuntimeBlockFlexGridTrackSizingV1` allow gate positive/negative control을 요구한다. |
| 18 | calc 검증 오류가 partial layout이나 이전 revision 결과를 남긴다. | 실패는 명시적 layout error로 전파하고 부분 output 없음 및 다음 revision 복구를 확인한다. |
| 19 | fixture code/symbol이 기본 제품 artifact에 포함된다. | 기본/product archive negative control과 fixture-test positive control을 분리한다. |
| 20 | fixed fixture 결과를 일반 CSS support 또는 C11.4 완료로 승격한다. | 공식 checkbox와 `72/213`, 내부 버전 `0.1.0`을 유지하고 미지원 조합을 명시한다. |
