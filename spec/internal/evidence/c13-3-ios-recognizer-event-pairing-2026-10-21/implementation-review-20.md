# C13.3 iOS recognizer/event pairing 구현·실행 실패 관점 검토 20개

대상은 독립 UIKit 계측 host와 새 XCUITest 두 개다. 기존 계획 검토표와 다른 실패 조건을 기준으로 검토했으며, 최종 소스 재실행 결과에 연결한다.

1. **`sendEvent` 진입 예약 전에 callback이 재진입하면 sequence 순서가 깨지는가?** — entry에서 slot과 sequence를 만들고 `super.sendEvent` 뒤 같은 slot을 채운다. analyzer가 연속 sequence와 완성 snapshot을 검사한다.
2. **중첩 event가 바깥 event slot을 덮는가?** — 저장소 index를 slot으로 보존하므로 안쪽 event의 append가 바깥 index를 바꾸지 않는다. 두 final ledger 모두 sequence gap이 없다.
3. **`super.sendEvent`가 누락·중복되어 실제 UIKit 전달이 달라지는가?** — override에 단일 호출 경로가 있고 경과 시간을 바로 기록한다. XCTest 앱 event 및 direct-touch positive control이 최종 실행에서 통과했다.
4. **`super` 반환 전에 post snapshot을 저장하는가?** — `complete`는 호출 반환 뒤에만 실행한다. event row의 pre/post callback delta로 검사했다.
5. **다른 thread에서 recognizer를 읽고 유효한 row처럼 남기는가?** — row별 `mainThread`, 전체 violation flag와 빈 snapshot을 기록한다. analyzer는 비-main row와 violation을 거부한다.
6. **weak recognizer가 해제된 뒤 sentinel `-1`이 정상 상태로 통과하는가?** — analyzer가 음수 state/callback count를 거부한다. 최종 2,400행에는 음수 값이 없었다.
7. **mode 인수와 실제 등록 recognizer가 어긋나도 캡처가 시작되는가?** — ready gate는 허용 mode별 정확한 recognizer 이름 집합과 중복 여부를 검사한다. 테스트는 일치하는 ready marker를 기다린다.
8. **등록 전 startup event가 recognizer 전후 snapshot처럼 보이는가?** — `viewDidAppear` 이후 준비된 probe만 capture를 시작하고 이전 event는 ledger에서 제외한다. 각 fresh launch는 ready 후 zero baseline이었다.
9. **앱 process 재시작 때 이전 실행의 counter나 sequence가 이어지는가?** — configure에서 모든 static ledger state를 재설정하고 테스트가 앱을 3회 종료 후 새로 실행한다. launch ID 1–3이 각각 한 파일로 나왔다.
10. **4,096행을 넘긴 뒤 일부 행을 조용히 잃는가?** — 초과 시 overflow를 고정하고 파일에 남긴다. analyzer는 overflow를 거부한다. 실제 최대 실행은 711행이었다.
11. **완료되지 않은 dispatch가 정상 기록으로 export되는가?** — export 전에 post/elapsed 누락 수를 계산하며 analyzer는 `incompleteRows=0`과 row별 필드를 확인한다. 6개 ledger 모두 완전했다.
12. **sequence 또는 monotonic clock이 역행하는가?** — row sequence의 정확한 연속성과 receive time 단조성을 검증했다. 6개 ledger 모두 회귀 0건이었다.
13. **state transition과 target-action callback delta를 같은 사실로 취급하는가?** — 서로 독립 필드로 저장하고 analyzer는 불일치를 실패로 보지 않는다. 관찰된 state와 callback delta를 그대로 보존했다.
14. **scroll type union의 continuous 또는 discrete callback이 union에서 빠지는가?** — action-window gate와 per-event analyzer가 continuous delta equality 및 discrete-positive union 존재를 점검한다. 위반은 0건이었다.
15. **indirect scroll과 direct touch 결과가 한 성공률로 합쳐지는가?** — mode마다 별도 테스트, run ID, ledger 파일을 사용한다. direct-touch는 30/30 touch/callback/sign window를 별도 통과했다.
16. **누락 event/callback이 짧은 window 때문에 늦게만 관찰되는가?** — 새 테스트의 관찰 한도를 계획대로 2초로 맞추고 150ms settle을 유지했다. 각 action window parser는 missing·감소 counter를 무효 처리한다.
17. **한 앱 event가 XCTest action 하나라고 잘못 귀속되는가?** — per-event sequence와 XCTest 전후 window를 서로 별도 관찰로 남긴다. 결과에는 event/action ID join을 주장하지 않는다.
18. **export 버튼 event가 `super.sendEvent` 완료 전에 snapshot되어 마지막 row를 잃는가?** — export serialization을 main queue 다음 turn에 예약한다. final ledger에는 export 입력 row가 포함되고 UI status는 `complete`였다.
19. **파일 경로가 인수로 조작되거나 이전 launch 파일을 덮는가?** — 식별자에서 경로 구분자를 허용하지 않고 각 mode run UUID와 launch 번호를 파일명에 넣는다. analyzer가 두 run ID와 launch 1–3의 정확한 여섯 파일을 확인한다.
20. **기능은 통과했지만 시각적 증거가 잘리거나 export 전 화면을 보여 주는가?** — 첫 캡처에서 긴 status text clipping을 발견해 화면 문구를 짧게 하고 export 완료 뒤 screenshot을 찍도록 수정했다. 최종 두 캡처에서 전체 marker와 완료 status가 읽힌다.

첫 실행에서 XCTest의 기대 파일 prefix가 앱 출력 이름과 달라 export 검사가 실패했다. 실제 생성 파일을 확인해 기대 prefix를 고쳤다. 그 뒤 화면 검토에서 ready/callback 문구가 잘리는 것을 발견해 UI 문구와 캡처 시점을 조정했다. 최종 재실행의 판정 기준은 [실행 근거](README.md)와 `xctest/test-results-summary.json`이다.
