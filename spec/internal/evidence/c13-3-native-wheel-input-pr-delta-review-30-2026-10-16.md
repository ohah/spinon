# PR #198 변경분 실패 관점 검토 30개 · 2026-10-16

검토 대상은 PR #198의 변경분 `4480ec2c69b640d7db45890b7059eb5816bff52c`와 본문, 현재 확인 가능한 AVD·Agent Browser 근거다. 구현 검토 20개와 분리해 저장소 범위, 계약, 증거와 merge 조건을 다시 확인했다.

| # | 독립 변경분 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 제목·작업 브랜치가 실제 변경 범위를 설명하는가 | 제목과 브랜치 모두 C13.3 Android native wheel adapter를 가리킨다. **통과** |
| 2 | PR 유형/영역 label이 빠졌거나 엉뚱한가 | `enhancement`, `area: android`, `area: ios`, `area: testing`, `area: docs`가 설정됐다. **통과** |
| 3 | Android 구현을 양 플랫폼 완료로 읽을 수 있는가 | 계획·상태·본문 모두 Android mouse slice와 iOS gate를 분리한다. **통과** |
| 4 | unrelated product surface가 함께 바뀌었는가 | 변경은 C13.3 host adapter, preflight, 계획·계약·상태·실행 자료와 그 검증 도구에 한정된다. **통과** |
| 5 | 내부 계약 버전이 구현 slice만으로 올라갔는가 | 내부 계약 0063의 `0.1.0`을 유지했다. **통과** |
| 6 | 공개 `WheelEvent`/DOM 계약이 구현되지 않은 채 변경됐는가 | 공개 API를 바꾸지 않고 fixture/native input 결과를 내부 queue로 보낸다. **통과** |
| 7 | 공식 상태에서 C13.3 전체가 체크 완료됐는가 | `spec/STATUS.md`에는 부분 실행 근거만 추가했고 완료 체크는 바꾸지 않았다. **통과** |
| 8 | Tailnet 또는 PR의 공식 완료 숫자가 갱신 결과와 충돌하는가 | 현재 로드맵과 PR 본문은 `61/197`이며 이 PR에서 증가시키지 않았다. **통과** |
| 9 | Activity가 기존 generic input을 무조건 삼키는가 | C13.3 handler가 처리하지 않으면 `super.dispatchGenericMotionEvent`로 전달한다. **통과** |
| 10 | debug 기본 앱에서 고정 fixture 요청 없이 wheel adapter가 작동하는가 | `BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST`와 fixture route를 함께 요구한다. 기본 APK `false`를 확인했다. **통과** |
| 11 | mouse와 touchpad source를 섞거나 unknown source를 통과시키는가 | touchpad 우선 분류, 허용 source/action만 enqueue하는 조건과 source 로그가 있다. **통과** |
| 12 | Android native 단위와 Rust queue ABI 필드 순서가 서로 다른가 | physical pixel unit, scale, source, phase, modifiers와 generation을 기존 C13.3 wheel FFI signature 순서로 전달한다. **통과** |
| 13 | RTL/momentum/pinch 동작을 근거 없이 이 slice에 포함하는가 | momentum은 `None`; pinch·magnification·RTL·fling은 명시적으로 미지원이다. **통과** |
| 14 | callback이 worker drain이나 새 draw를 동기 대기하는가 | bounded enqueue 뒤 drain을 예약하고 callback에서 worker 완료를 기다리지 않는다. **통과** |
| 15 | Rust queue가 거부한 input에도 새 draw를 예약하는가 | `status=0`과 `admitted=true`일 때만 status/draw drain을 예약한다. **통과** |
| 16 | 기본 APK의 Java string 존재를 fixture 활성화 증거로 오해하는가 | artifact 문서가 debug DEX의 비-minify 특성과 BuildConfig/native symbol 판정을 분리한다. **통과** |
| 17 | 기본 native library에 wheel FFI가 남아 있는가 | 기본 ARM64 `.so`의 `llvm-nm`/문자열 검사에서 enqueue Rust symbol과 native marker가 없고 opt-in positive control에는 있다. **통과** |
| 18 | surface marker literal이 기본/opt-in source에 실수로 남는가 | marker 조립 방식과 `fixture-module-boundary.test.mjs` assertion을 함께 넣었다. **통과** |
| 19 | 성공 matrix가 앱 재실행 없이 이전 scroll state에 누적되는가 | 방향별로 fixture를 새로 실행하며 시작 offsets/revision을 확인한다. **통과** |
| 20 | screen pixel 정수 반올림으로 target point가 다른 target에 걸리는가 | CSS point 허용치와 ready/input SurfaceView origin을 capture oracle에서 확인했다. **통과** |
| 21 | 로그만 성공하고 rendered state는 갱신되지 않았는가 | 모든 8개에서 queue sequence/admission, target 7, revision/owner offsets와 draw를 대조한다. **통과** |
| 22 | scrollport 바깥 paint 변화가 화면 자료에서 누락되는가 | 전후 캡처를 SurfaceView local origin에 정렬해 border 밖 134,064 pixels 중 0 changed를 확인한다. **통과** |
| 23 | negative controls가 다른 조건 때문에 우연히 통과하는가 | outside-surface, missing touchpad point, zero axes가 각각 명시 reason으로 거부되고 revision 1을 유지한다. **통과** |
| 24 | AVD 결과가 physical device나 GPU 성능 결과로 확대되는가 | 문서와 본문은 Android API 37 ARM64 emulator, hardware GPU/physical device 미검증으로 제한한다. **통과** |
| 25 | iOS Simulator 준비 상태를 UIKit callback 통과로 과장하는가 | preflight log는 ready만 기록하고 callback/sign 미관찰과 adapter 보류를 분명히 한다. **통과** |
| 26 | PR 변경 자체에서 필요한 검증이 비어 있는가 | fixture boundary test 5/5, 기본/opt-in Android build, AVD 8/8 + 3/3 실행 기록을 연결했다. 전체 workspace test는 주장하지 않는다. **통과** |
| 27 | Tailnet 미리보기에서 stale 문서나 화면이 계속 제공되는가 | 최종 PR head로 제한 Rspress build와 Tailnet sync를 다시 수행했다. Agent Browser에서 로드맵 `61/197`, Android 실행 자료 `8/8` 및 거부 제어 `3/3`, 전후 화면 1080×2400, 변경분 manifest 82/82 HTTP 200 및 SHA-256 일치를 확인했다. [제공 검증 JSON](c13-3-native-wheel-input-2026-10-16/tailnet-served-verification.json)과 [Agent Browser 화면](c13-3-native-wheel-input-2026-10-16/tailnet-roadmap-agent-browser.png)에 남겼다. **통과** |
| 28 | PR 본문 화면 링크가 branch 이동/삭제 뒤 깨지는가 | image URL은 branch 대신 commit `4480ec2c…`에 고정했다. Agent Browser에서 body image 1080×2400 로드를 확인했다. merge 후 안정 URL을 다시 확인한다. **현재 통과** |
| 29 | PR 본문·문서 링크가 PR의 실제 head 파일을 가리키는가 | PR 페이지에서 plan, 계약, evidence와 review 링크가 head branch 파일을 연다. **통과** |
| 30 | merge 가능/검증 gate가 미확인 상태인가 | PR은 open/non-draft, `MERGEABLE`/`CLEAN`; checks 보고가 없고 main required-status-check endpoint는 404다. Android build·AVD 및 focused test 근거는 확인했다. **통과** |

## 검토에서 찾은 후속 조치

검토 27은 최종 Tailnet 동기화 및 Agent Browser 제공 해시 확인으로 해결했다. merge 뒤에는 PR 본문 링크를 `main` 경로로 바꾸고 이미지가 계속 로드되는지 Agent Browser로 다시 확인한다.
