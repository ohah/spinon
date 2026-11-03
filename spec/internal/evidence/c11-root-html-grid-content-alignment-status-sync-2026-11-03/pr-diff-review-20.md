# PR diff 적대 검토 20개 · C11.1 content-alignment 상태 동기화

**검토 대상:** 공식 상태 대장 `spec/STATUS.md`에 PR #1121의 제한 구현·검증 근거를 추가한 문서 diff
**검토 근거:** PR #1121의 GitHub merge metadata, [구현 계획](../../../../plan/c11-root-html-grid-content-alignment.md), [내부 계약 0059](../../0059-c11-grid-layout.md), [Chrome 사전 비교](../c11-root-html-grid-content-alignment-precomparison-2026-11-03/README.md), [Android/iOS 실행 근거](../c11-root-html-grid-content-alignment-runtime-2026-11-03-pass02/README.md)
**결과:** 20개 모두 통과. 이 검토는 문서 주장·범위·연결만 확인하며 제품 코드나 플랫폼 실행을 다시 검증하지 않는다.

1. **PR 식별:** 번호와 링크가 #1121 및 `ohah/spinon`을 가리킨다 — 통과.
2. **병합 시각:** UTC `2026-11-03T14:29:31Z`가 GitHub `mergedAt`과 일치한다 — 통과.
3. **병합 SHA:** `a5ed70206342d35a99a19e1297772e08d1bdaa1e`가 GitHub merge commit OID와 일치한다 — 통과.
4. **체크 결과:** status check rollup이 비어 있으므로 CI 통과를 주장하지 않는다 — 통과.
5. **상태 원본:** 기록은 미리보기나 개인 계획이 아니라 공식 `spec/STATUS.md`의 C11 섹션에 추가됐다 — 통과.
6. **체크 승격:** C11 상위 체크박스를 바꾸지 않았고 이 제한 slice를 전체 Grid 완료로 표현하지 않는다 — 통과.
7. **집계 불변:** 공식 완료 수를 `72/213`으로 유지한다 — 통과.
8. **profile 경계:** Root HTML 기본 runtime의 좁은 profile에만 적용된다고 한정한다 — 통과.
9. **container 입력:** definite fixed-size container 조건을 명시한다 — 통과.
10. **track 문법:** explicit rows/columns와 gap을 fixed-px로 제한한다고 기록한다 — 통과.
11. **implicit sizing:** implicit track이 필요 없는 대상과 `grid-auto-*` implicit track 조건을 구분해 기록한다 — 통과.
12. **placement·template:** sparse row flow, repeat·areas·line names 제외가 계획과 맞는다 — 통과.
13. **box 경계:** zero margin/padding/border와 visible overflow 조건을 포함한다 — 통과.
14. **item 집합:** static, fixed-size, empty direct `div`, all-auto placement 및 explicit cell 수 이하 조건을 기록한다 — 통과.
15. **writing mode:** LTR `horizontal-tb` 조건만 언급하며 RTL·다른 writing mode를 지원한다고 넓히지 않는다 — 통과.
16. **oracle 역할:** Chrome 156 결과는 비교로만 적고 CSS 전체 oracle 적합성으로 과장하지 않는다 — 통과.
17. **Rust 판정:** Rust geometry 확인을 구현 근거로 적되 별도 전체 suite 통과를 추가 주장하지 않는다 — 통과.
18. **플랫폼 실행:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 정확히 지칭한다 — 통과.
19. **화면 증거:** `layout=ready`, `boxes=7`을 화면 smoke로 한정하고 PixelCopy·Metal readback은 수행하지 않았다고 명시한다 — 통과.
20. **문서 링크:** 계획·내부 계약·사전 비교·실행 근거·이 검토의 상대 경로가 저장소 내 실제 파일을 가리키며, 추가 문서의 주장도 수정 diff 범위 안에 머문다 — 통과.
