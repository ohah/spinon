# C11.4 percentage `row-gap` staged PR diff review 30

**검토 기준:** 2026-11-07 staged diff, candidate `feat/c11-4-root-html-auto-row-percentage-gap`, base `8a1917ea6465d79174a4ebd0b974de701f3860f9`. 구현/runtime 사전 검토 20개와 별도로 코드·명세·근거를 hostile review했다.

1. **모드 분기:** 신규 profile 검증은 `RootHtmlAutoRowStretchV1`에만 적용된다. 다른 intrinsic track profile은 기존 helper 경로를 유지한다. **통과.**
2. **percentage basis:** Grid content box width/height가 정확히 40×40px인 경우만 허용한다. 임의 viewport basis를 사용하지 않는다. **통과.**
3. **explicit columns:** template columns는 단일 40px track이어야 한다. **통과.**
4. **explicit rows:** template rows는 단일 10px track이어야 한다. **통과.**
5. **implicit row mode:** implicit rows/columns는 `auto`이고 auto flow는 row여야 한다. **통과.**
6. **template shape:** auto-repeat와 template areas를 거부한다. **통과.**
7. **visible item 범위:** visible direct items는 2~3개만 허용한다. **통과.**
8. **placement:** 모든 visible direct item이 auto-placement여야 한다. **통과.**
9. **visibility:** Layout에서 hidden subtree를 제외한 direct child count를 사용하고 Style/Layout profile gate를 모두 거친다. **통과.**
10. **overflow geometry:** item frame을 Grid root 경계에 clamp하는 신규 코드가 없고 Taffy의 기존 overflow 경로를 쓴다. **통과.**
11. **fixed px 회귀:** 기존 `row-gap:0..=7.5px` 허용 범위는 별도 match arm으로 보존한다. **통과.**
12. **percentage 유효성:** finite non-negative percentage만 허용한다. negative, NaN, infinity 대조가 projection과 Layout helper에 있다. **통과.**
13. **used value 계산:** percentage fraction은 40px block basis에 곱해지고 typed percentage 자체는 px로 바꾸지 않는다. **통과.**
14. **f32 extent 보호:** 두 gutters와 30px base track extent의 전체 합이 finite `f32` 범위인지 검증한다. 최대 허용 fraction과 다음 f32 fraction을 경계 테스트한다. **통과.**
15. **`normal`:** Grid `row-gap:normal`은 used zero gap으로 projection되고 RuntimeSession이 15px/15px auto rows를 확인한다. **통과.**
16. **CSS math:** `calc(5px)`는 제한 profile에서 거부되며 completed layout을 publish하지 않고 이전 environment 복귀 후 회복한다. **통과.**
17. **column axis:** `column-gap:0`만 허용하고 percentage column-gap은 runtime negative control로 거부한다. **통과.**
18. **alignment:** 기존 10개 `align-content` allowlist만 허용하며 safe/unsafe와 baseline 값은 닫혀 있다. **통과.**
19. **상호작용 style:** `align-items:normal`, `justify-content:normal`, `justify-items:normal`, static position, visible overflow, zero margin/padding/border 검사를 유지한다. **통과.**
20. **diagnostic attribution:** projection에서 unsupported gap은 Grid node의 `row-gap` property로 귀속한다. Layout validator도 `row-gap`을 표시한다. **통과.**
21. **Taffy 진입 전 validation:** direct typed Layout 입력의 잘못된 basis/shape는 `layout_profile_is_supported`가 거부한 뒤 Taffy로 진행하지 않는다. `height=2e38`, 200% 회귀가 있다. **통과.**
22. **기존 intrinsic rows:** 기존 single/two auto-row, fixed-gap geometry 및 auto-placement tests를 유지하고 회귀를 실행했다. **통과.**
23. **Layout geometry matrix:** 7 percentages × 10 alignment values, 70 scene에서 item rect와 auto-row track sizing trace를 독립 expected JSON과 비교한다. **통과.**
24. **RuntimeSession matrix:** 실제 parsed Root HTML과 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 같은 70 scene의 painted item frame을 비교한다. **통과.**
25. **oracle 독립성:** Chrome 157 관측과 독립 계산 expected values를 별도 파일로 보존하며 관측 좌표 최대 편차 1/32 CSS px 이하다. **통과.**
26. **fixture 오염:** 제품 feature flag는 simulator 기본 app route에서 끄고 C04 runtime GPU만 켠다. bootstrap 원본을 pass별로 보존·복원한다. **통과.**
27. **기존 산출물 보존:** simulator harness는 설치 APK/executable 및 build outputs의 원래 hash를 저장·복원하며 split Android install과 실행 중 앱은 교체하지 않고 중단한다. **통과.**
28. **screenshot 주장 범위:** pass04 Android/iOS OS PNG의 색 픽셀 bounds에서 orange item이 Grid root 아래 시작한다. PixelCopy/Metal drawable readback, real device, hardware GPU, iOS compositor parity, WPT로 확대 주장하지 않는다. **통과.**
29. **parser limitation:** authored `row-gap:-5%`는 현재 fatal cascade diagnostic임을 테스트·명세에 적고 browser invalid-declaration recovery parity를 주장하지 않는다. **범위 한계 기록.**
30. **완료 수와 명세:** internal contract는 후보 범위와 제한을 추가하되 version `0.1.0`, 공식 `78/214`, C11.4/C11 미완료 상태를 유지한다. `spec/STATUS.md`에 미완성 slice를 완료로 표시하지 않았다. **통과.**

## 결과

Blocking finding 없음. `git diff --cached --check` 및 worktree `git diff --check` 통과. 이 결과는 코드 reviewer의 외부 승인을 대신하지 않으며 CI·merge 상태는 PR 후 별도 확인한다.
