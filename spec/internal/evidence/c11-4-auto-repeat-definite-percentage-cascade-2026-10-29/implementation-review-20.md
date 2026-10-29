# C11.4 definite percentage auto-repeat cascade 구현 검토 20개

각 관점은 `auto_repeat_calc_minimum.rs`의 테스트 diff, pinned Stylo computed value, `auto_repeat_track_has_supported_fixed_size` allowlist 및 기존 `spinon-layout` percentage auto-repeat 경계를 대조했다.

1. **축 누락** — Columns와 Rows 각각을 포함한다. PASS.
2. **repeat mode 누락** — AutoFill과 AutoFit을 각각 포함한다. PASS.
3. **교차 조합 공백** — 네 축/mode 조합이 한 번씩 존재한다. PASS.
4. **잘못된 definite basis** — root width/height가 고정되어 두 축 모두 definite하다. PASS.
5. **border-box 혼동** — 124px outer size와 네 변 2px border로 content basis는 120px이다. PASS.
6. **다른 축의 우발적 반복** — 비반복 축은 단일 48px track으로 고정한다. PASS.
7. **Stylo가 declaration을 무시하는 거짓 양성** — 결과 track을 정확히 검사한다. PASS.
8. **percentage 소수 변환** — computed result `Percentage(0.1)`을 exact 비교한다. PASS.
9. **repeat metadata 누락** — mode 기대값은 `Some(...)`과 비교해 metadata 부재도 실패한다. PASS.
10. **AutoFill/AutoFit 혼동** — 각 행의 authored mode와 metadata mode를 연결한다. PASS.
11. **잘못된 입력을 넓게 허용** — 새 positive 입력은 finite positive `10%`로 제한된다. PASS.
12. **0% 회귀** — 기존 0% + flexible maximum positive 검사는 변경되지 않았다. PASS.
13. **음수·비유한 값 범위 혼입** — 새 테스트는 이들을 지원한다고 주장하지 않는다. PASS.
14. **intrinsic maximum 거부 제거** — `minmax(0%|10%, auto|max-content)` 네 조합을 유지한다. PASS.
15. **오류 노드 식별 손실** — 기존 rejection assertions는 root node와 property를 계속 검사한다. PASS.
16. **오류 property 혼동** — unsupported negative case의 `grid-template-columns` 확인은 유지한다. PASS.
17. **계산 순서 위장** — cascade test는 layout의 반복 수·최종 track size를 주장하지 않는다. PASS.
18. **indefinite axis 통과** — layout의 별도 test가 indefinite/clamped axis 거부를 유지한다. PASS.
19. **변경이 제품 동작을 가장함** — diff에 production cascade/layout 변경은 없다. PASS.
20. **과도한 완료 승격** — C11.4와 `72/213`은 그대로 미완료로 기록한다. PASS.

결론: test expectation을 기존 CSS grammar 및 layout contract와 일치시킨다. 제품 동작을 변경하지 않는다.
