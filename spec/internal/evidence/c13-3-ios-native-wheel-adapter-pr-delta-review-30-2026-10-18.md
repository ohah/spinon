# C13.3 iOS native wheel adapter · PR 변경 적대 검토 30회

**대상:** iOS adapter source, source regression tests, normalization oracle, plan, status/index 및 runtime evidence
**판정:** 아래 검토 결과는 현재 PR delta를 30개의 서로 다른 failure boundary로 대조한 기록이다. 실제 Simulator callback이 관찰되지 않은 조건은 정적 검토로 구분한다.

| # | 공격 관점 | 결과 및 근거 |
|---:|---|---|
| 1 | 기본 앱에 전용 launch marker 문자열이 남는가 | compile-time gated allowlist와 default executable strings 확인에서 marker 부재. 최초 검토 발견을 수정했다. |
| 2 | 기본 Rust archive에 wheel enqueue ABI가 남는가 | default archive `nm` negative control에서 `_spinon_runtime_gpu_host_enqueue_c13_3_wheel` 부재. |
| 3 | opt-in positive control이 없어 기능이 죽은 채로 경계 검사를 통과하는가 | opt-in executable와 archive에서 marker/symbol 존재 확인. |
| 4 | adapter와 preflight launch route가 동시에 활성화되는가 | route conflict 검사에서 동시 route를 거부한다. |
| 5 | gesture fixture route 없이 adapter 인자만 전달되는가 | route validator가 fixture route 없이는 진입하지 못한다. |
| 6 | discrete/continuous recognizer가 source를 교차 분류하는가 | recognizer identity가 `MouseWheel`과 `TrackpadScroll`로 각각 매핑된다. Discrete callback 자체는 미검증이다. |
| 7 | 손가락 pan이 두 recognizer에 중복 전달되는가 | 두 recognizer의 `allowedTouchTypes`가 빈 배열이다. 물리 touch 병행 실행은 미검증이다. |
| 8 | recognizer 하나의 누적 translation이 각 callback마다 다시 소비되는가 | recognizer별 이전 translation과 차분한다. |
| 9 | 첫 `.began` callback의 nonzero translation을 버리는가 | began state도 delta 계산에 포함한다. Simulator callback에서 nonzero began sample을 관찰했다. |
| 10 | ended callback의 같은 translation이 추가 scroll을 만드는가 | zero 차분은 enqueue 전에 거부하며 terminal state에서 baseline을 지운다. Simulator에서 `zero-delta`는 관찰했지만 그 zero sample의 terminal state는 log에 없어 해당 세부 동작은 정적 확인이다. |
| 11 | cancelled/failed 상태 뒤 이전 baseline이 재사용되는가 | 모든 terminal state에서 baseline cleanup 경로를 호출한다. Runtime cancel injection은 미실행이다. |
| 12 | generation 변경 뒤 기존 surface 좌표계의 delta가 새 화면에 들어가는가 | baseline에 generation을 저장하고 layout callback에서는 baseline을 조기 삭제하지 않는다. 다음 callback에서 rebase한다. Runtime resize 주입은 미실행이다. |
| 13 | background→foreground 뒤 누적 input이 섞이는가 | background 시 recognizer를 재설정하고 translation map을 비운다. Lifecycle runtime stress는 미실행이다. |
| 14 | shutdown 중 닫힌 handle을 enqueue하는가 | callback에서 ready/closing/handle/generation/viewport를 잠금 범위에서 함께 읽고 ready/nonzero handle을 요구한다. |
| 15 | 0·음수·비유한 viewport scale로 좌표가 왜곡되는가 | 각 축 scale의 finite/positive 조건과 균일성 허용치를 검사한다. 현재 Simulator scale만 1.0이다. |
| 16 | canvas 바깥 위치 또는 max edge 입력이 유효점이 되는가 | half-open bounds 검사 뒤 CSS point를 만들며 바깥점은 FFI 전 reject한다. Runtime 바깥점 입력은 미실행이다. |
| 17 | NaN/infinite location·translation이 FFI에 도달하는가 | native·normalized sample의 finite 확인을 enqueue 전에 수행한다. |
| 18 | zero 또는 중복 delta가 revision을 증가시키는가 | callback zero는 FFI 전 거부한다. 로그에서 duplicate callback `zero-delta`를 관찰했다. |
| 19 | CSS wheel sign이 UIKit translation과 뒤집혀 전달되는가 | `deltaCss=-translationDelta/scale`; negative-Y와 positive-X sample을 queue에서 확인했다. 전체 ±X/±Y 결론은 내리지 않는다. |
| 20 | raw unit, scale, source unit이 서로 불일치하는가 | raw=`UIKitPoints`, source unit `2`, scale=`points/CSS px`; observed report scale은 1.0이다. |
| 21 | modifier bit가 Rust contract와 다른 순서인가 | Shift/Control/Alt/Meta를 1/2/4/8로 정규화한다. Runtime modifier 주입은 미실행이다. |
| 22 | Control-wheel이 zoom intent를 ordinary scroll로 queue하는가 | Control bit guard가 enqueue 전 실행된다. iOS Control-wheel runtime negative는 미검증으로 남긴다. |
| 23 | UIKit gesture ended를 momentum end로 오인하는가 | UIKit sample은 항상 phase `None`이다. |
| 24 | queue reject나 malformed report에도 worker drain을 예약하는가 | `status=0`과 `admitted=true` 둘 다 만족해야 drain한다. Rejection stress는 미실행이다. |
| 25 | 받아들인 sample이 drain 예약 없이 유실되는가 | 관찰 positive-X sample에서 queue accepted와 `processed_count=1`이 연속 보고됐다. |
| 26 | queue 이후 target이 현재 hit point와 다른 owner로 간다 | Simulator positive-X는 target 7에서 inner owner만 변경했다. Negative-Y target 5는 boundary no-op이었다. |
| 27 | XCTest UI pass를 네 방향 native callback 통과로 잘못 승격하는가 | evidence는 UI 4/4와 native callback 2/4를 별도로 보고한다. |
| 28 | fixture screenshot만으로 scroll 변화나 callback을 주장하는가 | 첨부 장면은 캡처 실행에서 callback 미관찰임을 evidence와 PR에 명시한다. |
| 29 | 이전 host `loginwindow` preflight가 최신 결과와 충돌하는가 | 이전 preflight를 historical section으로 보존하고 후속 callback 결과를 별도 날짜 섹션에 기록한다. |
| 30 | Simulator PoC 때문에 C13.3 공식 체크 또는 완료 집계가 올라가는가 | `spec/STATUS.md`는 부분 서술만 추가하고 C13.3/C13 checkbox는 그대로 미완료로 둔다. |

## 남은 제한

검토는 native discrete callback, Control-wheel runtime, resize/background injection, 물리 입력, public DOM event, screenshot pixel diff 또는 C13.3 전체 완료를 입증하지 않는다. Review에서 수정한 feature-off launch marker 누출은 opt-in/default artifact를 다시 빌드해 확인했다.
