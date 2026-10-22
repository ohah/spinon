# C11.4 cyclic max-content 혼합 text/image 구현·runtime 실패 관점 20개

계획의 사전 검토를 복사하지 않고, 새 adapter·trace 검증·플랫폼 capture와 실제 simulator 결과를 별도 경로로 검토했다. 최초 실행에서 발견한 기대값·분류·빌드 flag 문제는 수정한 뒤 영향받는 장면을 다시 실행했다.

| # | 실패 관점 | 코드·실행 대조 및 결과 |
| ---: | --- | --- |
| 1 | 네 신규 scene 외 입력도 adapter가 받아들이는가 | Runtime 및 FFI adapter는 기존 네 장면에 네 고정 ID만 추가한다. 8개 열거/길이 검사가 통과했다. |
| 2 | DOM 역순 fixture가 A/B/C identity나 placement를 바꾸는가 | 정방향/역방향은 child order만 바꾸고 ID별 frame을 비교한다. Android/iOS 각 4/4 통과했다. |
| 3 | FFI scene source와 runtime adapter의 scene 목록이 어긋나는가 | FFI source·outer matrix·runtime source의 8개 ID 검사가 통과했다. |
| 4 | max-content scene에서 computed width가 `min-content`로 projection되는가 | Adapter의 `Width::MaxContent`와 `ComputedCssDimension::MaxContent`를 확인했다. 화면 root width `114px`와 독립 기대값이 맞았다. |
| 5 | definite control이 max-content 조건을 잘못 물려받는가 | 네 ID를 prefix/width로 분류한다. definite 두 scene의 `[12,12,12]`, `[19,19]`, `44×44`가 양 simulator에서 통과했다. |
| 6 | `minmax(calc(10% + 2px),max-content)` 또는 2px 절대항이 바뀌는가 | FFI 입력 source와 고정 CSS 확인을 통과했다. cyclic 초기 bases는 Columns `[2,2,2]`, Rows `[2,2]`다. |
| 7 | max-content Columns 기여가 source order에 따라 누적되는가 | A/B span-2와 C span-3의 column contribution 및 ID별 geometry를 정/역순으로 대조했다. 같은 track 결과다. |
| 8 | cyclic Columns의 첫 pass를 resolved Columns와 혼동하는가 | 첫 Android 실행에서 validator가 잘못된 pass-final 값을 기대한 것을 확인했다. `[24,34,48]`과 resolved `[107/3,125/3,86/3]`를 별도 assertion으로 고쳐 전 장면 재실행했다. |
| 9 | resolved cyclic percentage minimum basis가 max-content content width를 놓치는가 | trace `Initialize=[13,13,13]` 및 resolved Columns 유리수 결과를 Android/iOS에서 확인했다. |
| 10 | image natural ratio가 Rows로 넘어가지 않는가 | PNG natural size `24×16`, CSS width `40px`, aspect ratio와 transferred `80/3px` contribution을 확인했다. |
| 11 | cyclic Rows의 percent basis가 border box나 column track sum에서 파생되는가 | resolved Rows `Initialize=[14/3,14/3]`, final `[37/3,37/3]`를 독립값과 대조했다. |
| 12 | Minimum, MinContent, MaxContent contribution 종류가 합쳐지는가 | 각 item·axis·span의 세 종류를 trace에서 따로 확인한다. Rust style test와 양 platform capture가 통과했다. |
| 13 | A/C text의 max-content line width가 min-content width 또는 item frame과 혼동되는가 | Ahem direct text inputs와 line widths A=`60`, C=`110`을 확인했다. 앱에는 glyph raster가 없다고 결과 문서에 한계를 기록했다. |
| 14 | reversed scene에서 frame이 source index에만 연결되어 A/C가 뒤바뀌는가 | 노드 ID와 frame을 identity별 비교하고 source order 배열도 검사한다. 네 방향 모두 통과했다. |
| 15 | image attach가 잘못된 element나 scene root에 적용되는가 | 네 ID별 image element mapping을 C11.4 paint fixture에서 분리했다. 두 definite scene의 image B frame·natural metadata가 일치했다. |
| 16 | definite overflow가 레이아웃 frame에서만 보이고 표면에서 clip되는가 | 각 definite 장면의 root border, root 밖 green PNG, image 이후 clear sample이 Android 6/6, iOS 6/6 exact다. |
| 17 | Android pixel 결과가 앱 screenshot이나 예상색을 반환하는가 | 로그의 `SPINON_C114_GRID_PIXEL_MATRIX_CAPTURE status=ok`와 3개 실제 PixelCopy 샘플을 확인했다. |
| 18 | iOS pixel fixture가 비활성인 빌드로 성공을 주장하는가 | 첫 빌드에서 `status=-90`을 확인했다. `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 재빌드해 세 Metal drawable sample을 scene별 통과시켰다. |
| 19 | 검증 JS의 max-content width flag와 contribution map이 충돌하는가 | 실행 중 같은 `maxContent` 키가 boolean 대신 contribution map으로 덮여 definite를 잘못 분류했다. `maxContentWidth`와 contribution `maxContent`를 분리하고 전체 네 장면 재검증했다. |
| 20 | fixture-only 코드·문자열·FFI 진입점이 제품 core로 새거나 C11.4 완료로 승격되는가 | 제품 core archive에는 신규 marker/scene source symbol이 없고 fixture positive archive에 있다. 일반 CSS/WPT·APK/IPA release 제외는 주장하지 않고 C11.4와 `77/213`을 유지한다. |

## 판정

위 수정 후 Rust focused tests, 해당 crate fixture-enabled Clippy, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator의 최종 결과는 통과다. 20개 관점에서 남은 차단 구현 결함은 찾지 못했다. 전체 border stroke, glyph raster, Android 실기기/hardware GPU, iOS compositor 이후 pixels, WPT 및 일반 CSS admission은 검증하지 않았다. 상세 로그와 화면은 [실행 근거](README.md)에 있다.
