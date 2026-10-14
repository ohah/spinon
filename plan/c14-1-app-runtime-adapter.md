# C14.1 · Android/iOS 앱 runtime fixture adapter 계획

## 목표

Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 앱에서 같은 고정 fixture를 호출한다. fixture-owned HostDocument와 PNG 요청을 디코드하고 `ImageResourceSnapshot`을 C14.1 CSS px intrinsic snapshot으로 바꿔 기존 HostDocument → Stylo → Taffy 경로로 전달한다. 앱 화면·로그에는 Chrome oracle와 비교한 계산 결과를 보여준다.

이 경로는 live V8 session의 HostDocument를 읽지 않는다. UI에 PNG를 그리거나 WGPU/Metal image paint를 증명하지 않는다. 제품 resource loader·비동기 교체·image paint·stale scheduling은 범위 밖이다. 내부 계약 버전 `0.1.0`과 C14.1/C14 미완료 상태를 유지한다.

## 범위와 경계

- Cargo feature, Android/iOS 빌드 플래그와 C ABI 선언은 공통 이름 `c14-1-app-fixture-test` / `SPINON_ENABLE_C14_1_APP_FIXTURE_TEST`를 사용한다. 기본값은 0이고 C04 GPU feature에 의존하지 않는다. fixture FFI symbol, CSS/PNG payload와 화면 route는 opt-in 산출물에만 포함한다.
- 두 앱은 전용 `--spinon-c141-intrinsic-size` 실행 인자에서만 고정 `c14-1-png-basic` 입력을 호출한다. 다른 fixture route와 동시에 전달되면 거부한다. feature-off 실행은 disabled/error 결과를 표시하고 성공으로 기록하지 않는다.
- Rust adapter는 매 실행마다 synthetic `HostDocument`를 만들고 HTML `img`의 `src`에서 `ImageRequest`를 생성한다. PNG decode → `ImageResourceSnapshot` → layout intrinsic snapshot → `compute_runtime_style_layout_with_intrinsic_size_snapshot`의 기존 C14.1 경로를 사용한다. 보고값에는 source/document/intrinsic revision, NodeId, natural size와 frame이 포함된다.
- 각 앱은 native 계산을 UI thread 밖에서 실행한다. Android JNI 또는 iOS ObjC++/Swift 다리의 반환 상태·UTF-8·출력 용량 오류를 확인하고, 결과 문자열만 UI thread에서 표시한다.
- 앱 캡처는 계산 report 화면의 동작 증거다. CSS renderer의 실제 이미지 픽셀·GPU surface·paint 완료 증거로 해석하지 않는다. PR에는 Android와 iOS Simulator 화면을 각각 첨부하고 증거 디렉터리에 원본 PNG·로그·환경을 보존한다.
- overflow/border paint는 C14.1의 이 adapter 범위에 넣지 않는다. C13.1에는 Android PixelCopy의 6/6 border 표본 근거와 iOS Simulator의 geometry/hit-query/Metal surface 근거가 별도로 있다. iOS Metal pixel readback은 확인하지 않았으며 C13.1 전체 gate도 미완료다.
- `spec/STATUS.md`에서 C14.1·C14는 미완료로 유지한다. 이번 앱 경로는 기존 Chrome 155 fixture oracle에 대한 정적 intrinsic sizing 입력 관찰만 추가한다.

## 비교 기준과 재현 입력

- 독립 oracle은 [C14.1 사전 비교 근거](../spec/internal/evidence/c14-1-replaced-intrinsic-inputs-precomparison-2026-10-14.md)의 agent-browser Chrome `155.0.8059.40` 관측이다. 고정 입력은 [replaced-auto-size.html](../tests/fixtures/css/c14/replaced-auto-size.html), [PNG](../tests/fixtures/resources/c14-1-png-basic.png), [U05.1 reference](../tests/fixtures/resources/c14-1-png-basic-reference.json)다. 기존 [C14 fixture 무결성 검사](../tools/css-reference/c14-replaced-auto-size.test.mjs)로 입력/reference 연결을 검사한다.
- 대상 관찰은 decoded natural size `24×16 CSS px`, 두 축 `auto`인 `img` frame `(0,0) 24×16 CSS px`다. x/y/width/height를 oracle과 각각 절대 오차 `0.5 CSS px` 이내로 비교하고 node identity 및 input/output intrinsic revision 보존을 확인한다.
- 계산 viewport는 Chrome oracle과 동일한 `120×80 CSS px`다. AVD density, iOS screen scale, safe area, surface 크기는 CSS layout 입력에 곱하지 않는다. DPR은 이번 비교의 pass 조건이 아니다.
- Android 실행은 명시한 `emulator-*` serial로만 한다. iOS 실행은 명시한 booted Simulator UDID로만 한다. USB/실기기를 선택하지 않고, 두 환경의 OS·API·ABI/기기·앱 build flag·실행 인자를 기록한다.

## 사전 판정과 검증 순서

1. 공통 계획을 구현 전에 20개의 서로 다른 실패 관점으로 검토한다. Android JNI와 iOS ObjC++/Swift 연결, 두 simulator의 route 및 screenshot을 독립 경계로 취급한다.
2. Cargo feature와 C header declaration/definition, Android Gradle `BuildConfig`·JNI, iOS Xcode preprocessor define·Swift route가 하나의 opt-in flag에 묶이는지 확인한다. 기본 artifact에서 symbol과 fixture payload가 빠지고 opt-in artifact에만 들어가는지 검사한다.
3. 두 앱의 route-off, 잘못된 flag, route 충돌, output buffer 부족, invalid UTF-8/error report를 실패로 표시하는지 확인한다. JNI/native 계산은 main/UI thread가 아닌 곳에서 수행한다.
4. 기존 U05.1 decoder/reference 입력을 재사용해 FFI가 생성한 report를 Chrome oracle과 node별 비교한다. 별도 unit test 성공만으로 Android/iOS app 경로 성공을 대체하지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 앱을 실행한다. 로그의 success marker, 24×16 CSS px와 `(0,0)` frame, revision identity를 검사한다. 어느 한 쪽의 성공을 다른 플랫폼 결과로 대신하지 않는다.
6. 양쪽 앱 report 화면을 캡처하고 원본 PNG 및 실행 log와 함께 보존한다. PR 본문에 두 화면을 포함하고 agent-browser에서 실제 렌더링된 이미지와 링크를 확인한다.
7. 계획과 다른 20개 실패 관점으로 구현을 검토한다. 이후 최신 프로젝트 규칙에 따라 PR 통합 변경은 별도 30개 관점으로 검토한다.

## 성공 조건과 남은 범위

- Android와 iOS 앱이 같은 C ABI fixture를 실행하고 oracle 일치 결과를 화면·로그에 표시한다.
- 기본 app/library에는 fixture symbol과 PNG/CSS payload가 없으며, opt-in 앱에서만 route와 호출 코드가 활성화된다.
- 두 Simulator 캡처와 환경 정보가 evidence에 보존되고 PR 본문에서 보인다. 실기기, hardware GPU, image paint/pixel parity, 비동기 resource 교체/re-layout, product runtime의 C14.1 자동 연결은 완료로 표시하지 않는다.
- C13.1 overflow/border 검증은 위에 연결한 별도 evidence의 범위와 한계를 유지하며, C14.1 report 화면으로 overflow paint를 재검증했다고 주장하지 않는다.
- C14.1 및 상위 C14/U05/C15/X16은 미완료로 둔다.
