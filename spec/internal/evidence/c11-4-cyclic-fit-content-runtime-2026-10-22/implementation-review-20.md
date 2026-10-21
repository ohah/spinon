# C11.4 cyclic percentage `fit-content()` 구현 실패 관점 검토 20개

계획 자체의 20개 검토와 별개로, 실제 Rust projection/layout, fixture dispatch, native harness, 최종 simulator 동작과 artifact 경계를 다시 확인했다. 캡처는 구현 commit `9af5ee8fa29269c589a02d6f1aafe3f72d53d1d0`에서 새로 수행했다.

1. **규범과 관측값 혼동:** 기대값을 Chrome 출력에서 역산하지 않는가? CSS Grid 2025 CRD와 Ahem 입력으로 독립 계산한 표를 계획에 고정했고 Chrome 156은 보조 관측으로 분리했다. PASS.
2. **cyclic column basis:** intrinsic root width 50px 뒤 최종 limit 25px이 되는가? Rust와 양 simulator의 Columns `[25]`, root `50×20`, item `25×20`이 일치한다. PASS.
3. **cyclic row basis:** width 20px에서 text block basis 40px를 보존한 뒤 final row 20px을 적용하는가? Rows `[20]`, root `20×40`, item `20×20`으로 확인했다. PASS.
4. **definite control:** 이미 definite인 40px root에서 50% limit이 20px인가? 두 simulator의 Columns `[20]`, root `40×20`, item `20×20`이 일치한다. PASS.
5. **typed projection 손실:** `fit-content(50%)`가 길이·auto로 평탄화되지 않는가? typed `FitContent(Percentage(0.5))` projection regression이 통과한다. PASS.
6. **intrinsic text font 의존:** 시스템 폰트 fallback이 Ahem 수치를 바꾸는가? 고정 Ahem bytes와 10px/10px direct-text paragraph를 사용한다. PASS.
7. **허용되지 않은 tree shape:** image·추가 child·anonymous grouping이 text intrinsic 결과를 대신하는가? 세 source는 root/item 두 div와 단일 direct text만 반환하고 node count가 2다. PASS.
8. **잘못된 style 축:** row scene이 columns의 `fit-content` 결과를 우연히 통과하는가? column/row track typed 값과 root sizing을 scene별로 검증한다. PASS.
9. **미등록 scene fail-closed:** 비슷한 이름이 고정 HTML 생성으로 이어지는가? FFI dispatcher는 세 ID만 허용하고 미등록 ID를 오류로 거부한다. PASS.
10. **validator looseness:** 새 scene 때문에 기존 text fixture의 `align-items:start` 계약이 느슨해지는가? `normal` 예외는 FitContentPercentage scene에만 적용되고 나머지는 `start` 검증을 유지한다. PASS.
11. **min-size 누락:** `min-width:0;min-height:0` 선언이 computed style 검사에서 사라져 auto minimum이 결과를 바꾸는가? 두 min-size를 명시 검증한다. PASS.
12. **dimension category 혼용:** `max-content`/`auto`를 임의 px로 받아들이는가? Px·Auto·MaxContent 세 expected variant만 각각 맞는 computed dimension에 대응한다. PASS.
13. **첫 pass 오판:** 여러 sizing pass 중 초기 값으로 final track을 판정하는가? 축별 마지막 pass를 고르는 direct layout assertion을 두어 column 25px을 확인한다. PASS.
14. **교차 축 누락:** 대상 축만 맞으면 다른 축의 잘못된 sizing도 통과하는가? 각 scene에서 Columns와 Rows를 모두 검사한다. PASS.
15. **frame/track 대체:** track 값만 맞고 root/item frame이 잘못되어도 통과하는가? 플랫폼 harness와 Rust test가 두 frame을 별도로 비교한다. PASS.
16. **Android dispatch·표본 ID:** AVD에서 신규 scene 또는 색 ID가 기본 expectation으로 흐르는가? 세 scene allowlist, root/item expectation과 각 경계 표본 ID/RGB를 추가해 PixelCopy 6/6 exact를 확인했다. PASS.
17. **iOS dispatch·표본 경계:** iOS가 다른 scene으로 fallback하거나 compositor 이후를 주장하는가? 요청 ID/frame이 맞고 Metal drawable 6/6 exact이며 compositor 제출 전임을 명시했다. PASS.
18. **draw 완료·재시도:** layout만 준비되고 present 전이거나 superseded retry를 숨기는가? 두 simulator의 각 3개 `result.json` 항목이 `drawPresented=true`, retry 0이다. PASS.
19. **fixture product 누출:** 신규 payload가 기본 `.rlib`에 들어가는가? 기본 release scan에는 없고 feature-positive control에만 존재한다. PASS.
20. **소스·범위 경계:** 캡처가 untracked source를 놓치거나 세 fixture를 일반 지원으로 과장하는가? metadata source commit이 clean implementation commit을 가리킨다. 실기기, WPT, glyph, compositor-after 및 전체 C11.4는 범위 밖으로 남겼다. PASS.

수정 중 발견한 실제 실패도 기록한다. 초기 Android scene dispatch는 runtime text case registry 누락으로 실패해 3개 고정 case를 추가했다. 신규 case의 computed `align-items:normal`과 `min-height:0`가 기존 validator 기대와 다른 문제를 scene 한정 규칙으로 분리했다. row inside-item 표본의 RGB mapping 누락을 추가했고, track unit test가 초기 pass를 읽던 문제는 마지막 축 pass를 선택하도록 수정했다. 각 실패 로그는 최종 근거 폴더에 보존했다.

최종 판정은 제한 fixture에서 PASS다. 이 검토는 PR 최종 diff의 별도 30관점 검토를 대체하지 않는다.
