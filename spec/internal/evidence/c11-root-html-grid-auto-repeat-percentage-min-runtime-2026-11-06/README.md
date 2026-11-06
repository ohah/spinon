# C11.1 Root HTML auto-repeat percentage minimum 실행 근거

## 범위

기본 parsed Root HTML `RootHtmlAutoRepeatV1`에서 definite 축의
`repeat(auto-fill|auto-fit, minmax(<finite non-negative percentage>, 1fr))`를
검증했다. 반복 fragment는 한 축의 1~2개 track으로 제한한다. 0%의 1px floor는
반복 횟수 산정에만 적용하며 Stylo typed percentage는 Taffy sizing 입력에 보존한다.
100% 초과 minimum은 clamp하지 않아 실제 track overflow를 유지한다. C11.1 전체와
C11 완료를 뜻하지 않는다.

사전 비교 기준은 [계획](../../../../plan/c11-root-html-grid-auto-repeat-percentage-min.md),
[Chrome 157 관측과 독립 기대값](../c11-root-html-grid-auto-repeat-percentage-min-precomparison-2026-11-06/README.md),
고정 runtime fixture의 SHA-256 `fec26d7a64139fc16d9c313e630932bc76d37075a65084bad49b1bcf253df1b5`다.
6개 장면은 auto-fill/auto-fit, columns/rows, 0%, 두-track pattern, 120% overflow를 포함한다.

## Rust 검증

| 명령 범위 | 결과 |
| --- | --- |
| `spinon-layout root_html_auto_repeat` | 11/11 통과 |
| `spinon-style-to-layout root_html_auto_repeat` | 15/15 통과 |
| Stylo percentage-minimum cascade | 1/1 통과 |
| 기본 `RuntimeSession` percentage-minimum geometry | 1/1 통과, 6개 Grid scene·27 boxes |
| 기존 C11.4 positive percentage auto-repeat layout 회귀 | 13/13 통과 |
| 기존 C11.4 percentage auto-repeat cascade 회귀 | 6/6 통과 |
| scoped Clippy (`spinon-style`, `spinon-layout`, `spinon-style-to-layout`, `spinon-runtime`, `-D warnings`) | 통과 |
| `cargo fmt --all -- --check` | 통과 |

실행 로그는 [`rust/`](rust/)에 있다. `cargo test --locked -p spinon-style
c11_1_root_html`은 29개 중 27개 통과, 2개 실패다. 실패는 기존 `grid-auto-rows`
지원 제한 기대값(`15px min-content`)과 fit-content가 auto row를 제외한다는
기대값에 관한 것이다. 이 변경은 `grid-template` auto-repeat cascade helper만
수정하며 두 실패 코드 경로는 변경하지 않았다. 따라서 해당 suite를 green으로
표현하지 않는다. 전체 workspace 테스트도 실행하지 않았다.

## Android·iOS Simulator

- Android API 37 ARM64 AVD에서 기본 C04.12 parsed Root HTML 화면 경로에 새 fixture를
  연결했다. 최종 요청은 `status=0`, `layout=ready`, 27 boxes, root 320×360 CSS px,
  viewport 320×240 CSS px다. 화면은 [Android screenshot](android/simulator.png),
  현재 PID 실행은 [runtime log](android/runtime.log)다.
- iPhone 17 Pro / iOS 26.2 Simulator도 같은 parsed Root HTML 경로에서 최종 sequence 3,
  `status=0`, `layout=ready`, 27 boxes와 같은 root/viewport 크기를 냈다. 화면은
  [iOS screenshot](ios/simulator.png), 로그는 [iOS log](ios/log.txt)다. 시작 중 오래된
  요청 하나가 `status=-12`로 superseded됐으며 최종 sequence 3 결과만 통과로 집계했다.
- 두 Simulator bundle의 fixture source SHA-256은 고정 파일과 동일하다. 이는 화면이
  표시되고 scene 제출까지 됐다는 visual smoke다. 실기기, hardware GPU readback,
  Android/iOS pixel parity 또는 iOS compositor 이후 픽셀 검증은 아니다.
- 앞선 iOS 시도에서 C04.10 route 인자가 빠져 기본 앱이 뜬 실행은 잘못된 route로
  분류하고 제품 경로 검증에서 제외했다. 해당 보조 캡처와 전체 Android logcat은
  로컬 evidence 디렉터리에만 보존했다.

## Fixture artifact 경계

기존 opt-in `SPINON_ENABLE_C11_1_ROOT_HTML_AUTO_REPEAT_ZERO_MIN_FIXTURE_TEST=1`에서
Android APK와 iOS app bundle에 3,070-byte HTML이 들어가고 source와 같은 SHA-256을
가짐을 확인했다. 동일 빌드를 flag `=0`으로 다시 만들어 기본 APK 및 iOS Simulator
bundle에서 asset이 빠지는 negative control을 확인했다. 상세 결과와 빌드 로그는
[`artifact-boundary.txt`](artifact-boundary.txt), Android/iOS의
[`build-opt-in.log`](android/build-opt-in.log), [`build-default.log`](android/build-default.log),
[`iOS opt-in log`](ios/build-opt-in.log), [`iOS default log`](ios/build-default.log)다.

## Agent Browser 입력 대조

runtime fixture에는 화면에 120% overflow를 드러내는 `width:240px` item과 auto-fit
stretch를 보이는 `width:auto` item이 있다. Agent Browser의 Chrome 157.0.8103.39에서
해당 fixture 자체를 열어 auto-fit item 200px / track list `200px 0px …`, overflow
item 240px / track 240px임을 확인했다. 이는 precomparison fixture의 intrinsic 10px
item control과 구별한 입력별 예상값이다. 측정은
[`Agent Browser observation`](agent-browser-runtime-input-observation.json)에 기록했고
검증 후 해당 Agent Browser 세션과 전용 headless Chrome process를 종료했다.
