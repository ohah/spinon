# C11.4 max-content span-group 구현·런타임 적대 검토 20개

**검토 범위:** 새 max-content intrinsic scene 두 개와 80px definite control 두 개, Rust layout/FFI fixture, Android·iOS Simulator runner, 기록된 pixel/trace 결과. 계획 단계 검토와는 별도로 실제 구현 및 실행의 실패 경로를 확인했다.

1. **허용하지 않은 scene ID가 새 입력으로 실행되는가?** — runtime 및 FFI 양쪽은 고정 scene 배열에서만 source를 만들고 알려지지 않은 ID는 반환하지 않는다. 중복 없는 8개 allowlist 검사와 unlisted ID 검사가 통과했다.
2. **scene ID가 같아도 item 개수·span 구성이 달라질 수 있는가?** — 고정된 A/B/C 세 item, span 2/2/3과 배치를 검증하고 Android/iOS에서 node ID 및 box 수를 확인한다. 4개 신규 scene 모두 4 boxes다.
3. **`max-content` 문자열이 computed style에서 다른 dimension으로 처리되는가?** — Rust runtime validator가 `ComputedCssDimension::MaxContent`를 요구하며, 네 플랫폼 capture 결과의 root width도 기대값과 일치했다.
4. **max-content intrinsic 결과가 80px definite 결과로 잘못 대체되는가?** — intrinsic root는 content 110×10, border 114×14이고 control은 content 80×20, border 84×24다. Rust cases와 양쪽 Simulator 결과가 둘을 구분한다.
5. **cyclic percentage의 auto 초기 basis가 사라지는가?** — max-content Columns 첫 pass 초기값 `[2,2,2]`, 두 번째 resolved pass 초기값 `[13,13,13]`를 trace에서 따로 검사한다. 예상 두 pass가 모두 기록됐다.
6. **resolved percentage minimum을 cyclic-auto 값으로 계속 쓰는가?** — 두 번째 Columns pass 최종값은 고정 rational 기대치 `[107/3,125/3,86/3]`와 일치한다. 첫 pass와 두 번째 pass가 혼동되지 않는다.
7. **span 2 항목의 증가 계산이 DOM 순서에 따라 달라지는가?** — 첫 cyclic-auto Columns pass에서 span-2 batch 이후 base `[9,19,19]`를 확인하고, 기존 span order 검증과 두 순서의 동일 결과를 유지한다.
8. **span 3 그룹이 span 2 그룹보다 먼저 소비되는가?** — trace validator는 span stage를 조회해 span-2 증가를 검사하며 두 플랫폼에서 trace chunk 전체를 파싱한다. 장면별 trace가 모두 완결되어 있다.
9. **columns 계산 결과가 rows의 text 측정에 반영되지 않는가?** — trace에 Columns 뒤 Rows pass가 있고, final line advances `[60]/[40]/[110]` 및 `[40,10]/[40]/[80,20]`은 고정 Rust oracle과 일치한다.
10. **max-content row의 cyclic basis와 final row가 혼동되는가?** — Rows 초기 basis `[2,2]`→resolved `[3,3]`→final `[4,4]`를 별도로 비교한다. 두 순서의 결과가 같다.
11. **80px control의 column pass가 불필요한 intrinsic 재실행을 하는가?** — definite width에는 Columns pass 하나만 요구하고 초기 `[10,10,10]` 및 최종 `[76/3;3]`를 검사한다.
12. **80px control의 row pass가 cyclic-auto 및 resolved basis를 건너뛰는가?** — Rows 두 pass를 요구하며 초기 `[2,2]`, resolved `[4,4]`, 최종 `[9,9]`를 검사한다.
13. **부동소수 오차가 넓은 허용치에 숨는가?** — frame·track assertion 허용치를 `1/64 CSS px`로 제한하고 simulator에 기록된 rational thirds 값이 모두 통과했다.
14. **reverse order에서 frame이 배열 위치를 따라 item에 잘못 귀속되는가?** — Android/iOS validators가 key와 예상 node order를 함께 비교한다. reverse 장면에서 A/B/C frame은 forward 결과와 동일하게 유지된다.
15. **root frame은 맞지만 item frame 또는 border box가 넘치는가?** — 모든 root와 세 item의 x/y/width/height를 검증하고 화면 sample을 root right border와 root 바깥 위치에 배치했다.
16. **pixel 표본이 장면의 다른 node를 가리키거나 border에서 벗어나는가?** — 각 플랫폼은 root ID, 장면별 sample ID, x/y를 고정 확인한다. Android PixelCopy와 iOS Metal에서 scene당 두 표본이 정확하다.
17. **root 바깥 clear 표본이 실제로 검증되는가?** — border 바로 바깥의 black `[0,0,0]` sample을 별도 확인한다. 네 scene마다 border와 clear sample이 모두 있으며 Android/iOS 각 8/8이다.
18. **Android 캡처가 stale APK를 새 코드로 오인하는가?** — 최초 실행은 capture가 빌드를 하지 않는 점 때문에 이전 APK를 설치했다. 명시 빌드 후 설치 APK와 산출물 SHA-256 `51c0344f…3be5e6e` 일치를 확인해 네 scene을 다시 캡처했다.
19. **iOS 결과가 픽셀 검사를 하지 않았는데 통과로 기록되는가?** — 최초 실행은 별도 Metal surface-pixel feature 없이 `status=-90`으로 표본 readback이 비활성화됐다. 해당 feature를 켜고 앱을 다시 빌드·실행해 네 scene의 두 Metal 표본씩 통과했다.
20. **fixture가 기본 제품 artifact 또는 일반 CSS 지원으로 확대되는가?** — default iOS Simulator archive에는 신규 문자열·prepare symbol이 없고 fixture-test positive control에는 있다. 검사는 정적 archive 한정이며, STATUS의 C11.4 미완료·`77/213`과 지원 범위 제한을 유지한다.

## 발견 및 조치

- Android 첫 캡처에서 stale APK가 사용됐다. capture runner는 앱 빌드 단계가 아니므로 명시적으로 다시 빌드하고 설치본 SHA를 대조했다.
- Android PixelCopy의 새 sample ID가 기대 RGB에 연결되지 않아 sentinel mismatch가 났다. border 및 clear 색 매핑을 추가하고 네 scene을 재실행했다.
- iOS 첫 캡처는 Metal 표면 표본 기능이 꺼져 있었다. feature flag를 추가한 재빌드로 네 scene의 정확한 표본을 확인했다.

최종 AVD와 iOS Simulator run은 각각 4/4 scene, pixel 표본은 각각 8/8 exact다. 실기기, Android hardware GPU, glyph raster equality, iOS compositor 제출 뒤 pixel, WPT, 일반 CSS admission은 이 검토 범위가 아니다.
