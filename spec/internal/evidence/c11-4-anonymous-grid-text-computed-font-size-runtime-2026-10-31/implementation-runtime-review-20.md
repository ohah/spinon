# Implementation/runtime 실패 관점 검토 20개

범위: 현재 working-tree의 C11.4 anonymous Grid text computed `font-size` 10-scene opt-in candidate와 최종 Android/iOS Simulator 실행을 별도로 검토했다. 결과는 각각의 source, Rust tests, capture logs, matrix, simulator metadata 및 static archive scan과 대조했다. 계획 검토와 PR diff 검토는 별개다.

1. **Scene admission:** 새 ID는 `FONT_SIZE_SCENE_IDS` 10개로 제한되고 FFI generator는 다른 ID를 `None`으로 닫는다. `font-size-6-min-content` unit test가 거부된다.
2. **Scene/value 대응:** 5/7.5/10/12.5/20px 각 값은 min/max 장면 한 쌍과 대응한다. 중복 없는 ID 수 10 및 모든 scene 출력값을 검사했다.
3. **CSS serialization:** `5px`, `7.5px`, `10px`, `12.5px`, `20px`의 정확한 computed serialization만 수용한다. `7.50px`는 거부된다.
4. **Typed computed value:** CSS property와 `font_size_css_px`를 각각 검사해 7.5px property/7.5001 typed value의 불일치를 거부한다.
5. **허용 범위:** 0, 음수, `NaN`, infinity, 다른 fractional 값은 runtime validator test에서 거부된다. 이 경계는 해당 text model fixture에 한정한다.
6. **Unknown scene fail-closed:** 초기 검토에서 모르는 scene ID가 기본 10px로 해석될 수 있음을 발견했다. 부모 route가 먼저 allowlist하더라도 validator 자체도 거부하도록 수정했고 unknown-ID positive-property control을 추가해 통과했다.
7. **Known legacy scenes:** 기존 55 anonymous-text IDs, line-height IDs 및 full-tab generated IDs는 기존 10px 계약을 유지한다. 기존 anonymous text 및 C11.4 regression tests가 통과했다.
8. **Paragraph handoff:** anonymous `ParagraphStyle.font_size`는 scene에서 재계산하지 않고 검증된 typed computed value를 받는다. five-size unit test와 runtime trace가 일치한다.
9. **Fixed typography:** Ahem 400, normal style, LTR, horizontal-tb, collapse/wrap와 absolute 10px line-height는 그대로다. shared typography mutation test가 다른 `font-size`와 속성 변경을 거부한다.
10. **Text ownership/content:** runtime validator는 direct child count, parent ownership, text node kind와 정확한 `XX XX` source를 확인한다. FFI source test는 생성 HTML과 10개 scene 값을 확인한다. malformed native-runtime injection은 별도로 실행하지 않았다.
11. **Track contract:** 각 scene에서 `min-content`/`max-content` function을 그대로 유지하고 잘못된 반대 track 및 fixed row control을 Rust test에서 거부한다.
12. **Independent inline oracle:** pinned Ahem의 X/space advance를 사용해 min `2em`, max `5em`을 별도 Rust text engine으로 확인했다. 다섯 font size 모두 통과한다.
13. **Auto row:** fixed 10px line-height에서 min-content 두 줄 높이 20px, max-content 한 줄 높이 10px를 intrinsic API와 definite-width paragraph layout 양쪽에서 확인했다.
14. **Fractional CSS px:** 7.5px와 12.5px는 부동소수점 정밀도 허용오차 내에서 별도 보존된다. Android/iOS track matrix에 37.5px와 62.5px exact 결과가 있다.
15. **Runtime trace completeness:** capture runner는 중복·누락 chunk를 거부하고 실제 intrinsic source, line advance, final Columns/Rows stage, root/item frame 및 revision을 모든 scene에서 비교한다.
16. **Android integration:** API 37 ARM64 AVD에서 10/10 scene 통과, 설치 APK hash와 빌드 hash가 같다. AVD는 실기기/hardware GPU 증거가 아니다.
17. **iOS integration:** iPhone 17 Pro/iOS 26.2 Simulator에서 10/10 scene 통과, 설치 executable hash와 빌드 hash가 같다. compositor 이후 pixels는 확인하지 않았다.
18. **Feature boundary:** default `c04-runtime-gpu` static archive에는 신규 ID 0/10, `c04-runtime-gpu-fixture-test` positive control에는 10/10이 있다. 전체 APK/IPA 제외는 입증하지 않는다.
19. **Capture interpretation:** 양쪽 screenshot은 fixture route/status와 검은 renderer surface를 보여준다. glyph painting 또는 pixel equality 증거로 사용하지 않으며 PR 이미지로도 올리지 않는다.
20. **Scope/status:** 일반 CSS admission, WPT, 다른 font/script, 물리 기기와 전체 C11.4/C11 완료를 추론하지 않는다. C11.4/C11 checkbox와 공식 `72/213`은 그대로 유지한다.

추가 검증 중 wrong-track assertion의 첫 컴파일은 테스트 helper를 가린 지역 변수 때문에 실패했다. 변수 이름을 고쳐 최종 Rust tests와 Clippy를 다시 통과시켰고 최초 로그는 `builds/runtime-computed-font-size-test-initial-failure.log`에 보존했다. 다른 unresolved finding은 없으며, malformed DOM을 앱에 주입하는 runtime negative matrix는 이 고정-scene harness에 없다는 한계를 계속 명시한다.
