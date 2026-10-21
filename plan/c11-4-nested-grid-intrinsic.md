# C11.4 일반 nested Grid item의 intrinsic contribution 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Agent Browser `HeadlessChrome/156.0.0.0` 사전 관찰과 계획 실패 관점 20개 검토를 바탕으로 일반 nested Grid 한 단계의 opt-in fixture·Rust trace 검증을 구현했다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 세 acceptance scene을 다시 실행했다. 현재 구현은 고정 입력에서 독립 기대값과 Chrome 관찰을 보존하며, layout algorithm 수정은 필요하지 않았다. [사전 비교](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/README.md) · [Simulator 실행·화면·hash](../spec/internal/evidence/c11-4-nested-grid-intrinsic-simulator-2026-10-21/README.md) · [PR diff 검토 후 최종 재실행](../spec/internal/evidence/c11-4-nested-grid-intrinsic-final-confirmation-2026-10-21/README.md) · [구현 실패 관점 20개](../spec/internal/evidence/c11-4-nested-grid-intrinsic-simulator-2026-10-21/implementation-review-20.md) · [PR #442](https://github.com/ohah/spinon/pull/442) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-nested-grid-intrinsic-simulator-2026-10-21/pr-diff-review-30.md). 이 opt-in fixture는 일반 nested/subgrid 지원이 아니며 C11.4 checkbox와 공식 72/213 완료 수는 그대로 둔다.

## 목표

현재 C11.4 input 수집은 각 Grid의 direct child metadata를 만들고, text/image intrinsic source input은 leaf item에 대해서만 만든다. direct nested Grid item은 outer Grid의 item metadata에는 있지만 `collect_grid_item_intrinsic_inputs`에서 non-leaf로 건너뛴다. `record_intrinsic_measurement`는 metadata가 있는 Grid item의 intrinsic query를 기록할 수 있다. 그러므로 코드 모양만 보고 결함을 가정하지 않는다. 먼저 고정 입력을 Rust `LayoutInput` 경로와 opt-in 앱 경로에 연결해 Taffy의 실제 nested-item contribution과 trace가 CSS Grid 계산에 맞는지 재현한다. 결함이 없으면 테스트·fixture 검증만 추가하고 제품 알고리즘을 불필요하게 바꾸지 않는다.

대상은 parent Grid의 direct item인 **일반 nested Grid 한 단계**다. nested Grid의 direct-leaf Ahem text 두 개가 내부 min-content/max-content track을 구성하고, 그 nested Grid border box가 outer track sizing contribution에 반영되는 경로를 고정한다. Subgrid는 parent track을 공유하는 별도 기능이므로 범위에서 제외한다.

## 입력과 기대값

[Agent Browser fixture](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/chrome-nested-grid-oracle.html)는 gap 4px, Ahem 10px/10px, 내부 `min-content max-content` columns를 사용한다. `XX XXXXXXX`의 min-content 폭은 70px, max-content 폭은 100px이고, `XXXXXX`는 60px이다. 독립 규범 계산과 browser 관찰은 JSON 및 [README](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/README.md)에 따로 기록한다.

| scene | 바깥 columns | nested item placement | 독립 기대값 / Chrome 관찰 |
| --- | --- | --- | --- |
| `span-two` | `min-content min-content`, `width:max-content` | 두 track span | outer tracks `65/65px`, outer/nested width `134px`, inner tracks `70/60px` |
| `one-track` | `min-content min-content`, `width:max-content` | 첫 track만 점유 | outer tracks `134/0px`, outer width `138px`, nested width `134px`, inner tracks `70/60px` |
| `definite-flex-control` (`#definite`) | `minmax(auto,1fr)` 두 track, outer width `120px` | 두 track span | outer tracks `58/58px`, nested used width `120px`; inner content overflow는 contribution 값과 분리해 검사 |

세 번째 scene은 intrinsic growth 기대를 대신하지 않는다. §6.6에 따라 flexible max track을 span하는 item의 auto minimum 경계와 definite available size를 통제한다. nested item의 child `b` background는 outer border-box 오른쪽으로 14 CSS px 넘는다. 앱 surface readback을 추가할 때는 root 안·경계·바깥의 표본을 분리한다. 이 표본은 해당 실행 surface의 배경색만 입증하며 border stroke나 compositor 이후 표시를 증명하지 않는다.

