# C11.2.1 numeric Grid 배치 · 모바일 런타임 실행 · 2026-10-12

**판정:** 공통 V8→HostDocument→Stylo→Taffy→WGPU 경로에서 고정 `numeric-lines` 장면을 Android AVD와 iOS Simulator로 실행했다. 두 플랫폼의 네 frame이 pinned Chrome 154 reference와 정확히 일치하고 surface present marker가 나왔다. Android 실기기와 pixel readback은 이번 실행에서 확인하지 않았다.

## 실행 환경과 결과

| 항목 | Android | iOS |
| --- | --- | --- |
| 대상 | Android 17 / API 37 AVD `emulator-5554`, `sdk_gphone16k_arm64`, arm64 | iPhone 17 Pro Simulator / iOS 26.2, `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` |
| 런타임 | `spinon_c1121_grid_placement=true` → V8 host → Rust runtime profile → WGPU surface | `--spinon-c1121-grid-placement` → AppDelegate → V8 host → Rust runtime profile → WGPU surface |
| 표면 | 840×630 px, ANGLE GL; device는 SwiftShader(Vulkan) | 960×720 px, WGPU Metal, `Apple iOS simulator GPU` |
| 실행 | `status=0`, `layout=ready`, 4 boxes | `status=0`, `layout=ready`, 4 boxes |
| 앱 아티팩트 SHA-256 | APK `a6c1134a4b7f10754a9b2e58898375520defb675615916fecb4c607cda8d58e0` | executable `2eeeb65771089339a0397d9fbf123891e83187d23a1928cdc11849257ff41967` |

Android는 소프트웨어 렌더러 AVD 결과이며 Android 실기기 GPU 결과가 아니다. 실행 당시 `adb devices -l`에는 `emulator-5554`만 있었다.

## Chrome geometry 비교

고정 reference는 Chrome 154.0.8037.98 (`b859317bf11f6be47f9b7799ec690a0a42a1fb33`)의 `numeric-lines` 장면이다. Root를 포함해 각 node의 CSS px frame이 양 플랫폼 로그에서 동일하다. 각 필드의 Chrome 대비 최대 절대 오차는 **0 CSS px**다.

| node | 기대 frame (x, y, width, height) CSS px | Android | iOS |
| --- | --- | --- | --- |
| 1 · viewport root | `(0, 0, 360, 800)` | exact | exact |
| 2 · Grid | `(0, 0, 250, 85)` | exact | exact |
| 3 · 양수 line + span | `(0, 0, 165, 40)` | exact | exact |
| 4 · 음수 line | `(170, 45, 80, 40)` | exact | exact |

고정 장면은 3개 explicit column track(`80px`, gap `5px`)과 2개 row track(`40px`, gap `5px`)을 사용한다. 양수 line과 span은 두 열을 차지하고, 음수 line은 explicit grid의 끝에서 역산한다.

## 시각 결과와 원본

- [Android AVD 화면 캡처](android-avd.png) · 1080×2400 · SHA-256 `9c291eb2b4b78e0522ac8ea978f084a143e136ff95de1566797a3012bd6b35da`
- [iOS Simulator 화면 캡처](ios-simulator.png) · 1206×2622 · SHA-256 `04ab77dd81e3f411053c2c69c8944c284b0d87736e1d9931cddb92d1262d9cc1`
- [Android runtime·renderer·present log](android-avd.log)
- [iOS runtime·renderer·present log](ios-simulator.log)

캡처에서는 파란색 양수 span이 두 track 폭을, 빨간색 음수 배치가 두 번째 row와 마지막 column을 차지한다. 캡처와 `presented boxes=4`는 surface에 장면이 제출된 근거이지, 이번 장면의 Android PixelCopy 또는 iOS Metal texture pixel readback은 아니다.

## 자동 검증

- `cargo test --locked -p spinon-layout grid_placement -- --nocapture`: 4 passed.
- `cargo test --locked -p spinon-layout grid::tests -- --nocapture`: 9 passed, including nested Grid mode and hidden child placement boundaries.
- `cargo test --locked -p spinon-style-to-layout c11_2_1_grid_placement_tests -- --nocapture`: 9 passed. Positive/negative lines, 양 끝 span, 역순, shorthand cascade, overlap, zero/equal/named/implicit/mixed-auto 실패 및 부분 결과 비공개를 확인했다.
- `cargo test --locked -p spinon-runtime c1121 -- --nocapture`: 3 passed. V8 runtime profile의 frame 게시, mixed auto/explicit 거부, overlapping item DOM paint snapshot 순서를 확인했다.
- `cargo test --locked -p spinon-ffi --features c12-4-metal-surface-readback-test c11_2_1 -- --nocapture`: 1 passed. null host와 잘못된 출력 인수를 거부한다.
- Chrome reference source 검증: `bun run test:css-c11-2-1-grid-placement` 6/6, runtime matrix inventory gate `bun run test:css-c11-grid-runtime-matrix` 12/12.
- `mise exec -- bun run test`: JS 2/2, CSS reference 308/308, Android touch 분석 29/29, Rust workspace 617 passed·4 ignored, 추가 Release max-depth test 1/1.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` 및 `cargo fmt --all -- --check` 통과.
- Android 앱 빌드와 iOS Simulator 앱 빌드 통과.
- [구현 실패 관점 검토 20개](../c11-2-1-grid-placement-implementation-review-2026-10-12.md). 겹침 snapshot 검사는 DOM 순서를 고정하며 surface pixel 검증은 아니다.

1,024-depth runtime-worker 검사는 8 MiB worker가 사용되는 최적화 profile에서 실행한다. 이 경계 test는 Debug에서 stack overflow가 나므로 Debug harness에서는 제외하고 기본 `bun run test` 안에서 Release로 별도 실행한다. 이 변경은 테스트 실행 profile을 분리하며 제품 stack 용량을 바꾸지 않는다.

## 남은 경계

- Android 실기기에서 동일 scene 실행을 확인하지 않았다. 현재 연결은 AVD만이다.
- 이번 C11.2.1 scene에 대한 Android PixelCopy와 iOS Metal pixel readback은 하지 않았다. 화면 캡처 및 present marker를 pixel parity로 취급하지 않는다.
- 모바일 앱에서는 고정 `numeric-lines` 단일 scene을 실행했다. 10-scene Chromium corpus 전체 모바일 실행은 별도 행렬 관문이다.
- WPT 후보는 named/implicit/mixed auto-placement를 포함해 이 제한 profile에 그대로 적용하지 않았다. 저장된 Chrome 후보 결과는 Spinon WPT 적합성 결과가 아니다.
- C11.2.1의 범위는 explicit track 내부 numeric line/span placement다. named line/area, implicit track, mixed auto-placement, dense/column flow, intrinsic sizing, 전체 CSS Grid 지원은 미완료다.
- 따라서 이 기록은 C11.2.1 전체 완료, Android 실기기 parity, surface pixel parity 또는 CSS 전체 지원을 뜻하지 않는다.
