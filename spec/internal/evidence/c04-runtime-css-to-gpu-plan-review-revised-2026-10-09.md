# C04.10 계획 수정본 계약 재검토

- 대상: [`plan/c04-runtime-css-to-gpu.md`](../../../plan/c04-runtime-css-to-gpu.md)
- 검토 대상 SHA-256: `5b06f4916b56e37f43a73be008f2055bee343c8d58cd6f5abf3631594b077300`
- 목적: canonical JavaScript 입력과 실제 Android/iOS 실행 구조를 명시한 수정본에서 서로 다른 구현 실패 경로를 확인한다.

| # | 실패 관점 | 확인 및 계획 반영 |
| ---: | --- | --- |
| 1 | V8이 Chromium HTML oracle과 다른 문서를 실행할 위험 | 모바일은 단일 `.js`, Chromium은 대응 `.html`을 사용한다. 정적 검사에 노드 순서와 인라인 style 동등성 검사를 추가했다. |
| 2 | 수정 가능한 fixture가 oracle provenance를 바꾸지만 해시를 놓칠 위험 | 기존 Chromium reference는 HTML과 inventory 기준으로 고정하며, 실행 JS의 별도 SHA-256을 precomparison 문서에 기록했다. 둘의 증명 범위를 분리한다. |
| 3 | C04.9 JSON 허용 범위를 바꾸어 회귀하는 위험 | 계획은 기존 C04.8/C04.9 schema와 허용 목록을 보존하고 paint 허용을 새 runtime profile에만 둔다. |
| 4 | 새 scene이 fixture provenance에 잘못 묶이는 위험 | 동적 `RuntimeRenderSnapshot`은 static fixture snapshot과 분리하고, 정적 산출물 경로를 그대로 유지한다. |
| 5 | scene이 revision 일부만 비교해 오래된 스타일을 섞는 위험 | generation, document, render-tree, style, environment의 다섯 revision을 모두 요구한다. |
| 6 | 계산 도중 DOM이 변경된 결과를 publish하는 위험 | 현재 요청 key와 style/layout key가 모두 같을 때만 장면을 만들고 publish하도록 적었다. |
| 7 | 누락·중복 NodeId 또는 잘못된 preorder가 부분 화면으로 나타나는 위험 | scene 전체 실패 조건에 node 집합, DOM preorder, paint 순서 연속성을 넣었다. |
| 8 | NaN, 음수 frame 또는 오른쪽/아래쪽 합산 overflow 위험 | frame 검증에서 유한 좌표, 음수 크기, x+width/y+height overflow를 거부한다. |
| 9 | `display:none`·영역 0·투명 요소가 기본 배경색으로 잘못 칠해지는 위험 | 숨김·영역 0은 제외하고 투명은 `paint none`으로 둔다. root 배경과 host clear 색을 구분한다. |
| 10 | 투명 CSS 색을 불투명 RGB로 변환해 검정 테두리를 만드는 위험 | 첫 슬라이스는 불투명 sRGB 또는 paint 없음만 지원하고 alpha/gradient를 허용하지 않는다. |
| 11 | sRGB 색을 두 번 변환하거나 색 비교를 screenshot 안티앨리어싱에 맡기는 위험 | shader 입력의 선형화와 surface sRGB 변환을 명시하고 offscreen 표본과 surface screenshot의 역할을 나눴다. |
| 12 | 형식 이름만 sRGB이고 실제 surface color space가 다른 위험 | RGBA8/BGRA8 후보의 per-format `SRGB` capability가 있어야 초기화하고 없으면 실패하도록 고정했다. |
| 13 | 플랫폼별 GPU backend가 계획과 다르게 선택되는 위험 | Android/iOS 각각의 실제 surface 생성과 backend 보고를 요구하며 WGPU 형식·present mode를 적었다. |
| 14 | Android UI callback에서 WGPU surface를 만지거나 GLSurfaceView 가정이 들어가는 위험 | 실제 구현인 `SurfaceView` + 전용 serial render executor로 계획을 고쳤다. |
| 15 | iOS UIKit 객체를 queue 밖에서 해제하거나 weak/raw pointer가 먼저 사라지는 위험 | renderer가 UIView/CAMetalLayer를 사용하는 동안 view 수명 보장, serial render queue 파괴 순서와 shutdown을 계약에 포함했다. |
| 16 | resize/color scheme 변경 후 이전 scene이 새 surface에 표시되는 위험 | 이벤트 시작 시 presentation sequence를 즉시 무효화하고, 플랫폼 surface generation 검사와 present 직전 sequence 검사를 분리해 둔다. |
| 17 | 색상 체계 변경에 존재하지 않는 새 surface generation을 요구하거나 resize를 누락하는 위험 | surface generation은 surface create/size/lifecycle에서만 갱신하고, color scheme은 environment revision만 갱신하도록 구분했다. |
| 18 | 종료 중 pending draw가 sequence 검사 전에 화면에 나타나거나 host가 먼저 해제되는 위험 | shutdown 시 즉시 sequence를 무효화하고 runtime queue drain → render queue drain → surface/host destroy 순서를 명시했다. |
| 19 | render queue가 V8/CSS 계산을 기다리며 UI·종료를 막는 위험 | 계산 대기는 background runtime executor에서만 하고 WGPU serial queue는 immutable scene만 읽도록 분리했다. |
| 20 | Rust 단위/offscreen 테스트를 실제 V8·모바일 surface 성공으로 과장하는 위험 | Android API 37과 iPhone 17 Pro iOS 26.2 simulator에서 실제 V8·제품 surface screenshot을 요구하고, offscreen readback만으로 완료하지 못하도록 했다. |

## 수정 결과

- 기존 계획의 GLSurfaceView 언급을 실제 Android `SurfaceView` 실행기로 바꾸었다.
- surface generation과 presentation sequence의 역할을 구분하고, color scheme 변경을 surface generation 갱신과 분리했다.
- 앱 runtime 입력 `.js`와 Chromium oracle `.html`의 동등성 및 입력별 증명 범위를 적었다.
- 출시 전 계약 버전은 `0.1.0` 고정을 유지한다.

## 구현 전제와 미검증 경계

- 이 계획 검토는 수정된 문서와 확인 가능한 구조를 대상으로 한다. 수정본 실행을 위해 실제 V8 소스 checkout·Android/iOS simulator build와 화면 검증은 별도로 필요하다.
- 실제 기기·성능·전체 HTML/CSS 적합성은 이 작업의 검증 범위가 아니다.
