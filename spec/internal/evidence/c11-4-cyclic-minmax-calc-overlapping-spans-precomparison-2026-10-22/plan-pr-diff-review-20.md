# 계획 PR diff 실패 관점 검토 20개

계획 PR의 실제 diff를 기준으로 2026-10-22에 별도로 검토했다. 이 검토는 plan review와 구분하며 runtime code, simulator 결과 또는 구현 PR diff를 검토하지 않는다.

| # | 변경 실패 관점 | diff에서 확인한 조건 |
|---:|---|---|
| 1 | 계획 PR이 C11.4 완료를 체크하거나 완료 수를 올림 | `STATUS.md`의 C11.4 checkbox를 유지하고 완료 수를 `77/213`으로 명시했다. |
| 2 | 이미 병합된 #497을 구현 미착수로 남김 | 상위 계획과 `STATUS.md`에 #497의 merge SHA·runtime 결과를 추가했다. |
| 3 | #497의 simulator 범위를 실기기나 최종 compositor 증명으로 확대 | AVD/iOS Simulator와 Metal 제출 전 경계를 적었다. |
| 4 | 새 계획을 직전 slice가 이미 포함한 것으로 오해 | #497이 single spanning item, 새 계획이 overlapping span groups라고 차이를 적었다. |
| 5 | 계획 문서에 runtime 결과를 미리 서술 | 새 계획 상태는 사전 비교와 review만 완료, runtime 미착수다. |
| 6 | six-scene scene ID와 fixture ID 불일치 | HTML, expectation JSON, plan의 여섯 scene ID를 대조했다. |
| 7 | CSS font 경로가 잘못되어 fallback 관측 | HTML 상대 경로가 `tests/fixtures/resources/c15/Ahem.ttf`에 도달하며 Chrome Ahem check가 true다. |
| 8 | font 또는 fixture SHA가 내용과 불일치 | 고정한 fixture/font/observation/screenshot/expectation SHA-256 manifest를 파일에서 다시 계산해 기록했다. |
| 9 | screenshot이 다른 HTML 또는 다른 scene 수에서 생성 | browser screenshot을 여섯-row fixture와 같은 Agent Browser session에서 갱신했다. |
| 10 | 관측 JSON에 document position을 root-relative geometry로 오인 | observation은 page 좌표를 보존하고 계획 비교는 root-relative item frame을 쓴다. |
| 11 | cyclic case의 intrinsic basis와 final percentage basis 혼합 | 140px intrinsic root 계산과 final `10% + 2px = 16px`를 분리했다. |
| 12 | span-2 gutter를 contribution에서 잘못 계산 | 각 span-2에 내부 2px gutter 하나만 subtract한다. |
| 13 | 같은 span group의 planned increase를 합산 | item별 34/39px increments의 track별 maximum `[34,39,39]`을 명시했다. |
| 14 | span-3 group을 span-2 group보다 먼저 적용 | 112px tracks + 4px gutters 이후 남는 24px을 8px씩 분배한다. |
| 15 | definite 100px 계산에 max-content track을 그대로 강제 | 12px minimum과 남은 60px의 equal distribution에서 `[32,32,32]`를 계산했다. |
| 16 | frame과 overflowing text width를 한 값으로 취급 | span-3 100px frame과 140px text `scrollWidth`를 분리했다. |
| 17 | overflow sample이 border 안 또는 마지막 글자 밖에 위치 | root-relative `(126,7)`과 `(150,7)`을 각각 글자와 글자 뒤 control로 고정했다. |
| 18 | browser page white를 native host clear로 착각 | precomparison README가 browser/native 표면을 다른 배경으로 구분한다. |
| 19 | PR 계획 fixture를 제품 API 지원으로 설명 | fixed opt-in C11.4 profile, fail-closed guard, 범위 밖을 유지한다. |
| 20 | Agent Browser 종료 지침이 기존 세션까지 닫음 | 이번 전용 session/PID만 기록하고 global close 사용은 금지했다. |

## 판정

20개의 별도 실패 관점을 확인했다. 발견된 blocker는 없으며, checksum manifest와 JSON 구문을 검증했고 `git diff --check`가 통과했다.
