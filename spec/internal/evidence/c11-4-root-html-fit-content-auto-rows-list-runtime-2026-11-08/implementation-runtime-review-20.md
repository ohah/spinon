# C11.4 Root HTML fit-content auto-row 목록 · 구현/runtime 검토 20개

구현 코드와 focused runtime 근거에서 서로 다른 실패 경로 20개를 확인했다. 계획 검토 20개와 이후 PR diff 검토와는 별도다.

1. **두 값 순환** — 두 track pattern은 explicit 10px row 뒤 implicit row를 A/B/A/B로 만들어야 한다. Layout은 `[20, 34, 20, 6]px`를 만들었다.
2. **세 값 순환** — 세 번째 값 뒤에만 첫 값으로 돌아와야 한다. Layout 결과는 `[10, 20, 50, 10]px`다.
3. **첫 pattern index** — explicit 10px row가 목록 첫 값을 소비하면 안 된다. item y 좌표는 10px에서 시작해 목록 순서로 증가한다.
4. **혼합 fit-content cap** — `fit-content(20%) 12px fit-content(80%)`에서 fixed 중간 track과 양쪽 cap을 유지한다. Layout 결과는 `[20, 12, 50, 6]px`다.
5. **direct percentage identity** — 8px payload가 있는 10% track은 definite 100px block basis에 따라 10px이어야 하며 payload 높이로 축소하면 안 된다. mixed-size cycle 결과는 `[20, 12, 10, 20]px`다.
6. **projection의 typed 순서** — projection과 RuntimeSession computed 값이 `FitContent(20%)`, `LengthPx(12)`, `Percentage(10%)` 순서를 유지하고 `fit-content(20%) 12px 10%`로 직렬화한다.
7. **cap보다 큰 minimum contribution** — 첫 implicit item의 `min-height:28px`가 20% cap track을 28px로 올려야 한다. Layout과 runtime 모두 item frame 28px 및 보이는 34px nested overflow를 유지했다.
8. **floor shape 경계** — 네 implicit row topology에서 28px minimum을 거부하고, 제한된 3-item shape의 첫 row만 허용한다. Layout rejection assertion이 통과했다.
9. **zero cap** — 0% cap은 Grid item frame을 0 높이로 두고 34px child overflow를 보존한다. Layout과 RuntimeSession assertion이 통과했다.
10. **100% 초과 cap** — 120% cap을 임의로 100%로 clamp하지 않고, max-content 34px가 실제 row를 제한한다. Layout과 runtime assertion이 통과했다.
11. **track 목록 상한** — `GRID_MAX_TRACKS`보다 긴 목록은 track을 만들기 전에 거부한다. Layout과 projection 모두 상한 + 1개를 거부한다.
12. **지원 밖 typed 항목** — `auto`, `fr`, intrinsic keyword, `minmax()`가 섞이면 값 전체를 거부한다. Projection negative matrix가 각 종류를 확인한다.
13. **음수·비유한 값** — 음수, NaN, 무한 fit-content cap 및 음수 direct percentage가 Layout에 들어가면 안 된다. Layout과 projection negative test가 통과했다.
14. **계산 AST 경계** — percentage가 든 `Calc` cap은 거부한다. 기존 singleton runtime regression이 길이 상수 calc의 `LengthPx` 정규화를 유지한다.
15. **definite percentage basis** — 높이가 indefinite면 viewport나 surface 크기를 대신 쓰지 않는다. 기존 percentage fit-content Layout/runtime test가 거부한다.
16. **기존 singleton probe** — singleton cap, minimum floor, 다음 6px probe, px 정규화 회귀가 유지된다. 기존 singleton Layout/runtime tests가 통과했다.
17. **placement mode 분리** — column flow, explicit item placement, areas, nonzero gap은 이 목록 profile에서 허용하면 안 된다. 기존 Root HTML rejection/recovery tests가 대조한다.
18. **DOM owner·자손 경계** — style validator는 visible Grid owner 하나, 직접 HTML `div` item 및 auto-sized item당 fixed-px payload 하나만 받는다. nested Grid, 추가 자손, replaced item과 지원 밖 text를 닫힌다.
19. **자식 수 경계** — 기존 2-row/singleton 3-item shape와 4-row 5-item shape만 받는다. `3 | 5` guard가 다른 자식 수를 publication 전에 거부한다.
20. **실패 원자성과 복구** — dark media의 미지원 목록 항목에서 완료 layout/render snapshot을 내보내지 않고, 같은 RuntimeSession이 지원 값으로 돌아온 뒤 복구한다. runtime assertion이 복구 frame을 확인한다.

