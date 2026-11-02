# Root HTML direct `fr`·percentage track + fixed implicit row runtime evidence

## 범위와 기준

이 후보는 기존 parsed Root HTML 경로에서 direct finite non-negative explicit `px`·`fr`·percentage track과 단일 finite non-negative px `grid-auto-rows`를 조합하는 제한 slice다. 계획 PR [#1026](https://github.com/ohah/spinon/pull/1026)은 2026-11-02에 rebase merge됐고 merge SHA는 `3f7bfa71ae67a1f61cb8a5154f0cb74137bad9fc`다. 공개 CSS 지원이나 C11.1/C11 완료로 승격하지 않는다. 공식 완료 집계 `72/213`과 상태 체크박스는 그대로다.

구현 전 기준은 [Chrome 156 사전 비교](../c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/README.md)다. 네 개의 definite-size scene은 explicit columns/rows 각각의 `fr`·percentage sizing, px implicit row, row auto-placement, fixed gaps와 visible overflow를 독립 산술 및 Chromium CSSOM/frame 관측으로 고정했다.

## 구현·Rust 검증

- `spinon-style-to-layout`: `RuntimeRootHtmlTextPaintV1`에서 unnamed direct finite non-negative px/percentage/fr만 허용하고, 기존 definite-axis 및 computed-size 범위 검사를 유지한다. px 외 track을 고정 implicit row와 결합했을 때에도 item-capacity 사전 검사가 동일한 범위를 인식하도록 수정했다.
- `spinon-layout`: `RootHtmlImplicitRowsV1`이 두 explicit 축에서 px/percentage/fr direct track을 받고, auto-flow row·auto columns·단일 fixed-px implicit row와 기존 sparse auto-placement만 유지한다.
- 검증: [layout Grid 74개](layout-grid-tests.log), [style projection 12개](style-projection-tests.log), [Root HTML runtime Grid 20개](runtime-root-html-grid-tests.log), [Clippy](clippy.log), [rustfmt](format-check.log), [diff check](diff-check.log)가 통과했다. 통합 runtime 네 scene은 computed track type과 모든 root-relative item frame을 검사한다. negative matrix는 named area, intrinsic/auto-repeat explicit track, implicit percentage/fr/list, implicit columns, dense/column flow, authored placement/order, clipping, text/nested item을 실패시키고 partial scene이 없는지 확인한다.
- 네 scene의 Rust 런타임 geometry는 별도 기준 파일의 frame과 일치한다. Percentage-column overflow item은 x=35, width=100으로 root의 x=120 오른쪽을 15 CSS px 넘어가며 clamp되지 않는다.

- 구현 코드·실행 경로에 대한 별도 20개 실패 관점: [implementation/runtime review](implementation-runtime-review-20.md).
- 실제 PR #1027 변경 diff에 대한 30개 실패 관점: [PR diff review](pr-diff-review-30.md).

## Android / iOS Simulator

실제 `RuntimeSession::new_runtime_gpu_with_root_html()` 기본 Root HTML 경로를 사용했다. 앱 빌드에서 C04 runtime GPU만 켰고 C04 fixture-test, C11.2.3 fixture-test, Metal readback fixture-test는 모두 껐다. Android API 37 AVD `emulator-5556`과 iPhone 17 Pro / iOS 26.2 Simulator에서 단일 overflow scene을 실행했다. 앱 로그 양쪽에서 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready boxes=6`, `SPINON_C0410_DRAW ... presented boxes=6`를 확인했다.

| 플랫폼 | 실행 결과 | 화면 | 판정 범위 |
|---|---|---|---|
| Android API 37 AVD, 1080×2400, density 420 | [runtime log](android-api37-runtime.log) · [build log](android-build.log) | [screenshot](android-api37.png) | 기본 Root HTML 앱 boot/layout/draw·present 및 visible overflow의 시각적 확인. PixelCopy 표면 readback은 하지 않았다. |
| iPhone 17 Pro / iOS 26.2 Simulator, 1206×2622 | [runtime log](ios-26_2-runtime.log) · [build log](ios-simulator-build.log) | [screenshot](ios-26_2-simulator.png) | 기본 Root HTML 앱 boot/layout/draw·present 및 visible overflow의 시각적 확인. Metal drawable readback은 하지 않았다. |

Android APK의 `assets/index.html` 및 설치된 iOS 앱의 `index.html` SHA-256은 후보 HTML의 `3b65d6be1ddca70975b6cbbf95fc7c743a0c7112fa903f67da2e4b1d5296cc7e`와 일치했다. 상세 관측은 [platform-runtime-observation.json](platform-runtime-observation.json), 파일 checksum은 [hash-manifest.txt](hash-manifest.txt)에 기록했다. 화면 캡처는 색·픽셀 동일성 검증으로 해석하지 않는다.

PR용 텍스트 log 사본에서는 Git diff whitespace 검사에 걸린 줄 끝 공백과 마지막 빈 줄만 제거했다. 명령, test case, diagnostics와 결과 summary는 유지했다.

한 앱 문서에 네 개 `<main>` scene을 합친 초기 실험은 이 Root HTML 앱 canvas 입력 계약에서 CascadeFailed/no-draw가 발생했다. 해당 실패 자료([log](android-api37-multiple-grid-attempt.log), [screenshot](android-api37-multiple-grid-attempt.png))는 범위 밖 입력으로 보존했다. 네 조합 전체는 독립 Rust runtime matrix에서 검증하고, 앱은 기본 경로에서 단일 scene을 실행했다.

후보 HTML을 빌드 입력으로 복사하기 전에 `examples/bootstrap`과 생성된 `build/spinon/bootstrap` 다섯 파일을 백업했다. 빌드 후 다섯 원본과 복원 파일을 byte-for-byte 비교했고 SHA-256도 각각 일치했다. 앱 프로세스는 캡처 후 종료했다. 이 작업에서 Agent Browser를 시작하지 않았고, 종료할 Agent Browser 프로세스도 없었다.

## 실패 관점 검토 및 남은 한계

- 아직 미검증: Android PixelCopy 및 iOS Metal surface readback, 실기기/hardware GPU, pixel parity, WPT, intrinsic/`minmax()`/auto-repeat, named line/area 결합, arbitrary CSS Grid 및 C11 전체 적합성.
