# C11.4 cyclic `minmax(calc())` minimum span runtime review · 20 failure angles

검토 범위는 네 고정 scene 구현과 최신 Android/iOS Simulator 실행이다. 계획 PR의 기존 20점 검토와 분리했다. 결과는 아래 코드 경계와 보존된 최신 실행 로그를 직접 대조했다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 네 scene 외 ID가 fixture 경로에 들어가는가 | 통과. FFI source registry와 runtime scene registry가 네 ID만 추가하며, 등록되지 않은 ID는 source를 돌려주지 않는다. 이전 첫 AVD run에서 누락 registry가 `unregistered cyclic calc track scene`로 실패한 뒤 같은 고정 목록을 양쪽 registry에 연결했다. |
| 2 | cyclic/definite control이 잘못 분류되는가 | 통과. 네 `Scene`은 width/height와 `cyclic` 여부를 개별 고정한다. 두 cyclic scene은 해당 축만 `max-content`; control은 같은 basis를 definite px로 지정한다. |
| 3 | 열/행 calc의 percentage 또는 length가 뒤바뀌는가 | 통과. 각 track은 typed calc `Sum([Percentage(0.5), LengthPx(4|2)])`와 `MaxContent`만 허용한다. |
| 4 | 두 track의 개수나 동등성이 흐려지는가 | 통과. runtime matcher는 해당 scene에서 정확히 두 track을 요구하고 각 track이 같은 min/max grammar인지 검사한다. |
| 5 | span placement가 다른 축 또는 다른 span을 가리키는가 | 통과. computed placement를 typed line/span으로 검사한다: columns `1 / span 2`, rows `1 / span 2`, 다른 축은 Auto다. |
| 6 | gap을 intrinsic distribution에서 0회·2회 빼는가 | 통과. column/row scene enum의 고정 4px gap과 `intrinsic_sizing_tracks_with_gutters` 회귀 기대값이 `[0,23,4,23,0]`, `[0,8,4,8,0]`이다. |
| 7 | cyclic percentage가 intrinsic 기여에 먼저 적용되는가 | 통과. cyclic cases는 intrinsic basis를 별도로 고정하고 final used tracks `[29,29]` / `[12,12]`를 확인한다. definite controls는 같은 최종 값으로 독립 대조한다. |
| 8 | `max-content` limit이 calc minimum 아래에서 잘못 이기는가 | 통과. 직접 style-to-layout regression은 intrinsic 분배 `[23,23]` / `[8,8]`와 final minimum floor를 모두 검사한다. |
| 9 | text metrics가 font/platform fallback에 의존하는가 | 통과. Ahem fixture font byte를 include하고 no-system-font text engine을 사용한다. columns는 min/max inline `20/50px`, rows는 `10/30px`를 검사한다. |
| 10 | wrapping 및 line count가 달라지는가 | 통과. columns `aa aa`는 one-line max-content 50px, rows `a a`는 20px inline width에서 two-line block max-content 20px로 검사한다. |
| 11 | root border가 percentage basis/frame에서 빠지거나 두 번 더해지는가 | 통과. 2px border와 content-box basis를 고정하고 root border box `54×14` / `24×24px`를 Android/iOS에서 확인했다. |
| 12 | item이 root content box로 잘려 overflow frame이 보이지 않는가 | 통과. 기대 item frame은 column `62×10px`, row `20×28px`이며 root보다 넓거나 높다. 두 Simulator의 node frame trace와 screenshot에 overflow가 남는다. |
| 13 | item 외부 표본이 item paint를 재사용하거나 좌표를 잘못 찍는가 | 통과. root 기준 `(58,6)`/`(8,27)`은 item의 root-border 바깥 내부에서 빨강 `(229,72,77,255)`이고, 더 먼 `(66,6)`/`(8,31)`은 검정 `(0,0,0,255)`이다. 각 플랫폼 8/8 exact. |
| 14 | Chrome white page backdrop을 앱 native surface와 혼동하는가 | 통과. precomparison white는 browser page 관찰이다. AVD PixelCopy와 iOS Metal drawable은 실제 host surface clear black을 따로 기대한다. |
| 15 | Android 테스트가 오래된 APK를 설치하는가 | 통과. 소스 변경 뒤 Gradle `assembleDebug`를 다시 실행했고 캡처 로그는 그 패키지의 네 scene을 모두 시작해 검증했다. 이전 실패 로그는 유지했다. |
| 16 | iOS 테스트가 다른 시뮬레이터를 선택하는가 | 통과. 명시 UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`의 iPhone 17 Pro/iOS 26.2 Simulator에만 설치·실행했다. |
| 17 | stage trace 누락을 frame만 보고 통과시키는가 | 통과. Android/iOS runtime logs에서 intrinsic span 2, axis used tracks, trace chunks 및 draw/present를 장면별 확인했다. |
| 18 | 기본 제품 FFI archive에 fixture payload가 섞이는가 | 통과. 별도 default archive와 opt-in positive-control archive의 scene ID·marker·symbol string 검사를 기록한다. 이는 최종 APK/IPA 전체 release payload 검증과 구분한다. |
| 19 | style/runtime registry가 drift해 실행 직전 거부되는가 | 통과. 최초 실패가 이 경계를 실제로 드러냈다. `scene.rs` exact registry를 16→20으로 늘린 뒤 registry unit test 및 runtime fixture-test suite를 다시 실행했고 두 플랫폼 4/4가 통과했다. |
| 20 | 검증 결과가 C11.4 전체 또는 일반 CSS 지원으로 과장되는가 | 통과. exact four-scene opt-in fixture만 다루며 WPT, 일반 CSS admission, physical Android/hardware GPU, iOS compositor-after pixels와 C11.4 전체 완료는 미검증으로 남긴다. C11.4 checkbox를 유지하고 현재 공식 `77/213` 집계는 바꾸지 않는다. |

상세 입력, trace, pixel log, screenshots, build 및 archive hash는 [실행 근거](./README.md)에 기록한다.
