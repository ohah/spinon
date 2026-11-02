# C11.1 parsed Root HTML percentage `grid-auto-rows` runtime 근거

이 제한 slice는 [구현 계획](../../../../plan/c11-root-html-grid-auto-row-percentage-list.md)의 직접 px/percentage implicit row 목록을 기본 parsed Root HTML route에 연결한다. 구현 전 Chrome 156 observation과 독립 수치 oracle은 [사전 비교](../c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md)에 보존했다.

## 구현 범위

- 기본 `RuntimeRootHtmlTextPaintV1` Root HTML profile에서만 direct finite non-negative px/percentage `grid-auto-rows` scalar/list를 허용한다. 기존 px scalar 표현, 단위·순서, 첫 implicit row부터의 목록 순환을 유지한다.
- percentage 목록은 definite finite non-negative CSS-pixel height를 basis로 사용한다. `height:auto`, min/max height, non-auto aspect ratio, math-backed height, unsupported percentage gap은 거부한다. definite `0px`는 유효하며, 100%보다 큰 track은 clamp하지 않는다.
- 명시 rows 및 기존 px/fr/percentage explicit track 조합과 row-sparse placement를 유지한다. percentage implicit column, intrinsic/calc/fr implicit item, named-area 결합, dense/column flow와 general CSS admission은 포함하지 않는다.
- CSS typed cascade, projection, Root HTML validator, `GridContainerInput`, Taffy layout, frame publication 전후의 오류 경계를 검사한다. 각 percentage와 확장된 row/gap 합의 f32 범위를 확인하고 allocator failpoint 뒤 같은 worker의 유효 요청이 회복되는지 확인한다.

## 수치·Rust 검증

- `cargo test --locked -p spinon-style c11_1_root_html_auto_row_list`: 7 passed. px 회귀, percentage 단독/혼합 목록, syntax-invalid negative 선언 cascade fallback, 지원하지 않는 값, 할당 실패 원자성·회복을 포함한다.
- `cargo test --locked -p spinon-layout root_html_`: 14 passed. 첫 implicit row 순환, explicit percentage/fr 조합, 0px basis, overflow, 제약/개별·누적 f32 범위, resource cap을 포함한다.
- `cargo test --locked -p spinon-style-to-layout root_html_`: 26 passed. profile 경계, invalid/unsupported 값, named-area 배제와 parsed Root HTML→Taffy frame을 포함한다. 별도 `parsed_root_html_` 필터는 4 passed이며 위 26개 안에 포함된다.
- `cargo clippy --locked -p spinon-style -p spinon-style-to-layout -p spinon-layout --all-targets -- -D warnings`: 통과.
- `cargo fmt --all -- --check` 및 `git diff --check`: 최종 diff에서 확인했다.
- 위 Rust/Clippy/서식 검증의 재실행 원본 출력은 [`rust/`](./rust/)에 저장했다. parsed-root integration 4/4는 `root_html_` 26/26 결과의 일부이며 중복해서 더하지 않는다.
- 공유용 Rust log에서는 EOF의 빈 줄 하나만 제거했다. byte 원본은 iOS build 로그와 함께 로컬 ignored artifact의 `raw-logs/` 아래에 보존했다.

## Android API 37 ARM64 AVD

- AVD `spinon_api37_0_applied_txn`, serial `emulator-5556`, API 37 / Android 17 / `arm64-v8a`; 모델 문자열은 `sdk_gphone64_arm64`다.
- [빌드 로그](./android-build-pass03.log)는 `BUILD SUCCESSFUL`이다. Root HTML runtime GPU 외 C04/fixture-test feature toggles는 빌드 스크립트에서 0으로 고정했다.
- [실행 로그](./android-runtime-pass03.log)는 `root_app_marker=ready`, `layout=ready boxes=13`, `presented boxes=13`을 기록한다. root frame은 `411.42856×100 CSS px`, document/render-tree revision은 2, environment revision은 0이다. renderer는 Android Emulator ANGLE/Vulkan/SwiftShader software path다.
- [화면 캡처](./android-api37-avd-pass03.png)는 1080×2400이며 SHA-256은 `bf0145b407e3f7d3d11108a5a41d0954e8760c3c3a5bc4dff3833ab6454f80d6`이다.

