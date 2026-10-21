# C11.4 anonymous Grid text item 계획 적대 검토 20개

| # | 실패 관점 | 공격 검토와 판정 |
| ---: | --- | --- |
| 1 | 인접 text node를 여러 anonymous item으로 세는가 | first/last node grouping과 comment 대조 case를 고정했다. 한 run으로 처리하지 못하면 계획 gate 실패다. |
| 2 | comment가 inline content를 끊는다고 가정하는가 | Chrome에서 comment 앞뒤 Range rect가 같은 line에 이어짐을 관찰했다. comment는 layout item boundary가 아니다. |
| 3 | 빈 text node가 0 크기 item을 만드는가 | empty node는 run anchor나 item을 만들지 않고 다음 유효 run에 영향을 주지 않도록 계획했다. |
| 4 | collapsible-only whitespace가 auto-placement cell을 먹는가 | Chrome whitespace case에서 Range rect가 비고 두 block이 첫 두 track에 남는다. run 자체를 제외한다. |
| 5 | NBSP를 collapsible HTML whitespace로 잘못 버리는가 | collapsible 문자 판정은 HTML/CSS whitespace 규칙으로 제한한다. NBSP 및 다른 보존 문자는 이번 고정 입력에서 다루지 않고 임의 `is_whitespace()` 삭제를 금지한다. |
| 6 | `white-space:pre` 공백을 무시해 콘텐츠를 잃는가 | pre-preserving mode는 지원 범위 밖으로 정하고, silent drop 대신 명시적 unsupported error를 요구한다. |
| 7 | block child가 두 text run을 하나로 합치는가 | Chrome 세 번째 case에서 text/block/text가 서로 다른 3개 item으로 자동 배치된다. element child에서 run을 flush한다. |
| 8 | direct inline span을 anonymous text와 혼동하는가 | inline element 자체는 explicit child item이며 별도 formatting을 가진다. 이 구현은 이를 anonymous text run에 합치지 않고 scope 밖 입력으로 거부한다. |
| 9 | nested element 안 text를 부모 Grid의 직접 item으로 승격하는가 | 직접 child 경계만 스캔하고 descendant text를 부모 run에 합치지 않는다. |
| 10 | `display:none` subtree의 text를 포함하는가 | 기존 hidden-node pruning 뒤에 grouping해 숨겨진 source가 item·trace에 들어오지 않도록 한다. |
| 11 | absolute/fixed element가 text-run boundary가 아닌가 | out-of-flow element은 Grid item이 아닐 수 있어 이번 고정 profile에서는 fail-closed boundary로 둔다. |
| 12 | source order가 whitespace 원시 index와 generated item 순서를 혼동하는가 | grouped item과 explicit block이 같은 ordered stream을 사용하고, 무시된 whitespace는 item order를 소비하지 않는다. |
| 13 | `order`가 있는 explicit sibling이 auto-placement 순서를 바꾸는가 | `order` 입력을 지원하지 않는 allowlist에 두고 default order-zero만 사전 기준에 포함한다. |
| 14 | 익명 item이 Grid container의 preferred width/border를 복사하는가 | box 자체는 unstyleable이므로 item 기본 style과 상속 text style을 분리한다. 부모 sizing/edge를 복제하지 않는다. |
| 15 | grid container font inheritance를 놓치는가 | text paragraph는 Grid owner computed font identity와 revision을 상속해야 한다. 미등록 Ahem은 fallback하지 않고 사전 gate를 실패시킨다. |
| 16 | min-content 측정을 callback available-space에서 잘못 추론하는가 | Parley intrinsic sizes를 독립 입력으로 쓰고 callback trace와 raw contribution input을 별도로 assert한다. |
| 17 | Range rect를 anonymous item frame 또는 glyph ink로 과대해석하는가 | Range fragment는 oracle 관찰만으로 한정하고 앱 item geometry는 layout/track trace 및 불변식으로 별도 판정한다. |
| 18 | text node ID anchor가 중복되거나 다른 Grid owner와 충돌하는가 | 첫 non-whitespace source node anchor를 문서 고유 ID로 사용하고 `(grid owner, anchor, source run IDs)` uniqueness 검사를 요구한다. |
| 19 | group 생성 중 오류에서 partial LayoutOutput이 노출되는가 | unsupported style와 reservation 오류는 layout result 전부를 거부하고 partial trace/layout을 게시하지 않아야 한다. |
| 20 | fixture FFI가 제품 기본 artifact나 public API로 새어 나가는가 | 기존 opt-in fixture-test feature에서만 연결하며 default artifact 제외와 positive-control 포함을 별도 확인한다. |

## 판정

Chrome oracle, CSS Grid item generation, text-node grouping, whitespace, source order, inherited style, provenance, 오류 원자성과 fixture 경계를 서로 다른 실패 관점으로 검토했다. 입력·허용 범위·실패 경로가 계획에 고정됐다. 구현 전 oracle은 Spinon 출력과 독립인 Chrome 155 관찰 및 CSS Grid §6 규범을 함께 사용한다.
