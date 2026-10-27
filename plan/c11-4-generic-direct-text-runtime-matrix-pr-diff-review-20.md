# C11.4 direct-text app matrix 계획 PR 변경 검토 20관점

- 대상: [PR #678](https://github.com/ohah/spinon/pull/678)
- 검토 기준: PR base `8b5a3f9a0cbb664b479bf70aa78dc398baa71de8`. 최초 네 파일 검토 시 원격 head는 `b6d58a38a01b485966f583c01995d814f50a29c0`였고, 이후 review report와 계획 내 링크만 추가했다.
- 최종 원격 diff는 계획, 두 review, `spec/STATUS.md`, `spec/public/roadmap.html`의 5개 경로다. STATUS/roadmap substantive content는 최초 검토 이후 바뀌지 않았다.
- 결과: **PASS 20 / PARTIAL 0 / FAIL 0**
- 이 review는 PR 통합·상태 동기화를 대조한다. 새 build/test/browser 실행은 없으며 CI 통과를 주장하지 않는다.

| # | 독립 PR 실패 관점 | 대조 및 판정 |
| --- | --- | --- |
| 1 | 계획 PR이 최신 원격 main을 기준으로 하는가 | PR base SHA는 검증 시 `origin/main` `8b5a3f9a`와 같아 PASS. |
| 2 | PR diff에 의도 밖 구현 파일이 섞였는가 | 원격 diff는 계획/검토 두 문서와 상태·로드맵 두 문서뿐이어서 PASS. |
| 3 | 기존 runtime evidence가 PR 안에 중복 복사됐는가 | 실행 logs/screenshots나 새 generated artifact는 변경 목록에 없어 PASS. |
| 4 | 현재 untracked 실행 자료가 실수로 포함됐는가 | PR 변경 경로와 분리된 기존 #675 run 폴더들은 `git status`에서만 보이고 stage/remote diff에는 없어 PASS. |
| 5 | 공식 C11 checkbox와 수가 바뀌었는가 | C11 `[ ]`와 `72/213` 문자열이 그대로라 PASS. |
| 6 | 상태 항목의 Markdown nesting이 주변과 맞는가 | 새 bullet은 같은 C11.4 subtask 들여쓰기에 들어가며 별도 완료 item을 만들지 않아 PASS. |
| 7 | STATUS 내 계획·검토 링크가 올바른 파일을 가리키는가 | 두 링크 target이 PR 내 신규 파일과 일치해 PASS. |
| 8 | 계획 문서의 계획 검토 보고서 링크가 양방향으로 유효한가 | 계획의 검토 section과 보고서의 대상 경로가 같은 이름을 가리켜 PASS. |
| 9 | 기존 oracle 링크가 새 위치에서 해석되는가 | 계획의 `../spec/internal/evidence/.../README.md`와 Rust `cases.rs` 경로가 원본 저장소에 존재해 PASS. |
| 10 | 공개 roadmap 카드가 C11.4 진행 lane 안에 놓였는가 | generic direct-text runtime 카드 바로 뒤에 계획 카드를 넣어 관련 문맥 안에 PASS. |
| 11 | roadmap markup이 text/attribute를 깨뜨리는가 | 새 `<p>`/`<a>`/`<code>` 요소가 기존 구조 안에서 닫히며 `&` 같은 미escape attribute 문자가 없어 PASS. |
| 12 | 공개 roadmap 링크가 정적 경로 규칙을 따르는가 | 새 plan/report URL은 저장소 plan 경로의 `.html` 변환과 `/spinon/` base path를 사용해 PASS. |
| 13 | STATUS와 roadmap이 계획 상태를 서로 다르게 표현하는가 | 둘 다 5개 추가 runtime scene의 계획, 미시작 구현, `72/213` 유지를 적어 PASS. |
| 14 | PR 본문이 코드/테스트 변경을 암시하는가 | 본문은 계획 PR이며 앱 build/runtime은 후속 gate라고 밝혀 PASS. |
| 15 | 스크린샷이 없는 PR이 화면 검증을 암시하는가 | 본문에 구현 화면이 없어 screenshot 첨부가 없다고 밝혀 PASS. |
| 16 | 실행하지 않은 test나 CI 결과를 보고하는가 | PR body와 문서는 실행 성공을 주장하지 않는다. remote `statusCheckRollup`은 빈 배열이어서 PASS. |
| 17 | 라벨·제목 형식이 프로젝트 정책과 맞는가 | 제목 `plan(c11.4): ...`, labels `planning`/`area: layout`을 확인해 PASS. |
| 18 | merge 방식이 main의 linear-history 정책과 맞는가 | main protection은 linear history를 요구하며 PR merge를 예정해 PASS; merge commit을 계획하지 않는다. |
| 19 | GitHub mergeability 및 review/check 상태가 오해될 수 있는가 | PR은 OPEN·MERGEABLE, reviewDecision 비어 있고 status rollup도 비어 있다. 이를 승인/CI 통과로 표현하지 않아 PASS. |
| 20 | 계획 PR 병합 전 Tailnet preview가 갱신됐다고 잘못 말하는가 | 이번 PR은 로컬 문서 source만 바꾼다. Tailnet은 merge 이후 별도 sync/route/hash/Agent Browser 확인 대상으로 남겨 PASS. |

## 관찰한 제한

main에는 required status check context가 보이지 않고 GitHub PR check rollup도 비어 있다. 이 PR은 코드 실행을 포함하지 않으므로 merge 여부와 관계없이 build/test/CI 통과 근거가 되지 않는다.

## 최종 원격 diff 재확인

review report와 그 링크가 추가된 뒤 PR #678의 diff를 다시 가져와 5개 경로, 현재 base/head, 라벨, mergeability와 빈 check rollup을 확인했다. 이 후속 변경은 review artifact와 plan 내 link만 추가하며 앞서 검토한 STATUS·roadmap 결과는 그대로다. 20개 통합 관점은 최종 diff에서 PASS 상태를 유지했다.
