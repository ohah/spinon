# C19 계획 PR 변경 적대 검토

계획 문서 검토와 겹치지 않도록 이 PR의 파일·상태·배포 효과를 20개의 별도 관점에서 대조했다.

| # | 변경 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 분리 작업 중 원본 worktree의 사용자 수정이 PR에 섞이는가 | 최신 `origin/main`에서 만든 전용 plan worktree만 변경했다. | 통과 |
| 2 | C19 계획 파일명과 상태 대장 링크가 달라지는가 | 두 파일의 상대 경로와 slug가 일치한다. | 통과 |
| 3 | STATUS의 계획 review 경로에 날짜·파일명 오타가 있는가 | review 파일 두 개가 같은 evidence directory에 존재한다. | 통과 |
| 4 | internal README의 새 계획 링크가 잘못된 기준 디렉터리를 쓰는가 | `../../plan/` 경로가 저장소 root의 계획 파일로 해석된다. | 통과 |
| 5 | internal README의 review 링크가 evidence root를 벗어나는가 | 두 `./evidence/` 경로가 실제 파일로 해석된다. | 통과 |
| 6 | C19 plan의 STATUS anchor가 잘못된 section을 가리키는가 | link는 현재 `spec/STATUS.md` 안의 CSS 구현 체크리스트 section에 둔다. | 통과 |
| 7 | C19 substage 설명이 완료 checkbox로 노출되는가 | 상위 C19 표시는 계속 `[ ]`다. | 통과 |
| 8 | 이전 C18.4 계획 전 상태가 병합 뒤에도 현재처럼 보이는가 | stale 문장을 제거하고 merged #227과 official status를 연결했다. | 통과 |
| 9 | C18.4 설명이 새 plan 문서에서 다시 다르게 복제되는가 | 단일 공식 상태 대장 링크를 기준으로 남은 범위를 확인하게 했다. | 통과 |
| 10 | 외부 링크가 임의 블로그/2차 해설로 바뀌는가 | plan은 W3C 규범 문서 URL만 직접 가리킨다. | 통과 |
| 11 | 새 숫자 계약 ID가 아직 구현되지 않은 계획으로 예약되는가 | 어떤 번호 있는 내부 계약 파일도 추가하지 않았다. | 통과 |
| 12 | STATUS 변경이 C19를 완료로 보이게 하거나 다른 기능 상태를 바꾸는가 | C19 checkbox는 그대로 미완료이며, 해당 요약에 계획 링크만 추가했다. | 통과 |
| 13 | Rspress dependency나 lockfile이 불필요하게 변하는가 | docs package, package manifest, bun.lock은 diff에 없다. | 통과 |
| 14 | generated HTML 또는 screenshot이 검증을 수행하지 않고 결과처럼 커밋되는가 | generated page와 product screenshot을 추가하지 않았다. | 통과 |
| 15 | public roadmap HTML을 hand-edit해 source/status가 갈라지는가 | 공개 generated HTML은 변경하지 않고 공식 Markdown source만 고쳤다. | 통과 |
| 16 | 변경 범위 밖의 CSS feature status가 영향을 받는가 | source 문서의 변경은 C19 계획 링크와 C18.4 stale index 갱신뿐이며 그 외 feature state는 건드리지 않았다. | 통과 |
| 17 | 계획 review와 PR review가 같은 20문장을 재사용하는가 | 두 review 파일의 관점은 계획 요건과 PR 경로/상태/패키징으로 분리했다. | 통과 |
| 18 | plan-only PR의 설명이 실행·빌드 통과를 암시하는가 | 변경 review는 runtime result가 없음을 명시한다. | 통과 |
| 19 | 문서 PR이 승인 없이 Pages deploy나 서버 재시작을 유발하는가 | workflow/deploy 설정과 server 파일은 바뀌지 않았다. | 통과 |
| 20 | commit staging에 예상 밖 파일이 들어가는가 | 변경 허용 목록은 plan, STATUS, internal index, 두 review 파일뿐이다. | 통과 |

## 변경 경계

이 PR은 계획·계획 검토·공식 index 참조만 갱신한다. 구현 코드, C19 완료 check, versioned runtime contract, simulator result, Tailnet 서버 파일은 변경하지 않는다.
