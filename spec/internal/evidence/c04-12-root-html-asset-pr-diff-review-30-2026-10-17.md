# C04.12 bundle HTML asset fixture PR diff 적대 검토

- **검토일:** 2026-10-17
- **범위:** feature gating·Android/iOS build integration·asset packaging·문서·로드맵 diff
- **검토 기준:** 20개 구현 검토는 V8 runtime assertion을 살폈다. 이 30개는 그 검토와 겹치지 않게 build/product boundary, 문서 링크와 상태 반영을 확인했다.
- **결과:** 차단 결함 없음. default app/artifact 경계를 확인했고 C04.12의 미완료 상태와 `61/197` count를 유지했다.

| # | 별도 PR diff 실패 경로 | 확인 및 판정 |
| --- | --- | --- |
| 1 | fixture source module이 기본 `spinon-ffi` feature에 추가되어 production compile에 포함 | Cargo fixture feature는 opt-in이며 기본 feature 목록에 추가되지 않았다. |
| 2 | `lib.rs`가 feature를 켜지 않아도 fixture module을 컴파일 | fixture module 선언은 전용 `c04-12-root-html-fixture-test` cfg 안에 있다. |
| 3 | C header declaration guard와 Rust export gate가 서로 다름 | 헤더 macro와 Cargo feature guard가 같은 fixture-test boundary를 표현한다. |
| 4 | Android Gradle flag가 임의 값에서 빌드를 허용 | Gradle 설정은 `0` 또는 `1`만 허용한다. |
| 5 | Android JNI C define은 0인데 BuildConfig만 true | Gradle이 동일 환경값에서 C define과 BuildConfig 값을 만든다. |
| 6 | Android fixture adapter가 일반 main source set에 남음 | adapter는 C04.12 fixture-test source set에 추가된다. feature-off APK도 빌드됐다. |
| 7 | build-android.sh가 Gradle feature만 켜고 Rust fixture ABI는 생략 | Android native build가 같은 환경 flag로 Cargo feature 배열과 C++ define을 설정한다. |
| 8 | feature-on Android APK에 source asset이 빠지거나 다른 경로로 패키징 | API 37 feature-on APK의 `assets/c04-12-root-html-asset.html` 422 bytes를 확인했다. |
| 9 | feature-off Android build가 이전 bootstrap asset 사본을 남김 | build script는 비활성 상태에서 bootstrap asset을 삭제하며 default APK에 파일이 없다. |
| 10 | iOS simulator wrapper가 native build로 feature flag를 넘기지 않음 | `build-ios-sim.sh`가 같은 C04.12 flag를 `build-ios-native.sh`로 전달한다. |
| 11 | iOS native Rust build가 Swift macro와 다른 feature set 사용 | feature-on iOS build 출력은 `ffi=c04-12-root-html-fixture-test`를 보고했고 빌드가 성공했다. |
| 12 | Objective-C++ declaration guard가 Cargo feature와 다름 | iOS C++ define은 simulator build flag와 함께 적용된다. |
| 13 | Swift conditional compilation suffix와 Xcode 설정 불일치 | app controller/helper는 project의 `_1` Swift define 아래에만 컴파일되고 feature-off build가 성공했다. |
| 14 | iOS asset copy script가 기본 리소스 디렉터리를 무시 | 기존 bootstrap resource copy 단계가 `build/spinon/bootstrap/`의 고정 asset을 app bundle에 복사한다. |
| 15 | feature-off iOS bundle에 이전 HTML resource가 남음 | copy script 비활성 경로가 resource를 삭제하며 feature-off `.app`에서 파일이 없었다. |
| 16 | Android와 iOS가 동일한 이름 대신 다른 fixture를 포함 | 둘 다 `c04-12-root-html-asset.html` basename과 같은 source SHA-256을 사용한다. |
| 17 | 고정 test fixture가 production example/app source에 들어가 기본 앱 내용 변경 | fixture는 `tests/fixtures/html/`에 있고 build flag 없이는 native bootstrap asset에 복사하지 않는다. |
| 18 | HTML feature branch가 기존 `app.js` build output을 대체 | Android default와 opt-in APK 모두 기존 `assets/app.js`를 포함한다. |
| 19 | default app constructor가 `new_with_root_html`로 바뀌어 JS-only 동작을 암묵 변경 | 이번 diff는 platform fixture route와 별도 FFI fixture만 추가하며 기본 session constructor 호출부는 바꾸지 않는다. |
| 20 | 기존 document accessor fixture가 새 asset fixture에 의해 제거됨 | 기존 직접 전달 route/adapter와 새 `runAsset` method가 별도로 유지된다. |
| 21 | iOS runner Objective-C API가 feature-off import에서 항상 선언됨 | 두 C04.12 runner method 선언은 fixture-test macro 안에 있다. |
| 22 | Android/iOS route 상태 화면이 기본 app launch에서 자동 표시 | 각 route는 전용 intent/argv 값이 있을 때만 선택된다. |
| 23 | Android reflection class가 feature-off package에 없는데 기본 route에서 강제 참조 | BuildConfig false branch는 adapter reflection 전에 unavailable 상태로 돌아간다. |
| 24 | PR 변경이 HTML metadata 효과를 제품 지원 계약으로 문서화 | 계약 0077은 bundle 전달이 opt-in test route라고 밝히고 production loader 미지원 범위를 남긴다. |
| 25 | 계획 문서가 원래 계획에 있던 기능처럼 서술 | C04.12 계획 상단과 본문은 viewport/color-scheme 앱 처리가 기존 계획에 없었던 신규 범위라고 명시한다. |
| 26 | STATUS 항목을 fixture PASS 뒤 완료로 체크하거나 완료 count 변경 | C04.12는 미완료 체크 상태로 유지되고 새 fixture는 `spec/STATUS.md`에 제한 증거로만 기록된다. |
| 27 | 내부 계약 인덱스에서 새 fixture와 남은 기능을 찾을 수 없음 | 내부 README row가 precomparison, 두 구현 검토, simulator evidence, 미완료 범위를 링크한다. |
| 28 | 실행 README가 캡처를 실제 HTML render 화면으로 오인하게 함 | README는 두 simulator 이미지를 fixture 전용 status 화면이라고 명시하고 viewport/Canvas/paint 검증과 분리한다. |
| 29 | 수정 후 이미지/fixture가 이전 해시 manifest에 남음 | `SHA256SUMS`를 새 bundle fixture와 두 asset route 캡처로 갱신했고 manifest 전체 검증이 통과했다. |
| 30 | 공개/Tailnet roadmap에서 새 기능 진행을 완료로 표시하거나 총계를 증가 | `roadmap.html`은 신규 C04.12가 원래 계획에 없었다고 설명하고, opt-in asset 전달만 확인된 상태와 미구현 viewport/color-scheme을 적으며 공식 `61/197` count를 유지한다. |

## 재검증

- Android feature-off/feature-on APK build 성공; feature-off에는 HTML fixture asset이 없고 feature-on에는 source hash와 같은 422-byte asset이 포함됐다.
- iOS feature-off/feature-on Simulator build 성공; feature-off app bundle에는 asset이 없고 feature-on bundle에는 동일한 422-byte asset이 포함됐다.
- Default host release staticlib에는 C04.12 fixture symbol name/assertion string이 없고 opt-in positive control에는 포함됨을 `strings -a`로 확인했다.
- Rust runtime 8 tests, FFI 5 tests, targeted Clippy, rustfmt, build-script syntax, checksum manifest, `git diff --check` 통과.
- Android API 37 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 실제 V8 asset fixture PASS.

검증 경계는 기존과 같다. 기본 앱의 `index.html` loader, viewport/page scale, OS preference/media-query 연동, UA Canvas와 HTML rendering은 포함하지 않는다. 전체 CSS 지원 및 C04.12 완료 상태로 승격하지 않는다.
