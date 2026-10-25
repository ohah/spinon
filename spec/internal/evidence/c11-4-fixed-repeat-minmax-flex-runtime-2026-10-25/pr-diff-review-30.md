# C11.4 fixed-repeat `minmax(px, fr)` + auto-repeat PR diff 검토 30개

검토 대상은 `origin/main...HEAD` 전체 변경분이며 HEAD는 `240f051c87bedce3f6d3eb60b03c900c63f26f71` (PR #615)이다. 아래는 구현/runtime review 20개와 별도로 변경 diff·주변 계약·테스트·fixture·증거 연결을 검토한 결과다. 이번 검토에서는 source 수정, build/test, simulator 및 browser 실행을 하지 않았다. PR 본문 screenshot raw URL의 Agent Browser 로드 확인은 별도 절차에 남긴다.

## 결과

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | 후보 범위가 opt-in 밖으로 새는가 | 통과. C11.4 fixture module은 `c04-runtime-gpu-fixture-test` cfg 아래 있고 기본 제품 CSS admission을 바꾸지 않는다 (`crates/spinon-ffi/src/runtime_gpu.rs:29`, `:44`; `spec/internal/0059-c11-grid-layout.md:476`). |
| 2 | fixed-repeat pattern의 허용 track 수가 넓어졌는가 | 통과. 정수 repeat pattern은 비어 있지 않은 1–2 track으로 계속 제한된다 (`crates/spinon-style/src/stylo_dom/cascade/incremental/grid/auto_repeat.rs:112`). |
| 3 | 두 개 이상의 auto-repeat 또는 두 축 혼합을 허용하는가 | 통과. 각 list에서 하나만 허용하는 기존 검사와 `TrackSizing` profile gate를 유지한다 (`auto_repeat.rs:42`; `grid_values.rs:44`). |
| 4 | CSS 문법 오류와 computed-profile 거부를 혼동하는가 | 통과. parser에서 거부되는 `minmax(fr, px)`·음수 breadth 경로를 별도 진단으로 둔다 (`fixed_repeat_auto_repeat.rs:635`; `plan/c11-4-fixed-repeat-minmax-flex.md:27`). |
| 5 | authored 표기가 아니라 computed typed shape가 결정하는가 | 통과. `calc(20px)`는 computed `LengthPx(20)`일 때만 받아들이는 cascade 회귀가 있다 (`fixed_repeat_auto_repeat.rs:502`; 계획 `:21`). |
| 6 | provenance가 MinMax 값만 보고 잘못 만들어지는가 | 통과. metadata count는 fixed-repeat `Number` group만 순회하며 MinMax의 유효 shape를 확인한다 (`auto_repeat.rs:324`; snapshot `crates/spinon-style/src/stylo_dom/cascade/snapshot.rs:237`). |
| 7 | provenance가 authored 순서의 expanded track index를 가리키는가 | 통과. 확장 중 현재 `tracks.len()`을 기록하고, prefix/auto-repeat/suffix를 양축에서 고정한다 (`auto_repeat.rs:234`; `fixed_repeat_auto_repeat.rs:522`). |
| 8 | 일반 authored neighbor MinMax가 provenance 없이 통과하는가 | 통과. 양축 projection negative test가 같은 `MinMax(px,fr)` shape를 metadata 없이 거부한다 (`auto_repeat_neighbor_tests.rs:153`; guard `grid_values.rs:286`). |
| 9 | provenance의 중복·역순·범위 오류가 잘못된 track을 허용하는가 | 코드상 fail-closed. 단조 증가, 중복 없음, repeat range 바깥, 실제 shape 일치를 모두 검사한다 (`grid_values.rs:254`). 이 malformed metadata 조합 각각의 전용 test는 없다. |
| 10 | 일반 neighbor 거부가 한 축에만 적용되는가 | 통과. columns와 rows 모두 provenance가 있는 positive 및 없는 negative projection을 가진다 (`auto_repeat_neighbor_tests.rs:111`, `:153`). |
| 11 | fixed-repeat prefix와 suffix의 위치가 보존되는가 | 통과. 두 축에서 같은 CSS 입력으로 prefix 2 tracks와 suffix 1 track index `[0,1,3]`를 확인한다 (`fixed_repeat_auto_repeat.rs:522`). |
| 12 | auto-repeat 자체의 기존 fixed-minimum 정책이 바뀌는가 | 통과. auto-repeat pattern은 기존 `auto_repeat_track_has_fixed_minimum` 조건을 거치며 새 provenance는 fixed-repeat만 채운다 (`auto_repeat.rs:221`; `:243`). |
| 13 | auto-repeat 반복 수에서 MinMax fixed minimum을 쓰는가 | 통과. layout neighbor validator는 지원 MinMax의 `LengthPx` minimum을 repeat count basis로 사용한다 (`crates/spinon-layout/src/grid.rs:1402`; 양축 test `flex_factor_bound.rs:260`). |
| 14 | projection이 fraction maximum을 손실하거나 다른 breadth로 바꾸는가 | 통과. typed projection test가 `MinMax(LengthPx, Fraction)`을 layout 입력으로 그대로 확인한다 (`grid_values/tests.rs:15`). |
| 15 | 0fr 및 1 미만 fraction의 CSS partial-fill 의미를 잃는가 | 통과. cascade shape와 layout stage에서 0 및 0.5 factor를 고정한다 (`fixed_repeat_auto_repeat.rs:467`; `flex_factor_bound.rs:210`). |
| 16 | columns/rows, auto-fill/auto-fit sizing 기대값이 편향되는가 | 통과. six-scene oracle은 양축 fill/fit과 collapsed tracks를 나누고, item 시작 및 Chrome CSSOM rect를 보관한다 (`spec-derived-expectations.json:14`; `chrome156-observation.json:1`). |
| 17 | 여러 fixed-repeat group의 순서·suffix flex sizing이 빠지는가 | 통과. 서로 다른 pattern의 여러 group과 suffix factor를 포함하는 `columns-groups` 기대값이 독립 계산과 Chrome 관측에 연결된다 (`spec-derived-expectations.json:69`; `fixed_repeat_minmax_flex_scenes.json:560`). |
| 18 | MinMax가 line-name seam 순서를 깨뜨리는가 | 통과. MinMax와 px track이 섞인 fixed group의 physical seam 및 반복 경계 이름을 정확히 검사한다 (`fixed_repeat_line_names.rs:92`). |
| 19 | 1 MiB metadata 경계가 새 MinMax pattern에서도 적용되는가 | 통과. MinMax pattern으로 1,048,576 bytes 성공 및 1 byte 초과 거부를 검증한다 (`fixed_repeat_line_names.rs:121`). provenance Vec는 별도 bounded track 개수만큼 fallible reserve된다 (`auto_repeat.rs:192`). |
| 20 | provenance allocation 실패가 부분 결과를 남기거나 회복하지 못하는가 | 통과. MinMax 포함 fixed pattern에서 새 failpoint를 주입하고 오류 뒤 같은 worker 호출을 다시 성공시킨다 (`fixed_repeat_auto_repeat.rs:1201`; `fixed_repeat_line_names.rs:178`). |
| 21 | expanded descriptor 한도를 넘어 provenance/track이 무한 확장되는가 | 통과. component와 projected track count는 checked arithmetic 및 10,000 ceiling으로 선행 제한되고, provenance 수 역시 그 ceiling 안이다 (`auto_repeat.rs:133`, `:324`; resource controls `spec-derived-expectations.json:140`). |
| 22 | 유한한 개별 factor 합이 Taffy의 f32 accumulator를 무한대로 만드는가 | 통과. cascade는 최대 repeated pattern 포함 상한을 Taffy 전에 검사한다 (`auto_repeat.rs:361`). `1e38fr` fixed 및 repeated controls를 추가했다 (`fixed_repeat_auto_repeat.rs:539`). |
| 23 | direct layout 입력의 다른 축이나 auto-repeat ceiling에서 overflow가 빠지는가 | 통과. layout preflight가 두 축 모두에서 repeat 없는 합과 10k auto-repeat 상한을 확인한다 (`grid.rs:485`, `:856`; `flex_factor_bound.rs:229`, `:244`). |
| 24 | 거부 오류가 element/property context를 잃는가 | 통과. cascade overflow test는 root node와 `grid-template-columns`를 확인하고, layout test도 축별 property를 검사한다 (`fixed_repeat_auto_repeat.rs:554`; `flex_factor_bound.rs:230`). |
| 25 | 유효하지만 지원 밖인 shape가 우연히 통과하는가 | 통과. percentage minimum, intrinsic maximum, fixed max 및 3-track pattern이 profile에서 거부된다 (`fixed_repeat_auto_repeat.rs:568`; projection malformed shape `auto_repeat_neighbor_tests.rs:181`). |
| 26 | fixture data가 실행 경로에 연결되지 않거나 미등록 ID가 route 되는가 | 통과. C11.4 router에 새 module을 연결하고, fixture test가 six IDs와 unlisted rejection을 확인한다 (`c11_4.rs:50`, `:1208`; `fixed_repeat_minmax_flex.rs:56`, `:270`). |
| 27 | fixture expectation이 독립 oracle·사전 입력과 엇갈리는가 | 대체로 통과. FFI test가 oracle scene ID·축·track list·CSS 입력·item order를 맞추고, 전체 rect를 산출해 fixture 및 Chrome 관측과 비교한다 (`fixed_repeat_minmax_flex.rs:56`; `:94`). |
| 28 | runner가 예상 frame을 확인하지 않거나 adapter가 frame 검사를 우회하는가 | runner는 통과. capture script는 fixture expectation에서 root/item node frames를 만들고 runtime log에서 전 좌표를 비교한다 (`capture-c11-4-auto-repeat-neighbors-simulator.mjs:4694`; `:5951`). Android/iOS adapter 자체의 generic node-frame matcher는 이 six IDs에 대해 항상 통과시키고 단일 item frame도 생략하므로 runner 없이 앱 로그만 읽으면 geometry를 검증한 것으로 볼 수 없다 (`C0410RuntimeGpuDemo.java:1209`, `:1552`; Swift `:813`, `:1346`). README는 외부 runner의 전체 비교와 adapter 범위를 구분해 적는다 (`README.md:24`). |
| 29 | 신규 fixtures가 일반 build에 유입되거나 scene catalog 선택이 깨지는가 | 통과. feature gate를 지키고 six scenes를 총 484→490 catalog 변경 및 기존 fixed-repeat selector 제외에 반영했다 (`runtime_gpu.rs:29`; `capture-c11-4-auto-repeat-neighbors-simulator.mjs:4530`, `:4847`; boundary test에서 count 수정). product/positive-control archive 결과는 README `:46`에서 구분한다. |
| 30 | 공개 roadmap 화면이 주장한 final-source 실행을 가리키는가 | **해결 확인.** 이전 href는 `spec/public/roadmap.html:177`에서 Android `T234338872Z-53f6e951` 및 iOS `T234343852Z-a54267ba`의 tracked-diff SHA `5b359b…` 캡처를 가리켰다. 사후 확인에서 해당 href가 README `:19-20`의 Android `T235142067Z-4887fd16` 및 iOS `T235205801Z-de685760` PNG로 바뀌었고, 두 대상 파일이 존재함을 확인했다. [Android `simulator.json`](android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T235142067Z-4887fd16/simulator.json)과 [iOS `simulator.json`](ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/simulator.json)은 각각 tracked-diff SHA `9a79a11be6cc3cd88f6fcb5d49db10eebf66c4fe4407563edbb88cba5aa0a391`, 26개 input-file manifest를 기록하며 runtime README `:15`, `:19-20`의 선언과 맞는다. 이번 재확인은 URL 경로·파일 존재·metadata 대조이며 browser 실행은 하지 않았다. |

## Findings 및 상태

### P2 — 해결 확인

이전 review의 P2는 공개 roadmap screenshot href가 기록된 final-source 실행과 달랐다는 증거 provenance 이슈였다. `spec/public/roadmap.html:177`을 runtime README `:19-20`의 final-source Android/iOS run으로 수정한 뒤 href·PNG 파일 존재·metadata를 재대조했다. 두 run의 `trackedDiffSha256`은 모두 `9a79a11be6cc3cd88f6fcb5d49db10eebf66c4fe4407563edbb88cba5aa0a391`이고 각 metadata에 기록된 26개 input-file manifest 수가 README `:15`의 선언과 맞는다. 이전 P2는 해결됐다.

문서 상태도 동기화됐다. [계획 `:3`](../../../../plan/c11-4-fixed-repeat-minmax-flex.md)은 PR diff review 완료와 PR #615 open 상태를 기록하고, [runtime README `:52`](./README.md)는 별도 PR diff review 완료를 기록한다. C11.4 checkbox·공식 `72/213`·내부 contract version `0.1.0`은 유지되며 제한 candidate를 전체 완료로 표시하지 않는다 (`spec/internal/0059-c11-grid-layout.md:476`; `spec/STATUS.md:364`; roadmap `:177`).

현재 미해결 blocking finding은 없다. 이번 사후 검토는 관점 #30 및 문서 상태 동기화만 재확인했으며 나머지 29개 관점을 다시 실행하지 않았다. PR 본문 raw screenshot URL은 이 재확인 범위에 포함하지 않았고, 앞선 별도 Agent Browser 확인을 유지한다.
