# C13.3 Android native wheel source 분류 구현·실행 검토 · 20개 관점 · 2026-10-16

**대상:** `WheelSourceClassifier`, `C0410RuntimeGpuDemo.handleC133WheelMotionEvent`, JVM harness, Android API 37 AVD 실행과 opt-in artifact 대조.

| # | 독립 관점 | 실행·코드 근거와 판정 |
| ---: | --- | --- |
| 1 | 명시 `SOURCE_TOUCHPAD` | classifier test가 명시 touchpad source를 `TRACKPAD`로 판정한다. **통과** |
| 2 | mouse source + finger | classifier test가 `SOURCE_MOUSE`/finger tool 조합을 `TRACKPAD`로 판정한다. **통과** |
| 3 | mouse source + mouse | 실제 API 37 AVD 성공 8건 모두 `event_source_bits=0x2002`, `pointer_tool=mouse`이며 `MouseWheel`이다. **통과** |
| 4 | unknown source | JVM test에서 mouse tool만 있고 허용 source가 없을 때 `UNKNOWN`이며 enqueue하지 않는 분기다. **통과** |
| 5 | unknown tool | JVM test에서 mouse source와 미확인 tool 조합이 `UNKNOWN`이다. **통과** |
| 6 | pointer count 0 방어 | handler는 pointer 0 접근 전 count를 확인한다. classifier는 `SOURCE_TOUCHPAD`를 포함해 pointer가 없으면 unknown을 반환하고 JVM harness가 그 조합을 확인한다. **통과** |
| 7 | composite device legacy negative control | harness는 장치 union에 touchpad capability가 있고 현재 event는 mouse+mouse인 입력에서 legacy 결과 `TRACKPAD`와 새 결과 `MOUSE`가 달라짐을 고정한다. **결함 재현·수정 확인** |
| 8 | source decision의 입력 경계 | 분류 helper에는 event source 상태와 pointer tool만 전달된다. helper와 caller의 판정값 계산은 device union을 참조하지 않는다. **통과** |
| 9 | union의 용도 제한 | `getSources()` 결과는 `device_supported_source_bits` 문자열에만 기록되고 source enum 선택에는 들어가지 않는다. **통과** |
| 10 | 성공 진단 일관성 | mouse callback은 raw event bits, pointer count/tool/type, device union, device id를 함께 기록한다. 예: `0x2002`, `mouse`, `0x301`. **통과** |
| 11 | touchpad 진단 | ADB touchpad sample은 `0x100008`, `finger`, `TrackpadScroll`로 분류된 뒤 좌표가 SurfaceView 밖이어서 거부됐다. **통과** |
| 12 | 모든 reject 경로 진단 | 이전 ADB 검증에서 outside reject 로그에 event source field가 빠져 assertion이 실패했다. unknown source, Control, axis, readiness, scale/factor, point reject 분기에 `sourceDiagnostics`를 추가했고 outside/touchpad/zero AVD 로그에서 확인했다. **실패 원인 수정·통과** |
| 13 | queue no-op | 세 negative input의 draw revision은 1, 시작 offsets는 `root:0,0;outer:0,0;inner:86,106`; wheel process event는 없다. **통과 3/3** |
| 14 | 방향 회귀 | 기존 horizontal/vertical과 diagonal native wheel matrix는 queue→draw, target/offset/revision oracle을 만족했다. **통과 8/8** |
| 15 | 표시 경계 | aligned SurfaceView crop에서 root scrollport 바깥 134,064 pixel은 changed 0, 안쪽은 50,684 pixel 변경이다. **통과** |
| 16 | 순수 Java oracle와 실제 SDK compile | `javac`/`java` harness 8 assertions 및 Android Gradle Java compile이 통과했다. harness는 platform MotionEvent adapter 자체를 대체하지 않는다. **경계 명시** |
| 17 | 출력 경로 탈출 | evidence root 자체가 저장소 안인지 canonical path로 확인하고, 출력 경로의 각 component symlink를 검사한다. negative control은 root 밖에 디렉터리를 만들지 않고 ADB 접촉 전 거부된다. **통과** |
| 18 | 소스 재현성 | capture JSON은 untracked helper/test/harness와 plan/status/contract/roadmap을 포함한 source file SHA-256을 기록한다. **통과** |
| 19 | artifact 분리 | off APK BuildConfig false 및 native wheel symbol 부재, on APK BuildConfig true 및 symbol 존재를 확인했다. source diagnostics는 opt-in DEX에만 있고 helper는 양쪽 DEX에 있다. **통과** |
| 20 | 증거 범위·미완료 경계 | 실행 serial은 `emulator-5554`, qemu=1/API 37; ADB에 연결된 실기기는 대상으로 사용하지 않았다. AVD composite device, 물리 trackpad, iOS callback, release/scanout 증거는 없고 C13.3은 미완료다. **한계 유지** |

## 결론

구현과 AVD 재실행에서 source-union 오분류를 제거하고 reject 진단 누락을 보완했다. 실제 composite input device를 재현하지 못한 점은 JVM negative control로 보완한 것이며 하드웨어 검증으로 승격하지 않았다. 최종 AVD log, 화면, build와 source hash는 이 디렉터리의 `README.md`, `android-avd-matrix.json`, `android-artifact-boundary.md`, `source-revision.json`에 있다.
