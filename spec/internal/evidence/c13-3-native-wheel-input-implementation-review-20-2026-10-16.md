# C13.3 native wheel Android 구현 실패 관점 검토 20개 · 2026-10-16

이 검토는 계획 검토와 분리해 Android adapter, capture oracle, opt-in 경계의 실제 변경을 대상으로 했다. AVD 결과는 API 37 ARM64 emulator에 한정한다.

| # | 독립 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 기본 앱에서 fixture route가 활성화되는가 | `BuildConfig`는 기본 APK에서 `false`, opt-in APK에서 `true`였다. 기본 native library에는 wheel FFI symbol이 없었다. **통과** |
| 2 | Activity가 wheel을 소비한 뒤 기존 generic event를 망가뜨리는가 | adapter가 처리하지 않는 event는 `super.dispatchGenericMotionEvent`에 전달한다. **통과** |
| 3 | hover 또는 button motion을 scroll로 오인하는가 | handler는 `ACTION_SCROLL`만 진입시킨다. **통과** |
| 4 | 알 수 없는 source가 queue로 들어가는가 | mouse/touchpad 외 source는 `unknown-source`로 거부한다. **통과** |
| 5 | 장치가 mouse와 touchpad source bit를 함께 가질 때 분류가 불안정한가 | touchpad를 먼저 검사해 `TrackpadScroll`로 분류한다. **통과** |
| 6 | 좌·우 modifier bit만 설정되면 Control-wheel guard를 우회하는가 | `KeyEvent.normalizeMetaState`로 aggregate bit를 만든 뒤 `SHIFT/CONTROL/ALT/META`를 정규화한다. **코드 결함 수정 후 통과**; ADB는 Control meta state를 mouse event에 전달하지 않아 Control runtime은 미검증이다. |
| 7 | 0, NaN, infinite 축 값이 scroll revision을 바꾸는가 | zero-axis negative control은 `zero-or-nonfinite-axis`로 거부됐고 revision 1을 유지했다. NaN/infinite 입력은 코드 guard만 확인했다. **부분 통과** |
| 8 | surface/runtime 준비 전 host handle을 사용하거나 stale generation을 enqueue하는가 | closing, handle, surface, fixture-ready, initial-draw, dimensions를 state lock 안에서 검사한다. 이 상태 전환을 wheel 전용 AVD 케이스로 주입하지 않았다. **부분 통과** |
| 9 | 세로·가로 surface scale 차이 또는 0 scale을 허용하는가 | surface dimensions에서 축별 scale을 계산하고 finite/positive 및 aspect 차이를 검사한다. API 37 AVD 관측은 2.625 px/CSS px였다. **통과** |
| 10 | ViewConfiguration factor가 없거나 잘못되면 잘못된 delta를 전달하는가 | 두 factor를 finite/positive로 검사한다. AVD 관측값은 수평·수직 각각 168 px였다. **통과** |
| 11 | Android H/V 부호 또는 raw physical unit을 잘못 변환하는가 | ±H/±V와 네 대각 조합 8/8에서 factor, raw physical delta, CSS delta, owner offsets가 oracle과 일치했다. **통과** |
| 12 | finite 입력의 곱셈 overflow가 infinite CSS delta로 enqueue되는가 | raw 및 normalized delta 모두 finite인지 검사한다. overflow 입력 주입은 하지 않았다. **부분 통과** |
| 13 | screen origin이 바뀌거나 ADB 좌표에 정수 pixel 반올림이 있으면 target point가 틀리는가 | callback origin을 ready origin과 비교하고 CSS point를 target `(132,112)`에서 최대 `0.191476` CSS px 허용치로 검증한다. 8/8은 `(132.19048,112)`였다. **통과** |
| 14 | 좌표가 없거나 SurfaceView 밖 좌표가 이전 pointer 위치를 재사용하는가 | outside-surface와 좌표 없는 ADB touchpad 표본이 각각 `point-outside-surface`로 거부됐다. stale pointer fallback은 없다. **통과** |
| 15 | SurfaceView 경계에서 right/bottom 좌표가 유효 영역으로 잘못 포함되는가 | `x >= width`, `y >= height`, 음수 및 빈 view를 거부한다. 1px 상단 경계만 runtime negative control로 확인했다. **부분 통과** |
| 16 | 입력 point에서 다른 scroll owner가 잘못 고정되는가 | target CSS point와 queue report의 target `7`을 방향 행렬 전부 검사했다. **통과** |
| 17 | FFI에서 source, unit, scale, phase 또는 modifiers가 유실되는가 | Android physical pixel unit, mouse/touchpad source, scale, `None` phase와 normalized modifier를 enqueue에 전달하고 raw/canonical 값을 로그에 보존한다. **통과** |
| 18 | queue rejection 또는 늦은 worker가 callback을 동기 대기시켜 UI를 막는가 | admission 응답 뒤에만 drain을 예약하고 worker 완료를 기다리지 않는다. bounded queue saturation은 이번 matrix에서 주입하지 않았다. **부분 통과** |
| 19 | 성공 callback이 queue 처리 후 실제 draw에 반영되지 않는가 | 8/8에서 sequence 1 admission, revision 2, 예상 owner offsets와 revision 2 draw를 확인했다. **통과** |
| 20 | scroll 영역 바깥의 paint가 바뀌거나 iOS 표본을 Android 결과처럼 취급하는가 | 대각 입력 전후 root scrollport 바깥 134,064 픽셀은 변경 0개, 안쪽 50,684 픽셀은 변경됐다. iOS callback 미관찰은 별도 gate로 유지했다. **통과** |

## 수정 및 남은 관문

검토 중 sided key modifier bit가 aggregate Android meta flag를 만들지 않는 경우를 발견해 `KeyEvent.normalizeMetaState`를 적용했다. capture oracle도 정수 screen pixel 반올림에 따른 CSS point 오차와 input 직전 SurfaceView origin 변화를 검증하도록 보강했다. 수정 후 fixture boundary test 5/5, 기본/opt-in Android debug build, API 37 AVD 방향 행렬 8/8, 거부 경로 3/3을 다시 실행했다.

Control-wheel 실제 meta 전달, NaN/infinite 입력 주입, surface teardown/background 중 wheel, queue saturation, view 경계 전체, iOS native callback은 이 실행에서 확인하지 않았다. Android 결과는 native mouse wheel fixture slice이며 C13.3 전체 완료 판정이 아니다.
