# PR #1372 병합 후 상태·계약·로드맵 문서 diff 검토 30개

기준은 PR #1372 병합 commit `15b7263c7045c7f55f02d74307605a6d85b35ee5`다. 각 관점은 서로 다른 오류 경로를 확인하며, 아직 병합되지 않은 문서 PR의 실제 diff에 적용한다.

1. **병합 식별자** — GitHub PR #1372의 URL·번호가 문서 링크와 일치한다. **PASS**
2. **제목 메타데이터** — PR 제목이 `feat(layout): five-fit row-span item별 mixed minimum 지원`으로 기록된다. **PASS**
3. **병합 시각** — Rebase and merge 시각 `2026-11-12T05:20:43Z`가 PR metadata와 일치한다. **PASS**
4. **merge SHA** — 문서와 local HEAD가 `15b7263c7045c7f55f02d74307605a6d85b35ee5`를 사용한다. **PASS**
5. **base SHA** — 병합 전 base SHA `33d35857b6e3f7f5b2bc2f83781ec7ed711e99b6`를 보존한다. **PASS**
6. **PR head SHA** — 병합된 PR head `5a45e5a38e21d940012bed6a0a1a3bfa780af07c`를 구분해 적는다. **PASS**
7. **실제 diff 규모** — PR metadata의 67 changed files와 원격·로컬 diff 대조 결과를 기록한다. **PASS**
8. **CI 상태 과장** — check/workflow 0개를 CI green으로 바꾸어 쓰지 않는다. **PASS**
9. **5-fit과 6-fit 경계** — #1372 변경은 five-fit row-span, #1369 회귀는 six-fit mixed로 분리한다. **PASS**
10. **sample 분모** — 3,984 RuntimeSession sample을 전체 Chrome oracle과 같은 분모로 표현하지 않는다. **PASS**
11. **Chrome oracle 범위** — 414,720 Chrome observation을 제품 RuntimeSession 실행 완료로 부르지 않는다. **PASS**
12. **five-fit uniform 회귀** — 34,560/34,560 수치를 해당 uniform endpoint에만 연결한다. **PASS**
13. **six-fit uniform 회귀** — 11,520/11,520 수치를 six-fit uniform row-span 회귀에만 연결한다. **PASS**
14. **six-fit mixed 미실행분** — #1369 matrix는 11,520/138,240만 재실행했고 126,720 미실행임을 유지한다. **PASS**
15. **caps profile** — 다섯 typed cap의 값과 singleton tail 조건을 five-fit 문맥에만 쓴다. **PASS**
16. **control minimum** — 첫 explicit-row control의 `min-height:0` 고정을 유지한다. **PASS**
17. **item별 의미** — minimum mask가 spanning text item별 독립 설정임을 적는다. **PASS**
18. **admission 범위** — 일반 CSS Grid나 다른 fit 개수/auto-placement가 새 지원 범위인 것처럼 쓰지 않는다. **PASS**
19. **Android 근거** — API 37.2 ARM64 AVD/SwiftShader를 실기기나 hardware GPU로 확대하지 않는다. **PASS**
20. **iOS 근거** — iPhone 17 Pro/iOS 26.2 Simulator smoke를 실기기 검증으로 확대하지 않는다. **PASS**
21. **fixture 화면 한계** — 기본 Root HTML route 화면을 five-fit mixed fixture 화면으로 표현하지 않는다. **PASS**
22. **surface 한계** — PixelCopy, Metal drawable, compositor pixel parity를 수행했다고 주장하지 않는다. **PASS**
23. **API/FFI 경계** — 공개 API 또는 FFI가 추가된 것으로 표시하지 않는다. **PASS**
24. **contract version** — 내부 numeric contract `0.1.0`을 유지한다. **PASS**
25. **C11.4 상태** — bounded PR 병합과 C11.4 전체 완료를 구분한다. **PASS**
26. **C11 상태** — C11 parent 완료 체크를 추가하지 않는다. **PASS**
27. **공식 집계** — `78/214`를 현재 완료 수로 유지한다. **PASS**
28. **roadmap 최신 카드** — 현재 카드·hero가 PR #1372와 merge SHA를 가리킨다. **PASS**
29. **roadmap 실행 stage** — 칸반 JS marker가 `#1372 merged · next candidate not selected`로 맞춰진다. **PASS**
30. **링크 경로·시각 회귀** — 새 plan/contract/evidence/STATUS 경로를 선택 build와 Agent Browser에서 확인하고 roadmap SVG의 정적 도형에는 stale PR metadata가 없다. **PASS**

이 검토는 변경된 상태 문서·내부 계약·계획·로드맵과 선택 미리보기 경로에 한정된다. 전체 문서 사이트의 모든 링크 검증이나 C11.4/C11의 제품 완료 판정이 아니다.
