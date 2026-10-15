# C13.3 slop 전 tap arbitration 계획 실패 관점 검토 20개

검토 대상은 `plan/c13-3-scroll-gestures.md`의 “native slop 전 tap arbitration” 후속이다. 기능 구현 전에 고정 fixture·simulator 입력과 합격/실패 oracle을 확인했다.

| # | 적대적 관점 | 실패 경로 및 계획의 차단 근거 |
|---:|---|---|
| 1 | fixture route 누락 | tap flag만 일반 앱에 전달해 내부 test 동작이 켜질 수 있다. route 없이 flag를 주면 명시적으로 거부하고 default artifact에서도 전용 이름/코드/자료가 빠지는지 검사한다. |
| 2 | scenario 인자 중복·범위 오류 | 잘못된 scenario를 `auto`로 처리하면 none/pan-x oracle이 무효가 된다. 단일 3/4 선택을 검증하고 중복·잘못된 값은 ready 이전 실패로 기록한다. |
| 3 | 예상 computed action을 실제 값처럼 사용 | Android/iOS 실행 인자만 보고 recognizer 상태를 정하면 FFI style projection 불일치를 숨긴다. runtime ready report의 actual effective action을 검증한 뒤 연결한다. |
| 4 | draw 전 입력 | FFI ready 직후 첫 surface draw 전에 들어온 tap을 성공 처리할 수 있다. initial draw/revision/offset 및 fixture input-ready를 기다린 뒤 입력한다. |
| 5 | CSS point와 native point 혼동 | simulator density/point scale·letterbox 때문에 `(148,132)`이 inner scrollport 밖일 수 있다. 변환식, surface scale, bounds와 실제 hit/target 상태를 trace한다. |
| 6 | Android OnTouch가 click을 삼킴 | listener의 기존 `true` 반환은 clickable view의 ACTION_UP 처리와 `performClick`을 막는다. opt-in tap mode에서만 적합한 이벤트를 처리 후 `false`로 넘기고 실제 OnClick 호출을 센다. |
| 7 | OnClick이 scroll 이후 늦게 발생 | ACTION_UP 전에 후보를 세우면 pan이 끝난 뒤 delayed click이 tap으로 오인된다. 수락된 Up, pan 상태, 실제 OnClick을 하나의 gesture ID로 결합하고 candidate가 없으면 실패다. |
| 8 | Android ACTION_CANCEL | OS가 cancellation을 보냈는데 stale candidate가 다음 click을 세게 할 수 있다. cancel/배경/표면 종료에서 candidate와 gesture state를 함께 지우고 취소 후 새 tap recovery를 확인한다. |
| 9 | slop 직전 움직임을 pan으로 집계 | sub-slop jitter가 tap을 억제하면 임계 경계가 깨진다. native slop 값과 raw displacement를 로그로 남기고 tap case가 native pan을 시작하지 않았음을 별도 단언한다. |
| 10 | slop 초과 후 activation 누수 | Android clickable SurfaceView가 이동 종료를 click으로 처리할 수 있다. 실제 native pan 시작 뒤 activation count가 그대로인지 확인하고 OnClick 예기치 호출은 실패 처리한다. |
| 11 | `touch-action:none`의 Android native state 혼동 | OS 자체 click은 CSS 정책을 모르고, Rust session이 Begin/End만 받는 사실도 scroll 불변만으로 보장되지 않는다. tap 횟수와 pan callback, revision, 모든 owner offset을 독립적으로 확인한다. |
| 12 | pan-x의 방향 불일치 | 수직 swipe를 사용하면 올바른 거부를 tap/pan 결함처럼 보거나 반대로 허용할 수 있다. scenario 4는 X 방향, 양의 inner X range, slop 초과 delta를 고정한다. |
| 13 | iOS recognizer 동시 인식 | tap recognizer와 pan recognizer가 독립 인식하면 하나의 swipe가 tap과 pan으로 함께 집계될 수 있다. tap recognizer가 해당 pan recognizer의 failure를 기다리게 하고 두 인식 결과를 동시에 대조한다. |
| 14 | iOS `.began` 전에 첫 translation 누락 | UIKit `.began` zero translation을 pan delta로 처리하거나 tap으로 확정하면 첫 유효 이동이 소실된다. pan 시작과 tap 결과를 recognizer state 및 nonzero changed sample에 연결한다. |
| 15 | iOS `none` 비활성화 시점 경쟁 | computed style 확인 후 입력이 열린 다음 recognizer를 끄면 첫 pan이 이미 시작될 수 있다. main queue에서 fixture-ready/input gate보다 먼저 effective `none`에 따른 recognizer 상태를 구성한다. |
| 16 | scenario 4에서 pan recognizer까지 비활성화 | 설정을 하드코딩하면 `pan-x` swipe가 scroll하지 못한다. 실제 effective `pan-x`만 native pan을 유지하고 swipe 뒤 inner X offset/revision 증가를 요구한다. |
| 17 | 같은 화면 좌표의 stale target | 첫 tap 후 layout/scroll 변경 또는 다른 view overlay가 두 번째 swipe를 다른 owner로 보낼 수 있다. 실행 전후 generation/revision을 연결하고 예상 inner owner와 initial offset이 맞는지 검사한다. |
| 18 | activation 수와 callback 수 혼합 | 앱 status 문구나 자체 touch log가 실제 activation을 대신할 수 있다. Android는 View.OnClickListener, iOS는 UITapGestureRecognizer의 실제 handler 진입만 count한다. |
| 19 | 여러 번 호출되어도 합계만 맞음 | 한 tap 중복 2회와 다른 gesture에서 누락 1회가 총합으로 상쇄될 수 있다. gesture별 activation 수와 시나리오별 누적 count를 함께 저장한다. |
| 20 | 화면·로그가 다른 build를 가리킴 | 캡처가 소스나 실행 대상과 무관한 stale app일 수 있다. simulator ID/OS, app·source hash, route args, 초기/최종 screenshot와 원시 로그를 한 manifest로 묶는다. |

## 계획 검토 결과

20개 관점에서 고정 입력·실제 activation callback·recognizer arbitration·scroll 불변과 pan 허용·artifact 경계·증거 귀속을 분리해 확인했다. 이 계획은 tap 후속의 내부 fixture 합격 판정만 다룬다. 구현 후에는 별도의 새로운 실패 관점 20개와 PR 변경 검토를 수행해야 한다.
