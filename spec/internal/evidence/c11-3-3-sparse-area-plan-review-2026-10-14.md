# C11.3.3 sparse named-area 표현 계획 검토 · 2026-10-14

## 검토 변경

계획의 `owned row/cell/name` 표현은 Stylo typed `GridTemplateAreas`의 row/column dimensions와 named area별 owned name·반열린 rectangle bounds로 구체화했다. null cell은 named area 목록에서 제외한다. area 전체를 row×column dense matrix로 복사하지 않으며 선언 문자열도 다시 파싱하지 않는다. 저장 표현이 바뀌어도 CSS observable contract나 fixture scope는 바뀌지 않는다.

기준 근거는 [W3C CSS Grid §7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas-property), [§7.3.2–7.3.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-named-lines), [§8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)이다. 고정 Chromium geometry 및 고정 WPT source/subtest는 [사전 비교](./c11-3-3-grid-template-areas-precomparison-2026-10-14/README.md)를 따른다.

## 실패 관점 20개

| # | 독립 검토 관점 | 판정 기준 또는 방지 장치 |
| --- | --- | --- |
| 1 | Stylo canonical value 대신 authored CSS string을 재파싱하는가 | `GridTemplateAreas` typed values만 읽고 raw string parser를 추가하지 않는다. |
| 2 | null cell을 dense matrix에서만 표현해 메모리가 rows×columns로 증가하는가 | named rectangle 목록과 dimensions만 소유하고 null cell은 rectangle 부재로 표현한다. |
| 3 | 모든 null cell인 유효 area가 `none`으로 붕괴하는가 | `none`과 `rows>0, columns>0, areas=[]`를 구별해 보존한다. |
| 4 | Stylo의 1-based ranges를 내부 cell bounds로 옮길 때 off-by-one이 나는가 | 내부 cell bounds는 0-based 반열린 범위로 한 번만 정규화하고 Chrome 단일·span geometry를 고정한다. |
| 5 | area 끝 경계를 track 수와 혼동해 line 0 또는 마지막 track을 가리키는가 | cell `[start,end)`와 grid line `start+1/end+1` 변환을 명시적으로 검사한다. |
| 6 | 이름 목록의 순서만으로 영역 모양을 재구성해 L자/분리형을 허용하는가 | 사각형 정합성은 Stylo typed parser가 거부하며 fixture cascade fallback으로 확인한다. |
| 7 | row width 불일치·empty string·trash token을 runtime error와 cascade fallback으로 혼동하는가 | parser-invalid declaration은 선택된 computed value를 사용하고 runtime area 오류와 분리한다. |
| 8 | explicit track 수와 area dimensions 불일치에서 implicit auto track을 추정하는가 | layout projection에서 mismatch를 node/property 문맥과 함께 거부한다. |
| 9 | row/column 축을 교차 적용하는가 | asymmetric fixture를 두고 각 축의 alias와 frame을 별도로 비교한다. |
| 10 | area name의 대소문자·escaped/non-ASCII 식별자를 바꾸는가 | Stylo atom에서 이름을 복사하고 string 비교는 case-sensitive Unicode 내용으로 한다. |
| 11 | bare `<custom-ident>`와 `1 ident`를 같은 occurrence로 정규화하는가 | bare는 `None`, authored integer는 `Some(n)`으로 보존하고 adversarial geometry를 유지한다. |
| 12 | bare area 이름이 edge alias 전에 ordinary line처럼 조회되는가 | area `foo`는 property edge별 `foo-start`/`foo-end` 후보를 먼저 찾는다. |
| 13 | authored `foo-start`와 generated alias의 물리 line 순서를 잃는가 | authored와 generated 후보를 합쳐 CSS §8.3의 첫 physical line을 선택한다. |
| 14 | 한 물리 line의 중복 authored/generated 이름이 여러 occurrence로 세어지는가 | 동일 index에서 authored/generated가 함께 있어도 named occurrence는 한 번만 센다. |
| 15 | `span foo`가 named area로 오인되는가 | `span`은 line-name occurrence 검색을 하고 area-edge 해석 경로를 타지 않는다. |
| 16 | explicit `foo-start`/`foo-end` implicit area가 한 축만으로 생성되는가 | both axes에서 start와 end line을 모두 찾아야 implicit named area를 만든다. |
| 17 | explicit line implicit area와 template area가 같은 이름일 때 정해진 우선 순서가 없는가 | template area를 먼저 찾고 explicit-line implicit area는 fallback으로 조회한다. |
| 18 | area metadata allocation 실패 뒤 일부 line alias 또는 placement가 남는가 | area copy reserve 지점에 실패를 주입하고 cascade/layout 입력을 원자적으로 거부한다. |
| 19 | area 의미가 Taffy parser나 `Style`의 큰 line/area 버퍼로 흘러드는가 | style-to-layout에서 numeric line으로 정규화하고 Taffy에는 기존 compact style만 전달한다. |
| 20 | Chrome/WPT oracle를 Spinon 적합성으로 과장하거나 예외를 확대하는가 | 네 WPT의 73 subtest는 Chrome oracle only이며 unknown-area implicit tracks는 현재 미지원으로 유지한다. |

## 판정

이 계획 보완은 named-area 저장의 공간 형태를 명확히 한다. CSS 동작 subset, 오류 경계, WPT classification, 플랫폼 gate는 확장하지 않는다. 구현은 본 문서와 C11.3.3 고정 비교 fixture 이후 시작한다. 이 표는 계획 변경에 대한 review이며 이후 별도 C11.3.3 구현 실패 관점 20개를 대체하지 않는다.
