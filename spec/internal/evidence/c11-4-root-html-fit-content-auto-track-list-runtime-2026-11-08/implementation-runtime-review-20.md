# C11.4 parsed Root HTML `fit-content()` + `auto` row list · 구현/runtime 실패 관점 검토 20개

계획 검토와 PR diff 검토에서 분리해, 이번 `auto` 목록 항목 허용의 코드 경계와 실행 경로를 20개 실패 관점으로 확인했다. 지원 범위는 기본 parsed Root HTML의 단일 고정 Grid profile이며 `grid-auto-rows` 목록에 finite non-negative `fit-content(px|%)`가 하나 이상 있고, 나머지는 `auto`·finite non-negative px·definite percentage인 경우뿐이다.

1. **다른 placement profile로 `auto` 누출** — layout의 `validate_tracks`에는 `RootHtmlFitContentAutoRowsV1`일 때만 `allow_auto`를 켠다. projection도 fit-content 목록 predicate가 통과할 때만 이 placement mode를 고른다. 일반 CSS/Grid profile로 `auto` 목록 항목을 넓히지 않는다. 코드 경계 검토.
2. **fit-content 없는 `auto 8px`** — `auto`만으로 이 profile을 선택하지 않는다. projection과 Layout negative assertion에서 `grid-auto-rows` 거부를 확인했다.
3. **빈 목록** — 빈 목록은 projection의 non-empty predicate와 Layout validator에서 거부한다. 두 경로 모두 부분 결과를 만들지 않는다.
4. **목록 quota 초과** — `GRID_MAX_TRACKS + 1` typed 항목은 projection에서 `grid-auto-rows` 오류로 거부한다. 긴 목록을 생성해 시뮬레이션하기 전에 차단한다.
5. **혼합 목록의 `min-content`** — fit-content와 auto 사이에 intrinsic keyword가 섞인 Layout 및 projection 입력을 거부한다.
6. **혼합 목록의 `max-content`** — 지원 목록의 허용 타입 match에서 제외되며 projection negative matrix가 확인한다.
7. **혼합 목록의 `fr`** — finite fraction이어도 이 C11.4 profile에서는 명시 거부한다. projection과 Layout negative matrix가 확인한다.
8. **혼합 목록의 `minmax()`** — `minmax(0, 1fr)` typed 값은 direct px/%/auto/fit-content 외 값이라 projection에서 거부한다.
9. **잘못된 fixed-px 값** — 음수와 NaN 길이는 projection에서 거부한다. `supports_track_list`도 finite non-negative 조건만 허용한다.
10. **잘못된 direct percentage 값** — 음수와 infinity percentage를 거부한다. 0%는 finite non-negative 입력이므로 유지한다.
11. **잘못된 fit-content 한도** — 음수·NaN·양/음 infinity 한도를 거부한다. 0%, 120% 한도는 유한 non-negative 값이라 그대로 인정한다(120%를 100%로 임의 제한하지 않는다).
12. **계산식 cap** — typed `fit-content(calc(...))`는 허용된 px/% direct cap이 아니므로 fail-closed 한다. dark media revision의 `fit-content(calc(12px + 12%))` RuntimeSession negative 경로도 `grid-auto-rows`로 실패한다.
13. **indefinite percentage basis** — profile은 40×100 CSS px definite content box로 제한된다. percentage cap은 100px block basis를 사용하며 viewport 크기를 대체 basis로 끌어오지 않는다. projection의 bounded-shape 검사와 세 RuntimeSession 장면이 이 조건을 고정한다.
14. **명시 row가 목록을 먼저 소비** — 첫 explicit row는 10px이며 첫 implicit row부터 pattern index 0을 사용한다. 각 positive 장면에서 첫 implicit item y=10px로 확인했다.
15. **목록 순환 경계** — `fit-content(20%) auto 8px` 세 항목 뒤 네 번째 implicit row는 첫 항목으로 돌아간다. Layout과 RuntimeSession의 다섯 row frame이 `[10, 20, 30, 8, 20]`을 확인한다.
16. **`auto` 위치를 바꾼 순서 회귀** — `auto fit-content(20%) 8px`은 `[10, 30, 20, 8, 25]`를 만든다. Auto의 intrinsic 기여를 pattern 위치와 혼동하지 않고, typed projection order도 유지한다.
17. **zero cap에서 양수 row 생성** — `fit-content(0%) auto 8px`의 첫 implicit row와 반복되는 네 번째 row는 0px이어야 한다. Layout·RuntimeSession이 `[10, 0, 30, 8, 0]`을 확인한다.
18. **row 크기와 자손 overflow 혼동** — 0px row 안의 34px nested payload는 34px frame과 같은 y 위치를 유지한다. 행 크기에 맞춰 자손 frame을 잘라내거나 높이를 0으로 바꾸지 않는다. fixture는 visible overflow를 고정한다.
19. **typed 값·computed serialization 순서** — RuntimeSession에서 typed 목록이 `FitContent(20%), Auto, LengthPx(8)` 순서이며 computed string이 `fit-content(20%) auto 8px`인지 확인했다. 10개 Grid/item/payload frame의 고유 identity와 좌표도 함께 비교했다.
20. **실패 원자성과 회복** — 같은 세션에서 dark media의 미지원 calc revision은 owner node와 `grid-auto-rows`를 보고하고 completed layout/render snapshot을 내지 않는다. light revision으로 돌아온 뒤 같은 RuntimeSession instance가 다시 `Ready`가 되고 payload frame을 확인했다.

