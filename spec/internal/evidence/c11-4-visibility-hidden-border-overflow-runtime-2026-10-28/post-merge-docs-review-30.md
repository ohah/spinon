# C11.4 visibility-hidden border overflow · 병합 후 문서·로드맵 검토 30개

검토 대상은 PR #747 병합 뒤 수정한 border-overflow 계획, mixed-track 상위 계획, C11 Grid track-sizing 현황 문서, 내부 계약 0059와 `spec/public/roadmap.html`이다. 공식 상태 대장 `spec/STATUS.md`는 수정하지 않았다. 근거는 PR #747 merge SHA `cb3c9a50b9a10d1d441cf15a584bacefcd91630d`와 [runtime evidence](./README.md), [18-scene matrix](./matrix-verification-18-scenes.json)다.

1. **계획 순서:** 계획 PR #746의 실제 Rebase and merge 시각과 SHA를 기록했다. 통과.
2. **구현 병합:** PR #747의 merge time과 merge SHA가 GitHub의 merged state와 일치한다. 통과.
3. **코드·문서 경계:** 변경은 opt-in fixture route의 검증으로 설명하며 공개 CSS admission으로 승격하지 않았다. 통과.
4. **공식 상태 대장:** C11.4 checkbox 및 다른 STATUS 행은 바꾸지 않았다. 통과.
5. **공식 집계:** 계획, 계약, 로드맵에 완료 수 `72/213`을 유지했다. 통과.
6. **내부 버전:** 계약 0059 숫자 버전은 `0.1.0`으로 유지했다. 통과.
7. **API/ABI 주장:** 새 public API/ABI를 추가했다고 서술하지 않는다. 통과.
8. **plan PR 상태:** 계획 PR #746은 merge SHA `a44ab6c079b35e91055afd66e9300a70c8de2918`와 함께 기록했다. 통과.
9. **PR status checks:** PR #747의 check rollup 공란을 CI 성공으로 표시하지 않았다. 통과.
10. **required checks:** main의 `required_status_checks: null`을 기록해 병합 정책과 CI 실행을 구분했다. 통과.
11. **Android scene 수:** 기존 14개와 신규 4개를 합친 18/18로 기록했다. 통과.
12. **iOS scene 수:** iOS도 18/18로 별도 서술했다. 통과.
13. **축 범위:** 신규 scene은 Columns와 Rows에 각각 hidden/visible control 두 개다. 통과.
14. **Root border box:** Column `100×120px`, row `120×100px`을 구분했다. 통과.
15. **Content box:** 2px border를 제외한 target-axis content `96px`을 root border box와 혼동하지 않는다. 통과.
16. **Track 기대값:** 신규 네 scene의 target tracks `[150,40,0]px`를 기록했다. 통과.
17. **Remaining space:** 음수 remaining `-114px`을 0으로 clamp해 기록하지 않았다. 통과.
18. **Hidden overflow 표본:** hidden scene의 root 밖 파란 A item sample을 설명했다. 통과.
19. **Visible control 표본:** 동일 root 밖 좌표의 visible control 보라 span sample을 설명했다. 통과.
20. **Border 표본:** 가려지지 않는 border sample `#f0c674`를 별도로 보존한다. 통과.
21. **Android surface 계층:** PixelCopy 표본이라고 명시하고 전체 화면 parity로 확대하지 않는다. 통과.
22. **iOS surface 계층:** Metal drawable readback은 compositor 제출 전이라고 명시한다. 통과.
23. **Android 환경:** API 37 ARM64 16 KB AVD로 기록하고 실기기로 부르지 않는다. 통과.
24. **iOS 환경:** iPhone 17 Pro/iOS 26.2 Simulator로 기록한다. 통과.
25. **Negative controls:** 독립 matrix의 17개 negative controls를 별도 결과로 적었다. 통과.
26. **Rust 검증 수:** runtime 4/4, FFI exact route 1/1, layout crate 308/308을 구분했다. 통과.
27. **정적 archive 한계:** macOS ARM64 static archive 비교를 APK/IPA 전체 exclusion으로 해석하지 않는다. 통과.
28. **이미지 경로:** 로드맵 링크는 Android/iOS의 실제 tracked PNG 경로를 가리킨다. 통과.
29. **계획 연쇄:** `display:none` 다음 계획으로 남아 있던 문장을 완료된 #734와 후속 #738/#747 기록으로 갱신했다. 통과.
30. **로드맵 문구:** PR #747 최신 항목을 추가하되 범용 지원·전체 C11.4·미검증 플랫폼을 주장하지 않는다. 통과.

## 남은 경계

이 동기화는 제한 fixture의 구현 기록만 보강한다. 실기기, Android hardware GPU, iOS compositor 이후 pixels, clipping/scrolling, 전체 WPT/전체 CSS Grid, 일반 CSS admission은 계속 미검증이다. C11.4 공식 상태와 `72/213` 집계는 바꾸지 않았다.