## Focused 검증

- `cargo fmt --all -- --check`: 통과.
- `cargo test --locked -p spinon-layout root_html_fit_content`: 8개 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_fit_content`: 3개 통과.
- `cargo test --locked -p spinon-runtime parsed_root_html_fit_content`: 6개 통과.
- `cargo clippy --locked -p spinon-runtime -p spinon-layout -p spinon-style-to-layout --all-targets -- -D warnings`: 통과.
- 현재 source의 최종 출력은 [Layout](./test-logs/layout.log), [projection](./test-logs/style-to-layout.log), [RuntimeSession](./test-logs/runtime.log), [Clippy](./test-logs/clippy.log)에 보존했다. `cargo fmt --all -- --check`와 `git diff --check`도 통과했다.

넓은 탐색 실행 `cargo test --locked -p spinon-style-to-layout root_html_grid`은 **통과하지 않았다**. 66개 통과, 변경하지 않은 C11.5 grid-item alignment 및 percentage-gap 경로 5개가 `UnsupportedGridContainerStyle { property: "C11.5 fixed-track auto-size/stretch contract" }`로 실패했다. 이를 fit-content 근거 또는 전체 crate 통과로 계산하지 않는다.

## 시뮬레이터 실행

- Android API 37 `sdk_gphone64_arm64` AVD에서 현재 source 후보를 앱 기본 Root HTML route로 실행했다. 새 PID `14892`의 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT`는 `status=pass`, `layout=ready`, `boxes=12`를 보고했다. [화면](./simulator/android/screenshot.png) · [boot log](./simulator/android/current-source-run.log) · [device/build evidence](./simulator/android/device-and-run.txt).
- iPhone 17 Pro/iOS 26.2 Simulator (`ACA7BF91-D144-4DA8-B1BF-7F6B34007F8B`)도 `status=pass`, `layout=ready`, `boxes=12`를 보고했다. [화면](./simulator/ios/screenshot.png) · [boot log](./simulator/ios/root-html-boot.log) · [device/build evidence](./simulator/ios/device-and-run.txt).
- 두 화면은 visual smoke다. PixelCopy/Metal surface readback 및 compositor 이후 pixel parity를 주장하지 않는다. Geometry와 track 값은 focused Layout/RuntimeSession tests가 판정한다.
- 첫 Android 시도는 Gradle의 `prepareSpinonBootstrap`이 `examples/bootstrap`에서 bundle을 다시 만들어 이전 HTML을 패키징했다. 그 화면과 log를 [asset mismatch attempt](./simulator/android/attempt-01-asset-mismatch/)에 보존했다. 다음 시도에서는 fixture HTML은 패키징됐지만 앱 DOM의 `document.getElementById` 미지원으로 root script가 실패했다. 로그와 화면은 [DOM API mismatch attempt](./simulator/android/attempt-02-dom-api-mismatch/)에 보존했다. `document.body`와 지원되는 `setAttribute`를 사용해 최종 실행을 통과시켰다.
- 시험 중 바꾼 `examples/bootstrap` 원본, `build/spinon/bootstrap` 산출물, Android APK와 iOS app bundle은 기록된 SHA-256으로 복원했다. Android AVD는 기존 설치 APK로 복구해 실행 중 상태로 되돌렸고, iOS target에는 설치 전 app bundle을 다시 설치했다. `/tmp`의 복원 backup 위치는 `simulator/backup-location.txt`에 기록했다.

PR diff 30개 검토와 PR 생성은 남아 있다. 전체 C11.4/C11 완료 또는 public CSS Grid 지원을 주장하지 않는다.
