# C11.4 auto-row `calc()` `row-gap` 구현 PR diff 적대 검토 30개

검토 대상은 staged 구현 diff 전체, 독립 oracle와 fixture, Android/iOS simulator 실행 근거, 제한 plan 및 내부 계약 candidate다. 기존 계획 review 20개와 구현/runtime review 20개를 재사용하지 않고 변경 결과의 별도 실패 경로를 확인했다.

| # | 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | bounded profile이 일반 CSS `calc()` 지원으로 확대 서술됨 | two-term length-percentage grammar, 지원 shape와 제외 범위를 plan/contract에 고정했다. C11.4/C11 완료를 주장하지 않는다. PASS |
| 2 | calc map을 만드는 조건이 다른 Grid profile의 계산을 바꿈 | CSS math index 추가는 `RootHtmlAutoRowStretchV1`의 calc row-gap 입력에 한정되고 기존 profile 분기를 보존한다. PASS |
| 3 | duplicate math id가 map insertion 순서에 따라 덮임 | 공통 index와 inner placement 검증 양쪽에서 duplicate ID를 오류로 거부하는 Layout test가 있다. PASS |
| 4 | missing math id가 0px 또는 다른 record로 대체됨 | node와 `row-gap`을 포함한 `MissingCssMath` 오류를 직접 주입해 검사한다. PASS |
| 5 | 다른 Grid node의 math record를 재사용 | owner mismatch가 `CssMathBindingMismatch`로 실패하는 test가 있다. PASS |
| 6 | `ColumnGap` record가 row-gap으로 소비됨 | property mismatch를 오류로 주입하고 validator가 `row-gap` 경계로 거부한다. PASS |
| 7 | `px + px`, 단일 term 또는 nested expression이 새 grammar에 섞임 | projection 및 layout 양쪽에서 length-percentage 2-term 외 AST를 fail-closed 처리한다. PASS |
| 8 | percentage와 length 항 순서에 따라 의미가 달라짐 | 양 순서를 120-scene oracle과 typed projection grammar에 포함한다. PASS |
| 9 | unary negative operand에서 부호 또는 dimension을 잃음 | length/percentage 각각의 direct unary negate를 검사하고 일반 nested negate는 허용하지 않는다. PASS |
| 10 | NaN/infinity가 typed projection에서 admission됨 | projection grammar와 layout 결과의 finite 검사에 비유한 입력 negative test를 둔다. PASS |
| 11 | 큰 유한 operand가 기존 CSS math range를 건너뛰거나 잘못 거부됨 | 기존 resolver range를 적용한 `f32::MAX` test와 이후 total Grid extent guard를 대조했다. PASS |
| 12 | percentage basis가 viewport·width·border-box·indefinite height로 바뀜 | 고정 40px content-box block-size만 기준으로 쓰고 auto height를 기존 root-size 오류로 거부한다. PASS |
| 13 | grid extent 계산에서 gutter 개수가 item 수와 혼동됨 | 두 gutter와 30px base-row extent의 표현 가능성 검사 및 2/3-item geometry matrix가 있다. PASS |
| 14 | 기존 px/direct percentage row-gap 경로가 바뀜 | 기존 supported range와 percentage regression을 유지하고 RuntimeSession row-gap suite 6/6을 기록했다. PASS |
| 15 | Taffy gutter와 수동 adapter 보정이 중복 적용됨 | 수동 geometry 보정은 없고 Taffy track trace 및 item frame을 독립 기대값과 대조한다. PASS |
| 16 | explicit 10px row가 `normal`/`stretch`에서 잘못 확장됨 | expected rows와 `StretchAutoTracks` trace를 120 scene 모두에서 비교한다. PASS |
| 17 | negative free-space alignment가 positive distribution처럼 동작 | 10개 alignment를 음수·양수·zero free-space 계산과 Chrome 관측에 교차 대조한다. PASS |
| 18 | projection이 authored math 대신 `Unsupported` sentinel에서 거부 | CSS math record는 typed `RowGap` binding으로 전달되고 제한 grammar 검사를 통과해야만 허용된다. PASS |
| 19 | projection에서 기존 two-item positional alignment 제한이 열림 | positional/distributed 값의 세-item admission을 유지하고 two-item full matrix는 Layout-only다. PASS |
| 20 | inner one-axis placement validator가 math map 없이 valid row-gap을 거부 | 같은 owner/property binding을 해당 validator에도 전달하며 missing/misbound 입력을 별도로 검사한다. PASS |
| 21 | 기대 JSON이 Chrome 관측 결과를 재사용해 oracle 순환을 만듦 | 독립 Python generator는 `max(0, 40 × percentage + px)`와 CSS sizing 식만 사용한다. PASS |
| 22 | 120 scene 또는 1,500 coordinate 집계가 누락·중복 | 6 expressions × 2 item counts × 10 alignments와 모든 item-frame/row coordinates를 assert한다. PASS |
| 23 | item frame만 맞고 used row sizing이 달라짐 | 각 장면에서 auto-track stretch stage의 gutter 제외 row base sizes도 비교한다. PASS |
| 24 | 허용 오차가 관측 뒤 편의상 확대됨 | 사전 고정 허용치 `1/32px`는 기록된 최대 오차 `0.0208333334px`보다 크고 matrix 전체에 동일하게 적용된다. PASS |
| 25 | RuntimeSession test가 fixture route만 통과해 기본 경로를 놓침 | `new_runtime_gpu_with_root_html()` 기본 route에서 3개의 calc representative scene과 failure/recovery를 실행한다. PASS |
| 26 | unsupported calc 실패 후 이전 완료 장면이 부분 결과로 남거나 복구 불가 | dark media override에서 `Failed`·`completed=None`, light 복원에서 새 `Ready` 완료를 확인한다. PASS |
| 27 | simulator positive/overflow fixture가 oracle와 다른 style shape를 가짐 | 양 fixture는 고정 40×40 Grid, 10px explicit row, 세 direct item과 calc 값만 다르게 구성하고 입력 hash를 기록한다. PASS |
| 28 | simulator harness가 사용 중 앱이나 기존 evidence를 덮어씀 | 대상 앱 실행 여부와 pass 디렉터리를 사전 검사하고, 설치/build assets를 백업한 후 hash 대조 복원한다. PASS |
| 29 | simulator 검증 중 이전 사용자 앱·bootstrap 입력이 손실됨 | 복원 완료 hash와 실패 시 보존 recovery backup 경계를 README 및 pass logs에 기록한다. PASS |
| 30 | OS screenshot을 PixelCopy/Metal surface parity 또는 C11 완료로 과장 | pixel analysis는 post-compositor OS PNG 경계만 말하며 plan/contract는 WPT·실기기·surface readback·C11 미완료를 명시한다. PASS |

## 판정

이 diff 검토에서 blocking finding은 없었다. Rust focused checks와 simulator run은 별도 실행 근거에 기록하며, 이 문서는 GitHub CI나 PR merge를 증명하지 않는다. 공식 C11 checkbox 및 `78/214`는 그대로 유지한다.
