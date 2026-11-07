# C11.4 percentage fit-content implicit rows · 구현 PR 변경분 실패 관점 검토 30개

검토 대상은 `origin/main`의 `183d8e493ea8664475d5047ffe29078f0ac4b9c9`부터 구현 commit `117344ecc670d60dda02eddbb06f74d378b3367e`까지의 11개 소스 diff와 이 PR에 포함된 실행 근거다. 기존 계획 PR의 검토나 구현/runtime 실패 관점 20개 표를 재사용하지 않았다.

| # | PR diff 관점 | 변경분 확인과 판정 |
| --- | --- | --- |
| 1 | 변경 경계가 계획된 C11.4 slice에 한정되는가 | 기본 parsed Root HTML singleton implicit-row `fit-content()` cap만 넓혔다. 공개 API, FFI ABI, 다른 Grid feature를 변경하지 않았다. 통과. |
| 2 | px 경로의 기존 enum·호출부 연결이 깨지는가 | 내부 `FitContentPx` 이름을 값 종류 중립적인 `FitContent`로 바꾸고 px/percentage dispatch를 같이 연결했다. 전체 `rg` 검사와 focused 회귀에 이전 px 동작이 남는다. 통과. |
| 3 | explicit `fit-content()` route와 implicit route가 합쳐지는가 | 변경은 `RootHtmlFitContentAutoRowsV1` placement mode와 auto-row projection 분기에만 적용한다. explicit `RootHtmlFitContentV1` 경로와 분리된다. 통과. |
| 4 | computed percentage typed value가 px로 직렬화되는가 | projection은 `Percentage(ratio)`를 layout track까지 그대로 전달하며 RuntimeSession cascade assertion도 authored ratio/100을 검사한다. 통과. |
| 5 | percentage가 포함된 `calc()`를 direct percentage로 오인하는가 | singleton 직접 `Percentage`와 `Calc` AST는 다른 typed variant다. fixture의 dark rule과 RuntimeSession rejection이 Calc를 거부한다. 통과. |
| 6 | 여러 auto-row track이 singleton gate를 우회하는가 | style candidate와 layout validation 모두 정확히 한 `FitContent` track만 받는다. mixed list runtime negative도 있다. 통과. |
| 7 | negative cap이 style projection을 통과하는가 | projection test가 `-0.01`을 실패로 고정한다. 통과. |
| 8 | NaN 또는 infinity가 style projection을 통과하는가 | projection에서 `is_finite`와 non-negative 조건을 같이 적용하고 NaN/+∞ negative가 있다. 통과. |
| 9 | layout input 경계에서 invalid ratio가 살아남는가 | `validate_grid_layout`, auto-placement normalization 및 bounded contract가 동일 finite non-negative 조건을 확인한다. Layout test가 invalid percentage를 직접 검사한다. 통과. |
| 10 | fixed explicit track profile가 무심코 넓어지는가 | 기존 positive px explicit row/column 검사와 exact single-track grammar를 유지한다. intrinsic explicit column negative가 있다. 통과. |
| 11 | definite block-size 조건 없이 percentage resolver가 호출되는가 | 기존 exact Root HTML profile의 definite 40×100px container guard가 남고 `height:auto` runtime input은 atomic하게 실패한다. 통과. |
| 12 | percentage basis가 viewport나 border box로 바뀌는가 | layout oracle 및 Chrome comparison은 100px content-box basis를 고정한다. RuntimeSession은 viewport 633px에서 20/24px row를 기대한다. 통과. |
| 13 | 100% 초과 percentage를 임의로 clamp하는가 | `120%` 입력이 typed ratio `1.2`로 보존되고 max-content contribution 34px이 사용 크기를 결정한다. 통과. |
| 14 | `0%`가 거부되거나 auto로 치환되는가 | projection/runtime/layout oracle에 zero case가 있고 used implicit track은 0px이다. 통과. |
| 15 | cap보다 작은 max-content에서 track이 cap을 넘는가 | 24%/34px nested payload scene의 expected row와 probe 위치가 assertion된다. 통과. |
| 16 | cap보다 큰 max-content에서 track이 잘못 커지는가 | 34%, 40%, 120% 경계에서 Chrome expected row 34px과 layout/runtime matrix가 일치한다. 통과. |
| 17 | item automatic minimum이 cap 아래로 잘리는가 | 20% cap 아래 `min-height:28px` 장면에서 row 28px과 다음 item y=38px을 확인한다. 통과. |
| 18 | 후속 implicit row가 sizing function을 반복하지 않는가 | 6px probe가 다음 implicit row를 만들어 각 percentage case에서 expected y와 height를 검사한다. 통과. |
| 19 | item content가 track cap에 맞춰 잘려야 한다고 오해하는가 | 34px nested payload frame을 별도로 검사하고 visible overflow를 보존한다. 통과. |
| 20 | zero-height item을 render box lookup이 놓치는가 | 색상 기반 paint frame 조회가 불가능한 0px item은 completed layout frames에서 exact 0-height box를 확인한다. 통과. |
| 21 | 실패한 layout이 완료 snapshot을 일부 노출하는가 | mixed list, Calc 및 다른 profile negatives에서 `completed.is_none()`을 명시적으로 검사한다. 통과. |
| 22 | failure 뒤 같은 session이 복구되지 않는가 | dark Calc rejection 뒤 light direct percentage를 같은 RuntimeSession에 적용하고 Ready/completed 복귀를 검사한다. 통과. |
| 23 | `grid-template-areas`가 percentage profile에 섞이는가 | runtime negative가 implicit-row candidate에 areas를 더해 fail-closed 결과를 확인한다. 통과. |
| 24 | gap이나 column flow가 새 profile에 섞이는가 | nonzero row gap과 column auto-flow negative를 별도로 유지한다. 통과. |
| 25 | explicit placement가 sparse auto placement 제약을 우회하는가 | probe의 `grid-row-start:3` negative가 atomic failure를 확인한다. 통과. |
| 26 | fixtures가 실행 시 외부 파일·네트워크에 의존하는가 | runtime fixture는 self-contained HTML이며 test는 `include_str!`로 같은 pinned input을 사용한다. 통과. |
| 27 | fixture의 dark-mode control이 테스트와 분리되는가 | dark media rule은 percentage-bearing Calc를 만들고 same-session test가 dark rejection/light recovery를 실행한다. 통과. |
| 28 | 기존 px route의 rejection 설명이 stale하게 남는가 | 기존 px test의 same-session failure 메시지를 Calc 거부로 바꾸고 px positive/rejection regression을 별도 실행 근거에 유지한다. 통과. |
| 29 | 앱 화면을 geometry/readback 근거로 과장하는가 | Android AVD와 iOS Simulator 캡처를 visual smoke로 구분하고 CSS frame oracle은 Rust tests에 둔다. iOS data-container 경로 변경과 데이터 연속성 미확인도 README에 공개했다. 통과. |
| 30 | roadmap card 부재와 완료 상태를 혼동하는가 | live board Agent Browser 캡처는 `C11.4 · ACTIVE` 카드는 표시하지만 제목이 일반 `intrinsic track sizing`임을 기록한다. PR은 C11/C11.4 체크나 `78/214`를 변경하지 않는다. post-merge roadmap sync를 별도 후속 PR로 둔다. 통과. |

## 판정

실제 구현 diff에서 추가 수정이 필요한 결함은 찾지 못했다. 계획 검토 20개, 구현/runtime 검토 20개와 이 PR 변경분 검토 30개는 서로 다른 자료로 보존한다. 이 검토는 전체 C11.4/C11 적합성, WPT, 실기기 GPU 또는 iOS compositor 이후 pixel 동등성을 추가 주장하지 않는다.
