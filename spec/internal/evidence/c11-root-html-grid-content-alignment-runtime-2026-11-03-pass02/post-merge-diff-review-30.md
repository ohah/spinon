# C11 콘텐츠 정렬 병합 후 문서 변경 검토 30개

PR #1121 병합 뒤 internal contract와 plan의 시점 정보를 동기화하는 작은 문서 변경을 검토했다. 이 변경은 공식 완료 상태를 승격하지 않는다.

1. **잘못된 PR 번호:** 내부 계약 링크와 계획 링크가 실제 PR #1121을 가리킨다.
2. **병합 시각 오기:** 기록한 `2026-11-03T14:29:31Z`는 GitHub PR 메타데이터와 일치한다.
3. **merge SHA 오기:** `a5ed70206342d35a99a19e1297772e08d1bdaa1e`를 PR `mergeCommit`과 대조했다.
4. **잘못된 병합 전략:** PR 기록이 Rebase and merge인 것을 확인했다.
5. **CI 과장:** 빈 PR check rollup을 통과로 표시하지 않는다.
6. **승인 과장:** 승인 리뷰가 없는 것을 CI와 혼동해 쓰지 않는다.
7. **C11 parent 승격:** official C11 unchecked 상태를 그대로 둔다.
8. **C11.1 완료 오표기:** 이 slice가 parent 완료가 아니라고 다시 명시한다.
9. **공식 수 변경:** `72/213`을 바꾸지 않는다.
10. **내부 버전 변경:** 출시 전 계약 `0.1.0`을 유지한다.
11. **public CSS 과장:** scoped candidate와 공개 지원을 구분한다.
12. **WPT 과장:** 전체 WPT 적합성을 주장하지 않는다.
13. **시뮬레이터 결과 범위:** Android와 iOS 문구가 visual smoke로 유지된다.
14. **surface readback 주장:** PixelCopy와 Metal readback을 완료로 적지 않는다.
15. **Android 실기기 혼동:** API 37 ARM64 AVD라는 기존 표현을 보존한다.
16. **iOS 실기기 혼동:** iPhone 모델명이 Simulator임을 계속 표시한다.
17. **실패 이력 손실:** 초기 cascade/AVD 실패 설명과 보존 경로를 유지한다.
18. **앱 artifact 혼동:** build hash와 simulator 실행 결과의 역할을 섞지 않는다.
19. **internal contract 링크 깨짐:** evidence 링크 상대 경로가 기존 경로와 같다.
20. **계획 문서 상태 불일치:** “PR 전” 상태를 제거하고 실제 merge link로 교체한다.
21. **계획 scope 확대:** CSS profile의 eligibility를 변경하지 않는다.
22. **명세 지원 값 변경:** allowlist를 변경하지 않는다.
23. **런타임 동작 변경:** 문서 전용 변경이며 Rust 소스는 수정하지 않는다.
24. **status ledger 수정:** `spec/STATUS.md`를 건드리지 않는다.
25. **공개 roadmap 변경:** official progress가 그대로이므로 공개 roadmap 파일은 건드리지 않는다.
26. **화면 링크 훼손:** simulator screenshot 경로는 유지한다.
27. **Chrome 근거 훼손:** Agent Browser 비교 링크는 변경하지 않는다.
28. **기존 WIP 손상:** 이 브랜치에 stage할 파일은 세 문서뿐이다.
29. **불필요한 테스트 주장:** 문서 metadata 수정에 실행하지 않은 검증을 추가하지 않는다.
30. **merge metadata를 완료 근거로 확대:** merge SHA는 소스 반영 사실만 가리키며 official C11 완료의 증거가 아니다.

변경은 문서 두 곳과 이 검토 기록으로 제한한다. 전체 PR #1121 runtime·platform evidence는 해당 README에 남아 있다.
