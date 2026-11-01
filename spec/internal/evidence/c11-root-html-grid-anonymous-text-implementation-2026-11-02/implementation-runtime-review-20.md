# C11.1 Root HTML anonymous Grid text 구현·런타임 실패 관점 검토 20개

**검토 대상:** `feat/c11-root-html-anonymous-grid-text`의 현재 구현 diff와 기본 parsed Root HTML runtime
**기준 입력:** [사전 비교 fixture](../c11-root-html-grid-anonymous-text-precomparison-2026-11-02/README.md)
**판정 기준:** [전용 계획](../../../../plan/c11-root-html-grid-anonymous-text.md), CSS Grid §6·§8.5, Root HTML/C11.4 회귀, Android API 37 AVD 및 iOS Simulator 근거

계획 검토 20개는 구현 전 계획을 검토한 별도 자료다. 아래 항목은 실제 변경 코드와 실행 경로를 대상으로 새로 검토했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 computed-style profile에서 새 입력 gate가 켜짐 | 새 projection 진입점은 `RuntimeRootHtmlTextPaintV1`만 허용하고 viewport 일치도 검사한다. 그 외 profile은 거부된다. 통과. |
| 2 | Root HTML 입력이 C11.4 fixture gate를 우회하거나 켬 | Root HTML 옵션과 C11.4 옵션이 별도이고 동시에 켜지지 않음을 assertion으로 고정했다. C11.4 anonymous-text 32개와 fixture positive control 1개 통과. |
| 3 | 다른 document generation의 root가 사용됨 | root handle generation을 snapshot과 대조하고, 런타임 source NodeId가 실제 HostDocument text node인지 검증한다. 통과. |
| 4 | text sequence가 손상된 UTF-16을 받음 | source를 UTF-16에서 문자열로 바꾸는 과정에서 오류를 반환한다. malformed source 런타임 입력은 별도 실행하지 않아 입력 경계는 코드 검사로만 확인했다. |
| 5 | 연속된 Text 노드가 두 Grid item으로 나뉨 | comment 경계가 있는 `A`와 `B`를 한 item·두 glyph로 그리는 기본 runtime 회귀가 통과한다. 전체 source-node provenance를 유지한다. |
| 6 | 첫 source가 공백이면 잘못된 anchor를 고름 | collector가 첫 비공백 source를 anchor로 고르고 공백 source도 provenance에 둔다. 기존 C11.4 leading-whitespace 회귀가 통과했다. Root HTML 전용 leading-whitespace 변형은 별도 실행하지 않았다. |
| 7 | whitespace-only text가 phantom item/frame을 만듦 | 기본 Root HTML 회귀에서 text scene이 비어 있고 Grid box는 유지된다. CSS Grid §6의 white-space-only sequence 무렌더링 규칙과 맞는다. `pre` 변형도 기존 Root HTML Grid 회귀가 확인한다. |
| 8 | visible element와 anonymous text가 섞여 source order가 틀림 | visible element sibling은 scene 생성 전에 `unsupported_root_html_grid_text_item`으로 거부한다. 부정 회귀 통과. |
| 9 | `display:none` sibling이 text run을 끊거나 배치 공간을 먹음 | hidden subtree 수집은 layout item을 만들지 않고 visible-tree 검사는 이를 visible sibling으로 세지 않는다. 기존 hidden-subtree 및 C11.4 run 경계 회귀 통과. |
| 10 | 한 문서에서 복수 Grid text item이 부분 지원으로 통과 | 값이 `AB`가 아니거나 이미 다른 Grid text owner가 있으면 거부한다. `ABC`는 실행해 확인했고, 두 Grid owner 조합은 분기 코드만 검사했다. |
| 11 | 지원하지 않는 white-space·font·line-height·정렬·색이 성공 scene에 반영됨 | paragraph adapter가 제한된 computed tuple을 확인한다. `pre`와 다른 색은 실제 기본 runtime에서 거부됨을 확인했다. 모든 개별 속성 조합을 실행한 것은 아니다. |
| 12 | 비-LTR 또는 지원하지 않는 word-break가 첫 cell 원점을 바꿈 | 방향·word-break·writing mode 검사가 제한 profile에 남아 있다. 기준 fixture는 LTR/horizontal이다. 대체 방향 입력의 거부는 코드 검사이며 별도 matrix는 없다. |
| 13 | anonymous item에 element의 grid placement가 잘못 전파됨 | source anchor는 Grid의 실제 text child이고 generated item은 Grid child다. §8.5는 anonymous item을 항상 auto-place한다. 기준 geometry 테스트에서 첫 cell에 놓였다. |
| 14 | `auto` 크기 허용이 일반 Grid item 전체로 새어 나감 | `Auto` dimension 예외는 `root_html_default` 표식과 지정 parent Grid에만 적용한다. Default Grid·C11.4 회귀 통과. |
| 15 | 두 축 stretch가 아닌 content-size 결과가 성공 처리됨 | 고정 40×70 첫 cell에 익명 item frame이 정확히 일치하고 텍스트 origin은 (0,0)이다. Chrome 독립 기대값과 runtime 단위 테스트가 일치한다. |
| 16 | text frame이 문서 밖 node로 들어오거나 element frame 검증을 우회함 | 추가 frame은 실제 HostDocument text node이면서 해당 text layout이 있는 경우에만 renderer admission에 포함한다. 일반 frame 외부 node 검증은 유지했다. |
| 17 | text layout owner와 실제 painted box/color가 서로 다름 | source anchor → Grid style owner 연결을 검증하고 Grid box에 text를 한 번 attach한다. 색·두 glyph·DrawText command를 runtime test로 확인했다. |
| 18 | text가 누락·중복되거나 여러 줄로 잘못 paint됨 | `AB` scene은 text 하나, glyph 두 개, 한 baseline, glyph batch draw 하나다. Android/iOS simulator에서도 text count 1 및 present 로그를 확인했다. 픽셀 parity는 주장하지 않는다. |
| 19 | 실패 layout이 성공 또는 이전 scene처럼 publish됨 | 잘못된 sibling·텍스트·style은 `completed=None`으로 실패하고 Root HTML 환경 변경 실패 회귀도 통과한다. 강제 allocator-failure 경로는 실행하지 않았고 OOM 복구를 주장하지 않는다. |
| 20 | 제한 fixture가 일반 CSS Grid/WPT·실기기 지원으로 과장됨 | 구현·simulator 근거는 고정 `AB` slice만 다룬다. 공식 C11 상위와 `72/213`은 그대로다. Android는 API 37 ARM64 AVD, iOS는 iPhone 17 Pro Simulator이며 실기기·GPU pixel equality는 미검증이다. |

## 별도 비녹색 게이트

- 전체 `spinon-style-to-layout` 테스트는 279개 중 278개 통과했다. 실패는 변경하지 않은 C07 aspect-ratio computed serialization 비교에서 `1.7777778 / 1`과 `1.77778 / 1`이 다른 건이다. 원본은 `style-to-layout-full-tests.log`에 있다.
- 기본 대상 all-target Clippy는 통과했다. C11.4 feature를 켠 library Clippy도 통과했다. 그 feature의 all-target Clippy는 변경하지 않은 `anonymous_text_hyphens.rs` test helper의 `too_many_arguments (11/7)` lint에서 실패했다. 기록은 `clippy-c11-4-feature.log`에 있다.
- 공백-only `white-space:pre`를 새 경계 누락으로 의심했으나 W3C §6와 기존 Root HTML 회귀를 대조해 false positive로 판단했다. 변경을 추가하지 않았으며, 공백-only 시퀀스를 무시하는 기존 동작을 유지했다.
