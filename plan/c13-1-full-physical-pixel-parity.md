# C13.1 전체 물리 표면 RGBA 비교 계획 · 2026-10-18

## 목표

C13.1의 13-case overflow fixture에서 현재 확인한 30,000 CSS pixel-center 표본을 전체 drawable/PixelCopy texel 비교로 확장한다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 모두 CSS viewport `300×100`, DPR 3, 물리 표면 `900×300`으로 맞춘다. AVD는 검증 중에만 Android density를 480 dpi로 임시 설정해 앱 scale을 3으로 만들며, 실행 후 기존 density를 복구한다. 양 플랫폼 표면을 같은 크기의 고정 Chromium surface와 픽셀 좌표 그대로 비교하고 모든 RGBA byte 일치를 요구한다.

이 follow-up은 renderer paint·clip 결과와 readback/comparison 경로를 검증한다. CSS 지원 범위를 늘리거나 C13.1/C13 완료를 선언하지 않는다. AVD software renderer, iOS Simulator의 compositor 전 drawable, 고정 fixture에서만 결과를 주장한다.

## 기준, fixture, 관찰값

- **고정 입력:** 기존 [C13.1 compact 13-case fixture wrapper](../tests/fixtures/css/c13/c13-1-compact-surface-matrix.html), wrapper가 읽는 [공유 DOM/CSS source](../tests/fixtures/css/c13/c13-1-compact-surface-matrix.js), [Agent Browser evaluator](../tools/css-reference/c13-1-compact-surface-matrix-agent-eval.js), Chrome 155 geometry observation 및 DPR 1 표면 원본을 재사용한다. source SHA-256은 각각 `431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355`, `d36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760`, `5622623ce7d0f7c29452382b873ed1e12cc5ee2dc159e07c7ff61f48ee42b45b`다. 기존 geometry observation SHA-256은 `515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299`다. HTML wrapper는 상대 JS를 읽으므로 loopback HTTP origin에서 열고, 캡처 전 13 case·41 node와 Chrome 버전을 확인한다.
- **Chromium oracle:** Google Chrome `155.0.8059.85`(binary SHA-256 `6e156f2271cef44afce9db36a5b920e94518857b94a4160f082696f8c3bd1c9d`)을 Agent Browser `0.31.1`로 실행한다. `Emulation.setDeviceMetricsOverride`로 device scale factor 3을 지정하고 live Agent Browser tab에서 CDP `Page.captureScreenshot`을 호출한다. Agent Browser의 일반 screenshot 명령은 CSS 크기 PNG를 내므로 고DPI oracle로 사용하지 않는다. headless Chrome의 `--hide-scrollbars`를 켜 스크롤바 구현 차이가 surface oracle에 들어오지 않게 한다. 논리 viewport는 `300×100` CSS px, scroll `(0,0)`, color scheme·zoom 기본값이다. 두 플랫폼 공통 oracle PNG는 `900×300`이어야 한다. browser-reported viewport·DPR, screenshot color profile와 실제 PNG dimensions가 기대값과 다르면 해당 run은 무효다.
- **Android surface:** `emulator-5554`만 허용한다. 실행기는 기존 `wm density` 값을 보존하고 density를 480 dpi로 임시 변경해 app scale 3을 만든 뒤, 성공·실패 모두에서 앱을 force-stop하고 이전 density override 또는 물리 density로 복구한다. `PixelCopy`가 복사한 현재 WGPU SurfaceView 전체 `900×300` RGBA 이미지가 비교 대상이다. 물리 display size는 변경하지 않는다. 전체 앱 screenshot은 장식/상태 표시줄이 들어 있으므로 표면 비교 입력으로 사용하지 않는다.
- **iOS surface:** 고정 iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 현재 WGPU drawable 전체 `900×300`을 읽는다. 기존 opt-in surface readback 경계를 재사용해 row-major 전체 texel 좌표를 64개씩 요청하고 각 결과에 render key·surface generation·drawable 크기·`Bgra8UnormSrgb`를 연결한다. 앱은 swizzle된 RGBA byte를 순서대로 packed `.rgba` artifact에 쓴다. 1×1 copy가 사용하는 256-byte padded readback row에서는 앞 4 byte만 취하고 padding을 내보내지 않는다. 이전 run·이전 drawable·compositor 화면 캡처는 비교 대상으로 받지 않는다.
- **비교:** 같은 크기의 PNG/RGBA 배열에서 `(x,y)`를 재표본화하지 않고 `0≤x<width`, `0≤y<height`의 모든 texel과 네 채널을 직접 비교한다. 허용 오차·경계 band·색상 보정은 없다. 산출물에는 양쪽 파일 hash, dimensions, pixel/channel 수, mismatch 전체 좌표와 expected/actual RGBA를 기록한다.
- **고정 관찰:** 양 플랫폼은 `900×300 = 270,000` texel / `1,080,000` channel이다. 각 플랫폼에서 mismatch 0, 최대 channel 오차 0이어야 통과한다. 기존 30,000 CSS 중심 exact 결과는 독립 회귀 증거로 유지한다.

## 사전 실패 경계

