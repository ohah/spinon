# C11.4 definite percentage auto-repeat maximum 계획 실패 관점 20개

계획 자체만 검토했다. 이 관점을 구현/runtime 검토나 PR diff 검토에 재사용하지 않는다.

1. **자동 반복 문법** — auto-repeat의 fixed-size 제한과 minmax fixed breadth의 허용을 CSS Grid 2025 CRD §7.2.3.1·§7.2.3.2에 연결했다.
2. **maximum 선택 규칙** — definite max가 있을 때 min 대신 max로 반복 수를 정한다는 규범을 입력·기대값에 분리해 반영했다.
3. **max/min flooring** — min 25%, max 20%의 역순 범위를 invalid로 오인하지 않고 effective max 46px로 바꾸어 계산한다.
4. **zero maximum** — max 0%를 허용하되 positive min에 floor하므로 division-by-zero나 zero-size repeat이 발생하지 않는다고 한정했다.
5. **repeat count 수식** — n×track+(n−1)×gap 식을 사용했고 46px case의 세 번/네 번 경계를 검산했다.
6. **gap 개수** — 8px gap은 track 사이에 N−1개만 포함하며 외곽 gutter로 더하지 않는다.
7. **columns percentage basis** — 25%와 calc percentage를 width 184px content box 기준으로 계산한다.
8. **rows percentage basis** — rows는 definite height 184px content box 기준으로 별도 장면을 둔다.
9. **box-sizing 혼동** — content-box 184px에 border가 있어도 percentage basis는 border box나 outer 188px이 아니다.
10. **min-size/max-size guard** — axis min-size 0과 max-size auto를 전제해 min/max clamp가 계산 basis를 바꾸지 않게 했다.
11. **calc dimension** — calc은 20px + 10% 한 식만 고정하고 다른 연산식으로 확대하지 않는다.
12. **subpixel quantization** — ideal 38.4px와 Chrome 1/64px used value 차이를 허용오차로 명시했다.
13. **auto-fit 시점** — repeat count 3을 먼저 보존하고 item placement 뒤 empty-track/gap collapse를 별도 판정한다.
14. **auto-fill 빈 track** — 두 item만 두어 남는 반복 track이 collapse되지 않는 결과를 확인한다.
15. **양축 동시 계산** — columns와 rows를 각각 분리한 단일 축 matrix에 더해 같은 Grid의 두 축에 percentage/calc maximum을 함께 지정한 scene을 두어 shared calc binding 및 cross-axis side effect를 살핀다.
16. **fixed control 강도** — percentage 및 calc 각각 같은 effective fixed maximum의 control을 제공한다.
17. **min percentage 회귀** — percentage max 확대가 positive percentage minimum의 existing path나 zero-min 거부를 바꾸지 않게 제한했다.
18. **typed calc binding** — 기존 calc tree의 node와 property identity 검증을 유지하고 문자열 재파싱을 금지했다.
19. **독립성 오해** — Chrome observation은 규범 계산과 분리해 보존하고 브라우저 관측을 표준 기대값이나 Spinon 결과로 치환하지 않는다.
20. **검증 과장** — simulator screenshot을 runtime geometry/presentation 근거로 제한하고 WPT, general CSS admission, real device, hardware GPU, compositor-after pixels를 남은 범위로 명시했다.
