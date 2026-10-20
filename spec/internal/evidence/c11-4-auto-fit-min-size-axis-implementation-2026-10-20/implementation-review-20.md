# C11.4 auto-fit definite minimum-size 구현 검토

검토 범위는 Taffy auto-fit track 초기화, definite minimum-size repeat count 회귀, C11.4 fixture, Android/iOS scene expectation, simulator capture 검증이다. 계획 검토 20개 항목과 겹치지 않도록 구현된 코드와 실제 런타임 실패 경로만 확인했다.

| # | 독립 검토 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | Columns와 Rows에서 definite minimum-size가 같은 repeat 수를 주는가 | `grid_track_sizing_tests.rs`의 양 축 추적이 repeat track 수 8개를 확인한다. Android와 iOS 각각 53 scene 실행도 통과했다. |
| 2 | minimum-size 계산이 preferred-size 경로와 섞이지 않는가 | `ResolveWithin`과 `min_width`/`min_height`를 쓰는 새 테스트 및 fixture를 확인했다. 기존 definite preferred-size scene은 별도 기대값으로 남아 있다. |
| 3 | 190/200/342px 경계의 반복 수가 off-by-one 없이 고정되는가 | 양 축·auto-fill/auto-fit 테스트에서 각각 2/3/4회, 즉 비-gutter track 6/8/10개를 확인한다. |
| 4 | neighbor가 양쪽에 있는 경우 gap을 repeat count에 포함하는가 | 구현 식과 기대 track 수가 prefix·suffix 두 track과 모든 내부 gap을 포함한다. 53 scene 양 플랫폼 캡처가 통과했다. |
| 5 | prefix만 있는 경우 repeat count 수식이 누락된 gap을 만드는가 | simulator scene은 200px minimum-size와 prefix track을 분리해 실행한다. trace 검사에서 track과 gutter 폭을 확인한다. |
| 6 | suffix만 있는 경우 마지막 gap을 중복 계산하는가 | suffix-only scene의 final trace를 양 플랫폼에서 검사해 suffix 보존과 collapse gutter를 확인했다. |
| 7 | explicit neighbor가 auto-fit collapse에 같이 접히는가 | Taffy의 `track_has_items` 판정은 반복 track index에만 적용된다. prefix/suffix track은 실행 결과에서 유지된다. |
| 8 | 빈 repeat가 다른 explicit track 사이에 있을 때 앞 gutter가 남는가 | 빈 track 생성 시 직전 gutter도 collapse한다. prefix-to-repeat 및 inter-repeat gutter 값을 trace로 확인했다. |
| 9 | 마지막 repeat 뒤 explicit suffix가 있을 때 끝 gutter 처리가 중복되는가 | 반복 뒤 suffix가 오는 기존 `auto-fit-fixed-neighbors`와 새 suffix fixture의 final trace를 확인했다. suffix와 내부 gap 값이 일치한다. |
| 10 | auto-fit repeat 자체가 마지막 template component인 경우 trailing gutter가 남는가 | 마지막 repeat의 reverse sweep이 trailing gutter와 빈 tail track만 접고 마지막 occupied track에서 멈춘다. 기존 empty-neighbor scene을 포함한 Android/iOS matrix가 통과했다. |
| 11 | 반복 시작 전 기존 implicit 또는 explicit track의 경계 gap이 잘못 접히는가 | 앞 gutter를 현재 track의 인접 gutter로만 변경한다. 명시된 prefix와 그 폭은 보존됨을 trace에서 확인했다. |
| 12 | 첫 repeat track에 item이 놓이면 그 track까지 collapse되는가 | single-item scene의 첫 flexible track은 유지되고 이후 빈 repeat만 접힌다. 예상 162px와 item 위치를 양 플랫폼에서 확인했다. |
| 13 | item이 두 repeat track을 span하면 둘 중 하나가 비는가 | span-two scene에서 두 track의 occupied 상태와 내부 gutter, 134px flexible track을 양 플랫폼에서 확인했다. |
| 14 | 뒤쪽 repeat에 item이 있으면 앞의 빈 repeat가 살아남는가 | second-repeat scene에서 앞 두 track이 접히고 index 4 track만 유지된다. root 좌표와 item 좌표가 `(20,20)`/축별 20px로 일치한다. |
| 15 | 뒤쪽 occupied track의 양쪽 gutter까지 과도하게 collapse되는가 | occupied track 앞쪽은 직전 빈 track 때문에, 뒤쪽은 다음 빈 track 때문에 각각 접힌다. 170px track과 양 플랫폼 trace가 기대값과 같다. |
| 16 | auto-fill 동작이 새 auto-fit gutter 변경에 영향받는가 | auto-fill scene은 새 조건에 들어가지 않는다. 기존 auto-fill fixed/pattern/overflow controls가 두 simulator에서 기존 기대값으로 통과했다. |
| 17 | `minmax(40px, 1fr) 20px`와 고정 `40px 20px` 패턴이 혼동되는가 | fixture의 `Pattern` 분기와 fixed-pattern scenes를 확인했다. fixed track scene은 flexible track 기대를 공유하지 않고 별도 trace로 통과했다. |
| 18 | fixture ID 중복 또는 잘못된 입력 문자열이 앱에서 실행될 수 있는가 | Rust fixture test가 20개 ID의 유일성과 생성 source의 길이·repeat 구문을 확인한다. 테스트 1개 통과. |
| 19 | Android와 iOS가 scene 수·box 수·좌표 기대를 다르게 표현하는가 | 양 플랫폼 adapter에서 20개 ID와 root/item/node frame 판정을 대조했고, 각 53 scene 결과가 전부 `drawPresented=true`다. |
| 20 | 테두리 경계와 root 바깥 item이 실제 표면에서 clip되는가 | Android Surface와 iOS Metal에서 root 안쪽 초록색, 경계 바깥 배경, root 바깥 빨간 item, 내부 배경 표본 4개가 모두 `exact=true`로 통과했다. |

## 결과

검토한 20개 실패 경로에서 수정이 필요한 구현 결함은 발견하지 못했다. 고정된 시뮬레이터 검증은 Android AVD `emulator-5554`와 iPhone 17 Pro iOS 26.2에서 실행했으며 물리 기기는 사용하지 않았다. 전체 결과와 화면은 같은 상위 `c11-4-auto-fit-min-size-axis-2026-10-20/` 디렉터리의 플랫폼별 run 디렉터리에 있다.

남은 범위는 이 변경에 포함되지 않는다. Chrome 155의 gutter 계산은 CSS Grid 규범 계산과 다르게 관찰되었고, out-of-flow Grid item은 C11.1 runtime에서 명시적으로 거부되어 새 positive runtime case로 주장하지 않는다.
