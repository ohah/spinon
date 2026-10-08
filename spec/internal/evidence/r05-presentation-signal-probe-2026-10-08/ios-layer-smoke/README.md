# iOS 시뮬레이터 WGPU drawable 획득 smoke harness

이 격리된 Xcode 대상은 저장소의 `platforms/ios/Sources/R08GpuDemo.swift`를 그대로 컴파일하고 실제 `spinon-wgpu-r08-spike` iOS 시뮬레이터 정적 라이브러리를 연결한다. 로컬에 고정 V8 checkout이 없을 때 layer hook을 확인할 수 있도록 V8 앱 래퍼만 제외한다. `SpinonRunner.mm`은 기존 R08 WGPU C API 호출만 전달한다.

저장소 루트에서 시뮬레이터 UI test를 실행한다.

```sh
xcodebuild test \
  -project spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/R05LayerSmoke.xcodeproj \
  -scheme R05LayerSmoke \
  -destination 'platform=iOS Simulator,name=iPhone 17 Pro,OS=26.2' \
  -derivedDataPath build/spinon/r05-ios-layer-smoke \
  CODE_SIGNING_ALLOWED=NO
```

iOS 기기용 API 분기도 시뮬레이터에서 실행하지 않고 compile/link만 확인한다.

```sh
mise exec -- cargo build --manifest-path spikes/wgpu-backend/Cargo.toml --locked --release --target aarch64-apple-ios
xcodebuild \
  -project spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/R05LayerSmoke.xcodeproj \
  -scheme R05LayerSmoke -sdk iphoneos -destination 'generic/platform=iOS' \
  -derivedDataPath build/spinon/r05-ios-layer-smoke-device \
  CODE_SIGNING_ALLOWED=NO build
```

실행 판정은 `--r05-probe`로 실행했을 때 연결된 WGPU renderer가 그리는 동안 `R05ProbeMetalLayer.nextDrawable()`에서 남기는 `SPINON_R05_DRAWABLE_ACQUIRE` 로그다. XCTest는 중앙 GPU 도형을 탭하고 화면 카운터와 로그 ticket의 입력 sequence·revision·draw sequence를 확인한다. 인자 없이 실행한 일반 R08 WGPU 경로는 probe 비활성 대조다. 이 결과는 시뮬레이터의 실제 WGPU surface 경로와 synthetic UI 입력의 acquire 귀속을 검증한다. 물리 touch, iOS 기기 callback runtime, V8을 포함한 전체 앱 bundle은 검증하지 않는다.

`runtime.png`는 기본 화면이고 `tap-after.png`는 XCTest 탭 뒤 화면이다. 실행 로그와 빌드 결과도 이 디렉터리에 보존한다. 실행 당시 destination은 iPhone 17 Pro / iOS 26.2 Simulator였다. `SHA256SUMS`는 재현 근거의 파일 무결성을 확인하는 manifest다.
