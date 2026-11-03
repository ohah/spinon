# C11.1 `calc()` implicit-column 구현 PR diff 실패 관점 검토 30개

현재 checkout의 실제 구현·테스트·계획·실행 근거 diff를 대상으로 별도 검토했다. 구현/runtime 실패 관점 20개와 다른 계약 경계 및 실패 경로를 확인했다.

| # | 검토 관점 | 확인 결과 |
|---:|---|---|
| 1 | 일반 CSS cascade에서 새 calc track이 노출되는가 | cascade option은 `RuntimeRootHtmlTextPaintV1`에서만 켜진다. 기존 CSS admission은 바뀌지 않았다. |
| 2 | `grid-auto-flow:column dense`도 허용되는가 | 정확한 `Column` flow만 option을 켠다. dense와 row-dense는 calc를 보존하지 않는다. |
| 3 | `grid-auto-rows`가 같은 calc option을 공유하는가 | row option은 계속 `false`; auto-row 수식은 이 변경으로 통과하지 않는다. |
| 4 | explicit `grid-template-columns/rows`로 지원이 새는가 | Root HTML column-flow의 explicit track 검사는 계속 direct finite non-negative px만 받는다. |
| 5 | template areas, repeat, named line이 calc admission으로 우회되는가 | 별도 column-flow candidate와 layout 계약이 area·repeat·line metadata를 계속 거부한다. |
| 6 | top-level 계산식 인자 수 제한이 맞는가 | cascade projection과 layout validator 모두 정확히 두 항의 `Sum`만 받는다. |
| 7 | 두 항의 dimension 조합이 넓어지는가 | 유한 px 하나와 유한 percentage 하나만 허용하고 중복 dimension은 거부한다. |
| 8 | px와 percentage 항 순서가 바뀌는가 | 양쪽 순서를 받으며 별도 projection test가 node/property-bound 결과를 확인한다. |
| 9 | 음수 항의 sign이 계산 전에 사라지는가 | 직접 항과 한 단계 `Negate`만 받는다. `calc(1px - 10%)` layout test가 0px clamp를 확인한다. |
| 10 | 중첩 연산·곱셈·나눗셈·min/max/clamp가 통과하는가 | 지원 AST 바깥의 operator/tree는 두 단계에서 fail-closed한다. |
| 11 | NaN·무한대가 finite check를 우회하는가 | 두 typed term을 projection과 layout admission에서 검사하며 비유한 test 입력을 거부한다. |
| 12 | 문법 오류와 유효하지만 미지원인 CSS가 같은 경로인가 | 잘못된 `calc()`는 cascade diagnostic, 유효한 `min()`은 projection rejection으로 분리된다. |
| 13 | 상수 calc가 새 percentage-basis 경로로 강제되는가 | Stylo가 `calc(2px * 3)`을 direct `6px`로 정규화하는 결과를 E2E test가 고정한다. |
| 14 | 계산식 ID가 다른 property로 투영되는가 | auto-column calc는 `GridTemplateColumns` binding으로 투영하고 잘못된 property test에서 거부한다. |
| 15 | ID 소유자가 다른 Grid node일 때 통과하는가 | layout lookup이 node/property를 다시 확인하고 wrong-owner test는 mismatch를 받는다. |
| 16 | ID 누락·중복이 Taffy 입력까지 도달하는가 | 누락은 calc 사용점에서 거부한다. duplicate ID는 공통 binding 검증과 index 생성 모두에서 거부한다. |
| 17 | 10,000개 calc 목록에서 ID 탐색이 제곱 시간인가 | 첫 검토에서 track마다 전체 math slice를 선형 검색하는 경로를 발견했다. 두 validation pass에서 ID `BTreeMap`을 한 번 구성하고 각 사용은 logarithmic lookup을 하도록 수정했다. 성능 benchmark는 실행하지 않았다. |
| 18 | CSS math index가 잘못된 경우를 덮어쓰는가 | `index_css_math`는 중복 ID에서 오류를 반환하며 유효 입력에서만 lookup을 진행한다. |
| 19 | style projection이 10,000-track quota를 우회하는가 | auto-column list는 기존 shared `GRID_MAX_TRACKS`와 fallible reservation을 유지한다. |
| 20 | allocation 실패가 partial computed Grid를 공개하는가 | cascade와 layout의 기존 fallible allocation/error 반환 경로가 보존된다. |
| 21 | percentage basis가 viewport나 gutter 차감 폭인가 | definite Grid width를 CSS percentage basis로 쓰고 gap을 별도로 더한다. 29px calc 장면이 이를 고정한다. |
| 22 | auto·math width 또는 min/max/aspect-ratio constraint를 basis로 받는가 | direct finite width만 허용하며 width 계산식, min/max, aspect-ratio는 projection/layout에서 거부한다. |
| 23 | definite `0px` width가 무효로 취급되는가 | zero-basis test에서 percentage 항은 0px이고 px 항은 5px인 scene이 통과한다. |
| 24 | 음수 used track이 placement로 전파되는가 | `GridTemplateColumns` non-negative censor가 적용되고 item 좌표는 0px implicit track에 맞는다. |
| 25 | 100% 초과 track을 container 크기로 자르는가 | `calc(120% + 1px)`가 240px basis에서 289px로 남는 좌표를 확인했다. |
| 26 | 수식 개별값 cap이나 cumulative extent가 빠졌는가 | 기존 33,554,428px CSS math ceiling과 별도 cumulative finite-f32 검사를 유지한다. |
| 27 | auto-column pattern 반복 중 explicit prefix나 순서가 바뀌는가 | expanded-column verifier가 explicit prefix, exact track count, 반복된 typed track/id를 비교한다. |
| 28 | calc와 direct `fr` 혼합 시 남은 공간 분배가 바뀌는가 | mixed parsed Root HTML test와 layout test가 29px calc, 43.5px fraction 및 후속 좌표를 확인한다. |
| 29 | 최종 모바일 실행의 source·환경·pixel 주장이 과장되는가 | 최신 재빌드된 코드가 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 15-box layout/draw를 기록했다. iOS의 오래된 environment 요청 `-12` 뒤 최신 요청은 `status=0`이다. 캡처는 visual smoke이며 surface pixel readback이 아니다. |
| 30 | PR 문서가 후보를 공식 완료나 CI 통과로 표시하는가 | 계획과 실행 README는 merge 전 candidate, 공식 `72/213`, CI 미확인을 유지한다. STATUS 및 내부 계약은 이 PR 전 변경하지 않았다. |

## 검토 결과

해결하지 않은 구현 diff 결함은 발견하지 못했다. 항목 17의 repeated-linear-scan은 ID index로 수정했고, 그 뒤 layout focused tests 25/25, CSS math ceiling test 1/1, layout Clippy와 rustfmt가 통과했다. 앱 재빌드와 Android/iOS Simulator runtime도 갱신했다. CI, 실기기, hardware GPU, WPT, Android PixelCopy, iOS Metal drawable readback 및 compositor 이후 pixel 동등성은 확인하지 않았다.
