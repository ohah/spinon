# C13.3 pure Rust gesture session PR 변경 관점 30개

검토 대상은 이번 변경의 공개 Rust API, snapshot/session 경계, 회귀 테스트와 상태·계약 문서 delta다. 제품 native gesture 또는 앱 surface 동작을 승인하는 검토는 아니다.

| # | 변경 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 런타임 모듈 노출 | `runtime.rs`에서 session 모듈과 타입을 re-export하고 crate root `lib.rs`에서 외부 Rust 사용 경로를 제공한다. 통과. |
| 2 | 타입 파일 분리 | 입력·결과·상태 타입은 `types.rs`, 오류는 `errors.rs`, geometry key는 `geometry.rs`에 분리했다. 기능 소유권이 명확하다. 통과. |
| 3 | 오류 계약 | invalid sequence/identity/action/geometry와 renderer 오류를 구분하며 `Display`/`Error`를 구현한다. 조용한 `auto` fallback이 없다. 통과. |
| 4 | 웹 API와 혼동 가능성 | 문서와 타입 설명이 내부 Rust state API임을 밝히며 Pointer/Wheel/scroll event 또는 CSS 지원 선언으로 노출하지 않는다. 통과. |
| 5 | 입력 생성 API | `Begin`, pan sample, terminal 생성자는 expected render key와 표면/입력 identity를 받는다. 내부 필드는 외부에서 임의 부분 편집할 수 없다. 통과. |
| 6 | Begin snapshot 검증 | `begin`은 전달된 expected key와 실제 immutable snapshot key가 일치해야 active 상태를 만든다. stale key test가 있다. 통과. |
| 7 | sequence 및 gesture ID 수명 | 수락한 입력은 strictly increasing sequence를 요구하고 정상 Begin gesture ID는 0보다 크며 직전 accepted Begin보다 커야 한다. suppressed contact는 ID를 예약하지 않는다. 통과. |
| 8 | 시작 좌표 검사 | hit query 이전에 viewport CSS 좌표의 유한성을 검사하며 miss는 session을 만들지 않는다. 통과. |
| 9 | coordinate scale 의미 | scale은 finite positive인지 검증하고 active 동안 bitwise identity를 비교한다. CSS px 변환은 host 소유임을 API 문서에 적었다. 통과. |
| 10 | resolved touch-action 실패 처리 | 현재 제한값만 축으로 변환하고 `Unsupported`는 명시 오류로 반환한다. 시작 action이 gesture 동안 고정된다. 통과. |
| 11 | 시작 target 선택 | Begin 시점의 snapshot hit-test 결과 하나만 target으로 저장하고 후속 pan에서 retarget하지 않는다. 통과. |
| 12 | inclusive owner chain | target 자체가 scroll owner인 경우 맨 앞에 넣고 metadata ancestor 순서를 보존한다. 중복이나 없는 geometry는 거부한다. 통과. |
| 13 | frozen target geometry | target frame/hit clip/order와 viewport, clip layers, owner parent/scrollport/range를 geometry key에 넣는다. offset과 paint color를 불필요하게 key에 넣지 않는다. 통과. |
| 14 | native recognizer 경계 | tracking pan 입력은 offset을 바꾸지 않고 `pan_began` 호출 책임을 host의 slop/recognizer 결과로 제한한다. core가 임의 slop을 만들지 않는다. 통과. |
| 15 | dominant axis 결정 | magnitude가 큰 축을 고르고 동률은 두 축을 후보로 둔 뒤 frozen allowed axes와 교집합한다. 통과. |
| 16 | 거부된 pan 지속성 | 허용 축이 없으면 `PanRejected`로 남아 후속 sample이 축을 다시 latch하지 않는다. `pan-x`의 세로 방향 test가 있다. 통과. |
| 17 | accepted translation 기준 | first pan과 Move는 누적 translation을 직전 accepted sample에서 한 번만 차감한다. repeated cumulative input의 no-op을 확인한다. 통과. |
| 18 | 축 lock 유지 | 시작 시 고른 축만 후속 translation에 반영해 수직/수평 축 추가를 막는다. 통과. |
| 19 | router 재사용 | offset 적용을 새 별도 알고리즘으로 복제하지 않고 기존 immutable snapshot delta router에 위임한다. 통과. |
| 20 | router 원자성 및 오류 경로 | router 결과 snapshot만 받아 revision baseline을 전진시키며 router 오류는 active pointer를 취소·억제한 뒤 오류를 반환한다. 통과. |
| 21 | scroll revision ownership | 각 accepted delta 결과의 `ScrollStateRevision`을 다음 입력 baseline으로 저장하고 no-op이 revision을 올리지 않는 기존 계약을 따른다. 통과. |
| 22 | style-only snapshot 처리 | style revision 자체를 geometry key에 넣지 않아 geometry가 같으면 gesture를 유지한다. 시작 touch-action은 유지한다. 통과. |
| 23 | 갱신 overscroll 정책 | style-only 교체 뒤 다음 sample은 새 snapshot router policy를 사용한다. focused test에서 시작 action과 policy 갱신을 별도로 확인했다. 통과. |
| 24 | 문서/tree/environment 장벽 | generation, document/tree, environment 및 intrinsic geometry 차이를 현재 sample의 delta 적용 전에 판단한다. 통과. |
| 25 | 외부 scroll 장벽 | session이 소유하지 않은 scroll revision을 발견하면 늦은 delta 없이 취소한다. 회귀 test가 있다. 통과. |
| 26 | surface/geometry 장벽 | surface 또는 scale identity와 target/clip/owner/range geometry가 달라지면 취소하고 pointer를 terminal까지 억제한다. 통과. |
| 27 | 두 번째 pointer 정책 | 다른 pointer Begin이 들어오면 첫 pan을 취소하고 양쪽 contact를 보관한다. malformed second contact도 억제에 포함한다. 통과. |
| 28 | suppressed terminal identity | suppressed pointer를 gesture ID와 surface generation 양쪽으로 확인한다. 다른 surface의 terminal이 contact를 풀지 못하는 adversarial test가 있다. 통과. |
| 29 | terminal 및 늦은 sample | active terminal identity를 검사하고 중복/unknown terminal을 거부한다. 정상 cancel 뒤 늦은 Move는 offset을 바꾸지 않는다. 통과. |
| 30 | 테스트·문서·주장 범위 | focused 16/16, workspace 837/0/4, Clippy/rustfmt/staged diff check 로그를 연결했다. Android/iOS 앱 화면이나 C13.3 완료를 주장하지 않는다. 통과. |

현재 diff에서 미해결 P1/P2 결함은 발견하지 못했다. 남은 작업은 이번 session 내부 API 밖의 native queue/coalescing/terminal reservation, style profile·Android/iOS adapter 연결과 앱/displayed-frame 검증이다. 이 목록은 그 후속 gate를 대체하지 않는다.
