# C13.2 후속 · visible overflow surface pixel 검증

- **상태:** 고정 Chrome oracle 확정 · 최신 source run5 Android/iOS Simulator 표면 표본 5/5 통과 · comparator negative controls 및 feature 경계 통과 · 구현/PR 변경 적대 검토 완료 · [구현 PR #176](https://github.com/ohah/spinon/pull/176)
- **상위 계획:** [C13.2 scroll container와 revision 분리](./c13-2-scroll-container-state.md)
- **내부 계약:** [0062 C13.2 scroll state](../spec/internal/0062-c13-2-scroll-state.md), `0.1.0-draft` 유지
- **구현 전 기준과 화면:** [Chrome 155 표본·geometry·PNG hash](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-plan-2026-10-15/README.md)
- **Simulator 실행 증거:** [Android PixelCopy·iOS Metal 표본과 화면](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-2026-10-15/README.md)
- **계획 실패 관점 20개:** [검토 기록](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-plan-review-20-2026-10-15.md)
- **계획 PR 변경 관점 30개:** [검토 기록](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-pr-review-30-2026-10-15.md)
- **구현 실패 관점 20개:** [검토 기록](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-implementation-review-20-2026-10-15.md)
- **PR 변경 적대 관점 30개:** [검토 기록](../spec/internal/evidence/c13-2-visible-overflow-surface-pixels-pr-review-30-2026-10-15.md)

## 목적과 범위

C13.2 visible scrollport의 descendant가 parent border 밖에서도 실제 WGPU surface에 그려지는지 Android API 37 ARM64 AVD의 SurfaceView PixelCopy와 iOS 26.2 Simulator의 Metal drawable readback으로 검사한다. 현재 screenshot 육안 확인과 hit identity assertion을 실제 RGBA 표면 표본으로 보강한다.

같은 C13.2 runtime scroll fixture와 400×240 CSS px viewport를 유지한다. Android는 고정 AVD만 쓰고 iOS는 고정 Simulator만 쓴다. 실기기, hardware GPU, compositor/display scanout, WPT, 새로운 CSS 동작, 공개 CSSOM/gesture/event는 범위 밖이다. 이 후속의 통과로 C13.2 또는 상위 C13 전체를 완료 처리하지 않는다.

## Chrome 155 기준

`tests/fixtures/css/c13/c13-2-visible-border-pixel-probe.html`은 현재 runtime fixture source `runtime-scroll-container-c13-2.js`를 불러 실제 matrix의 DOM/style 선언을 재사용한다. HeadlessChrome `155.0.8059.40`, viewport `400×240` CSS px, DPR 1에서 root와 visible parent/escaped child geometry 및 PNG 표본을 고정했다.

| 표본 CSS cell | 기대 RGBA | 기하 판정 |
| --- | --- | --- |
| `(150,30)` | `(227,84,78,255)` | escaped child 안이며 parent 오른쪽 border `x=130` 바깥 |
| `(136,30)` | `(16,24,39,255)` | parent 바깥 gap. escaped child와 겹치지 않음 |
| `(20,20)` | `(52,66,86,255)` | parent 내부 배경 positive control |
| `(11,11)` | `(240,198,116,255)` | parent border strip positive control |
| `(50,110)` | `(16,24,39,255)` | 같은 root의 빈 배경 control |

표본은 CSS integer cell 기준이며 surface 좌표에는 각 cell의 중심 `x+0.5,y+0.5`를 사용한다. 기대값·fixture·viewport·screenshot hash는 `chrome155-observation.json`에 고정한다. 바뀐 fixture bytes, parent/child geometry, screenshot hash, 대상 element identity는 사전 비교 검사에서 fail해야 한다.

## 구현 순서

1. C13.2의 opt-in runtime host에만 표면 표본 request를 열고 C ABI·JNI·Objective-C++ 경계를 feature-gate한다. 공개 header API 또는 기본 artifact에 fixture symbol/payload를 넣지 않는다.
2. 같은 C13.2 26-frame·9-owner final matrix의 draw가 준비된 뒤 위 5개 CSS pixel cell을 요청한다. request와 readback draw 사이에는 DOM 재평가를 하지 않고 document/render-tree/style/environment/scroll revision이 final matrix draw와 같은지 검사한다.
3. Android SurfaceView PixelCopy는 final matrix draw 완료 뒤 실제 surface size와 viewport를 사용해 표본 좌표를 계산하고 PNG bytes·dimensions·SHA-256·run ID·PID·surface generation을 기록한다. density 추정값만으로 좌표를 만들지 않는다.
4. iOS Metal sample request는 같은 current renderer host에 예약하고, 다음 draw report의 RGBA만 읽는다. viewport 400×240, drawable scale, format, generation, run/process ID를 기록한다. 결과는 compositor 제출 전 drawable 표본으로 설명한다.
5. 앱 route는 같은-host C13.2 runtime assertion을 먼저 통과한 뒤 matrix draw 표면을 캡처한다. 이전 PID/run/surface/draw revision의 표본이나 callback을 섞으면 실패시킨다.
6. comparator negative controls는 escaped sample 예상 채널 변조, escaped sample을 parent 안 좌표로 변경, 표본 누락/중복, stale run/generation, 잘못된 image size/hash를 각각 reject한다. mutation은 실제 pass artifact에 섞지 않는다.

## Pass/fail 기준

- Chrome source/version, viewport/DPR, HTML bytes와 PNG hash가 고정 manifest와 exact 일치한다.
- Android API 37 ARM64 AVD PixelCopy에서 다섯 표본 모두 Chrome 기준 RGBA와 exact 일치하고, 실제 escaped 표본 cell center가 parent 오른쪽 border 바깥임을 재검사한다.
- iPhone 17 Pro / iOS 26.2 Simulator Metal drawable에서도 같은 다섯 표본이 exact 일치한다. iOS 표본은 compositor 제출 전 데이터로 한정한다.
- 두 platform summary는 같은 C13.2 source, fixture matrix, runtime revisions, run ID와 최종 26-box draw에 연결되어야 한다. capture가 다른 process/surface generation에 속하면 통과할 수 없다.
- 모든 negative control은 positive baseline artifact에서 fail하며, 실패 sample id/좌표/reason이 남는다.
- feature-on/off Rust FFI·archive 경계, Android/iOS feature flags, formatter, workspace/feature Clippy·test와 boundary test를 다시 실행한다. 실행하지 않은 실기기·hardware GPU·compositor 검증을 주장하지 않는다.

## 의도적으로 남길 한계

C13.2 `box-shadow` ink-only scroll overflow는 CSS paint property가 fail-closed 상태라 이 후속에서 지원하지 않는다. 실제 OS OOM, concurrent scheduler race, stored state failure 중 기존 displayed frame 유지, public scroll API/events, scrollbar, snap, RTL/logical axes와 WPT도 이 표면 검증으로 닫히지 않는다.
