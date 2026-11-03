# 계획 실패 관점 검토 20개 · Root HTML Grid item alignment

**검토 대상:** [C11.1 `align-items`·`justify-items` 계획](./c11-root-html-grid-item-alignment.md)
**검토 전 상태:** 제품 코드 변경 전. pinned HeadlessChrome 156 observation, 독립 expected frames, 현재 projection/parser/adapter 코드를 대조했다.
**결과:** 계획 단계 20개 경계 확인 완료. 기능 구현 검토와 PR diff 검토는 별도 수행한다.

1. **계획 순서:** oracle와 independent expected가 구현 전 고정되어 있고, feature tests는 그 뒤에 추가하도록 순서를 정했다 — 통과.
2. **위치 vs. track alignment:** `align-items`·`justify-items`를 item self-alignment로, content distribution을 별도 기존 경로로 설명했다 — 통과.
3. **same-profile admission:** 새 profile이나 FFI를 만들지 않고 existing Root HTML eligibility를 재사용한다 — 통과.
4. **definite geometry:** container와 track이 fixed finite px이고 item이 fixed size인 조건이 explicit하다 — 통과.
5. **implicit track interaction:** auto-placement와 grid-auto track이 auto/default인 경우만 허용하고 implicit extent 추가를 닫았다 — 통과.
6. **named structure:** area, named line, repeat 및 dense placement를 제외 범위에 적었다 — 통과.
7. **alignment values:** Box Alignment positional values와 현재 typed parser 표현 범위를 대조했다 — 통과.
8. **flex aliases:** Grid context에서 flex-start/end가 start/end처럼 동작한다는 규범 처리를 계획했다 — 통과.
9. **physical alignment:** left/right가 justify-items에만 적용되고 horizontal LTR inline edge로 매핑되도록 제한했다 — 통과.
10. **direction/writing mode:** left/right 매핑을 다른 direction/writing-mode로 확장하지 않고 해당 shape에서 거부한다 — 통과.
11. **overflow safety:** safe/unsafe center/end positive와 overflow controls를 모두 고정했다 — 통과.
12. **normal/stretch fixed-size item:** auto-size stretch로 과장하지 않고 fixed item frame을 별도 대조한다 — 통과.
13. **shorthand cascade:** place-items를 authored string에서 추측하지 않고 Stylo computed longhand 두 값으로 처리한다 — 통과.
14. **self override:** child `align-self`·`justify-self`는 auto만 허용해 precedence를 묵시적으로 생략하지 않는다 — 통과.
15. **baseline:** first/last baseline을 포함한 baseline contribution을 명시적으로 닫고 C11.4 경계로 남긴다 — 통과.
16. **legacy:** `justify-items:legacy`의 at-risk 및 context-dependent inheritance를 범위 밖에 둔다 — 통과.
17. **invalid vs unsupported:** CSS invalid declaration의 cascade fallback과 유효한 미지원 runtime value를 다른 결과로 비교하도록 했다 — 통과.
18. **error attribution:** 잘못된 justify-items가 align-items 오류로 잘못 보고되지 않도록 exact property를 판정한다 — 통과.
19. **publication/failure:** unsupported shape·stale revision rejection에서 partial frame/draw publication이 없는지 검증 관문에 넣었다 — 통과.
20. **claim limits:** simulator, surface sampling, WPT/public support, C11 parent promotion을 서로 다른 증거로 다루고 전체 `72/213`을 유지한다 — 통과.
