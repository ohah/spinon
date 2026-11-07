# C11.4 fit-content implicit-row track-list plan · 20 adversarial checks

이 검토는 [계획](./c11-4-root-html-fit-content-auto-rows-list.md), Chrome 157 사전 비교, CSS Grid 규범, 현재 Style projection/Layout 경계를 대조했다. 구현 검토나 PR 변경분 검토로 재사용하지 않는다.

1. **첫 implicit row의 시작 index** — explicit 10px row 뒤 첫 pattern entry가 적용되는지 확인한다. 2-entry/3-entry 관측 및 독립 기대값에 각 row의 local y를 포함한다. 계획에 반영.
2. **2-entry 주기 반복** — 네 implicit rows에서 `[A,B,A,B]` 순서가 유지되는지 two-caps scene으로 구분한다. 6 scene 중 이 입력이 직접 검사한다.
3. **3-entry 주기 반복** — 네 implicit rows에서 세 번째 후 첫 entry로 돌아오는지 three-caps scene으로 확인한다. 임의 목록 펼치기를 허용 기준에 넣는다.
4. **mixed typed entries 순서** — fit-content percentage, fixed px, direct percentage가 서로 다른 타입으로 보존되는지 mixed-size-pattern으로 확인한다.
5. **fit-content와 direct percentage 구별** — fit-content cap은 content와 minimum에 의해 제한되지만 direct percentage row는 10px track으로 남는다. computed typed projection과 layout rows를 모두 검사한다.
6. **percentage basis 선택** — viewport, surface 또는 border box가 아닌 definite 100px Grid content-box block size를 사용한다. 10/20/40/70/80/120% 경계와 40×100px container를 명시했다.
7. **0% cap** — 유효한 0 cap에서 zero-height row와 item frame이 유지되는지 zero-and-overflow scene과 기존 singleton regression으로 검사한다.
8. **100% 초과 cap** — 120%를 100%로 임의 clamp하지 않고 max-content contribution이 이기는지 확인한다.
9. **minimum floor가 cap보다 큼** — 첫 implicit item의 min-height 28px이 20% cap보다 큰 28px row를 만드는지 검사한다.
10. **nested content overflow** — 20px track에서 34px payload가 visible하게 overflow하는 기존 contract를 유지하고 frame과 simulator 화면을 구분해 기록한다.
11. **fixed-px track identity** — mixed list의 12px entry가 content size나 주변 percentage로 바뀌지 않는지 typed value와 final rows로 검사한다.
12. **direct percentage track identity** — 10% entry가 100px basis의 10px로 계산되고 8px child contribution에 맞춰 줄어들지 않는지 확인한다.
13. **CSS computed serialization** — computed `grid-auto-rows` serialization이 list 순서·track kind를 유지하는지 projection/runtime assertion으로 검사한다.
14. **unsupported list member** — 지원 track 뒤의 `minmax()`, `fr`, intrinsic keyword, `auto` 또는 계산식 AST가 일부만 반영되지 않고 property failure가 되는지 확인한다.
15. **finite/non-negative 경계** — zero는 허용하고 음수·NaN/무한 typed 값은 fail-closed한다. Parser invalid declaration과 유효하지만 미지원 typed value를 다른 rejection stage로 확인한다.
16. **indefinite basis** — Grid height가 `auto` 또는 percentage인 경우 cap basis를 viewport로 대체하지 않고 거부한다.
17. **placement topology** — row sparse flow와 auto placement만 지원한다. column/dense flow, named/explicit placement, negative implicit rows, areas 및 repeat은 거부한다.
18. **owner/item admission** — visible Grid owner 하나와 정해진 3-item singleton 또는 5-item list topology만 허용하고 두 번째 owner, hidden-item 재분류, text/replaced/nested Grid shape를 fail-closed한다.
19. **resource ceiling** — 목록 크기 및 implicit item capacity는 기존 track/item limits를 따라야 하며 목록을 truncation하거나 overflow arithmetic으로 partial output을 내지 않는다.
20. **atomicity/regression/product route** — unsupported revision은 frame/draw snapshot을 발행하지 않고 같은 RuntimeSession의 valid revision이 복구되어야 한다. 기존 singleton px/percentage/length-only calc, 다른 Grid mode, 기본 parsed Root HTML route를 회귀 확인한다.

## 결과

20개 모두 계획 수준에서 비교 입력·typed identity·실패 경계·원자성 기준으로 해소했다. Chrome/독립 비교는 6개 모두 exact이며, 일반 CSS admission·임의 track grammar·전체 WPT는 목표 범위로 넓히지 않았다. 구현 전 확인해야 할 핵심 위험은 mixed list에서 Taffy가 track list 순서를 유지하는지와 fixed-size Grid 안에서 fit-content track minimum/floor가 보존되는지다. 구현/runtime 검토는 이 계획 검토와 별도로 새 사례를 작성한다.
