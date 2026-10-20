# C13.3 iOS recognizer/event pairing PR 변경 관점 검토 30개

이번 구현 변경 전체를 API 경계, opt-in 영향, 관찰 parser, 실행 산출물 관점에서 다시 검토했다. 계획 문서의 사전 검토표와 구현 리뷰표는 재사용하지 않았다.

1. **기존 `--app-event-boundary-oracle`가 계속 켜지는가?** — 기존 인수와 새 인수를 OR 조건으로 처리한다. 기존 모드 코드 경로를 유지한다.
2. **새 계측이 기본 host 실행까지 활성화되는가?** — `--recognizer-event-pairing-oracle`에서만 ledger와 controls를 켠다. 일반 launch는 기존 UI를 만든다.
3. **pairing 모드의 log suppression이 event count도 멈추게 하는가?** — `record`는 매번 counter를 올리고 로그 emit만 끈다. export event type과 XCTest app counts가 유지됐다.
4. **기존 모드의 event log 형식이 달라지는가?** — `emitLog` 기본값은 `true`이고, pairing 외 모드는 기존 synchronous publish를 유지한다.
5. **configure 순서 때문에 앱 클래스명이 틀리는가?** — application audit configure가 앱 delegate에서 먼저 실행되며, export에 실제 `NSStringFromClass` 값을 넣는다.
6. **중복 command-line key가 앞선 값과 뒤의 값을 다르게 해석하는가?** — parser는 첫 `--key=value`만 취한다. 테스트는 key를 한 번만 전달한다. 후속 확장 시 duplicate rejection이 필요할 수 있음을 한계로 기록한다.
7. **mode label이 경로 문자를 포함할 수 있는가?** — identifier 검증은 ASCII 영숫자·`-`·`_`만 허용해 파일 경로 separator를 거부한다.
8. **run UUID가 launch 이름과 바뀌어도 파일을 혼합하는가?** — export 파일명과 JSON 안에 mode, run ID, launch ID를 함께 저장하고 분석기가 이를 재검증한다.
9. **다른 recognizer 프로브가 함께 등록되면 map이 조용히 덮이는가?** — ready gate는 정확한 이름 집합 및 unique count를 확인한다.
10. **scroll callback map에서 제거된 recognizer가 이전 값을 재사용하는가?** — key는 recognizer identity이고 callback counter는 name별로 분리한다. 세 probe 이름의 전후 map을 analyzer가 요구한다.
11. **새 direct-touch probe가 scroll event를 scroll recognizer로 세는가?** — direct mode는 `allowedScrollTypesMask=[]`와 direct touch type만 사용한다. 독립 action window는 `.touches`를 요구한다.
12. **scroll mode에 direct-touch 설정이 섞이는가?** — mode 인수에서 scroll type mask recognizer 세 개를 만들고 direct mode의 단일 pan 설정은 건너뛴다.
13. **기존 plain-pan/scroll-type oracle들이 새 UI controls 때문에 구조를 바꾸는가?** — controls는 새 pairing 인수에서만 배치되고 target은 기존 oracle별 구성으로 만든다.
14. **target 좌표가 controls에 가려져 테스트 입력이 다른 view를 치는가?** — constraints는 controls 아래 target을 배치하고 XCTest가 target `isHittable`을 검사한다.
15. **표시 문구 축약이 접근성 판정값도 줄이는가?** — 화면 text는 짧게 표시하지만 ready/status 전체 문자열은 `accessibilityValue`에 둔다.
16. **callback UI 갱신이 `sendEvent` 안에 동기적으로 들어가 표본을 오염시키는가?** — pairing 모드에서 counter/recognizer label refresh는 한 번 예약한 main-queue block으로 합친다.
17. **스크롤 callback refresh 중 여러 recognizer callback이 label을 계속 덮어쓰는가?** — 예약 flag로 다음 turn의 단일 refresh를 보장하고 counter는 recognizer별 저장 값을 읽는다.
18. **direct-touch translation이 absolute 누적값을 action delta로 더해 offset을 과장하는가?** — 직전 translation을 빼 delta를 더하고 terminal state에서 기준을 zero로 복구한다.
19. **새 run 시작 offset이 label에 남아 zero baseline을 속이는가?** — 앱 launch에서 counter와 offset은 0으로 초기화되고 XCTest가 zero baseline을 검증한다.
20. **`viewDidAppear`가 두 번 호출돼 ledger를 재설정하는가?** — 첫 성공 뒤 `ready`가 true가 되어 후속 appearance는 반환한다.
21. **mode invalid면 테스트 입력을 받고도 유효한 것처럼 표시하는가?** — invalid setup은 ready=false status와 invalid log를 남기며 XCTest가 정확한 ready string을 기다린다.
22. **UIEvent enum의 unknown case를 앱 counter에서 잃는가?** — 기존 counter는 `@unknown default`로 `other`를 보존하고 raw type은 ledger에 숫자로 유지한다.
23. **timestamp 측정이 event receive 이후 UI work까지 포함하는가?** — receive monotonic time은 reserve 때, duration은 `super.sendEvent` 직전·직후에 각각 읽는다.
24. **`UInt64` 경과값의 invalid 형태가 parser를 통과하는가?** — analyzer는 안전한 nonnegative integer를 요구한다.
25. **array export가 중첩 event의 처리 완료 순서를 바꾸는가?** — 배열은 sequence로 정렬해 직렬화하고 analyzer는 배열 위치와 sequence가 맞는지 확인한다.
26. **atomic write가 같은 이름의 파일을 부분적으로 노출하는가?** — Foundation `.atomic` write를 쓰며 XCTest는 완성 status와 기대 파일명을 확인한다.
27. **실패한 write를 성공 marker로 보고하는가?** — catch 경로는 `status=error reason=write_failed`; XCTest는 `complete`만 성공으로 센다.
28. **export 결과가 UI status에서만 통과하고 파일 내용 오류는 숨기는가?** — post-run analyzer는 app container에서 복사한 JSON 여섯 개의 schema, run/launch ID, sequence, counters, completion과 overflow를 독립 검증한다.
29. **테스트 산출물을 source revision이 다른 실행과 혼합하는가?** — 최종 xcresult, xcodebuild log, 앱 event ledger, attachments manifest, source/test SHA-256을 같은 evidence directory에 기록한다.
30. **screenshot이 export 전에 찍히거나 UI가 잘려 PR에서 오해를 부르는가?** — export completion 뒤 캡처한다. 최종 direct-touch 및 scroll 화면에서 ready, counters와 `status=complete`가 읽힌다.

최종 판정은 [실행·한계·화면 근거](README.md), `analysis.json`, `xctest/test-results-summary.json` 및 이 PR의 실제 diff에 한정한다. 테스트 action과 event row 사이의 인과 join, UIKit 내부 인식 단계, 외장 trackpad 및 실기기 입력을 주장하지 않는다.