## iOS Simulator

- iPhone 17 Pro, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- [빌드 로그](./ios-simulator-build-pass03.log)는 `BUILD SUCCEEDED`다. 기존 duplicate-symbol/dSYM 및 AppIntents metadata warning이 로그에 남아 있다.
- [실행 로그](./ios-runtime-pass03.log)는 `root_app_marker=ready`, `layout=ready boxes=13`, `presented boxes=13`을 기록한다. root frame은 `402×100 CSS px`, document/render-tree revision은 2, environment revision은 0이다.
- [화면 캡처](./ios-26_2-simulator-pass03.png)는 1206×2622이며 SHA-256은 `59bae0aace7ed08f6cb251b9d62fedaf26b551d64e1346c69cce58c7cdeab2de`다.

## 입력·설치 artifact·복구 식별

- 단일-grid parsed Root HTML 입력 [HTML](./root-html-grid-auto-row-percentage-list-runtime.html)의 SHA-256은 `7556e1203e22579a87fad7ea570f0521483d036c950f6afc9279c0e4b33a2d94`다. V8 document 경로와 semantic ID·class·역순 source order를 확인하는 [probe](./root-app-runtime-probe.js)의 SHA-256은 `7caadb6d0457b240af0357ea9aea2923595363b8c442ea606ba5ac21fee2709b`다.
- APK SHA-256 `b8958ca48a08f4b12f19fbb90b0af37a9a10ae721236eb6f130f8e60d8b92e05`; 설치 후 AVD의 `base.apk`를 pull해 확인한 hash도 같다. iOS executable SHA-256 `3d02f8992d290901456ce81e8f045f3695493f8e8465fda75e8b301b38942e0d`; 설치된 simulator bundle의 executable hash도 같다.
- source base, Rust patch, 추가 test source, HTML·probe, 설치 artifact와 screenshot의 hash는 [source manifest](./source-revision.json)에 기계 판독 가능한 형식으로 기록했다.
- iOS build log의 공유 가능한 저장본에서는 줄 끝 공백과 EOF의 빈 줄만 정리했다. byte 원본은 로컬 ignored `build/spinon/artifacts/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/raw-logs/`에 보존했다. 로그 내용과 build 결과는 변경하지 않았다.
- 두 플랫폼 앱 프로세스는 실행 확인 후 종료했다. 임시로 교체한 bootstrap 파일은 저장된 backup과 byte-for-byte 동일하게 복원했다: source `index.html` `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `root-app.js` `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`; build bootstrap `index.html` `f2d0d35e88ce89d64ebaac5d37365ad9621735f54f6399e90a1959a7412ccd5d`, `root-app.js` `c3e817bc8ddbeab5734e353136b94894c0e04d41d0d51ec01242c798e8b88ce5`, `app.js` `a0254f51eebd28df0c25c77ab5bd3e53e0a91ec55de0df05e85a01164d643f4c`.
- [재현 빌드 스크립트](./build-platform-apps.sh)와 [AVD/iOS 실행 스크립트](./run-simulators.sh)는 `PASS=passNN`으로 이름을 분리해 이전 실행을 덮어쓰지 않는다. 최초 pass01은 harness가 기대한 box 수를 11로 잘못 적었고 앱은 `boxes=13`으로 정상 실행했다. pass02·pass03에서 harness 기대치를 13으로 바로잡았으며 pass03은 최종 코드의 새 artifact다.

## 한계와 상태

화면 캡처와 draw/present는 simulator visual smoke evidence다. Android PixelCopy, iOS Metal drawable readback 또는 post-compositor pixel 비교를 실행하지 않았다. Android AVD는 SwiftShader이며 실기기·hardware GPU 결과가 아니다. WPT, 모든 CSS Grid 조합, 공개 CSS API 범위 및 C11 전체는 검증하지 않았다. 이 limited slice만으로 C11.1/C11 체크박스나 공식 완료 `72/213`을 바꾸지 않는다.
