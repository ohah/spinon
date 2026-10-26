# C11.4 image auto/max mixed-track plan review — pass03

**검토 기준:** 현재 checkout `a33b245983fae050b66efb9ff259b3fe1132a051` (origin/main). image 계획, HTML fixture, precomparison evidence, 부모 계획 연결과 공식 상태를 읽기 전용으로 대조했다. 새 실패 관점 20개를 사용했다. 제품 코드·계획·fixture·evidence는 수정하지 않았다. Rust test, browser, simulator는 실행하지 않았다. 요청된 pass03 리뷰 파일만 새로 작성했다.

## 판정

**PASS 19 / PARTIAL 1 / FAIL 0.** pass02의 구현 설계 blocker 두 개는 해결됐다. Taffy trace의 `null`은 non-finite projection으로만 판정하고 내부 `+∞`와 구분하며, x-only/y-only overflow negative controls가 각각 추가됐다. branch는 최신 `main` `a33b2459`에 있고 부모 [C11.4 계획](./c11-4-grid-track-sizing.md#2026-10-26-image-automax-mixed-track-입력-계획)은 이 계획·evidence·pass02·pass03 리뷰를 연결한다. 다만 image 계획 본문에 rebase와 부모 링크를 아직 기다린다는 문구와 이전 기준 commit `b58de5df`를 “최신”으로 부르는 내용이 남아 있어 최종 계획 snapshot의 상태 기록은 정확하지 않다.

## 새 실패 관점 20개

| # | 확인한 실패 경로 | 판정 | 근거와 한계 |
|---:|---|---|---|
| 1 | 기존 PNG와 브라우저 fixture가 같은 고정 asset을 가리키지 않는가 | PASS | Evidence README와 JSON은 `c14-1-png-basic.png`, 97 bytes, `24×16`을 고정한다. PNG SHA-256 `5d8c9509…bce4a2`가 재계산값과 일치한다. HTML은 해당 asset을 상대 경로로 읽고, 앱 fixture는 같은 bytes를 `include_bytes!`와 고정 image URL로 사용한다고 계획한다. |
| 2 | image decode 전 natural size를 너무 일찍 샘플링하는가 | PASS | HTML fixture는 `image.decode()` 완료 뒤 `naturalWidth/Height`, `complete`, CSSOM과 frame을 캡처한다. Observation은 `complete=true`, `24×16`이다. |
| 3 | CSS author input과 Chrome computed input이 서로 달라지는가 | PASS | CSSOM raw JSON에는 root track/gap/alignment, item overflow·sizing·box model·placement와 frame이 있고 evidence 표의 기대값과 맞는다. 이 자료는 Chrome 비교값으로 한정돼 Stylo 결과를 대신하지 않는다. |
| 4 | box sizing, border, padding 또는 margin이 기여 산술을 바꾸는가 | PASS | root/item은 `border-box`, padding/border/margin 0이다. root `100×40`, image natural size `24×16`; CSSOM도 같은 used size를 기록한다. |
| 5 | 자동 배치 또는 다중 span이 이 single-track 기대를 무효화하는가 | PASS | HTML 입력은 direct child image 한 개, explicit placement 없음이다. CSSOM의 네 placement 값은 모두 `auto`; fixture contract는 column 1/row 1, span 1로 제한한다. |
| 6 | `minmax(auto,max-content)`의 automatic minimum 조건이 성립하지 않는가 | PASS | 계획은 track min function `auto`, item span 1, non-scrollable overflow와 replaced image natural contribution을 따로 연결한다. width/min/max 입력은 `auto/auto/none`이다. |
| 7 | overflow typed 추출이 projection의 기본 `Visible` fallback에 가려지는가 | PASS | 계획은 `RuntimeBlockFlexGridTrackSizingV1` snapshot의 두 `Some(Visible)` 필드를 projection 전 검사하고 projection 결과도 확인하도록 지정한다. 현재 Stylo extraction 경로가 해당 profile에서 x/y typed 값을 채우는지도 source와 대조했다. |
| 8 | x 필드 검사가 빠져도 negative가 통과하는가 | PASS | `(Clip, Visible)` 단일 축 control이 추가됐고 x 값이 `Some(Visible)`이 아니므로 projection/image request 전에 거부하도록 JSON과 계획을 고쳤다. |
| 9 | y 필드 검사가 빠져도 negative가 통과하는가 | PASS | `(Visible, Clip)` 대조 control이 별도로 있어 y 검사 누락을 구분한다. 두 negative 모두 아직 실행 전 acceptance다. |
| 10 | shorthand `auto/scroll/hidden` rejection이 축별 control을 대체한다고 오해하는가 | PASS | 계획은 shorthand controls와 축별 `Clip` controls를 각각 열거하며 browser CSSOM 관측과 runtime rejection을 분리한다. |
| 11 | replaced image의 transferred-size suggestion이 24×16 계산을 바꾸는가 | PASS | 이미지의 width/height는 `auto`, 반대 축 definite preferred size와 authored aspect ratio/min/max가 없다. 계획은 자연 크기 기반 contribution으로 한정한다. |
| 12 | minimum/min-content/max-content contribution의 값 또는 provenance가 섞이는가 | PASS | JSON은 Columns `24`와 Rows `16`의 세 contribution kind를 구분한다. `Minimum`의 `AutomaticMinimum` 전체 flags와 세 fixed-limit `null`을 고정하고 다른 kind의 source는 `null`이다. |
| 13 | gutter나 남는 free space가 track/frame 기대에 빠지는가 | PASS | Expected columns `[24,17]`, 5px gutter, row `[16]`, 남는 inline 54px의 trailing 정렬과 root-relative item frame `(0,0,24,16)`이 산술상 일치한다. |
| 14 | stage acceptance가 최종 used tracks만 비교하는가 | PASS | JSON은 Columns/Rows의 Initialize부터 StretchAutoTracks까지 각 stage, internal gutter index, base/growth, collapse/flexible flags와 contribution을 보존한다. Expected trace는 아직 acceptance prediction으로 명시한다. |
| 15 | `null` growth-limit projection을 내부 positive infinity의 증거로 확대하는가 | PASS | 계획·README·JSON이 Taffy의 모든 non-finite `+∞/−∞/NaN → None/null` 변환을 설명한다. `null`은 projection만, finite stage는 `Some(finite)`만 비교한다. CSS Grid의 normative `+∞`는 별도 유지한다. |
| 16 | 잘못된 입력이 image snapshot 또는 layout의 일부 결과를 게시하는가 | PASS | Negative matrix는 style/topology/source/decode/dimension/revision/profile 경계 및 error/detail을 나누고 `C11_4ImageInput`, intrinsic snapshot, frame, trace가 나오지 않는 atomicity 결과를 지정한다. 임시 resource snapshot 생성 가능성은 게시 결과와 구분한다. |
| 17 | evidence가 Chrome oracle, app output, paint proof를 혼동하는가 | PASS | README/계획은 CSSOM, layout frame/stage, draw/present, simulator screenshot, PixelCopy/Metal readback을 분리하고, 이 사전자료는 Spinon runtime·paint·WPT 통과를 뜻하지 않는다고 제한한다. |
| 18 | native scene이 등록돼도 결과/trace/frame assertion에서 빠지는가 | PASS | 계획은 Android와 iOS의 scene ID, image result, trace, root/item frame allowlist 경로를 각각 명시하고 같은 scene ID와 두 frame을 추가하라고 요구한다. 이는 검증 계획이며 아직 실행 결과는 아니다. |
| 19 | fixture가 product artifact에 새 ABI/payload를 끼워 넣거나 screenshot/browser가 남는가 | PASS | 변경은 opt-in `c11-4-grid-track-sizing-fixture-test` 안에 제한하고 default/product vs fixture positive control을 요구한다. simulator screenshots, PR image link Agent Browser 확인 후 dedicated session/Chrome 종료도 단계로 명시한다. 현재 사전관측 파일 hashes(HTML `2f5305ac…f8541`, observation `798a7259…0000d2`, metadata `f6e9cd23…98434`, screenshot `4182f752…625eb`)는 재계산값과 일치한다. |
| 20 | 구현 시작 전에 계획의 branch/status/부모 연결이 한 상태를 가리키는가 | PARTIAL | Git branch HEAD와 origin/main은 `a33b2459`이며 부모 계획에 image plan, evidence, pass02/pass03 링크가 있다. 공식 `spec/STATUS.md`의 C11.4는 미완료이고 부모 계획은 공식 `72/213`을 유지한다. 그러나 image 계획 §상태(5행)와 검증 단계 1(101행)은 아직 parent rebase/link 및 pass03 검토를 앞날 일로 적고, 13행은 `b58de5df`를 최신 plan branch HEAD라고 부르며 tracked tree clean이라고 한다. `b58de5df`는 JSON과 Chrome capture의 유효한 **캡처 당시** source commit이므로 JSON 값을 바꿀 필요는 없다. 계획 문구만 current branch `a33b2459`, pass03 완료와 현재 parent link 상태를 반영하도록 정리해야 한다. |

## 구현 시작 판정

**아직 시작하지 않는다.** 계산·입력·trace·overflow rejection 설계에 남은 기술 blocker는 찾지 못했다. 다만 계획 본문의 pass/rebase 상태를 고쳐 snapshot 기록을 일치시키고, 프로젝트 규칙상 별도 plan PR diff 30관점 검토를 마친 뒤 계획 PR을 `Rebase and merge`해야 한다. 그 후 최신 main에서 구현을 시작할 수 있다. 이 pass03 자체는 implementation/runtime 20관점 또는 PR diff 검토를 대신하지 않는다.

이번 검토는 계획·evidence/links/hash readback에 한정된다. Product code, runtime, tests, Android/iOS simulator, browser, WPT는 실행하지 않았다. C11.4 공식 완료 표시는 바꾸지 않았다.
