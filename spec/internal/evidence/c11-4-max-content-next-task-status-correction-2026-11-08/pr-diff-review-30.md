# C11.4 max-content 상태 정정 PR diff 검토 30개

검토 대상은 `plan/c11-4-root-html-intrinsic-auto-rows-max-content.md`, `plan/c11-4-grid-track-sizing.md` 및 이 기록이다. 이전 PR #1282의 다음 후보 문구를 바로잡으며 GitHub PR metadata와 `spec/STATUS.md`를 대조했다.

1. **PASS — 대상 PR:** max-content 구현은 정확한 PR #1264로 연결한다.
2. **PASS — 병합 시각:** `2026-11-07T11:56:35Z`가 GitHub metadata와 일치한다.
3. **PASS — merge SHA:** `5389aae4981bb6dccc7083b554c446f055a72708`가 metadata 및 공식 대장에 일치한다.
4. **PASS — 병합 방식:** `Rebase and merge`로 기록한다.
5. **PASS — 구현 경로:** singleton 구현을 `RuntimeRootHtmlTextPaintV1`/`RootHtmlIntrinsicTrackSizingV1` profile로 한정한다.
6. **PASS — projection 상태:** typed `MaxContent` projection이 이미 구현된 사실을 기록한다.
7. **PASS — layout 상태:** max-content layout admission이 PR #1264에서 구현됐다.
8. **PASS — runtime 상태:** default RuntimeSession 구현이 이미 완료된 기록을 대조한다.
9. **PASS — Layout 수치:** 17/17은 PR #1264 본문과 일치한다.
10. **PASS — projection 수치:** style-to-layout 25/25는 PR #1264 본문과 일치한다.
11. **PASS — 신규 runtime 수치:** max-content RuntimeSession 3/3을 기록한다.
12. **PASS — 회귀 수치:** 기존 intrinsic-row RuntimeSession 7/7과 신규 3/3을 구분한다.
13. **PASS — 정적 검사:** workspace Clippy와 rustfmt 기록을 보존한다.
14. **PASS — Android 기기 경계:** API 37 ARM64 AVD를 실기기라고 부르지 않는다.
15. **PASS — iOS 기기 경계:** iPhone 17 Pro/iOS 26.2 Simulator를 실기기라고 부르지 않는다.
16. **PASS — box 수:** 두 simulator의 7-box 결과는 공식 기록과 PR 본문에 일치한다.
17. **PASS — screenshot 한계:** 화면 증거를 visual smoke로만 분류한다.
18. **PASS — pixel readback:** PixelCopy/Metal readback이나 compositor 이후 parity를 주장하지 않는다.
19. **PASS — profile 분리:** PR #1280의 four-member `RootHtmlFitContentAutoRowsV1`을 singleton #1264와 분리한다.
20. **PASS — PR #1282 정정:** #1282에서 다음 후보로 잘못 반복된 singleton max-content를 제거한다.
21. **PASS — 다음 작업 상태:** PR #1280 이후 새 bounded candidate는 아직 선택되지 않았다고 표시한다.
22. **PASS — 미완료 범위:** C11.4 전체 sizing matrix를 max-content slice 하나로 완료 처리하지 않는다.
23. **PASS — CSS admission 경계:** 일반 authored CSS admission을 구현 완료로 주장하지 않는다.
24. **PASS — WPT 경계:** 전체 WPT 적합성 주장을 추가하지 않는다.
25. **PASS — stage 범위:** Grid §11.3–§11.8의 전체 sizing/contribution matrix가 남아 있음을 유지한다.
26. **PASS — 공식 C11.4 체크:** 상위 C11.4를 완료로 바꾸지 않는다.
27. **PASS — 공식 C11 체크:** 상위 C11을 완료로 바꾸지 않는다.
28. **PASS — 전체 집계:** 공식 완료 수 `78/214`를 변경하지 않는다.
29. **PASS — 중복 상태 방지:** PR #1264가 이미 공식 `spec/STATUS.md`에 기록돼 있으므로 중복 행을 추가하지 않는다.
30. **PASS — 외부 효과 경계:** 코드·API·simulator 실행·Tailnet 배포를 이 문서 정정의 결과로 주장하지 않는다.

## 남은 범위

이 검토는 문서 정정만 확인한다. 새로운 C11.4 구현 후보를 선택하거나 구현하지 않았고 제품 동작 검증도 하지 않았다.
