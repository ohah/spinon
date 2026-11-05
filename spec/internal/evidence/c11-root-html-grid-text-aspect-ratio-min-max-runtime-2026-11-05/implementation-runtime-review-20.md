# C11.1 Root HTML `aspect-ratio` min/max 구현·runtime 실패 관점 20개

이 검토는 계획 리뷰 및 PR diff 검토와 별도로, 실제 구현 경계와 두 시뮬레이터 readback을 확인한다.

| # | 실패 경로 | 확인 및 판정 |
|---:|---|---|
| 1 | CSS typed min/max가 `calc()`·percentage를 direct px로 오인 | `layout_math_values`와 typed variant를 검사하며 positive matrix 및 negative controls가 통과했다. 통과. |
| 2 | 음수·비유한·70px 초과 값이 bounded range에 유입 | Projection은 finite `0..=70px`만 허용하고 calc·percentage·71px controls를 거부한다. 통과. |
| 3 | min/max 제약을 숫자 ratio가 아닌 임의 ratio에 연결 | 제약이 있는 ratio는 computed `2 / 1`, typed `2.0`, 기존 `BoxSizing` basis를 모두 요구한다. 통과. |
| 4 | computed serialization과 typed ratio가 서로 다른 scene을 가리킴 | 동일 `NodeId`의 property map과 typed cascade를 검사하며 3:2 constrained control은 fail-closed다. 통과. |
| 5 | CSS 기본 `auto`/`none`을 0px로 취급 | 기본값을 `Auto`/`None`으로 보존하고 constrained 및 no-ratio controls가 각각 확인한다. 통과. |
| 6 | `min > max`에서 max가 min보다 우선 | `min-height:40/max-height:20`은 40px, `min-width:70/max-width:50`은 70px로 Chrome와 일치한다. 통과. |
| 7 | ratio-free control도 비율 전이를 받음 | width와 height control scene은 ratio transfer 없이 min/max sizing만 검사한다. 통과. |
| 8 | owner 또는 첫 sibling의 min/max를 target 예외로 허용 | owner topology 검사와 explicit short sibling guard를 확인했고 negative test가 있다. 통과. |
| 9 | 두 번째 child가 아닌 NodeId에 constraints가 허용 | projection의 target identity와 layout의 second-child identity가 각각 제한한다. 통과. |
| 10 | 다른 Root HTML profile이나 opt-in C11.4 profile로 예외 확장 | admission은 `RuntimeRootHtmlTextPaintV1` 및 ratio target node set에만 적용된다. 통과. |
| 11 | explicit min-height가 automatic text minimum으로 덮어써짐 | `auto_min_height_for_text_item`은 min-height가 초기값일 때만 적용한다. 통과. |
| 12 | explicit min-width가 automatic text minimum으로 덮어써짐 | `auto_min_width_paragraph_for_text_item`은 min-width가 초기값일 때만 측정값을 쓴다. 통과. |
| 13 | max constraint와 automatic minimum의 상호작용에서 default path 회귀 | 기존 ratio matrix와 새 constrained/no-ratio 10-scene runtime test를 함께 실행했다. 통과. |
| 14 | computed constraints가 Taffy layout stage까지 전달되지 않음 | bounded min/max frame/row 예상값이 style-to-layout, layout, 기본 Root HTML runtime 세 경계에서 통과한다. 통과. |
| 15 | 70px item이 60px track 또는 70px Grid 경계에서 잘림 | Chrome frame과 Android PixelCopy/iOS Metal overflow 표본에서 Grid 밖 item 색을 읽었다. 통과. |
| 16 | Android capture가 visible viewport를 layout viewport로 오인 | 초기 실패를 보존했고, 최종 980×2004.4629 layout viewport mapping으로 PixelCopy 3/3 exact다. 통과. |
| 17 | iOS Metal sample이 UIKit visible viewport만으로 변환 | 초기 r2 mismatch 뒤 980×1896.617 layout viewport 및 drawable 비율을 사용했고 r3 3/3 exact다. 통과. |
| 18 | 잘못된 run ID나 다른 scene이 min/max 표본을 호출 | Android/Swift run ID와 iOS scene prefix를 고정하고 C11 surface feature flag 뒤에서만 진입한다. 통과. |
| 19 | run 중 fixture가 빌드 산출물과 달라짐 | Android APK와 iOS app의 fixture bytes/hash 및 root-app verifier markers를 비교한다. 같은 SHA로 통과. |
| 20 | 시뮬레이터만 종료되고 앱 또는 임시 HTML이 남음 | runner의 `finally`가 두 앱을 stop/terminate하고 index/root-app 원본 SHA 복원을 검증한다. 통과. |

**결과:** 범위 내 focused Rust 검증, scoped Clippy/rustfmt, Android AVD PixelCopy 3/3, iOS Simulator Metal drawable 3/3을 확인했다. Android는 SwiftShader software renderer이고 iOS readback은 compositor 제출 전이다. 전체 CSS Grid, WPT, 실기기 또는 최종 OS 표시를 검증하지 않았다.
