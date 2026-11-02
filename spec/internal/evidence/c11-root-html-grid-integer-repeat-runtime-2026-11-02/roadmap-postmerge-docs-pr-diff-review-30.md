# PR #1039 병합 후 C11 상태·로드맵 diff 적대 검토 30개

검토 대상은 [문서 동기화 PR #1039](https://github.com/ohah/spinon/pull/1039)의 STATUS, C11 계획, 내부 계약, runtime/merge evidence와 공개 roadmap 변경이다. 기능 PR #1038의 구현/runtime 검토 및 코드 PR diff 검토와 분리해, 각 항목에서 다른 문서 정합성·오표기 실패를 확인했다.

| # | 공격 관점 | 문서·근거 대조 및 판정 |
|---:|---|---|
| 1 | #1038을 아직 open/draft로 남겨 병합 상태를 숨기는가 | `gh pr view 1038`은 `MERGED`를 반환한다. STATUS·plan·contract·roadmap은 병합 상태로 갱신됐다. 통과. |
| 2 | 병합 시각을 PR event 시각과 다르게 적는가 | GitHub `mergedAt`의 `2026-11-02T06:23:58Z`를 네 문서와 merge record에 대조했다. 통과. |
| 3 | merge SHA 대신 원 branch head를 merge commit으로 적는가 | PR `mergeCommit.oid`와 `origin/main` HEAD는 `b3ad473d4bfef4bb2f54a61603d498e620c03b0a`다. 통과. |
| 4 | squash/merge commit으로 잘못 기록하는가 | merge evidence는 `Rebase and merge`를 명시한다. 통과. |
| 5 | original PR head/base를 merge commit과 혼동하는가 | 원 head `1dbf28b…`, base `51d2bdae…`를 실제 PR JSON과 대조했다. 통과. |
| 6 | rebase merge가 만든 first-parent commit chain을 빠뜨리거나 섞는가 | implementation `2eb52d25…`, pre-merge docs `74ea827…`, review/evidence `b3ad473…` 순서를 `origin/main` log와 대조했다. 통과. |
| 7 | 로컬 main이 아니라 원격 main 반영을 확인했다고 주장하는가 | `git fetch origin main` 뒤 확인한 `origin/main` HEAD가 GitHub merge SHA와 같다. 통과. |
| 8 | checks가 빈 PR을 CI 성공으로 서술하는가 | PR #1038 checks는 0개다. 모든 갱신 문서는 “보고되지 않음”으로 표기한다. 통과. |
| 9 | required check이 비활성인데 branch protection gate 통과라 주장하는가 | required-status-check API의 `Required status checks not enabled` 응답을 기록했고 성공 판정에 쓰지 않는다. 통과. |
| 10 | preview/deploy까지 병합 사실에서 추론하는가 | merge record는 Tailnet preview가 미동기화임을 분리한다. GitHub Pages 배포 주장도 없다. 통과. |
| 11 | 문서 갱신으로 공식 C11 parent checkbox를 체크하는가 | `spec/STATUS.md`의 C11 line은 `- [ ]`로 유지된다. 통과. |
| 12 | 세부 Grid 상태 숫자를 무심코 다시 집계하는가 | 기준 `origin/main:spec/STATUS.md`와 현재 상태의 `- [x]`/`- [ ]` 개수가 같다. 통과. |
| 13 | 공식 aggregate를 72에서 임의로 올리는가 | STATUS, C11 plan 및 public roadmap 모두 `72/213`을 유지한다. 통과. |
| 14 | 제한 slice를 전체 C11.1 완료로 부르는가 | 각 결과 문서가 C11.1 및 C11 completion을 명시적으로 미완료로 남긴다. 통과. |
| 15 | 내부 계약 숫자 버전을 구현 PR 병합과 함께 올리는가 | 계약 `0059`는 `0.1.0`을 유지하고 이를 문서에 확인했다. 통과. |
| 16 | 병합한 profile의 입력 단위를 부정확하게 넓히는가 | contract는 direct finite px, definite-axis percentage, finite fraction integer repeat로 범위를 한정한다. 통과. |
| 17 | auto-repeat 또는 intrinsic sizing도 병합했다고 오해하게 쓰는가 | auto-repeat, named line, `minmax()`/intrinsic 및 기타 out-of-profile 입력을 계속 거부한다고 명시한다. 통과. |
| 18 | 10,000-track cap 또는 1 MiB metadata quota를 빠뜨리는가 | STATUS·contract·전용 plan/runtime evidence가 두 경계를 모두 보존한다. 통과. |
| 19 | allocation 실패/과잉 입력에서 partial scene을 허용한다고 서술하는가 | implementation contract는 contextual failure와 scene publication 부재를 유지한다. 통과. |
| 20 | Chrome 비교 숫자에서 row track을 누락하거나 item 수를 틀리는가 | 보존 observation 기준 7 scenes, 35 total column/row tracks, 28 items를 문서에 대조했다. 통과. |
| 21 | Rust suite 수치를 다른 범위의 전체 crate 성공으로 승격하는가 | 계획과 STATUS는 Root HTML 30/30, style 13/13, style-to-layout 15/15로 한정한다. 통과. |
| 22 | Android AVD를 physical/hardware GPU 결과로 표현하는가 | Android API 37 ARM64 AVD/ANGLE SwiftShader와 visual-smoke 한계를 명시한다. 통과. |
| 23 | iOS Simulator를 iPhone 실기기 또는 compositor pixel parity로 표현하는가 | 문서는 iPhone 17 Pro iOS 26.2 Simulator draw/present만 주장하고 Metal readback은 부인한다. 통과. |
| 24 | 스크린샷을 frame geometry readback으로 오인시키는가 | STATUS, plan, PR body는 screenshots를 visual smoke로 제한한다. 통과. |
| 25 | 병합 후 internal contract의 evidence 경로가 깨지는가 | C11 관련 문서의 새·수정 Markdown 상대 링크 23개를 file existence 기준으로 검사했다. 통과. |
| 26 | C11 master plan의 최신 milestone이 날짜순으로 잘못 위치하는가 | `c11-css-grid.md` 첫 milestone에 #1038 병합 결과를 기록하고 기존 #1030/#1035 결과를 보존한다. 통과. |
| 27 | dedicated implementation plan이 계속 pre-merge candidate라 하는가 | 결과 section을 implementation/merge result로 바꾸고 PR merge metadata 및 evidence 링크를 추가했다. 통과. |
| 28 | STATUS와 internal contract가 서로 다른 merge SHA/state를 표시하는가 | 두 문서 모두 #1038, 같은 merge time/SHA, no checks 및 미완료 경계를 표시한다. 통과. |
| 29 | 공개 roadmap의 새 링크가 화면/evidence/공식 상태를 찾지 못하는가 | PR card의 계획·README·review·merge record·두 PNG·STATUS를 검사해 local target 8개가 존재한다. 통과. |
| 30 | 문서 전용 PR을 기능 재구현으로 오인하거나 PR #1039 상태를 과장하는가 | diff는 8개 문서/evidence file만 바꾼다. PR #1039는 draft·mergeable이며 checks 0개라 CI나 Tailnet 완료를 주장하지 않는다. 통과. |

## 검토 결과

- `git diff --check`: 통과.
- 새/수정된 Markdown 상대 링크 23개와 공개 roadmap 카드의 local target 8개를 확인했다.
- 공식 status checkbox 수, parent C11 미완료 상태 및 내부 계약 버전은 변하지 않았다.
- PR #1038 병합 확인과 PR #1039 변경의 근거는 [merge record](./merge-PR1038.md)에 연결한다.
- 남은 blocking 문서 오류는 없다. Tailnet 동기화는 PR #1039 병합 뒤 별도 수행하고 Agent Browser에서 확인한다.
