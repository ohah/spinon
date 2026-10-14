# C14.2 계획 적대 검토 · 2026-10-14

대상: [`c14-2-image-ratio-sizing.md`](../../../plan/c14-2-image-ratio-sizing.md). 계획과 사전 비교를 서로 다른 실패 경로 20개에 대조했다. 확인된 빈틈은 구현 전 계획에 반영했다.

| # | 실패 관점 | 판정과 계획 반영 |
|---:|---|---|
| 1 | 사전 fixture의 상대 URL이 틀리면 PNG가 깨져 모든 자연 크기 case가 0×0으로 보일 수 있다 | 저장소 fixture PNG를 상대 경로로 다시 연결하고 실제 agent-browser natural size 24×16 및 source hash를 캡처했다. |
| 2 | 성공한 빈 metadata와 snapshot entry 누락을 같은 상태로 처리해 300×150 기본 크기가 실패한 로드에도 적용될 수 있다 | entry 존재 여부와 nullable metadata 값을 분리하고 누락·실패는 visible auto-axis에서 별도 오류라고 명시했다. |
| 3 | 선택 필드를 도입하며 기존 width/height 필수값 사용이 남아 compile 시점 또는 runtime에서 잘못 읽힐 수 있다 | 모든 consumer와 decoder adapter를 optional model로 전환하고 PNG 24×16 회귀를 계획에 넣었다. |
| 4 | 자연 비율이 0·음수·NaN·infinity면 곱셈/나눗셈 결과가 레이아웃에 전파될 수 있다 | 양수·유한 검증과 곱셈/나눗셈 overflow negative control을 추가했다. |
| 5 | 두 natural axis와 별도 ratio가 서로 다르면 어떤 값이 기준인지 불명확하다 | 두 축이 있으면 width/height로 자연 ratio를 만들고 별도 ratio 불일치는 거부한다. |
| 6 | width와 height가 모두 지정됐는데 metadata나 ratio가 실제 결과를 덮어쓸 수 있다 | 두 축 definite는 natural metadata를 무시하고 authored dimensions에 제약만 적용한다고 고정했다. |
| 7 | width만 지정된 이미지를 width×default-height로 처리해 자연 비율이 손실될 수 있다 | 지정 축에 대한 ratio transfer, 자연 반대축, default 반대축 순서를 명시했다. |
| 8 | height만 지정된 이미지에서 width 방향 transfer가 비대칭으로 누락될 수 있다 | 반대 축의 동일 규칙과 Chrome 36×24 사전 기준을 명시했다. |
| 9 | 한 natural 축만 있고 ratio가 없을 때 반대축을 0이나 같은 값으로 추정할 수 있다 | 존재하는 축은 보존하고 빠진 축은 300×150 기본 객체 크기에서 대응 축을 가져온다. |
| 10 | 자연 크기와 비율이 모두 없는 성공 entry를 실패하거나 잘못된 빈 크기로 처리할 수 있다 | 성공 metadata entry에 한해서 300×150을 사용하고 누락 entry와 분리해 검증한다고 명시했다. |
| 11 | ratio-only auto/auto를 임의로 contain 300×150해 context별 intrinsic contribution 계약을 가릴 수 있다 | ratio-only auto/auto intrinsic contribution은 C14.3으로 이관하고 이번 solver에서 거부한다. |
| 12 | CSS 2.2 min/max 10개 violation 조합을 일부만 구현하거나 두 축 violation 우선순위를 틀릴 수 있다 | 10개 조합 각각을 수치 fixture와 table-driven unit test로 대조하는 구현 gate를 넣었다. |
| 13 | min이 max보다 크면 어느 값이 우선할지 구현에 따라 달라진다 | 각 축에서 max를 min으로 정규화해 min 우선으로 한다. |
| 14 | 한 축 명시 + 반대 auto축 max/min을 적용한 뒤 ratio를 다시 강제해 Chrome과 차이가 날 수 있다 | `width:48;height:auto;max-height:20`처럼 auto축 constraint로 ratio가 깨지는 case를 기준 oracle로 남기고 Taffy 보정을 우회한다. |
| 15 | border-box width/min/max에서 padding과 border를 두 번 더하거나 빼지 않을 수 있다 | 콘텐츠 상자 계산과 외부 border-box 프레임 변환을 분리해 edge 합계를 한 번만 적용한다. |
| 16 | DPR 또는 snapshot revision이 섞여 CSS px와 device px가 혼용될 수 있다 | solver를 DPR 전 CSS px에 두고 source/revision stamp 및 stale snapshot negative control을 유지한다. |
| 17 | percentage/calc padding 또는 sizing 값이 definite처럼 오인될 수 있다 | 이 단계 지원값을 절대 CSS px와 auto로 제한하고 percentage/calc 입력은 식별 가능한 실패로 둔다. |
| 18 | numeric aspect-ratio와 auto 또는 min/max 조합에서 기존 Taffy 보정이나 C07.3 제약이 결과를 다시 바꿀 수 있다 | numeric ratio는 자연 ratio를 대체하는 limited path로 정의하고 auto/ratio-only 경계 및 authored ratio+minmax 제한을 별도 preflight한다. |
| 19 | display:none 또는 숨긴 조상 하위에서 metadata 누락을 요구하거나 invalid size가 숨은 트리에 전파될 수 있다 | hidden subtree에는 metadata entry를 요구하지 않고, 제공된 entry는 source와 값 유효성 검증을 유지한다. |
| 20 | flow-only/visual pass, flex auto-basis, source 교체가 서로 다른 크기를 내거나 문서가 runtime 완료로 확대 해석할 수 있다 | 모든 pass에 같은 solver를 연결하고 flex·revision 회귀를 별도 검증한다. AVD는 Rust 경계 증거로만 보고하며 앱 runtime, GPU, U05/C14 완료를 주장하지 않는다. |

20개 계획 실패 관점을 검토했다. 이번 라운드에서 계획의 주요 경계를 확정했다. 이 검토는 구현 검토나 PR 적대 검토를 대체하지 않는다.
