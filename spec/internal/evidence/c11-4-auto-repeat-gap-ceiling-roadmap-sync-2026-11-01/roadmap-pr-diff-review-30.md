# C11.4 positive-gap auto-repeat ceiling 로드맵 PR diff 적대 검토 30개

검토 대상은 PR #985 전체 최종 diff다: Rust trace test, 계획·검토 문서, 상태 대장, 공개 로드맵, 실행 로그, Tailnet 동기화 기록 및 Agent Browser 캡처. 공식 구현 상태는 `spec/STATUS.md`, 입력·기대값은 계획과 실행 README, CSS Grid 2025 CRD, pinned Taffy 및 테스트 코드를 대조했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 기존 no-gap 테스트 helper의 입력 의미가 바뀌는가 | 기존 helper는 `LayoutGap::default()`를 새 helper에 전달하므로 기존 호출의 기본 gap 의미가 유지된다. 통과. |
| 2 | 새 helper에서 gap이 잘못된 axis에 설정되는가 | Columns case는 column gap만, Rows case는 row gap만 `1px`로 둔다. 통과. |
| 3 | 직교 축에도 gutter가 들어가 기대 frame을 오염시키는가 | orthogonal axis는 `LayoutGap::default()`의 0px gap을 유지한다. 통과. |
| 4 | basis와 고정 root dimension이 축에 따라 뒤바뀌는가 | 축별 `(width,height)`를 선택한 뒤 root dimensions를 설정한다. 통과. |
| 5 | named-line child의 반대축 크기가 target axis assertion을 오염시키는가 | child는 width/height 모두 1px로 두고 target axis frame만 비교한다. 통과. |
| 6 | test fixture가 의도한 두-track named pattern을 입력하지 않는가 | helper track/name 입력과 계획을 대조해 prefix, 반복 pair, suffix 순서를 확인했다. 통과. |
| 7 | exact-fit repetition 계산이 잘못되었는가 | `6 + 5R = 25,001`에서 `R=4,999`이며 독립 계산과 CSS Grid auto-repeat 규칙이 일치한다. 통과. |
| 8 | cap case가 실제로 ceiling을 요청하는가 | `25,006px`은 5,000회 반복을 요구하고 10,000-track cap을 시험한다. 통과. |
| 9 | expected track vector 길이가 10,000과 다른가 | exact-fit은 prefix + 4,999 pairs + suffix, cap은 prefix + 4,999 pairs + final 1px로 10,000개다. 통과. |
| 10 | cap 뒤 source-order truncation에서 suffix가 남는가 | cap expectation은 마지막 1px 반복 track을 두고 suffix 3px를 제외한다. 통과. |
| 11 | gap이 repetition sizing 결과에 반영되지 않고 track 수만 우연히 맞는가 | 두 입력 basis와 마지막 child offsets에 gap 누적을 assertion한다. 통과. |
| 12 | auto-fill tracks가 auto-fit처럼 collapse되는가 | 모든 track state에서 collapse flag가 false인지 검사한다. 통과. |
| 13 | 최대 trace가 과도한 상태를 만들어도 test가 무제한 메모리를 쓰는가 | 각 실행의 trace state 수가 100,000 미만인지 검사한다. 통과. |
| 14 | Rows 결과를 x/width로 비교하거나 Columns 결과를 y/height로 비교하는가 | match arm이 Columns의 `(x,width)`와 Rows의 `(y,height)`를 대응시킨다. 통과. |
| 15 | named-line child offset 또는 크기 기대값이 틀리는가 | exact-fit `24,993px`, cap `24,998px`, target size `1px`를 axis별로 확인한다. 통과. |
| 16 | child id가 helper에서 생성한 item과 다를 수 있는가 | layout helper의 item id와 `output.frames[&id(2)]`를 대조했다. 통과. |
| 17 | layout failure가 불투명한 unwrap panic이 되는가 | `unwrap_or_else`에 axis/basis label과 실제 error를 포함한다. 통과. |
| 18 | 4가지 조합 중 축 또는 basis 조합이 빠지는가 | 두 축과 두 basis의 중첩 loop가 4회 실행을 만든다. 통과. |
| 19 | production layout behavior가 변경되는가 | PR 코드 변경은 Grid sizing test 파일의 helper와 test에 한정된다. 통과. |
| 20 | test 결과가 실제 source revision과 분리되는가 | runtime evidence `source-revision.json` 및 로그가 candidate source hash에 묶여 있다. 통과. |
| 21 | 고정 Taffy cap이 CSS 표준 규정으로 서술되는가 | 계획은 내부 cap을 CSS general limit 또는 implicit-grid 규칙으로 확대하지 않는다. 통과. |
| 22 | 표준 oracle이 gap을 무시하는 반복 산식인가 | CSS Grid 2025 CRD §7.2.3.2와 산식 모두 gap을 포함한다. 통과. |
| 23 | 계획 범위에 시험하지 않은 pattern이 들어오는가 | auto-fit, 3+ tracks, 다양한 gap, percentage/calc gap은 제외 범위에 남는다. 통과. |
| 24 | 상태 대장이 하위 slice를 완료로 승격하는가 | `spec/STATUS.md`에는 근거 기록만 추가되고 C11.4/C11 체크 상태는 유지된다. 통과. |
| 25 | 공개 roadmap 카드가 상위 구현 완료를 암시하는가 | test-only, public admission 미완료, C11.4/C11 미완료를 기록한다. 통과. |
| 26 | README의 통과 숫자가 저장 로그와 맞지 않는가 | focused 1, ceiling module 4, track-sizing 53 결과와 rustfmt/Clippy 로그를 대조했다. 통과. |
| 27 | 계획/runtime/PR review가 서로 재사용되거나 범위가 섞이는가 | 계획 20, implementation/runtime 20, 최종 PR diff 30을 별도 파일로 둔다. 통과. |
| 28 | 로드맵 카드·계획·실행 근거 링크가 실제 배포 route에 없는가 | 관련 selected routes의 Tailnet HTTPS status 200과 SHA-256 일치를 확인했다. 통과. |
| 29 | Tailnet 동기화가 전수 검증이나 full dead-link 통과로 과장되는가 | selected route 1,029/1,029만 검증했고 full-site dead-link 검사는 비활성화라고 쓴다. 통과. |
| 30 | 화면 증거나 프로세스 상태가 앱 동작처럼 오인되는가 | 캡처를 문서 preview로 한정하고 mobile overflow를 측정했으며, `close --all` 뒤 browser process 부재와 Tailnet server 유지를 기록한다. 통과. |

## 판정

30개 관점에서 code/test, 독립 기대값, 공식 상태, 링크, 배포 hash/backup, 화면 범위를 확인했다. 발견한 차단 결함은 없다. 이 검토는 Rust 구현/runtime 20개 검토 및 계획 20개 검토와 별도다.
