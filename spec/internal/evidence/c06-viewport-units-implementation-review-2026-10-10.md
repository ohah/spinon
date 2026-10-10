# C06.6a viewport 단위 구현 검토와 실행 근거

## 검토 범위

계획 검토와 별도로 실제 변경 코드, CSS token scanner, Chromium 기준, Android/iOS host routing, resize·renderer 수명 경계를 검토했다. 표의 각 항목은 서로 다른 입력·실패 경로다. 발견한 두 결함은 수정한 뒤 simulator에서 해당 경로를 다시 실행했다.

| # | 공격 관점 | 확인·조치 |
| --- | --- | --- |
| 1 | 기본 `vw`와 `vh`가 서로 다른 축을 잘못 참조 | Chromium reference 및 Rust layout 비교에서 width·height viewport 축을 각각 비교한다. |
| 2 | CSS viewport에 drawable pixel 또는 DPR을 곱함 | DPR 1·2에서 computed CSS 값·CSS frame이 동일한지 확인하고 Android `790×263` drawable을 CSS `301×100`과 구분한다. |
| 3 | `sv*` 접두 계열 일부만 동작 | fixture에서 `svw`, `svh`, `svi`, `svb`, `svmin`, `svmax`를 `calc()` 입력으로 함께 실행한다. |
| 4 | `lv*` 접두 계열 일부만 동작 | fixture에서 `lvw`, `lvh`, `lvi`, `lvb`, `lvmin`, `lvmax`를 `calc()` 입력으로 함께 실행한다. |
| 5 | `dv*` 접두 계열 일부만 동작 | fixture에서 `dvw`, `dvh`, `dvi`, `dvb`, `dvmin`, `dvmax`를 `calc()` 입력으로 함께 실행한다. |
| 6 | logical inline/block 축을 항상 폭으로 매핑 | 현재 supported `horizontal-tb`에서 inline=width, block=height를 fixture의 `vi`/`vb`와 각 prefix 변형으로 비교한다. vertical writing mode는 이 profile에서 지원하지 않는다. |
| 7 | `vmin`/`vmax`가 portrait 전용 상수 | 320×800, 800×320, 390×844 viewport를 각각 Chrome과 비교한다. |
| 8 | viewport 단위가 `calc()`·`min()` 안에서 타입을 잃음 | `calc(25svw + 1svi + 1svmin)` 등 mixed length를 Chrome computed value와 Taffy frame으로 비교한다. |
| 9 | `var()` 치환 뒤 viewport 값이 사라짐 | 사용자 지정 속성 `--viewport-width:12vw`를 `width:var(...)`로 cascade해 비교한다. |
| 10 | Flex `gap`·`flex-basis`가 computed value와 used frame에서 엇갈림 | gap/basis 입력과 최종 geometry를 별도 비교한다. Chrome의 used width와 Stylo의 `auto` 차이는 최종 frame으로 판정한다. |
| 11 | margin/padding에서 단위만 바뀌고 spacing basis가 달라짐 | `margin-left`, `padding-left` computed CSS px를 기준값에 비교한다. |
| 12 | DPR 변경이 layout 값을 바꿈 | 같은 viewport의 DPR 1/2 reference nodes와 Rust values/frames를 정확 비교한다. |
| 13 | 같은 surface 크기 재전달이 environment revision을 계속 증가 | 기존 runtime same-environment no-op test를 workspace에서 실행한다. |
| 14 | width/height resize가 environment cache를 재사용 | Rust resize fixture와 Android/iOS 301×100↔341×128에서 새 revision과 새 root frame을 확인한다. |
| 15 | 빠른 신규 revision 뒤 오래된 layout이 최신 scene을 덮음 | 기존 runtime stale-completion test와 `RuntimeRenderKey` admission 검사를 실행한다. |
| 16 | Android surface 재생성 중 renderer destroy가 새 계산 scene까지 지움 | Android에서 resize 직후 빈 scene을 재현했다. renderer destroy의 불필요한 scene invalidation을 제거한 뒤 301→341→301 resize 모두 `boxes=15`와 `presented`를 확인했다. |
| 17 | iOS launch flag가 demo 내부에서만 읽히고 AppDelegate 선택에서 누락 | 처음 실행이 빈 흰 화면으로 재현됐다. AppDelegate routing을 추가하고 `layout=ready`, `boxes=15`, `presented`를 재확인했으며 CSS reference test가 routing을 고정한다. |
| 18 | Android host flag와 V8 fixture가 서로 다른 화면 경로를 탐 | MainActivity intent→C0410 demo→JNI→Rust FFI 경계를 test와 실제 API 37 emulator 실행으로 확인한다. |
| 19 | C21 전 `cqw` 계열이 Stylo의 viewport fallback으로 성공 | 여섯 container unit을 inline style·author stylesheet에서 모두 fail-closed하는 Rust tests를 실행한다. |
| 20 | CSS source scanner가 주석·문자열을 오검출하거나 escape/malformed token을 놓침 | nested function·escaped dimension token·주석/문자열 false-positive·bad token stream을 scanner와 cascade tests에서 확인한다. |

## 실행 근거

- Chromium oracle은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`다. 3개 viewport × DPR 1·2에서 15개 node를 고정 capture했다. fixture·inventory·capture tool hash는 [reference](../../../tests/fixtures/css/references/c06-viewport-units-v1.json)에 저장했다.
- Rust evidence: `cargo test --locked --workspace`, `cargo clippy --locked --workspace --all-targets`, `cargo fmt --all -- --check`, `cargo check --locked -p spinon-ffi --all-features`가 모두 종료 코드 0으로 통과했다. `node --test tools/css-reference/c06-*.test.mjs`는 24개 테스트 통과, 실패 0개다.
- Native evidence: `SPINON_ENABLE_C04_RUNTIME_GPU=1`로 Android API 37 emulator 및 iPhone 17 Pro / iOS 26.2 Simulator 앱을 빌드했다. Android screenshot/log는 [301×100](c06-viewport-units/android-301x100.png), [341×128](c06-viewport-units/android-341x128.png), [원본 log](c06-viewport-units/android-api37-emulator.log)다. iOS screenshot/log는 [301×100](c06-viewport-units/ios-301x100.png), [341×128](c06-viewport-units/ios-341x128.png), [원본 log](c06-viewport-units/ios-26.2-iphone-17-pro-simulator.log)다.
- Android 화면은 emulator ANGLE/SwiftShader software backend다. 실기기·hardware GPU·성능 주장은 하지 않는다.
- `RuntimeGpuHost`를 unit test에서 직접 만들려는 경로는 현재 Rust unit test process에서 V8 Isolate 생성이 되지 않아 사용할 수 없었다. 대신 같은 renderer-destroy race를 Android API 37 emulator의 V8→Stylo→Taffy→WGPU resize 실행으로 직접 재현·수정·재확인했다.

## 미확인 경계

- WebView toolbar collapse, visual viewport, OS keyboard resize, safe area, vertical writing mode는 실행하지 않았고 이 계약이 보증하지 않는다.
- iOS/Android 실기기, hardware GPU, 장시간 resize stress 및 성능 비교는 범위 밖이다.
- C06.6b container-relative units는 C21 query-container 계약 전까지 미지원이다.
- C06.6a는 현재 branch 작업이며 아직 merge되지 않았다. 문서 ID `0042`는 버전이 아니고 내부 숫자 계약 및 Spinon crate 버전은 `0.1.0`을 유지한다.
