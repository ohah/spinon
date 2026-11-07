# C11.4 parsed Root HTML `fit-content(px)` implicit row · 2026-11-07

## 판정

기본 parsed Root HTML route의 `grid-auto-rows:fit-content(px)` 제한 구현을 Chrome oracle, Rust layout/runtime, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 확인했다. PR diff 검토 중 새 route가 product layout trace를 켜던 값을 발견해 제거했다. 최종 pass 4는 그 수정이 포함된 production source로 Android/iOS를 다시 빌드하고 실행했다. Android `emulator-5554`와 새로 만든 빈 iPhone 17 Pro Simulator 모두 `page_scale=1`, `layout=ready`, `boxes=7`을 기록했다. pass 4 screenshot은 화면 구성의 visual smoke다. PixelCopy, Metal drawable readback, iOS compositor 이후 pixel equality를 측정하지 않았다.

C11.4/C11은 미완료다. 공식 완료 `78/214`를 유지한다. 이 candidate는 일반 CSS admission, 전체 Grid 적합성, 실기기 또는 hardware GPU 검증을 주장하지 않는다.

## 실행 행렬

| 실행 | 변경 | Android AVD | iOS Simulator | 판정 |
| --- | --- | --- | --- | --- |
| pass 1 | viewport meta 없음 | viewport width 980 CSS px, `page_scale≈0.4198` | viewport width 980 CSS px, `page_scale≈0.4102` | fixture 입력이 모바일 viewport를 좁히지 않음을 확인하고 원본 화면·로그 보존 |
| pass 2 | viewport meta 추가 | `page_scale=1`, layout ready, 7 boxes | `page_scale=1`, layout ready, 7 boxes | viewport mapping smoke; pass 2 APK/app 사본 보존 |
| pass 3 | pass 2 이후 Rust lint 경계 수정 포함 source로 재빌드 | `page_scale=1`, layout ready, 7 boxes | `page_scale=1`, layout ready, 7 boxes | 당시 source의 실행 근거; build artifact는 `pass03-build-copy/`에 보존 |
| pass 4 (최종) | product route에서 불필요한 track trace 제거 후 새 source로 재빌드 | `page_scale=1`, layout ready, 7 boxes | `page_scale=1`, layout ready, 7 boxes | 현재 source와 APK/iOS executable hash는 `final-pass04-source-revision.json`에 연결 |

| 대상 | 환경 | 최종 화면 |
| --- | --- | --- |
| Android | `emulator-5554`, Android 17 / API 37, ARM64 AVD | [pass 4 screenshot](android/final-pass04/screenshot.png), 1080×2400 |
| iOS | 새 `Spinon-C114-FitContent-Pass04`, iPhone 17 Pro / iOS 26.2 Simulator | [pass 4 screenshot](ios/final-pass04/screenshot.png), 1206×2622 |

## 최종 source와 artifact identity

`final-pass04-source-revision.json`은 build에 사용한 Git base HEAD, 변경된 product/test source의 SHA-256, fixture hash, APK/iOS executable hash와 screenshot hash를 고정한다. Build 당시 변경은 commit 전 working tree였으므로 HEAD만으로 source를 식별하지 않고 파일 hash 목록을 함께 기록한다. pass 3의 APK와 app bundle은 [별도 사본](pass03-build-copy/artifacts.sha256)으로 남겼다.

| 내용 | SHA-256 |
| --- | --- |
| Fixture 및 Android APK/iOS bundle `index.html` | `6a1394ff4bb6ccead6d369a3b2f6a6ecf23a26c654e6402fa055be032336fb79` |
| Android pass 4 APK | `b33193f6207c3586b619b3c84c66048f62b51ae52f549814c8e9c532350a4b91` |
| Android pass 4 ARM64 `libspinon_bootstrap.so` | `d158d179baea9bddefb45b19220ffea055b860a2330485d0b40bafb8761ba715` |
| iOS pass 4 Simulator executable | `90f94313895d5e68a46e4fecbbd61cb3c5e016235e0643dc362d6e581da9d104` |
| Android pass 4 screenshot | `ce39542395082bae24b13c83e184a50a53f8a35ff6854bc7191785781d46bc91` |
| iOS pass 4 screenshot | `90b20c65b05d6331bd8e6776608d2ec2672940281a1b30ef8c6abf73aaf64211` |

