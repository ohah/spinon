# C04.12 page-scale WGPU 구현 실패 관점 검토 20개

각 항목은 서로 다른 실패 경로를 실제 변경 코드, 고정 fixture 또는 실행 결과와 대조했다. PASS는 그 경로가 명시한 범위에서 확인됐다는 뜻이며 일반 HTML 렌더링 지원을 뜻하지 않는다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | DPR 0·음수·NaN·무한대 입력 | `RuntimePageScaleTransform::new` 거부 assertion 통과. |
| 2 | page scale 0·음수·무한대 입력 | 같은 입력 검증 테스트 통과. |
| 3 | 양수 두 배율의 곱셈 overflow/underflow | `f32::MAX × 2`와 두 `MIN_POSITIVE` 곱셈 거부 테스트 통과. |
| 4 | 변환 추가로 CSS layout viewport가 물리 크기로 변함 | snapshot의 `viewport_css_px()`가 원래 CSS 크기를 유지하는 assertion 통과. |
| 5 | hit-test가 physical point를 CSS point로 오인 | 매핑 snapshot에서도 `(100,50)` CSS hit가 같은 node를 선택하는 테스트 통과. |
| 6 | 생성자 기본 경로가 의도치 않게 배율 적용 | 기존 transform 없는 WGPU 경로 테스트와 전체 render/WGPU 회귀 테스트 통과. |
| 7 | DPR와 scale이 두 번 적용됨 | Chrome scale 0.5·2 geometry 경계 테스트 및 세 case WGPU 표본 통과. |
| 8 | scale 곱에서 x축만 surface 폭을 사용 | `x_to_ndc`와 `y_to_ndc`가 각 physical surface 축을 쓰며 portrait 1170×2532 readback 통과. |
| 9 | 원점/safe-area offset이 무심코 추가됨 | Chrome oracle 물리 bbox와 transform vertex 원점 테스트에서 offset 없이 일치. safe-area 자체는 미지원으로 유지. |
| 10 | frame 경계와 half-open right/bottom edge 처리 불일치 | scale 2 WGPU readback에서 내부·마지막 포함 표본·다음 픽셀 표본 비교 통과. |
| 11 | visual viewport 우측 바깥 요소가 surface에 나타남 | opt-in 세 case의 오른쪽 outside control 표본이 배경값으로 통과. |
| 12 | visual viewport 하단 바깥 요소가 surface에 나타남 | opt-in 세 case의 하단 outside control 표본이 배경값으로 통과. |
| 13 | CSS clip 이후 page-scale 투영 순서가 바뀜 | 변환된 readback에서 clip 안팎 sample이 예상 색과 일치. |
| 14 | border가 box mapping과 다른 scale을 쓰거나 paint 순서가 바뀜 | page-scale readback의 clipped box에서 top border blue, 본체 red 순서 확인. |
| 15 | image UV crop이 scale에 따라 변함 | image geometry test가 destination mapping과 source UV 보존을 비교. 세 case adapter readback의 magenta image sample 통과. |
| 16 | image source revision/node key가 변환 과정에서 유실됨 | fixture image는 scene key와 같은 generation/document/render-tree/intrinsic revision으로 연결; WGPU preparation 및 표본 통과. |
| 17 | glyph mask가 DPR scale과 page scale을 중복 반영 | glyph vertex test가 source raster DPR과 표면 변환을 분리해 경계를 비교. |
| 18 | glyph CPU crop은 layout viewport만 보고 잘못 잘림 | page-scale glyph test가 visual viewport CPU 가시 폭과 physical edge clip을 비교; app readback의 glyph 표본 통과. |
| 19 | opacity group이 mapped scene에서 위치 또는 opacity를 잃음 | scale 3 WGPU readback에서 page-scaled clip/border와 opacity group을 같이 검사; 50% green 표본은 `[0,127,0,255]`. |
| 20 | fixture PASS가 중간 실패·기본 빌드 포함을 가림 | FFI가 30개 표본 전부 일치한 뒤에만 token 생성; Android/iOS PASS 조건은 token 포함; default staticlib negative 및 opt-in positive control 통과. |

## 발견 후 수정

19번을 추가하며 50% 초록 opacity의 readback 예상값을 `128`로 적었지만 실제 WGPU 결과는 `[0,127,0,255]`였다. fixture 예상값을 관찰값에 맞춰 수정한 뒤 새 readback test가 통과했다. 계획 검토 표는 재사용하지 않았다.

## 증거 경계

검증은 고정 scene, pinned Chrome oracle 수치, 현재 host WGPU adapter, Android API 37 AVD, iOS 26.2 Simulator, host static library에 한정한다. HTML body paint, 화면 compositor 이후 픽셀, 실기기, pinch/pan/input mapping과 기본 RuntimeSession 자동 연결은 검증하지 않았다.
