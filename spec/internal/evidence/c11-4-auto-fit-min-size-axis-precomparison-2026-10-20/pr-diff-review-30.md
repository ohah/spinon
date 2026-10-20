# C11.4 auto-fit 축 차이 계획 PR diff 적대 검토 30개

**대상:** 계획, 사전 비교 fixture/관찰 자료, 상태 대장 및 공개 로드맵 변경. 20개 계획 검토와 별개로 최종 PR diff의 계약·근거·링크·상태 경계를 확인한다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | 계획 PR에 Rust/runtime 제품 코드가 섞여 구현 범위를 미리 넓히는가? | **통과.** diff에는 제품 Rust·앱 구현이 없고 계획, HTML oracle, 근거, 상태·로드맵만 포함한다. |
| 2 | C11.4의 공식 완료 체크를 계획 추가만으로 켜는가? | **통과.** `spec/STATUS.md`의 C11.4 체크는 미완료로 유지된다. |
| 3 | 계획·브라우저 관찰을 제품 구현 완료 수에 더하는가? | **통과.** 상태 대장의 official count 변경은 없고 공개 로드맵도 미완료를 명시한다. |
| 4 | 로드맵 기록이 Chrome 관찰을 Spinon 동작으로 소개하는가? | **통과.** public roadmap은 조사 계획과 Chrome 사전 비교로만 링크하며 runtime 결과를 주장하지 않는다. |
| 5 | 작업의 상위 의존성과 순서가 생략되는가? | **통과.** 현재 계획 PR 뒤에 구현 PR을 최신 `main`에서 시작한다고 적었다. |
| 6 | 내부 계약 버전을 계획만으로 올리는가? | **통과.** 계약 0059를 변경하지 않고 버전도 올리지 않는다. |
| 7 | `auto-fit` repeat count 규범의 기준이 명시되는가? | **통과.** CSS Grid Level 1 §7.2.3.2를 독립 기준으로 연결한다. |
| 8 | Chrome 155의 축별 불일치를 기대값으로 복사하는가? | **통과.** Chrome은 관찰로 표시하고 columns의 2회 결과를 규범 판정에 사용하지 않는다. |
| 9 | minimum-size 반복 수를 preferred/max-size 계산과 혼동하는가? | **통과.** 200px definite minimum-size 기대 3회와 definite preferred-size control 2회를 다른 경로로 기술한다. |
| 10 | `minmax(40px, 1fr) 20px`의 고정 두 번째 track을 반복 기여에서 빠뜨리는가? | **통과.** minimum contribution에 40px과 20px을 모두 포함한다. |
| 11 | 양쪽 explicit neighbor와 8px gutter 수가 잘못 계산되는가? | **통과.** both-neighbor 식 `38 + 76n`은 prefix·suffix, 반복 내부 및 경계 gutter를 반영한다. |
| 12 | prefix-only와 suffix-only control이 같은 식으로 뭉개지는가? | **통과.** 각각 `20 + 76n`과 `10 + 76n`으로 분리한다. |
| 13 | neighbor 없는 list의 처음/마지막 gutter를 두 번 세는가? | **통과.** no-neighbor 식 `68n - 8`과 양 축 control을 명시한다. |
| 14 | 190/200/342px에서 ceil 최소 충족과 definite 최대 fit 경계를 바꾸는가? | **통과.** minimum-size 경계 2/3/4와 preferred-size control 2를 별도로 고정한다. |
| 15 | logical columns/rows case가 한 물리 축의 결과에만 의존하는가? | **통과.** columns와 rows를 독립 기록하고 `vertical-rl`은 물리 방향 분리 진단으로 표시한다. |
| 16 | `auto-fill`과 `auto-fit`의 count 계산 및 사후 collapse 순서를 뒤섞는가? | **통과.** 같은 pre-placement count와 placement 이후 collapse를 분리한다. |
| 17 | in-flow 단일 item이 첫 repeated track을 occupied로 만드는 조건이 빠지는가? | **통과.** 단일 item control과 첫 repeated track 배치 기준이 계획에 있다. |
| 18 | span-two item의 두 번째 repeated track을 비어 있다고 접는가? | **통과.** 두 track 모두 occupied 상태로 남아야 한다고 판정한다. |
| 19 | out-of-flow item을 occupancy로 세어 빈 track collapse를 가리는가? | **통과.** fixture와 계획에서 out-of-flow를 occupancy로 세지 않는 control을 포함한다. |
| 20 | collapse 시 explicit neighbor 또는 비대상 gutter까지 제거하는가? | **통과.** explicit track 보존과 repeated empty track 및 인접 gutter collapse를 따로 검증한다. |
| 21 | repeat component track 수를 반복 횟수로 오인하거나 explicit track을 포함하는가? | **통과.** JSON은 `repeatCount`, `repeatedTrackCount`, 전체 resolved track list를 별도 저장한다. |
| 22 | 관찰 자료가 case를 재구성할 CSS 입력이나 실제 resolved 결과 없이 요약만 남기는가? | **통과.** 30개 case의 mode, axis, basis, item, CSS size, track list와 geometry를 JSON에 보존한다. |
| 23 | Agent Browser/Chrome 환경 차이가 비교를 재현할 수 없게 남는가? | **통과.** Chrome, V8, Agent Browser, OS, viewport, DPR과 user agent를 기록한다. |
| 24 | source revision 출처가 실제 관찰 바이너리와 같다고 과장되는가? | **통과.** Chrome revision과 공개 Gitiles commit을 다르게 기록하고 exact-source 여부를 부정한다. |
| 25 | 공개 Blink 코드의 가설을 확정 root cause로 단정하는가? | **통과.** private available-size 입력이 미측정이라 diagnostic hypothesis로 한정한다. |
| 26 | CSS spec 계산과 Chromium source 조사 중 무엇이 합격 기준인지 모호한가? | **통과.** spec-derived counts를 acceptance로, source 조사를 진단 단서로 구분한다. |
| 27 | 저장된 screenshot이 Spinon 결과 또는 앱 검증 이미지로 오인되는가? | **통과.** README가 Chrome reference 전용이며 Spinon runtime 증거가 아니라고 표시한다. |
| 28 | screenshot/fixture/JSON이 본문 기재 크기·내용과 다른 파일로 바뀌었는가? | **통과.** fixture·JSON·두 PNG SHA-256이 README 값과 일치하며 JSON의 `cases`는 30개다. |
| 29 | 새 문서 링크가 정적 문서 빌드의 Rspress route 또는 아직 존재하지 않는 `main` 파일을 가리키는가? | **통과.** 새 저장소 링크는 PR 브랜치 경로를 사용하고, dead-link 진단에서 신규 파일 오류는 0건이다. repo 전체 문서 빌드는 기존 dead link 625건으로 실패했다. |
| 30 | 실기기·시뮬레이터·WPT·공개 지원·완료 상태를 실행하지 않고 통과했다고 주장하는가? | **통과.** 이번은 계획 PR이며 플랫폼 실행은 구현 단계로 남겼고 WPT·실기기·hardware GPU·compositor-after 픽셀 및 C11.4 완료를 주장하지 않는다. |

문서 빌드에서 이 PR의 새 파일 관련 dead link는 발견되지 않았다. `docs:build` 전체 결과는 저장소의 기존 dead-link 오류 625건으로 exit 1이며, 이를 통과로 보고하지 않는다.
