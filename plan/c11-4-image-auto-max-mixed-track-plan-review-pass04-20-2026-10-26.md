# C11.4 image auto/max mixed-track plan review — pass04 · 새 20관점

**검토 기준:** checkout/`origin/main` `a33b245983fae050b66efb9ff259b3fe1132a051`. pass03 뒤 수정된 standalone image 계획, HTML fixture, precomparison JSON/README/metadata/screenshot을 다시 대조했다. 이 review는 새 20개 관점으로 계획 경계를 검토한다. CSSOM 재캡처는 전용 Agent Browser로 했고 session은 종료 확인했다. 제품 코드, parent plan, pass01–pass03 리뷰, 계획 PR diff-30 FAIL 보고서는 수정하지 않았다. Rust tests와 simulator는 실행하지 않았다.

## 판정

**PASS 18 / PARTIAL 2 / FAIL 0. Implementation-start gate: CLEAR** for the bounded image sizing slice. Positive input에서 authored overflow declaration을 제거했고 Chrome CSSOM의 실제 `clip/clip`을 기록했다. Plan/evidence에서 app overflow value, projection, adapter guard, clip/scroll/paint acceptance와 overflow negative/error matrix를 제거했다. 남은 두 partial은 현재 implementation 선택과 무관한 raw-source/audit-log provenance다.

이 plan review의 clear 판정은 이전 plan PR diff-30 report를 대체하지 않는다. 그 보고서는 옛 authored overflow payload를 대상으로 한 FAIL rows를 그대로 보존하며, parent가 요청한 targeted recheck를 나중에 해야 한다. 이 review는 parent plan을 수정하지 않았다.

## 새 실패 관점 20개