Build/install logs, screen captures, device metadata and bootstrap source restoration hashes remain alongside this document. Pass 1–4 captures are preserved. Exact pass 3 APK/app-bundle copies remain in the local workspace with their checksum manifest; the large binaries are not part of the PR. Pass 4 manifests are `final-pass04-built-artifacts.sha256` and `final-pass04-source-revision.json`.

## Rust 검증

- `cargo test --locked -p spinon-layout root_html_fit_content_implicit_row -- --nocapture` — 2 passed; latest run is in `final-pass04-checks/layout.log`.
- `cargo test --locked -p spinon-runtime parsed_root_html_fit_content_auto_rows -- --nocapture` — 2 passed; latest run is in `final-pass04-checks/runtime.log`.
- `cargo test --locked -p spinon-style-to-layout root_html_explicit_fit_content -- --nocapture` — 2 passed; existing explicit `fit-content` regression is in `final-pass04-checks/style-regression.log`.
- `cargo fmt --all -- --check`, workspace all-target Clippy, and `git diff --check` — passed; logs are in `final-pass04-checks/`.

추가 `bool` argument가 layout projection 함수의 Clippy 인자 수 한계를 넘긴 것을 발견해 `RootHtmlAutoRowsSupport` enum으로 지원 모드를 묶었다. 경고를 suppress하지 않았고 최종 workspace Clippy를 다시 통과했다.

## Simulator app-data 경계

실행 전 baseline은 iOS data container를 다음 경로로 기록했다.

`.../Data/Application/E1FB9C90-1EAB-4017-B83B-BB4E711C53A8`

이전 pass 3 뒤 `simctl get_app_container`가 반환한 경로는 다음과 같고 당시 크기는 9.6 MB였다.

`.../Data/Application/C69E53F9-47E7-4049-8112-30AB9EA64349`

baseline 경로는 더 이상 존재하지 않는다. 저장된 설치 스크립트는 `simctl install`만 호출하며 `simctl uninstall`, `simctl erase` 또는 데이터 clear 명령은 호출하지 않았다. 하지만 container identity가 달라졌으므로 앱 data의 연속 보존을 확인할 수 없고, 경로가 바뀐 정확한 시점과 원인은 실행 기록만으로 특정되지 않는다. baseline에는 설치 app bundle을 복사했지만 data container 내용을 백업하지 않았다. 현재 Simulator data를 지우거나 복원하려는 추가 조치는 하지 않았다. [읽기 전용 경로 확인 기록](ios/final-pass03/data-container-inspection.txt).

Pass 4 iOS 확인은 새 빈 Simulator `97E99AA8-D144-4DA8-B1BF-7F6B34007F8B`에서 실행했다. 기존 iPhone 17 Pro Simulator의 app data는 수정하지 않았다. 새 Simulator의 생성된 data container 경로는 `ios/final-pass04/data-container.txt`에 있다. 이전 pass 3의 container identity 차이는 해결된 것으로 간주하지 않는다.

Android pass 4는 기존 `emulator-5554`에서 `adb install -r`를 사용했으며 `pm clear` 또는 uninstall은 실행하지 않았다. Android app-data contents는 읽거나 비교하지 않았으므로 content continuity를 보증하지 않는다. pass 4 설치·실행 로그는 `android/final-pass04/`에 있다. 새로 만든 빈 Android AVD는 bounded startup 중 ADB device를 노출하지 않아 사용하지 않았고, 그곳에는 app을 설치하지 않았다.

## 범위와 남은 검증

- Cap 24px/40px, item minimum이 cap보다 큰 경우, zero cap, length-only constant `calc()`, px 환산 단위, percentage 및 percentage-bearing `Calc` AST 거부, wrong flow/explicit placement, nonzero gap, intrinsic column rejection과 same-session recovery는 focused Rust tests에서 확인했다.
- 최종 pass 4는 현재 source hash로 빌드된 APK/iOS executable을 기본 app route에서 실행하고 같은 fixture hash, `page_scale=1`, layout status 및 7-box count를 확인한다. Screenshot은 visual smoke 한정이다.
- 계획 검토 20개와 구현/runtime 검토 20개를 완료했다. 구현/runtime 검토의 별도 문서를 참조한다.
- PR diff 실패 관점 30개는 [`pr-diff-review-30.md`](pr-diff-review-30.md)에 완료 기록했다. Feature PR, GitHub checks, merge 상태는 아직 남아 있다.
