# C11.1 parsed Root HTML dense auto-placement 계획 검토 · 20개 실패 관점

**검토 대상:** `c11-root-html-dense-auto-placement.md`와 Chrome 156 six-scene precomparison, 기준 `main` `bd6f68e54f5f3a5013fee505b3d89bd6946c3dc0`.

1. **중복 기능 재구현:** C11.2.3 already owns dense placement. **통과** — 기존 normalizer만 재사용하고 제품 Root HTML projection과 mode gate가 결손이다.
2. **잘못된 입력 경로:** declaration 문자열을 앱에서 재파싱할 위험. **통과** — computed typed flow 사용을 명시했다.
3. **`dense` 기본 축:** `grid-auto-flow:dense`를 별도 미지원 값으로 취급할 위험. **통과** — computed default-row 해석을 `row dense`와 함께 지정했다.
4. **Sparse 대조:** dense 재배치를 증명하지 못하는 동일 결과만 있을 위험. **통과** — row/column sparse controls에서 서로 다른 cell을 고정했다.
5. **Row/column 전치:** ColumnDense를 row algorithm으로 처리할 위험. **통과** — 두 축 각각의 gap-fill과 collision scene을 요구한다.
6. **Definite item 충돌:** dense가 확정 배치 item 위에 덮을 위험. **통과** — positive definite line collision control을 포함한다.
7. **Span 충돌:** 복수 span 사이의 occupancy 누락. **통과** — 기존 candidate matrix와 span collision 장면을 그대로 이식한다.
8. **Source order:** 시각 위치 변경을 DOM/paint 순서 재정렬로 혼동할 위험. **통과** — tree/accessibility/paint order 불변을 계약으로 둔다.
9. **`display:none`:** 숨긴 항목이 cell을 점유하는 위험. **통과** — occupancy 제외를 명시했다.
10. **`visibility:hidden`:** layout participant를 잘못 제외할 위험. **통과** — occupancy 유지가 명시됐다.
11. **`order`:** 우선순위 정렬을 일부만 구현할 위험. **통과** — nonzero `order`는 거부하고 0만 지원한다.
12. **Invalid line input:** 음수 또는 named line을 positive-line 지원으로 넓힐 위험. **통과** — 둘 다 명시적으로 fail-closed다.
13. **Unsupported combinations:** areas/repeats를 우연히 dense path로 받아들일 위험. **통과** — 현재 slice에서 제외하고 기존 거부를 보존한다.
14. **Implicit track creation:** 둘 이상의 implicit axis, track list cycling, 또는 새 track이 필요한데 해당 축에 `auto`를 쓰는 입력을 허용할 위험. **수정** — 기존 `row-dense-fixed-row`는 implicit column이 필요하고 `grid-auto-columns:auto`라 `unsupported_grid_auto_columns`가 기대값이다. `grid-auto-rows`와 `grid-auto-columns` 각각 `auto`는 해당 축에서 implicit track을 만들지 않는 경우에만 허용한다고 계획을 좁혔다. 단일 px 값과 10,000 축별 track 상한을 유지한다.
15. **Nested mixed flow:** 한 nested container의 실패/성공이 다른 container를 오염시킬 위험. **통과** — per-container 검증과 기존 nested candidate scene을 포함한다.
16. **Atomic rejection:** failure 뒤 partial track/frame/draw output을 남길 위험. **통과** — layout publication까지 부분 결과가 없어야 한다.
17. **Allocation failure:** reserve 실패 후 같은 worker 재사용이 손상될 위험. **통과** — missing handle와 allocation failure를 negative control에 포함한다.
18. **Oracle confusion:** Chrome 156이나 C11.2.3 내부 성공을 Spinon 적합성으로 부를 위험. **통과** — browser/C11.2.3은 oracle이며 product success와 분리한다.
19. **Platform evidence:** AVD/Simulator 표본을 실기기나 compositor 이후 결과로 부를 위험. **통과** — software renderer와 pre-compositor 한계를 기록한다.
20. **Completion inflation:** limited dense route를 C11.1/C11 완료로 올릴 위험. **통과** — checkbox와 `72/213` 유지 조건을 적었다.

**결론:** 검토에서 implicit `auto` sizing 허용 범위가 넓은 점을 찾아 계획을 수정했다. 구현 단계에서는 C11.2.3의 10,000 한계가 두 축 합산이 아닌 축별이라는 점도 원본 normalizer와 `auto-span-at-track-limit` 성공 scene에서 확인해 문구를 바로잡았다. 현재 계획은 각 축의 `auto` sizing을 해당 축에 implicit track을 만들지 않는 경우에 한정해 기존 `auto-track-no-implicit` 성공 및 `row-dense-fixed-row` 거부 계약을 구분한다. Implementation/runtime review와 PR diff review는 별도로 수행해야 한다.
