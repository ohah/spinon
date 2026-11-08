# C11.4 PR #1284 roadmap sync PR diff 실패 관점 검토 30개

검토 대상은 공식 상태, Grid 계획·내부 계약, 공개 roadmap, 다음 slice 계획·사전 비교, Tailnet 선택 게시 도구와 증거다. 마지막 검토에서 staged source hash, 게시 route hash, 화면 캡처와 lifecycle 결과를 각각 대조했다.

| # | 독립 실패 관점 | 판정과 근거 |
| --- | --- | --- |
| 1 | PR #1284의 merge SHA나 시각이 잘못 기록되는 경우 | 통과. 공식 상태, 계획, roadmap에서 `f2f9312e39ddf6d7ca8e703c28d33bf26e5325b8`과 merge 시각을 대조했다. |
| 2 | 비어 있는 GitHub check를 CI green이라고 쓰는 경우 | 통과. workflow/status checks 미보고를 그대로 유지한다. |
| 3 | PR #1284 limited profile을 일반 CSS 지원으로 확대하는 경우 | 통과. exact list, topology, cap와 row 범위를 제한적으로 적는다. |
| 4 | 최신 #1284를 이전 #1280의 24순서 구현과 혼동하는 경우 | 통과. 병합 내역과 후속 candidate를 별도 단락·카드로 둔다. |
| 5 | 끝난 24-order work를 진행 칸에 남기는 경우 | 통과. `Grid 24순서 구현`을 제거하고 미착수 다음 candidate를 `할 일` 열에 표시한다. |
| 6 | 다음 구현 candidate가 공식 완료로 집계되는 경우 | 통과. C11.4/C11 checkbox와 `78/214`는 유지한다. |
| 7 | 아직 선택되지 않은 작업을 진행 중이라고 게시하는 경우 | 통과. 코드 구현 전임을 밝히고 active detail list는 비운다. |
| 8 | next candidate의 목록 multiplicity가 CSS track list와 맞지 않는 경우 | 통과. `min-content×2`, `max-content×2`, `auto×1`, `fit-content×1`을 문서·생성기·기대값에서 대조했다. |
| 9 | 30개 unique tail order를 잘못 계산하는 경우 | 통과. 생성기가 중복 제거된 다중집합 순열 30개를 assert한다. |
| 10 | current #1284 순서가 candidate에서 빠지는 경우 | 통과. 기존 `min max auto min max` suffix는 candidate의 한 순서다. |
| 11 | browser observation을 독립 기대값으로 재사용하는 경우 | 통과. expected model은 고정 px payload와 Grid rule로 별도 생성한다. |
| 12 | 120 scene을 전체 CSS Grid conformance로 오해하는 경우 | 통과. Chrome fixed-fixture precomparison으로만 부른다. |
| 13 | 9,480 checks 산정에서 frame 종류를 빠뜨리는 경우 | 통과. track rows, item/payload rects, container geometry 및 overflow 계산을 검증기에서 세었다. |
| 14 | fixed-px payload를 일반 min/max content sizing 검증으로 확대하는 경우 | 통과. 한계와 arbitrary contribution 미포함을 계획/README에 적는다. |
| 15 | `20%` cap의 percentage basis를 모호하게 두는 경우 | 통과. definite 100px block-size basis를 고정하고 px cap과 동일한 20px로 비교한다. |
| 16 | stretch에서 두 auto occurrence를 하나로 계산하는 경우 | 통과. auto가 tail 첫 member일 때 row 2와 row 8 두 occurrence를 계산한다. |
| 17 | mobile이 active column에 고정돼 next card를 숨기는 경우 | 통과. active slice가 없을 때 first `할 일` column을 초기 노출한다. |
| 18 | desktop에서 칸반 heading과 card가 서로 다른 단계를 주장하는 경우 | 통과. heading·next card는 candidate/계획 완료, active column은 실제 구현 중 항목만 뜻한다. |
| 19 | 공식 status API와 detail board note의 차이를 숨기는 경우 | 통과. 공식 집계와 C11.4 detail lane이 서로 다른 지표임을 설명한다. |
| 20 | new plan route가 Tailnet build에서 누락되는 경우 | 통과. stage manifest의 34개 갱신 경로와 117개 선택 root 파일이 현재 source hash와 일치하며, 계획 HTML route도 두 base에서 HTTP 200/hash 일치다. |
| 21 | relative links가 `spec/internal` mirror를 요구하면서 route가 없는 경우 | 통과. Agent Browser에서 plan과 precomparison README를 열었고 두 route 및 `spec/internal` mirror route가 제공됐다. |
| 22 | precomparison PNG 또는 fixture route의 MIME이 잘못되는 경우 | 통과. PNG는 Agent Browser에서 1440×2196으로 로드됐고 fixture HTML·image MIME 및 두 base의 응답을 확인했다. |
| 23 | roadmap이 오래된 24-order title을 script data에 남기는 경우 | stage assertion은 stale marker 부재와 next/merge/current-count marker를 함께 확인한다. |
| 24 | no-active 상태에서 WIP count가 허위 active를 표시하는 경우 | `detailInProgress=[]`이며 board가 0 active를 표시한다. |
| 25 | PR #1284 simulator screenshots를 surface pixel readback으로 표현하는 경우 | history card는 OS screenshot visual smoke와 readback 한계를 유지한다. |
| 26 | Tailnet sync 중 기존 파일을 backup 없이 비원자적으로 교체하는 경우 | preflight SHA와 allowlist를 확인하고 backup 후 same-directory atomic rename을 사용한다. |
| 27 | preview route 교체 때 기존 page reader나 server process를 손상하는 경우 | server를 재시작하지 않고 열린 file handles와 PID를 기록한다. |
| 28 | staging/publication이 unrelated untracked files를 포함하거나 삭제하는 경우 | 이전 selected root 복사와 명시된 updated routes만 사용하며 `noDeletes=true`다. |
| 29 | local preview와 Tailnet 응답 내용이 다르게 게시되는 경우 | 통과. 두 base에서 각각 16개 page/image route와 18개 generated asset, 총 68개 HTTP·SHA-256 검사가 모두 일치했다. |
| 30 | browser process를 남기거나 tmux/preview server까지 닫는 경우 | 통과. Agent Browser session 종료 후 Chrome/Agent Browser process가 0개다. Tailscale `Running`, preview PID 28040, tmux `spinon` 세션은 유지했다. |

## 결론

이 문서는 계획·화면 20-review와 별개의 PR diff review다. 30개 항목을 source diff, selected-root manifest, preflight·backup·publish 결과, 68개 served hash, Agent Browser desktop/mobile 화면과 process lifecycle에 대조했다. 차단 결함은 없고 모든 검토 항목을 통과로 판정한다. 공식 CSS Grid 완료를 뜻하지 않는다.
