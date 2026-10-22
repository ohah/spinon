# PR #507 공개 로드맵 변경 적대 검토 20개

대상: spec/public/roadmap.html에 추가하는 PR #506 양축 span text runtime slice 요약과 이 검토 링크다. 각 항목은 별도 오류 경로 또는 계약 경계를 확인한다.

1. **순서 역전:** PR #503 뒤, 구현 단계 도식 앞에 추가해 2026-10-22 병합 순서를 유지한다.
2. **PR 미병합 표기:** 원본 PR API에서 MERGED와 병합 시각 2026-10-22T07:57:29Z를 확인했다.
3. **병합 커밋 오기:** PR의 실제 merge commit bd4e50157cf5496f6b44638dd8eee9bf82d1b3c2를 사용한다.
4. **검사 상태 과장:** PR status-check rollup이 비어 있음을 적고, 통과한 검사로 표현하지 않는다.
5. **필수 검사 오인:** 병합 당시 main에 필수 status check가 설정되지 않았다는 범위로 기록한다.
6. **scene 수 확대:** Android AVD와 iOS Simulator 각각 네 scene, 4/4로 한정한다.
7. **실기기 오표기:** Android 결과는 API 37 ARM64 AVD이고 실기기 결과로 부르지 않는다.
8. **iOS 대상 혼동:** iPhone 17 Pro / iOS 26.2 Simulator로 표기하고 실기기와 구분한다.
9. **min-content 값 전치:** Columns [4,4], Rows [29,29], root 14×64px, item 10×60px를 runtime README와 대조했다.
10. **max-content 값 전치:** Columns [29,29], Rows [4,4], root 64×14px, item 60×10px를 대조했다.
11. **입력 범위 과장:** 결과 설명은 explicit 2×2 track을 두 축으로 span하는 direct Ahem text 네 장면으로 한정한다.
12. **PixelCopy 표본 확대:** Android의 여섯 표본씩 네 scene, 24/24 exact 결과만 적는다.
13. **Metal readback 시점 누락:** iOS 24/24 표본은 compositor 제출 전 drawable readback으로 한정한다.
14. **제품 archive 오해:** 기본 static archive에는 새 fixture marker가 없고 opt-in fixture archive에는 포함된다고 구분한다.
15. **glyph raster 증거 혼동:** 앱 화면이 box geometry를 표시하며 glyph paint를 검증하지 않는다고 명시한다.
16. **border stroke 오해:** PixelCopy·drawable 점 표본을 CSS border stroke 검증으로 확대하지 않는다.
17. **전체 C11.4 완료 오인:** checkbox는 계속 미완료로 두고 slice를 전체 CSS Grid 지원으로 표현하지 않는다.
18. **완료 수 변경:** 공식 집계 77/213을 유지한다.
19. **미실행 적합성 주장:** 일반 CSS admission, WPT, Android 실기기·hardware GPU, iOS compositor 이후 pixel은 남은 범위로 표시한다.
20. **링크 경로 파손:** 계획, runtime README, 구현/runtime 검토, PR diff 검토, 이 문서, 두 simulator PNG, PR #506 및 공식 상태 경로를 대조한다.

근거는 [runtime README](./README.md), [계획](../../../../plan/c11-4-cyclic-minmax-calc-two-axis-spanning-text.md), [공식 상태](../../../STATUS.md), [PR #506](https://github.com/ohah/spinon/pull/506)와 병합된 main이다. 이 검토는 문서 변경을 대상으로 하며 런타임 재검증을 뜻하지 않는다.
