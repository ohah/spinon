# C13.3 구현 전 Chrome gesture 비교 · 2026-10-15

## 판정

고정 Chrome 155 Agent Browser session에서 touch pan, wheel delta, per-axis overscroll, touch-action, pointer cancel 및 two-pointer input을 기록했다. 기준 fixture와 reference JSON은 [source](../../../../tests/fixtures/css/c13/c13-3-scroll-gesture.html), [capture script](../../../../tools/css-reference/capture-c13-3-scroll-gesture-precomparison.mjs), [reference test](../../../../tools/css-reference/c13-3-scroll-gesture-precomparison.test.mjs)에 있다.

이 결과는 Chrome oracle일 뿐 Spinon의 runtime/app 입력, public Pointer/Wheel/scroll event, Android/iOS 동작 근거가 아니다. Android API 37 ARM64 AVD와 iOS Simulator는 이 사전 비교에서 실행하지 않았다. 표준 draft와 Chrome의 차이는 예상값에 덮지 않고 아래에 분리했다.

![Chrome 155 nested scroll gesture oracle fixture, 앱 화면 아님](./chrome155-c13-3-scroll-gesture-400x240.png)

## 환경 및 원본

- Chrome for Testing `155.0.8059.40`, WebKit revision `cfaadc5a132d78e1828635aa8405a499f3e14864`.
- agent-browser `0.31.1`, macOS arm64, viewport `400×240` CSS px, DPR 1, mobile/touch emulation 2 contacts.
- direct touch/wheel input은 agent-browser가 연 fixture tab의 CDP target에 전달했다. browser session은 캡처 뒤 닫았다.
- fixture SHA-256: reference JSON `fixture.sha256`; screenshot은 RGB `400×240`, SHA-256은 `screenshot.sha256`에 기록했다.
- 재생성: `bun run css:reference:c13-3-scroll-gesture-precomparison`; 확인: `bun run test:css-c13-3-scroll-gesture-precomparison`.
- local product source scan에서 `touch-action` 및 `overscroll-behavior` runtime projection은 아직 없었다. 이 단계는 해당 property의 제한 typed projection을 계획한다.

## 관찰값

| 입력 | Chrome 결과 |
| --- | --- |
| inner 시작점, 60px vertical/horizontal pan | inner `(x,y)=(0,45)` / `(45,0)` |
| inner 시작점, 60×60 diagonal | inner `(49,49)` |
| inner y=`86`에서 상방 180px | inner y=`106`에 clamp, outer y=`0`; 같은 gesture의 남은 touch delta는 outer로 가지 않음 |
| inner y max에서 상방 60px 시작 | `auto`: outer y=`45`; `contain`/`none`: outer y=`0` |
| inner overflow `hidden`/`clip`/`visible` | inner offset `(0,0)`, outer y=`45`. `hidden`은 programmatic-only boundary, `clip`/`visible`은 자체 owner 없이 ancestor로 전달된다. |
| `touch-action:none` 사전 설정 / touchstart 뒤 none 적용 | 사전 설정은 offset `(0,0)` 유지; active gesture 중 변경은 y=`45` scroll을 중지하지 않음 |
| `pan-x`/`pan-y` 반대 축 단독 pan | offset 변화 없음; 허용 축 pan은 inner offset을 바꿈 |
| inner-content `pan-x` | vertical pan은 모든 owner `0`; horizontal pan은 inner x=`45` | target과 nearest scroll owner 사이 supported action 교집합을 확인한다. 바깥 `outer` action은 inner scroll target의 nearest owner 경계 밖이다. |
| pan 시작 뒤 `touch-action:none` 변경 / `touch-action:none` 상태의 wheel | active pan은 inner y=`45`를 계속 소비; wheel은 inner y=`40` 이동 | active action freeze와 wheel 입력 분리를 확인한다. |
| auto horizontal-start 뒤 vertical direction change | x는 양수, y=`0`; Chrome이 첫 horizontal axis를 유지 |
| inner와 outer y max에서 pan 시작 | inner=`106`, outer=`186`, root y=`45` | 두 ancestor를 지나 root까지 남은 delta를 전달한다. |
| touchcancel 후 late move 요청 | cancel까지 inner y=`15` 유지. CDP가 후속 touchMove를 `TouchStart first` 오류로 거부 | 이 거부는 browser harness가 입력을 전달하지 못한 결과다. late queued Move 거부는 Spinon core/Simulator gate에서 따로 검증한다. |
| two-pointer simultaneous start→move→cancel | offsets `(0,0)`, visualViewport scale `1`, cancel 뒤 pointercancel 두 개. 이후 새 단일-pointer pan은 inner y=`30` | 두 contact 시작과 복구를 기록한다. 이미 active인 pan 중 두 번째 pointer 삽입은 앱 gate에서 확인한다. |
| X=`contain`, Y=`auto`, X-only/Y-only 입력 | outer X=`0`; Y는 vertical touch=`45`, wheel=`40` |
| X=`contain`, Y=`auto`, 대각 touch/wheel | Chrome에서 outer `(0,0)` |
| X/Y=`auto`, 대각 wheel `(40,40)` | outer `(40,40)` |

`C13.3` 계획은 각 축을 독립 route하는 계약을 선택한다. 따라서 X contain, Y auto 대각 입력의 구현 pass는 outer X=`0`, outer Y>0이어야 하며 Chrome의 `(0,0)`은 oracle discrepancy다. [Overscroll Behavior 초안](https://www.w3.org/TR/css-overscroll-1/)은 동시에 scroll되는 축별 동작을 독립 고려하도록 적고 있다. 문서는 2019 FPWD이며, UA에 chaining/bounce를 구현하도록 의무화하지 않는다.

`touch-action`은 [Pointer Events](https://www.w3.org/TR/pointerevents/#the-touch-action-css-property)의 시작 시점 동작을 따른다. active gesture 중 값 변경을 소급하지 않으며, 제한 profile에서는 `auto`/`none`/`pan-x`/`pan-y`만 지원 대상으로 제안한다. headless 화면에는 native boundary affordance가 없으므로 iOS bounce/Android glow는 관찰하지 않았다.

## 남은 경계

- iOS/Android 앱 gesture, C13.3 runtime delta math/owner handoff는 아직 구현 전이다.
- Chrome capture의 DOM event는 oracle를 읽는 데만 쓴다. Spinon JS event callback을 뜻하지 않는다.
- CDP의 touchCancel 뒤 late touchMove 거부는 Spinon의 stale input 처리 근거가 아니다. 앱/core cancellation 검증을 별도로 해야 한다.
- AVD/iOS Simulator, real device, hardware GPU, OS OOM, fling/inertia, pinch zoom, keyboard, scrollbar, snap, RTL/logical axis, S04.10 displayed-frame input은 포함하지 않는다.
