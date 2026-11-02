# PR #1056 변경분 실패 관점 검토 30개

대상은 `origin/main...feat/c11-root-html-grid-column-flow-20261102` 최종 diff다. C11.1 계획 검토 20개 및 구현/runtime 검토 20개와 별도로 코드, projection, 회귀, 문서, Android/iOS evidence 및 PR metadata를 다시 확인했다.

| # | 검토한 실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | CSS `grid-auto-flow:column`이 row 흐름으로 잘못 typed 변환될 수 있음 | PASS. Stylo cascade가 `Column`을 보존하고 runtime에서 typed value를 검사한다. |
| 2 | Root HTML이 아닌 fixture/profile에도 새 placement mode가 노출될 수 있음 | PASS. mode 선택은 `RuntimeRootHtmlTextPaintV1` route에만 연결되며 opt-in fixture route는 건드리지 않았다. |
| 3 | 여러 Grid 또는 중첩 Grid 중 일부만 column-flow로 선택할 수 있음 | PASS. candidate validator가 단일 non-nested container만 허용하고 nested Grid item은 거부한다. |
| 4 | Column sparse 순서가 row sparse 또는 reverse로 섞일 수 있음 | PASS. 2×2 fixture의 네 semantic frame 및 source-order reverse control을 검사한다. |
| 5 | 기존 기본 row placement가 regression될 수 있음 | PASS. 동일 runtime fixture의 row-flow control과 기존 Root HTML Grid 37/37 회귀가 통과했다. |
| 6 | row/column gap을 잘못된 축에 더하거나 두 번 적용할 수 있음 | PASS. layout core와 runtime에서 비대칭 `3px`/`5px` gap frame을 각각 검사한다. |
| 7 | CSS `order`가 문서 순서를 덮어쓰거나 순서 동률이 불안정할 수 있음 | PASS. 비영 order는 실패하고 source order는 semantic item 대조군으로 확인했다. |
| 8 | NodeId, tag 이름, class selector 또는 `main`에 결합된 구현이 다른 문서에서 작동하지 않을 수 있음 | PASS. 대체 `div` root, ID selector 및 hidden prefix 대조군이 같은 geometry를 만든다. |
| 9 | hidden item이 capacity를 차지하거나 다음 item 위치를 밀 수 있음 | PASS. `display:none` item은 visible count와 scene에서 제외된다. |
| 10 | 5번째 visible item이 implicit column/row를 조용히 만들 수 있음 | PASS. 2×2 explicit capacity 초과를 layout 전에 거부한다. |
| 11 | 배치 normalizer가 explicit track 밖의 line을 반환할 수 있음 | PASS. normalized placement의 두 축 line 범위를 재검사하고 expanded track result를 거부한다. |
| 12 | row/column dense 키워드를 sparse profile로 잘못 분류할 수 있음 | PASS. `RowDense`/`ColumnDense` 모두 rejection matrix에 있다. |
| 13 | numeric·named·span placement를 auto placement로 축소할 수 있음 | PASS. 세 authored placement 형태를 각각 runtime rejection matrix로 확인한다. |
| 14 | `%`, `fr`, intrinsic, 음수 또는 비유한 explicit track이 px로 투영될 수 있음 | PASS. typed projection과 layout contract가 non-negative finite `LengthPx`만 허용한다. |
| 15 | `none` 또는 빈 explicit track이 0-cell grid로 처리될 수 있음 | PASS. empty columns와 rows negative case 및 core nonempty 검사가 있다. |
| 16 | Stylo가 fixed `repeat()` 확장 뒤 authored group을 잃어 지원하지 않는 syntax를 허용할 수 있음 | PASS. axes별 fixed-repeat metadata를 snapshot에 보존하고 projection 전에 검사한다. |
| 17 | auto-repeat, named explicit line 또는 named area가 새로운 route에서 우회될 수 있음 | PASS. 두 축의 repeat count, line-name group, `grid-template-areas`를 각각 거부한다. |
| 18 | `grid-auto-rows`/`grid-auto-columns` 비-auto 값이 implicit sizing을 일으킬 수 있음 | PASS. style projection과 layout core 양쪽에서 `Auto`만 허용한다. |
| 19 | RTL/vertical writing mode에서 physical column 순서가 불명확할 수 있음 | PASS. computed `direction:ltr`와 `writing-mode:horizontal-tb`를 요구한다. |
| 20 | nonempty direct div 이외 text, nested content, image/replaced item이 빈 fixed box처럼 처리될 수 있음 | PASS. HTML element shape와 item dimensions를 검사하고 direct text/nested/image cases를 거부한다. |
| 21 | min/max size, aspect ratio 또는 item CSS math가 fixed track 결과를 바꿀 수 있음 | PASS. 해당 item style 값을 거부하며 negative runtime cases를 실행했다. |
| 22 | hidden subtree 또는 display 상태가 layout `NodeId` map과 불일치할 수 있음 | PASS. visible child filter를 HostDocument owner map과 layout hidden map 양쪽에서 확인한다. |
| 23 | capacity 곱셈 overflow가 wrap되어 잘못 성공할 수 있음 | PASS. projection과 layout contract 모두 `checked_mul`을 사용한다. |
| 24 | rejection 중 partial `RuntimeLayoutSnapshot` 또는 scene이 공개될 수 있음 | PASS. 각 negative runtime case에서 `Failed`, error 보존과 `completed=None`을 assert한다. |
| 25 | 추가 visible-item list 할당이 strict validation 도중 실패할 수 있음 | PASS. review에서 validation 복사를 발견해 iterator count/scan으로 바꿨고 영향받은 suites를 다시 실행했다. |
| 26 | fixed-repeat count를 추가한 뒤 기존 computed Grid struct literal/fixture가 누락될 수 있음 | PASS. 모든 기존 test literal에 기본 metadata를 채웠고 scoped all-target Clippy가 전 crate targets를 확인했다. |
| 27 | C11.1의 이미 병합된 percentage implicit-row 기대값과 이 변경이 충돌할 수 있음 | PASS. stale negative를 제거하고 positive percentage-list regression을 보존했다. Root HTML Grid regression은 37/37이다. |
| 28 | Chrome reference 값만 맞고 typed projection/runtime은 어긋날 수 있음 | PASS. 독립 JSON expected, computed typed tracks와 semantic frame을 별도로 검사한다. Full style-to-layout 305/306 C07 실패는 숨기지 않았다. |
| 29 | 앱 bundle에 이전 fixture 또는 이전 Rust source가 포함되어 simulator 결과가 낡을 수 있음 | PASS. 최종 source 변경 뒤 두 플랫폼을 재빌드했고 HTML SHA-256, APK/executable hash, `layout=ready`, 7-box draw/present를 기록했다. |
| 30 | 제한 구현이 공식 C11 완료·CI green·pixel parity로 과장되거나 PR screenshot 링크가 깨질 수 있음 | PASS. `STATUS.md` parent/C11.1을 미완료로 두고 `72/213`을 유지한다. PR checks 0건을 확인했고 Agent Browser에서 Android/iOS image `naturalWidth` 1080/1206 로드를 확인했다. |

## PR 상태와 남은 범위

- PR #1056의 merge state는 `CLEAN`, GitHub status check rollup은 비어 있으며 `gh pr checks`는 `no checks reported`를 반환했다. CI 통과로 간주하지 않는다.
- 이 검토는 제한 Root HTML sparse column-flow slice만 다룬다. 전체 CSS Grid/WPT, dense 및 explicit placement, implicit sizing, 실기기/hardware GPU와 compositor 후 pixel parity를 증명하지 않는다.
