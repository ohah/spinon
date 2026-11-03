# C11.1 Root HTML Grid auto minimum 계획 실패 관점 검토 20개

검토 대상은 [계획](./c11-root-html-grid-auto-minimum.md), [고정 fixture](../tests/fixtures/css/c11/root-html-grid-auto-minimum-precomparison.html), [규범 기대값과 Chrome 관측](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/README.md)이다. 이는 구현 전에 계획 자체를 확인한 기록이다. 구현/runtime 검토에는 재사용하지 않는다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | Opt-in fixture 결과를 기본 parsed Root HTML 지원으로 오인한다 | **PASS** — 목표를 `RuntimeRootHtmlTextPaintV1` 기본 경로로 한정하고 fixture selector 경로를 금지한다. |
| 2 | CSS parser나 FFI에 별도 구현을 만들어 product path와 달라진다 | **PASS** — existing Stylo typed `MinMax` 값만 연결하고 parser/profile/ABI 추가를 제외한다. |
| 3 | 모든 `minmax(auto, …)` grammar를 한 번에 허용한다 | **PASS** — minimum `auto`, maximum finite non-negative `fr`인 shape만 추가한다. |
| 4 | 열에서는 width, 행에서는 height가 아닌 축을 basis로 쓴다 | **PASS** — 각 방향의 definite content axis를 따로 요구하고 양 축 positive matrix를 둔다. |
| 5 | 축 크기가 indefinite, zero, negative 또는 비유한 값일 때 임의 fallback한다 | **PASS** — positive definite axis만 허용하며 zero/negative/non-finite 및 missing basis를 negative matrix로 둔다. |
| 6 | computed min/max size가 track 결과를 제한·확장한다 | **PASS** — axis의 `min-size:auto`, `max-size:none`을 gate로 고정하고 override 입력을 거부한다. |
| 7 | aspect ratio가 다른 축에서 자동 minimum을 전달한다 | **PASS** — ratio 미설정 조건을 두고 aspect-ratio override를 negative로 검사한다. |
| 8 | grid item의 scrollable overflow auto minimum을 visible overflow와 혼합한다 | **PASS** — root와 item은 visible만 허용하고 item의 hidden/auto/scroll/clip override를 negative로 검사한다. |
| 9 | multi-track span에서 auto minimum의 spanning 규칙을 누락한다 | **PASS** — item이 한 track만 span하도록 두고 spanning은 제외한다. |
| 10 | `minmax(0,1fr)` control에도 auto minimum이 남는다 | **PASS** — 0 minimum control은 `[60,40]`과 border 내 second item으로 분리한다. |
| 11 | `0fr`이 auto minimum을 없애거나 positive free space를 가져간다고 가정한다 | **PASS** — 규범 기대값에 `[80,40]` 및 18px overflow 경계를 별도 기록하고 구현에서 확인한다. |
| 12 | 고정 neighbor, gap, padding 또는 border가 사용 track 계산에 중복 반영된다 | **PASS** — neighbor 40px, gap/padding 0, border 2px를 명시하고 content/outer box를 구별한다. |
| 13 | Row auto-placement가 두 번째 child를 예상하지 않은 cell에 놓는다 | **PASS** — 두 direct child, 기본 row placement, 한 explicit flex track과 fixed neighbor를 정확히 요구한다. |
| 14 | overflow가 root outer border 기준이 아니라 content edge 기준으로 보고된다 | **PASS** — item frame은 border origin 기준이고 overflow는 root border 바깥 길이로 기록한다. |
| 15 | node ID/class를 바꾸면 product 지원이 달라진다 | **PASS** — 식별자는 성공 selector가 아니며 ID/class 변경 positive control을 계획한다. |
| 16 | 미지원 breadth가 CSS syntax 오류 fallback을 통해 성공 처리된다 | **PASS** — typed value의 unsupported breadth를 node/property 오류로 fail-closed하고 CSS-invalid control과 분리한다. |
| 17 | 일부 child frame이 publish된 뒤 다른 child validation이 실패한다 | **PASS** — unsupported input에서 partial scene/frame publication이 없는지 요구한다. |
| 18 | style/document revision이 섞인 결과를 유효한 새 layout으로 수용한다 | **PASS** — runtime matrix에 source/style/layout revision identity와 revision negatives를 포함한다. |
| 19 | Chrome 화면을 규범 oracle, pixel equality 또는 native platform 증거로 과장한다 | **PASS** — spec-derived JSON, browser observation, Android PixelCopy, iOS drawable을 별도 증거로 둔다. |
| 20 | bounded slice만으로 C11.1/C11/전체 완료 수를 올린다 | **PASS** — plan/PR/STATUS 수락 기준이 상위 checkbox와 `72/213` 보존을 명시한다. |

초기 확인에서 `0fr` 경계 결과가 계획에 있었지만 독립 기대 JSON에는 빠져 있었다. 규범 계산을 expected manifest에 추가했고, 수정 뒤 20개 관점을 다시 확인했다. 계획 및 PR diff 검토는 별도 기록으로 유지한다.
