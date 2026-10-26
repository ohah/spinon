# C11.4 image mixed-track implementation/runtime review (20 views)

## 판정

**17 PASS / 3 PARTIAL / 0 FAIL.** 현재 local implementation candidate에서 sizing 값·양 플랫폼 trace/frame matcher·정정된 simulator run·hash 연결에 구현 blocker는 찾지 못했다. 남은 PARTIAL은 첫 실패 원인의 세부 attribution, focused test 원문 실행 출력, 앱 종료 후 process-absence 확인이다. 이 검토는 remote PR/CI를 판정하지 않는다.

대상 branch `plan/c11-4-image-auto-max-mixed-track-20261026`, `HEAD` 및 `origin/main` `040a5360be92f73e141797aa7dcd4ef00ab24942`. Working tree의 authored candidate 변경은 tracked-diff SHA-256 `447c37a07c84c63951792e5ad10b53c5378781140940aa5c4d31e1ab2712440a`로 runtime source manifest와 일치했다.

테스트, 브라우저, 시뮬레이터는 실행하지 않았다. 정적 코드·문서·JSON, 저장된 양 플랫폼 로그를 읽고, trace chunk를 read-only로 재조립해 확인했으며, 기존 hash manifest를 검산했다. 스크린샷 파일도 열어 화면이 앱 UI 캡처인 점과 이미지 paint 증거로는 쓸 수 없는 점을 확인했다.

## 20개 실패 관점

