# Grid track-sizing 문서 링크 수정 적대 검토 · 30개 관점

검토일: 2026-10-26
대상 worktree HEAD 및 `origin/main`: `86b9cd9125f01f5d1277db2d2e525a5498688d70`
검토 대상 변경: `plan/c11-4-grid-track-sizing.md`의 한 Markdown 링크 target 수정
결과 기준: 현재 main의 source path, target 존재, 문서 route mapping 및 저장된 route/build evidence. 소스 문서는 수정하지 않았고 build·test·브라우저는 실행하지 않았다.

## 판정

**PASS 29 / PARTIAL 1 / FAIL 0.** 바뀐 링크는 존재하는 dated review source와 그 RSPress route에 맞는다. 이전 unsuffixed target은 main에 없다. 유일한 PARTIAL은 GitHub 조회 시 PR #647이 아직 존재하지 않아 remote PR payload/state를 대조할 수 없다는 검토 범위 한계다. 이는 local one-line link 수정의 target 검증을 막지 않는다.

수정 전 link는 `.../track-sizing-pr-diff-review-30.html`을 가리켰다. 수정 뒤 주소는 `/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/track-sizing-pr-diff-review-30-2026-10-19.html`이며, source는 `spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/track-sizing-pr-diff-review-30-2026-10-19.md`다.

