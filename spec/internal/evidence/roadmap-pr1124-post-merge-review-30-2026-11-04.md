# PR #1124 병합 후 상태·로드맵 변경 적대 검토 30개

**검토 대상:** PR #1124 병합 결과를 공식 상태 대장, 내부 계약 0059, C11 overview plan, item-alignment plan에 반영한 문서 변경. 코드·테스트·공식 지원 범위는 변경하지 않는다. 구현 PR의 기존 30개 변경 검토와는 별개의 문서 diff 검토다.

1. **병합 식별자:** GitHub PR 상태의 merge commit `e11df25732ef1a2c8365287b8ef475b55f6aa408`과 문서의 SHA가 일치한다 — 통과.
2. **병합 시각:** GitHub의 `2026-11-03T15:49:14Z`와 plan·STATUS 기록이 일치한다 — 통과.
3. **병합 방식:** 기록은 프로젝트 규칙의 `Rebase and merge`를 사용하며 squash 또는 merge commit을 주장하지 않는다 — 통과.
4. **PR 식별 링크:** 네 문서 모두 구현 PR #1124를 가리키며 다른 C11 PR 번호로 바뀌지 않았다 — 통과.
5. **공식 대장 과거형:** `spec/STATUS.md`의 항목은 이제 열린 candidate라고 부르지 않는다 — 통과.
6. **작업 계획 상태:** item-alignment plan은 PR이 열려 있거나 merge 대기라고 서술하지 않는다 — 통과.
7. **계약 상태:** 내부 계약 0059는 PR #1124를 열린 PR로 표시하지 않는다 — 통과.
8. **Tailnet 진행 상태:** item-alignment plan은 로드맵 preview sync를 완료로 오인하지 않고 다음 미완료 단계로 둔다 — 통과.
9. **C11 상위 checkbox:** 변경 전후 `spec/STATUS.md`의 C11 상위 미완료 상태가 같다 — 통과.
10. **C11.1 상위 범위:** bounded alignment slice를 C11.1 완료로 승격하지 않는다 — 통과.
11. **공식 집계:** 변경 문서는 공식 완료 `72/213`을 유지한다 — 통과.
12. **내부 계약 버전:** 변경 문서는 출시 전 내부 숫자 버전 `0.1.0`을 유지한다 — 통과.
13. **기능 범위:** item alignment가 기존 eligible fixed-track Root HTML Grid에 한정된 설명을 유지한다 — 통과.
14. **속성 범위:** `align-items`, `justify-items`, `place-items`만 이 slice의 추가 대상으로 기재한다 — 통과.
15. **일반 CSS 주장:** 문서가 일반 author CSS 또는 전체 CSS Grid 지원을 주장하지 않는다 — 통과.
16. **WPT 주장:** Chrome 비교나 로컬 검증을 Spinon WPT 적합성으로 표현하지 않는다 — 통과.
17. **CI 주장:** PR check rollup이 비어 있다는 사실을 CI 통과로 바꾸지 않는다 — 통과.
18. **필수 체크 주장:** main required status checks가 설정되지 않았다는 한계를 그대로 적는다 — 통과.
19. **테스트 근거 링크:** Root HTML runtime evidence README의 상대 경로가 존재한다 — 통과.
20. **사전 비교 링크:** Chrome 156 관찰·독립 기대값 precomparison README가 연결되어 있다 — 통과.
21. **구현 검토 링크:** 구현/runtime 실패 관점 20개 파일이 연결되어 있다 — 통과.
22. **PR 검토 링크:** 구현 PR의 별도 diff review 30개 파일이 연결되어 있다 — 통과.
23. **계획 링크:** 전용 item-alignment plan의 상대 경로가 유효하다 — 통과.
24. **Android 범위:** Android 문구는 API 37 AVD/SwiftShader의 앱 실행 관찰을 유지하고 실기기로 확대하지 않는다 — 통과.
25. **iOS 범위:** iPhone 17 Pro/iOS 26.2 Simulator 관찰을 유지하고 iOS 실기기로 확대하지 않는다 — 통과.
26. **화면 근거 한계:** 화면 캡처를 visual smoke로 설명하며 PixelCopy·Metal readback으로 부르지 않는다 — 통과.
27. **후보 결과 일치:** STATUS와 내부 계약의 runtime 결과 요약은 `layout=ready boxes=83`, draw/present로 일관된다 — 통과.
28. **overview 동기화:** 상위 C11 plan은 #1124 merge 결과를 포함하며 동일한 미완료 경계를 쓴다 — 통과.
29. **변경 범위:** 이 후속 변경은 상태·계획 문서와 이 검토 기록으로 제한되고 제품 소스는 바꾸지 않는다 — 통과.
30. **diff 무결성:** 최종 staged diff에서 whitespace 오류와 의도하지 않은 파일 변경이 없어야 한다 — 통과 (`git diff --check`, staged path inventory 확인).

이 문서 변경은 병합 사실과 roadmap 기록을 맞춘다. C11/C11.1 완료, 공식 완료 수 증가, CI 통과, PixelCopy/Metal readback 또는 전체 CSS 지원을 뜻하지 않는다.
