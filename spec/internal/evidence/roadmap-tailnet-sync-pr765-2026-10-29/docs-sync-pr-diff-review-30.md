# PR #765 병합 후 로드맵·문서 동기화 PR diff 검토 30개

검토 대상은 PR #765 병합 결과를 반영한 `spec/STATUS.md`, `spec/0001-conformance.md`, 내부 계약 0059, C11.4 계획·runtime 근거, 공개 `roadmap.html`과 이 검토 문서다. 2026-10-29 GitHub 상태를 다시 조회하고 RSPress 2.0.22 선택 문서 빌드를 실행했다.

1. **PR 식별:** #765가 C11.4 single-track overflow automatic minimum runtime slice인지 PR 제목과 링크를 확인했다 — 통과.
2. **#765 병합 시각:** GitHub의 `2026-10-28T16:52:08Z`가 상태 대장·계획·로드맵의 기록과 일치한다 — 통과.
3. **#765 merge SHA:** `31103049d4a99a70fb97d3dfc58c96ac67cb173c`가 GitHub와 모든 갱신 문서에서 일치한다 — 통과.
4. **#764 병합 시각:** GitHub의 `2026-10-28T14:30:20Z`가 기존 fit-content 기록과 일치한다 — 통과.
5. **#764 merge SHA:** `db9b51c6ae72328c408b2a0ed15d602a90938d47`를 계획·로드맵과 대조했다 — 통과.
6. **check run 유무:** #764·#765 head와 merge SHA 네 곳의 check-run 수가 모두 0이다 — 통과.
7. **status context 해석:** 네 commit의 status context 수는 0이며 API의 기본 `pending` 상태를 실행 중인 CI로 해석하지 않는다 — 통과.
8. **필수 check 정책:** main의 required-status-check 설정이 비활성화되어 있음을 GitHub API에서 확인했다 — 통과.
9. **CI 표현:** 어느 문서도 빈 check 결과를 CI 성공으로 주장하지 않는다 — 통과.
10. **공식 완료 수:** 상태 대장과 공개 로드맵 모두 `72/213`을 유지한다 — 통과.
11. **상위 C11.4 상태:** 제한 runtime slice 병합을 상위 C11.4 완료로 승격하지 않았다 — 통과.
12. **Android 환경명:** `API 37 ARM64 AVD`로 기록해 Android 실기기로 오인되지 않는다 — 통과.
13. **Android 그래픽 범위:** emulator/SwiftShader와 PixelCopy 근거를 hardware GPU나 실기기 결과로 확대하지 않는다 — 통과.
14. **iOS 환경명:** `iPhone 17 Pro / iOS 26.2 Simulator`로 기록한다 — 통과.
15. **iOS 픽셀 경계:** Metal drawable 표본이 OS compositor 제출 전임을 명시한다 — 통과.
16. **행렬 분모:** 각 플랫폼의 고정 14-scene 결과를 14/14로 기록하며 전체 C11.4 분모와 섞지 않는다 — 통과.
17. **입력 축:** Columns Ahem text와 Rows natural 10×10 PNG 입력을 구별한다 — 통과.
18. **overflow 집합:** visible/clip/hidden/auto/scroll 결과를 적되 실제 clipping·scroll 동작은 범위 밖으로 남겼다 — 통과.
19. **minimum controls:** item minimum과 track minimum control을 automatic minimum 결과와 별도로 설명한다 — 통과.
20. **fixture 격리:** 결과를 opt-in fixture-test profile에 한정하고 일반 제품 경로로 일반화하지 않는다 — 통과.
21. **CSS admission:** 일반 author CSS admission이 추가됐다고 쓰지 않는다 — 통과.
22. **WPT 범위:** 고정 fixture 결과를 전체 WPT 통과로 표현하지 않는다 — 통과.
23. **공개 API/ABI:** 이번 문서 갱신으로 공개 API 또는 ABI가 변경된 것처럼 표현하지 않는다 — 통과.
24. **계약 버전:** 내부 계약 0059의 `0.1.0`을 유지한다 — 통과.
25. **상태 대장 연결:** #765 상태 행에서 계획과 실행 근거로 가는 링크의 상대 경로를 확인했다 — 통과.
26. **conformance 연결:** conformance 요약의 #765 merge 정보가 계획 및 README와 일치한다 — 통과.
27. **계획 일관성:** 이전의 “다음 동기화에서 PR #764를 기록” 문구를 제거하고 실제 #765 병합 결과로 갱신했다 — 통과.
28. **로드맵 순서:** #764 fit-content slice가 #765 overflow slice보다 앞에 표시되고 둘 다 미완료 상위 상태를 유지한다 — 통과.
29. **화면 링크 출처:** Android·iOS 화면 링크가 움직이는 branch 대신 #765 merge SHA에 고정되어 있다 — 통과.
30. **선택 페이지 빌드:** RSPress 2.0.22 선택 빌드가 `0001-conformance`, `STATUS`, Grid 계획·계약과 두 runtime 근거 페이지를 생성했다 — 통과. 전체 문서의 기존 dead-link 검사는 별도 전체 빌드에서 확인하지 않았다.

## 검토 결과

이 diff에서 차단 결함을 찾지 못했다. C11.4는 미완료이고 공식 `72/213`은 그대로다. 빌드 결과는 선택한 문서 경로만 검증하며 저장소 전체의 dead link가 없다는 증거는 아니다.
