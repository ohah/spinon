# C13.1 · 복합 stacking descendant의 overflow clip 경계

**상태:** 구현 전 계획 · Chrome 155 사전 비교 고정 · fixture 전용 범위
**상위:** [C13 overflow·scroll·클리핑 계획](c13-overflow-scroll-clipping.md) · [구현 상태 대장](../spec/STATUS.md)
**내부 계약:** [0060 C13.1 overflow clip](../spec/internal/0060-c13-1-overflow-clip.md), `0.1.0-draft` 유지
**Chrome 근거:** [고정 사전 비교와 화면](../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/README.md)

## 목적

C13.1 내부 계약에 남은 복합 stacking descendant 경계를 좁게 검증한다. overflow parent가 stacking context를 만들고, 그 안의 높은 `z-index` positioned descendant가 clip 밖으로 나가며, 외부 sibling overlay가 겹치는 장면에서 **paint 색**과 **topmost hit identity**를 각각 Chrome과 비교한다.

같은 geometry를 둔 `overflow:visible` 대조군은 탈출 child가 clip 밖에서 paint/hit 되는지 확인한다. `overflow:clip`과 `overflow:hidden`은 clip 안쪽 positive control, clip edge 양쪽, target frame 안의 clip 바깥, sibling overlay 겹침에서 별도로 판정한다. 이 작업의 통과는 제한된 C13.1 fixture 동작 근거이며 일반 stacking/overflow CSS 지원이나 C13.1 완료를 뜻하지 않는다.

## 고정 입력과 Chrome 기준

- 원본: [`overflow-clip-stacking-descendants-c13-1.html`](../tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html), UTF-8 3-case HTML. fixture SHA-256 `6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e`.
- 브라우저: Agent Browser `0.31.1`, Google Chrome `155.0.8059.85`, macOS `26.5.1` arm64, CSS viewport `540×128`, DPR `1`, scroll `(0,0)`. UA, full-version metadata와 browser binary SHA-256은 [metadata JSON](../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome-browser-metadata.json)에 둔다.
- 기준 screenshot: `540×128` PNG, SHA-256 `a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c` ([화면](../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome155-nested-stacking-descendants.png)).
- 구조: 각 parent는 `position:absolute; z-index:0; width:120px; height:72px`다. 내부 layer는 `z-index:900`, target은 `z-index:10`이고 parent 오른쪽으로 40px 탈출한다. root sibling overlay는 `z-index:1`로 탈출 영역과 16px 겹친다. 세 case는 geometry가 같고 overflow 값만 `visible`, `clip`, `hidden`이다.
- [실측 geometry·hit identity](../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome-observation.json) 및 [18개 screenshot pixel·hit 표본](../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome-paint-hit-samples.json)이 이 계획의 expected 값이다. HTML·브라우저·viewport/DPR·PNG·표본 inventory가 바뀌면 이 기준을 그대로 재사용하지 않고 새 비교로 갱신한다.

## 사전 등록 표본

각 좌표의 y는 `52.5 CSS px`다. x는 case parent 왼쪽을 기준으로 하며 Chrome과 runtime 모두 동일한 정수 CSS pixel cell을 읽는다. paint 색은 불투명 RGBA다.

| 표본 | 상대 x | visible hit / RGBA | clip hit / RGBA | hidden hit / RGBA |
| --- | ---: | --- | --- | --- |
| parent-only | `+24.5` | parent / `240,198,116,255` | parent / `240,198,116,255` | parent / `240,198,116,255` |
| target-inside | `+102.5` | target / `87,166,217,255` | target / `87,166,217,255` | target / `87,166,217,255` |
| clip-edge-inside | `+119.5` | target / `87,166,217,255` | target / `87,166,217,255` | target / `87,166,217,255` |
| clip-edge-outside | `+120.5` | target / `87,166,217,255` | root / `240,244,247,255` | root / `240,244,247,255` |
| escape-before-overlay | `+124.5` | target / `87,166,217,255` | root / `240,244,247,255` | root / `240,244,247,255` |
| stacking-overlay | `+136.5` | overlay / `228,81,79,255` | overlay / `228,81,79,255` | overlay / `228,81,79,255` |

