# C11.4 two-track span · 병합 후 상태·로드맵 동기화 검토 30개

| # | 독립 실패 경로 | 판정 |
|---:|---|---|
| 1 | 계획 PR #755의 상태를 구현 PR의 병합 proof로 혼동 | 통과 — #755는 계획, #756은 test-only 구현으로 구분했다. |
| 2 | #756이 실제 병합되지 않았는데 병합으로 기록 | 통과 — GitHub PR 상태는 `MERGED`다. |
| 3 | 다른 PR의 merge timestamp를 사용 | 통과 — #756 `mergedAt`은 `2026-10-28T11:33:07Z`다. |
| 4 | 리베이스 전 head SHA를 main merge SHA로 기록 | 통과 — 문서에는 `ca7f6e93842a72f6e410588832a9a431820dffcc`를 쓴다. |
| 5 | Rebase and merge 방식을 생략하거나 squash로 잘못 표기 | 통과 — PR 기록과 같은 방식을 적었다. |
| 6 | 서로 다른 계획·구현 PR의 순서를 바꿈 | 통과 — #755 계획 후 #756 구현으로 표기했다. |
| 7 | 다섯 overflow 입력을 10개나 14개 case와 합산 | 통과 — 이번 module 범위는 다섯 개다. |
| 8 | visible/clip 결과의 track 순서를 바꿈 | 통과 — 두 track은 `[50,50]px`다. |
| 9 | hidden/auto/scroll 결과를 content-based minimum으로 표현 | 통과 — 해당 경우는 zero automatic minimum 뒤 `[40,40]px`다. |
| 10 | item frame을 track 합계와 맞지 않게 기록 | 통과 — 비-scrollable는 100×20px, scrollable은 80×20px다. |
| 11 | scrollable case의 MaxContent contribution을 0으로 기록 | 통과 — max-content 100px contribution은 유지된다. |
| 12 | trace source와 contribution query 검증을 누락 | 통과 — 구현 evidence에 Minimum source/value와 MaxContent query를 연결했다. |
| 13 | 공유 `1/64px` 허용 오차를 exact 비교로 표현 | 통과 — 신규 helper는 `assert_eq!`를 사용한다. |
| 14 | focused test 수를 package suite 수로 오인 | 통과 — focused 1/1과 crate 298/298을 분리했다. |
| 15 | test suite 결과를 실행하지 않은 뒤 문서에 기록 | 통과 — merge 전 최종 source에서 기록된 실제 gate를 옮겼다. |
| 16 | package test count를 297/297로 오래된 상태 유지 | 통과 — #756 이후 298/298로 갱신했다. |
| 17 | fmt check와 Clippy 중 하나를 빠뜨림 | 통과 — 둘 다 별도 기재했다. |
| 18 | crate all-target Clippy를 workspace Clippy라고 확대 | 통과 — `spinon-layout` crate-target 범위로 썼다. |
| 19 | 빈 GitHub check rollup을 CI 성공으로 승격 | 통과 — PR check rollup이 비어 있어 통과 주장을 하지 않는다. |
| 20 | pure Rust unit-test를 Android/iOS runtime 검증으로 표현 | 통과 — 앱 runtime 및 Simulator 미실행을 명시했다. |
| 21 | Chrome screenshot을 Spinon 결과로 제시 | 통과 — 독립 browser baseline이라고 표기했다. |
| 22 | CSS overflow parser/admission 완료를 암시 | 통과 — 문서는 test-only slice 범위만 설명한다. |
| 23 | screenshot의 폭 넘침을 clipping pixel 검증으로 표현 | 통과 — 이번 gate는 frame/track 계산만 다룬다. |
| 24 | C11.4 상위 checkbox를 완료로 바꿈 | 통과 — `spec/STATUS.md` checkbox는 미완료다. |
| 25 | 공식 완료 수를 72/213보다 높임 | 통과 — 상태 대장, 계획, 공개 roadmap에 72/213이 유지된다. |
| 26 | 구현 결과가 plan에 pending으로 남음 | 통과 — 상세 계획 section을 #756 병합 결과로 교체했다. |
| 27 | 부모 계획의 최신 병합 목록에서 #755/#756 누락 | 통과 — 최신 PR 목록을 두 건 추가했다. |
| 28 | 공개 roadmap paragraph가 계속 다음 계획이라고 표시 | 통과 — 병합된 구현 slice와 한계를 표시한다. |
| 29 | 새 내부 evidence 링크를 public roadmap에서 잘못된 base로 연결 | 통과 — `/spinon/internal/evidence/...` 및 `/spinon/plan/...` 경로를 사용한다. |
| 30 | 문서 변경만으로 전체 RSPress build 성공을 주장 | 통과 — 이번 검토는 문서 내용과 링크 경로를 대조하며 full build 통과를 주장하지 않는다. |

이 검토는 PR #756 병합 후 계획, 상태 대장 및 공개 roadmap의 사실 동기화만 다룬다. C11.4 전체 완료나 공식 집계 승격을 승인하지 않는다.