| # | 검토 관점 | 판정 | 근거와 경계 |
|---:|---|:---:|---|
| 1 | 단일 fixture가 여러 feature를 묶어 Stage B 경계를 흐리는가 | PASS | 계획 범위는 direct single PNG, span 1, `minmax(auto,max-content) 17px` 두 column으로 제한돼 있다. 다른 text/image 조합과 row-axis/general admission은 제외한다. |
| 2 | positive HTML/CSS가 overflow를 우회 author하는가 | PASS | [HTML fixture](../tests/fixtures/css/c11/grid-image-auto-max-mixed-track-c114-precomparison.html)의 image CSS rule에서 `overflow` 선언을 제거했다. observation JSON `authored.imageRule`에도 overflow 선언이 없다. |
| 3 | Chrome computed overflow가 CSS initial `visible`로 잘못 단정되는가 | PASS | 새 Agent Browser observation은 Chrome 156 `<img>` `overflowX/Y=clip/clip`을 기록한다. Plan과 README는 이를 CSSOM의 UA computed value로 한정한다. |
| 4 | `clip/clip` 관측을 Spinon overflow 구현 증거로 확대하는가 | PASS | Chrome 값은 §6.6 applicability context로만 언급한다. Spinon app computed value와 projection은 acceptance 밖이다. |
| 5 | sizing scope가 clip, scroll, paint semantics를 약속하는가 | PASS | Plan, README, JSON은 overflow extraction, projection, clipping, scrolling, paint behavior를 scope 밖으로 명시한다. |
| 6 | 삭제한 overflow adapter guard/error matrix가 JSON에 남는가 | PASS | `overflow_axis_guard`, authored-overflow error cases, `Some(Visible)`/`LayoutOverflow::Visible` acceptance는 expected JSON에서 제거됐다. JSON은 `overflow_scope`만 둔다. |
| 7 | overflow controls가 다른 unsupported CSS 계약과 섞여 다시 도입되는가 | PASS | Fail-closed matrix에는 기존 image/grid/source controls만 남고 overflow rejection controls는 없다. |
| 8 | intrinsic input이 image decode 전 관측되는가 | PASS | observation은 `complete=true`, natural dimensions `24×16`을 보존한다. |
| 9 | item sizing constraints가 natural-size 계산을 바꾸는가 | PASS | CSSOM은 width/height `24px/16px`, min `auto`, max `none`, zero padding/border/margin을 기록한다. |
| 10 | mixed tracks/gutter와 최종 frame 산술이 맞지 않는가 | PASS | Chrome computed columns `24px 17px`, row `16px`, gap `5px`; root/image rect는 `(0,0,100,40)`/`(0,0,24,16)`이다. 독립 규범 산술과 일치한다. |
| 11 | CSS Grid §6.6 automatic minimum 전제가 불가능한가 | PASS | Chrome UA `clip/clip`은 non-scrollable computed overflow이므로 CSSOM 입력의 applicability context다. 이 판정은 Spinon의 overflow behavior를 증명한다고 주장하지 않는다. |
| 12 | AutomaticMinimum source의 정밀도가 빠졌는가 | PASS | JSON에 두 축의 전체 flags, replaced bit, fixed sum/gutter/limit `null`, contribution 24px/16px와 min/max-content 값을 유지한다. |
| 13 | Taffy trace `null`을 내부 `+∞`로 오독하는가 | PASS | Plan/README/JSON은 모든 non-finite 값(+∞/−∞/NaN)→`None/null` projection과 finite stage의 `Some(finite)` tolerance 비교를 분리한다. |
| 14 | normative expected stages와 설명 표가 어긋나는가 | PASS | Columns/Rows의 Initialize, item batch, completion, maximize, flex, stretch stage arrays가 JSON과 계획 표에서 일치한다. |
| 15 | fixture, observation, metadata, screenshot이 서로 다른 input을 가리키는가 | PASS | 현재 SHA는 HTML `e3b6edfe…06c92c2b`, observation `82c020de…ce635f4`, metadata `5043ffde…8f1bb90`, screenshot `4182f752…6625eb`다. JSON/README provenance와 일치한다. |
| 16 | recapture가 branch baseline과 맞지 않는가 | PASS | Metadata/observation 및 JSON capture source는 current `a33b245983fae050b66efb9ff259b3fe1132a051`이다. 초기 `b58de5df` capture는 historical review context로 분리했다. |
| 17 | 전용 Agent Browser가 남아 있거나 사용자 브라우저를 종료했는가 | PASS | Metadata cleanup summary는 session `active:false,pid:null`, active session list empty, dedicated profile process match 0을 기록한다. User Chrome/HWPJS session은 보존했다. 원문 CLI/process transcript는 저장하지 않았다(관점 #20). |
| 18 | runtime/native 결과가 아직 계획인 것을 완료 증거처럼 표현하는가 | PASS | README와 plan은 runtime, simulator, native screenshots, artifact, paint/readback을 미실행 future gates로 둔다. 이 pass는 tests/simulator를 실행하지 않았다. |
| 19 | CSS Images §4.3.1 source pin을 원문 preimage까지 재현할 수 있는가 | PARTIAL | JSON/README는 2023-12-18 CRD URL, sections, response byte 수/SHA를 고정했으나 원문 raw response 파일은 evidence 폴더에 없다. Bounded implementation contract를 바꾸지 않는 provenance 개선이다. |
| 20 | Agent Browser close/process claim에 원문 audit transcript가 보존되는가 | PARTIAL | Metadata는 session inactive, empty active list, dedicated process match 0을 기록하지만 raw CLI/process transcript는 저장하지 않았다. Closure summary는 남아 있으며 이는 bounded implementation contract를 바꾸지 않는 provenance 개선이다. |

## 비차단 provenance partials

- **CSS Images raw bytes:** dated W3C URL, size, SHA-256은 남지만 raw response preimage는 저장하지 않았다.
- **Browser cleanup audit log:** session inactive, active-session list empty, dedicated Chrome/crashpad match 0은 metadata에 기록했지만 원문 command transcript는 보존하지 않았다.

둘 다 이 plan에서 implementation start를 막지 않는다. 이전 plan PR diff-30 FAIL report는 변경하지 않았으며, parent가 요청한 targeted recheck 대상으로 보존한다.
