# C18.3 계획 실패 관점 검토 20개

구현 전에 계획 자체의 별도 실패 관점을 검토했다. 결과는 계획·고정 Chrome 사전 비교의 범위다. 아직 구현 동작을 검증한 결과가 아니다.

| # | 실패 관점 | 계획·근거 확인 |
|---:|---|---|
| 1 | `fill`에서 비율을 잘못 유지해 요소 box 전체를 채우지 못함 | 24×12 source를 24×24 frame에 맞추는 oracle scene과 concrete rect를 정했다. |
| 2 | `contain`이 한 축을 넘겨 element background를 가림 | frame 24×24, object 24×12, 위/아래 background sample을 고정했다. |
| 3 | `cover`가 min scale을 써서 box를 덮지 못함 | source ratio 2:1, square frame에서 object 48×24로 고정했다. |
| 4 | `cover` 결과의 넘치는 texel이 image element 밖에 보임 | object rect와 element content-box clip을 별도 계약 항목으로 적었다. |
| 5 | `none`이 natural size를 무시하거나 auto-scale함 | 16×16 frame 안에 24×12 natural object를 두고 안/밖 pixel을 비교한다. |
| 6 | `scale-down`이 항상 contain 또는 항상 none을 선택함 | 16×16 축소 case와 32×20 no-upscale case를 각각 둔다. |
| 7 | position percentage를 content size에 곱해 origin에서 더하지 않음 | `(box-object)×p` 식과 25%, 75% scene을 명시했다. |
| 8 | cover의 negative free space에 percentage 방향을 반대로 적용 | 0%, 50%, 100% cover frame 위치와 edge 색을 대조한다. |
| 9 | 100% 밖 또는 음수 position을 불필요하게 clamp함 | `-25%`, `150%` 값을 Chrome에서 계산하고 frame clip 결과를 고정했다. |
| 10 | 미지정 property의 initial value가 잘못됨 | 미지정 scene에서 computed `fill`, `50% 50%`를 기록했다. |
| 11 | fit 계산이 element layout frame 자체를 바꿈 | 계획이 Taffy frame을 보존하고 paint object rect만 계산한다고 분리한다. |
| 12 | `scale-down` 또는 fractional size가 정수로 잘못 반올림됨 | concrete object 수식과 CSS px frame을 분리하고 finite geometry 검증을 요구한다. |
| 13 | scaled texture sampling을 nearest로 고정해 Chrome 값과 다름 | default `image-rendering:auto` output 표본을 사전 비교 근거에 고정한다. |
| 14 | 샘플링 변경이 C18.1 opaque 또는 C18.2 alpha 결과를 회귀시킴 | 두 선행 fixture를 implementation PR regression gate로 요구한다. |
| 15 | image fit 속성을 비이미지 노드에도 적용 | 범위를 HTML replaced `<img>` 및 전용 fixture profile로 제한했다. |
| 16 | valid CSS지만 미지원 position grammar가 기본값으로 조용히 바뀜 | keyword/length/calc/1·3·4항 입력은 profile에서 명시적으로 fail closed한다. |
| 17 | stale resource 또는 style revision이 새 geometry에 결합 | source/style/render/intrinsic revision을 함께 검사하도록 명시했다. |
| 18 | object와 ancestor clip 교집합 누락으로 UV 범위가 밖으로 나감 | image content clip 후 ancestor clip 교차 및 UV 유효성을 완료 관문에 넣었다. |
| 19 | 수치 overflow/invalid dimensions가 WGPU 정점까지 전파 | zero/non-finite/overflow 입력을 draw 전 구체 오류로 거부한다. |
| 20 | 제한 fixture가 public CSS나 모든 GPU format 지원으로 과장됨 | 내부 계약 0066, opt-in artifact 검사 및 상위 C18 미완료를 명시했다. |
