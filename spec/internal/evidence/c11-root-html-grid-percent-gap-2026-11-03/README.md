# C11.1 parsed Root HTML Grid percentage gap 사전 비교와 실행 근거

## 판정 기준과 출처

- 규범 기준: [CSS Gaps Level 1 §2.3 Percentages In gap Properties](https://www.w3.org/TR/css-gaps-1/#gap-percent). Grid gap percentage는 해당 축 content box 크기를 기준으로 하고, cyclic percentage는 intrinsic contribution 계산 중 0, contents layout에서는 content box로 해석한다. [§2.4 legacy aliases](https://www.w3.org/TR/css-gaps-1/#gap-legacy)는 `grid-gap`을 `gap`의 호환 별칭으로 정의한다.
- 상호 확인: [CSS Grid Layout Module Level 2 §11.1 Gutters](https://www.w3.org/TR/css-grid-2/#gutters). Gap은 track 사이의 고정 크기 gutter로 sizing과 item span에 참여한다.
- 브라우저 oracle: Agent Browser `HeadlessChrome/156.0.0.0`, user agent `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/156.0.0.0 Safari/537.36`.
- 입력: [`grid-percentage-gap-v1.html`](../../../../tests/fixtures/css/c11/grid-percentage-gap-v1.html), SHA-256 `c79a97f3d5fa55c838e8e1ab5386cec4e3cbb73adee682c614bad0e5cce28a36`.
- Agent Browser 관측 JSON SHA-256: `12de0fa47eb4ae66473c334618e3f1de0d9dc32fb1d77380ca9d39ef1648bb12`.
- Chrome 장면 캡처 SHA-256: `f415a6f8922b7e4e1c3829241a35bd6d5588007f316d6055f259c6b94961a1c4`.
- 화면: [Chrome 156 precomparison](./chrome156-precomparison.png). 원 관측: [JSON](./chrome156-observation.json).

## 독립 기대값과 브라우저 관측

기대값은 CSS Gaps §2.3의 축별 content-box percentage 규칙과 definite Grid sizing으로 계산했다. Chrome은 computed percentage 문자열과 최종 used track/frame을 별도로 반환했다.

| 장면 | 독립 기대값 | Chrome 156 관측 | 결과 |
|---|---|---|---|
| `basis` | 10% of 200px = 20px column gap; 10% of 100px = 10px row gap. Tracks `[40,140]` / `[20,70]`; item origins `(0,0)`, `(60,0)`, `(0,30)`, `(60,30)` | `gridTemplateColumns=40px 140px`; rows `20px 70px`; row/column gaps `10%`; item frames match | 일치 |
| `zero` | 0% resolves to 0px on both axes | `0%`; tracks and item frames unchanged | 일치 |
| `zero-basis` | A definite 0px content width makes a 75% column gap 0px; both zero-width item origins are x=0 | computed `75%`; two `0px` columns; both item origins x=0; root scroll width 0 | 일치 |
| `shorthand` | `gap:5% 10%` means 5px row gap and 20px column gap; tracks `[30,150]` / `[10,85]` | tracks `30px 150px` / `10px 85px`; origins `(0,0)`, `(50,0)`, `(0,15)`, `(50,15)` | 일치 |
| `legacy-alias` | `grid-gap:5% 10%` has the same longhand values and geometry as `gap:5% 10%` | computed row/column `5%`/`10%`; tracks and item origins exactly match `shorthand` | 일치 |
| `over-100` | 120% of 100px = 120px; second item x=130px | computed `120%`; second item x=130px; root scroll width 140px | 일치, clamp 없음 |
| `implicit-row` | 25% of 100px = 25px implicit row; 10px row gap; second row y=30px | implicit rows `20px 25px`; second row y=30px | 일치 |
| `column-flow` | 50% of 200px = 100px implicit column; 20px gap; next column x=60px | columns `40px 100px`; next item x=60px | 일치 |
| `named-area` | 12px column/7px row gap; lower row y=27px; second column x=52px; spanning header width 132px | `scrollWidth=132`, `scrollHeight=77`; header width 132px; side/content positions `(0,27)` / `(52,27)` | 일치 |
| `named-area-px` | Existing px gap route uses 6px columns / 4px rows; lower row y=24px; second column x=46px; header width 126px | same used track sizes and frames; scroll size 126×74px | 일치 |
| `invalid-negative` | Invalid `gap:-10px` declaration is discarded; preceding 5px remains | computed row/column gap `5px`; second item x=15px | 일치 |
| `math-control` | Chrome computes the valid `calc()` value for this size; Spinon Root HTML profile remains out of scope and must reject `column-gap` without partial layout | computed `calc(5% + 2px)`; second item x=22px | 경계 고정 |

이 고정 비교는 12 scene만 다룬다. cyclic/indefinite gap resolution, general author CSS, arbitrary track grammar, WPT runner 적합성, app surface pixel equality를 입증하지 않는다.

## 현행 구현 기준선

현 `main` `227aa285`에서 `root_html_grid_gap_admits_only_finite_nonnegative_px_or_normal`를 실행해 1/1 통과했다. 이 테스트는 `Percentage(0)`도 거부한다. `project_styles`의 Root HTML Grid gate 역시 현재 `Normal`/finite non-negative px만 허용하고, `root_html_named_area` gate는 gap 0만 허용한다. 이는 현재 제한을 확인한 기준선이며 새 구현의 통과 증거가 아니다.

## 후속 구현 계획

세부 수용 범위, 거부 경계, finite extent 검사, 구현·실행 판정은 [C11.1 percentage gap 계획](../../../../plan/c11-root-html-grid-percentage-gap.md)을 따른다. Chrome 12 scene 관측과 독립 기대값은 사전 비교 입력이다. 모바일 앱의 단일-grid smoke는 별도 fixture를 사용하고 서로 다른 근거를 합쳐 pixel parity로 주장하지 않는다.

## 구현과 Rust 검증

구현 commit은 `f92352d97eddfa5e6142d9db78d5a1900879587b`이며 [source revision](./source-revision.json)에 tree hash를 기록했다. 제품 범위는 기본 `RuntimeRootHtmlTextPaintV1` 경로에서 definite content-box axis를 basis로 하는 유한·비음수 percentage `row-gap`/`column-gap`이다. `gap` shorthand와 `grid-gap` alias는 computed longhand 경로를 공유한다. >100%는 clamp하지 않으며 math, indefinite basis, 비정상 값과 finite-f32 초과는 fail-closed한다. Flex와 다른 runtime profile은 변경하지 않았다.

| 검사 | 결과 | 원본 |
|---|---:|---|
| Percentage-gap Root HTML cascade/projection/layout | 10/10 | [Rust log](./style-focused-pass02-clean.log) |
| Gap resolver `normal`/px/%/basis 경계 | 2/2 | [Rust log](./style-gap-helper-pass01-clean.log) |
| Grid placement·implicit-row/column direct adapter | 57/57 | [Rust log](./layout-focused-pass02-clean.log) |
| 200×200 simulator fixture exact item geometry | 1/1 | [Rust log](./rust-runtime-fixture-pass01-clean.log) |
| Clippy, `spinon-layout` + `spinon-style-to-layout`, all targets, `-D warnings` | 통과 | [log](./clippy-pass02.log) |
| `cargo fmt --all -- --check`, `git diff --check` | 통과 | [format](./format-check-pass02.log) · [diff](./diff-check-pass02.log) |

실제 시뮬레이터 입력 [HTML](./root-html-grid-percentage-gap-runtime.html)은 SHA-256 `db10b2cfd2fcafe8062c0bdd7cfcba922d2e41c04e4471f1f29d1377ffa23d3e`, V8 DOM probe는 `685d0f3b8ab8d78a9d14a5e3dfd019c60323e4eac4f4f5dcf172c12000a79d9c`이다. 독립 기대 item frame은 `[0,0,20,20]`, `[260,0,20,20]`, `[0,40,20,20]`, `[260,40,20,20]`이다. 해당 fixture와 같은 CSS를 사용하는 [Rust 회귀 검사](../../../../crates/spinon-style-to-layout/src/c11_1_root_html_percentage_gap_tests.rs)는 네 frame을 exact 비교한다.

전체 package suite는 green이 아니다. `spinon-style-to-layout`은 319 passed / 2 failed이고, 실패는 변경하지 않은 C07 aspect-ratio computed-string 기대(`1.77778 / 1` 대 `1.7777778 / 1`)와 기존 C11 `grid-auto-columns:-1fr` 진단 타입 기대다. `spinon-layout --lib`은 376 passed / 1 failed이며 실패는 변경하지 않은 C15 HostDocument text test다. 전체 suite 결과는 [style log](./style-package-full-pass01.log)와 [layout log](./layout-package-full-pass01.log)에 보존한다. 집중 slice 통과를 전체 crate 통과로 확대 해석하지 않는다.

## Android와 iOS Simulator

두 앱은 동일한 implementation commit, HTML fixture hash, probe hash로 빌드했다. Android는 별도 API 37 ARM64 AVD `spinon_api37_1_compat16k` (`emulator-5558`, Android 17)이며 ANGLE/Vulkan/SwiftShader software backend다. 연결되어 있던 Samsung 실기기와 기존 부팅 시뮬레이터 앱은 사용하거나 종료하지 않았다. iOS는 새로 부팅한 iPhone 17 Pro Max / iOS 26.2 Simulator다.

| 대상 | 실행 확인 | 캡처와 로그 |
|---|---|---|
| Android API 37 AVD | `source=index.html`, V8 probe `root_app_marker=ready`, `layout=ready boxes=7`, `presented boxes=7`; viewport `411.42856×841.52374`, root CSS `411.42856×200` | [화면](./android-api37-avd.png) · [실행 로그](./android-runtime.log) · [설치](./android-install.log) · [launch](./android-launch.log) |
| iPhone 17 Pro Max iOS 26.2 Simulator | `source=index.html`, V8 probe `root_app_marker=ready`, `layout=ready boxes=7`, `presented boxes=7`; viewport `440×860`, root CSS `440×200`, scale 3 | [화면](./ios-26_2-simulator.png) · [실행 로그](./ios-runtime.log) · [launch](./ios-launch.log) |

Android APK SHA-256은 `318ad3ba31eb4b8deddf11259fdf553f23c94b0806416cd5e299a6be1a749043`, iOS app executable SHA-256은 `4c4f245ba75ab36bb7fde79ee62ad0a8c5cdc57565d4a3d0190debc86ea2ed9d`이다. 원 fixture·probe·앱·캡처 hash와 simulator 식별자는 [관측 JSON](./platform-runtime-observation.json)에 정리했다. 임시 Bootstrap 입력은 원본 파일 hash로 복구했다([hash 목록](./restored-bootstrap-sha256.txt)). 앱 프로세스는 캡처 후 두 대상에서 종료했다.

iOS build는 성공했으며 Xcode가 V8 static archive 안의 duplicate timestamp object에 대한 dSYM 경고와 `AppIntents.framework` 미사용 metadata skip 경고를 출력했다. 앱 설치·실행과 layout/draw marker에는 실패가 없었다. [정규화 build log](./ios-simulator-build-pass01-clean.log)에서 확인할 수 있다.

두 screenshot 모두 200 CSS px root의 우측 바깥 x=260에 두 번째 column이 보인다. 이는 시각 smoke이며 Android PixelCopy·iOS Metal drawable readback 또는 Chrome과의 pixel equality가 아니다. Exact node geometry 근거는 Rust 검사이고 mobile runtime log는 기본 app route, layout-ready, draw/present를 증명한다.

## 적대적 검토와 남은 범위

[구현/runtime 실패 관점 검토 20개](./implementation-runtime-review-20.md)는 plan review 20개와 별도로 작성했다. PR 제출 전에는 실제 최종 변경 delta에 대해 저장소 필수 30관점 검토를 새로 기록한다. 일반 CSS admission, arbitrary track grammar, WPT conformance, cyclic/indefinite gap, hardware GPU, 실기기, compositor 이후 픽셀은 검증 범위에 포함되지 않는다.

C11.1과 C11은 미완료다. 공식 `72/213`은 유지한다.
