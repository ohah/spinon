# PR #201 Android wheel source 분류 변경 적대 검토 · 30개 관점 · 2026-10-16

**검토 대상:** PR #201 최종 구현 커밋 `ed4b6a72e9d6b6a4d08beae8e48359c805907067`, PR 본문과 연결된 AVD 근거. 기준 브랜치는 `origin/main`이다. 계획과 구현 자체의 별도 20개 검토는 각각 [계획 검토](../c13-3-native-wheel-source-classification-plan-review-20-2026-10-16.md), [구현·실행 검토](../c13-3-native-wheel-source-classification-implementation-review-20-2026-10-16.md)에 있다.

| # | 독립 실패 관점 | 코드·실행 근거와 판정 |
| ---: | --- | --- |
| 1 | 복합 장치의 지원 source 합집합이 event를 덮어씀 | legacy union negative control은 touchpad 기능을 광고하는 장치의 mouse event를 `TRACKPAD`로 오분류하고, event source/tool 분류는 `MOUSE`를 반환한다. [JVM 출력](./classifier-jvm-output.log). **결함 재현·수정** |
| 2 | 명시 `SOURCE_TOUCHPAD` 경로 | pointer가 있을 때 명시 touchpad source는 `TRACKPAD`로 분류한다. **통과** |
| 3 | 일반 touchpad의 mouse-equivalent source | `SOURCE_MOUSE + TOOL_TYPE_FINGER`는 `TRACKPAD`다. **통과** |
| 4 | 보통 mouse wheel source | `SOURCE_MOUSE + TOOL_TYPE_MOUSE`는 `MOUSE`이며 API 37 AVD 성공 입력 8건도 `MouseWheel`이다. **통과** |
| 5 | 허용되지 않은 event source에 mouse tool만 있음 | source/tool 둘 다 맞아야 하므로 `UNKNOWN`이며 listener는 native enqueue에 보내지 않는다. **통과** |
| 6 | pointer 없는 명시 touchpad source | 첫 구현은 explicit touchpad 분기를 pointer 검사보다 먼저 실행했다. JVM case가 이를 찾아 `hasPointer` 거부를 최우선으로 옮겼고 결과는 `UNKNOWN`이다. **결함 수정·재검증** |
| 7 | pointer 없는 mouse source | `SOURCE_MOUSE`라도 pointer가 없으면 `UNKNOWN`을 반환한다. **통과** |
| 8 | 미확인 pointer tool | finger/mouse 이외의 tool 조합은 `UNKNOWN`이다. **통과** |
| 9 | pointer index 범위 | callback은 `getToolType(0)` 전에 pointer count를 확인하고, 0이면 `TOOL_TYPE_UNKNOWN`을 전달한다. **통과** |
| 10 | stylus·eraser tool이 mouse로 승격됨 | `SOURCE_MOUSE`와 stylus/eraser tool 조합은 finger/mouse 조건에 맞지 않아 unknown이다. 명시 `SOURCE_TOUCHPAD`는 tool과 별도로 trackpad source가 우선한다. 실제 stylus/eraser AVD 입력은 주입하지 않았다. **코드 경계 확인** |
| 11 | 일반 motion/touch가 wheel로 오인됨 | handler는 fixture opt-in 상태에서 `ACTION_SCROLL`만 처리하고 다른 action은 `false`로 반환한다. **통과** |
| 12 | fixture 기본 빌드 또는 실기기가 capture 대상에 섞임 | callback은 `BuildConfig`와 fixture 요청을 확인한다. capture는 `emulator-*` serial, `qemu=1`, API 37을 요구하며 이번 실행은 `emulator-5554`였다. **실기기 미사용** |
| 13 | feature-off APK에 C13.3 diagnostics/FFI 잔류 | `BuildConfig=false`, DEX source diagnostics 없음, C13.3 wheel FFI symbol 없음. helper class는 앱 코드라 포함되고 test harness는 빠진다. [artifact 기록](./android-artifact-inspection.log). **통과** |
| 14 | opt-in APK가 실제 경로를 포함하지 않음 | positive control은 `BuildConfig=true`, source diagnostics와 native wheel FFI symbol이 있고 JVM harness는 없다. **통과** |
| 15 | event source와 device capability 진단 혼동 | 성공 로그는 `event_source_bits=0x2002`, tool `mouse`, device union `0x301`을 별도 필드로 남긴다. touchpad reject는 `0x100008`/`finger`다. **통과** |
| 16 | unknown source 거부가 queue까지 전파됨 | unknown 분기는 진단을 남긴 뒤 즉시 return한다. helper negative case가 판정을 확인하고 AVD queue oracle은 거부 입력 0건을 검사한다. **통과** |
| 17 | Control-wheel 정책을 새 source 분기가 우회함 | source 진단 추가 뒤에도 normalized Control modifier reject가 enqueue 전에 남아 있다. 기존 별도 AVD virtual-uinput 행렬은 4/4 거부를 기록했으며, 이번 PR 재실행 수치로 세지 않는다. **코드 경계 확인** |
| 18 | zero-axis input이 빈 wheel event를 생성함 | API 37 AVD zero axes는 `zero-or-nonfinite-axis`로 거부되고 queue/process·revision은 그대로다. **통과** |
| 19 | NaN/Infinity axis가 계산에 유입됨 | H/V 각 축 `Float.isFinite`와 all-zero check가 native enqueue 앞에 있다. 이런 실제 OS event를 AVD에서 주입하지는 않았다. **코드 경계 확인** |
| 20 | surface/runtime 종료 중 host 접근 | `closing`, host handle, surface availability, gesture readiness를 `stateLock` 아래 검사한다. 실패는 enqueue 전에 reject된다. **통과** |
| 21 | callback 중 surface generation 변경 | host와 generation은 같은 lock 구간에서 snapshot하고 그 generation을 enqueue에 넘긴다. generation 재사용을 새 ID로 치환하지 않는다. **통과** |
| 22 | 비유한·0·음수·비등방 surface scale | X/Y scale이 finite, positive, 허용 오차 안에서 같은 경우만 정규화한다. 나머지는 reject된다. **코드 경계 확인** |
| 23 | ViewConfiguration scroll factor 이상 | horizontal/vertical factor 모두 finite·positive인지 enqueue 전 확인한다. **코드 경계 확인** |
| 24 | 곱셈·나눗셈 뒤 delta overflow | physical raw delta와 CSS delta의 유한성을 다시 검사해 overflow를 거부한다. **코드 경계 확인** |
| 25 | 화면 좌표와 SurfaceView 좌표 기준 불일치 | `getRawX/Y`에서 surface screen origin을 빼고 현재 surface scale로 CSS point를 만든다. **통과** |
| 26 | SurfaceView 밖 입력이 target hit-test에 들어감 | AVD outside-surface case는 `point-outside-surface`로 거부됐다. revision 1과 시작 offsets를 유지했다. **통과** |
| 27 | CSS point 변환 중 비유한 값 | 변환 결과 `pointXCssPx/pointYCssPx`의 finite check가 enqueue 바로 전에 있다. 실제 비유한 좌표 입력은 주입하지 않았다. **코드 경계 확인** |
| 28 | 네 cardinal axis의 부호·factor 회귀 | ±H/±V 4 case에서 native axis 1이 기대 ±64 CSS px로 정규화되고 각 target/offset oracle이 맞았다. **4/4 통과** |
| 29 | 대각 입력 분리·clip 바깥 출력 오염 | 네 대각 case가 각각 queue→draw까지 갔다. 양성 대각 화면에서 scrollport border 밖 134,064 pixel 중 changed 0, 안쪽은 50,684 changed다. **4/4 통과** |
| 30 | PR 근거·제품 상태가 검증을 과장함 | 거부 3/3은 queue/process 없이 revision/offset을 유지한다. evidence symlink negative control은 root 밖 디렉터리를 만들지 않고 ADB보다 먼저 실패했다. Agent Browser는 PR 이미지 `complete=true`, 1080×2400으로 렌더링했다. PR 본문과 로드맵은 C13.3 미완료 및 `61/197`을 유지하며 iOS callback, composite hardware, release·실기기 결과를 주장하지 않는다. **범위 정합** |

## 판정

30개 관점에서 발견한 두 결함(pointer 없는 explicit touchpad 처리 순서, symlink 출력 경로의 외부 directory 생성 가능성)을 수정했다. 소스 분류기는 JVM 9 assertion, Android API 37 AVD 방향 입력 8/8·거부 3/3, feature-off/on artifact 대조로 재확인했다. iOS native callback, 실제 composite input hardware, Release artifact와 실기기 동작은 여전히 미검증이다.
