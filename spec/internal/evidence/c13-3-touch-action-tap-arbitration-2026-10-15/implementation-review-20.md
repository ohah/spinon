# C13.3 native tap arbitration 구현 실패 관점 20개 · 2026-10-15

검토 대상은 Android/iOS tap probe, fixed-fixture capture harness, 최종 Simulator logs와 기본/opt-in artifact다. 계획 검토표를 재사용하지 않고 구현과 실제 실행에서 아래 독립 실패 경로를 확인했다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | Android tap flag가 fixture feature 없이 진입 | `MainActivity`와 runtime route가 opt-in gesture fixture를 요구한다. 기본 build에서 fixture FFI/payload가 없다. 통과. |
| 2 | iOS tap argument가 일반 C04 화면을 tap fixture로 바꿈 | `--spinon-c133-tap-probe` 단독은 `--spinon-c133-scroll-gesture`와 별도 feature 없이 실행되지 않는다. 통과. |
| 3 | scenario 누락·중복·다른 ID가 기본 장면으로 조용히 대체 | tap probe는 정확히 policy scenario 3/4만 허용하고 나머지는 거부한다. 통과. |
| 4 | host가 scenario 번호를 보고 computed action을 추측 | iOS arbitration은 FFI report의 `touch_action_effective`를 읽고 실제 값이 `none`/`pan-x`인지 확인한다. 통과. |
| 5 | 첫 native draw 전 tap이 성공으로 집계 | fixture evaluation 및 initial draw가 준비되기 전에 gesture route가 input을 열지 않는다. capture harness도 ready와 initial revision을 대조한다. 통과. |
| 6 | synthetic callback을 native tap으로 잘못 계산 | Android는 실제 `OnClickListener`, iOS는 실제 `UITapGestureRecognizer` recognition만 activation으로 기록한다. 통과. |
| 7 | accepted `End` 없이 Android tap 후보를 확정 | down accepted, pointer active, pan 미시작, cancel/suppression 부재와 `End` enqueue 수락을 모두 요구한다. 통과. |
| 8 | touch state reset이 ACTION_UP click 후보를 지워 정상 tap을 누락 | reset 뒤 local candidate와 accepted End 결과로 click candidate를 세우며 Android AVD에서 실제 activation 1회를 관찰했다. 통과. |
| 9 | below-slop move가 pan으로 처리돼 tap을 삼킴 | Android log는 native slop 21px 이전에 pan을 시작하지 않고 tap을 보존한다. iOS recognizer는 system threshold를 사용한다. 통과. |
| 10 | native slop 직후 pressed 상태가 남아 swipe 종료가 click 발생 | 실제 Android pan-x 실행에서 두 번째 click을 재현했다. slop 첫 crossing에서 pressed/long-press 상태를 취소하고 click forwarding을 끊어 수정했다. 최종 activation은 1회다. |
| 11 | pan 뒤 ACTION_UP이 tap 후보로 재분류 | Android 최종 `up_candidate=false`이며 iOS swipe 뒤 tap activation 수가 증가하지 않는다. 통과. |
| 12 | ACTION_CANCEL 뒤 click candidate가 남음 | cancel 경로가 C13.3 gesture와 tap forwarding/candidate 상태를 reset한다. Simulator 최종 tap/swipe 순서에서 stale candidate는 관찰되지 않았다. 통과. |
| 13 | 두 번째 pointer가 들어온 뒤 첫 pointer up이 click으로 계산 | multi-pointer path가 gesture를 취소하고 모든 pointer 종료까지 stream을 suppress한다. 해당 동작은 구현 검토로 확인했으며 이 tap run은 실제 multi-touch proof가 아니다. 경계 명시. |
| 14 | Android listener가 정상 tap down/up을 SurfaceView까지 전달하지 않음 | tap probe의 accepted tap stream만 native clickable view에 전달하고 final activation 1회를 확인했다. 통과. |
| 15 | iOS tap recognizer가 pan recognizer보다 먼저 성공 | `UITapGestureRecognizer.require(toFail:)` 관계를 설정하고 pan-x swipe에서 pan이 시작되는 것을 확인했다. 통과. |
| 16 | computed `none`인데 iOS native pan이 입력을 선점 | 첫 fixture input 전 pan recognizer를 비활성화한다. iOS run은 `effective=none`, `native_pan_enabled=false`와 tap 1회를 기록했다. 통과. |
| 17 | `pan-x`에서 iOS tap과 swipe 모두 인식 | tap은 pan 실패 뒤 한 번 인식하고, 이후 별도 horizontal swipe는 pan을 시작해 inner X만 움직였다. 총 tap activation은 1회다. 통과. |
| 18 | Android clipped accessibility bounds를 전체 SurfaceView origin으로 사용 | 초기 주입 좌표가 약 16.6 CSS px 어긋난 harness 오류를 재현했다. full view origin 복원과 runtime/local/screen 좌표 교차 검증으로 수정했다. 최종 좌표는 `(148,132)` CSS px와 일치한다. |
| 19 | screenshot의 marker가 log offset·clip과 불일치 | PNG decoder가 scene/root pixel과 기록된 inner offset 위치의 exact red marker를 확인한다. full screenshot과 crop preview를 보존했다. 통과. |
| 20 | 기본 앱에 test-only symbol 또는 fixture payload가 누출 | Android APK 및 iOS Simulator app에서 four C ABI exports와 fixture payload 부재를 확인하고 opt-in positive control에서 존재를 확인했다. 통과. |

원본 실행값·수정 전 harness 오류·수정 전 Android click 재현은 [실행 근거](README.md)와 인접 log에 보존했다. 확인된 Android slop 이후 click 회귀는 수정 후 양 플랫폼 Simulator 재실행에서 닫혔다. 이 검토는 고정 simulator fixture 범위이며 제품 click/event 완료 판정이 아니다.
