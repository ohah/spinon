# C11.4 definite auto-repeat percentage calc 최소값 계획 실패 관점 20개

계획만 검토했다. 이 결과는 구현/runtime 검토나 PR diff 검토에 재사용하지 않는다.

1. **Grammar가 auto-repeat에 허용되는가** — 2025 CRD의 `<auto-repeat>` track은 `<fixed-size>`이고 `<fixed-breadth>`는 `<length-percentage>`다. `minmax(calc(20px + 10%),1fr)`가 문법에 맞음을 계획의 표준 링크로 고정했다.
2. **잘못된 shorthand 해석** — percentage calc를 flexible/intrinsic size로 취급하지 않도록 fixed minimum과 `1fr` maximum을 구분해 기대값을 계산했다.
3. **Columns 기준 축 혼동** — 열 percentage는 definite inner inline size 112px의 10%인 11.2px이다. viewport, border box 또는 height를 기준으로 사용하지 않는다.
4. **Rows 기준 축 혼동** — 행 percentage는 definite inner block size 112px의 10%인 11.2px이다. column reference와 같은 axis basis의 조건을 별도로 명시했다.
5. **Cyclic percentage를 definite로 가장함** — grid axis 자체를 112px로 고정한다. intrinsic/auto root sizing에서 percentage를 다루는 알고리즘은 범위에서 제외했다.
6. **Repeat count와 1fr distribution 혼동** — 최소 31.2px로 반복 수 3을 먼저 정한 뒤 leftover space를 1fr track에 나눈다. 최종 112/3px track 크기를 repeat-count 입력처럼 쓰지 않는다.
7. **31.2px에서 부동소수 오차** — 독립 계산 20 + 0.1×112 = 31.2를 JSON에 보존한다. 사용 track 비교 오차는 1/64 CSS px로 제한했다.
8. **반복 count의 off-by-one** — 112/31.2 = 3.589…이므로 largest fitting integer는 3이다. floor 및 positive minimum 규칙을 고정했다.
9. **Percentage를 버려도 통과하는 입력** — percentage를 제외한 20px minimum이면 5 tracks가 가능하지만 지정된 calc는 3 tracks여야 한다. 112px 입력은 이 오류를 구분한다.
10. **Control이 약함** — numeric `31.2px` minimum을 동일한 axis·mode·items로 둬 calc가 다른 결과를 숨기지 못하게 했다.
11. **auto-fill placement 누락** — 3개 item을 둬 3 repeated tracks의 placement와 개별 frame을 모두 확인하게 했다.
12. **auto-fit collapse를 반복 수로 오해** — 한 item 장면에서 pre-collapse count 3과 post-placement `[112,0,0]`을 별도로 본다.
13. **Collapsed track 옆 gutter가 count를 바꿈** — gap은 0으로 고정해 반복 count와 collapsed-gutter 동작을 분리하고, 이 slice가 nonzero gap을 검증했다고 주장하지 않는다.
14. **두 축 동시 계산의 cross-axis dependency** — row/column은 여섯 별도 scene이며 한 scene에서 auto-repeat은 한 axis에만 있다.
15. **여러 repeat을 불법적으로 허용** — 한 track list와 axis에 auto-repeat 하나로 제한하고 다른 profile의 grammar는 변경하지 않는다.
16. **Invalid/non-finite 값 통과** — 이번 positive finite expression만 allowlist 대상으로 삼고, invalid/unsupported expression은 fail-closed 유지 조건을 acceptance에 둔다.
17. **비율이 들어간 모든 Calc로 확대 해석** — `20px + 10%` 고정 입력 하나만 지정하고 다른 operator, percentage, zero/negative basis는 제외했다.
18. **Computed style 재파싱 위험** — 기존 Stylo Calc binding을 사용하고 CSS 문자열 파서나 전용 evaluator를 추가하지 않는 조건을 정했다.
19. **Simulator 화면을 픽셀 동등성으로 과장** — geometry/track/trace와 draw-present를 분리하며 PixelCopy·Metal 표본과 compositor 경계를 PR에 구체적으로 적게 했다.
20. **전체 C11.4 또는 지원 API로 승격** — checkbox, `74/207`, 계약 0059 `0.1.0`을 유지하고 WPT, 일반 CSS admission, real-device 및 hardware GPU를 제외한다고 적었다.
