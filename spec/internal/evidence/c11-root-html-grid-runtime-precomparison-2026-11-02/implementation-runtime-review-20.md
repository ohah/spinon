# C11 Root HTML Grid 구현/runtime 실패 관점 검토 20개

검토 대상은 현재 C11.1 root HTML integration 변경과 Android API 37 ARM64 AVD·iOS 26.2 Simulator 실행이다. 실패 통제는 기본 `RuntimeSession::new_runtime_gpu_with_root_html` entry를 사용한다. 앱 capture는 scene 제출의 보조 시각 증거이며 exact item geometry의 기준은 Rust runtime test다.

1. **제품 session entry 선택** — fixture 전용 constructor가 아니라 `new_runtime_gpu_with_root_html`에서 실행되는가? **통과:** focused positive/negative tests가 기본 entry를 직접 호출한다.
2. **Computed profile 회귀** — viewport/color-scheme을 소유한 Root HTML computed profile을 바꾸지 않았는가? **통과:** `RuntimeRootHtmlTextPaintV1`을 유지하며 `root_html_` runtime regression 17/17 통과.
3. **NodeId 결합** — HostNode ID가 달라져도 selector 기반 Grid가 유지되는가? **통과:** 숨긴 prefix를 넣은 alternate 문서와 새 source tree가 같은 CSS geometry를 낸다.
4. **root tag 결합** — `<main>` tag 고정 분기가 없는가? **통과:** alternate `<section id="grid-node">`가 같은 tracks/frame을 낸다.
5. **selector 전달** — class selector를 id/attribute selector로 동등하게 바꿔도 cascade 결과가 같은가? **통과:** alternate 문서가 같은 red/green frame을 냈다.
6. **Typed track 값** — grid template이 문자열 비교에 머물지 않고 typed Grid layout으로 전달되는가? **통과:** test가 `40px 80px`, `70px`, default `grid-auto-flow: row`, `grid_layout.is_some()`을 확인한다.
7. **축 순서** — CSS column/row 값을 서로 바꾸거나 row track을 누락하면 expected frame과 맞지 않는가? **통과:** positive assertion이 root `120×70`, 두 item의 정확한 frame을 독립 비교한다.
8. **Visible overflow 경계** — authored item이 track/root 크기를 넘을 때 root frame 확장이나 track clamp로 숨겨지지 않고, authored clipping이 silently ignored 되지 않는가? **통과:** 별도 fixture에서 root `(0,0,120,70)`, item `(0,0,41,71)`·`(40,0,81,70)`을 확인했다. Agent Browser Chrome 156의 `getBoundingClientRect()` 및 right/bottom scroll overflow와 일치한다. CSS `overflow:hidden`/`clip`은 Grid container/item에서 projection 전에 거부된다.
9. **기본 auto-flow** — author 선언 없이 CSS 초기 `row` 값이 사용되는가? **통과:** computed `grid-auto-flow`와 source-order positive case를 검사한다.
10. **DOM 순서 변경** — 다른 NodeId/tag를 유지한 채 DOM 순서를 바꾸면 색상별 frame이 column에 따라 이동하는가? **통과:** red item은 column 2, green item은 column 1로 이동하고 각 고정 size는 유지한다.
11. **Explicit placement 경계** — author `grid-column`이 silently ignored되거나 auto로 낮아지지 않는가? **통과:** Root HTML cascade는 `cascade_failed`이며 scene을 발행하지 않는다.
12. **Non-default auto-flow 경계** — `grid-auto-flow:column`이 row로 무시되지 않는가? **통과:** `cascade_failed`, partial scene 없음.
13. **Track function 경계** — `repeat()`, `1fr`, percentage track이 fixed px로 잘못 변환되지 않는가? **통과:** 각각 root profile cascade gate 또는 fixed-track validator의 명시 오류로 실패하고 partial scene은 없다.
14. **Gap 경계** — nonzero `column-gap`이 zero gap으로 잘못 정규화되지 않는가? **통과:** `cascade_failed`, partial scene 없음.
15. **Box edge/position 경계** — nonzero Grid padding 또는 positioned Grid box가 예상 track geometry인 것처럼 성공하지 않는가? **통과:** spacing validator가 Grid containers/items의 margin·padding·border를 검사하고, non-static `position`은 current author cascade gate에서 거부된다. Runtime mutation test는 padding을 직접 layout failure로, position을 cascade failure로 확인했다.
16. **Grid writing direction 경계** — vertical writing mode나 RTL inline direction이 horizontal LTR track 좌표로 오인되지 않는가? **통과:** vertical writing mode는 cascade gate에서, `direction:rtl`은 computed Grid container/item validator에서 거부되고 partial scene이 없다.
17. **Anonymous text item 경계** — non-whitespace direct text가 `white-space:pre`에서도 조용히 사라지지 않고, whitespace-only sequence가 item count를 바꾸지 않는가? **통과:** preflight는 non-whitespace text를 항상 `unsupported_root_html_grid_text_item`으로 거부하고, typed tree 수집은 `white-space:pre`에서도 whitespace-only sequence를 건너뛴다. 두 동작 모두 [CSS Grid §6](https://www.w3.org/TR/css-grid-2/#grid-items) 규칙에 맞춘 positive/negative runtime assertions로 확인했다.
18. **Paint-order owner 누락** — `display:grid` parent box의 computed background가 scene filtering에서 빠지지 않는가? **통과:** 발견한 omission을 `runtime_document_order`의 Grid admission으로 고쳤고 positive test가 부모 배경 `(0,0,120,70)`을 assert한다.
19. **Cross-platform source identity** — 두 앱이 같은 HTML/probe를 default runtime route에서 읽었는가? **통과:** 최종 source manifest가 working tree Rust·fixture hash를 고정한다. Android base/overflow APK와 iOS 설치 `.app` asset hash는 각각 해당 fixture `index.html`과 probe hash `465eb5…`에 일치한다. API 37 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 모두 `layout=ready`, `boxes=5`, root height 70px을 기록했고 최종 visible-overflow screenshot을 남겼다. [base manifest](./platform-runtime-observation.json), [overflow manifest](./platform-visible-overflow-observation.json). PixelCopy/Metal drawable은 수집하지 않았다.
20. **제품 artifact·status 경계** — test-only Rust C11 fixture ABI가 이번 제품 빌드에 들어가거나 이 slice가 공식 C11 완료로 승격되는가? **통과:** 두 앱 모두 `C04_RUNTIME_GPU_FIXTURE_TEST=0`; Android `.so`와 iOS executable에서 `spinon_runtime_gpu_host_new_c11_1_grid_fixture` 문자열은 찾지 못했다. iOS app에는 기존 Swift demo의 `C11.1 Grid scene` UI 문자열이 남아 있어 전체 C11 문자열 부재를 주장하지 않는다. [검사 결과](./artifact-boundary.txt). C11/C11.4와 공식 완료는 72/213 그대로다.

## 결과와 남은 경계

검토에서 고친 실제 결함은 Root HTML renderer의 visible-node/paint-node filter가 `display:block`만 포함해 Grid container paint를 버리던 문제, fractional/percentage tracks admission, `white-space:pre` 아래 직접 Grid 텍스트 경계였다. 추가 경계 확인에서 inline Grid item은 Stylo가 blockify한 결과를 positive control로 검사하고, flex item과 nested Grid는 제한 profile에서 fail-closed하도록 했다. Chrome 156 visible overflow 결과와 Rust runtime frame assertion도 일치했다. C11.1 focused runtime 2/2, Root HTML 회귀 17/17, formatting과 scoped Clippy가 통과했다. `spinon-style-to-layout` 전체 실행은 276/277이며 기존 C07.3 aspect-ratio 직렬화 차이 1개가 남아 있다. 실기기·hardware GPU, Android PixelCopy, iOS compositor 이후 readback, 전체 WPT/일반 CSS Grid 적합성은 이 검토에 포함하지 않았다.
