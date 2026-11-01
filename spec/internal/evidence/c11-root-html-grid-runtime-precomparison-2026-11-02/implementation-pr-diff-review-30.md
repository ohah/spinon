# C11.1 구현 PR 변경분 적대 검토 30개

검토 대상은 root HTML 기본 renderer·typed Grid projection·layout/paint 연결, Root HTML Grid tests와 fixture, 앱/Chrome 실행 근거 및 계획 변경이다. 기존 계획 20개와 구현/runtime 20개 목록을 복사하지 않고, PR diff에 적용될 회귀·입력 변형·산출물 경계별 실패를 따로 확인했다. 코드는 현재 후보 소스 기준으로 검토했고, 실행 근거는 기록된 현재 소스 test·simulator 로그와 연결했다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | fixture constructor만 우연히 Grid를 받아들이는가? | 테스트 실행 helper가 `RuntimeSession::new_runtime_gpu_with_root_html`을 직접 호출한다. 전용 fixture constructor는 사용하지 않는다. PASS |
| 2 | root HTML의 기존 계산 profile을 Grid profile로 바꾸어 viewport/environment를 잃는가? | `RuntimeRootHtmlTextPaintV1`을 유지하고 그 profile에서만 제한 통합을 더한다. Root HTML 회귀 17/17이 통과했다. PASS |
| 3 | 다른 runtime profile의 display admission도 함께 넓어지는가? | `runtime_frames`의 `grid` 허용은 Root HTML text profile 분기 안에 한정되어 있다. 기존 fixture profile 분기는 그대로다. PASS |
| 4 | CSS 문자열을 다시 파싱하거나 node 이름으로 layout 입력을 합성하는가? | `ComputedCssGridLayout`을 typed projection에서 소비하고, root/item 선택은 cascade된 tree 관계로 한다. PASS |
| 5 | root tag, id, class, HostNode ID가 고정되어 있는가? | tag와 selector 종류, 숨김 prefix 및 node IDs를 바꾼 positive control에서도 같은 root/tracks와 의미상 item frame을 확인했다. PASS |
| 6 | root container가 계산돼도 paint node 필터에서 빠지는가? | Root HTML runtime paint tree가 visible `display:grid` node를 포함한다. positive assertion에서 root 배경 frame `(0,0,120,70)`이 유지된다. PASS |
| 7 | grid item owner가 직계 자식이 아닌 후손까지 잘못 확장되는가? | `runtime_grid_nodes`는 각 Grid container의 직계 visible children만 item owner에 연결한다. nested Grid는 이 slice에서 scene publish 전에 거부된다. PASS |
| 8 | `display:none` 자손이 보이는 item처럼 capacity를 소비하거나 text 오류를 내는가? | hidden ancestry와 computed `display:none`을 item 수집·Grid text preflight에서 제외한다. 코드 경로 검토 PASS |
| 9 | `white-space:pre`의 들여쓰기 공백이 익명 item으로 세어지는가? | CSS whitespace-only text sequence를 typed item map에서 제외한다. `white-space:pre` positive test가 같은 tracks/frames를 확인했다. PASS |
| 10 | non-whitespace 직접 텍스트가 조용히 사라져 성공하는가? | computed white-space와 무관하게 `unsupported_root_html_grid_text_item`으로 거부하고 completed scene을 발행하지 않는다. 두 negative test가 확인했다. PASS |
| 11 | 직접 자식 이미지가 intrinsic sizing 없이 빈 fixed box인 것처럼 처리되는가? | Root HTML Grid projection은 HTML image item을 명시 거부한다. 코드 경로 검토 PASS |
| 12 | Grid track 수가 overflow하거나 item 수와 맞지 않아 implicit track이 생기는가? | item count는 checked addition, explicit capacity는 checked multiplication을 사용하고 초과 item을 거부한다. 초과 item negative case가 partial scene 없이 실패했다. PASS |
| 13 | `-1px`, `NaN`·무한대 또는 길이 아닌 track이 고정 px처럼 전달되는가? | Root HTML 전용 validator는 유한하고 0 이상인 `LengthPx`만 허용한다. 코드 경로 검토 PASS |
| 14 | `fr`, percentage, intrinsic sizing 또는 `repeat()`이 fixed px로 낮춰지는가? | track validator가 px 이외와 auto-repeat을 거부한다. fixture 변형에서 `1fr`, percentage, `repeat()`을 확인했다. PASS |
| 15 | line names와 template areas가 무시된 채 unnamed grid로 성공하는가? | line-name metadata와 `grid-template-areas`를 거부한다. named area negative control이 확인했다. PASS |
| 16 | authored placement 또는 `grid-auto-flow:column|dense`가 row 흐름으로 무시되는가? | non-default placement는 Root HTML cascade gate에서 거부되고 typed auto-flow는 Row만 받는다. explicit placement와 column flow negative controls가 확인했다. PASS |
| 17 | `grid-auto-rows`·`grid-auto-columns`의 명시 sizing이 적용되지 않는데 성공하는가? | auto sizing 기본값만 허용한다. `grid-auto-rows:20px` 변형이 거부됐다. PASS |
| 18 | inline Grid item의 spec blockification을 지원하지 않아 유효 입력을 거부하는가? | computed `display:block`을 소비하고, authored inline item positive control이 기본 scene과 같은 frame을 낸다. PASS |
| 19 | flex item을 block box로 조용히 바꾸어 계산하는가? | computed item display가 block이 아니면 거부한다. `display:flex`는 제한 profile에서 실패한다. PASS |
| 20 | 중첩 Grid가 child track을 부모 item의 크기와 혼합하는가? | nested Grid 변형은 현재 cascade 경계에서 `cascade_failed`로 닫히며 scene이 없다. PASS |
| 21 | margin/padding의 nonzero·계산값이 0으로 간주되는가? | 각 네 변의 typed spacing과 math-backed 값을 확인해 nonzero/계산 값을 거부한다. padding/margin negative controls가 확인했다. PASS |
| 22 | border width가 track 또는 item frame 안에 흡수되는가? | 네 변의 computed border width가 0이 아니면 거부한다. border negative control이 확인했다. PASS |
| 23 | `gap:normal`과 0 gap의 정상 초기값까지 거부하거나 nonzero gap을 놓치는가? | Grid container에만 row/column gap을 검사하고 normal 또는 0px만 받는다. nonzero column gap 변형을 거부했다. PASS |
| 24 | `position:relative` offset이 frame 계산에 빠진 채 성공하는가? | non-static position을 거부한다. relative 변형은 expected layout failure로 끝난다. PASS |
| 25 | `overflow:hidden|clip`을 무시하면서 visible overflow라고 주장하는가? | container와 item 모두 computed overflow가 visible이어야 한다. 두 clipping 변형이 거부됐다. PASS |
| 26 | RTL 또는 vertical writing mode를 LTR horizontal geometry로 오해하는가? | `direction:ltr`, `writing-mode:horizontal-tb`만 허용한다. RTL·vertical 변형이 거부됐다. PASS |
| 27 | default row auto-placement에서 DOM source order가 semantic item 위치와 분리되는가? | order reversal control에서 root tracks는 유지되고 semantic red/green item만 서로 다른 column으로 이동한다. PASS |
| 28 | authored item이 track/root를 넘을 때 frame clamp나 root expansion으로 overflow가 사라지는가? | 별도 fixture의 item frame `(0,0,41,71)`·`(40,0,81,70)`와 root `(0,0,120,70)`을 검사했다. Chrome 156 oracle과 일치하고 앱 화면에서도 경계 바깥 면이 보인다. PASS |
| 29 | 변경이 cache/revision을 넘어선 stale document/style/environment 결과를 섞는가? | root HTML test가 document/style/environment/layout revision identity를 확인한다. renderer도 기존 revision validation을 통과해야 scene을 만든다. PASS |
| 30 | 앱 화면이 fixture ABI 또는 다른 source build를 증명하는 것처럼 과장되는가? | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 default route·asset·binary hashes를 각각 기록했다. fixture-test feature는 꺼져 있고 marker scan과 화면 캡처 한계를 명시했다. PixelCopy, iOS drawable readback, hardware GPU, 실기기 근거라고 주장하지 않는다. PASS |

## 판정

**PASS 30 / 발견된 PR 차단 결함 0.** 검토는 제한된 C11.1 Root HTML Grid slice에 한정한다. Root HTML runtime 17/17과 focused Grid 2/2는 저장된 현재 소스 실행 로그가 근거다. `spinon-style-to-layout` 전체는 276/277이며, 기존 C07.3 aspect-ratio serialization 차이 하나가 남아 있다. 그 실패는 이 diff의 변경과 무관하고 상태 대장에 이미 기록된 건이다. C11 parent·C11.3·C11.4 및 공식 완료 집계는 이 검토로 바뀌지 않는다.
