# C11.4 fixed-repeat 그룹 수 제한 제거 · 구현/runtime 실패 관점 20개

구현 코드·cascade tests·opt-in runtime fixture·Android/iOS simulator evidence를 서로 다른 실패 경로로 대조했다. 계획 검토표를 재사용하지 않았다.

초기 검증에서 두 가지를 수정했다. 첫 Rust compile에서 그룹 카운터의 정수형 추론이 모호해 `usize`를 명시했다. 첫 Android capture에서는 새 320×80 columns scene과 row item `(0,74)`를 기존 고정 좌표 validator가 거부했다. Android/iOS validator의 frame·pixel 기대값을 scene에 맞춰 추가한 뒤 최종 12-scene matrix를 다시 실행했다. iOS build의 새 unused-local 경고도 정리했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 기존 두-group 고정 배열 또는 숨은 두 개 상한이 남는가 | 동적 `Vec<Option<FixedRepeatGroup>>`; 3·4·8 groups cascade test 통과 |
| 2 | count 0, 음수 또는 `usize` 변환 실패가 들어오는가 | 양수 변환을 metadata 생성 전에 검증하고 fail closed |
| 3 | auto-fill/auto-fit 둘 이상이 하나의 list에 들어오는가 | Stylo parser diagnostic와 비어 있는 computed columns를 검사 |
| 4 | auto-repeat이 없거나 list 경계 metadata가 깨져도 진행되는가 | 필수 repeat·outer line-group cardinality 검사 유지 |
| 5 | authored component 수가 10,000을 넘어도 metadata를 먼저 할당하는가 | 첫 pass에서 component cap 검사 후 allocation |
| 6 | fixed pattern이 비거나 3개 이상 track을 가지는가 | 1–2 computed track shape만 수용 |
| 7 | named line이 fixed/outer/auto repeat에 섞이는가 | 현재 profile gate가 모두 거부 |
| 8 | percentage, non-finite/negative breadth, flex, intrinsic, minmax가 확장되는가 | finite nonnegative `LengthPx` 외 breadth를 거부 |
| 9 | `count × pattern length`가 overflow하는가 | `checked_mul` 뒤 contextual unsupported error |
| 10 | expanded track 수 누계가 overflow하거나 10,000을 넘는가 | `checked_add`와 10,000 cap을 각 component 누계에 적용 |
| 11 | group count만큼 metadata를 반복 탐색해 제곱 시간으로 커지는가 | list component index로 직접 get/set |
| 12 | metadata reservation이 infallible하거나 allocation 실패가 panic이 되는가 | `try_reserve` helper와 `FixedRepeatGroups` failpoint 사용 |
| 13 | metadata 실패 뒤 부분 확장 결과가 노출되는가 | 확장 전 reserve·profile validation; contextual `Err` 반환 |
| 14 | allocation 실패 뒤 다음 cascade 호출이 오염되는가 | failure guard 다음 정상 호출 복구 test 통과 |
| 15 | prefix/중간/suffix group의 authored order와 auto-repeat start/end가 어긋나는가 | 2-group 회귀와 3·4·8-group authored-order test 통과 |
| 16 | row axis와 columns axis 중 한쪽만 처리되는가 | cascade 8-group rows case, runtime four-group rows case 통과 |
| 17 | template areas 또는 반대 축 auto-repeat과 혼합되는가 | 현재 unsupported 조합 거부 test 유지 |
| 18 | 10,000 descriptor boundary에서 누락·부분 track이 생기는가 | 10,000 통과, 10,001 거부, partial result 없음 |
| 19 | Chrome oracle·독립 반복 수·simulator scene이 서로 다른 geometry를 기대하는가 | 두 신규 scene: 23.4px tracks와 row y=74, 1/64px tolerance에서 교차 확인 |
| 20 | 앱 fixture가 일반 build에 들어가거나 캡처가 frame만 통과하고 pixel/trace 실패를 숨기는가 | 12-scene boundary test, Android/iOS 12/12 draw/trace/frame 및 각각 15/15 surface sample 통과; archive negative/positive control 통과 |

전체 결과와 raw evidence는 [runtime README](./README.md)에 있다. 전체 C11.4 계약, 실기기·hardware GPU, iOS compositor 이후 pixel, WPT는 이 검토의 판정 범위가 아니다.