## Focused 검증

- `cargo test --locked -p spinon-layout root_html_fit_content_auto_row_track_lists -- --nocapture`: 2/2 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_fit_content_auto_row_lists_preserve_supported_track_kinds_and_order -- --nocapture`: 1/1 통과.
- `cargo test --locked -p spinon-runtime parsed_root_html_fit_content_auto_track_lists -- --nocapture`: 2/2 통과.
- `cargo test --locked -p spinon-runtime parsed_root_html_fit_content_auto_rows -- --nocapture`: 기존 singleton 회귀 2/2 통과.
- 변경 crate 3개 all-target Clippy `-D warnings`와 `cargo fmt --all -- --check` 통과. 원본 로그는 [`test-logs/`](./test-logs/)에 보존했다.

## Android·iOS Simulator

- Android API 37 ARM64 `sdk_gphone64_arm64` AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 앱 기본 Root HTML route를 실행했다. 두 로그 모두 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready`, `boxes=12`다.
- 양 앱 산출물 안 `index.html` SHA-256은 fixture `c206e7aef5b82129dce5b4f79599da7a58a4dc1d6fe5343bc49c03552e37300d`와 일치했다. Android 설치 APK와 빌드 APK hash도 일치했고, iOS 설치 executable은 빌드 executable과 일치했다.
- [Android screenshot](./simulator/android/screenshot.png) · [Android boot log](./simulator/android/current-source-run.log) · [iOS screenshot](./simulator/ios/screenshot.png) · [iOS boot log](./simulator/ios/root-html-boot.log).
- 캡처는 작은 고정 Grid가 화면 좌상단에 그려진 visual smoke다. PixelCopy/Metal drawable readback이나 compositor 이후 pixel parity를 뜻하지 않는다. 정확한 track·frame 판정은 위 Rust Layout/RuntimeSession assertion이 담당한다.
- 시험 전 bootstrap, APK, iOS build app과 simulator 설치본을 보존했다. 시험 뒤 bootstrap 원본 hash, Android APK/설치본 hash, iOS build app 및 설치 app hash를 원래 보관본과 대조해 복원했다. Android 앱은 실행 전처럼 전면 실행 상태로 돌아왔고, iOS target은 실행 전처럼 앱을 종료한 상태로 두었다. 저장 데이터는 읽거나 변경하지 않았다. 보관본 경로는 [`simulator/backup-location.txt`](./simulator/backup-location.txt), 복원 기록은 [`simulator/run-manifest.txt`](./simulator/run-manifest.txt)다.

이 검토는 C11.4 전체 구현, 공개 CSS Grid 지원, 실기기 GPU, 화면 pixel parity 또는 merge 완료 판정이 아니다. C11.4와 C11 공식 체크는 미완료다.
