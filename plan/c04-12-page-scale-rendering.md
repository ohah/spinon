# C04.12 후속 · 초기 page scale 렌더 변환

## 목적

루트 HTML에서 계산된 `device_scale_factor`와 초기 `page_scale`을 runtime render scene의 CSS 좌표에 적용해 WGPU surface에 그린다. layout viewport와 CSS hit-test 좌표는 기존 CSS px 값으로 유지하고, 초기 visual viewport의 물리 경계에서 출력이 잘려야 한다. 변환을 붙이지 않은 기존 scene은 현재 surface-fit 동작을 그대로 사용한다.

이 단계는 일반 HTML body를 layout·paint하는 기능이 아니다. 고정된 renderer scene fixture에 `RootHtmlEnvironmentSnapshot`의 두 배율을 적용해 색상 box, image, pre-rasterized glyph의 surface geometry를 검증한다. 루트 Canvas/native background는 scene transform 바깥에서 기존처럼 surface 전체에 칠한다. native touch 좌표 변환, pinch zoom, visual viewport pan/scroll 및 text rerasterization은 제외하며 C04.12 전체를 완료 처리하지 않는다.

## 구현 전 비교 모델

- 기준 구현: Agent Browser CLI `0.31.1`의 Chrome `155.0.8059.85`, iPhone 12 device emulation, screen `390×844` CSS px, DPR `3`, physical screenshot `1170×2532`.
- 재현 fixture와 재캡처: [visual viewport fixture](../spec/internal/evidence/c04-12-visual-viewport-2026-10-17/README.md), 이 계획의 세 PNG는 이번 실행에서 Agent Browser로 다시 열고 저장한 Chrome oracle이다. Chrome oracle 화면과 앱 화면은 구분한다.
- 좌표식: scene CSS point `(x,y)`는 physical target의 `(x × DPR × page_scale, y × DPR × page_scale)`에 놓인다. origin은 `(0,0)`이며 이동·safe-area inset은 없다. WGPU target boundary가 초기 visual viewport를 자른다.
- layout/media 입력은 계속 CSS layout viewport다. renderer transform은 cascade/layout 값이나 `viewport_css_px()`를 바꾸지 않는다.

| Chrome case | layout viewport CSS px | visual viewport CSS px | DPR × page scale | CSS probe | 예상 physical bbox |
| --- | ---: | ---: | ---: | ---: | ---: |
| `initial-scale=0.5` | `780×1688` | `780×1688` | `1.5` | `(100,50) 20×10` | `[150,180) × [75,90)` |
| `initial-scale=2` | `390×844` | `195×422` | `6` | `(100,50) 20×10` | `[600,720) × [300,360)` |
| `width=500, initial-scale=1` | `500×1082.0513` | `390×844` | `3` | `(100,50) 20×10` | `[300,360) × [150,180)` |

표면 배율은 브라우저 oracle의 정수 pixel 경계와 정확히 일치해야 한다. 레이아웃 viewport 및 DPR/page-scale 값은 기존 viewport fixture의 허용치 `0.001 CSS px` 안이어야 한다. image·glyph의 색/coverage raster는 자체 source raster 조건을 따르며 이 단계에서 Chrome 텍스트 안티앨리어싱 동일성을 주장하지 않는다.

## 구현 경계

1. `RuntimeRenderSnapshot`에 선택적인 유효성 검사를 거친 root page-scale surface mapping을 둔다. 기존 생성자는 mapping이 없는 상태를 유지한다. document/style/environment revision, text/image 연결 및 scroll offset 재발행은 mapping을 보존한다.
2. WGPU의 colored geometry, border, image, glyph 경로가 하나의 CSS→surface mapping을 쓴다. 없는 mapping은 현재 viewport-fit 계산을 사용한다. 있는 mapping은 같은 `DPR × page_scale`을 사용하며 viewport origin을 고정한다.
3. CSS clip은 기존 scene 좌표에서 적용한다. scale된 결과는 물리 target에서 자연스럽게 잘리고, glyph 경로의 CPU 가시 범위는 visual viewport와 일치하도록 한다. opacity group·paint command 순서는 기존대로 유지한다.
4. 유효하지 않은 DPR/page scale 또는 곱셈 overflow는 scene에 mapping을 부착하기 전에 구체적 오류로 거부한다. 매핑을 제외한 legacy scene은 영향을 받지 않는다.
5. scene-level WGPU fixture는 세 Chrome case, 화면 끝과 요소 frame 바깥 control, CSS clip, border, image, glyph, paint order/opacity, scene 재발행 보존, invalid transform 및 no-transform regression을 확인한다. Chrome과 동일 크기의 offscreen target에서는 사전 고정 bbox를 비교한다.
6. opt-in Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 실행은 WGPU-backed fixture readback/보고를 검증한다. 화면에 표시되는 PASS report는 HTML body paint 증거가 아니다. 물리 device는 대상이 아니다.

## 완료 조건과 제외

- 색상 box·border·image·glyph geometry의 physical mapping 및 visual surface clipping이 scene fixture 기준을 만족한다.
- scene mapping을 사용하지 않는 기존 renderer fixture와 hit-test CSS 좌표가 바뀌지 않는다.
- `DPR × page_scale`은 scene snapshot에 입력된 경우에만 적용된다. `RuntimeSession::set_root_html_environment()` 결과를 일반 문서 layout/paint에 자동 적용하는 연결은 이 PR의 opt-in fixture 경계 안에서만 입증하며, 전체 HTML 렌더링 지원을 뜻하지 않는다.
- pinch/pan, input surface→CSS 매핑, scroll 이후 visual viewport, safe area, system UI, text 재래스터화 및 일반 DOM body paint는 제외한다. C04.12 상위 체크는 미완료로 둔다.
