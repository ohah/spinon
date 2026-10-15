# C13.3 ADB 검증 실패 원인과 수정 · 2026-10-16

## 확인된 실패

이전 API 37 AVD 재실행에서 ADB touchpad 입력은 앱까지 도달했고 `TrackpadScroll`로 분류된 뒤, 위치 정보가 없어 `point-outside-surface`로 올바르게 거부됐다. 검증 runner는 거부 로그의 event source 진단도 확인하도록 되어 있었지만, 해당 reject branch는 `event_source_bits`와 pointer tool을 출력하지 않아 assertion이 실패했다. 이는 ADB 전달 실패나 wheel queue 오작동이 아니라 reject 진단 필드 누락이었다.

당시 보존된 pre-fix ADB 출력은 [기존 touchpad negative log](../c13-3-native-wheel-input-2026-10-16/android-touchpad-missing-position.log)에서 확인할 수 있다. 거부 reason은 있으나 `event_source_bits`, `pointer_tool`, `device_supported_source_bits`가 없다. runner의 원시 stderr 자체는 보존본에 포함되지 않아 여기서는 재구성한 assertion 조건만 기록한다.

## 별도로 수정한 제품 분류 결함

코드 대조에서 `InputDevice.getSources()`의 지원 source 합집합을 현재 event 분류에 사용하던 문제도 확인했다. mouse와 touchpad를 함께 지원하는 composite device의 mouse event는 합집합에 touchpad가 있다는 이유만으로 `TrackpadScroll`이 될 수 있다. 분류는 `MotionEvent` event source와 pointer 0 tool type으로 옮겼고, 장치 합집합은 진단에만 남겼다. JVM legacy negative control은 기존 규칙의 잘못된 결과와 새 규칙의 `MouseWheel` 결과를 비교한다. 실제 AVD에는 composite input device가 없으므로 해당 하드웨어 조합은 직접 재현하지 않았다.

## 수정 후 실행

새 구현은 성공 로그와 모든 reject 경로에 `event_source_bits`, pointer tool, device-supported source bits를 남긴다. 수정 후 API 37 ARM64 AVD에서 mouse 방향 8/8, outside/touchpad-without-position/zero-axis 거부 3/3이 통과했다. 새 [touchpad log](./android-touchpad-missing-position.log)는 `0x100008`, `finger`, `0x301`과 예상 reject를 보존한다. [전체 matrix와 artifact 경계](./README.md).