## 적대적 검토 30개

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | link 수정이 의도한 문서 한 곳에만 한정됐는가 | **PASS** — 검토 대상 source diff는 `plan/c11-4-grid-track-sizing.md`의 한 줄, 한 target 수정이다. 이 보고서는 별도 review artifact다. |
| 2 | 이전 target에서 실제 빠진 조각이 날짜 suffix인가 | **PASS** — 전후 diff는 `track-sizing-pr-diff-review-30.html` 뒤에 `-2026-10-19`만 복원한다. |
| 3 | 수정된 링크가 Markdown link 문법으로 완결되는가 | **PASS** — label과 괄호 안 URL이 닫히며 인접한 링크 구분자와 분리돼 있다 (`plan/c11-4-grid-track-sizing.md:90`). |
| 4 | link label이 실제 목적을 잘못 설명하는가 | **PASS** — label은 “track sizing PR diff 적대 검토 30개”; target title도 “C11.4 track sizing PR 변경 적대 검토 30개”다. |
| 5 | 수정 전 unsuffixed source가 우연히 존재해 혼동을 만드는가 | **PASS** — `origin/main`의 대상 evidence directory에는 suffix 없는 `track-sizing-pr-diff-review-30.md`가 없다. |
| 6 | 수정된 정확한 Markdown source가 current main에 있는가 | **PASS** — `git cat-file`과 `git ls-tree origin/main`에서 dated `.md` source를 확인했다. |
| 7 | source 존재 확인이 오래된 별도 branch 관찰에만 기대는가 | **PASS** — 검토 HEAD가 `origin/main`과 동일 SHA (`86b9cd9…`)여서 current main tree를 직접 기준으로 삼았다. |
| 8 | target이 잘못된 `plan/` 디렉터리를 가리키는가 | **PASS** — source는 계획 파일이 아니라 `spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/`에 있다. 수정 URL도 같은 evidence route를 쓴다. |
| 9 | 대상 evidence directory 이름에 날짜 suffix가 빠졌는가 | **PASS** — link와 source directory 모두 `c11-4-grid-track-sizing-precomparison-2026-10-19`를 쓴다. |
| 10 | Markdown source filename의 날짜와 route suffix가 일치하는가 | **PASS** — source 및 rendered route 양쪽에 `track-sizing-pr-diff-review-30-2026-10-19`가 동일하게 들어간다. |
| 11 | 대소문자 차이로 배포 filesystem에서 깨지는가 | **PASS** — `track-sizing`, `pr-diff-review`, 날짜 suffix의 case가 tracked source와 동일하다. |
| 12 | 확장자가 source/route 역할에 맞는가 | **PASS** — repo source는 `.md`, 문서 링크와 generated route는 `.html`이다. |
| 13 | 절대 route의 `/spinon` base prefix가 누락됐는가 | **PASS** — 현재 링크는 `/spinon/` prefix를 사용한다. 인접한 고정 자료와 evidence link도 같은 prefix 규칙을 쓴다. |
| 14 | site base path가 실제 `/spinon/`과 다른가 | **PASS** — `packages/docs/rspress.config.ts`는 `SPINON_DOC_BASE` override가 없을 때 `base: '/spinon/'`을 설정한다. |
| 15 | RSPress document root에서 evidence source가 제외되는가 | **PASS** — config의 `root`는 repository `spec/`; target source가 그 하위 `spec/internal/evidence/`에 있다. |
| 16 | `.md` source가 `.html` page로 생성되지 않는 route인가 | **PASS** — 저장된 `render-build.log`는 `internal/evidence/.../track-sizing-pr-diff-review-30-2026-10-19.html` output을 기록한다. |
| 17 | generated output 경로의 상위 directory가 달라지는가 | **PASS** — recorded build output path는 `/spinon/internal/evidence/` link path의 base를 제거한 동일 `internal/evidence/...` tree다. |
| 18 | 실제 route에 해당 review가 아닌 다른 페이지가 노출되는가 | **PASS** — 저장된 `browser-verification.json`의 path가 link와 일치하고 title은 “C11.4 track sizing PR 변경 적대 검토 30개 - 스피논 문서”다. |
| 19 | route가 빈/부분 페이지여서 목적을 충족하지 못하는가 | **PASS** — saved route verification은 `reviewTableRows: 30` 및 `incompleteConclusionPresent: true`를 기록한다. |
| 20 | target의 내용이 다른 PR/다른 계획의 review인가 | **PASS** — source header와 표는 C11.4 track sizing PR 변경의 30개 실패 관점 review를 다룬다. |
| 21 | 날짜 suffix가 문서 내용의 검토 날짜와 맞지 않는가 | **PASS** — source header는 검토일 2026-10-19이며 directory와 filename도 같은 날짜다. |
| 22 | URL에 anchor fragment 오타가 있거나 필요한 fragment가 빠졌는가 | **PASS** — 이 링크는 section anchor 없이 문서 전체 review를 가리킨다. target은 상단 title부터 결과 표와 결론을 가진다. |
| 23 | `/spinon` absolute path가 현재 문서 위치에 따라 다른 파일로 해석되는가 | **PASS** — leading slash는 문서 상대경로로 해석되지 않아 현재 `plan/` 위치에 의존하지 않는다. |
| 24 | base prefix와 target path 결합에서 slash가 누락되거나 중복되는가 | **PASS** — href의 `/spinon/` 뒤에 `internal/evidence/...`가 이어지며 saved rendered route에도 정확히 한 경계 slash가 있다. |
| 25 | target 파일이 symlink나 untracked-only payload라 main에서 사라지는가 | **PASS** — target source는 `origin/main` tree에 tracked entry로 존재한다. |
| 26 | 여러 유사 review 파일 중 다른 dated version에 연결되는가 | **PASS** — evidence source directory의 실제 matching file 하나가 날짜 suffix filename이며 link가 그 exact file을 선택한다. |
| 27 | 현재 main에 남아 있는 같은 link가 새 target과 모순되는가 | **PASS** — `origin/main` 계획은 수정 전 unsuffixed href를 갖지만 dated target source와 route는 이미 main에 존재한다. 이 PR의 변경은 그 내부 reference 하나를 복구한다. |
| 28 | 변경이 review label, 내용, C11.4 상태 또는 별도 API claim까지 바꾸는가 | **PASS** — diff는 URL target만 바꾸며 label, 본문, checkbox 및 공식 완료 수를 수정하지 않는다. |
| 29 | 요청한 PR #647의 원격 payload까지 검토됐다고 오인하는가 | **PARTIAL** — GitHub GraphQL 및 REST에서 PR #647은 404/Not Found였다. 이 결과는 local worktree diff와 current-main target 검토이며 remote PR 상태·review payload를 입증하지 않는다. |
| 30 | 링크 수정이 build/browser 재실행으로 검증됐다고 오인하는가 | **PASS** — 이 리뷰는 실행하지 않았다. route 판단은 repo config, tracked source와 저장된 2026-10-19 build/browser verification evidence를 대조했다. |

## 결과와 한계

- Current `origin/main` source target은 `spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/track-sizing-pr-diff-review-30-2026-10-19.md`다. 이전 unsuffixed target은 없다.
- RSPress config는 `spec/`를 문서 root, `/spinon/`을 기본 base로 둔다. 저장된 build log는 dated `.html` output을, 저장된 browser verification은 동일 route, title, 30개 표 행 및 결론 존재를 기록한다. 이는 현 live deployment의 재검증은 아니다.
- GitHub에서 PR #647은 조회되지 않았다. 본 보고서는 해당 번호의 원격 PR state를 주장하지 않는다.
- 테스트·빌드·브라우저를 새로 실행하지 않았고, source 문서는 변경하지 않았다. 보고서만 `spec/internal/evidence/c11-4-track-review-link-fix-2026-10-26/pr-diff-review-30.md`에 기록했다.
