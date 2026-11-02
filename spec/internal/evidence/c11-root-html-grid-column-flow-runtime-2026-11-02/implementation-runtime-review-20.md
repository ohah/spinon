# C11.1 Root HTML sparse column-flow 구현·runtime 실패 관점 검토 20개

검토 범위는 `RootHtmlColumnFlowV1` placement, Stylo computed metadata, Root HTML projection, 고정 runtime fixture 및 Android/iOS 기본 app-route 결과다. 기존 계획 검토 20개와 분리해 실제 변경 코드와 실행 결과를 다시 확인했다. 아래 검토는 제한 후보의 안전 경계이며 전체 CSS Grid 적합성을 의미하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | `grid-auto-flow:column`이 열 우선이 아니라 행 우선으로 배치될 수 있음 | PASS. 2×2 기대 순서 A/B가 첫 열, C/D가 둘째 열에 배치된다. Runtime frame 검사와 layout-core source-order 검사가 통과했다. |
| 2 | explicit rows/columns 축을 서로 뒤바꾸거나 capacity를 잘못 계산할 수 있음 | PASS. typed `[30,70]` columns·`[20,40]` rows와 4-cell capacity를 projection/layout 양쪽에서 사용한다. |
| 3 | row/column gap이 잘못된 축에 적용되거나 두 번 더해질 수 있음 | PASS. `row-gap:3px; column-gap:5px`에서 Green `(0,23)`, Blue `(35,0)`, Yellow `(35,23)`을 확인했다. |
| 4 | 새 mode가 기존 기본 row flow 회귀를 유발할 수 있음 | PASS. 같은 HTML에서 row-flow control의 네 semantic item frame을 검사했다. |
| 5 | item fixed size가 track 크기와 어긋나도 성공 scene을 만들 수 있음 | PASS. 독립 fixture의 semantic frame을 Chrome 156 기대값과 직접 비교하며, item 크기는 고정 finite px로 제한한다. |
| 6 | CSS source order가 아니라 NodeId나 map iteration 순서로 배치될 수 있음 | PASS. semantic color별 frame을 사용하고, DOM source order를 뒤집은 대조군을 실행했다. |
| 7 | `main`, class명, selector 또는 사전 할당 NodeId에만 의존할 수 있음 | PASS. 대체 root `div`, ID selector와 숨겨진 prefix를 사용한 실행에서도 같은 geometry가 나왔다. |
| 8 | `display:none` item이 capacity를 차지하거나 visible item 순서를 밀 수 있음 | PASS. 숨긴 첫 item을 scene과 배치 순서에서 제외했고, 나머지 세 item은 첫 셀부터 시작했다. |
| 9 | 2×2보다 item이 하나 많을 때 implicit cell을 조용히 추가할 수 있음 | PASS. 5번째 item은 배치 전에 거부되고 error가 남는다. |
| 10 | normalizer가 범위 밖 line 또는 implicit row/column expansion을 반환할 수 있음 | PASS. 지원 결과는 expanded track을 거부하고 배치된 각 item의 양 축 line이 explicit 범위 안에 있는지 재검사한다. |
| 11 | `column dense` 또는 `row dense`가 sparse mode로 잘못 선택될 수 있음 | PASS. 두 computed dense 값 모두 projection에서 거부되고 runtime negative matrix도 확인했다. |
| 12 | authored numeric, named-line 또는 span placement를 auto로 낮춰 계산할 수 있음 | PASS. 세 입력을 각각 runtime rejection matrix로 검증했다. |
| 13 | percentage, `fr`, intrinsic 또는 음수·비유한 explicit track이 px처럼 변환될 수 있음 | PASS. computed projection은 direct finite non-negative px만 허용한다. `%` 및 `fr` runtime 대조군은 scene 없이 실패했다. |
| 14 | `none`/빈 explicit track list 또는 0 capacity를 정상 grid로 오인할 수 있음 | PASS. 빈 columns/rows를 각각 실패시켰고 layout contract도 두 축의 nonempty를 요구한다. |
| 15 | Stylo가 확장한 fixed `repeat()`을 plain track list로 오인하거나 line names를 버릴 수 있음 | PASS. typed fixed-repeat group count를 snapshot에 보존하고 columns와 rows의 repeat를 각각 거부한다. 이름 있는 explicit line도 거부한다. |
| 16 | `grid-template-areas`가 별도 placement와 겹쳐 auto-flow 결과를 바꿀 수 있음 | PASS. area map을 runtime negative input으로 확인했고 style/layout 경계 모두 거부한다. |
| 17 | `grid-auto-rows`/`grid-auto-columns`가 implicit sizing을 발생시킬 수 있음 | PASS. 각 축의 비-auto 값을 거부하고 layout core에서도 둘 다 `Auto`인지 확인한다. |
| 18 | RTL 또는 vertical writing mode에서 물리 axis와 CSS column 흐름을 혼동할 수 있음 | PASS. `direction:rtl`과 `writing-mode:vertical-rl`은 Root HTML projection에서 거부된다. |
| 19 | `order`, nested content/Grid, direct text, image 또는 intrinsic/min/aspect item을 고정 box로 오인할 수 있음 | PASS. `order != 0`, 비어 있지 않은 div, 중첩 Grid, image, `min-width`, `aspect-ratio` negative cases는 모두 실패한다. |
| 20 | capacity/shape 오류가 일부 layout이나 scene을 먼저 publish할 수 있거나 검증용 visible-item 복사가 추가 실패점이 될 수 있음 | PASS. rejection matrix는 `completed=None`과 error 보존을 확인한다. 리뷰 중 projection/layout prevalidation의 불필요한 visible-item `Vec` 복사를 발견해 iterator count/scan으로 바꾼 뒤 focused suites를 재실행했다. |

## 실행 결과

- Layout core 4/4, Root HTML projection 3/3, runtime feature 4/4, Root HTML Grid regression 37/37
- Scoped Clippy와 `cargo fmt --all -- --check` 통과
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 모두 `layout=ready`, 7-box draw/present; 같은 고정 HTML asset hash
- 원본 log, screenshots, artifact hashes 및 전체-suite 단일 실패는 [runtime evidence README](README.md)에 연결했다.

## 남은 경계

전체 `spinon-style-to-layout` suite는 305/306이며 변경 경로 밖의 기존 C07 aspect-ratio serialization 기대 차이가 남아 있다. Simulator screenshot은 시각 확인만 했고 surface pixel parity, 실기기/hardware GPU, WPT 또는 전체 C11/C11.1 완료를 주장하지 않는다. 이 구현 review는 PR diff 별도 30관점 검토와 PR/merge 절차를 대체하지 않는다.
