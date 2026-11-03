# C11.1 parsed Root HTML dense auto-placement 구현·runtime 실패 관점 검토 20개

검토 대상은 Root HTML computed-style projection, dense layout normalization, fixture-test FFI 경계, Android/iOS 실행 adapter, 고정 14-scene 입력과 최신 [Android/iOS 실행 근거](./README.md)다. 계획 검토와 실제 변경분 검토는 각각 별도 문서로 기록한다.

| # | 검토한 실패 경로 | 판정과 근거 |
| --- | --- | --- |
| 1 | generic layout 입력이나 기존 CSS profile이 새 Root HTML dense 계약으로 승격 | 제한 `RootHtmlDenseAutoPlacementV1`은 `RuntimeRootHtmlTextPaintV1`에서 계산된 `row dense`·`column dense`에만 선택된다. layout 전용 mode는 Root HTML validator 없이 공개 경로에서 선택되지 않는다. |
| 2 | `row dense`와 `column dense`가 문자열 재해석 중 섞이거나 typed flow를 잃음 | projection은 computed enum을 각각 `GridAutoFlow::RowDense`·`ColumnDense`로 보존한다. style projection 2개와 layout core 8개 focused test가 통과했다. |
| 3 | snapshot의 고아·stale Grid style이 live tree의 dense route를 열어 줌 | dense owner 집합은 `runtime_grid_nodes.grid_containers`와 교집합을 만든다. 검토용 단위 test는 고아 dense style을 제외하고 sparse ancestor만 route 집합에 남기는 것을 확인한다. |
| 4 | nested mixed-flow 경로를 열면서 모든 sparse 조상의 배치도 무제한 허용 | ancestor route는 nested traversal에만 쓰고, 그 경로의 명시 placement는 별도 validator가 검사한다. `nested-mixed-flow` Android/iOS scene이 최종 행렬에서 통과했다. |
| 5 | sparse ancestor의 negative/named placement가 nested dense 지원을 통해 허용 | ancestor route는 `auto`, 양수 line, 양수 span만 받는다. property별 negative line·named line 거부 test가 통과했다. |
| 6 | zero span, zero line 또는 이름 있는 placement가 dense layout core로 전달 | Root HTML dense validator가 네 placement endpoint를 개별 검사한다. layout test는 0·음수·named line을 거부하고 양수 span을 유지한다. |
| 7 | template areas, repeat/auto-repeat, line-name metadata가 고정 px 지원 범위를 넓힘 | Root HTML dense track projection은 해당 topology를 명시적으로 거부하고 테스트에서 비-px explicit track도 막는다. |
| 8 | 퍼센트·fraction·intrinsic track이 fixed-px dense adapter에 전달 | 두 explicit 축은 finite non-negative direct px만 받고 projection test가 percentage track을 거부한다. |
| 9 | implicit track 목록·음수 px·비유한 값이 scalar 제한을 우회 | `grid-auto-rows`·`grid-auto-columns`는 `auto` 또는 유한 non-negative px scalar만 받는다. 잘못된 scalar와 목록에 property 오류가 발생한다. |
| 10 | implicit expansion이 필요한데 `auto` track content sizing을 지원하는 것처럼 성공 | no-expansion 대조는 통과하고 확장이 필요한 축은 해당 `grid-auto-*`로 거부한다. runtime test와 `row-dense-fixed-row` 기대 거부가 확인됐다. |
| 11 | dense cursor가 구멍을 다시 채우지 못하거나 sparse cursor를 재사용 | row·column dense hole-fill scene과 독립 Chrome expected geometry가 두 플랫폼에서 모두 0 CSS px 오차다. |
| 12 | definite item이 자동 item보다 나중에 occupancy에 반영되어 충돌 | `definite-first-dense`와 span-collision controls가 고정되어 두 플랫폼 행렬에서 일치한다. |
| 13 | hidden item이 occupancy에서 빠지거나 `display:none`이 빈 slot을 점유 | hidden-item control은 layout slot을 유지하고 화면 표본은 해당 item이 그리지 않는 결과와 맞는다. display-none subtree는 layout occupancy 입력에서 제외된다. |
| 14 | `visibility:collapse`가 `hidden`과 같다고 간주되어 특수 Grid 동작을 잘못 계산 | dense route는 `visible`·`hidden`만 받고 `collapse`를 `visibility` property 오류로 닫는다. unit test가 세 값을 분리한다. |
| 15 | nonzero order, positioned item, text/intrinsic item, auto sizing이 제한 geometry를 바꿈 | container/item validator는 이 값을 제한한다. layout tests는 nonzero order 및 unsupported item 조건을 확인하고 runtime matrix는 fixed-size element boxes만 계산한다. |
| 16 | 양 축 track quota를 넘거나 span arithmetic overflow가 부분 track을 남김 | 10,000 track 경계와 10,001 거부가 runtime test·matrix에 있다. reservation failure atomicity와 recovery test도 통과했다. |
| 17 | 실패한 layout이 이전 revision frame이나 부분 draw를 새 결과로 게시 | runtime rejection 로그에는 expected code/property/node가 있고 failed scene의 completed frame은 비교에 포함하지 않는다. 생성기와 comparator는 실패를 success count에서 분리한다. |
| 18 | dense 재배치가 DOM/source order나 paint order를 바꿈 | item IDs와 authored frame list를 scene 비교에 보존하고 runtime test는 authored paint order를 확인한다. |
| 19 | Android density로 CSS viewport를 줄이거나 surface 좌표를 잘못 표본화 | 최종 viewport는 360×800 CSS px, AVD density·device scale·physical scale은 2.625, PixelCopy surface는 945×2100이다. comparator가 두 이전 잘못된 scale 유형을 거부한다. |
| 20 | iOS 표면·fixture artifact·run lifecycle을 과장하거나 다른 run 결과를 섞음 | iOS Metal drawable 52/52는 compositor 제출 전 결과로 제한한다. 두 행렬은 scene별 fresh process·run ID를 기록했고 host archive feature-off/on 경계는 별도 근거에 있다. Android 앱은 종료됐고 다른 AVD PID 4193은 보존했다. |

최종 Rust focused 결과는 layout 8/8, style-to-layout Root HTML dense 4/4 및 route 경계 2/2, runtime 5/5다. fixture boundary 32/32, Android/iOS pixel comparator 71/71, scoped Clippy `-D warnings`, rustfmt와 `git diff --check`도 통과했다. Android와 iOS 각각 14 scene 중 성공 12·기대 거부 2, geometry 최대 오차 0 CSS px, 표면 표본 52/52 exact다. Android AVD의 한 재실행은 예상 track-limit 거부 뒤 host screenshot timeout으로 제외했고, 선택 AVD를 data wipe 없이 재부팅한 뒤 최종 run을 다시 통과했다. 실기기 GPU, iOS post-compositor output, WPT 및 전체 C11은 검증하지 않았다.
