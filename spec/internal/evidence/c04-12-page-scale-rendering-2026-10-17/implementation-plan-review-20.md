# C04.12 page-scale renderer 계획 적대 검토 20개

계획과 Chrome oracle를 대조해 구현 전에 서로 다른 실패 경계를 검토했다. 아래 항목은 모두 계획 계약 또는 검증 경계에 반영했다.

| # | 공격 관점 | 검토 결과와 계획상 처리 |
| ---: | --- | --- |
| 1 | `page_scale`을 layout viewport 계산에 곱해 media query를 바꾼다 | layout 및 `viewport_css_px()`는 기존 CSS layout 좌표로 고정하고 renderer 단계에서만 변환한다. |
| 2 | DPR과 page scale을 합친 값만 저장해 환경 입력의 의미를 잃는다 | snapshot에 두 유효 입력을 분리 보존하고 projection에서만 곱한다. |
| 3 | layout viewport 폭으로 target을 정규화해 fixed `width=500`을 500 CSS px 전체 표시한다 | surface mapping은 `DPR × page_scale`을 쓰고 target 자체가 visual viewport crop이 된다. |
| 4 | scale 2에서 화면 밖 layout 콘텐츠도 surface에 fit되어 축소된다 | scene 전체를 재정규화하지 않고 CSS origin 기준 배율을 적용해 scale 2의 195×422 visual region만 보인다. |
| 5 | scale 0.5에서 DPR을 빠뜨려 probe 크기가 20×10 physical px가 된다 | 배율을 `3 × 0.5 = 1.5`로 고정하고 expected bbox를 계획 표에 기록했다. |
| 6 | transform을 surface 중앙 기준으로 적용해 content가 왼쪽/위로 밀린다 | 초기 visual origin `(0,0)` 기준을 고정하고 이동은 별도 범위로 제외한다. |
| 7 | y축도 width 비율로 스케일해 aspect ratio가 다른 target에서 왜곡된다 | 동일한 CSS→physical scalar를 쓰되 x/y NDC는 각각 target width/height로 나눈다. |
| 8 | scene에 scale을 두 번 적용하거나 재발행 과정에서 잃는다 | transform은 snapshot의 단일 optional mapping이며 clone/revision/scroll/text/image 경로가 유지되는지 테스트한다. |
| 9 | border width가 box 변환과 달라진다 | box fill과 네 border side가 같은 CSS→surface mapping을 사용한다. |
| 10 | CSS clip 좌표를 먼저 page scale하여 clip 의미를 바꾼다 | CSS clip은 기존 CSS 좌표에서 적용하고, 변환 후 physical target이 visual crop을 담당한다. |
| 11 | image UV가 target crop에서 다시 시작해 texture가 늘어난다 | image quad의 CSS clipping과 UV는 원래 object rect를 기준으로 유지하고 target crop은 raster clip으로 처리한다. |
| 12 | glyph CPU viewport clamp가 layout viewport에 남아 scale된 text를 잘못 자른다 | mapping 적용 scene의 glyph visibility limit은 surface size / effective scale로 계산한다. |
| 13 | glyph scale geometry를 Chrome 텍스트 래스터 동일성으로 오해한다 | source glyph raster는 현재 DPR 조건으로 유지하며 text rerasterization·AA parity를 명시적으로 제외한다. |
| 14 | opacity offscreen group 안의 geometry에만 scale이 빠진다 | group draw와 direct draw가 같은 prepared geometry를 쓰는지 확인 항목에 둔다. |
| 15 | `DPR=0`, NaN, 무한대 또는 곱 overflow로 정점이 NaN이 된다 | constructor에서 finite·positive 입력과 finite 곱을 검증하고 실패를 반환한다. |
| 16 | mapping 없는 legacy scene에 기본 DPR 변환을 적용해 기존 출력이 달라진다 | 기본 생성자는 mapping 없이 유지하고 기존 NDC 및 회귀 fixture를 보존한다. |
| 17 | 극단 layout box에서 CSS float→NDC overflow가 조용히 전달된다 | geometry 생성에서 변환 좌표의 유한성을 확인하고 잘못된 장면을 거부한다. |
| 18 | hit-test 및 native touch 좌표도 자동 보정된 것으로 표시한다 | hit-test는 기존 CSS 좌표 그대로이고 touch input mapping은 계획의 명시적 제외다. |
| 19 | Canvas background나 native system chrome까지 확대된다 | transform 대상은 runtime scene geometry로 제한하고 native Canvas fill/system UI는 경계 밖에 둔다. |
| 20 | simulator report 화면을 실제 HTML body paint로 확대 해석한다 | simulator는 opt-in WGPU fixture/readback으로 서술하고 report 캡처와 Chrome oracle를 분리한다. |

## 계획 판정

계획에 구현 입력, 독립 Chrome 관측, 수치 판정값, 기존 scene 영향 경계, 렌더 primitive별 검증 및 Simulator 범위가 명시되어 구현에 착수할 수 있다. 계획 자체를 구현 검토나 최종 PR diff 검토로 재사용하지 않는다.