1. Capture manifest는 run ID, 앱 process ID, app/library SHA-256, fixture hash, render/document/style revision, drawable generation·크기·format, packed byte length를 연결한다. 필드 누락·중복·불일치·오래된 generation은 비교 전에 실패한다.
2. High-DPR Chrome PNG는 실제 캡처된 DPR로 크기를 판정한다. 스케일링된 DPR1 screenshot, CSS center upsample, 기존 PixelCopy를 reference로 되돌려 쓰지 않는다.
3. Android PixelCopy 대상은 surface crop 하나로 고정한다. 전체 화면에서 색을 추정해 crop하거나 `SurfaceView` 외의 UI를 포함하지 않는다.
4. Metal surface sample은 기존 64-point request 상한을 유지한다. 각 1×1 readback의 256-byte padded buffer row에서 RGBA 네 byte만 추출하고 padding byte를 artifact에 섞지 않는다. 요청 좌표는 row-major 연속 전체 surface여야 한다.
5. BGRA→RGBA swizzle은 한 번만 수행한다. sRGB 변환·premultiply/unpremultiply·alpha 보정은 하지 않는다. 형식/alpha 계약이 예상과 다르면 readback을 거부한다.
6. 모든 물리 texel을 비교하고 mismatch 좌표는 상한 없이 JSON에서 보존한다. 콘솔 요약은 제한해 대형 결과가 잘려 원본 mismatch를 잃지 않게 한다.
7. Negative controls는 각 RGBA channel의 1-byte mutation, 누락·중복·추가 texel, width/height 바꿈, 256-byte readback padding 혼입, 순서 뒤바뀜, run/generation/hash 불일치를 각각 거부한다.
8. Capture가 실패하거나 비동기 mapping/copy 완료 전에 surface가 바뀌면 성공 summary를 만들지 않는다. 다음 새 process/generation의 정상 capture는 복구되어야 한다.
9. Readback FFI·고정 fixture payload·새 JS route는 기존 C13.1 opt-in fixture feature 안에 둔다. 기본 Android/iOS artifact에서 전용 symbol·string·payload 부재와 opt-in positive control 존재를 검사한다.

## 실행 관문

1. 구현 전에 이 계획을 20개의 서로 다른 실패 관점으로 검토하고 oracle·pass criteria를 고정한다.
2. Loopback HTTP origin의 compact matrix wrapper를 Agent Browser로 열고 Chrome version/revision, 13 case·41 node, viewport/DPR/source hashes를 확인한 뒤 고정 Chromium DPR 3 PNG를 만든다. 일반 overflow reference page는 oracle로 사용하지 않는다. Agent Browser 세션은 검증 후 종료하고 프로세스 종료를 확인한다. terminal-browser는 사용하지 않는다.
3. Rust/Node PNG comparator의 전체 texel equality와 위 mutation controls를 먼저 구현·검증한다. iOS packed artifact는 정확히 `900×300×4` byte이고 64-point batch 경계를 넘어 sample이 재정렬·중복·누락되지 않아야 한다.
4. Android API 37 ARM64 AVD에서 최신 source app을 빌드하고 density 480 dpi 아래에서 app scale 3·`900×300` surface를 확인한다. PixelCopy 전체 RGBA를 추출해 Chrome `900×300` oracle와 비교한다. 새 앱 process와 surface generation으로 독립 3회 실행하며, 세 번 모두 exact해야 한다. 매 run 뒤 원래 density를 복구하고 복구값을 확인한다. device serial, density before/during/after, surface size, APK/native hash, logs, PNG와 comparison JSON을 보존한다.
5. iOS Simulator에서 최신 source app을 빌드해 Metal drawable 전체 RGBA를 추출하고 Chrome `900×300` oracle와 비교한다. 새 앱 process와 drawable generation으로 독립 3회 실행하며, 세 번 모두 exact해야 한다. UDID, process/build identity, render key, generation, format, logs, PNG와 comparison JSON을 보존한다.
6. Chrome CSS-center 30,000-point, Android PixelCopy 52-point/hit/frame matrix와 iOS frame/hit matrix를 회귀 실행한다. 수정이 필요하면 실패를 재현한 뒤 renderer/readback 원인을 고치고 전체 물리 표면을 재실행한다. 기대치나 허용 오차를 결과를 본 뒤 낮추지 않는다.
7. 실제 코드와 캡처 결과를 대상으로 계획과 구별되는 구현 실패 관점 20개를 검토한다. 이후 이 변경 PR 자체는 프로젝트 규칙에 맞춰 서로 다른 30개 실패 관점으로 검토한다.
8. 상태 대장과 내부 계약에는 관찰된 Android AVD/iOS Simulator 범위만 기록한다. C13.1 및 C13 완료 표시, WPT 적합성, 모든 CSS 조합, 실기기·hardware GPU, iOS compositor 이후 픽셀로 확대하지 않는다.
9. Rebase and merge 뒤 현재 정적 Tailnet 미리보기를 백업 포함 copy-only 절차로 갱신하고 SHA-256/HTTP를 확인한다. Agent Browser에서 roadmap의 새 evidence 링크와 PR 이미지 표시를 확인하고 세션을 종료한다. Pages workflow는 실행하지 않는다.

## 명시적 제외

실기기, hardware GPU, 다른 density/orientation, 일반 HTML/CSS 조합, 전체 WPT, DOM event/CSSOM scroll, product artifact, iOS OS compositor 이후 화면과 scanout은 이 follow-up에서 검증하지 않는다. 고정 Chrome 155 oracle와 두 Simulator/AVD surface 결과만 다룬다.
