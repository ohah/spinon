# C13.1 hit identity PR delta 변경 관점 검토 · 30개 · 2026-10-14

검토 범위는 PR #158의 host test 보강, agent-browser observation, content-box fallback control, 관련 plan·contract·status 갱신이다. 이 기록은 [구현 실패 관점 20개](implementation-review-20.md)와 별도다.

| # | 변경 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 제품 경로 변경 여부 | `assert_hit_identities_match_chrome`는 test 모듈 안에서만 컴파일된다. renderer 제품 함수는 바뀌지 않았다. |
| 2 | matrix row 수 과장 | Chrome capture는 target 보유 14 rows, 그중 invalid margin 1개는 runtime test 대상에서 제외하고 valid 13개만 검사한다. |
| 3 | float row 누락 | 세 float case는 원본 fixture에 일반 target이 없어 identity capture에서 생략된다. 기존 layout fail-closed test를 근거로 연결하고 surface pass를 주장하지 않는다. |
| 4 | invalid CSS를 정상 runtime 행으로 취급 | `clip-margin-invalid`는 Chrome capture에만 보이고 Spinon은 diagnostic으로 layout 전에 거부한다. |
| 5 | 다른 Chromium으로 oracle 교체 | browser product/version/revision과 agent-browser 버전을 새 JSON에 기록했다. |
| 6 | UA 드리프트 | helper가 raw UA를 고정 문자열과 비교한다. |
| 7 | viewport 드리프트 | helper가 너비, 높이, DPR를 개별 비교한다. |
| 8 | source fixture 변경 | helper가 source fixture digest와 선행 17-row observation digest를 비교한다. |
| 9 | capture script와 data 불일치 | capture script가 target frame, row ID, 축 fixed point 및 grouped topmost runs를 한 payload로 반환한다. |
| 10 | run data의 구간 손실 | test가 첫 시작·구간 연결·마지막 끝을 확인해 빈 integer 좌표나 중복 run을 거부한다. |
| 11 | hit identity를 bool로 축소 | 새 helper는 Chrome label에 따라 target, clip parent, nested inner 또는 root NodeId를 비교한다. |
| 12 | DOM wrapper 구조 차이 은폐 | transparent `main`/`body`/`html`은 단일 synthetic root로 묶는 경계를 문서화했다. 그 wrapper별 DOM hit를 주장하지 않는다. |
| 13 | NodeId 순서가 바뀌어 비교가 무효 | call site가 각 scene의 root/clip/inner/target node를 명시적으로 전달한다. |
| 14 | nested inner-node 기대가 다른 scene에 새어 들어감 | inner label은 `Some(innerNode)`가 전달된 nested row에서만 허용한다. |
| 15 | target rect가 observation과 달라도 identity만 pass | helper가 captured target x/y/width/height를 runtime frame과 0.5 CSS px 안에서 비교한다. |
| 16 | 정수 끝 좌표를 inclusive/exclusive로 혼동 | captured frame에서 `floor(start)` 및 `ceil(edge)-1`로 run extent를 검증한다. |
| 17 | fixed orthogonal coordinate drift | 각 축의 `fixedAt` 값은 Chrome payload에서 읽고 해당 위치만 검사한다. |
| 18 | `clip-x`/`clip-y` 축 분리 regress | 기존 x/y 축 행 모두 exact identity assertion에 연결한다. |
| 19 | border-strip in-flow regression | positioned in-flow selected rows에서도 Chrome box/root fallback을 유지하는지 검사한다. |
| 20 | contained absolute regression | target→clip box→root 전이 지점을 Chrome 값과 Rust NodeId로 비교한다. |
| 21 | outer containing-block escape regression | escape row의 target frame 내 target identity가 계속 보존되는지 검사한다. |
| 22 | content-box test의 부정 표본이 약함 | 이전 `is not target` 검사를 parent/root exact identity로 바꿨다. |
| 23 | content-box positive control 손실 | x=119는 target node인 것을 계속 assert한다. |
| 24 | parent border 끝점 해석 오류 | x=120/122/123은 clip node, half-open right edge x=124는 root를 요구한다. |
| 25 | old Chrome content-box observation이 fixture mismatch | browser viewport, revision, fixture digest, frame과 hit range를 같은 controlled HTML에 연결했다. |
| 26 | Android 결과를 새 identity test로 확대 | 새 delta는 host Rust test다. 이전 AVD run은 별도 evidence이고 신규 geometry를 실행하지 않았다. |
| 27 | 범위가 event dispatch 완료처럼 보임 | test는 `RuntimeRenderSnapshot` NodeId만 조회하고 DOM event 전파는 남은 항목으로 둔다. |
| 28 | 1D 결과를 2D matrix라 호칭 | README에 각 축 sweep이며 전체 2D scan이 아님을 명시했다. |
| 29 | 저장소 문서 상태가 결과와 다름 | plan, 내부 계약, C13 status를 17행 runtime disposition(13 valid + 4 reject/fail-closed)과 surface/event/WPT remaining gates로 동기화했다. |
| 30 | PR lifecycle 혼동 | PR #158은 draft/open 상태로 유지하며 merge나 public CSS 완료 주장을 하지 않는다. |

검증 로그는 matrix 8/8, content-box control 1/1, render crate 52 passed·1 ignored, Clippy·rustfmt·diff check다. 추가 Android build는 제품 코드가 바뀌지 않아 실행하지 않았다.
