# C13.1 전체 물리 표면 RGBA 비교 계획 · 2026-10-18

## 목표

C13.1의 13-case overflow fixture에서 현재 확인한 30,000 CSS pixel-center 표본을 전체 drawable/PixelCopy texel 비교로 확장한다. Android API 37 ARM64 AVD의 `788×263` PixelCopy와 iPhone 17 Pro / iOS 26.2 Simulator의 `900×300` Metal drawable을 각각 같은 크기의 고정 Chromium surface와 픽셀 좌표 그대로 비교한다. 표면의 모든 RGBA byte가 일치해야 한다.

이 follow-up은 renderer paint·clip 결과와 readback/comparison 경로를 검증한다. CSS 지원 범위를 늘리거나 C13.1/C13 완료를 선언하지 않는다. AVD software renderer, iOS Simulator의 compositor 전 drawable, 고정 fixture에서만 결과를 주장한다.

## 기준, fixture, 관찰값

- **고정 입력:** 기존 [C13.1 compact 13-case fixture](../tests/fixtures/css/c13/overflow-c13-1.html), Chrome 155 observation 및 DPR 1 표면 원본을 재사용한다. fixture source, observation JSON, inventory와 기존 renderer scene identity의 SHA-256을 run 전에 고정한다.
- **Chromium oracle:** Chrome for Testing `155.0.8059.40`을 사용하고 Agent Browser로 fixture를 연다. 논리 viewport는 `300×100` CSS px, scroll `(0,0)`, color scheme·zoom 기본값이다. Android oracle은 device scale factor `788/300`으로 캡처해 PNG가 `788×263`이어야 한다. iOS oracle은 device scale factor `3`으로 캡처해 PNG가 `900×300`이어야 한다. browser-reported viewport·DPR, screenshot color profile와 실제 PNG dimensions가 기대값과 다르면 해당 run은 무효다.
- **Android surface:** `emulator-5554`만 허용한다. `PixelCopy`가 복사한 현재 WGPU SurfaceView 전체 `788×263` RGBA 이미지가 비교 대상이다. 전체 앱 screenshot은 장식/상태 표시줄이 들어 있으므로 표면 비교 입력으로 사용하지 않는다.
- **iOS surface:** 고정 iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 현재 WGPU drawable 전체 `900×300`을 읽는다. 기존 opt-in surface readback 경계를 재사용해 row-major 전체 texel 좌표를 64개씩 요청하고 각 결과에 render key·surface generation·drawable 크기·`Bgra8UnormSrgb`를 연결한다. 앱은 swizzle된 RGBA byte를 순서대로 packed `.rgba` artifact에 쓴다. 1×1 copy가 사용하는 256-byte padded readback row에서는 앞 4 byte만 취하고 padding을 내보내지 않는다. 이전 run·이전 drawable·compositor 화면 캡처는 비교 대상으로 받지 않는다.
- **비교:** 같은 크기의 PNG/RGBA 배열에서 `(x,y)`를 재표본화하지 않고 `0≤x<width`, `0≤y<height`의 모든 texel과 네 채널을 직접 비교한다. 허용 오차·경계 band·색상 보정은 없다. 산출물에는 양쪽 파일 hash, dimensions, pixel/channel 수, mismatch 전체 좌표와 expected/actual RGBA를 기록한다.
- **고정 관찰:** Android는 `788×263 = 207,244` texel / `828,976` channel, iOS는 `900×300 = 270,000` texel / `1,080,000` channel이다. 각 플랫폼에서 mismatch 0, 최대 channel 오차 0이어야 통과한다. 기존 30,000 CSS 중심 exact 결과는 독립 회귀 증거로 유지한다.

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
2. Agent Browser에서 Chrome fixture의 viewport/DPR/source hash를 확인하고 고정 Chromium PNG 두 장을 만든다. Agent Browser 세션은 검증 후 종료하고 inactive/pid null을 확인한다. terminal-browser는 사용하지 않는다.
3. Rust/Node PNG comparator의 전체 texel equality와 위 mutation controls를 먼저 구현·검증한다. iOS packed artifact는 정확히 `900×300×4` byte이고 64-point batch 경계를 넘어 sample이 재정렬·중복·누락되지 않아야 한다.
4. Android API 37 ARM64 AVD에서 최신 source app을 빌드해 PixelCopy 전체 RGBA를 추출하고 Chrome `788×263` oracle와 비교한다. 새 앱 process와 surface generation으로 독립 3회 실행하며, 세 번 모두 exact해야 한다. device serial, surface size, APK/native hash, logs, PNG와 comparison JSON을 보존한다.
5. iOS Simulator에서 최신 source app을 빌드해 Metal drawable 전체 RGBA를 추출하고 Chrome `900×300` oracle와 비교한다. 새 앱 process와 drawable generation으로 독립 3회 실행하며, 세 번 모두 exact해야 한다. UDID, process/build identity, render key, generation, format, logs, PNG와 comparison JSON을 보존한다.
6. Chrome CSS-center 30,000-point, Android PixelCopy 52-point/hit/frame matrix와 iOS frame/hit matrix를 회귀 실행한다. 수정이 필요하면 실패를 재현한 뒤 renderer/readback 원인을 고치고 전체 물리 표면을 재실행한다. 기대치나 허용 오차를 결과를 본 뒤 낮추지 않는다.
7. 실제 코드와 캡처 결과를 대상으로 계획과 구별되는 구현 실패 관점 20개를 검토한다. 이후 이 변경 PR 자체는 프로젝트 규칙에 맞춰 서로 다른 30개 실패 관점으로 검토한다.
8. 상태 대장과 내부 계약에는 관찰된 Android AVD/iOS Simulator 범위만 기록한다. C13.1 및 C13 완료 표시, WPT 적합성, 모든 CSS 조합, 실기기·hardware GPU, iOS compositor 이후 픽셀로 확대하지 않는다.
9. Rebase and merge 뒤 현재 정적 Tailnet 미리보기를 백업 포함 copy-only 절차로 갱신하고 SHA-256/HTTP를 확인한다. Agent Browser에서 roadmap의 새 evidence 링크와 PR 이미지 표시를 확인하고 세션을 종료한다. Pages workflow는 실행하지 않는다.

## 명시적 제외

실기기, hardware GPU, 다른 density/orientation, 일반 HTML/CSS 조합, 전체 WPT, DOM event/CSSOM scroll, product artifact, iOS OS compositor 이후 화면과 scanout은 이 follow-up에서 검증하지 않는다. 고정 Chrome 155 oracle와 두 Simulator/AVD surface 결과만 다룬다.
