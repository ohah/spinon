# C14.2 live V8 app fixture · 구현 실패 관점 검토 20개

2026-10-16 현재 구현 코드·Rust tests·Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 서로 다른 실패 경로로 검토했다. 계획 검토 및 기존 C14.2 리뷰 표는 재사용하지 않았다. 발견한 문제는 수정하고 영향받는 항목을 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | Chrome 버전·viewport·DPR가 바뀌어 다른 oracle을 통과 처리 | `verify_oracle_header`가 Chrome `155.0.8059.40`, 1280×633, DPR 1을 강제한다. 두 플랫폼 report가 같은 oracle 이름을 기록했다. |
| 2 | Block denominator가 natural PNG 전체 22행으로 늘어남 | mapping test는 inventory의 미표시 21행과 정확히 일치한다. `aspect-ratio-auto-override`는 `layoutStage` 표시 때문에 제외된다. |
| 3 | Flex case 누락·순서 변경 또는 parent ID만 일치 | mapping test가 고정 8개 Flex 순서를 대조하고, V8 snapshot adapter가 parent class/style와 단일 img child를 검사한다. |
| 4 | 고정 PNG가 변했는데 자연 크기 결과만 우연히 일치 | Block/Flex inventory hash, embedded source bytes 및 24×16 metadata를 각각 검사한다. opt-in binary에도 동일 PNG byte sequence가 있다. |
| 5 | live V8 대신 합성 HostDocument가 비교에 사용됨 | Block·Flex 각각 `RuntimeSession::eval` 후 `snapshot_document_for_c14_1_fixture`를 사용한다. report `document_origin=live-v8`이며 두 generation은 독립적이다. |
| 6 | 다른 문서의 image source 또는 stale revision을 섞음 | 각 image request의 URL과 `LayoutSourceRevision::HostDocument`를 확인하고 resource·intrinsic snapshot source stamp를 비교한다. 두 report에서 source match가 참이다. |
| 7 | CSS source가 원본 fixture의 고정 author stylesheet와 달라짐 | fixture HTML style block을 전달하고 limited UA profile이 제공하지 않는 `<main>`의 Chrome 기본 `display:block`만 추가로 명시한다. 두 document의 style diagnostics는 비어 있다. |
| 8 | `aspect-ratio` 속성이 profile에 없어 두 ratio case가 빗나감 | 첫 27/29 실행에서 `aspect-ratio-override`와 `aspect-ratio-border-box`가 구체적으로 실패했다. C14.2 `aspect-ratio`와 Flex sizing을 가진 Flex custom-properties profile로 수정한 뒤 두 플랫폼 모두 통과했다. |
| 9 | JSON 자연 크기 `{width,height}`를 배열로 읽어 조기 실패 | 두 차례 simulator 실패로 드러난 parser를 `json_size` 객체 parser로 고쳤고, Chrome oracle 29행을 읽는 전용 unit test 4/4 및 양 플랫폼 gate 29/29를 확인했다. |
| 10 | `max-width:20px` oracle의 원래 소수 좌표를 정수로 반올림해 oracle을 덮어씀 | report에 `19.984375×13.328125` 기대값을 유지하고 frame field마다 inventory tolerance 0.05 CSS px를 적용한다. 양 플랫폼 report가 원시 float와 actual frame을 보존한다. |
| 11 | Block 이미지 위치만 비교해 Flex parent 또는 stretch 오류를 숨김 | 모든 image case의 x/y/width/height를 비교한다. 고정 Chrome Flex observation이 제공하는 parent width/height와 `align-items`를 8개 전부 대조하고 실제 parent frame은 report에 보존한다. Chrome oracle에 parent x/y는 없음을 계획에 명시했다. |
| 12 | case 누락·중복·잘못된 식별자가 상위 count만 맞춰 통과 | Rust mapping 순서, 앱 report의 case count, 플랫폼별 21/8 count 및 고유 ID를 모두 검사한다. 양 report에는 실패/중복 ID가 없다. |
| 13 | source/style/environment/intrinsic revision이 다른 시점을 섞음 | 문서별 generation/document/render-tree/style/environment/intrinsic revision과 output stamp를 기록한다. Android·iOS 각각 두 문서 stamp가 통과했다. |
| 14 | JSON report가 native↔Java/Swift 전송 중 잘려 성공처럼 파싱 | FFI report writer는 bounded buffer 오류를 반환하고, 각 플랫폼은 전체 29행 JSON parse와 count/status를 다시 검증한다. Android/iOS log chunks를 재조립해 각각 29행을 독립 검산했다. |
| 15 | Android JNI가 null pointer·빈 buffer·출력 길이 경계를 위반 | JNI는 65,536 byte buffer를 전달하고 report를 UTF-8 byte array로 반환한다. opt-in Android 실행에서 8개 chunk·29행 report를 소비했다. |
| 16 | iOS native report 문자열 변환·로그 분할 실패 | Objective-C++는 65,536 byte buffer를 사용하고 UTF-8 변환 실패 시 명시적 결과를 돌려준다. Swift는 JSON·행 수·ID·finite frame을 재검증한다. 36개 unified-log chunk를 재조립했다. |
| 17 | 잘못된 route 인자 또는 다른 fixture 동시 실행으로 애매한 상태 | Android extras와 iOS `--spinon-*` 인자를 검사하고 충돌 시 계산 전에 오류 화면/log로 닫는다. default route도 두 플랫폼에서 fixture 실행 없이 disabled 처리했다. |
| 18 | C14.2 ABI·JS/JSON/PNG payload가 기본 artifact에 유입 | Android/iOS opt-in에서 symbol·label·case ID·PNG bytes 양성 대조를 확인했다. 기본 archive/native/app에서 네 항목이 모두 없고 route는 `SPINON_C142_DISABLED`를 기록했다. |
| 19 | 작은 Simulator 화면에서 경계 수치가 앱 화면 밖으로 잘리거나 실행 상태가 숨음 | 두 성공 screenshot에서 `PASS 29/29`, 21/21·8/8 및 content-box/aspect-ratio/Flex 경계 수치가 보이고 텍스트가 화면 안에서 줄바꿈된다. |
| 20 | sizing report를 image paint, border overflow clip 또는 제품 loader 검증으로 과장 | 화면은 text report만 표시한다. border/padding이 포함된 CSS frame geometry는 통과했지만 native image pixels·border 바깥 overflow/clip·제품 resource loader는 이번 실행에서 검사하지 않았다고 evidence와 README에 분리 기록했다. |

최종 확인: opt-in FFI tests 4/4, 전체 workspace tests 통과, workspace Clippy 통과, C14.2 opt-in Clippy 통과, Rust formatting·build-script syntax·diff check 통과, Android/iOS opt-in builds 통과, 양 플랫폼 live V8 runtime 29/29. 해결되지 않은 P1/P2 없음.
