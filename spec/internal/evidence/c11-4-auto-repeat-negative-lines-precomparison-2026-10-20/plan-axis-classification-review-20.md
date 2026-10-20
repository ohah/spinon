# C11.4 auto-repeat 양축 분류 계획 재검토 20개

**대상:** 양축 auto-repeat의 secondary-axis negative line 분류, Taffy auto-track adapter, 현재 Chrome/Rust evidence 경계. 초기 계획과 1차 정정 검토에서 반복한 관점을 재사용하지 않는다.

| # | 새 실패 관점 | 확인 및 반영 |
|---:|---|---|
| 1 | columns repeat을 선택해 rows repeat metadata를 무시하는 경우 | 현재 branch가 columns를 우선 선택하고 나머지 axis를 authored length로 resolve하는 동작을 실제 source line으로 기록했다. 통과. |
| 2 | rows repeat을 선택할 때 columns 쪽 대칭 결함을 빠뜨리는 경우 | 계획은 양 방향 fixture를 요구하고 두 번째 반복 axis의 negative line을 각각 검증한다. 통과. |
| 3 | 양축 negative placement가 Taffy보다 먼저 실패한다는 보고가 추정에 불과한 경우 | Rust draft의 exact `InvalidGridPlacement`와 test 이름, pass/fail count를 별도 재현 문서에 기록했다. 통과. |
| 4 | 한 축 repeat 대조도 함께 망가졌다고 과장하는 경우 | 한 축 negative line test의 통과를 같은 실행에서 대조로 보존한다. 통과. |
| 5 | secondary axis의 단순 `definite` 분류에 line 좌표 연산을 불필요하게 넣는 경우 | plan은 Taffy로 넘길 raw placement를 보존하고 coordinate resolution 없이 definite/auto 상태만 분리하도록 한다. 통과. |
| 6 | definite classification이 `Line`과 `Span`을 혼동하는 경우 | 양쪽 `Line`과 `Span`/`Auto` 조합을 서로 다른 auto-placement classification 회귀로 명시한다. 통과. |
| 7 | reversed/equal numeric endpoints가 Taffy 전에 임의로 재정렬되는 경우 | raw line endpoints는 유지하고 Taffy placement algorithm을 회귀 기준으로 둔다. 통과. |
| 8 | secondary repeat axis의 positive line이 authored count를 넘지만 expanded repeat grid 안에 놓이는 경우 | 이 positive line도 양축 case에 포함하고 expanded grid 안의 정상 배치와 cap/error 경계를 각각 검사한다. 통과. |
| 9 | negative line maps explicit tracks와 leading implicit tracks를 같은 basis로 혼합하는 경우 | explicit negative line basis는 expanded explicit count이며 leading track origin shift는 used track/frame assertion으로 분리한다. 통과. |
| 10 | 두 축 repeat count를 한 축 count로 공유하는 경우 | columns/rows의 독립 fixed dimension, gap, 최소 track, count 대조를 유지한다. 통과. |
| 11 | non-repeating fixed axis의 기존 rejection/validation을 무심코 제거하는 경우 | repeat metadata가 없는 축은 기존 `resolve_axis` validation을 유지하도록 범위를 적었다. 통과. |
| 12 | `auto_placed_items`가 definite placement와 혼동되어 item sizing profile이 달라지는 경우 | coordinate resolving과 definite-vs-auto classification을 분리하고 TrackSizing dimension classification regression을 요구한다. 통과. |
| 13 | hidden item이 양축 occupancy 또는 auto-fit collapse를 바꾸는 경우 | hidden subtree filtering 후 visible item만 classification/occupancy하는 control을 명시했다. 통과. |
| 14 | auto-fit의 반복 count가 leading item만 보고 감소하는 경우 | auto-fit count는 auto-fill과 같고 배치 뒤 빈 반복 track만 collapse되는 것을 양 축에서 검사한다. 통과. |
| 15 | leading `grid-auto-*` vector가 Taffy에 전달되어도 cycle 순서가 달라지는 경우 | `[31,47]` 및 `[17,29]`의 1/2/3 leading track cycle과 frame origin을 기대값에 고정한다. 통과. |
| 16 | px가 아닌 intrinsic/list 값이 새 adapter 경로로 조용히 통과하는 경우 | Auto control, 유한한 px scalar, 비어 있지 않은 유한·0 이상 px 목록으로 지원 범위를 제한하고 다른 문법을 거부한다. 통과. |
| 17 | list 지원을 Taffy field 설정만으로 끝내고 Spinon validation을 생략하는 경우 | auto-repeat track list의 px-only validator, empty/negative/over-cap list 오류를 요구한다. 통과. |
| 18 | adapter 수정이 auto-repeat 없는 일반 Grid의 수동 확장과 이중 적용되는 경우 | Taffy auto track 필드는 auto-repeat 전용 경로에만 연결하고 일반 expanded track 경로는 기존 로직을 유지한다. 통과. |
| 19 | 임시 draft 테스트 실패를 병합된 제품 결함 또는 simulator evidence로 게시하는 경우 | draft test source와 실행 범위를 명시하고 현재 PR은 계획/evidence만 수정한다. 통과. |
| 20 | 반복 정정이 공식 C11.4 지원/완료 표시를 앞당기는 경우 | spec STATUS 및 contract completion count를 수정하지 않도록 plan에 명시한다. 통과. |

## 판정

양축 auto-repeat 결함과 auto-track adapter 제한을 분리했다. 한 축의 Taffy post-expansion line path는 유지하고, 양축의 secondary-axis pre-resolution만 수정 대상으로 삼는다. runtime 수정과 simulator 결과는 아직 없다.
