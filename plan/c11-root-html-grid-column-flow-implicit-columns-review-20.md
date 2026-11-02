# C11 Root HTML column-flow implicit columns 계획 실패 관점 검토 20개

검토 대상은 [전용 계획](./c11-root-html-grid-column-flow-implicit-columns.md), 독립 기대값, Chrome 156 사전 비교다. 각 관점은 구현 전 계약에 별도로 대조했다.

1. **auto-column 패턴 시작 위치** — explicit 마지막 track 다음의 첫 implicit column부터 auto sizing pattern을 시작하는가? 계획은 explicit `[30,40]` 뒤에 `[15,25]`가 두 번 오는 기대값을 고정한다. 통과.
2. **column-major source order** — auto item이 각 열을 위에서 아래로 채우는가? 12개 frame 표가 두 explicit rows 기준 6개 열을 순서대로 나열한다. 통과.
3. **반복 횟수 오판** — 12개 item, 2행이면 총 6열이 필요하며 explicit 2열 이후 implicit 4열이어야 한다. 표와 독립 JSON이 이를 반영한다. 통과.
4. **열/행 gap 분리** — column gap은 x origin, row gap은 y origin에 한 번만 포함되는가? 기대값은 x `[0,35,80,100,130,150]`, y `[0,23]`를 고정한다. 통과.
5. **track sizing과 item sizing 혼동** — 빈 자식의 자동 크기로 track을 추론하지 않는가? 비교 fixture는 item별 fixed px 크기를 명시하고 auto-sizing을 범위 밖으로 둔다. 통과.
6. **implicit rows 유입** — column mode에서 미지정 row가 늘어날 수 있는가? 모든 placement가 auto이고 두 행만 명시하며 layout 검토는 expanded rows를 거부한다. 통과.
7. **dense 재배치** — `column dense`가 hole을 채우며 source order를 바꿀 수 있는가? dense flow는 계속 fail-closed하도록 범위와 rejection 기준에 포함한다. 통과.
8. **명시 placement 무시** — authored line/span/area를 silently auto로 낮추는가? 네 placement longhand가 모두 `auto`인 경우만 받아 negative tests를 요구한다. 통과.
9. **`order` 정렬 차이** — nonzero `order`가 source-order 의미를 바꾸는가? `order:0`만 받고 source order를 고정한다. 통과.
10. **행 수가 0인 계산 오류** — required columns 계산에서 나누기 0이 가능한가? explicit rows가 비어 있지 않은 fixed px 목록이어야 하며 typed projection과 layout 모두 확인한다. 통과.
11. **계산 overflow** — item 수/행 수 산술이 usize overflow를 일으키는가? checked arithmetic, quota 검사, overflow/rejection tests를 계획했다. 통과.
12. **10,000 track 우회** — explicit 및 implicit track 합계가 개별 상한 검사를 우회하는가? 총 열 개수와 auto-list 길이를 합산 상한에 포함하도록 계획했다. 통과.
13. **거대 allocation 선행** — quota 확인 전에 item/track 벡터를 확장하는가? bounded allocation과 expansion 전 초과 거부를 요구한다. reservation 실패 주입 확인을 명시하지 않아 수정 필요.
14. **잘못된 CSS track을 기본값으로 오인** — `auto`, `%`, `fr`, `minmax`, `fit-content`, math가 silent fallback되는가? 이 값은 profile에서 fail-closed하고 property 진단을 확인한다. 통과.
15. **track list shape 오류** — 빈 typed list나 지나치게 긴 list를 받는가? nonempty와 `GRID_MAX_TRACKS` 범위를 계획했다. 통과.
16. **negative/NaN/infinity** — 계산 typed value가 음수나 non-finite일 때 Taffy에 전달되는가? 검증 및 synthetic typed-value rejection을 요구한다. 통과.
17. **line names/areas/repeat 결합** — 다른 Grid grammar가 이 profile로 섞이는가? named lines, area, fixed/auto repeat을 새 조합으로 열지 않는다고 명시했다. 통과.
18. **hidden child 수 산입** — `display:none`이 implicit column을 만들 수 있는가? 기존 hidden filtering 유지와 hidden-item 회귀를 요구한다. 통과.
19. **root/html host 변형** — 이 profile이 특정 tag, selector, node ID에 의존하는가? 실제 기본 route로 검증하고 identity 기반 분기 금지 원칙을 선행 계획에서 상속한다. 통과.
20. **부분 scene 공개/낡은 revision** — 잘못된 track 또는 stale style이 partial geometry를 publish하는가? layout 결과·revision·render scene atomicity와 stale-revision 회귀를 요구한다. 통과.

## 계획 수정

관점 13에서 quota 이전 reserve 실패가 일반 오류로 변환되고 scene이 노출되지 않는지 증명할 실패 주입 조건이 부족했다. 계획의 검증 기준에 auto-column expansion 및 item placement reserve 실패 주입을 추가한다. 나머지 관점은 현재 계획 범위와 독립 oracle로 확인했다.
