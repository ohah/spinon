# C11.3.3 `grid-template-areas` PR 적대적 검토 · 2026-10-14

## 범위와 판정

`origin/main`의 `ce062213`부터 현재 `feat/c11-3-3-template-areas` 작업 트리까지 구현, 테스트, 모바일 fixture 경로, 증거와 상태 문서를 검토했다. 아래 30개 항목은 실패 경로와 계약 경계를 각각 다르게 잡았다. `PASS`는 구현 또는 지정된 검증으로 반례를 확인하지 못했다는 뜻이고, `PARTIAL`은 코드 경로는 있으나 독립된 앱/회귀 증거가 부족하다는 뜻이다.

차단 결함은 발견하지 못했다. `PARTIAL` 항목은 지원 범위를 넓혀 말하지 않도록 남은 증거 한계로 기록했다. WPT 적합성, 실기기/hardware GPU, compositor 제출 후 픽셀은 이 PR의 검증 범위가 아니다.

## 30개 적대적 관점

| # | 실패 시나리오 / 경계 | 판정 및 확인 근거 |
|---:|---|---|
| 1 | 넓은 영역 행렬이 빈 셀까지 저장해 입력 크기에 비례하는 메모리를 요구한다 | **PASS.** computed snapshot은 행·열 크기와 named rectangle만 소유한다. 희소 영역 복사 테스트가 `hero`, `nav`, `main`만 저장하는지 확인한다. <a href="../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid_areas.rs">`grid_areas.rs`</a>, <a href="../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_areas.rs">`template_areas.rs`</a> |
| 2 | 모든 셀이 `.`인 템플릿이 `none`으로 축약되어 explicit row/column 크기를 잃는다 | **PASS.** 빈 rectangle 목록과 비어 있지 않은 2×3 dimensions를 함께 유지하는 cascade 테스트가 있다. 같은 셀 행렬을 snapshot에 복제하지 않는다. <a href="../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_areas.rs">`template_areas.rs`</a> |
| 3 | Stylo 셀 구간이 1-based inclusive라고 가정하거나 끝 인덱스를 한 칸 더 포함한다 | **PASS.** 변환은 시작·끝에서 각각 1을 빼고 끝을 exclusive로 저장한다. `hero` 두 열 영역의 0-based `<a href="../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid_areas.rs">0,2)` 경계가 Rust cascade 테스트로 고정됐다. [`grid_areas.rs`</a> |
| 4 | 0, 차원 초과, 빈 범위 같은 손상된 computed 구간이 unchecked index나 잘못된 rectangle로 통과한다 | **PASS.** row/column의 start 0, end 초과, reversed, empty 등 8 typed range를 직접 주입했다. 모두 node `733`과 `grid-template-areas`를 보존한 `UnsupportedComputedGridValue`로 끝났다. [후속 테스트](./c11-3-3-post-merge-followup-2026-10-14/README.md) |
| 5 | 영역 목록 `try_reserve_exact` 실패가 panic 또는 무문맥 오류가 된다 | **PASS.** test failpoint는 node와 `grid-template-areas`를 보존한 `GridAllocationFailure`를 확인하고 다음 cascade의 복구도 확인한다. <a href="../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_areas.rs">`template_areas.rs`</a> |
| 6 | area 이름 문자열 복사 예약 실패가 부분 snapshot을 성공으로 반환한다 | **PASS.** 이름 텍스트 예약을 별도 실패 지점으로 주입한다. 오류 뒤 동일 fixture 재계산도 성공한다. <a href="../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid_areas.rs">`grid_areas.rs`</a> |
| 7 | `grid-template-areas:none`가 빈 영역 객체로 바뀌어 기존 배치 모드를 바꾼다 | **PASS, code path.** computed `Areas`가 아닌 값은 `None`을 반환하며, layout projection도 `None`만으로 `TemplateGrid`를 선택하지 않는다. |
| 8 | 잘못된 뒤쪽 author 선언이 앞선 유효한 area 템플릿을 지우고 layout을 실패시킨다 | **PASS.** 뒤 선언이 non-rectangular일 때 cascade가 앞선 `'kept .'` 선언으로 되돌아오는 회귀가 있다. <a href="../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_areas.rs">`template_areas.rs`</a> |
| 9 | area의 행·열 수와 explicit track 수가 다를 때 일부 item frame이 먼저 생성된다 | **PASS.** projection이 dimensions를 layout 전에 검사한다. API 37 AVD와 iOS Simulator에서 mismatch scene이 `grid-template-areas`의 기대 거부로 frame 없이 끝났다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-matrix-20261014T023829118Z-d027504f/result.json">`결과 JSON`</a> |
| 10 | line-name group 개수가 track 수 N+1이 아닌 경우 alias 검색이 index를 넘는다 | **PASS.** template projection에서 양 축 shape를 검사하고, 잘못된 group 수를 node/property 포함 오류로 거부한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/tests.rs">`grid_values/tests.rs`</a> |
| 11 | explicit line alias만 한 축에 존재해 implicit named area라고 오인한다 | **PASS.** `box-start`와 `box-end`가 두 축 모두에 있을 때만 template sizing mode를 고른다. 한 축만 남긴 negative control은 dense mode를 유지한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/tests.rs">`grid_values/tests.rs`</a> |
| 12 | 생성 area alias가 행/열 축을 바꾸거나 잘못된 경계 번호를 돌려준다 | **PASS.** resolver는 axis와 start/end를 명시해 읽는다. `row-column-longhands-area` AVD scene이 같은 `body` area ident를 row와 column 양쪽에서 선택하고 Chrome geometry와 맞는다. 생성 alias의 column start/end와 named span은 별도 Rust 테스트로 확인한다. |
| 13 | bare area ident가 일반 named line occurrence 1로 해석되거나 `1 ident`가 area edge로 바뀐다 | **PASS.** bare `hero`는 `hero-start`, 명시 occurrence 1은 authored `hero` line을 선택한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line/tests.rs">`named_line/tests.rs`</a> |
| 14 | area가 아닌 bare ident가 일반 이름 줄의 첫 occurrence로 fallback하지 않는다 | **PASS.** area metadata와 양 축 area alias pair가 없는 fixture에서 pinned Chrome 154와 Rust style-to-layout 모두 item frame `(40,30,20,20)`을 반환한다. <a href="./c11-3-3-post-merge-followup-2026-10-14/chrome-154-bare-ident-no-area.json">Chrome 관찰</a> |
| 15 | authored `box-start`/`box-end`가 한 축에만 있어 area로 잘못 만들어진다 | **PASS.** bare ident area 경로는 두 축의 start/end pair 네 개가 모두 있어야 하고, 반대 축 pair가 빠진 경우 fail closed한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line/tests.rs">`named_line/tests.rs`</a> |
| 16 | 같은 line group 안의 중복 이름이 positive occurrence 또는 span count를 두 번 증가시킨다 | **PASS.** 기존 resolver는 이름 문자열 수가 아니라 line group 단위로 occurrence를 센다. 같은 line 중복과 case sensitivity 회귀가 유지된다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line/tests.rs">`named_line/tests.rs`</a> |
| 17 | 0 또는 극단 음수 occurrence가 산술 overflow나 implicit line 생성을 유발한다 | **PASS.** 0, `i32::MIN` 및 occurrence 부족은 overflow 없이 fail closed한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line/tests.rs">`named_line/tests.rs`</a> |
| 18 | named span이 반대쪽 definite endpoint를 무시하거나 검색 방향을 뒤집는다 | **PASS.** forward/backward 검색과 area generated alias span을 확인한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line/tests.rs">`named_line/tests.rs`</a> |
| 19 | 양 끝 named span이 충돌한 뒤 버려야 할 이름을 resolve해 오류를 낸다 | **PASS.** 충돌 끝점을 제거하고 남은 span/auto만 layout에 보낸 기존 회귀가 통과한다. |
| 20 | `grid-area` 한 값부터 네 값까지의 shorthand 확장이 C11.3.3 snapshot에 도달하지 않는다 | **PASS.** `grid-area` 1·2·3·4 값 scene, `grid-template` shorthand, `grid` shorthand가 Android AVD matrix에서 각각 success와 0 CSS px geometry로 끝난다. |
| 21 | authored/generated alias 충돌 또는 `-start`/`-end`로 끝나는 area 이름에서 검색 우선순위가 달라진다 | **PASS, 고정 scene 한정.** explicit line 전·후 배치와 `a-start`/`a-end` suffix area 이름 scene이 Android matrix에서 success, 0 CSS px로 끝났다. 이 결과는 행렬에 없는 모든 이름 충돌 조합의 보편 지원 주장이 아니다. |
| 22 | `TemplateGrid` 선택이 기존 template profile 전체에 적용되어 이전 C11 dense placement가 변한다 | **PASS.** explicit areas 또는 양 축 paired aliases가 있을 때만 새 mode를 선택한다. 일반 template profile은 이전 dense placement를 유지한다. <a href="../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/tests.rs">`grid_values/tests.rs`</a> |
| 23 | named area item의 definite 크기와 auto 크기 처리가 기존 dense mode로 전파된다 | **PASS.** dedicated mode에서 definite auto-sized item을 area에 stretch하는 layout 테스트가 있고, 일반 dense mode에서는 같은 동작을 넓히지 않는다. <a href="../../../crates/spinon-layout/src/grid_tests.rs">`grid_tests.rs`</a> |
| 24 | nested grid, hidden item 또는 source-order 변화 뒤 다른 node의 area metadata/revision을 재사용한다 | **PASS.** API 37 ARM64 AVD의 동일 host에서 root area template을 반전하고 두 번 평가했다. 동일 item NodeId가 x=0에서 x=80으로 이동했고 document revision `10→11`, render-tree revision `2→3`이 증가했다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032109347Z-0ee50c0d/result.json">AVD 결과</a> |
| 25 | ADB 초기 실패를 기기 연결 문제로 오진해 앱 결함을 가린다 | **PASS.** 로그를 분리했다. 첫 시도는 pinned V8 checkout 누락으로 ADB 설치 전에 멈췄고, 다음 앱 실행은 C11.3.3 MainActivity route 부재로 C11.1 경로에 빠졌다. route 수정 뒤 같은 AVD에서 전체 행렬이 실행됐다. [`runtime README`](./c11-3-3-grid-template-areas-runtime-2026-10-14/README.md) |
| 26 | 알 수 없는 scene 또는 track mismatch가 Android에서 runtime/GPU draw까지 진행한다 | **PASS.** MainActivity intent allowlist와 C11.3.3 runtime host를 연결했다. AVD negative case는 unknown scene을 draw 전 거부하고 mismatch는 예상 property로 거부한다. |
| 27 | 단일 smoke만 통과한 결과를 26-scene 지원으로 과장하거나 다른 artifact에 귀속한다 | **PASS.** API 37 ARM64 AVD 결과는 run ID와 app SHA에 묶여 있다. 26 scene·65 node 중 success 19, cascade fallback 5, expected rejection 2이며 61 visible frame의 최대 geometry 오차가 0 CSS px다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-matrix-20261014T023829118Z-d027504f/result.json">`result.json`</a> |
| 28 | 픽셀 readback이 허용 scene/sample 범위를 벗어나거나 compositor 결과라고 오해된다 | **PASS, 범위 제한.** Android PixelCopy 2/2, iOS Metal drawable 2/2는 고정 Chrome 표본과 exact다. 잘못된 좌표는 readback 전에 거부한다. iOS 표본은 compositor 제출 전이며 AVD renderer는 ANGLE/Vulkan/SwiftShader다. 실기기·hardware GPU·compositor 이후 픽셀은 증명하지 않는다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/result.json">`iOS 결과`</a> |
| 29 | fake ADB의 shebang exec 경합이 host process identity를 바꾸고 collector 종료 검사를 실패시킨다 | **PASS.** test shim이 `/usr/bin/env node` 대신 `process.execPath`를 실행하도록 바뀌었다. collector 정리 오류는 stream 이름과 원인을 보존하고, 통합 회귀는 29/29 benchmark 테스트를 통과했다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/full-regression-final.log">`회귀 로그`</a> |
| 30 | 테스트/로드맵 문구가 C11.3.3 전체 완료, WPT 적합성 또는 미확인 플랫폼 지원을 암시한다 | **PASS.** `spec/STATUS.md`와 내부 범위 문서는 C11.3.3을 미완료로 둔다. Chrome 154는 고정 oracle이지 Spinon WPT 결과가 아니며, 공개 CSS Grid 전체 지원 문구도 없다. C12.4 source-level 검사는 공용 초기화 method와 C11.3.3 제외 조건을 확인한다. |

## 별도 diff sweep

- 기본/옵트인 FFI release artifact를 현재 source에서 다시 빌드했다. 기본 archive에는 C11.3.3 symbol 및 scene payload가 없고, opt-in archive에는 host 생성·평가·픽셀 요청 symbol과 두 scene이 있다. 해시는 <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/ffi-artifact-boundary.json">artifact 경계 JSON</a>에 기록했다.
- C ABI의 untrusted scene ID, null/잘못된 pointer-count 조합, count overflow는 C11.3.3 feature test 4/4가 거부했다.
- Android/iOS 경로의 fixture feature gate, run ID, marker prefix, 320×240 viewport는 현재 simulator 결과와 일치한다. Chrome oracle sha와 fixture sha는 각 result JSON에 남아 있다.
- 새 명령과 matrix generator의 drift 검사는 `package.json`에 연결됐고, 문서 변경은 `git diff --check`, `cargo fmt --all -- --check`, workspace Clippy 및 통합 `bun run test` 로그를 통해 확인했다.

## 남은 확인 경계

WPT runner/적합성, 실기기 SM-S731N, hardware GPU, compositor 제출 후 pixel, 모든 CSS Grid 문법 조합과 nested/hidden mutation runtime은 확인하지 않았다. 이 항목을 이유로 현 PR의 성공 결과를 넓혀 해석하지 않는다.


## PR #152 병합 후 PARTIAL 해소 · 2026-10-14

원 검토에서 증거 한계로 남긴 #4, #14, #24를 각각 직접 typed-range 주입, pinned Chrome 154 + Rust placement, Android API 37 AVD 동일-host mutation으로 재검증했다. 이 문서의 30 관점은 현재 구현과 해당 후속 증거를 기준으로 모두 PASS다. 이는 제한된 C11.3.3 profile review이며 Spinon WPT 적합성·실기기·hardware GPU·전체 CSS Grid 완료 판정은 아니다. [후속 증거와 실행 기록](./c11-3-3-post-merge-followup-2026-10-14/README.md) · [구현 후속 20-view review](./c11-3-3-post-merge-followup-implementation-review-2026-10-14.md)
