# C11.3 계획 실패 관점 검토 · 2026-10-13

## 검토 대상

[C11.3 계획](../../../plan/c11-3-template-repeat-named-lines-areas.md), 상위 [C11 CSS Grid 계획](../../../plan/c11-css-grid.md), [내부 계약 0059](../0059-c11-grid-layout.md), Stylo `0.22.0`, Taffy `0.14.0`, 고정 Chromium 154 및 WPT revision을 대조했다. 이 검토는 구현 전에 계획의 누락·모순을 찾는 단계다. 코드, Spinon runtime, 플랫폼 실행을 검증한 결과로 취급하지 않는다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검토 결과와 반영 |
| --- | --- | --- |
| 1 | C11.3 세 기능을 한 번에 구현해 변경·회귀 원인을 분리 못함 | `repeat()` 확장, named line 조회, template area를 독립 PR 세 개로 나누고 순서를 고정했다. |
| 2 | 고정 `repeat()`과 `auto-fill`/`auto-fit`을 같은 기능으로 처리 | 양의 정수 반복만 지원하고 auto-repeat은 명시적으로 제외했다. |
| 3 | 반복 count 0·음수·0이 아닌 소수를 runtime에서 성공 처리 | parser invalid/cascade fallback과 runtime unsupported computed value를 나눴다. WPT 원문의 Chrome PASS를 Spinon 결과로 승격하지 않고 test/subtest별 지원·oracle-only·미지원 이유를 기록한다. |
| 4 | nested repeat이나 반복 내부 unsupported track size를 Taffy 기본값으로 흘림 | nested repeat은 제외하고 기존 `px`·definite `%`·finite positive `fr` subset만 허용한다. |
| 5 | count × inner track 수의 overflow를 먼저 계산 | checked arithmetic, 기존 10,000 track cap 사전 검사, 10,001 경계 fixture를 요구한다. |
| 6 | repeat 확장 중 예약 실패가 일부 track/line만 남김 | 확장과 line metadata 예약의 failure injection, 전체 결과 부재, 동일 worker 복구를 요구한다. |
| 7 | 반복 경계 line을 track 수와 같은 크기로 저장해 마지막 line 유실 | N개 track에 N+1 line groups를 보존하는 engine-neutral 입력을 고정했다. |
| 8 | 반복 사이 끝/시작 line-name groups를 서로 다른 line으로 계산 | seam을 단일 line metadata 위치로 합치고 count 1·빈/비빈 group fixture를 둔다. |
| 9 | 한 line에 같은 이름이 중복될 때 목록을 임의 dedupe해 occurrence 변경 | 원본 name list는 보존하되 line occurrence는 index 기준으로 한 번만 세고 repeat seam을 Chrome oracle로 확인한다. |
| 10 | CSSOM `getComputedStyle()`의 resolved track value를 Stylo computed value와 같은 값으로 비교 | CSSOM resolved serialization, Stylo typed computed value, normalized metadata, geometry를 별도 관측으로 기록하도록 고쳤다. |
| 11 | 잘못된 CSS declaration을 layout adapter가 다시 파싱해 다른 오류로 보고 | CSS parser/cascade fallback과 unsupported typed computed value 진단을 분리했다. |
| 12 | `"..."` 같은 점 연속을 여러 null cell로 세어 area column 수를 바꿈 | 연속 dot run은 null cell token 한 개로 처리하고 별도 사례를 둔다. |
| 13 | area matrix 행마다 cell 수가 다른데 partial rectangle로 계속 진행 | 모든 행의 cell 수 동일 및 최소 한 cell 조건을 parser oracle로 확인한다. |
| 14 | 한 area 이름이 여러 cell에 나오면 모두 invalid라고 가정 | 반복 cell이 하나의 직사각형이면 유효하며 L자/분리 영역만 invalid라고 명시했다. |
| 15 | area matrix가 track list가 없을 때 implicit/auto-sized tracks를 만드는 동작을 놓침 | C11.3.3은 양 축의 explicit track list와 일치하는 area matrix만 받고 auto-sized track 생성은 범위 밖으로 닫았다. |
| 16 | `grid-template`/`grid` shorthand를 computed input만으로 구별해 거부하려 함 | authored shorthand provenance는 typed computed 입력에서 일반적으로 구별할 수 없으므로, 결과 typed values가 지원 subset에 맞으면 받도록 수정했다. shorthand side effects가 unsupported면 computed property 진단으로 실패한다. |
| 17 | named line의 양수/음수 occurrence를 동일한 시작 방향에서 검색 | positive는 explicit grid 시작, negative는 끝 기준으로 분리하고 양쪽 Chrome fixture를 요구한다. |
| 18 | 없는 named line이나 occurrence 부족을 임의 track 위치로 바꿈 | 표준상 implicit line 생성 동작은 현재 subset에서 지원하지 않는다고 선언하고 구체 오류로 닫았다. |
| 19 | `span <ident>`가 이름을 세는 방향과 `grid-area` named-area 우선순위를 놓침 | 반대편 definite line 기준 span 해석, generated `foo-start`/`foo-end` alias, implicit named area, CSS §8.3 첫 매칭을 테스트하도록 보강했다. |
| 20 | Rust 결과만으로 모바일 GPU 화면까지 완료 처리하거나 iOS Simulator를 실기기로 승격 | Android 실기기 geometry/PixelCopy와 iOS Simulator geometry/Metal drawable을 분리하고, compositor 후·iOS 실기기·WPT 적합성은 미완료로 남기는 관문을 추가했다. |

## 계획에 반영한 핵심 수정

검토 중 authored shorthand를 차단하려는 문장은 Stylo computed-value 경계에서 판별할 수 없다는 모순을 확인해 제거했다. `grid-template`/`grid`가 만든 값도 computed property가 제한 subset이면 처리하고, shorthand side effect가 지원 밖인 경우에만 해당 computed value를 거부한다. `getComputedStyle()`의 row/column 값은 resolved value이므로 Stylo typed computed value와 동일시하지 않도록 비교 모델을 분리했다. Template area는 cell 수·직사각형 규칙, explicit track 차원, generated line 이름 우선순위까지 범위를 명시했다.

## 판정

계획 범위·소유자·세 PR 경계·기준 엔진·WPT 원문 hash·test/subtest 분류·fixture 최소 목록·오류 경계·named-line occurrence 규칙·모바일 검증 계층이 기록되었다. 남은 의도적 미지원은 auto-repeat, implicit named line/track, auto-sized area track, intrinsic sizing, `minmax()`/`fit-content()`, subgrid, masonry, full WPT runner/적합성 등이다. 이 문서는 C11.3 구현을 시작하거나 완료한 근거가 아니다. C11.3.1 구현 PR은 이 계획과 별도의 20개 구현 실패 관점 검토, 실제 Chrome/Rust/Android/iOS 실행을 가져야 한다.
