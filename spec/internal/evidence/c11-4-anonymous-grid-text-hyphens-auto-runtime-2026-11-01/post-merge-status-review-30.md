# PR #982 병합 후 상태 대장 동기화 검토 30개

PR #982가 main에 병합된 뒤 `spec/STATUS.md`에 제한 slice의 병합 이력과 근거 링크를 추가하는 문서 변경을 검토했다. 이 기록은 기능 구현/PR diff 검토와 별개이며, C11.4/C11 완료 표시를 변경하지 않는다.

1. **항목 위치가 맞는가? — 통과.** 새 이력은 `C11.4 intrinsic track sizing` 하위에 둔다.
2. **C11.4 checkbox가 유지되는가? — 통과.** `- [ ]`를 그대로 둔다.
3. **상위 C11 checkbox가 유지되는가? — 통과.** C11은 `- [ ]` 상태다.
4. **공식 완료 수가 바뀌는가? — 통과.** `72/213`을 유지한다.
5. **미완료 slice가 완료로 표현되는가? — 통과.** 제한 opt-in slice로 부르고 전체 C11.4/C11 지원을 명시적으로 제외한다.
6. **PR 번호가 실제 병합 PR과 맞는가? — 통과.** 기록은 #982를 가리킨다.
7. **merge method가 저장소 규칙과 맞는가? — 통과.** Rebase and merge로 기록한다.
8. **merge timestamp가 실제 값과 맞는가? — 통과.** GitHub 반환값 `2026-11-01T10:38:00Z`를 사용한다.
9. **merge SHA가 실제 main commit인가? — 통과.** `b54d4853fe7cd5c70393c7083eea2c3d8f6f223f`를 PR merge metadata와 대조했다.
10. **고정 locale이 과장되는가? — 통과.** exact `lang=en`만 표기하고 일반 언어 지원은 미검증으로 둔다.
11. **고정 단어 범위가 빠지거나 늘었는가? — 통과.** 세 allowlist 단어만 한정 지원으로 표기한다.
12. **font/style 조건이 누락됐는가? — 통과.** Ahem 10px 고정 profile 및 fixture-only 범위를 유지한다.
13. **dependency version이 근거와 맞는가? — 통과.** Hypher 0.1.8 English feature를 기록한다.
14. **feature admission이 제품 CSS로 오해되는가? — 통과.** opt-in fixture feature라고 적는다.
15. **Android target이 정확한가? — 통과.** API 37 ARM64 AVD로 표기한다.
16. **iOS target이 정확한가? — 통과.** iPhone 17 Pro/iOS 26.2 Simulator로 표기한다.
17. **scene 수가 저장된 matrix와 맞는가? — 통과.** 각 플랫폼 30/30을 기록한다.
18. **설치 artifact 검사를 생략했는가? — 통과.** status summary는 evidence의 installed/build hash 대조를 참조한다.
19. **text-layout test 수가 맞는가? — 통과.** 96 passed를 evidence와 대조한다.
20. **style cascade/CSS adapter 수가 맞는가? — 통과.** 7 passed를 evidence와 대조한다.
21. **runtime 결과의 ignored 수가 숨겨졌는가? — 통과.** 394 passed와 3 ignored를 함께 적는다.
22. **FFI 결과의 ignored 수가 숨겨졌는가? — 통과.** 188 passed와 2 ignored를 함께 적는다.
23. **fixture boundary 결과가 최신값과 맞는가? — 통과.** 32/32를 기록한다.
24. **Clippy/format 결과가 실행 범위를 넘는가? — 통과.** fixture-enabled FFI Clippy와 rustfmt만 기록한다.
25. **archive 결과가 mobile app 전체를 보증하는가? — 통과.** macOS ARM64 static archive로 한정한다.
26. **CI 상태가 빈 rollup인데 성공으로 기록되는가? — 통과.** GitHub check rollup이 비어 있고 CI 성공으로 세지 않는다고 명시한다.
27. **WPT 또는 공개 CSS 적합성으로 오해되는가? — 통과.** 두 범위 모두 미검증으로 남긴다.
28. **screenshot/paint 또는 기기 결과가 과장되는가? — 통과.** glyph pixels, 실기기와 hardware GPU를 미검증으로 둔다.
29. **계획·계약·evidence·review 상대 링크가 맞는가? — 통과.** `spec/STATUS.md` 기준 상대 경로의 각 파일이 존재한다.
30. **status sync 자체가 계획/구현 상태의 역사와 어긋나는가? — 통과.** 이 항목은 merge 이력만 추가하고 계약 version `0.1.0` 및 미완료 판정을 유지한다.

결함은 발견하지 못했다. 확인한 결론은 PR #982의 병합 기록을 C11.4 이력에 연결하는 것이며 완료 수 증가는 아니다. Tailscale 정적 미리보기는 이 문서 PR 병합 후 별도 served-content/hash 확인으로 동기화한다.
