# C11.4 남은 fit-content/min-content/auto 순서 계획 PR diff 검토 30개

검토 대상은 focused plan, C11.4 순차 후보 갱신, Chrome 157 fixture·capture와 사전 비교 근거다. 계획 자체 검토 20개와 별도로 변경 diff에서 발생할 수 있는 실패 경로 30개를 확인했다.

| # | 실패 관점 | 판정 |
| --- | --- | --- |
| 1 | 계획만 추가했는데 구현 완료 상태로 표현하는가? | 통과 — 구현 전 계획이라고 표시했고 feature 코드 변경은 없다. |
| 2 | C11/C11.4 공식 checkbox나 78/214 count를 변경하는가? | 통과 — 상태 대장은 diff에 없고 계획 문서는 두 상태를 미완료로 명시한다. |
| 3 | 최근 병합된 #1274 범위를 완료된 선행 작업으로 연결하는가? | 통과 — 기존 두 order만 선행 slice로 연결한다. |
| 4 | 기존 두 order를 신규 순서처럼 다시 계획하는가? | 통과 — fit-content가 첫 항목인 기존 두 order와 빠진 네 순서를 분리한다. |
| 5 | 세 track member permutation 중 하나를 누락하는가? | 통과 — six permutations 중 existing two/new four를 대조했다. |
| 6 | 네 신규 order에서 member 중복이나 추가 track을 허용하는가? | 통과 — length 3과 exactly-one-of-each 계약이다. |
| 7 | px cap의 허용 범위가 빠지는가? | 통과 — finite non-negative direct px로 한정한다. |
| 8 | percentage cap의 basis가 indefinite하게 남는가? | 통과 — 100px definite content-box height를 고정한다. |
| 9 | `calc()` 또는 기타 length-percentage 타입이 우회 허용되는가? | 통과 — direct px/%만 허용하고 calc는 fail-closed다. |
| 10 | `grid-auto-rows` pattern이 explicit row부터 시작하는가? | 통과 — explicit 10px와 첫 implicit row를 분리해 표기한다. |
| 11 | 목록이 세 implicit row 뒤에서 순환하는지 확인하지 않는가? | 통과 — 네 번째 implicit row의 used size/frame을 capture한다. |
| 12 | auto track 하나인 두 순서에 free space가 잘못 배정되는가? | 통과 — 20px/6px의 단일-auto 계산과 관측값이 같다. |
| 13 | auto track 두 개인 순서에서 공간을 두 번 나눠 쓰는가? | 통과 — 20px 및 6px를 각각 10px 및 3px씩 나눈다. |
| 14 | fit-content cap 자체와 larger payload frame을 혼동하는가? | 통과 — 20px row와 34px nested payload frame을 별도 보존한다. |
| 15 | payload보다 큰 cap control에서 row가 자연 크기로 남는지 누락하는가? | 통과 — 10px payload와 20px cap 장면을 포함한다. |
| 16 | percentage와 px cap의 동등성만 독립 계산으로 가정하고 Chrome 대조를 빠뜨리는가? | 통과 — 각 신규 order의 20%와 20px scene을 모두 Chrome에서 캡처했다. |
| 17 | authored declaration만 기록하고 used track sizes를 빠뜨리는가? | 통과 — capture JSON에 authored list와 used `grid-template-rows`를 분리 기록한다. |
| 18 | explicit row의 frame을 implicit used rows에 포함하지 않는가? | 통과 — explicit frame과 전체 used rows 모두 저장한다. |
| 19 | Grid item의 stretch 결과를 빠뜨리는가? | 통과 — 각 장면에 item frame 네 개를 모두 저장한다. |
| 20 | child payload의 위치·overflow 결과를 빠뜨리는가? | 통과 — 각 장면에 payload frame 네 개를 모두 저장한다. |
| 21 | 이전 scene section의 page y-offset이 item 좌표에 섞이는가? | 통과 — frame은 각 Grid의 local origin을 뺀 값이다. |
| 22 | border, padding, gap이 percentage basis/free-space 산정에 개입하는가? | 통과 — Grid border-box와 content box 크기가 같고 gap·edge는 0이다. |
| 23 | label 또는 fixture body style이 Grid item tree에 들어가는가? | 통과 — label은 Grid 외부 형제 section에 있다. |
| 24 | outline이 track sizing 크기에 합산되는가? | 통과 — outline은 시각 표시일 뿐 border/padding은 0이다. |
| 25 | source fixture가 선택된 eight scene을 재현하지 못하는가? | 통과 — fixture에는 네 순서의 px/% 각 scene이 고정되어 있다. |
| 26 | screenshot이 viewport 아래 마지막 scenes를 잘라내는가? | 통과 — full-page capture에서 여덟 장면 label과 Grid가 보인다. |
| 27 | screenshot hash가 최종 캡처와 다르게 기록되는가? | 통과 — README의 hash와 저장된 PNG의 SHA-256이 같다. |
| 28 | fixture hash가 capture 대상 source와 다르게 기록되는가? | 통과 — README·JSON의 hash와 fixture 파일의 SHA-256이 같다. |
| 29 | 링크 오타가 계획 PR에서 유실 근거를 만드는가? | 통과 — 수정한 세 Markdown 문서의 로컬 링크 10개를 확인했고 깨진 링크는 없다. |
| 30 | 계획 검토 20개를 PR diff 검토 30개로 재사용하는가? | 통과 — 계획 failure review 20개와 이 diff review 30개를 분리했다. |

검토 중 기능 구현 필요 사항은 발견하지 않았다. PR check와 merge 상태는 계획 PR 자체의 결과로 별도 기록한다.
