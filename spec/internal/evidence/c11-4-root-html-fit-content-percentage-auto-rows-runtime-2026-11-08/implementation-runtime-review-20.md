# C11.4 `fit-content(percentage)` implicit rows 구현/runtime 실패 관점 검토 20개

검토 대상은 이번 변경의 Style projection, Grid layout admission·auto-placement, parsed Root HTML `RuntimeSession`, focused test 결과와 두 Simulator 실행 근거다. 계획 검토 표는 재사용하지 않았다.

| # | 독립 실패 관점 | 확인 근거와 판정 |
| --- | --- | --- |
| 1 | 다른 Grid profile까지 percentage `fit-content()`가 새어 들어가는가 | `RootHtmlFitContentAutoRowsV1`만 직접 허용하고 기존 `RootHtmlFitContentV1`와 분리했다. Style projection은 exact parsed Root HTML profile에서만 이 mode를 고른다. 통과. |
| 2 | 여러 implicit auto-row track이 singleton 계약을 우회하는가 | candidate는 한 항목짜리 track list만 받으며 RuntimeSession의 혼합 list negative가 partial output 없이 실패한다. 통과. |
| 3 | negative percentage가 Taffy에 전달되는가 | Style projection과 layout admission에서 각각 `-0.01`을 거부한다. 통과. |
| 4 | NaN 또는 무한 percentage가 크기·allocation 계산에 들어가는가 | Style projection과 layout admission에서 NaN/+∞를 거부한다. 통과. |
| 5 | `0%`가 invalid 또는 auto row로 재해석되는가 | computed typed `Percentage(0.0)`를 보존하고 사용 행 높이 0px, 다음 probe y=10px을 확인했다. 통과. |
| 6 | 직접 percentage typed ratio가 px로 바뀌는가 | 20/24/34/40/120%가 `ComputedCssGridTrackLengthPercentage::Percentage`로 RuntimeSession cascade와 layout input에 보존된다. 통과. |
| 7 | percentage basis가 viewport 높이로 잘못 선택되는가 | 633px CSS viewport에서 20/24%가 각각 20/24px 행을 만든다. viewport를 basis로 썼을 때 예상되는 max-content cap 결과와 다르다. 통과. |
| 8 | basis가 indefinite일 때 cyclic sizing을 임의로 결정하는가 | `height:auto` negative는 기본 Root HTML route에서 실패하고 completed output을 발행하지 않는다. 통과. |
| 9 | cap 아래 max-content contribution의 사용 크기가 잘못되는가 | 24% cap과 34px payload에서 used row 24px, probe y=34px을 확인했다. 통과. |
| 10 | cap과 max-content가 같은 경계에서 반올림·비교가 어긋나는가 | 34% cap에서 row 34px 및 probe y=44px을 확인했다. 통과. |
| 11 | cap이 max-content보다 크면 row가 content보다 불필요하게 커지는가 | 40%와 120% 모두 percentage typed value는 유지하면서 row는 34px max-content에서 멈춘다. 통과. |
| 12 | item minimum contribution이 cap보다 클 때 무시되는가 | 20% cap보다 큰 28px minimum contribution이 used row 28px로 반영되고 probe y=38px이다. 통과. |
| 13 | 같은 auto-row sizing function이 다음 implicit row에서 반복되는가 | 6px probe가 두 번째 implicit row를 만들고 해당 행 6px·위치 y=`10px + preceding row`를 확인했다. 통과. |
| 14 | explicit row나 40px explicit column까지 percentage가 적용되는가 | explicit rows `[10]px`, columns `[40]px`와 각각의 item frame을 전체 oracle matrix에서 확인했다. 통과. |
| 15 | item의 34px nested content가 cap 때문에 잘리거나 이동되는가 | item frame cap을 넘는 nested payload 34px와 visible overflow frame `[0,10,40,34]`를 확인했다. 통과. |
| 16 | `fit-content(calc(...))`가 direct percentage와 혼동되는가 | percentage term이 있는 `Calc`를 CSS cascade rejection과 dark-mode same-session negative에 사용했고 partial output 없이 거부한 뒤 light-mode direct percentage로 복구했다. 통과. |
| 17 | areas, intrinsic explicit track, gap, column flow, explicit placement가 profile에 함께 들어오는가 | 8개 Root HTML runtime negative가 모두 `Failed`이며 completed layout/render를 내지 않는다. 통과. |
| 18 | 실패한 revision 뒤 이전·부분 scene이 성공 결과처럼 노출되는가 | 실패 snapshot의 `completed=None`을 검사하고 같은 RuntimeSession에서 유효한 light-mode percentage로 회복한다. 통과. |
| 19 | 기존 px·length normalization·explicit percentage route가 깨지는가 | 기존 px runtime positive/rejection 2/2와 explicit fit-content px/percentage projection regression을 다시 실행했다. 통과. |
| 20 | Simulator 화면이 CSS geometry, readback 또는 사용자 데이터 보존을 과장하는가 | Rust frame assertions와 분리해 AVD/iOS screenshot은 visual smoke로만 기록했다. iOS `simctl install` 뒤 data-container 경로가 달라져 기존 데이터 연속성은 확인 불가로 남겼다. 설치 전/후 경로와 실행 한계는 README에 기록했다. |

## 실행 요약

- Style projection percentage implicit-row gate: 1/1.
- Layout px/percentage cap oracle 및 malformed-cap rejection: 2/2.
- RuntimeSession percentage positive, unsupported-shape atomic rejection, dark-calc recovery: 3/3.
- 기존 px positive/rejection 회귀: 2/2.
- 기존 explicit fit-content projection px/percentage 회귀: 2/2.
- 변경 crate all-target Clippy, `cargo fmt --all -- --check`, `git diff --check`: 통과.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML 앱이 `layout=ready`, `boxes=7`, `page_scale=1`을 기록했다. 앱 로그와 화면은 track geometry 표면 readback이 아니다.

각 명령의 원본 출력은 같은 디렉터리의 `checks/`에 있다. Android는 ANGLE/SwiftShader AVD이며 hardware GPU evidence가 아니다. iOS는 compositor 제출 뒤 pixel 비교가 아니다. 실기기, WPT, 전체 C11.4/C11 적합성은 검증하지 않았다.
