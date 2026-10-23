# C13 코드·로드맵 동기화 PR diff 적대 검토 30개

각 관점은 실제 staged diff, 공식 checkbox, feature gate, 변경 전 Agent Browser 관찰을 대상으로 분리했다.

1. **공식 ID** — 수정 대상은 부모 C13과 그 계획이며 다른 roadmap ID를 변경하지 않는다.
2. **부모 상태** — C13 checkbox는 계속 unchecked다.
3. **집계** — 완료 count를 구성하는 checkbox 수나 상태를 변경하지 않는다.
4. **변경 전 표시** — Agent Browser의 C13 unchecked와 `72/213` 관찰 JSON을 함께 보존한다.
5. **변경 전 설명 누락** — live card가 C13.1만 요약하고 C13.2/C13.3을 누락한 상태를 캡처했다.
6. **C13.1 경계** — 기존 clip profile·AVD/iOS 근거를 C13 전체 지원으로 확대하지 않는다.
7. **C13.2 owner** — 추가 요약은 제한 scroll owner·offset snapshot을 말하며 CSSOM API로 표현하지 않는다.
8. **C13.2 root/body** — 전파와 lifecycle은 기존 대장 근거의 범위를 유지한다.
9. **C13.2 surface** — visible-overflow 표본 연결과 전체 overflow paint를 구분한다.
10. **ink overflow** — ink-only `box-shadow`는 fail-closed 잔여 결손으로 유지한다.
11. **C13.2 feature gate** — `c13-2-scroll-fixture-test`의 opt-in 상태를 코드로 확인했다.
12. **C13.3 style** — typed `touch-action`/`overscroll-behavior` 입력만 요약하고 일반 CSS admission을 말하지 않는다.
13. **C13.3 router** — delta routing은 public DOM event dispatch와 다르다고 구분한다.
14. **session lifetime** — pure Rust session/queue 존재를 public product adapter와 혼동하지 않는다.
15. **direct pan** — Android/iOS 증거를 제한 fixture 입력 결과로 표현한다.
16. **normalized wheel** — 합성 normalized delta fixture와 OS native wheel을 구분한다.
17. **Android native wheel** — API/fixture subset 근거만 유지한다.
18. **iOS native wheel** — callback이 부분 관측된 결과를 전체 입력 지원으로 일반화하지 않는다.
19. **fixture boundary** — `c13-3-gesture-fixture-test` 및 runtime `cfg`를 확인한다.
20. **product build** — 기본 앱의 scroll support가 완성됐다고 쓰지 않는다.
21. **CSSOM ownership** — `scrollTop`/`scrollTo`와 public `scroll` event는 완료로 표시하지 않는다.
22. **S04.10** — displayed-frame target input 경계를 미완료로 남긴다.
23. **C13.4** — scrollbar/gutter gate를 완료로 올리지 않는다.
24. **C13.5** — scroll snap gate를 완료로 올리지 않는다.
25. **C13.6** — logical overflow/RTL gate를 완료로 올리지 않는다.
26. **C12.5 dependency** — sticky positioning의 C13 선행 조건을 바꾸지 않는다.
27. **historical evidence** — 2026-10-15 C13.1 run section의 날짜와 사실을 보존하고 `현재`라는 오해 표현만 제거한다.
28. **C13.2 plan date** — 진행 중이라는 오래된 날짜 표기를 최신 제한 완료·미완료 경계로 바꾼다.
29. **plan links** — 새 상위 요약은 C13.2/C13.3의 기존 계획으로 이동하며 상대 경로가 유효하다.
30. **change class** — 코드·fixture·API·checkbox·계약 버전을 바꾸지 않는 문서 동기화로 한정한다.

**결과:** 30개 관점을 모두 대조했다. C13.2/3 구현 세부 검증과 roadmap 문구 갱신은 별도이며 C13 전체 완료 판정은 바뀌지 않는다.
