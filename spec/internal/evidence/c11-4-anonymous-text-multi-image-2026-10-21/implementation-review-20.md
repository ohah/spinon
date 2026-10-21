# C11.4 anonymous text + multiple spanning image 구현 실패 관점 검토 20개

검토 범위는 fixture adapter, runtime routing, FFI scene generator, Android/iOS verifier, simulator capture 도구, 실행 결과다. 계획 검토와 별도로 실제 구현·실행에 나타날 실패 경로를 확인했다.

1. **기본 제품 artifact로 fixture가 새어 나오는가?** Runtime adapter/test는 `c11-4-grid-track-sizing-fixture-test`로, FFI scenes는 상위 `c04-runtime-gpu-fixture-test`로 묶인다. 기본 `spinon-ffi` archive에서 새 scene ID가 없고 opt-in positive control에는 있어 통과했다.
2. **잘못된 root나 HTML namespace를 fixture로 오인하는가?** 제한 adapter는 고정 profile, HTML namespace, `div`, document root parent를 모두 검사한다. 다른 root는 선택되지 않아 통과했다.
3. **다른 runtime calculation profile에 영향을 주는가?** C11.4 fixture 준비와 계산 분기는 Block/Flex/Grid track-sizing profile에서만 열린다. 제품의 다른 layout profile은 바뀌지 않아 통과했다.
4. **알 수 없는 유사 scene ID가 조용히 계산되는가?** 알려진 prefix지만 allowlist 밖인 scene은 `unsupported` 오류를 반환하고, 다른 작업 ID는 기존 경로로 넘긴다. 잘못된 C11.4 입력을 fail closed해 통과했다.
5. **중첩·추가·누락 child를 direct item처럼 처리하는가?** root의 직접 자식 개수와 각 부모 관계를 확인하고, image descendant를 허용하지 않는다. 고정 topology 외 입력은 거부되어 통과했다.
6. **text item이 여러 run·다른 문자열로 합쳐져 결과를 오염시키는가?** 정확히 하나의 direct text node와 `XX`만 받으며 diagnostics에서 anchor, source node, 순서, Ahem metrics를 검사한다. 통과했다.
7. **다른 image나 네트워크 자원을 고정 PNG처럼 다루는가?** 각 이미지의 HTML `img`, 고유 ID, 고정 source URL을 검사한 뒤 고정 PNG bytes로 decode한다. 다른 source의 임의 크기 대체를 차단해 통과했다.
8. **DOM order, source order, grid placement 순서가 불일치하는가?** 네 scene의 FFI source, Rust expectation, Android/iOS 입력 order가 forward/reversed DOM을 각각 명시한다. simulator diagnostics와 frame이 일치해 통과했다.
9. **일부 PNG decode 실패 후 부분 intrinsic snapshot을 게시하는가?** 모든 요청·decode 결과를 `Result<Vec<_>>`로 수집한 뒤 한 snapshot을 만들므로 하나라도 실패하면 layout 경로가 중단된다. 부분 성공 게시가 없어 통과했다.
10. **natural size나 aspect ratio 변형이 잘못된 track 계산을 통과하는가?** 모든 decoded PNG가 24×16인지 확인하고 Android/iOS가 natural size 및 null style aspect ratio를 검사한다. height 16/32/48에서 24/48/72 CSS px intrinsic width를 반영해 통과했다.
11. **이미지 intrinsic revision과 layout revision이 어긋나는가?** image resource snapshot revision 1을 runtime stamp로 전달하며 각 input revision, 앱 diagnostics revision도 1인지 확인한다. 두 플랫폼 네 scene에서 일치해 통과했다.
12. **system font나 host별 Ahem 대체로 text size가 흔들리는가?** `TextLayoutEngine::new_without_system_fonts()`에 저장된 Ahem font bytes를 등록하고 20×10 min/max metrics 및 font registry revision을 확인한다. 통과했다.
13. **grid style 변경으로 implicit track·gap·flex 분배가 결과를 바꾸는가?** 220×180 root, 네 `min-content` columns, 세 60px rows, 0 gap, row auto-flow, line-name/template-area 부재를 검사한다. 지원 입력 범위가 고정되어 통과했다.
14. **익명 text auto-placement가 image의 explicit placement에 밀려 다른 cell에 놓이는가?** 이미지 row/column placement와 anonymous text의 root-relative item frame을 함께 검사한다. text는 2-image에서 `(40,0)`, 3-image에서 `(52,0)`이고 통과했다.
15. **track 결과만 맞고 stage 순서나 row sizing이 틀리는가?** Columns intrinsic batches `[1,2]` 또는 `[1,2,3]`, 최종 columns, `[60,60,60]` rows를 동시에 검사한다. Android/iOS 각 4/4가 통과했다.
16. **contribution trace가 개수만 맞고 item/kind/span/value가 바뀌는가?** Rust는 각 contribution을 node·kind·span·value로 비교하고, 두 앱 verifier와 캡처기는 단계별 기록 순서까지 검사한다. 최초 검토에서 harness가 contribution을 item별로 예상하는 결함을 찾아 sizing phase 순서로 수정했고, 최종 실행 8/8이 통과했다.
17. **DOM 순서를 뒤집으면 track 크기가 달라지는가?** forward와 reversed 두 쌍 모두 2-image `[12,28,20,0]`, 3-image `[18,34,20,0]`을 예상하고 simulator가 동일 값을 냈다. 통과했다.
18. **누락되거나 non-finite 숫자가 Android 검증기를 통과하는가?** 검토에서 `Math.abs(NaN - expected) > tolerance`가 false가 되는 허점을 찾았다. frame과 track에 `Double.isFinite` 및 track JSON 필수 필드 검사를 추가한 뒤 Android APK를 다시 빌드하고 네 장면을 재실행해 통과했다.
19. **scene 누락·중복·잘못된 APK 설치가 결과 집계를 오염시키는가?** capture 도구는 scene ID 고유성, 요청 scene 전부 선택, run ID 길이, draw/present 및 전체 diagnostics를 확인한다. AVD 설치 APK hash가 로컬 파일과 일치하고 4/4 결과가 저장되어 통과했다.
20. **화면 캡처를 pixel·border·compositor 검증으로 과장하는가?** 저장된 PNG는 scene host surface와 상태 문구 확인 자료로만 취급한다. texel readback, CSS border stroke, border 바깥 overflow, OS compositor pixel은 검사하지 않았으며 README와 계획에 이 한계를 명시했다.

## 결과

고친 구현 검증 결함은 contribution trace 기대 순서와 Android `NaN` numeric check 두 건이다. 두 수정 뒤 Runtime C11.4 테스트 8/8, FFI scene matrix 1/1, Android API 37 AVD 4/4, iPhone 17 Pro/iOS 26.2 Simulator 4/4, 기본·opt-in archive 문자열 경계, feature 및 workspace Clippy, rustfmt가 통과했다. Chrome 155의 3-image 관측과 독립 CSS Grid 계산 차이는 미해결 호환성 차이로 유지한다.
