# C10.2 Flex 크기 배분 계획 · 실패 경로 20개 검토

검토 대상은 `plan/c10-2-flex-distribution.md`, C10 상위 계획, 공식 상태 대장 및 현재 cascade→typed layout→Taffy 경로다. 계획 작성 단계 검토이며 기능 구현이나 플랫폼 실행을 완료했다는 뜻은 아니다. 서로 다른 실패 관점을 확인하고 계획을 수정한 뒤 최종 표를 다시 읽었다.

| # | 독립 관점 | 확인한 실패 가능성 | 계획의 통제 |
|---:|---|---|---|
| 1 | 로드맵 선행 관계 | C10.1 병합 전에 line-distribution 작업을 앞당기거나 C09.4의 의존 조건을 무시할 수 있다. | PR #114 병합을 선행으로 기록하고 C09.4 미완료 상태를 유지한다. |
| 2 | 현재 구현의 과장 | typed property mapping이 있으니 CSS 배분 알고리즘도 이미 호환된다고 오판할 수 있다. | 기존 projection을 조사 대상으로만 삼고 Chromium geometry 비교를 완료 조건으로 둔다. |
| 3 | 실제 runtime profile | Rust 단위 profile은 통과하지만 기본 V8 app의 custom-property paint profile에서 속성이 빠질 수 있다. | 여섯 runtime Flex profile과 기본 `RuntimeFlexCustomPropertiesPaintV1`를 명시한다. |
| 4 | full/incremental cascade 일치 | 변경 이후 incremental path가 grow/basis/min/max 값 일부를 stale snapshot에서 재사용할 수 있다. | 두 cascade 결과의 typed output 및 style revision 갱신을 비교한다. |
| 5 | flex base와 width 혼동 | `width`만 사용하거나 flex-basis가 width를 덮는 우선순위를 놓칠 수 있다. | conflicting width/basis case를 고정한다. |
| 6 | `auto` basis와 content basis 혼동 | `flex-basis:auto`가 명시 main size를 가져오는 경로와 content-based sizing이 뒤섞일 수 있다. | 명시 width/height의 auto case만 허용하고 content sizing은 제외한다. |
| 7 | percentage basis의 containing block | indefinite main size나 잘못된 축에서 percentage를 px로 처리할 수 있다. | definite row width·column height의 percent만 지원 범위로 두고 indefinite 입력은 후속으로 남긴다. |
| 8 | shorthand 전개·source order | `flex:1`의 omitted component 기본값이나 shorthand 이후 longhand override를 잘못 적용할 수 있다. | `1`, `auto`, `none`, 3-part 및 override를 Chrome computed longhand와 대조한다. |
| 9 | grow 비율 정규화 | factor가 1 이상인 단순 두 항목은 맞아도 unequal factor에서 잘못 배분할 수 있다. | basis 100×3, grow 1/2/1에서 175/250/175 손계산 기대를 둔다. |
| 10 | grow factor 합 1 미만 | grow factor를 무조건 1로 정규화하면 남는 공간까지 채우는 오동작이 생긴다. | 0.25+0.25 입력에서 112.5/112.5와 50 CSS px 잔여를 관찰한다. |
| 11 | factor 0 및 inflexible item | 0 factor item을 나누기에 포함하거나 초기 freeze를 생략해 다른 항목이 틀어질 수 있다. | grow/shrink 0 case 및 Taffy projection의 factor 검증을 별도 둔다. |
| 12 | scaled shrink | shrink factor만 정규화하고 basis 곱을 빼먹어 큰 항목이 잘못 줄어들 수 있다. | basis 200/100, shrink 1/1에서 폭 160/80을 독립 기대값으로 둔다. |
| 13 | grow/shrink 분기 선택 | flex base 합만 보고 mode를 선택해 min/max가 만든 hypothetical main size의 영향을 놓칠 수 있다. | `hypothetical-factor-choice`에서 base와 hypothetical 합이 다른 입력을 관찰한다. |
| 14 | min/max freeze 반복 | 첫 clamp 뒤 모든 항목에 한 번만 비율을 적용하고 재분배하지 않을 수 있다. | grow max/min, shrink min/max freeze case에 clamp 후 예상 frame을 고정한다. |
| 15 | min과 max가 역전된 입력 | `min > max`를 max에 맞춰 clamp해 CSS의 min 우선 결과와 달라질 수 있다. | factor 0의 별도 case에서 used main size 150을 확인한다. |
| 16 | gap·outer size 회계 | gap 또는 fixed margin을 free space에서 빼지 않아 children 합이 넘칠 수 있다. | gap 20의 expected width 140/140, 두 번째 x=160을 둔다. 별도 case는 margin만 변경한다. |
| 17 | line 간 격리 | wrap이 된 모든 항목의 free space를 합쳐 line 사이에 재분배할 수 있다. | `wrapped-per-line`에서 각 C10.1 line의 independently distributed size를 기록한다. |
| 18 | row/column 축 투영 | column에서 width/basis 또는 vertical gap을 main axis로 잘못 잡을 수 있다. | definite row width와 column height의 paired percentage/factor cases를 둔다. |
| 19 | box sizing과 fractional precision | padding/border/box-sizing 또는 DPR 반올림 차이가 algorithm 오차로 숨을 수 있다. | content-box/border-box paired input, CSS px field 비교, DPR 1/2 및 node별 0.5 px ceiling을 둔다. |
| 20 | 오류·revision·증거 경계 | invalid factor나 Taffy error 뒤 stale frame이 성공으로 공개되거나 simulator를 실기기 성능으로 주장할 수 있다. | 구체적 error context·revision 확인, Android/iOS simulator 원본 저장, 실기기·성능 주장의 제외를 완료 조건에 넣는다. |

## 검토 후 수정 및 재검토

검토 중 확인한 모호점은 세 가지였다. 첫째, 기존 `flex-*` typed mapping만으로 distribution 호환을 가정할 위험이 있어 목표와 완료 조건을 Chromium frame 비교로 좁혔다. 둘째, grow factor 합이 1 미만일 때의 부분 채움 규칙이 일반 비율 나누기에 묻힐 수 있어 독립 수치 case를 추가했다. 셋째, grow/shrink branch가 basis 합이 아니라 hypothetical main size 합에 의존하므로 분기 선택 case를 추가했다. 이 변경은 표의 관점 2, 10, 13에 반영했다.

최종 재검토에서 각 case가 Chromium oracle을 먼저 만들고 Taffy를 후보로 둔다는 점, 명시 min/max와 자동 최소 크기의 경계, wrapped line별 독립 분배, full/incremental profile coverage, 내부 버전 고정을 확인했다. 이 계획만으로 C10.2 기능을 지원 완료로 표시하지 않는다. 코드 수정·테스트·Android/iOS 실행은 계획 PR의 완료 주장에 포함하지 않는다.
