# C13.1 Android AVD border paint · 2026-10-14

## Oracle와 사전 조건

고정 Chrome 155 oracle은 [기존 agent-browser fixture observation](../c13-1-overflow-clip-paint-hit-border-2026-10-14/oracle-fixture-observation.json)과 [픽셀 목록](../c13-1-overflow-clip-paint-hit-border-2026-10-14/screenshot-pixels.txt)이다. 최초 AVD 실행의 실패와 각 채널 pass 조건은 [precomparison](precomparison.md)에 고정했다.

Android API 37 ARM64 AVD `emulator-5554`의 CSS viewport는 400×160이고 실제 SurfaceView는 1050×420이다. 실행 보고는 WGPU `backend=Gl`, ANGLE/SwiftShader device를 기록한다. USB나 실기기는 연결하지 않았다. 현재 기기 목록과 AVD 속성은 [환경 기록](android-avd-environment.txt)에 있다.

## 실패 원인과 수정

최초 [AVD PixelCopy](android-border-pixelcopy.png)는 Chrome 색상 표본 3/6만 맞았다. 부모 배경색이 border 영역에 남은 까닭은 runtime scene이 background만 전달하고 border를 그리지 않았기 때문이다. positioned child는 기존 paint order가 부모 뒤에 두어 부모 배경에 가려졌다. hit query 6/6과 frame geometry 5/5는 이 paint 실패와 별개로 이미 통과했다.

수정은 공통 runtime/WGPU 경계에 적용했다.

- runtime box가 면별 border 폭과 단색 paint를 보유하고, C13.1 overflow profile이 Stylo computed border color를 전달한다. currentColor는 computed `color`로 풀이한다.
- 현재 renderer 범위인 불투명 단색 `solid` 이외의 비영(非零) border style/color는 조용히 solid로 바꾸지 않고 cascade에서 거부한다.
- auto `z-index`의 absolute/fixed overflow-clip 부모 아래에 정적 in-flow subtree만 있으면 painter 순서에서 그 부모를 subtree 앞에 둔다. hit order는 별도 유지한다.
- WGPU는 부모 배경 뒤에 border side quads를 그리고, 각 quad에 ancestor clip을 적용한다.

## 최종 검증

- [최종 Android 앱 빌드](android-border-rebuild-final.log): C04 runtime GPU와 C13.1 fixture-test opt-in으로 `BUILD SUCCESSFUL`.
- [최종 AVD runtime/PixelCopy log](android-border-runtime-final.log): 5/5 frame, 6/6 hit query, 6/6 RGBA 표본이 Chrome 기대값과 exact match다. 정적/positioned 내부는 `#57a6d9`, 두 border strip은 `#f0c674`, 외부는 `#101827`이다.
- [PixelCopy PNG](android-border-pixelcopy-final.png)는 1050×420이고 SHA-256은 `d5a970cf2dc62b6d79351c026ccdadb3e673247acabe80e617271ab136499578`이다.
- [기존 C13.1 Android route 재검증](android-existing-overflow-route-final.log)은 13개 node frame, 15/15 hit query, 11/11 PixelCopy 색 표본을 통과했다.
- [route 충돌 negative control](android-border-route-conflict-final.log)은 overflow route와 border route를 함께 지정했을 때 `SPINON_C131_ARGUMENT_ERROR`로 거부한다. border host 생성 로그는 없다.
- [Workspace 테스트](workspace-tests.log)는 736 passed, 0 failed, 4 ignored다. 변경된 WGPU crate 테스트 18/18, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`, [Android JNI/Java 경계](android-fixture-boundary-test.log) 5/5가 통과했다.

앱은 두 route 검증 뒤 `force-stop`했다. AVD는 booted 상태이며 현재 `adb devices -l`에는 emulator 하나만 나온다.

## 한계

이 확인은 고정 static/absolute fixture의 불투명 단색 solid border와 정적 in-flow 자식 paint, 내부 hit query, Android SurfaceView PixelCopy에 한정된다. dashed/dotted/double 또는 반투명 border는 이 runtime profile에서 거부한다. 둥근 모서리와 서로 다른 면 색상 경계, 더 복잡한 stacking subtree, DOM event dispatch, scanout, 실제 GPU/실기기 동작은 증명하지 않는다. C13.1 전체 matrix는 여전히 미완료다.

## 적대 검토

[구현 경로 20개](implementation-review-20.md)와 [PR 변경 30개](pr-delta-review-30.md)는 이번 Android delta를 기존 iOS 검토와 분리해 확인했다.
