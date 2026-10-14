# C13.1 border-strip iOS Simulator 확장 PR 변경 관점 검토 30개 · 2026-10-14

검토 범위는 PR #158에 추가되는 iOS 전용 border fixture route, gated FFI entry points, evidence, 기존 route 회귀 및 PR 주장의 경계다. 이미 기록한 C13.1 painter/hit-order PR 검토와 구분한다.

1. **제품 기능 과장** — opt-in test fixture를 CSS 지원으로 설명하는가? PR과 STATUS는 C13.1 제한 profile/fixture로 한정한다.
2. **기본 Cargo feature 변경** — 신규 fixture가 기본 빌드로 켜지는가? feature 선언은 `c13-1-overflow-clip-fixture-test` opt-in에 종속된다.
3. **모듈 경계 우회** — fixture include가 gated `fixtures::c13_1` 외부에 추가되는가? source include와 FFI 모듈 경계 테스트가 feature module을 확인한다.
4. **default archive 누출** — off archive에 border API symbol이 있는가? matching LLVM 22 `llvm-nm` 검사에서 두 신규 C symbol 모두 없다.
5. **source payload 누출** — off archive에 고유 JS marker/query가 있는가? `strings` negative control에서 marker와 sample ID가 없다.
6. **positive-control 무력화** — opt-in 자체도 symbol을 포함하지 않는 잘못된 검사인가? feature-on archive는 두 API와 세 고유 fixture 문자열을 포함한다.
7. **feature 의존성 미스매치** — C13 fixture가 C04 runtime fixture 없이 활성화되는가? Cargo feature는 C04 runtime fixture feature를 함께 요구한다.
8. **헤더 선언 누락** — iOS ObjC++가 헤더에서 함수를 보지 못하는가? opt-in header 블록에 양쪽 function prototype이 있다.
9. **Rust/C symbol 오타** — 선언, Rust export, ObjC++ call 간 symbol 이름이 다른가? positive archive의 exported names와 호출을 직접 대조했다.
10. **C API ABI 변화** — 기존 symbols나 struct layout을 바꾸는가? delta는 새 constructor/eval export뿐이며 공유 host ABI layout을 바꾸지 않는다.
11. **FFI 호출 안전 주석** — 새 unsafe entry point의 host lifetime/threading 규칙이 빠졌는가? eval safety docs에는 valid handle, concurrent free 금지, wait-safe executor, writable buffer가 적혀 있다.
12. **C constructor 오류 전달** — runtime host 생성 실패가 가짜 non-null handle이 되는가? constructor는 오류를 caller buffer에 쓰고 null을 반환한다.
13. **C eval 인수 실패** — null host나 invalid output이 dereference되는가? early argument checks와 focused null/invalid-output test가 유지된다.
14. **출력 크기·호출 완료** — 고정 32 KiB report에 여섯 sample이 잘리는가? Simulator FFI eval report는 모든 sample과 종료 bracket까지 parse되고 6/6 판정됐다.
15. **V8 없는 환경 해석** — host-native isolate failure를 runtime 제품 실패 또는 통과로 섞는가? evidence에서 macOS isolate 생성 오류와 iOS Simulator runtime을 분리했다.
16. **report 생애주기 혼합** — border expected samples가 environment resize report에도 붙는가? report flag는 border fixture host에서만 켜고 environment publication 호출은 false로 전달한다.
17. **다른 fixture host 오염** — common `RuntimeGpuHost` 기본값이 border report를 활성화하는가? field는 default false이며 전용 constructor만 true로 설정한다.
18. **AppDelegate route 미연결** — 신규 argument로 generic demo가 열리지 않는가? app launch가 신규 argument를 runtime route로 분류하고 PID를 반환했다.
19. **Swift/ObjC declaration drift** — Runner selector 선언·정의가 어긋나는가? header selector와 ObjC++ implementation은 동일하다.
20. **route 충돌 우선순위** — 두 C13 route를 같이 넘겼을 때 silent precedence가 생기는가? Simulator negative control에서 충돌을 기록하고 host 생성 전에 반환했다.
21. **다른 CSS fixture 조합** — Grid/position route를 추가로 전달해 scene이 섞이는가? 공통 C13 argument guard가 비허용 CSS fixture 인수를 거부한다.
22. **viewport 오차** — query 좌표와 surface viewport가 서로 다른 CSS 단위인가? 전용 surface constraint와 runtime environment가 400×160 CSS px이며 fixed integer points를 사용한다.
23. **frame 수만 맞는 거짓 통과** — 다섯 frame geometry/순서가 달라도 성공하는가? Swift는 5개 ordered frame의 모든 좌표를 oracle과 비교한다.
24. **sample labels 신뢰** — id는 맞지만 target/point/status가 다른 report를 성공 처리하는가? Swift가 여섯 field tuple을 정해진 순서로 모두 검사한다.
25. **기존 route 회귀 누락** — 공통 Objective-C logger의 expected count parameter가 15-query path를 깨뜨리는가? 같은 binary의 기존 앱 경로가 13 frames·15/15 queries를 다시 통과했다.
26. **화면 증거 범위 확대** — screenshot을 drawable readback이나 scanout으로 주장하는가? evidence는 app report/Simulator 화면과 drawable readback 부재를 분명히 구별한다.
27. **플랫폼 parity 과장** — iOS Simulator 결과를 Android evidence로 일반화하는가? Android의 신규 border coordinates는 남은 gate로 문서화했다.
28. **테스트 matrix 축소 은폐** — 6-point route가 전체 C13.1/WPT 완료로 표시되는가? STATUS와 계약은 C13.1 전체 matrix, event breadth, WPT가 남았다고 유지한다.
29. **현재 수정의 회귀 검증** — 변경 뒤 cargo test/Clippy/module boundary를 생략했는가? workspace 733개 테스트, feature-gated FFI boundary test, Clippy, rustfmt, Node boundary 5/5를 수집했다.
30. **worktree·외부 세션 영향** — shared checkout/Simulator/tmux를 닫거나 바꾸는가? 변경은 PR worktree에 있고 app process만 종료했다. Simulator는 booted로 남겼으며 실행 중인 attached `spinon` tmux pane은 현재 C11 Codex 작업이라 건드리지 않았다.

## 판정

이 PR delta에서 추가 수정이 필요한 결함은 발견하지 못했다. iOS Simulator runtime route와 feature-on/off archive control은 확인했다. Android 신규 route, Metal drawable readback, DOM event dispatch, compositor/scanout, 전체 C13.1/WPT gate는 PR 범위와 완료 주장 밖이다. 기존 C13.1 구현 review와 이 앱 integration delta review는 별도 자료로 유지한다.
