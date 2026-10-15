# C13.3 Android native wheel source 분류 수정 계획 검토 · 20개 관점 · 2026-10-16

**검토 범위:** 병합된 #198 Android wheel adapter의 event/device source 구분 보완. **공식 상태:** C13.3 미완료 유지. **기준:** Android [`MotionEvent.getSource()`](https://developer.android.com/reference/android/view/MotionEvent#getSource()) 및 [`getToolType(int)`](https://developer.android.com/reference/android/view/MotionEvent#getToolType(int)) API 계약, 내부 계약 0063, API 37 ARM64 AVD와 JVM classifier oracle.

| # | 독립 관점 | 실패 경로와 판정 |
| ---: | --- | --- |
| 1 | 이벤트와 장치 능력 구분 | `InputDevice.getSources()` 합집합은 현재 event가 아닐 수 있다. event source/tool을 판정 입력으로 고정하고 합집합은 로그 전용으로 둔다. **계획 반영** |
| 2 | 일반 touchpad source | Android 문서는 touchpad event가 보통 `SOURCE_MOUSE`를 반환한다고 명시한다. mouse source만으로 mouse 판정하면 안 되며 tool 검사 단계가 필요하다. **계획 반영** |
| 3 | captured touchpad | captured event는 `SOURCE_TOUCHPAD`로 올 수 있다. 명시 source를 직접 trackpad로 판정하는 분기를 둔다. **계획 반영** |
| 4 | mouse-equivalent finger tool | `SOURCE_MOUSE + TOOL_TYPE_FINGER`를 trackpad scroll로 분류한다. **계획 반영** |
| 5 | 실제 mouse tool | `SOURCE_MOUSE + TOOL_TYPE_MOUSE`만 mouse wheel로 분류한다. **계획 반영** |
| 6 | 알 수 없는 source | 다른 source는 mouse/trackpad 어느 쪽으로도 추정하지 않고 queue 전에 거부한다. **계획 반영** |
| 7 | 알 수 없는 tool | mouse source라도 finger/mouse가 아니면 fail-closed한다. **계획 반영** |
| 8 | pointer 부재·index 오류 | pointer 0 접근 전에 pointer count를 확인하고, count가 0이면 명시 `SOURCE_TOUCHPAD`여도 classifier가 unknown으로 거부한다. API 문서상 count는 1 이상이지만 방어적 경계를 둔다. **계획 반영** |
| 9 | SDK 호환성 | `getSource`/`isFromSource`와 `getToolType`는 minSdk 29보다 오래된 API다. 신규 런타임 API level gate가 필요하지 않다. **확인** |
| 10 | 복합 장치 | mouse와 touchpad 기능을 함께 광고하는 장치의 mouse event가 legacy union 경로에서 오분류될 수 있다. 해당 조합을 negative control로 고정한다. **계획 반영** |
| 11 | AVD의 한계 | 현재 AVD에는 composite hardware가 없다. JVM negative control과 실제 AVD 입력을 분리해 보고하고 하드웨어 재현이라고 부르지 않는다. **한계 명시** |
| 12 | ADB 장치 식별 | ADB 입력은 `deviceId=-1`인 가상 입력이 될 수 있다. 실기기/peripheral 근거로 확대하지 않고 serial을 `emulator-*`로 제한한다. **계획 반영** |
| 13 | source 진단 보존 | 성공 및 모든 reject 로그에 event bits, pointer tool, 지원 source union을 남겨 source 판정과 장치 능력을 사후 대조한다. **계획 반영** |
| 14 | hit-test 거부 | 위치 없는 touchpad 샘플은 분류 결과와 별개로 point 검사에서 거부한다. 해당 reject 로그에도 source 진단을 포함한다. **계획 반영** |
| 15 | 무효 축 | zero/non-finite axis는 source 분류 이후 queue 전에 거부하고 revision/offset 불변을 본다. **기존 계약 유지** |
| 16 | Control-wheel | 새 분류기가 modifier 정책을 우회하지 않도록 기존 Control reject를 유지하고 diagnostic source만 추가한다. **기존 계약 유지** |
| 17 | pinch/magnification | source/tool 분류를 pinch/zoom 지원으로 확대하지 않고 고정된 입력 범위 밖으로 둔다. **계획 반영** |
| 18 | fixture ABI 경계 | helper 자체는 앱 callback 코드일 수 있지만 C13.3 listener/FFI route는 opt-in이며 off/on artifact negative/positive control을 비교한다. **계획 반영** |
| 19 | 제품 계약과 완료 수 | 공개 `WheelEvent`/DOM event나 명세 버전은 바꾸지 않는다. iOS/native gates가 열려 있으므로 공식 완료 체크를 올리지 않는다. **계획 반영** |
| 20 | iOS와 표시 완료 게이트 | iOS callback 관찰 및 S04.10 displayed-frame은 Android fix로 증명되지 않는다. Tailnet roadmap도 C13.3 미완료와 이 경계를 유지한다. **계획 반영** |

## 결론

20개 관점 모두 계획에 반영하거나 기존 경계로 유지했다. 검증 가능한 AVD와 JVM fixture를 분리하고, composite peripheral 실기기 재현·iOS callback·제품 event API는 완료로 주장하지 않는다.
