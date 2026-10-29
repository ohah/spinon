# C11.4 definite percentage auto-repeat PR 변경 검토 30개

검토 범위는 cascade test, `spec/STATUS.md`, 이 evidence 디렉터리다. 제품 cascade/layout 코드는 바뀌지 않았다.

1. **CSS grammar 근거** — §7.2.3.1 direct fixed-breadth grammar에 맞춘 양성 기대다. PASS.
2. **definite axis 조건** — test root는 고정 inline/block size를 사용한다. PASS.
3. **content-box basis** — 124px border-box와 4px border로 120px basis다. PASS.
4. **Columns 경로** — auto-fill과 auto-fit을 모두 검사한다. PASS.
5. **Rows 경로** — auto-fill과 auto-fit을 모두 검사한다. PASS.
6. **조합 완전성** — axis/mode 교차 4개가 전부 있다. PASS.
7. **computed AST 형상** — `Percentage(0.1)` exact equality를 요구한다. PASS.
8. **repeat metadata 부재** — 누락 시 명시적 panic으로 실패한다. PASS.
9. **repeat mode 보존** — authored mode와 computed mode가 같아야 한다. PASS.
10. **repeat range** — 단일 반복 descriptor 범위 `(0,1)`을 확인한다. PASS.
11. **non-repeated axis** — 다른 축을 fixed 48px로 고정해 교란하지 않는다. PASS.
12. **기존 minimum positive controls** — calc/direct percentage minimum tests를 보존한다. PASS.
13. **intrinsic maximum 거부** — auto와 max-content maximum 조합을 유지한다. PASS.
14. **zero percentage negative coverage** — `minmax(0%, auto|max-content)` 거부를 유지한다. PASS.
15. **positive percentage negative coverage** — `minmax(10%, auto|max-content)` 거부를 유지한다. PASS.
16. **error property context** — 기존 `grid-template-columns` context assertion을 유지한다. PASS.
17. **error node context** — root NodeId assertion을 유지한다. PASS.
18. **other unsupported calc** — 별도 unsupported percentage calc tests는 변경하지 않는다. PASS.
19. **percentage maximum** — dedicated percentage-maximum tests는 변경하지 않는다. PASS.
20. **indefinite basis behavior** — layout suite의 indefinite/clamped-axis rejection이 유지된다. PASS.
21. **auto-fit layout behavior** — 기존 layout test의 empty-track collapse 검증이 유지된다. PASS.
22. **test module boundary** — 변경은 기존 `#[cfg(test)]` cascade test 파일에 머문다. PASS.
23. **product behavior boundary** — production Rust 소스 변경은 없다. PASS.
24. **public API boundary** — API signature와 interface version은 바뀌지 않는다. PASS.
25. **official completion** — C11.4 checkbox는 unchecked로 남는다. PASS.
26. **roadmap count** — 공식 전체 `72/213`을 올리지 않는다. PASS.
27. **evidence command** — README의 사전 재현 command가 test 이름과 일치한다. PASS.
28. **test evidence** — style 250/250, focused auto-repeat 46/46, layout 13/13을 각각 구분해 기록한다. PASS.
29. **Clippy honesty** — 기본 실행 실패와 단일 기존 lint 제외 재실행 통과를 따로 기록한다. PASS.
30. **platform claim** — app runtime을 바꾸지 않아 simulator를 실행하지 않았다고 명시한다. PASS.

결론: stale negative assertion를 고치며 기존 fail-closed 경계와 미완료 상태를 보존한다.
