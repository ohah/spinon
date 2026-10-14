# C13.1 iOS hit identity grid 구현 적대 검토 20개

검토일: 2026-10-15
대상: iOS app route, report parser, run/sample/frame 상관관계, capture flow
판정: 수정 후 최종 Simulator 근거에서 통과. 계획 검토와 별도 관점이다.

1. **앱 진입점 분기 누락** — 새 launch argument만 추가하고 AppDelegate가 demo controller를 열지 않으면 기본 화면에 남는 경로를 확인했다. 최초 실행에서 흰 화면을 재현했고 AppDelegate route 목록에 flag를 연결했다. 최종 세 프로세스에서 demo log와 화면을 확인했다.
2. **기존 fixture route와 동시 선택** — 두 C13 route가 전달될 때 먼저 선언된 route가 조용히 승리하는지 확인했다. conflict 검사에서 host init 전에 거부하며 negative control에는 host 반환과 eval marker가 없다.
3. **잘못된 route의 부수 효과** — argument 오류 이후 V8 host, environment 변경 또는 fixture eval이 시작되는지 확인했다. 중복 route 실행 로그에서 세 효과 모두 시작되지 않았다.
4. **이전 compact route 회귀** — 새 flag 없이 기존 compact route가 동일 host/eval을 유지하는지 확인했다. 분기 조건은 기존·신규 route의 공통 wrapper로만 확장됐고 기존 41 frame·52 sample 검증을 보존한다.
5. **CSS viewport와 UIKit canvas 크기 혼동** — 화면 point나 display scale이 300×100 CSS grid 좌표에 섞이는지 확인했다. route는 300×100 canvas와 CSS viewport를 지정하며 FFI가 viewport와 30,000개 query 수를 다시 검사한다.
6. **요약 marker 누락 또는 중복** — 부분 report나 중복 marker를 성공으로 해석할 수 있는지 확인했다. parser가 missing/duplicate marker를 거부하고, malformed report는 route 실패로 전달한다.
7. **중첩 JSON·문자열 brace 처리** — preview 문자열 안 brace/escape가 객체 경계를 조기 종료하는지 확인했다. quote/escape/depth 상태를 추적한 뒤 JSONSerialization으로 전체 객체를 검증한다.
8. **잘린 객체와 후행 report 처리** — 객체 종료 brace가 없거나 뒤에 다른 report가 붙을 때 prefix만 읽는지 확인했다. scanner는 닫힌 객체를 요구하고 parser는 해당 raw object 전체의 JSON 유효성을 요구한다.
9. **중복 JSON 키·비정규 직렬화** — JSONSerialization이 duplicate key를 덮어쓴 뒤 통과하는지 확인했다. sorted compact canonical JSON과 원문을 비교해 중복 키와 비정규 요약을 거부한다.
10. **스키마 확장·필드 누락** — 예상하지 않은 필드나 빠진 digest를 무시하는지 확인했다. summary key set을 정확히 비교하고 각 field의 타입을 요구한다.
11. **숫자 타입 위장** — bool, 소수, 문자열 숫자, 범위 밖 정수를 count로 받는지 확인했다. CoreFoundation boolean type과 NSNumber 표현을 검사하고 정수만 Int로 변환한다. parser mutation harness의 타입 변조 controls가 이를 거부한다.
12. **count 산술과 status 불일치** — matched와 mismatch 합이 30,000이 아니거나 status가 mismatch 수와 모순될 때 성공하는지 확인했다. 두 조건을 명시적으로 검사하며 preview 제한도 16으로 고정한다.
13. **preview 폭주·좌표 경계** — 16개 초과 preview, viewport 밖 좌표, 빈 expected ID, 잘못된 actual ID를 검사했다. parser가 거부하고 Rust FFI는 최대 16개만 report에 넣는다.
14. **digest 핀 교체 또는 mismatch 요약 통과** — expected identity, 실제 identity, Chrome observation, grid oracle 중 하나가 달라져도 success가 되는지 확인했다. 고정 네 digest를 비교하며 mismatch인 actual identity가 expected와 같아지는 모순도 거부한다.
15. **세대 정보 누락과 오래된 callback 혼합** — report에 generation이 없으면 frame/sample 로그와 다른 실행을 섞어 성공할 수 있는지 확인했다. generation 누락을 실패 처리하고 app run ID·generation을 모든 관련 로그에 붙인다.
16. **frame/sample 부분 성공** — 전체 grid만 맞고 41 frame 또는 기존 52 sample이 실패한 경우를 확인했다. 기존 compact report 검증이 먼저 완료되어야 full-grid summary를 판정하며, 캡처는 모든 개별 frame/sample에 동일 context가 있는지 검사한다.
17. **검증 오류의 성공 UI 누수** — parse failure, mismatch, 누락 report가 완료 상태를 표시하는지 확인했다. 각 단계에서 실패 상태를 설정하고 검사 반환값이 false면 성공 경로에 들어가지 않는다.
18. **비동기 환경 갱신의 상태 덮어쓰기** — runtime 검증 완료 뒤 environment lane이 일반 layout 상태로 교체하는 실제 화면 오류를 확인했다. route 전용 status를 보존하도록 수정했고 최종 화면 세 장에 30,000/30,000·52/52가 표시된다.
19. **draw 제출과 픽셀 readback의 혼동** — 화면 캡처나 draw log를 Metal drawable parity로 확대 해석할 위험을 확인했다. 앱 화면은 제출 증거로만 기록하며 문서와 evidence는 drawable readback을 미실행으로 명시한다.
20. **fixture ABI와 Xcode source inclusion 경계** — 새 parser가 Xcode Sources에 빠지거나 새 product FFI ABI를 요구하는지 확인했다. PBX file reference/build entry를 확인했고 기존 fixture host/report를 재사용한다. feature-on Simulator build log에는 기존 fixture-test flag가 기록된다.

## 수정 사항과 남은 경계

- 실제 실행에서 발견한 AppDelegate route 누락과 environment lane의 완료 상태 덮어쓰기를 수정한 뒤 새 PID 세 개로 다시 실행했다.
- 최종 실행의 run ID별 runtime log는 41 frame, 52 sample, 30,000/30,000 grid를 모두 귀속한다. parser positive/negative mutation, FFI 8/8, workspace tests, Clippy, rustfmt 및 feature-on Simulator build는 같은 실행 evidence 묶음에 연결했다.
- Simulator 실행은 실제 DOM event dispatch, WPT, iOS 실기기, Metal drawable readback 또는 전체 C13.1 지원을 입증하지 않는다.
