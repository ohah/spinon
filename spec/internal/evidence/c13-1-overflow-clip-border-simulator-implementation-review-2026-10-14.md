# C13.1 border-strip iOS Simulator 확장 구현 실패 관점 20개 · 2026-10-14

검토 대상은 `--spinon-c131-overflow-clip-border` route, 고정 5-node fixture의 FFI 경로, report 검증, feature isolation이다. 이전 C13.1 paint/hit 리뷰와 별도로 실행했다. simulator 결과는 [실행 근거](c13-1-overflow-clip-paint-hit-border-2026-10-14/ios-border-simulator-verification.txt)를 기준으로 삼는다.

1. **신규 route 미인식** — AppDelegate가 border 인수를 runtime GPU 앱으로 분류하는가? 앱이 생성되고 전용 `SPINON_C131_BORDER_*` log를 내는 것을 확인했다.
2. **기존 route 오염** — 기존 C13 route와 border route를 함께 전달할 때 어느 route가 임의로 선택되는가? 두 인수를 함께 준 Simulator negative control이 `both-overflow-fixture-routes`로 실패했고 신규 host는 생성되지 않았다.
3. **다른 fixture 충돌** — border route에 다른 `--spinon-c*` fixture가 섞이는가? 경계 검사는 허용된 runtime GPU 및 두 C13 인수 외 C13 argument를 거부한다.
4. **오인 접근성 문구** — border 화면이 기존 clip scene으로 읽히는가? 별도 설명과 accessibility label에 border-strip scene을 표시한다.
5. **surface/viewport 불일치** — surface는 400×160인데 CSS viewport가 옛 320×240으로 남는가? report 환경과 캡처 앱은 border route에서 400×160 CSS px로 설정한다.
6. **scene 범위 초과** — 고정 scene에 예상 밖 node가 들어가도 조용히 비교를 계속하는가? Rust report는 frame과 render box가 모두 5개가 아니면 오류를 반환한다.
7. **geometry의 잘못된 순서** — frame 순서나 좌표가 달라도 총 개수만 검사하는가? Swift report는 다섯 frame의 순서별 x/y/width/height를 고정 oracle과 비교한다.
8. **static clip 내부 표본 손실** — `(125,40)`이 잘못 parent/root를 반환하는가? Simulator는 `static-target`, `matched`를 기록한다.
9. **static border-strip 누출** — `(128,40)`에서 child가 paint clip을 넘어 hit되는가? Simulator는 `static-clip`을 기록한다.
10. **static border 밖 과잉 hit** — `(131,40)`에서 clip parent가 border 밖까지 확장되는가? Simulator는 `root`를 기록한다.
11. **positioned clip 내부 target 가림** — `(315,40)`에서 parent painter rank가 child hit를 가리는가? Simulator는 `positioned-target`을 기록한다.
12. **positioned border-strip target 소실** — `(318,40)`에서 absolute child가 잘못 parent로 바뀌는가? Chrome oracle와 Simulator는 `positioned-target`이다.
13. **positioned clip 밖 child 누출** — `(321,40)`이 child로 계속 hit되는가? Simulator는 `root`를 기록한다.
14. **query 누락·중복·재정렬** — 결과 일부만 맞아도 성공 처리되는가? Swift는 6개의 고정 ID, 점, target, matched 상태를 순서대로 검사하고 전체 수를 확인한다.
15. **JSON 숫자 변환 실패** — `JSONSerialization`의 숫자가 `Double`로 읽히지 않거나 잘못된 값이 승인되는가? 실제 simulator report에서 여섯 숫자를 읽고 고정 좌표 비교를 통과했다.
16. **geometry parser 부분 승인** — 좌표 누락이나 문자열/NaN이 기본값으로 대체되는가? 네 좌표 중 하나라도 숫자로 읽히지 않으면 equality check가 실패한다.
17. **Chrome oracle drift** — 다른 browser version이나 수정된 표본이 승인되는가? Rust helper는 고정 HeadlessChrome 155 UA, 5 frames, 6 samples, oracle 내부 expected/actual 일치를 확인한다.
18. **FFI pointer·buffer 경계** — null host/output 또는 0 capacity에서 runtime을 호출하는가? feature-gated FFI boundary test가 invalid output과 null host를 `ERR_ARGUMENT`로 거부한다.
19. **feature leak** — 기본 archive에 신규 symbols/source가 포함되는가? feature-off LLVM 22 archive에서 두 API symbol과 fixture 문자열이 없고, feature-on positive control에는 있다.
20. **기존 C13 fixture 회귀** — shared sample logger 변경이 15-query route를 깨뜨리는가? 같은 Simulator app binary가 13/13 frames, 15/15 hits를 다시 통과했다.

## 판정

이 delta에서 발견한 수정 필요 회귀는 없다. 전체 증거는 build, focused FFI boundary, workspace test·Clippy, rustfmt, module-boundary test와 Simulator log로 분리해 보존했다. Metal drawable readback/scanout, Android border route, DOM event dispatch, WPT matrix, 전체 C13.1 완료 판정은 검증하지 않는다. macOS host-native V8 isolate 생성 실패는 Simulator runtime 통과로 바꾸어 세지 않았다.
