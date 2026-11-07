# C11.4 merged-slice roadmap sync · PR diff 실패 관점 30개

## 검토 범위

- 대상: PR #1272 병합 뒤 plan, internal contract, runtime evidence, public/Tailnet roadmap 및 Agent Browser sync evidence 갱신.
- 구현 source/runtime 코드는 변경하지 않았다. 이 review를 제외한 staged candidate diff SHA-256은 `1363a07b6d0776424a89046946552a7cfeb34d85d11e85d24bd7c114d64432ab`다.

## 실패 관점

1. **잘못된 PR 병합 주장** — GitHub merge tool은 `merged=true`와 merge SHA `5ce4c735777c88542627838bc41411621f397cc3`를 반환했고 PR readback은 `merged=true`다. 통과.
2. **잘못된 merge 방법** — PR #1272 merge request는 프로젝트 규칙에 맞춰 `rebase`를 사용했다. 통과.
3. **시간대 혼동** — roadmap의 merge timestamp는 GitHub `merged_at` UTC `2026-11-07T20:28:56Z`와 일치한다. 통과.
4. **base SHA 불일치** — post-merge sync branch의 base `origin/main`은 PR #1272 merge SHA `5ce4c735...`다. 통과.
5. **공식 상태표 오염** — staged path에 `spec/STATUS.md`가 없고 official completion `78/214`를 유지한다. 통과.
6. **C11.4 완료 과장** — roadmap은 auto member를 병합된 제한 slice로 기록하면서 C11.4 전체를 active/incomplete로 남긴다. 통과.
7. **C11 완료 과장** — C11 상위 항목은 완료로 체크하지 않으며 roadmap 문구에서 미완료를 명시한다. 통과.
8. **공식 grid leaf count 변경** — 세부 count/status ledger를 편집하지 않아 두 unfinished grid leaf 수에 영향을 주지 않는다. 통과.
9. **내부 계약 버전 오염** — internal contract remains `0.1.0`; diff does not change the version. 통과.
10. **bounded slice 범위 확대** — PR #1272 history names only the `auto` member in the existing fit-content list profile. 통과.
11. **미지원 grammar 누락** — runtime evidence preserves rejection boundaries for intrinsic keywords, `minmax()`, `fr`, `calc()`, indefinite percentage basis, and changed topologies. 통과.
12. **현재 작업 카드의 오래된 상태** — Kanban active card now names umbrella `Track sizing`, not the already-merged `auto member` slice. 통과.
13. **카드 label clipping** — pass07 Agent Browser DOM reports the entire SVG title `Track sizing` without ellipsis. 통과.
14. **active ID 누락** — the same card displays `C11.4 · ACTIVE`. 통과.
15. **desktop viewport mislabeled** — pass07 explicitly sets 1280×1200 and records viewport, document, and body widths as 1280. 통과.
16. **mobile viewport mislabeled** — pass07 explicitly sets 390×844 and records viewport, document, and body widths as 390. 통과.
17. **mobile active column hidden** — mobile Kanban scroll position is 272px, aligned to the in-progress column. 통과.
18. **pass06 mistaken as desktop proof** — evidence README identifies inherited 390px pass06 captures as diagnostic only and directs readers to corrected pass07 captures. 통과.
19. **stale Tailnet response** — pass07 source and served SHA-256 match `5972620e626f3cf16b52d5d2335b029187c24b7b80244939036b87d6341d6e3b`. 통과.
20. **backup overwritten or mismatched** — pass07 backup path is distinct and its SHA matches the previous served pass06 content. 통과.
21. **wrong preview server** — Tailscale reports `Running`, expected host matches, and existing preview PID `28040` remains the sole listener. 통과.
22. **unintended server restart** — sync evidence records `serverRestarted=false`; PID is unchanged. 통과.
23. **unintended file deletion** — pass07 sync records zero deletions. 통과.
24. **unreviewed public source drift** — only `/spinon/roadmap.html` was copied to the preview target. 통과.
25. **PR CI portrayed as green** — GitHub returned no statuses or PR workflow runs; docs explicitly say no CI green claim. 통과.
26. **required check omitted from merge decision** — main branch protection reports `required_status_checks=null`; repository rulesets collection is empty. 통과.
27. **review requirement omitted** — main branch protection reports `required_approving_review_count=0`. 통과.
28. **simulator scope exaggerated** — roadmap calls screenshots visual smoke and explicitly excludes surface/pixel parity. 통과.
29. **Agent Browser left open** — session was closed; page count/browser launch are zero after close, background PID exited, and `pgrep -x agent-browser` found none. 통과.
30. **unrelated Chrome state destroyed** — pre-existing Chrome profiles/processes were preserved; no broad Chrome kill was issued. 통과.

## 결과

30개의 서로 다른 문서·roadmap 실패 경로를 대조했고 고쳐야 할 결함을 찾지 않았다. `git diff --cached --check`를 통과했다. PR #1273의 status/merge state는 별도로 확인한다.
