# C11.3.3 named-line auto implicit tracks · PR diff adversarial review 30

검토 범위는 `feat/c11-3-3-named-line-auto-implicit-tracks`의 전체 source/document/runtime diff와 최종 fixture evidence다. 구현 실패 관점 20개와 별도로 PR 통합 경계, 회귀 영향, 지원 범위가 넓어지는 경로를 나누어 확인했다. 초기 비교에서 fixed-only mode가 no-area `auto`를 허용하는 validation 회귀를 발견해 기존 area 조건을 복원하고 direct-layout/style regressions를 통과시킨 뒤 이 검토를 완료했다.

| # | 독립 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 새 enum mode가 일반 Grid profile에서 선택되는가 | projection에서 C11.3.3 implicit named-line opt-in 경로만 새 mode를 선택한다. 기존 profile enum 선택은 유지된다. |
| 2 | fixed-only mode가 no-area `grid-auto-* : auto`를 받는가 | diff에서 과도한 무조건 허용을 발견했다. `template_areas.is_some()` gate를 복원했고 direct layout negative test가 `UnsupportedGridAutoColumns`를 확인한다. |
| 3 | 새 mode가 `grid-auto-rows:auto`를 row fixture에 허용하는가 | 새 mode에만 row `Auto` validation 분기를 두고 rows fixture에서 `[20, 0, 25]`를 검증했다. |
| 4 | 새 mode가 `grid-auto-columns:auto`를 column fixture에 허용하는가 | 새 mode에만 column `Auto` validation 분기를 두고 columns fixture에서 `[40, 40, 0, 30]`을 검증했다. |
| 5 | signed/leading extent mode가 새 auto mode로 내려가는가 | `TemplateGridSignedImplicitTracks`는 mode selection에서 보존하고 signed placement 회귀를 통과한다. |
| 6 | leading implicit name의 `auto`가 trailing candidate를 가장하는가 | leading line은 signed extent로 분류되며 auto track validator에서 계속 거부된다. |
| 7 | 두 named endpoint가 모두 implicit일 때 새 mode가 열리는가 | grammar gate가 named start + `auto` end만 인정한다. `missing / missing` negative regression은 계속 거부된다. |
| 8 | named end + auto start를 허용하는가 | 반대 endpoint 순서에는 candidate flag가 설정되지 않는다. end-only source-order negative fixture도 `grid-auto-columns:auto`를 거부한다. |
| 9 | implicit named placement가 두 축을 동시에 확장하는가 | usage aggregation 뒤 정확히 하나의 implicit axis만 허용한다. 양축 implicit 조합은 candidate를 선택하지 않는다. |
| 10 | 반대 축의 자동 배치가 섞이는가 | 반대 축 start에 definite `Line`을 요구하고 implicit start를 layout normalization에서 거부한다. |
| 11 | 반대 축 end의 `auto`가 false positive definite start를 만드는가 | definite-axis 검사에서 start `Line`과 end `Line | Auto` 조합만 인정한다. |
| 12 | authored explicit named line이 실수로 implicit mode를 선택하는가 | `uses_implicit_tracks`가 false이면 mode 변경 분기에 들어가지 않는다. explicit named placement 기존 tests를 유지한다. |
| 13 | template-area item이 no-area named candidate를 선택하는가 | candidate 판정은 `template_areas.is_none()`으로 닫히며 area-backed typed auto path는 기존 mode와 테스트를 유지한다. |
| 14 | auto width 또는 height item이 fixed-size scope에 들어오는가 | projection은 두 치수가 모두 `Fixed`일 때만 허용하고 style test가 auto-size named item을 거부한다. |
| 15 | 직접 layout 호출로 auto-size sibling이 새 mode를 우회하는가 | layout validator가 새 mode의 모든 in-flow item width/height를 검사하며 direct negative test를 포함한다. |
| 16 | 적격 item 다음의 부적격 implicit item이 mode를 유지하는가 | 새 mode를 fixed-only mode로 내리고, row auto sizing 거부를 style regression으로 확인했다. |
| 17 | 부적격 implicit item 다음의 적격 item이 mode를 다시 승격하는가 | fixed-only mode는 승격 match arm에서 sticky하게 유지된다. positive end-only-before-start candidate source-order test가 거부를 확인한다. |
| 18 | `grid-auto-*` fixed-px control이 sizing 값이나 phase를 바꾸는가 | Columns 30px control은 `[40,40,30,30]`와 x=110px이며 Android/iOS 표면 표본도 exact다. |
| 19 | 비-px track list가 새 named-line mode에서 열리는가 | 새 expansion mode도 fixed-only 경로와 같은 `LengthPx` list gate를 사용한다. typed intrinsic list는 별도 `TrackSizing` mode를 요구한다. |
| 20 | implicit track count cap이 새 mode에서 우회되는가 | 새 mode가 기존 `normalize_dense_grid_container` count/extent 경로를 재사용하고 전체 Grid 143 tests가 통과한다. |
| 21 | auto placement가 새 mode에서 implicit track을 추가하는가 | explicit row/column start 필수 검사를 유지하고 placement normalization negative coverage가 통과한다. |
| 22 | hidden/정렬 처리와 dense placement가 달라지는가 | 변경은 expansion mode만 분리하며 전체 Grid regression 143/143 및 existing dense/hidden tests를 통과한다. |
| 23 | JS-to-runtime FFI scene ID가 allowlist를 벗어나는가 | Rust source map, Android MainActivity/Demo, iOS scene set은 정확히 동일한 세 ID를 allowlist한다. Node boundary test 51/51이다. |
| 24 | fixture source와 pixel JSON scene ID/node ID가 어긋나는가 | 세 FFI source와 JSON의 root/item IDs, viewport, frames, sample target을 대조했다. Chrome fixture hash와 oracle reference hash가 일치한다. |
| 25 | Android pixel sample이 root 내부의 background와 혼동되는가 | item sample은 각각 root 오른쪽/아래 밖에 있고 PixelCopy에서 expected opaque RGBA가 세 장면 모두 exact다. |
| 26 | Android bad sample/scene가 표면 readback 또는 runtime draw를 시작하는가 | 세 최종 API 37 ARM64 AVD 실행에서 bad sample은 readback 전, bad scene은 runtime draw 전 거부됐다. |
| 27 | iOS 표면 비교가 compositor 이후 화면을 증명한다고 확대 해석되는가 | evidence와 fixture README가 Metal drawable readback을 compositor 제출 전으로 분리해 표기한다. 화면 캡처는 별도 UI 자료다. |
| 28 | fixture override가 기존 capture script 기본 scene을 바꾸거나 다른 C11 runner를 깨뜨리는가 | `--fixture=`는 지정된 schema만 활성화하고 기본 fixture 경로는 유지한다. iOS 비교는 `oracle.samples ?? samples`로 기존 fixture schema와 호환되며 generated README는 후행 공백 줄 없이 끝난다. |
| 29 | C11.4 fixture boundary assertion가 formatting/추가 인자로 새 false negative를 내는가 | optional `word_spacing` argument를 whitespace-tolerant regex로 반영했고 C11.4 boundary 포함 Node tests 51/51이다. |
| 30 | 새 FFI source가 기본 Release artifact에 포함되거나 문서가 C11 완료를 암시하는가 | 최신 macOS ARM64 기본 archive는 symbol/payload 0, opt-in은 symbol 2/payload 3 marker다. STATUS의 C11 checkbox와 `72/213`은 그대로이며 APK/IPA 제외나 WPT 적합성을 주장하지 않는다. |

최종 근거: [fixed-only guard 이후 test·Clippy·archive 기록](./post-validator-guard-20261031/README.md), [최종 Android/iOS runtime matrix](./README.md), [구현 실패 관점 20개](./implementation-review-20.md). PR diff review는 코드 수정 뒤 실행했으며, GitHub CI rollup과 merge 결과는 PR 생성 후 별도 판정 대상이다.
