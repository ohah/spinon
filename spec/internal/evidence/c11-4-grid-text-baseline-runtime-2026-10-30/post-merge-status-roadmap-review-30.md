# C11.4 last-baseline post-merge status/roadmap review (30 checks)

검토 대상은 PR #830 병합 후 갱신한 `spec/STATUS.md`, `spec/public/roadmap.html`, 계획의 병합 상태다. 이 변경은 이미 병합된 제한 구현의 근거를 동기화하며 C11.4/C11 완료 판정은 바꾸지 않는다.

1. **PR 번호** — PASS. status, plan, roadmap 모두 구현 PR #830을 가리킨다.
2. **병합 방식** — PASS. GitHub가 보고한 Rebase and merge 결과와 맞는다.
3. **병합 시각** — PASS. `2026-10-30T01:12:10Z`를 기록했다.
4. **merge SHA** — PASS. `926bc4839aa72c8859ec0c5dfddf74675b7dfabb`로 통일했다.
5. **head 결과** — PASS. PR 최신 source head `fa4e4668a93a44e09263af344c6673e05ca8a65a`와 병합 내용을 대조했다.
6. **check rollup 표현** — PASS. PR rollup이 빈 배열임을 기록하고 CI 성공으로 말하지 않는다.
7. **required checks 표현** — PASS. `main`에 required status checks가 설정되지 않은 조회 결과와 일치한다.
8. **C11 checkbox** — PASS. 상위 C11 항목은 unchecked 상태다.
9. **C11.4 checkbox** — PASS. C11.4 intrinsic track sizing 전체도 unchecked 상태다.
10. **공식 완료 수** — PASS. 전체 공식 수 `72/213`을 그대로 둔다.
11. **후보와 완료 구분** — PASS. C11.4 내부의 제한 implementation/runtime slice로만 기록한다.
12. **fixture 범위** — PASS. opt-in fixture 경계를 적고 public CSS 지원으로 넓히지 않는다.
13. **기존 first-baseline 관계** — PASS. 기준 PR #827의 제한 first-baseline 결과를 이 후속 slice와 구분한다.
14. **multiline last-baseline 범위** — PASS. 두 고정 direct Ahem item과 다섯 last/control scene으로 제한한다.
15. **scene 총수** — PASS. first-baseline regression 4개와 신규 장면 5개, 총 9개로 기록한다.
16. **Android scene 결과** — PASS. API 37 ARM64 QEMU AVD에서 9/9다.
17. **iOS scene 결과** — PASS. iPhone 17 Pro/iOS 26.2 Simulator에서 9/9다.
18. **Android 표면 표본** — PASS. PixelCopy 27/27 exact다.
19. **iOS 표면 표본** — PASS. Metal drawable 27/27 exact다.
20. **iOS capture 시점** — PASS. compositor 제출 전 결과라고 제한한다.
21. **Android renderer 경계** — PASS. SwiftShader AVD를 실기기/hardware GPU와 혼동하지 않는다.
22. **text glyph 경계** — PASS. box surface evidence만으로 glyph paint를 증명하지 않는다.
23. **CSS admission** — PASS. ordinary projection의 first/last baseline 거부를 명시한다.
24. **WPT 경계** — PASS. Spinon WPT conformance를 주장하지 않는다.
25. **남은 플랫폼/표면 범위** — PASS. physical device와 compositor-after pixels를 미완료로 쓴다.
26. **focused tests 연결** — PASS. Rust, FFI, module-boundary 및 iOS build-flag counts가 runtime README에 있다.
27. **archive boundary 연결** — PASS. debug product/fixture positive control을 별도 evidence로 연결한다.
28. **roadmap 링크 경로** — PASS. 계획, Chrome 기준, 실행 README, 검토 문서와 screenshots의 `/spinon/` route를 고정했다.
29. **official status 링크** — PASS. roadmap card에서 `/spinon/STATUS.html`로 연결한다.
30. **raw/public roadmap 동기화** — PASS. `spec/public/roadmap.html`에 PR #830과 동일 merge SHA·범위·72/213을 기록했다. Tailnet served-copy 확인은 별도 publication evidence에서 수행한다.

모든 체크는 source diff, PR metadata 또는 repository evidence와 대조했다. 이 문서는 병합 상태를 요약할 뿐 기능을 추가로 검증하지 않는다.
