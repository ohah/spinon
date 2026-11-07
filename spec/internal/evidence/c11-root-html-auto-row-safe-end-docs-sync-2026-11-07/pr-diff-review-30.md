# `#1249` 상태·로드맵 및 다음 후보 문서 PR diff 실패 관점 검토 30개

검토 범위는 [상태 대장](../../../STATUS.md), [내부 계약 0059](../../0059-c11-grid-layout.md), 공개 roadmap, [선행 safe-center 계획](../../../../plan/c11-root-html-auto-row-safe-center.md), C11.4 master plan, [safe-end 신규 계획](../../../../plan/c11-root-html-auto-row-safe-end.md), [Chrome 사전 비교](../c11-root-html-auto-row-safe-end-precomparison-2026-11-07/README.md), 계획 검토다. 변경 전 완료 여부·PR metadata·검증 한계와 대조했다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | C11 상위 항목이 제한 PR 때문에 완료 체크되는가? | 통과 — C11 상위 체크는 미완료로 남았다. |
| 2 | C11.3 또는 C11.4 하위 gate가 잘못 완료되는가? | 통과 — 둘 다 미완료로 유지했다. |
| 3 | `78/214`가 C11 자체 완료율로 설명되는가? | 통과 — 전체 로드맵 counter로 표기하고 C11별 퍼센트를 만들지 않았다. |
| 4 | PR 번호·제목이 실제 merged PR과 일치하는가? | 통과 — GitHub PR #1249 메타데이터와 대조했다. |
| 5 | Rebase-and-merge 시각이 정확한가? | 통과 — `2026-11-07T02:49:40Z`를 원 PR과 대조했다. |
| 6 | merge SHA가 기능 branch SHA와 혼동되는가? | 통과 — merge SHA `a69030722d82a97e6ce40e276c0aae2b8447550d`를 명시했다. |
| 7 | C11.4 plan이 구현 전 후보로 남는가? | 통과 — `#1249` 병합 상태로 바꿨다. |
| 8 | 내부 계약 제목이 아직 candidate라고 말하는가? | 통과 — title/body를 bounded merged implementation으로 갱신했다. |
| 9 | 내부 계약 숫자 버전이 무단 상승하는가? | 통과 — `0.1.0`을 유지한다. |
| 10 | safe/unsafe 일반 지원 선언을 하는가? | 통과 — fixed RootHtml profile의 두 computed 값만 기록했다. |
| 11 | positive-space 좌표가 기존 Chrome/독립값과 다른가? | 통과 — `[5,15,25]` center와 `#1249` 6-scene evidence를 확인했다. |
| 12 | overflow subject에서 gutter가 빠졌는가? | 통과 — 세 track과 두 gutter를 포함하는 122px subject를 유지한다. |
| 13 | safe/unsafe center overflow 좌표가 뒤바뀌는가? | 통과 — safe `[0,56,112]`, unsafe `[-41,15,71]`을 검토했다. |
| 14 | test 결과를 PR body와 다르게 적었는가? | 통과 — focused layout/projection/RuntimeSession 및 regression counts를 대조했다. |
| 15 | simulator 결과를 physical device 결과로 부풀리는가? | 통과 — Android AVD와 iOS Simulator로만 표기했다. |
| 16 | screenshots를 PixelCopy/Metal drawable readback으로 오인시키는가? | 통과 — visual-only caveat를 남겼다. |
| 17 | PR check rollup이 비었는데 CI green이라고 하는가? | 통과 — CI 통과로 세지 않는다고 기록했다. |
| 18 | WPT 또는 public CSS admission을 완료로 읽게 하는가? | 통과 — 범위 밖이라고 명시했다. |
| 19 | 기존 `#1246` 기록이 `#1249`보다 최신으로 남는가? | 통과 — roadmap에서 `#1249`를 앞에 추가했다. |
| 20 | `#1246` paragraph가 center 구현을 미착수라 잘못 말하는가? | 통과 — 잘못된 next-task 문구를 `#1249` 병합 이력으로 교체했다. |
| 21 | 새 safe-end 계획이 이미 구현된 것처럼 표현되는가? | 통과 — plan/evidence는 있고 구현/runtime은 미시작이라고 적었다. |
| 22 | safe-end oracle에서 positive/overflow 예상이 규범과 어긋나는가? | 통과 — `safe end` fallback, `unsafe end` retained alignment를 W3C draft와 대조했다. |
| 23 | end 정렬 후보가 track sizing 변경을 끼워 넣는가? | 통과 — rows `[10,10,10]`, 46px gutters 불변으로 제한했다. |
| 24 | safe-center regression control을 빠뜨렸는가? | 통과 — safe/unsafe center overflow scene을 8-scene oracle에 포함했다. |
| 25 | 신규 계획에서 unsupported values가 넓게 열리는가? | 통과 — safe/unsafe start, distributed, baseline 및 다른 shapes를 reject로 둔다. |
| 26 | 신규 plan review를 구현 review와 같은 증거로 세는가? | 통과 — plan review는 사전 검토이며 구현 후 20개 review를 대신하지 않는다고 썼다. |
| 27 | 문서의 신규 relative links가 존재하지 않는 파일을 가리키는가? | 통과 — plan, review, precomparison, runtime artifacts를 로컬 경로에서 확인했다. |
| 28 | 신규 roadmap paragraph가 다음 `<p>`를 감싸는 malformed HTML인가? | 발견 후 수정 — 닫는 `</p>`를 넣고 이후 stale candidate 문구도 갱신했다. |
| 29 | next candidate가 구체적 계획 없이 이름만 있는가? | 통과 — safe-end plan, Chrome evidence와 20-point plan review가 연결되어 있다. |
| 30 | 변경이 사용자의 untracked files를 stage/delete하거나 worktree를 쓰는가? | 통과 — normal checkout에서 tracked docs만 편집하고 증거 파일을 명시적으로 추가한다. |

체크 전 `git diff --check` 및 HTML paragraph boundary 검사를 수행한다. 이 문서 검토는 구현/runtime review 20개와 safe-end 최종 PR diff review 30개를 대신하지 않는다.