case별 절대 x는 visible `20+offset`, clip `190+offset`, hidden `360+offset`이다. `chrome-paint-hit-samples.json`에는 소수점 CSS hit 좌표와 PNG에서 읽은 정수 pixel 좌표를 함께 기록했다. expected hit를 색 비교로 대체하지 않으며, 색 일치를 hit identity 증거로 쓰지 않는다.

## 구현 범위와 판정

1. 고정 HTML과 독립 Chrome observation을 C13.1 opt-in fixture host 입력에 연결한다. source/browser/viewport/DPR와 예상 node ID 순서를 검증하며 임의 HTML이나 앱이 전달한 expected 값을 받지 않는다.
2. host Rust runtime에서 세 case의 frame·computed overflow와 18개 `hit_test_css_point` 결과를 검증한다. geometry는 axis별 최대 `0.5 CSS px`를 허용하고 hit identity는 exact match여야 한다.
3. Android API 37 ARM64 AVD `emulator-5554`와 iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에 같은 opt-in scene을 연결한다. 각 플랫폼에서 서로 다른 app process 3개를 사용하고 18개 hit identity 및 같은 revision의 18개 RGBA surface 표본을 Chrome과 exact 비교한다. 화면과 raw PixelCopy/Metal drawable artifact를 함께 남긴다.
4. clip 밖 target point는 `visible`에서 target/blue, `clip`·`hidden`에서 root/background여야 한다. `stacking-overlay`는 child 내부 z-index가 더 커도 항상 sibling overlay/red여야 한다. clip 안쪽 edge와 parent-only는 각각 positive control이다.
5. comparator와 app route의 negative controls는 최소한 case/point 순서 변경, 중복·누락 표본, 잘못된 target/overlay identity, RGBA 각 채널 1-byte mutation, 다른 run/process/surface generation 및 stale document/style/render revision을 거부한다. 실패 표본은 pass artifact에 합치지 않는다.
6. runtime fixture adapter·FFI symbol·payload는 기존 `c13-1-overflow-clip-fixture-test` feature 아래에만 둔다. default/product artifact 부정 대조와 opt-in positive control을 각각 확인한다.

## 제외 범위

- DOM event dispatch·pointer/click activation은 S05/C29 owner다. 이 작업은 `elementFromPoint()`에 해당하는 내부 hit query만 확인한다.
- WPT 실행과 일반 CSS 적합성 선언은 제외한다. WPT source 분류 또는 Chrome PASS를 Spinon WPT PASS로 바꾸지 않는다.
- border radius, transform/filter/perspective, clip-path/mask, float geometry, scroll state/input, text/replaced content, CSSOM 및 OS compositor/scanout은 제외한다. 관련 입력이 등장하면 조용히 기본값 처리하지 않고 기존 fail-closed 경계를 유지한다.
- 실기기, Android `localhost:5555`, hardware GPU는 사용하지 않는다. AVD software renderer와 iOS Simulator drawable 결과를 하드웨어/display 결과로 확대하지 않는다.

## 구현 후 확인

변경 책임에 따라 formatter, C13.1·renderer·FFI/reference test, workspace test·Clippy, Android API 37 AVD와 iOS Simulator app build/run을 수행한다. 실행한 명령과 실패/수정 전후 근거를 기록한다. default/opt-in artifact 경계를 확인하고 agent-browser로 Chrome fixture를 수집·시각 확인한 뒤 그 agent-browser process를 종료한다. `spec/STATUS.md`는 C13.1 및 C13 상위 항목을 미완료로 유지한다.

계획 변경 PR과 뒤이은 구현 PR은 서로 다른 관점의 적대적 검증을 20개씩 남긴다. 구현 PR에는 계획 자체의 20개 검토와 겹치지 않는 신규 구현 실패 관점 20개를 별도로 둔다.
