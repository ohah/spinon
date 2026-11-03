# C11.5 Grid item self-alignment 계획 실패 관점 20개

| # | 독립 실패 관점 | 검토 결과 |
|---|---|---|
| 1 | parent `align-items`/`justify-items`와 item self 값을 섞어 기본값 또는 override를 잘못 적용하는가 | **통과** — computed `auto`와 explicit child value를 분리하고 parent-center inheritance 및 child override 장면을 고정했다. |
| 2 | `align-self`/`justify-self`의 block/inline 축이 뒤집히는가 | **통과** — CSS Box Alignment §6.1.5/§6.2.5, CSS Grid §10.3/§10.4와 비대칭 좌표 scene을 쓴다. |
| 3 | 정렬 기준 사각형을 cell 대신 전체 Grid container로 잡는가 | **통과** — 단일 40×30px track이라 area가 고정되고 item frame을 area-relative로 비교한다. |
| 4 | CSS self-alignment subject인 margin box와 content box를 혼동하는가 | **통과** — 모든 positive item margin을 0으로 고정해 edges와 alignment subject를 분리한다. |
| 5 | self 값이 item auto-size/stretch 결과까지 바꾸어 위치 oracle을 오염하는가 | **통과** — item width/height를 명시하고 `normal`·`stretch` computed value는 scope에서 제외한다. |
| 6 | `auto`가 Grid container의 computed default를 사용하지 않는가 | **통과** — parent `center/center`, `end/end`와 invalid-declaration fallback controls로 양축을 관찰한다. |
| 7 | explicit self value가 parent default보다 우선하지 않는가 | **통과** — parent center와 item start/end를 결합한 scene이 좌표를 분리한다. |
| 8 | `place-self`의 첫/두 번째 value 순서 또는 single-value 복제를 뒤집는가 | **통과** — `center`, `end start`, 안전/비안전 비대칭 두-value scene을 둔다. |
| 9 | 물리 `left/right`가 `align-self`에도 잘못 허용되거나 RTL에 일반화되는가 | **통과** — `justify-self`만 허용하고 LTR·horizontal-tb로 scope를 제한한다. |
| 10 | CSS-invalid authored value를 computed unsupported value로 처리하는가 | **통과** — Chrome 156에서 `align-self:left`·`justify-self:top`은 `auto`로 계산된다. raw author string을 읽지 않고 cascade fallback을 별도 통과 조건으로 둔다. |
| 11 | 문법상 유효하지만 아직 매핑하지 않은 `safe start` 등 값을 암묵 허용하는가 | **통과** — valid-but-unsupported computed value는 property/node를 보존해 layout 전 거부한다. |
| 12 | `self-start/self-end`의 subject writing mode 의존을 임의로 바꾸는가 | **통과** — 이 profile은 item과 container를 LTR `horizontal-tb`로 고정하고 행렬은 시작/끝 결과만 확인한다. |
| 13 | Grid에서 `flex-start/flex-end`가 start/end와 다른 결과를 만드는가 | **통과** — 비-flex formatting context의 alias 규범과 end/start scene을 포함한다. |
| 14 | safe center/end가 item overflow에서 data-loss를 피하려 start로 돌아오지 않는가 | **통과** — 50×40 item의 40×30 area에서 safe 결과 `(0,0)`을 고정했다. |
| 15 | unsafe center/end가 overflow를 숨기거나 위치를 clamp하는가 | **통과** — 기대 음수 origin `(-5,-5)`와 `(-10,-10)` 및 pixel 샘플을 별도 검증한다. |
| 16 | `place-self`의 축별 safe/unsafe 값 적용 순서를 혼합하는가 | **통과** — `unsafe center safe end`와 역순을 `(0,-5)`·`(-5,0)`으로 구분한다. |
| 17 | auto margin이 self alignment보다 먼저 free space를 흡수하는 규범을 위반하는가 | **통과** — auto margin을 지원하지 않고 reject control로 fail-closed한다. |
| 18 | 한 unsupported item이나 stale revision이 형제 item의 partial frame publication을 남기는가 | **통과** — 전체 layout 실패, frame/scene 미발행과 기존 revision 유지가 완료 조건에 있다. |
| 19 | 기존 PR #1124 container alignment, placement, Grid tracks, source order 또는 교체 요소의 `justify-self:start` 예외가 바뀌는가 | **통과** — 현재 `LayoutStyle`에는 `justify_self`가 없어 새 typed 입력 추가가 필요하다. 계획에 `Auto` 기본값, Grid item Taffy 전달과 교체 요소의 기존 `start` fallback 보존을 반영했고 모두 회귀 대상으로 넣었다. |
| 20 | AVD·Simulator/Chrome 측정이 실기기, compositor scanout, WPT 또는 안정된 공개 CSS 지원으로 과장되는가 | **통과** — Android software renderer, iOS pre-compositor sample, CSS Align overflow at-risk와 전체 C11.5/WPT 미완료를 명시한다. |

이 검토는 **계획만** 판정한다. 구현 또는 플랫폼 실행의 통과 판정은 포함하지 않는다.
