# C07.2 고 DPR CSS 경계·원문 복구 구현 검토

이 검토는 PR #103에 포함된 C07.2 기본 구현을 다시 승인하는 문서가 아니다. Stylo가 serialize 과정에서 소수 정밀도를 잃는 경우를 보완한 후속 변경을 대상으로 한다. 기준은 고정 Chrome 154 fixture, Rust 회귀 테스트, cascade 원문 복구 경로다.

## 검토 관점과 결과

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 정수 경계 바로 아래의 폭이 다음 정수로 반올림되는가 | `1.999px`와 `1.9999px`는 `1 CSS px`로 유지된다. Chrome 154 probe와 Rust 테스트가 일치한다. |
| 2 | 경계값과 경계 직후 값이 같은 폭으로 처리되는가 | `2px`와 `2.001px`는 `2 CSS px`다. 경계 직전·경계·직후 reference를 분리했다. |
| 3 | 결과가 기기 pixel scale에 의존하는가 | DPR 1·2·2.625·3의 Chrome probe에서 authored CSS 폭 판정은 동일하다. 고정 환경 밖의 브라우저 동작까지 일반화하지 않는다. |
| 4 | authored 선언을 복구하지 못하면 부정확한 값을 조용히 채택하는가 | 선언 복구가 가능한 경우에만 원문 경로를 사용하고, 지원하지 않는 수식은 기존 Stylo typed 값 경로로 남는다. 레이아웃 입력은 기존 finite 검사를 거친다. |
| 5 | inline style의 cascade 승자 대신 뒤쪽 선언을 고르는가 | Stylo rule node와 declaration importance를 사용해 승자를 정하고 inline 선언 블록은 해당 원문과 연결한다. 회귀 사례가 통과한다. |
| 6 | author stylesheet의 선택자·우선순위가 원문 검색과 어긋나는가 | Stylo의 승자 `StyleRule` 선언 블록과 원문 source offset을 연결한다. CSS 전체를 텍스트 검색해 승자를 추측하지 않는다. |
| 7 | `!important`와 일반 선언의 순서가 뒤집히는가 | 중요도 tier와 declaration 순서를 함께 검사한다. 뒤에 온 일반 선언이 이전 `!important` 승자를 덮지 않는 사례를 테스트한다. |
| 8 | `border-width` shorthand가 네 면으로 확장되는 과정에서 누락되는가 | Stylo가 만든 면별 longhand 승자를 사용한다. author stylesheet와 inline override fixture가 네 면의 입력 경계를 확인한다. |
| 9 | `border-style:none` 또는 `hidden`인데 폭이 box에 반영되는가 | 면별 style gate가 계속 폭을 0으로 만든다. 후속 원문 폭 계산은 이 gate를 우회하지 않는다. |
| 10 | 정확히 0인 폭을 최소 1px로 올리는가 | floor 적용 전 0을 보존한다. 기존 0·none·hidden 경로와 신규 값 경로를 나눠 확인했다. |
| 11 | `thin`·`medium`·`thick` keyword의 computed 폭과 기존 동작이 깨지는가 | Stylo typed 값 fallback을 보존하며 각 keyword의 회귀 입력을 추가했다. |
| 12 | `em`·`rem` 값이 요소·루트 글꼴 크기와 다르게 환산되는가 | 요소 및 root font-size를 포함한 길이 문맥에서 inline과 author stylesheet 값을 확인한다. |
| 13 | viewport 단위가 width·height·small·large·dynamic viewport 축을 혼동하는가 | `vw`·`vh` 및 `sv*`·`lv*`·`dv*`·logical viewport·min/max 계열을 고정 fixture와 Rust 회귀 입력으로 다룬다. |
| 14 | `var()`의 중첩·fallback·미정의 경로가 엉뚱한 폭을 만든다 | 중첩 custom property, 직접 fallback, fallback 내부 변수와 미정의 값 사례를 확인한다. 순환·무효 값은 지원 입력으로 오인하지 않는다. |
| 15 | `calc(var(...))`에서 계산 전후 값 또는 단위 변환을 중복 적용하는가 | `calc()` 결과를 CSS px로 한 번만 계산해 floor한다. 경계 전후 값을 모두 둔 회귀 사례가 통과한다. |
| 16 | custom property가 다른 선언의 원문으로 잘못 연결되는가 | computed custom property는 Stylo 결과에서 가져오고, authored border 선언만 해당 원문 source와 연결한다. |
| 17 | 주석·문자열의 `{}`, `;`가 declaration 경계를 속이는가 | quote·escape·comment-aware scanner를 사용한다. 선언 앞 주석과 declaration block 주석 사례를 검사했다. |
| 18 | UTF-8 byte offset과 Stylo의 UTF-16 source location이 어긋나는가 | source offset index와 emoji·CRLF 입력을 이용해 원문 위치를 확인한다. 반복 전체 문자열 검색을 하지 않도록 offset checkpoint를 둔다. |
| 19 | 앞선 선언이 많을 때 탐색 예산이 마지막 cascade 승자를 버리는가 | 256개 이전 선언 뒤의 승자 declaration을 누락하는 결함을 재현해, 예산을 authored declaration별로 분리했다. 257번째 승자 회귀 테스트가 통과한다. |
| 20 | 큰 stylesheet/comment가 있어도 작은 유효 border 선언이 처리되는가 | 2 MiB 초과 원문 전체를 거부해 유효 선언을 놓치는 결함을 재현했다. 전체 입력의 조기 크기 차단을 제거하고 큰 선행 주석 회귀 테스트를 통과시켰다. |

## 수정한 결함

- Stylo serialization이 `1.9999px`를 정수 CSS px로 복원하기 전에 소수 정보를 잃을 수 있어, 승자 선언의 authored CSS와 computed custom property를 이용해 CSS px를 복원한다.
- declaration별 256값 예산이 공유되어 뒤의 cascade 승자를 버리던 문제를 declaration 단위 예산으로 고쳤다.
- 2 MiB가 넘는 원문을 통째로 거부하던 제한 때문에 앞에 큰 주석이 있는 짧은 유효 선언을 놓치던 문제를 고쳤다.
- CSS rule source 위치를 매번 원문 처음부터 선형 검색하던 경로를 checkpoint index로 바꿨다.

## 검증 근거와 경계

- 고정 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 DPR 1·2·2.625·3 probe는 `tests/fixtures/css/references/c07-2-border-width-high-dpr-v1.json`에 고정한다.
- focused Rust tests는 `cargo test --locked -p spinon-style c07_border_floor`로 실행한다. 전체 CSS reference 검사는 `bun run test:css-reference`로 실행한다.
- 이 후속 변경 자체는 Android 실기기나 iOS 앱 런타임에서 fractional CSS 경계를 실행하지 않았다. 기존 C07.2 플랫폼 스모크 근거를 이 값의 기기 검증으로 확대 해석하지 않는다.
- 테두리 paint, radius, logical edge, 외부 CSS URL, 전체 CSSOM과 무제한 authored source 복구는 이 구현의 지원 범위가 아니다.
