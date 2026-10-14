# C13.1 WPT 후보 분류 PR delta 검토 · 30개 · 2026-10-14

검토 대상은 PR #158의 WPT source triage, 고정 inventory validator, C13 roadmap/status/contract 갱신이다. 병합 승인이 아니며 WPT 실행 결과를 뜻하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | PR의 base/head를 잘못된 task branch로 갱신 | 변경은 기존 C13.1 PR #158 branch `feat/c13-1-overflow-clip`에 국한한다. |
| 2 | WPT commit가 plan의 Chrome version과 혼동 | WPT source commit과 Chrome 155 pixel oracle을 서로 다른 입력으로 기록했다. |
| 3 | source pin이 README와 JSON에서 다름 | README, precomparison, manifest 모두 같은 40자리 commit SHA를 쓴다. |
| 4 | commit timestamp가 로컬 날짜와 혼합 | API의 UTC timestamp를 명시했다. |
| 5 | root directory의 unrelated C11 worktree에 delta가 남음 | 분류 파일을 올바른 isolated C13 worktree로 이동하고 기본 checkout에서 생성 파일이 사라졌음을 확인했다. |
| 6 | inventory source 범위 설명이 지나치게 넓음 | prefix filter, HTML 확장자, reference 제외 조건을 manifest에 기록했다. |
| 7 | 58 source와 527 directory entries를 동일시 | 총 directory listing 수와 scope 필터 뒤 source 수를 분리한다. |
| 8 | reference page가 candidate count를 부풀림 | `-ref`를 sourceInventory와 classifications 모두에서 제외한다. |
| 9 | manifest 내부 목록만 같은 개수로 바꿔 source pin을 우회 | exact upstream commit, sourceInventory SHA-256, 분류 tuple SHA-256을 테스트 코드 상수로 고정하고 set equality도 확인한다. |
| 10 | category count가 문서마다 다름 | JSON, README, precomparison, roadmap, STATUS, contract를 13/17/28로 갱신했다. |
| 11 | test path가 tools/css-reference에서 잘못 resolve | Node test는 `../../spec/.../classification.json`을 열고 현재 worktree에서 3/3 통과했다. |
| 12 | negative-control validator가 실행 결과를 만들었다고 오인하거나 count-preserving 재분류를 놓침 | duplicate, missing, invalid category, unlisted path, category-swap, inventory-replacement, false WPT pass mutation을 검사한다. |
| 13 | validator test가 feature behavior test로 홍보 | README는 inventory/schema 검증으로 한정하고 runtime correctness로 확대하지 않는다. |
| 14 | WPT files가 repo에 복사되어 라이선스/업데이트 경계가 흐림 | upstream source를 vendor하지 않고 URL pin과 test path/요약만 저장했다. |
| 15 | basic clip test를 exact profile candidate로 무리하게 분류 | scroll parent, opacity, outline이 있는 clip-001..007은 Chrome oracle-only다. |
| 16 | alpha/outline paint가 WGPU border fix로 커버된 것으로 오인 | 이 effects는 C13.1 renderer candidate에서 분리했다. |
| 17 | hit-test oracle가 input dispatch 지원으로 확대 | candidate description에 fixed hit query만 포함하고 DOM event는 남은 gate로 둔다. |
| 18 | margin visual-box corner 결과가 rectangle profile로 확대 | radius variants는 unsupported/C22로 분류했다. |
| 19 | positive visual-box candidate가 border-radius 지원을 암시 | 후보는 radius 없는 counterpart에만 배치했다. |
| 20 | hidden margin ignored controls를 scroll state와 섞음 | 018·021 paint-only reftest는 C13.1 candidate이고 scroll API tests는 제외다. |
| 21 | negative offset source가 fixed CSS WD와 Chrome 차이를 숨김 | negative margin variants는 Chrome oracle-only로 분리했고 runtime pass 주장은 하지 않는다. |
| 22 | dynamic margin mutation이 초기 cascade 검증처럼 보고 | dynamic content-box/invalidation tests는 Chrome-only로 보류했다. |
| 23 | scroll size tests가 clip paint로 대신 증명 | scrollWidth/Height 및 scroll state tests는 C13.2/C29 unsupported로 남겼다. |
| 24 | scrollbar geometry가 software surface sample로 대신 증명 | scrollbar test는 C13.4로 분류했다. |
| 25 | writing mode/RTL이 horizontal-tb profile로 흡수 | vertical-rl과 RTL tests는 C17/C13.6 후속이다. |
| 26 | SVG/table/multicol candidate 누락 | 각 group을 current HTML Block/Flex profile 밖으로 분리했다. |
| 27 | radius/transform effect가 silent fallback으로 빠짐 | C22/C23 owner를 reason에 명시했다. |
| 28 | DOM content preservation을 clip이 DOM 삭제하지 않는 것과 혼동 | candidate는 HostDocument node/text retention invariant로 한정했다. |
| 29 | C13 또는 C13.1을 완료 체크 | STATUS checkbox는 미완료로 유지하고 남은 matrix/negative controls를 기록했다. |
| 30 | PR body와 evidence link가 stale | PR #158 body에 WPT triage 수치, WPT not-run, 남은 matrix/negative-control 관문과 evidence link를 갱신해 대조했다. |

검토 중 분류 경계 수정이 필요하면 manifest count와 negative controls를 다시 실행한다. 현재 evidence validator는 3/3 pass다. Full C13.1 gate, WPT execution, product CSS support, PR merge는 이 delta의 결과가 아니다.
