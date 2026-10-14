# C13.1 clip 축·margin 행렬 PR delta 검토 · 30개 · 2026-10-14

검토 대상은 새 stacking test module, 선택 Chrome observation, 증거 문서, C13.1 계획/status/계약/index 및 PR #158 본문 갱신이다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 변경이 C13.1 branch 밖에 적용 | 작업은 격리 worktree `feat/c13-1-overflow-clip` 안에 있다. |
| 2 | 기본 checkout의 C11.3 dirty 변경과 혼합 | 새 fixture/evidence/test 경로는 기본 checkout에 존재하지 않음을 확인했다. |
| 3 | 기존 17-case oracle 입력 대신 새 기준을 사후 조작 | 기존 pinned fixture와 JSON을 그대로 사용하고 두 SHA를 기록한다. |
| 4 | Chrome 버전만 쓰고 revision을 생략 | 버전·revision·UA를 observation에 보존한다. |
| 5 | terminal-browser 결과를 Chrome 근거로 표기 | agent-browser 0.31.1의 version page와 oracle function을 사용했다. |
| 6 | CSS fixture 입력이 변경됐는데 hash는 그대로 | 테스트가 원본 HTML SHA-256을 확인한다. |
| 7 | 선택 행 집합이 예상 밖으로 늘거나 바뀜 | 세 case의 expected values를 Rust test에서 고정한다. |
| 8 | JSON의 hit range가 바뀌어 기대 결과가 따라 변함 | 테스트가 각 case의 computed axes, margin, frame, hit ranges를 상수와 비교한다. |
| 9 | frame 비교가 너비·높이를 빠뜨림 | x/y/width/height 네 값을 각각 0.5 CSS px 이내로 검사한다. |
| 10 | right/bottom frame edge를 잘못 계산 | 전체 hit scan 전 target right/bottom도 oracle 값과 대조한다. |
| 11 | target 영역만 일부 표본 query | Chrome 고정 축에서 floor..ceil-1 전 정수 좌표를 검사한다. |
| 12 | x와 y hit band가 뒤바뀜 | 각 축의 fixedAt과 ranges를 독립 처리한다. |
| 13 | 비연속 expected ranges를 하나의 범위로 합침 | JSON range 배열을 그대로 순회해 각 segment를 판정한다. |
| 14 | 비대상 결과 node까지 검증했다고 오해 | 문서가 target 여부 비교와 fallback ID 미검증을 구분한다. |
| 15 | `clip-y` 테스트가 x visibility를 보장하지 않음 | paint/hit x clip이 없고 Chrome 전체 x hit band가 맞는지 검사한다. |
| 16 | border-box margin paint와 hit을 한 결과로 가정 | 두 clip object의 축별 bound를 따로 비교한다. |
| 17 | hidden margin이 border-box를 확장하는 회귀 | hidden paint는 padding, hit은 border edge로 각각 assert한다. |
| 18 | test helper가 production 모듈로 컴파일 | stacking test tree의 `#[cfg(test)]` 하위 모듈에 둔다. |
| 19 | 테스트가 fixture tree를 실제 런타임과 다르게 축약 | root/absolute parent/target 관계와 CSS declaration을 명시하고 pinned target frame을 검사한다. |
| 20 | 지나치게 큰 viewport가 parent geometry를 우연히 바꿈 | viewport만 Chrome baseline과 같게 하고 각 parent/target frame도 대조한다. |
| 21 | live browser가 expected screenshot과 다른 빌드 | agent-browser에서 같은 fixture function과 Chrome version을 다시 확인했다. |
| 22 | JSON capture 시 browser console 오류 무시 | 이번 추가 행은 computed/rect/elementFromPoint collector 출력만 읽었고 별도 console 통과 주장을 하지 않는다. |
| 23 | 새 screenshot이 실제 pixel 검증처럼 보임 | screenshot/PixelCopy를 추가하지 않았으며 README에서 명시적으로 제외한다. |
| 24 | WPT candidate triage를 WPT 실행 결과로 오인 | 별도 WPT evidence에 triage와 not-run을 유지한다. |
| 25 | C13 전체 완료 체크 | STATUS checkbox는 미완료로 유지하고 C13.1 전체 matrix가 남았다고 쓴다. |
| 26 | Android 실기기 검증처럼 확대 | 새 변경은 host Rust unit test이며 실기기를 연결하지 않았다. |
| 27 | Android AVD 또는 iOS Simulator 앱 route 결과로 과장 | 이번 테스트에서 두 앱 route를 실행하지 않았다. |
| 28 | 반올림 허용오차가 plan과 다름 | frame threshold는 계획의 0.5 CSS px를 사용한다. |
| 29 | PR 본문에 검토 횟수만 추가 | 본문에는 관찰·검증 실체와 한계만 추가하고 횟수는 적지 않는다. |
| 30 | PR 본문·evidence link가 commit 뒤 stale | PR #158 본문에 세 행의 범위, package 47/0/1 test 결과, 미완료 gate와 evidence 링크를 반영했다. push 뒤 head·draft·checks를 재확인한다. |

발견된 결함은 테스트 helper의 accessor 타입 오류였고 수정 후 세 test가 통과했다. 새 PR delta는 runtime product code를 변경하지 않으며 C13.1 전체 검증이나 merge 승인이 아니다.