`inner-span-diagnostic`은 내부 Grid child 자체가 두 track을 span하고 auto-placement가 두 번째 row를 만드는 입력이다. 두 번째 수준의 span-sizing이 섞이므로 사전 화면에 남기되 계획의 acceptance와 앱 matrix에서는 제외한다.

## 표준 기준과 판정

[CSS Grid Layout Module Level 2, 2025 CRD §12.5](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-content), §12.5.1의 contribution query와 span별 planned-increase 순서를 사용한다. §6.6은 definite flex-control에서 outer item auto minimum을 해석하는 데 적용한다. Chrome 156은 구현 간 관찰자이며, 구현 결과를 보고 규범 기대값을 고치지 않는다. fractional 결과는 CSSOM 문자열 동등 대신 root-relative numeric 값으로 비교하고 허용 오차를 최대 1/64 CSS px로 둔다. 현재 정수 fixture의 세 기대값은 그 오차를 쓰지 않아도 일치해야 한다.

Acceptance에서 확인할 항목:

1. Rust layout regression은 외부 Grid node, nested Grid node, 내부 text node 모두의 frame과 두 Grid의 Columns/Rows trace를 보존한다. outer Columns contribution query에서 item ID는 nested Grid node여야 한다. 내부 text intrinsic input의 `grid_container`는 nested Grid ID로 유지한다. nested Grid 자체를 text/image leaf input으로 위장하지 않는다.
2. 두 max-content/min-content intrinsic scenes의 frame, 최종 used tracks, contribution kind/value/span/source order를 규범 기대값 및 browser observation과 각각 비교한다. nested item contribution이 내부 text leaf의 direct input과 구분되어 추적돼야 한다.
3. definite flex control은 120px root 폭과 58/58px outer tracks를 보존하며 inner content overflow를 outer contribution으로 잘못 더하지 않는다. surface sample을 추가하면 AVD와 iOS Simulator에서 root 내부/경계/외부 표본을 개별 보고한다.
4. opt-in `c11-4-grid-track-sizing-fixture-test` 앱 경로에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 acceptance scene을 실행한다. 전체 scene/frame/track/trace 수, draw/present, 앱 및 설치 산출물 SHA를 기록한다. PixelCopy/Metal 표본은 실제 캡처 경로를 통과한 경우에만 표면 근거로 센다.
5. 기본 Android/iOS product artifact에는 fixture FFI symbol·scene 문자열·Ahem fixture payload가 없어야 한다. opt-in fixture-test 빌드는 positive control로 같은 경로가 포함되는지 확인한다.
6. 관련 Rust focused test, 기존 C11.4 fixture 회귀, `cargo fmt --all -- --check`, workspace all-target Clippy `-D warnings`, Android/iOS fixture-test build를 실행하고 결과를 분리 기록한다. Simulator frame 성공을 실기기, hardware GPU, WPT 또는 compositor-after 표본으로 표현하지 않는다.

실제 구현 차이가 발견되면 원인과 최소 수정 경계를 먼저 좁힌다. unsupported item topology, style/profile, source revision 또는 scene ID는 부분 input/layout을 내지 않고 기존 fail-closed 오류를 유지한다. Public CSS 지원, API/내부 계약 숫자 버전, C11.4 상태를 이 제한 fixture만으로 확장하지 않는다.

## 구현과 simulator 실행 (2026-10-21)

세 장면을 고정 FFI scene allowlist, runtime topology/style validator, direct-layout regression 및 runtime test로 연결했다. nested Grid는 내부 text leaf input과 구분된 outer Grid item으로 남고, parent contribution trace는 item node·kind·span·value를 보존한다. 제품 layout algorithm은 바꾸지 않았다.

| scene | Android AVD·iOS Simulator 결과 | outer contribution |
| --- | --- | --- |
| `nested-grid-span-two` | outer/nested width `134px`; outer tracks `65/65px`; inner tracks `70/60px` | `MinContent`, span 2, `134px` |
| `nested-grid-one-track` | outer width `138px`, nested width `134px`; outer tracks `134/0px` | `MinContent`, span 1, `134px` |
| `nested-grid-definite-flex-control` | outer/nested width `120px`; outer tracks `58/58px`; inner tracks `70/60px` | `Minimum`, span 2, `0px` |