| # | 실패 관점 | 판정 | 근거와 결론 |
|---:|---|---|---|
| 1 | branch/base 또는 simulator run이 다른 source revision을 가리키는가 | PASS | `HEAD`, `origin/main`, corrected run의 `source-head.txt`가 모두 `040a5360…24942`다. Source manifest는 uncommitted candidate 파일들의 개별 hash와 tracked-diff digest를 함께 고정한다. |
| 2 | runtime 변경이 일반 CSS나 grid algorithm 변경으로 새어 나가는가 | PASS | 변경은 C11.4 image fixture adapter의 case/typed track match, C0410 FFI scene registry, Android/iOS 전용 matcher와 테스트 assertion에 한정된다. Grid sizing algorithm 또는 public projection API 변경은 없다. |
| 3 | image case allowlist가 다른 root/item/geometry를 받아들이는가 | PASS | `IMAGE_CASES`의 새 항목은 root/item ID, 100×40 grid, 두 column, auto row, 5px gap, auto placement를 고정한다. 일반적인 임의 image subtree가 아니라 ID와 topology gate를 통과한 direct child 한 개만 받는다. |
| 4 | 새 track 허용이 의도보다 넓은 track grammar를 열어 두는가 | PASS | `track_matches`는 typed `MinMax { min: Auto, max: MaxContent }`와 fixed `17px`에만 새 enum case를 대응한다. 유사 문자열이나 임의 min/max 조합을 허용하지 않는다. |
| 5 | computed root/item style, placement, sizing 값이 fixture 계약 밖이어도 통과하는가 | PASS | Root의 display/box-sizing/alignment, 크기·spacing·gap·track 목록, 자동 흐름 등을 검사한다. Image의 auto width/height, min/max, zero spacing, static position 및 auto placement도 검사한다. 불일치 시 layout 전달 전에 거부한다. |
| 6 | fixture가 다른 image bytes/network loader를 사용하거나 revision provenance를 잃는가 | PASS | 기존 고정 `c14-1-png-basic.png` 바이트와 `ImageRequest::from_document`를 재사용하고, decoded natural size `24×16`을 확인한 뒤 document-bound `ImageResourceSnapshot` 및 intrinsic snapshot을 만든다. 새 loader/API 변경은 없다. |
| 7 | FFI scene이 sizing 산술을 바꾸는 추가 CSS나 overflow 선언을 포함하는가 | PASS | `C11_4_IMAGE_TRACK_SCENES`의 scene은 `minmax(auto,max-content) 17px`, auto row, 5px column gap, 100×40 root, auto-size image다. 생성된 image style에는 authored `overflow`가 없다. |
| 8 | scene registry/count assertion이 다른 scene 또는 무제한 case로 잘못 연결되는가 | PASS | 새 scene ID/root/item ID가 adapter와 FFI에서 일치한다. image scene 개수 assertion은 18→19로 조정됐고 생성 source 검사도 ID, geometry, track, gap, auto sizing, overflow 문자열 부재를 확인하도록 추가됐다. |
| 9 | Android scene allowlist가 크기만 검사하고 실제 root/item 위치를 놓치는가 | PASS | C0410 scene allowlist, root/item frame 기대값, full node-frame 기대값이 scene ID에 결합돼 있다. 기대 root `(0,0,100,40)`, item `(0,0,24,16)`이며, `node_frames_css_px` 전체를 검사한다. |
| 10 | Android intrinsic input matcher가 다른 item/revision/natural size를 받아들이는가 | PASS | 새 case는 intrinsic revision 1, grid 1/node 2/source order 0, replaced source, natural size 24×16을 검사하고 trace matcher로 이어진다. |
| 11 | Android trace matcher가 stage 누락·raw slot 축약·gutter flag 오류를 허용하는가 | PASS | Trace 1개, grid 1, Columns/Rows 두 pass 순서, 각각 6개 stage, raw slot index, gutter/collapsed/infinitely-growable flags, contribution 3종을 대조한다. Columns는 5 slots, Rows는 3 slots이다. |
| 12 | Android `Initialize` null과 finite batch expectation을 혼동하는가 | PASS | `Initialize`에서는 각 축의 실제 intrinsic track growth projection 하나만 JSON null을 허용한다. `IntrinsicItemBatch`부터는 column 24px, row 16px 및 다른 모든 slot을 finite number로 비교한다. 이는 `+∞`의 trace 증명이라고 주장하지 않는다. |
| 13 | iOS scene/frame allowlist가 Android와 다른 위치·크기를 기대하는가 | PASS | iOS `C0410RuntimeGpuDemo.swift`도 동일 ID, root/item dimension 및 전체 node-frame 문자열을 검사한다. Android와 같은 root/item CSS 좌표를 고정한다. |
| 14 | iOS intrinsic input 및 stage matcher가 Android보다 느슨하거나 다른 배열을 기대하는가 | PASS | iOS는 intrinsic revision, grid/node/source-order, replaced source와 24×16 natural size를 검사한다. trace의 axes, stage order, contribution/source string, 모든 raw slot 및 null/finite 조건은 Android matcher와 같은 계약이다. |
| 15 | 규범 산술·Taffy raw vector·JSON·native matchers·README가 서로 어긋나는가 | PASS | 독립 derivation과 pinned Taffy source는 columns `[0/0,0/null,5/5,17/17,0/0]`, rows `[0/0,0/null,0/0]`로 시작하고 batch 후 24/16을 기대한다. 실제 index 2 gutter는 `5/5`, uncollapsed다. Child plan, expectation JSON, Android/iOS arrays, runtime README 및 보존된 로그가 일치한다. |
| 16 | 첫 failed run을 pass로 승격하거나 실패 원인을 로그가 증명하지 않는데 단정하는가 | PARTIAL | Android/iOS run `…T000220Z…`는 별도 보존됐고 두 로그 모두 `SPINON_C114_GRID_ASSERTION_ERROR`가 있어 pass 근거로 쓰이지 않는다. 저장된 raw diagnostics를 읽으면 관측 track arrays 자체는 현재 corrected 기대값과 같다. Android failure marker는 `image_intrinsic_inputs_match=false`라는 합성 결과만 내고, pre-correction matcher source/build hash도 없다. 따라서 README의 구형 slot/batch 기대 불일치 설명은 그럴듯하지만 원문만으로 독립 재현되지 않는다. 실패 상태는 명확하다. |
| 17 | corrected Android 성공이 다른 run/device 또는 frame만의 성공인가 | PASS | Corrected run `…T002422Z…stagefix01`의 raw AVD 로그는 API 37 ARM64 scene ID, `status=0 layout=ready boxes=2`, root/item frame, 전체 trace chunk와 `status=0 presented boxes=2`를 기록한다. `image_draw_count=0`도 함께 기록한다. |
| 18 | corrected iOS 성공이 Android 로그를 재사용하거나 trace assertion 없이 종료되는가 | PASS | 같은 run ID의 iOS 로그는 iPhone 17 Pro / iOS 26.2 simulator metadata와 `status=0 layout=ready boxes=2`, 예상 frame, 전체 trace chunk, `status=0 presented boxes=2`를 기록한다. `image_draw_count=0`이다. |
| 19 | screenshot/build/source hashes가 서로 다른 candidate payload를 가리키는가 | PARTIAL | Corrected SHA manifest의 `shasum -c`가 모든 build/runtime/screenshot/source-hash 항목에서 통과했고, `source-hashes.json`의 code/fixture/plan/JSON/diff hash도 현재 파일과 일치한다. Standalone Android/iOS screenshots는 각각 1080×2400, 1206×2622이고 run-directory screenshots와 byte-identical이다. README에는 focused tests 4/4와 FFI test 1/1의 명령/결과가 적혀 있지만 터미널 원문 결과 파일은 없어 독립 재검산은 할 수 없었고, 이 리뷰에서는 테스트를 재실행하지 않았다. |
| 20 | 앱 종료를 검증했다고 쓰거나 simulator screenshot을 PNG paint/hardware/overflow 증거로 확대하는가 | PARTIAL | Runner는 Android `am force-stop` 및 iOS `simctl terminate`를 호출한다. 하지만 iOS 종료 실패는 `|| true`로 무시되고, 두 종료 로그에는 출력이 없으며 후속 process-absence 확인이 없다. 따라서 README의 “verified absent”는 이 기록만으로 증명되지 않는다. 반면 `runtime_image_count=0,image_draw_count=0`와 양 플랫폼 screenshot이 함께 있어 PNG paint/readback 주장은 하지 않으며, plan/evidence도 overflow, 실기기, hardware GPU, compositor 색상을 명시적으로 제외한다. |

## 남은 범위

- 제품 sizing/trace/frame 후보의 blocker는 찾지 못했다. 첫 실패의 정확한 matcher 세부 원인, test 원문 출력, 앱 종료 후 프로세스 부재를 더 강하게 주장하려면 각각 이전 matcher/build provenance, test stdout artifact, 명시적인 종료 후 process query 로그를 추가해야 한다.
- 검토 당시 코드/plan/evidence 변경은 local uncommitted candidate다. 이 문서는 remote PR diff·CI 통과·merge 판정을 대신하지 않는다.
- 시뮬레이터 frame/trace와 `presented`는 layout/submission 증거다. 스크린샷은 대표 simulator UI 캡처이며 `image_draw_count=0`이므로 PNG raster paint/readback 증거가 아니다. Overflow extraction/projection/clip/scroll 동작, 실기기 및 hardware GPU도 이 slice의 검증 범위가 아니다.
- C11.4 상위 체크는 미완료이고 현재 공식 집계 `72/213`은 변하지 않는다.
