# C11.4 cyclic `minmax(calc())` minimum 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 분리해 최종 runtime adapter, projection/layout regression, opt-in FFI 경로와 최신 Android/iOS Simulator 실행 결과를 대조했다. 첫 Android 시도에서 FFI·platform에는 등록됐지만 두 번째 runtime scene registry에는 빠진 경계를 발견해 고쳤다. adapter의 scene matrix를 별도 파일로 분리한 뒤 Rust 테스트와 두 플랫폼 capture도 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 |
| ---: | --- | --- |
| 1 | CSS declaration은 FFI 목록에만 있고 runtime dispatch가 누락되는가 | C11.4 FFI dispatcher가 새 module을 고정 source lookup 순서에 연결하고 네 ID를 source test에서 확인한다. |
| 2 | scene root 판정이 HTML이 아닌 node나 다른 namespace를 허용하는가 | runtime adapter는 HTML element, exact root ID 및 root ownership을 확인한다. 다른 종류는 scene으로 취급하지 않는다. |
| 3 | root ID만 알면 다른 node topology가 같은 fixture가 되는가 | root `div`, `data-node`, 단일 direct item, item ID와 owner 관계를 모두 검사한다. |
| 4 | 여러 자식 또는 nested element가 direct-leaf text로 합쳐지는가 | 한 direct item과 그 아래 단일 text node만 허용하며 내용은 정확히 `a`다. |
| 5 | 지원하지 않는 style profile이 track 계산으로 통과하는가 | `RuntimeBlockFlexGridTrackSizingV1`과 root/item computed style을 확인하고 나머지는 fail closed한다. |
| 6 | calc minimum이 typed `Calc`가 아니라 다른 breadth로 projection되는가 | style-to-layout test가 Columns와 Rows의 min `Calc(LayoutCalcId)` 및 max `MaxContent`를 직접 assertion한다. |
| 7 | calc ID가 축을 바꾸거나 두 expression을 공유하는가 | 회귀 test에서 column/row expression이 각각 `GridTemplateColumns`와 `GridTemplateRows` property의 별도 ID를 받는다. |
| 8 | percentage나 absolute term이 사라지거나 값이 바뀌는가 | runtime validator는 min calc의 200%와 axis별 4px/2px term 및 `max-content` maximum을 확인한다. |
| 9 | cyclic minimum 장면이 definite control과 뒤바뀌는가 | cyclic column/row는 각 intrinsic 축을 `max-content`; controls는 width/height 10px로 별도 등록하고 네 결과 모두 통과했다. |
| 10 | percentage basis가 border-box 14px로 잘못 적용되는가 | root는 content-box sizing과 사방 2px border를 검증한다. 독립 기대값 및 최종 track은 content basis 10px에서 24px/22px다. |
| 11 | root frame에 border edge가 빠지거나 두 번 합산되는가 | 네 scene 모두 root border box `14×14px`, item 원점 `(2,2)`와 expected frame을 두 플랫폼에서 확인했다. |
| 12 | cyclic column의 Rows 또는 cyclic row의 Columns가 누락되는가 | Android와 iOS 로그에서 column `[24]/[10]`, row `[10]/[22]`의 Columns/Rows trace를 각각 확인했다. |
| 13 | frame만 맞고 실제 text contribution이 빠지는가 | Ahem font를 system font 없는 engine에 등록하고 한 글자 `a`의 item frame 및 최종 track sizing을 확인했다. |
| 14 | intended overflow 표본이 root/item 바깥을 가리키지 않는가 | column x=13/16/27, row y=13/16/25 표본이 border-overpaint, root 바깥 item, item 바깥 순서로 설정됐다. |
| 15 | border를 테스트 overlay가 대신 그려 실제 CSS paint를 가리는가 | scene source 자체의 `2px solid #f0c674`를 쓰며 overlay 경로는 없다. border 안쪽 표본에서 실제 CSS 색을 읽었다. |
| 16 | pixel matcher가 위치·node·색을 일부만 확인하는가 | scene마다 다섯 sample ID, target node, CSS 좌표와 RGBA를 확인하며 중복·누락 sample은 실패한다. |
| 17 | Android 표본이 log에만 선언되고 PixelCopy surface에서 읽히지 않는가 | Android API 37 ARM64 SwiftShader AVD에서 4/4 scene, PixelCopy 20/20 sample exact로 통과했다. |
| 18 | iOS 결과가 compositor 이후 화면까지 확인한 것으로 과장되는가 | iPhone 17 Pro/iOS 26.2 Simulator의 Metal drawable에서 4/4 scene, 제출 전 sample 20/20 exact다. compositor 이후 pixels는 미검증이다. |
| 19 | 첫 runtime 실패나 구조 refactor가 최종 결과에 숨겨지는가 | 첫 실패의 `unregistered cyclic calc track scene` 로그를 보존했고, 추가 allowlist와 scene module 분리 후 Rust tests·build·최종 capture를 재실행했다. |
| 20 | 고정 fixture가 기본 product archive로 새어 나가거나 CSS 지원으로 확대 해석되는가 | 기본 release archive에서 새 scene ID와 fixture host 진입점 marker가 없고 opt-in archive에는 있다. 이는 static archive 문자열/심볼 경계이며 전체 APK/IPA나 일반 CSS 지원 판정은 아니다. |

**범위:** 네 fixed scene의 opt-in runtime만 검증했다. Android 물리 기기·hardware GPU, iOS compositor 이후 pixels, WPT, arbitrary CSS admission 및 전체 C11.4는 확인하지 않았고 공식 `72/213`은 유지한다.
