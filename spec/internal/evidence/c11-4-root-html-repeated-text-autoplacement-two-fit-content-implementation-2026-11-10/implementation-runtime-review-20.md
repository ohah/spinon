# C11.4 두 fit-content 자동 배치 구현/runtime 실패 관점 검토

검토 대상은 실제 구현 변경과 기본 Root HTML 경로의 RuntimeSession 동작이다. 사전 계획 검토 20개와 PR diff 검토 30개를 재사용하지 않았다.

| # | 독립 실패 경로 | 확인 결과 |
|---:|---|---|
| 1 | 여섯-member보다 짧거나 긴 목록이 새 profile을 우회 허용 | layout profile은 정확히 여섯 개를 요구하고 matrix는 authored list 1,440개를 검사한다. |
| 2 | cap 개수가 0·1개 또는 3개 이상이어도 two-fit profile로 승인 | profile predicate는 정확히 두 fit-content member만 센다. matrix와 3-fit negative가 통과했다. |
| 3 | 음수 cap이 minimum contribution을 왜곡 | px 음수 negative control이 fail-closed했다. |
| 4 | NaN·infinity cap이 resolved frame을 비유한으로 만듦 | layout predicate의 비유한 cap controls와 runtime negative가 거부됐다. |
| 5 | calc cap이 direct px/% 제한을 우회 | calc cap runtime negative가 거부됐다. |
| 6 | px와 percentage를 문자열 기준으로 같은 typed value로 취급 | RuntimeSession은 ComputedCssGridAutoRows의 정확한 variant와 authored order를 비교한다. |
| 7 | 여섯 위치 중 같은 authored position을 두 번 센다고 가정 | 입력은 선형 track list이며 15개 서로 다른 위치쌍 모두가 matrix에 나타난다. |
| 8 | 남은 member multiplicity가 min-content×2/max-content×1/auto×1에서 벗어남 | 잘못된 multiplicity runtime negative와 layout predicate test가 거부했다. |
| 9 | 허용되는 tail 순서 일부가 조용히 빠짐 | layout profile은 12개 고유 순서를 열거해 15×12 조합을 확인한다. |
| 10 | 기존 한-fit 자동 배치 경로를 two-fit 분기 변경으로 깨뜨림 | 기존 1,440-scene single-fit matrix가 통과했다. |
| 11 | row-span track list가 auto-placement union으로 넓어짐 | 두 profile predicate를 나눴고 row-span 기존 3,840 및 1,920/4,320 회귀가 통과했다. |
| 12 | auto-placement 겹침을 이용해 row-span min-height:auto 규칙을 확장 | 최소 높이 허용은 row-span topology일 때만 true이며 two-fit overlap negative가 거부됐다. |
| 13 | direct child가 9개가 아닌 입력을 repeated text로 잘못 해석 | 7-item negative가 fail-closed했다. |
| 14 | 빈 explicit-row control item이 반복 implicit track cycle에 포함 | matrix가 첫 explicit row와 뒤의 8개 implicit row를 따로 비교했다. |
| 15 | nested·복수 payload 또는 미지원 text grammar가 frame 정규화를 우회 | 기존 supported-text predicate와 direct child shape 검사 뒤에만 normalization하며 유효 matrix의 glyph/line shape가 전부 일치했다. |
| 16 | unsupported column/dense flow 또는 implicit column이 row profile에 섞임 | 세 가지 placement/topology negative가 거부됐다. |
| 17 | indefinite percentage basis에서 px로 임의 변환 | indefinite block-size negative가 거부되고 positive cap controls는 definite 220px basis에서만 통과했다. |
| 18 | align-content 값이 start/stretch 밖으로 넓어짐 | center negative가 거부됐다. |
| 19 | stretch가 fit/min/max-content row를 키움 | 2,880-scene matrix의 Chrome frame 비교에서 non-auto 변화가 맞고 auto occurrence만 늘었다. |
| 20 | 누락·중복·잘못 정렬된 trace나 stale revision이 부분 결과를 남김 | 새 layout trace test 4/4에서 fail-closed 및 출력 불변을 확인했고 RuntimeSession invalid-scheme→valid recovery도 통과했다. |

주요 근거는 [RuntimeSession matrix와 controls](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/two_fit_auto_placement.rs), [layout trace 검사](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/track_trace/tests.rs), [실행 README](./README.md)다.

이 검토는 고정 direct-text profile의 코드/runtime 실패 경로다. 일반 CSS Grid admission, 임의 문서 적합성, WPT, Android physical/hardware GPU, iOS compositor 이후 pixel 또는 전체 C11.4/C11 완료를 주장하지 않는다.
