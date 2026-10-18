# C13.3 iOS UIApplication event 경계 구현·실행 적대 검토 20개

계획 검토와 분리해 구현 코드, clean Simulator 실행, 파생 자료의 실패 경로를 다시 대조했다.

1. **진단 변경이 제품 입력 경로에 섞였나?** — 변경은 evidence harness의 UIKit host·XCUITest와 계획/근거에 한정되고 runtime, adapter, 공개 API는 수정하지 않았다.
2. **관찰 기준이 private UIKit 동작에 의존하나?** — `UIEvent.type == .scroll` 및 공개 `UIApplication.sendEvent(_:)`만 사용하고 Apple API 문서와 iOS 26.2 SDK availability를 계획에 연결했다.
3. **custom UIApplication class가 실제 앱 진입점에 설치됐나?** — `UIApplicationMain`에 principal class를 넘기고 clean run 3 fresh launches 모두 `C133ProbeApplication`을 표시했다.
4. **이벤트 관찰이 실제 전달을 막나?** — override는 count/marker 기록 뒤 `super.sendEvent(event)`를 한 번 호출하고 화면·접근성 갱신은 super 뒤에 발행한다. 초기 검토에서 동기 notification이 super 전에 UI observer를 실행할 수 있음을 찾아 순서를 수정했다. 30 scroll windows와 direct-touch 30 rows에서 recognizer callback을 확인했다.
5. **계측 옵션이 기존 fixture 실행을 바꾸나?** — counter 기록과 marker는 `--app-event-boundary-oracle`로 opt-in하며 다른 launch mode에서는 비활성이다. 이번 matrix는 새 flag가 켜진 mode만 대상으로 했다.
6. **새 UIEventType 추가 시 무음 누락하나?** — 알려진 공개 event type을 개별 분류하고 `@unknown default`는 `other`로 세어 미래 enum 변화를 감춘 채 scroll로 오인하지 않는다.
7. **0 baseline이 미파싱 값을 정상으로 받아들이나?** — event counter snapshot은 모든 공개 분류 키를 `-1` sentinel로 시작하고 3/3 launch에서 실제 0 값과 앱 class를 확인했다.
8. **counter label 파싱 실패가 `no-app-event`로 보이나?** — action window 앞뒤 snapshot에서 앱 class, 각 키, 비음수 값, 단조 증가를 검사하고 실패는 `invalid-counter-window`로 분리해 assertion에 포함한다. final 30/30 valid, invalid 0이었다.
9. **event counter가 감소·reset되어 delta가 왜곡되나?** — fresh process마다 reset한 뒤 action window 간 증가만 허용한다. 30/30 행에서 역행·누락은 없었다.
10. **recognizer accessibility 값이 부분적으로 누락되나?** — continuous/discrete/union 세 키를 각각 파싱하고 비음수·단조 증가를 함께 확인했다. invalid recognizer window도 실패 수에 포함한다.
11. **direct-touch settling 직전 값으로 false negative를 기록하나?** — 150ms settling 뒤 callback과 offset을 다시 읽도록 수정했다. direct-touch positive control은 callback/sign 30/30이었다.
12. **recognizer 결과 이름이 union callback을 가리나?** — 과거 `continuous-only` 이름은 union 동시 callback을 숨겨 `continuous+union`으로 고쳤다. 최종 aggregate가 그 분류를 21 window로 기록한다.
13. **typed recognizer callback이 union 없이도 유효 처리되나?** — continuous/discrete가 union 없이 증가하면 invariant violation으로 분리하고 0을 요구한다. final 0건이었다.
14. **직접 손가락 입력을 간접 scroll로 합산하나?** — 별도 test method에서 `.touches` 증가·`.scroll` 불변·pan callback을 기록한다. 30/30 window에서 touches 증가, scroll delta 0이었다.
15. **direct-touch 대조군에서 부호 또는 baseline이 잘못되나?** — fresh app zero baseline 3/3, 좌우 방향별 translation 부호와 callback을 30/30 확인하고 앱 event class도 검증한다.
16. **target 또는 앱 launch 실패가 분모에서 빠지나?** — target·counter label 존재와 hittable 상태를 확인하고 실패 launch는 분리한다. 유효 launch 3/3, scroll row 30/30이다.
17. **카운터 이벤트 하나를 action 하나로 오해하나?** — action 직후 observation window의 delta로만 보고하며 `UIEvent` 대 XCTest action ID의 직접 join은 없다고 README·계획에 한계를 추가했다. 최종 revision은 30 window에 총 90 event를 관찰했다.
18. **화면 증거가 clipped label이나 다른 실행의 수치를 보여주나?** — 초기 한 줄 counter가 clipped되어 두 줄 시각 요약으로 고쳤다. 최종 focused screenshot은 `scroll=29`, `continuous=12`, `union=12`을, direct-touch screenshot은 `scroll=0`, `touches=730`을 보인다. 집계 30 observation windows는 별도 summary에 있다.
19. **공유 로그에서 process/thread ID가 노출되나?** — Simulator marker raw는 임시 경로에만 두고 공유 derivative에서는 timestamp와 process/thread 식별자를 제거했다. `runs/SHA256SUMS` 전체 검증을 수행했다.
20. **method pass나 상태 문구를 제품 완료로 확대하나?** — 최종 source focused xcresult는 2 pass·0 fail·0 skip이다. 전체 suite 17 pass는 notification refresh 이동 전 revision 결과다. 최종 source full-suite retry는 disk-full summary write failure로 count unknown/0이며 통과로 세지 않는다. STATUS는 harness-only 조사로 남기고 C13.3 및 74/207을 미완료 상태로 유지한다.

판정: category 명칭, event-counter fail-open, direct-touch settling 값, 화면 텍스트 잘림, 빠진 checksum manifest를 검토 중 발견해 각각 수정했다. clean build 뒤 현재 source의 focused result bundle attachment를 확인했고 30 scroll windows 및 30 direct-touch windows의 counter integrity를 다시 검증했다. 전체 suite 17/17은 이전 notification-order revision 결과다. 최종 source full-suite retry는 disk-full로 summary를 남기지 못했다. 앱 경계 뒤 UIKit 내부 단계, 실기기 trackpad, 제품 adapter 전달은 미확정이다.