API 37 ARM64 AVD에서는 세 frame/track/trace/draw 장면과 Android PixelCopy 표본 5/5를 통과했고, APK와 설치 `base.apk` SHA-256이 같다. iPhone 17 Pro / iOS 26.2 Simulator에서도 세 장면 및 Metal drawable 표본 5/5가 정확히 일치했다. flex control 표본은 x=72의 내부 gap, x=119/120/133의 빨간 item, x=134의 검정 배경을 읽어 inner overflow가 parent track contribution에 더해지지 않음을 확인한다. Android는 SwiftShader AVD이고 iOS Metal sample은 OS compositor 제출 전이다.

기본 `spinon-ffi` artifact에서는 fixture symbol과 장면 payload를 제외했고 fixture-test feature build를 positive control로 확인했다. direct-layout/runtime/FFI focused tests, `cargo fmt`, JS syntax·fixture module boundary, workspace Clippy `-D warnings`, Android 및 iOS fixture 앱 build가 통과했다. ADB에 Samsung 실기기가 나타나지 않아 사용자가 simulator를 선택했다. 일반 nested Grid 문법·다단계 nesting·subgrid·WPT·실기기·iOS compositor-after surface는 미검증이다. 전체 로그·화면·SHA-256과 첫 capture 시 검증 도구에서 발견해 수정한 allowlist/trace parser/sample-map 문제는 [실행 evidence](../spec/internal/evidence/c11-4-nested-grid-intrinsic-simulator-2026-10-21/README.md)에 기록한다.

## 제외 범위

- `subgrid`, multi-level nesting, nested Grid 안의 anonymous text, nested text/image 혼합, replaced item, intrinsic ratio
- inner-grid spanning child, auto-placement row 생성, nested implicit track 확장, auto-repeat 및 arbitrary track grammar
- cyclic `%`/`calc()`, aspect-ratio cycle, orthogonal writing mode, baseline shim, alignment/overflow policy matrix
- 일반 CSS parser·computed-style 지원, arbitrary page admission, WPT pass 주장, text glyph paint parity
- Android 실기기, iOS 실기기, hardware GPU, iOS compositor 이후 pixel 및 모든 border stroke
- C11.4 전체 구현 완료, checkbox/공식 완료 수 증가, 내부 계약 version bump

## 실행 순서

1. 먼저 Rust direct-layout fixture와 runtime allowlist가 nested Grid를 같은 parent/item 관계로 보존하는지 확인한다. 현재 Chrome 수치가 다른 이유를 Taffy trace 및 per-item contribution query의 첫 divergent stage로 좁힌다.
2. 독립 규범 기대값을 변경 코드 전에 고정한다. current output이 이미 일치하면 test-only slice로 유지한다. 불일치하면 CSS §6.6, §12.5, §12.5.1의 실제 query와 Taffy 0.15.0 소스를 함께 대조해 fail-closed 최소 correction을 계획한다.
3. fixture layout은 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/` 경계, opt-in scene FFI는 기존 C11.4 fixture 경계를 사용한다. Rust fixture adapter를 제품 runtime 코드에 섞지 않고 파일 책임을 기존 300–500줄 규칙에 맞게 유지한다.
4. 구현 후 새로운 구현/runtime 실패 관점 20개를 검토한다. 계획 검토를 재사용하지 않는다. 결함은 수정하고 영향받은 검토를 다시 수행한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 실제 실행해 frame, used track, trace, draw/present를 확인한다. 필요한 물리 픽셀 주장은 캡처 readback이 성공한 표본에만 제한한다.
6. 기본/opt-in artifact 경계를 확인하고 관련 focused tests, format, Clippy, platform build를 실행한다. 실행하지 않은 항목은 통과로 표시하지 않는다.
7. 구현 PR의 diff는 계획 검토와 별도로 최소 30개의 서로 다른 실패 관점으로 적대 검토한다. PR 본문에 재현된 동작, 수정, 플랫폼 근거와 남은 범위를 적고 required checks가 녹색일 때 Rebase and merge한다.

## 계획 근거

- [Chrome 156 observation·입력·화면·hash](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/README.md)
- [계획 자체 적대 검토 20개](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/plan-review-20.md)
- [계획 PR diff 적대 검토 30개](../spec/internal/evidence/c11-4-nested-grid-intrinsic-precomparison-2026-10-21/plan-pr-diff-review-30.md)
