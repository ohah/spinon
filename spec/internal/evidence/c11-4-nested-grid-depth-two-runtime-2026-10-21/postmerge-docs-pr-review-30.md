# C11.4 depth-two 병합 기록 문서 PR diff 검토 30개

PR #450 병합 뒤 상태 대장, 집중 계획, 상위 C11.4 계획, 공개 로드맵과 실행 근거를 갱신하는 문서 전용 PR을 대상으로 독립 실패 관점 30개를 확인했다. 런타임 구현을 다시 검토하지 않고 병합 기록, 집계, 링크와 근거의 일관성을 확인한다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 문서가 PR 번호를 다른 구현 PR과 혼동하는가 | 상태 대장·계획·로드맵·실행 근거가 모두 PR #450을 가리킨다. |
| 2 | 기록된 merge SHA가 GitHub PR 응답과 다른가 | gh pr view 450의 mergeCommit.oid는 80837bbe03542eb55ebc983964c72fd925454d28이다. |
| 3 | 병합 시각의 날짜 또는 UTC가 잘못 표기되는가 | GitHub 응답의 mergedAt은 2026-10-21T14:38:40Z이며 문서는 같은 UTC 시각을 쓴다. |
| 4 | Rebase and merge를 merge commit 생성으로 오인하는가 | PR은 gh pr merge 450 --rebase로 병합했고 문서는 Rebase and merge로 기록한다. |
| 5 | merge SHA가 현재 원격 main에 포함되지 않았는가 | origin/main HEAD가 merge SHA와 같고 해당 SHA가 원격 main의 조상이다. |
| 6 | 빈 PR check rollup을 성공한 CI로 표시하는가 | PR 응답의 status check rollup은 빈 배열이고 문서 어디에서도 통과로 표현하지 않는다. |
| 7 | required check이 비었다는 설명이 branch 설정과 맞는가 | required status checks API는 Required status checks not enabled를 반환했다. |
| 8 | C11.4 상위 항목이 focused fixture 결과 때문에 완료로 바뀌는가 | spec/STATUS.md의 C11.4 checkbox는 미완료로 유지된다. |
| 9 | 공식 완료 분모가 부정확하게 바뀌는가 | 상태 대장, 계획, 로드맵은 분모 213을 유지한다. |
| 10 | 이전 완료 수를 최신인 것처럼 과장하는가 | 공식 집계는 계속 72/213으로 표시한다. |
| 11 | 상태 기록이 이 PR의 범위를 전체 Grid 지원으로 넓히는가 | depth 3+, arbitrary CSS, WPT와 전체 C11.4 미완료 한계를 함께 유지한다. |
| 12 | 집중 계획 상단이 병합 대기 상태로 남는가 | 계획 상태는 PR #450의 병합 시각과 SHA를 기록한다. |
| 13 | 집중 계획 본문이 아직 병합 결과가 미래라고 말하는가 | 검토·구현 결과 단락은 30개 검토와 완료된 merge SHA를 적는다. |
| 14 | 상위 Grid track-sizing 계획의 후속 상태가 이전 상태와 충돌하는가 | 상위 계획의 PR #450 병합 기록과 72/213 집계가 집중 계획과 일치한다. |
| 15 | 공개 roadmap이 PR #450을 열린 PR처럼 표현하는가 | 공개 callout에 Rebase and merge 시각과 merge SHA를 표시한다. |
| 16 | 공개 roadmap의 화면 링크가 잘못된 실행 폴더를 가리키는가 | Android/iOS 링크는 최종 4/4 simulator run 디렉터리의 PNG를 가리킨다. |
| 17 | Tailnet root route에서 계획 URL이 site root를 벗어나는가 | runtime README의 plan link는 selected RSPress root 기준 세 단계만 올라가도록 고쳤고 roadmap은 /spinon/ 절대 경로를 쓴다. |
| 18 | 실행 README가 PR 본문 이미지 표시를 첨부 업로드로 오인하는가 | README는 merge SHA 고정 GitHub raw URL이라고 설명한다. |
| 19 | PR 본문의 이미지 URL이 삭제될 수 있는 branch 이름을 참조하는가 | 병합 후 PR 본문과 JSON의 두 raw URL은 immutable merge SHA를 사용한다. |
| 20 | PR 본문의 Android image URL이 저장소 artifact와 다른가 | Android URL은 최종 AVD representative nested-depth-two-min-content.png 경로다. |
| 21 | PR 본문의 iOS image URL이 저장소 artifact와 다른가 | iOS URL은 최종 iPhone 17 Pro simulator representative PNG 경로다. |
| 22 | browser 결과가 다른 페이지의 이미지 로드에서 나온 것인가 | Agent Browser는 https://github.com/ohah/spinon/pull/450 페이지 DOM에서 두 image를 확인했다. |
| 23 | 404 또는 빈 image가 complete만으로 성공 처리되는가 | 두 image 모두 complete=true이고 naturalWidth와 naturalHeight가 양수다. |
| 24 | Android 화면 크기 기록이 실제 loaded image와 다른가 | 브라우저가 반환한 Android 원본 크기는 1080×2400이다. |
| 25 | iOS 화면 크기 기록이 실제 loaded image와 다른가 | 브라우저가 반환한 iOS 원본 크기는 1206×2622이다. |
| 26 | JSON artifact에 다른 실행 이미지나 다른 merge ref가 섞이는가 | 두 src는 merge SHA와 README에 명시한 최종 simulator run 경로를 사용한다. |
| 27 | image URL verification을 추가 검증에서 glyph pixel 판정으로 확대하는가 | README와 JSON은 기존 geometry/trace/draw-present 범위 및 glyph 한계를 보존한다. |
| 28 | 문서 변경이 내부 계약 버전 또는 public API 완료를 암시하는가 | 기존 internal contract 0.1.0과 공개 API 범위는 수정하거나 완료 처리하지 않는다. |
| 29 | 사이트 배포가 Tailnet preview 동기화와 혼동되는가 | 이 PR은 문서 출처만 갱신하며 GitHub Pages를 실행하거나 배포하지 않는다. |
| 30 | Tailnet preview가 병합 전에 최신 문서를 받은 것으로 주장되는가 | 이 문서 PR 병합 뒤 안전 복사·hash 검증 및 Agent Browser 확인을 별도 단계로 남긴다. |

## 판정과 문서 빌드 한계

문서 값은 PR #450과 원격 main 상태에 맞고, C11.4 공식 완료 집계는 바뀌지 않는다. Tailnet preview 결과는 이 검토 범위의 완료 근거가 아니며 실제 병합 후 별도 근거로 확인한다.

일반 전체 문서 빌드 `mise exec -- bun run docs:build`는 RSPress dead-link 검사에서 실패했다. 변경하지 않은 기존 evidence README 안의 log, image, JSON 및 directory 링크도 HTML route로 잘못 해석되는 사례가 여러 개 보고됐다. 이 PR에서 추가한 JSON Markdown link 역시 같은 문제를 일으켜 README에서는 plain file name으로 바꿨고, plan 상대 링크도 selected site root 기준으로 수정했다. Tailnet selected-root build와 route 확인은 별도로 진행한다.
