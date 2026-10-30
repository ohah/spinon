# C11.4 direct-text aspect-ratio min/max 계획 검토 20개

**검토 대상:** [계획](./c11-4-direct-text-aspect-ratio-min-max.md) 및 [17-case 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.md) · 2026-10-30

각 항목은 별도 실패 경로를 확인한다. 첫 draft에서 vendored Taffy 버전을 `0.9.0`으로 잘못 적은 점을 실제 `vendor/taffy/Cargo.toml`과 `crates/spinon-layout/Cargo.toml` 대조에서 찾아 `0.15.0`으로 수정했다.

1. **Grid 용어의 대상성:** non-replaced text item에 replaced-only `transferred size suggestion`을 적용하지 않는지 확인했다. 계획은 CSS Grid §6.6 `content size suggestion` 경로와 두 용어를 구분한다.
2. **규범 문서 상태:** CSS Grid/Sizing Level 4 링크를 안정된 지원 계약으로 단정하지 않는지 확인했다. 초안 링크임과 fixture 범위를 명시했다.
3. **기존 collector 계약:** CSS 입력이 현재 collector의 row/column subset과 맞는지 확인했다. 사전 fixture를 지원되는 `auto` row 및 두 `max-content` column으로 맞췄다.
4. **Font 재현성:** 시스템 fallback이 17개 값을 바꾸지 않는지 확인했다. local Ahem과 embedded-font 두 입력의 readiness 및 결과 동일성을 요구한다.
5. **Text cardinality:** 문장·여러 run·추가 anonymous text가 넓은 admission으로 들어오지 않는지 확인했다. `AAAA`/`A` direct text 두 개만 허용한다.
6. **Ratio owner:** 짧은 item이나 두 item 모두 ratio를 소유하는 경우가 수용되지 않는지 확인했다. 유일한 owner를 `AAAA`로 제한하고, min/max 확장은 2:1만 허용하면서 min/max 초기값인 기존 arbitrary finite-positive ratio 회귀는 유지한다.
7. **Axis direction:** ratio를 곱해야 할 축과 나눠야 할 축이 뒤집히지 않는지 확인했다. 가로·세로 min/max 반례와 axis-swap mutation을 넣었다.
8. **Minimum transfer:** 반대 축 min이 contribution에 반영되는지 확인했다. `min-width:60`, `min-height:30`, 그리고 반대 max가 min 환산값을 누르는 장면을 고정했다.
9. **Maximum transfer:** 반대 축 max가 contribution에 반영되는지 확인했다. `max-width:30`, `max-height:15`, 그리고 반대 min이 max 환산값을 올리는 장면을 고정했다.
10. **Automatic min-content floor:** 기본 `min-width:auto`를 잘못 `0`처럼 취급하는 경로를 확인했다. `max-height-15`의 40px 폭과 명시적 `min-width:0` control의 30px 폭을 별도 유지한다.
11. **Min/max precedence:** min이 max보다 큰 경우의 constraint precedence를 확인했다. 가로·세로 source min-over-max 사례를 oracle에 둔다.
12. **Ratio basis와 edges:** numeric border-box와 `auto <ratio>` content-box를 혼동하거나 padding/border를 두 번 더하는 경로를 확인했다. nonzero edge case와 axis별 expected size를 명시했다.
13. **Unsupported CSS units:** percentage, `calc()`, 음수, 60px 초과, NaN/infinity, intrinsic keyword가 bounded px subset으로 둔갑하지 않는지 확인했다. 전부 fail-closed 조건이다.
14. **Computed-style coherence:** serialization, typed ratio/basis, min/max, style revision이 서로 다른 snapshot에서 합쳐지지 않는지 확인했다. node ownership과 revision 확인을 acceptance에 넣었다.
15. **Runtime fallback:** 불지원 입력이 `None`/기본 layout으로 조용히 대체되지 않는지 확인했다. adapter 단계 rejection을 명시했다.
16. **Vendored dependency:** 코드 추적 대상이 올바른 고정 버전인지 확인했다. 잠금된 local package는 Taffy `0.15.0`; 잘못 적힌 `0.9.0`을 고쳤다.
17. **Replaced path 격리:** 새 non-replaced 계산이 image ratio regression으로 새어 나가지 않는지 확인했다. replaced `transferred_size_suggestion` 기존 동작 유지와 이전 23-case 회귀를 요구한다.
18. **Intrinsic stage 귀속:** track frame만 보고 min-content, max-content, minimum contribution을 혼동하지 않는지 확인했다. stage trace로 어느 경로가 17개 output을 결정하는지 증명하도록 했다.
19. **Negative control 민감도:** fixture id나 CSS label을 바꿔 기대값을 고정해서 테스트를 속이지 않는지 확인했다. semantic computed values, exact expected matrix, unsupported geometry mutation을 검증한다.
20. **플랫폼·완료 과장:** AVD software renderer와 iOS pre-compositor drawable을 hardware/실기기 증거로 확대하거나 C11 완료로 표시하지 않는지 확인했다. 양 Simulator의 고정 행렬, artifact boundary, 상태 `72/213` 유지 및 미지원 한계를 적었다.

**결과:** 계획은 위 수정 후 사전 비교·구현 경계·검증 순서를 고정한다. 계획 PR diff의 독립 30관점 검토는 별도 파일에서 수행한다. 기능 구현 전 이 계획을 병합한다.
