# C11.4 반복 intrinsic tail 구현/runtime 실패 관점 검토 20개

범위는 `fit-content()`를 첫 track에 고정하고 후행 multiset `[min-content×2, max-content×2, auto]`의 30개 순서를 제한 Root HTML Grid profile에 연결한 변경이다. 계획 검토표를 재사용하지 않았다. 각 관점은 projection·layout·RuntimeSession 또는 플랫폼 경계의 별도 실패 경로를 확인했다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | track list 길이가 5 또는 7이어도 30-order gate에 들어가는가 | 거부. Style projection의 short/long list negative cases가 통과했다. |
| 2 | finite non-negative cap 조건이 풀려 음수·NaN·무한대가 허용되는가 | 거부. 기존 typed-cap negative cases가 계속 통과했다. |
| 3 | `fit-content()`가 첫 member가 아니어도 후행 multiset으로 오인되는가 | 거부. `fit_not_first` projection/layout cases와 RuntimeSession dark-mode rejection이 확인됐다. |
| 4 | 후행에 `fit-content`, fixed length, percentage 등 미지정 member가 섞이는가 | 거부. helper는 세 허용 enum만 세고 그 외 variant에서 false를 반환한다. |
| 5 | min-content 또는 max-content 중복 수가 바뀌어도 허용되는가 | 거부. 세 번째 min-content/missing max-content projection 및 layout negative case가 통과했다. |
| 6 | `auto` 누락 또는 중복이 허용되는가 | 거부. multiset count가 정확히 `[2,2,1]`인지 검사한다. |
| 7 | PR #1284의 기존 `min max auto min max` 순서가 회귀하는가 | 회귀 없음. 모든 30-order matrix 안에 기존 order가 포함되고 projection/layout/runtime가 통과했다. |
| 8 | generator가 같은 순서를 두 번 만들고 다른 순서를 빠뜨리는가 | 재귀 생성은 남은 member count를 한 단계씩 소비해 각 prefix 선택이 고유 경로를 만든다. 30 leaf count와 기존 order 포함을 각 matrix에서 확인했다. |
| 9 | computed cap이 0% 또는 120%일 때 projection에서 순서 보존이 깨지는가 | 30 order 각각의 px, 20%, 0%, 120% cap을 포함한 120 typed projection cases가 통과했다. |
| 10 | CSS computed serialization이 typed order와 달라지는가 | RuntimeSession 120 scenes에서 computed track vector와 `grid-auto-rows` 문자열을 각각 authored permutation과 비교했다. |
| 11 | explicit 10px row가 implicit cycle의 첫 member로 잘못 소비되는가 | Layout 120 scenes에서 explicit row 10px와 첫 implicit fit row 20px를 분리해 비교했다. |
| 12 | 두 번째 cycle의 fit-content row가 cap을 넘는 child 때문에 커지는가 | 30 order × cap × alignment에서 60px payload의 item/payload frame과 Grid 밖 overflow를 독립 기대값에 대조했다. |
| 13 | fixed-px min/max contribution이 order 변경 때 잘못된 payload index를 참조하는가 | 8 implicit rows의 고정 payload 높이를 매 matrix case에서 개별 frame으로 비교했다. 임의 콘텐츠 기여는 주장하지 않는다. |
| 14 | auto-track stretch가 non-auto track에 퍼지거나 auto occurrence 수를 놓치는가 | `start|stretch` 120 layout/runtime scenes에서 free-space distribution을 order별 auto occurrence에만 계산해 비교했다. tail 첫 member가 auto인 2회 발생 경우도 포함된다. |
| 15 | 8개 implicit item이 첫 cycle만 쓰거나 cycle 경계를 넘지 않는가 | 6-member pattern의 8 implicit rows와 explicit row를 포함한 전체 frame matrix가 통과했다. |
| 16 | visible Grid owner·child topology가 바뀌어도 이 bounded profile로 통과하는가 | Runtime validator의 단일 owner, 아홉 direct items, payload shape 검사를 유지한다. 모든 120 scene은 고정 fixture topology를 사용한다. |
| 17 | unsupported tail이 계산 일부를 publish한 뒤 실패하는가 | same-session rejection test는 실패 revision에 completed layout이 없음을 확인한다. |
| 18 | 실패한 revision 뒤 같은 RuntimeSession이 복구하지 못하는가 | 기존 rejection/recovery runtime test가 dark-mode 거부 뒤 유효 revision 복구와 overflow frame을 확인했다. |
| 19 | simulator 앱이 fixture가 아니라 이전 bootstrap 문서를 실행하는가 | Android APK와 iOS app bundle의 index hash가 fixture와 같고, 두 새 process 로그가 20 boxes를 기록했다. |
| 20 | screenshot·시뮬레이터 결과를 surface pixel 또는 실기기 결과로 과장하는가 | 범위를 OS screenshot visual smoke, Android draw-presented, iOS layout-ready로 제한했다. PixelCopy·Metal readback·hardware GPU·실기기 결과는 주장하지 않는다. |

**결론:** 구현/runtime 범위에서 발견된 두 번째 exact-order gate는 `spinon-layout`에서 재현해 multiset 판정으로 수정했다. 그 뒤 projection, layout, RuntimeSession matrix와 simulator route가 통과했다. 남은 PR 변경분 검토 30개는 별도 문서에서 수행한다.
