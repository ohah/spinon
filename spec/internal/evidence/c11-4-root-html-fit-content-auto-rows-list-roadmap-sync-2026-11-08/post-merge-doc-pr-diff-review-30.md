# C11.4 fit-content 목록 · 병합 후 문서 PR diff 실패 관점 검토 30개

**검토 기준:** 구현 PR #1270이 merge된 `de34c3c8f2b85826c61b6224f260f380ea972085` 기준으로 수정한 공식 상태·계획·내부 계약·공개 roadmap과 Tailnet sync evidence. 기능 PR의 코드 검토와 별도로, 문서/제공본이 stale·과장·상충하지 않는 실패 경로를 확인했다.

| # | 공격 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 문서가 이전 PR이나 예상 merge SHA를 사용한다. | PR #1270 GitHub metadata, GitHub `main` HEAD, `spec/STATUS.md`, 내부 계약과 공개 roadmap이 `de34c3c8f2b85826c61b6224f260f380ea972085`로 일치한다. **차단 없음.** |
| 2 | merge 시간이 날짜대 변환 중 잘못 기록된다. | GitHub `merged_at` UTC `2026-11-07T18:38:06Z`가 STATUS·계획·roadmap에 같다. KST 표기를 혼용하지 않는다. **차단 없음.** |
| 3 | PR 기록이 rebase merge가 아닌 merge commit을 주장한다. | PR #1270은 `Rebase and merge`로 닫혔고, main에 반영된 rebase commit SHA를 기록했다. **차단 없음.** |
| 4 | 구현 PR 링크가 다른 PR을 가리킨다. | STATUS, plan, contract, roadmap link는 모두 https://github.com/ohah/spinon/pull/1270 을 가리킨다. **차단 없음.** |
| 5 | 제한 slice 병합을 공식 C11.4 완료로 올린다. | `spec/STATUS.md`의 C11.4 checkbox는 미완료이며 merge 기록은 하위 제한 구현으로 표현한다. **차단 없음.** |
| 6 | 전체 C11 구현 또는 공개 CSS 완료로 표현한다. | STATUS, contract, plans, roadmap과 sync README가 C11/C11.4 미완료 및 지원 범위 밖을 남긴다. **차단 없음.** |
| 7 | 공식 완료 aggregate가 78/214에서 바뀐다. | 상태 문서·roadmap·Tailnet·desktop/mobile DOM에서 `78/214`가 유지되며 완료 checkbox는 수정하지 않았다. **차단 없음.** |
| 8 | 내부 계약 숫자 버전이 구현 slice마다 올라간다. | 내부 계약 `0059`의 숫자 버전 `0.1.0`을 그대로 뒀다. **차단 없음.** |
| 9 | C11.3.3이 새 roadmap 설명에서 완료 처리된다. | 새 진행 설명은 C11.4 slice만 기록하며 C11.3.3 완료 상태를 추가하지 않는다. **차단 없음.** |
| 10 | C11.4가 이제 끝났는데 진행 카드가 활성 상태로 잘못 남는다. | track-size list PR은 제한 slice다. 로드맵은 `C11.4 · ACTIVE`를 유지하고 공식 parent checkbox도 미완료다. **차단 없음.** |
| 11 | plan의 구현 상태가 PR merge 전 문구로 남는다. | 집중 계획은 PR #1270 merge SHA와 empty check rollup을 기록하고, C11.4 전체 미완료를 유지한다. **차단 없음.** |
| 12 | 상위 track-sizing plan이 후보 PR 대기 상태로 남는다. | 상위 계획의 C11.4 slice 요약을 병합 기록과 현재 제한 범위로 갱신했다. **차단 없음.** |
| 13 | 계획 링크가 이전 percentage slice나 경로가 바뀐 파일로 연결된다. | STATUS와 public roadmap의 계획 링크가 `c11-4-root-html-fit-content-auto-rows-list.md`를 가리킨다. **차단 없음.** |
| 14 | 구현 전 Chromium 비교 기록이 새 목록 fixture와 연결되지 않는다. | plan/STATUS/roadmap은 2026-11-08 list precomparison README에 연결되며 고정 fixture와 expected values가 그 폴더에 있다. **차단 없음.** |
| 15 | 실행 근거 링크가 다른 Simulator run 또는 이전 PR을 가리킨다. | 신규 링크는 C11.4 auto-row list runtime evidence 디렉터리를 사용하고 그 안의 manifest가 AVD·iOS screenshot과 run hash를 식별한다. **차단 없음.** |
| 16 | focused test 횟수가 현재 최종 test log와 어긋난다. | 문서에는 Layout 8/8, style projection 3/3, RuntimeSession 6/6을 썼고 committed logs의 결과와 같다. **차단 없음.** |
| 17 | 넓은 탐색 필터 실패를 감추고 전체 crate가 green이라고 한다. | STATUS와 roadmap은 C11.5 fixed-track auto-size/stretch 경로의 다섯 실패를 별도 exploratory limitation으로 기록하며 focused pass와 합산하지 않는다. **차단 없음.** |
| 18 | Android AVD를 Android 실기기로 표현한다. | 문서와 roadmap은 API 37 AVD라고 적고 hardware/physical-device claim을 하지 않는다. **차단 없음.** |
| 19 | iOS Simulator screenshot을 compositor/pixel parity 증거로 해석한다. | docs는 visual smoke만 주장하고 surface readback/pixel parity를 부인한다. **차단 없음.** |
| 20 | WPT 또는 임의 CSS author admission을 지원 완료로 표시한다. | merge 기록과 roadmap 모두 WPT/general CSS author admission을 scope 밖으로 분리한다. **차단 없음.** |
| 21 | PR merge를 CI 통과로 확대 해석한다. | GitHub PR check/status 및 workflow run 목록은 비어 있다. 문서는 이를 기록하되 CI success라고 부르지 않는다. **차단 없음.** |
| 22 | repository main에 required CI나 approval이 존재하는데 문서가 없다고 한다. | GitHub rulesets 응답은 빈 배열이고 branch protection의 required status check 항목은 없으며 approving review count는 0이다. **차단 없음.** |
| 23 | README의 source hash가 public roadmap source 내용과 다르다. | `spec/public/roadmap.html`은 628,254 bytes이며 SHA-256 `307632e0d8de1ccfc7441291e9625ff9ba79a7eb8cb48e11655d74f55d3f359f`다. **차단 없음.** |
| 24 | Tailnet target이 source와 달라 local 성공만 보고한다. | 로컬 preview target, loopback HTTP와 Tailnet HTTPS가 모두 628,254 bytes와 동일 SHA-256, HTTP 200을 반환한다. **차단 없음.** |
| 25 | 기존 preview를 backup 없이 덮거나 backup을 새 파일로 오인한다. | 기존 628,186-byte preview를 별도 pass01 backup으로 복사하고 이전 제공본 SHA `505a4d...`를 source와 비교해 보존했다. **차단 없음.** |
| 26 | Tailnet preview server를 불필요하게 재시작하거나 Pages 배포를 실행한다. | Preflight와 copy manifest는 PID 28040, `serverRestarted=false`, Pages `not requested; unchanged`를 기록한다. **차단 없음.** |
| 27 | Active roadmap card title이 잘리거나 이전 `fit-content %` 상태를 보여준다. | Desktop/mobile Agent Browser DOM은 새 카드 제목 `fit-content 목록`과 C11.4 active state를 표시한다. **차단 없음.** |
| 28 | 모바일 페이지가 viewport보다 넓어 수평 overflow를 만든다. | Desktop 1440px에서 document/body 폭은 1440px, mobile 390px에서 두 폭은 390px다. 보드 내부의 가로 진행은 viewport body overflow와 구분된다. **차단 없음.** |
| 29 | 첨부한 Tailnet screenshot이 다른 viewport나 다른 파일 hash를 가리킨다. | Desktop PNG는 1440×1000, mobile PNG는 390×844이고 각 JSON의 SHA-256 및 viewport 값과 일치한다. **차단 없음.** |
| 30 | browser 종료 뒤 agent-browser daemon이 남거나 사용자 preview server가 종료된다. | Agent Browser session 종료 뒤 `pgrep`에서 browser process가 사라졌고 lifecycle JSON은 active=false/pid=null을 기록한다. Preview server PID 28040은 유지됐다. **차단 없음.** |

## 판정

문서 diff와 Tailnet 제공본에서 구현 완료 범위를 넓히거나 C11.4/C11 상태를 왜곡하는 문제는 찾지 못했다. Tailnet source/served hashes, 기존 backup, desktop/mobile DOM 및 Agent Browser 종료 상태를 각각 독립적으로 확인했다. 공식 완료 aggregate는 `78/214`로 유지한다.
