# C13.1 iOS Simulator integration · PR 변경 delta 관점 30개

검토 범위는 기존 C13.1 Android/runtime 변경 전체가 아니라 이번 iOS Simulator bridge delta와 관련 문서다. 구현 전 기준, 실제 iOS execution, default/opt-in archive 대조를 별도로 확인했다. 이 PR은 draft이며 merge하지 않는다.

| # | PR delta 공격 관점 | 결과 |
|---:|---|---|
| 1 | build env 변수 오타로 opt-in이 무시된다 | PASS — public env `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST`와 Xcode용 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST` 변환을 빌드 로그에서 확인했다. |
| 2 | 신규 flag 기본값이 Debug에서만 0이다 | PASS — Debug와 Release target configuration 모두 default 0을 둔다. |
| 3 | Release 설정에서 새 C macro 정의가 빠진다 | PASS — Debug/Release preprocessor definition 모두 새 macro를 전달한다. |
| 4 | mise 설치 여부가 build script 동작을 바꾼다 | PASS — 양 xcodebuild 분기에 신규 flag를 전달한다. |
| 5 | C13 flag가 C04 없이 성공 처리된다 | PASS — simulator/native prerequisite negative controls가 모두 exit 2였다. |
| 6 | invalid flag value가 조용히 허용된다 | PASS — `2` 입력을 simulator build script가 exit 2로 거부했다. |
| 7 | native build만 feature를 빼고 link-time unresolved를 만든다 | PASS — native script가 C13 flag=1일 때 전용 Cargo feature를 붙이고 opt-in iOS 앱이 링크됐다. |
| 8 | 전용 feature 대신 모든 C04 build에 C13 payload가 들어간다 | PASS — C04-only/C13=0 build log에는 C13 feature가 없었다. |
| 9 | Cargo 기본 archive에 C13 symbols가 존재한다 | PASS — feature 인자 없는 iOS `spinon-ffi` archive에서 두 C ABI symbol을 찾지 못했다. |
| 10 | Cargo 기본 archive에 fixture JS/Chrome oracle 문자열이 존재한다 | PASS — default archive `strings` negative control이 통과했고 opt-in archive에서는 payload 문자열을 확인했다. |
| 11 | feature-on archive에서 host 함수만 있고 eval 함수가 빠진다 | PASS — opt-in archive에서 host/eval symbols 두 개를 확인했다. |
| 12 | Apple `nm`의 LLVM 22 reader 오류를 symbol 없음으로 오인한다 | PASS — Rust 1.96.1과 일치하는 LLVM 22 `llvm-nm`으로 검사했다. 사용한 도구와 결과를 evidence에 기록했다. |
| 13 | C ABI prototypes가 C13 macro보다 넓은 범위에 노출된다 | PASS — 기존 header guard 아래 전용 선언을 유지한다. |
| 14 | ObjC++ host route가 feature-off에서 미해결 Rust symbol을 참조한다 | PASS — C04와 C13 macro가 모두 참일 때만 extern C 함수에 참조한다. default archive로 확인했다. |
| 15 | ObjC++ eval route에 C13 전용 guard가 없다 | PASS — eval도 같은 C04/C13 macro pair를 확인한다. |
| 16 | 앱 실행 인자가 controller 진입을 유발하지 않는다 | PASS — `AppDelegate` route에 C13 flag를 추가했고 simulator app을 이 인자로 시작했다. |
| 17 | C13 route와 일반 C0410 route 간에 controller 상태가 바뀐다 | PASS — controller 내부 fixture selector와 고정 canvas/viewport를 C13 flag로 연결한다. |
| 18 | C12.3/C11 선택이 C13보다 먼저 host를 만든다 | PASS — host selection에서 C13을 먼저 선택하고 충돌 argument는 사전 차단한다. |
| 19 | collision check가 base `--spinon-c0410-runtime-gpu`도 거부한다 | PASS — C0410 base 인자는 허용 목록에 있고 C13 전용 fixture와 공존할 수 있다. C12.3 collision negative 실행은 별도로 저장했다. |
| 20 | C13 route viewport가 기존 화면 크기/고정 fixture 크기와 혼동된다 | PASS — 320×240 CSS viewport와 320×240 point canvas를 고정했다. 실행 환경 log가 그 값을 기록했다. |
| 21 | device scale이 CSS geometry 비교에 적용된다 | PASS — device scale 3은 surface texture 960×720에 반영되고 Chrome 비교는 CSS frame을 비교했다. |
| 22 | initial environment empty layout을 최종 결과로 쓴다 | PASS — 최종 gate는 뒤이은 C13 eval의 `layout=ready boxes=13` 결과와 별도 frame list를 검사한다. |
| 23 | runtime report의 성공 상태지만 node frame list는 누락된다 | PASS — Swift summary helper가 marker와 frame count 13을 요구한다. |
| 24 | frame list는 13개지만 node ordering 또는 geometry가 다르다 | PASS — 독립 Python 대조가 index/node order와 13×4 geometry 값을 Chrome에 비교했다. 오차는 0 CSS px였다. |
| 25 | hit sample JSON이 malformed거나 배열 수가 다르다 | PASS — Swift JSON decode, fixed 15-ID sequence, 15/15 matched gate가 있고 실제 app log에 통과 summary가 남았다. |
| 26 | sample이 같은 ID라도 좌표/element가 바뀐다 | PASS — 독립 비교기는 각 좌표·element ID를 pinned Chrome observation과 대조했다. |
| 27 | viewport 밖 negative control을 성공 hit로 잘못 기록한다 | PASS — 네 outside sample이 Chrome expected `null`과 일치하고 all-sample matched를 통과했다. |
| 28 | Simulator screenshot이 Metal surface present를 뜻한다고 과장된다 | PASS — 실행 로그는 Metal backend/draw 제출, PNG는 화면 캡처라고 구분하고 drawable readback/compositor/scanout은 주장하지 않는다. |
| 29 | 문서가 iOS integration을 마친 뒤 C13.1/C13을 완료로 승격한다 | PASS — STATUS, plan, contract는 iOS fixture gate만 완료로 적고 C13.1/C13 및 CSS 지원은 미완료로 유지한다. |
| 30 | PR metadata상 draft/no checks를 merge-ready로 표현한다 | PASS — 이 delta를 기존 PR #158 draft에 추가한다. 현재 `gh pr checks`는 no checks reported이며 merge하지 않는다. |

위험/제한: 이 PR은 별도 build flag가 있는 bootstrap fixture를 추가한다. iOS Simulator geometry와 고정 hit query는 확인했지만 drawable pixel readback, 최종 screen scanout, WPT 적합성, CSSOM scroll/event, C13 전체 지원은 확인하지 않았다.
