# C11.4 cyclic `minmax(calc())` minimum PR diff 검토 30개

구현/runtime 실패 관점 검토와 구분해 최종 staged diff의 dispatcher 연결, 입력·기대값 중복 관리, platform sample 계약, artifact 경계와 문서 claim을 각각 대조했다. review 대상은 기본 commit `790ecafa8ef3df76905a092e7af2257578927211` 대비 staged diff와 [최신 Android/iOS capture](./README.md)다.

| # | 별도 실패 관점 | diff 대조 및 결과 |
| ---: | --- | --- |
| 1 | 새 fixture module이 `fixtures/` opt-in C11.4 경로에 빠지는가 | `fixtures/c11_4.rs`에 module path와 scene dispatcher가 추가되고 C04 runtime fixture feature로만 빌드된다. |
| 2 | FFI source table과 runtime scene matrix가 서로 다른 ID를 받는가 | 두 표 모두 cyclic column/row와 definite column/row 네 exact ID를 갖고 미등록 ID를 거부한다. |
| 3 | 첫 Android 실패가 보존되지 않거나 수정되지 않은 것으로 보이는가 | 보존된 실패 log는 runtime registry 누락을 보여 준다. 수정 matrix test와 최종 Android 4/4 capture가 이를 닫는다. |
| 4 | 신규 root에 다른 tag, namespace 또는 document owner가 들어오는가 | runtime adapter는 HTML `div`, root owner, `data-node` 및 exact root ID를 검사한다. |
| 5 | item 순서나 child 수가 바뀌어 node id와 frame 기대가 어긋나는가 | FFI는 root/item 두 element와 하나의 direct text node만 만들고 runtime adapter가 직접 관계를 확인한다. |
| 6 | 두 축 projection이 동일한 calc owner/ID로 합쳐지는가 | projection test가 columns/rows 순서의 개별 `LayoutCalcId`와 해당 `GridTemplate*` property를 검사한다. |
| 7 | `minmax()` calc가 min이 아닌 max breadth로 놓이는가 | projection regression은 `min: Calc`, `max: MaxContent`를 각각 직접 기대한다. |
| 8 | CSS calc 항 순서나 수치가 fixture·runtime·계획 간 다르게 고정되는가 | style test, runtime validator, FFI source, plan matrix가 200%+4px/2px와 10px fixed axis를 서로 대조한다. |
| 9 | cyclic column/row sizing이 서로 바뀌거나 한 축만 기록되는가 | 두 platform adapters와 capture script 모두 column/row 각각의 Columns·Rows track을 기대하고 최신 로그가 일치한다. |
| 10 | definite controls가 cyclic intrinsic root처럼 처리되는가 | controls는 두 root dimension 10px을 고정하며 cyclic variants와 네 scene을 별도 ID로 비교한다. |
| 11 | min calc가 계산되더라도 Ahem intrinsic contribution이 달라 기대값이 우연히 맞는가 | 저장소 Ahem TTF와 system-font-free text engine을 쓰며 exact direct text `a`, 10px font/line-height를 검증한다. |
| 12 | content-box basis 대신 border-box percentage를 사용해도 frame만 우연히 맞는가 | root `box-sizing`, computed border와 width/height가 검사되며 independent expected value는 10px content basis를 쓴다. |
| 13 | root border 2px가 빠지거나 item origin이 decoration 안쪽에서 벗어나는가 | 네 scene의 root `14×14px`, item origin `(2,2)`와 크기를 Rust trace 및 두 platform frame assertion이 확인한다. |
| 14 | actual CSS border 대신 test-only overlay가 yellow control을 만든다 | FFI scene source에 `border:2px solid #f0c674`가 있으며 surface check는 그 렌더 결과를 읽는다. 별도 overlay는 없다. |
| 15 | item overflow가 right/bottom root border를 덮는지 검증하지 않는가 | column `(13,5)`, row `(5,13)` red sample이 border 위치의 item overpaint를 확인한다. |
| 16 | root 밖의 item overflow와 item 끝 바깥을 구분하지 못하는가 | column x=16/27, row y=16/25에서 각각 red/black을 검사한다. |
| 17 | Android matcher가 sample 수만 맞추고 중복 ID 또는 wrong node를 받는가 | Android validator는 expected ID set, target node와 CSS coordinate를 확인하고 duplicate/missing sample을 거부한다. |
| 18 | Android RGB 값만 비교해 alpha 또는 surface channel이 빠지는가 | expected sample은 opaque RGBA이며 PixelCopy log의 다섯 sample per scene이 exact로 matched다. |
| 19 | iOS matcher가 Android 기대와 다른 축 좌표를 쓰는가 | Metal pixel mapping은 같은 scene ID/node/CSS coordinates/five RGBA values로 20/20 matched다. |
| 20 | iOS Metal 결과를 compositor 제출 후 화면 관측으로 잘못 부르는가 | README와 capture log는 drawable sample 지점을 제출 전으로 제한하고 compositor 이후는 검증 밖으로 적었다. |
| 21 | capture helper의 scene class에 새 ID가 빠져 계산/픽셀 분기가 달라지는가 | 네 ID를 cyclic-percent group에 넣었고 실제 Android/iOS capture가 각 4/4 통과했다. |
| 22 | 정적 scene 개수 상수가 갱신되지 않거나 ID 중복이 숨겨지는가 | scene count를 227에서 231로 갱신하고 고유 scene ID assertion을 보존했다. |
| 23 | native platform adapter에 root/item frame 또는 box count가 누락되는가 | Android와 Swift adapter가 모두 14×14 root, 24×10/10×22 item, 두 node를 네 ID에 지정한다. |
| 24 | 새 expected color branch가 기존 C11.4 colors를 바꾸는가 | 새 IDs에만 yellow/red/black sample 색을 추가하고 기존 cases의 분기는 유지한다. |
| 25 | style test 파일이 fixture-test 밖의 production code로 편입되는가 | C11.4 track sizing test module은 `c11-4-grid-track-sizing-fixture-test` 조건으로 연결된다. |
| 26 | scene table 크기나 single-file 책임이 과도하게 커지는가 | 12 scene data를 cohesive `scene.rs` matrix로 분리하고 adapter file을 398줄로 줄였다. |
| 27 | default archive가 fixture ID/entry marker를 포함하는데 증거가 이를 누락하는가 | no-feature release archive는 새 ID 및 fixture host marker가 없고 opt-in positive archive에 둘 다 있다. 해시와 검사 범위를 README에 적었다. |
| 28 | 공식 roadmap이 계획 상태·완료 수를 잘못 표시하는가 | plan과 `STATUS.md`는 fixed runtime 결과를 반영하지만 C11.4 checkbox와 `72/213`을 유지하며 미검증 범위를 적는다. |
| 29 | PR/문서의 simulator 이미지 링크가 stale run이나 존재하지 않는 파일을 가리키는가 | README와 PR에 쓸 이미지들은 staged된 최신 source-hash run의 PNG를 가리킨다. README에는 Android/iOS 대표 화면이 inline이다. |
| 30 | simulator-only evidence가 physical device, hardware GPU, WPT 또는 전체 CSS support로 확대되는가 | README, plan과 roadmap 모두 AVD/Simulator 범위, iOS pre-compositor 지점, 남은 한계를 명시한다. |

**결과:** 이 30개 diff 관점에서 남은 결함은 찾지 못했다. 최신 source hash 기준 캡처, Rust tests, targeted Clippy, 두 platform build와 artifact positive/negative control을 대조했다.
