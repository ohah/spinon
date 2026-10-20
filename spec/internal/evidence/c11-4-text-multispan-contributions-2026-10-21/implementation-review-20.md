# C11.4 direct-leaf text multi-span 구현/runtime 적대 검토 20개

검토 대상은 [집중 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-text-multispan-contributions.md), Rust text adapter와 contribution test, FFI source router, Android/iOS fixture validator, 캡처 runner다. Chrome 155 사전 비교 후 구현의 입력 경계와 실제 앱 실행 경로를 서로 다른 실패 관점으로 확인했다. 수정 사항은 마지막에 반영된 코드로 다시 검사했다.

| # | 실패 관점 | 검사와 결과 |
|---:|---|---|
| 1 | fixture 소스가 기본 제품 artifact에 섞임 | FFI scene source는 fixture module 아래에 있고 `c04-runtime-gpu-fixture-test` feature가 opt-in한다. 기본/positive artifact 문자열 검사를 별도 기록했다. 통과. |
| 2 | 알 수 없는 scene ID가 동적 JS source로 이어짐 | FFI router와 source test는 4개 고정 ID만 반환하고 임의 ID를 오류로 거부한다. Android/iOS validator도 같은 ID 집합을 갖는다. 통과. |
| 3 | 비슷한 `c114-…-grid` ID가 다른 fixture로 오인됨 | 루트 ID에서 prefix/suffix를 분리한 뒤 고정 scene table에서 정확히 일치하는 ID만 처리한다. 통과. |
| 4 | 허용 scene의 루트가 Grid `div`가 아님 | known scene의 루트 tag를 검사하고 다르면 layout failure를 반환한다. 통과. |
| 5 | 누락·추가 grid item이 partial layout으로 통과 | root direct-child 개수를 해당 scene의 고정 item 개수와 대조한다. 통과. |
| 6 | reverse scene에서 DOM 순서와 item 의미가 뒤섞임 | 각 child의 순번 기반 ID와 scene table의 span/text를 함께 비교한다. forward/reversed 4개 실행에서 순서별 frame이 일치했다. 통과. |
| 7 | item이 다른 parent의 node이거나 nested wrapper임 | item tag, 정확한 ID와 root direct-parent 관계를 확인한다. 통과. |
| 8 | text 외 child, 빈 child 또는 두 개 이상의 child가 허용됨 | 각 item은 direct child 한 개만 허용하고 그 node kind가 Text인지 확인한다. 통과. |
| 9 | 테스트 문자열과 다른 공백·문자열이 측정됨 | scene별 UTF-8 문자열을 정확히 비교한다. Ahem 측정은 allowlist 값만 쓴다. 통과. |
| 10 | 시작/끝 선 또는 행 배치의 한 longhand가 바뀜 | column-start, column span, row-start, row-end를 모두 typed computed placement와 비교한다. 첫 검토에서 row-end 검사가 빠진 점을 발견해 `Auto` 비교를 추가했다. focused Rust test와 Android 실기기·AVD·iOS Simulator를 다시 실행해 통과했다. |
| 11 | 루트의 추가 min/max 또는 flex 제약이 기대 frame을 우회함 | width/height뿐 아니라 min/max width/height와 flex-basis를 고정 computed 값으로 검사한다. 검토 중 누락을 찾아 추가했고 focused test와 전체 플랫폼 재실행이 통과했다. |
| 12 | 숨은 implicit track 또는 다른 auto-placement가 입력에 들어옴 | auto-flow row, auto rows/columns, 4개 min-content columns, 3개 20px rows를 검사한다. 해당 auto-grid allowlist가 없던 점을 보완했다. 통과. |
| 13 | repeat, areas 또는 line-name metadata가 예상 topology를 바꿈 | auto-repeat·template areas는 없어야 하며 column/row line-name 배열은 기대 경계 수와 모두 빈 이름을 가져야 한다. 통과. |
| 14 | margin·padding·gap·border 또는 정렬이 item geometry에 섞임 | root의 정렬 속성, 모든 edge, gap, border를 검사한다. item edge와 box sizing도 별도로 고정한다. 통과. |
| 15 | 글꼴/줄바꿈 속성 차이가 intrinsic width를 바꿈 | Ahem family, 10px 크기, normal weight/style, 10px line-height, whitespace collapse, wrap, direction, break, spacing 등 measured input 속성을 검사한다. 통과. |
| 16 | system font fallback 또는 등록 실패가 0 크기로 보정됨 | system-font 사용 없이 한 engine에 고정 Ahem 데이터를 등록하며 등록 오류는 layout failure다. 통과. |
| 17 | 여러 item의 text style이 다른 DOM node에 연결됨 | 각 item handle을 key로 paragraph style을 따로 넣고 diagnostics의 node/source-order를 scene item과 비교한다. 통과. |
| 18 | min/max-content 크기나 높이가 잘못된 text 입력으로부터 옴 | 세 문자열의 폭 50/80, 70/120, 180/180 CSS px와 줄 높이를 Rust diagnostics에서 검사한다. 네 scene 모두 통과. |
| 19 | `Minimum` 또는 `MinContent` contribution 종류·값·호출 수가 다름 | min-content track에서 automatic minimum 0, 다중 span 사유, span별 item mapping과 두 단계의 MinContent 질의 순서를 정확히 검사한다. `MaxContent`는 측정 입력에 보존되고 이 all-min-content track pass의 contribution으로 요청되지 않는다. 통과. |
| 20 | span batch 순서, 최종 track, frame, app present가 어긋남 | Rust stage sequence `[2]`/`[2,3]`, 최종 track과 root-relative item frame을 검사했다. Android SM-S731N, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator에서 각 4/4 scene의 track·frame·trace·draw/present가 통과했다. |

실기기 실행은 `android-device-c114-text-multispan-contributions-android-device-20261020T212839375Z-464d2f7a/result.json`과 `text-span2-before-span3.png`, API 37 AVD의 `android-avd-c114-text-multispan-contributions-android-avd-20261020T212853774Z-2a625474/result.json`, iOS Simulator의 `ios-simulator-c114-text-multispan-contributions-ios-simulator-20261020T212856853Z-a54892a3/result.json`에 있다. 실기기는 Samsung SM-S731N Android 16/API 36, arm64-v8a, Xclipse 940/ANGLE over Vulkan이며 설치 APK hash가 로컬과 일치한다.

화면은 CSS item box와 frame 확인용이다. runtime log에 `runtime_text_count=0` 및 `glyph_batch_draw_count=0`으로 남아 있어 글리프 paint, iOS 실기기, OS compositor 이후 픽셀을 검증한 것으로 확대 해석하지 않는다. 이 제한 fixture 검토는 C11.4 전체 완료 판정이 아니며 STATUS checkbox와 완료 수를 그대로 둔다.
