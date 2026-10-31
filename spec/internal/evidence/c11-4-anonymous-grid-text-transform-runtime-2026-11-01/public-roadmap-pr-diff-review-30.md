# C11.4 text-transform 공개 로드맵 PR diff 실패 관점 검토 30개

- 대상: 공개 로드맵 카드 및 이 검토 기록을 추가하는 PR #954
- 기준: `main` `7bc111a6c3734c47823da34ed1e451c4e7dc1290`
- 검토 범위: `spec/public/roadmap.html`의 C11.4 text-transform card 한 개와 해당 링크/근거. 다른 카드와 CSS 호환성 범위는 수정하지 않았다.
- 판정: 상태 수, 병합 메타데이터, 링크와 지원 한계를 과장하는 문제를 발견하지 못했다.

1. **C11 전체를 완료로 오인** — 카드에 C11 미완료와 official `72/213`을 명시한다.
2. **C11.4 전체를 완료로 오인** — “제한 opt-in runtime” slice로 표시하고 전체 C11.4를 제외 범위에 둔다.
3. **체크박스 증가 암시** — 완료 수를 유지하며 roadmap 카드가 체크 상태를 바꾸지 않는다.
4. **PR 번호 오류** — 실제 merged implementation PR #952를 연결한다.
5. **Rebase merge 메타데이터 오류** — 실제 merge 시각과 merge SHA를 PR metadata와 대조했다.
6. **check 상태 과장** — GitHub check 미보고를 적고 CI pass라고 쓰지 않는다.
7. **실험을 제품 지원으로 표시** — fixture-only opt-in scope를 앞부분에 표시한다.
8. **scene 수 오류** — `none|uppercase|lowercase`와 min/max-content 조합이 여섯 장면이다.
9. **computed style 값 대신 ID 사용** — summary는 computed `text-transform` 값을 전달한 것으로 설명한다.
10. **style owner 범위 누락** — exact Grid owner 전제를 유지한다.
11. **언어 조건 누락** — direct `lang=en` 제약을 유지한다.
12. **임의 text 변환 주장** — fixed ASCII `IIII ii`만 layout buffer에서 변환한다고 밝힌다.
13. **source mutation 오인** — HostDocument 원문과 UTF-16 range가 보존된다고 적는다.
14. **Unicode casing 지원 오인** — Unicode/locale casing을 제외 범위에 둔다.
15. **inline run 지원 오인** — inline runs를 명시적으로 제외한다.
16. **일반 CSS admission 오인** — 일반 author CSS admission이 아님을 적는다.
17. **WPT 적합성 오인** — WPT를 완료하지 않았다고 적는다.
18. **하드웨어 GPU 결과 혼동** — AVD 결과를 hardware GPU로 확대하지 않는다.
19. **실기기 검증 혼동** — 시뮬레이터 결과를 실기기 결과로 확대하지 않는다.
20. **iOS compositor 확인 오인** — compositor 이후 pixels는 제외 범위다.
21. **screenshots를 glyph proof로 오인** — 화면은 runtime 상태이며 glyph pixel 검증이 아니라고 적는다.
22. **검사 수 산출 근거 누락** — Rust, FFI, Node, simulator 수를 실행 README와 대조했다.
23. **Android matrix 숫자 오류** — API 37 ARM64 AVD 6/6을 recorded matrix와 대조했다.
24. **iOS matrix 숫자 오류** — iPhone 17 Pro/iOS 26.2 Simulator 6/6을 recorded matrix와 대조했다.
25. **plan route 오타** — `/spinon/plan/c11-4-anonymous-grid-text-computed-text-transform.html`이 선택 build source와 일치한다.
26. **internal contract route 오타** — `/spinon/internal/0059-c11-grid-layout.html`을 사용한다.
27. **precomparison/runtime evidence route 오타** — 파일 경로와 확장자를 committed evidence에 대조했다.
28. **screen evidence route 오타** — Android/iOS representative PNG paths가 committed latest passing run을 가리킨다.
29. **review evidence route 누락** — PR diff와 merge-status review를 각각 별도 link로 연결한다.
30. **다른 roadmap card에 부수 영향** — 변경 diff는 text-transform paragraph 하나만 교체하고 주변 C11.4 카드들을 보존한다.
