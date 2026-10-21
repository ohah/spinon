# C11.4 mixed text/image runtime 구현 실패 관점 검토 20개

검토 범위는 `c11_4_mixed` runtime adapter, text+replaced-size 단일 계산 경로, fixture feature 경계, 앱 harness와 실제 세 플랫폼 실행이다. 계획 검토 표를 재사용하지 않았다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | fixture 외 scene ID가 부분 입력이나 일반 layout으로 흘러가는가 | 고정 `SCENES` 네 개만 mixed adapter가 인정한다. 혼합 image가 포함된 미등록 scene은 뒤의 이미지 fixture allowlist에서 거부된다. 기존 경계 테스트와 mixed FFI allowlist 테스트 통과. |
| 2 | fixture feature 없이 새 runtime 분기가 제품 빌드에 노출되는가 | runtime, style-to-layout adapter, layout 계산 method가 동일 목적의 opt-in feature에 묶였다. 기본 `spinon-ffi` archive에서 symbol·scene/error marker·PNG 97 bytes가 없고 positive control에는 모두 존재한다. |
| 3 | 잘못된 computed-style profile에서 fixture가 실행되는가 | mixed adapter가 `RuntimeBlockFlexGridTrackSizingV1` 외 profile을 거부한다. profile은 runtime dispatcher의 grid-track sizing 분기에서만 준비된다. |
| 4 | root의 namespace/tag/parent가 다를 때 허용되는가 | HTML namespace, `div`, document root child를 모두 확인하고 다르면 unsupported-case error를 돌려준다. |
| 5 | child 수가 2/3개 외일 때 빈 입력으로 계속되는가 | 장면별 direct child 수가 정확히 같아야 한다. mismatch는 layout 계산 전에 거부한다. |
| 6 | DOM 순서가 fixture source order와 다른데 node ID만 우연히 맞는가 | child를 고정 item 배열과 순서대로 비교하고 node ID, HTML namespace, parent를 함께 확인한다. Rust diagnostics와 native harness도 `sourceOrder`를 검사한다. |
| 7 | text item에 요소 자식, 복수 text node, 빈 text가 섞이는가 | direct child가 단일 text node이고 그 문자열이 정확한 Ahem fixture 문자열이어야 한다. |
| 8 | image가 nested child를 갖거나 `img`가 아닌 element로 바뀌는가 | 고정 tag와 leaf topology를 확인한다. child가 추가되면 허용하지 않는다. |
| 9 | 다른 URL이 같은 decoded 크기를 돌려주는 경우 고정 PNG로 오인하는가 | image `src`를 고정 URL과 정확 비교한 다음 `ImageRequest`를 생성한다. |
| 10 | PNG decode 실패 후 자연 크기 0 또는 기본값을 쓰는가 | decode error를 명시적으로 반환한다. 실패에서 부분 intrinsic snapshot을 만들지 않는다. |
| 11 | decode 결과의 metadata가 resource snapshot에서 빠지는가 | 모든 image node의 snapshot entry를 요구하고 누락 시 에러를 반환한다. |
| 12 | 자연 크기 24×16 이외의 asset을 사용해 예상 track이 달라지는가 | decoded dimensions를 상수와 비교한다. 독립 fixture test와 세 플랫폼 trace에서 replaced source 및 최종 track을 확인했다. |
| 13 | intrinsic-size revision이 증가 불가/누락일 때 revision 0으로 되돌아가는가 | `INITIAL.checked_next()`의 실패를 오류 처리하고 snapshot revision을 `LayoutInputRevision`에 전달한다. trace에서 `intrinsicSize=1`을 확인했다. |
| 14 | Grid 크기, column/row 수·크기 또는 named/auto tracks가 변형되는가 | root의 definite 220×180, 네 min-content columns, 세 60px rows, line-name 개수, auto-repeat 부재를 typed computed style에서 확인한다. |
| 15 | implicit track, auto placement, nonzero gap이 추적 계산을 바꾸는가 | row auto-flow, auto rows/columns, 0 gap, grid track vectors를 제한 조건으로 검증한다. |
| 16 | column/row line 또는 span이 scene 선언과 달라지는가 | 각 item의 start/end line, span, row line을 typed placement와 정확 비교한다. |
| 17 | margin/padding/border/min/max/flex basis가 contribution에 개입하는가 | item sizing, zero edges, zero border, auto min size와 max/basis 상태를 검증한다. root도 고정 dimensions와 spacing 조건을 검사한다. |
| 18 | text contribution이 다른 font, weight, wrap, direction 또는 line metrics에서 산출되는가 | Ahem direct leaf와 font registry, font size/weight/style, line-height, white-space, wrap, direction 등 CSS property를 고정 비교하고 Ahem 등록 실패를 반환한다. |
| 19 | image transferred inline size가 CSS height/aspect ratio 또는 self alignment 변형으로 달라지는가 | `width:auto`, `height:48px`, auto aspect ratio, `justify-self/align-self:auto`를 검사한다. runtime에서 이미지 span-2 72px contribution과 최종 frame을 확인했다. |
| 20 | Android/iOS가 scene을 거부하거나 캡처 overlay가 동작 화면으로 잘못 제출되는가 | 첫 Android 실행에서 Java allowlist 누락으로 `SPINON_C114_GRID_ARGUMENT_ERROR`를 재현해 allowlist/expectation을 고쳤다. 후속 캡처에서 알림 shade가 섞인 두 PNG를 버리고 Android 캡처 전에 status bar를 접도록 보완했으며 최종 세 플랫폼 네 장면을 재실행했다. |

## 발견과 수정

- 첫 Android 앱 harness가 새 scene ID를 허용하지 않았다. 허용 scene, box count, root/item frame, source kind/order, intrinsic revision 기대값을 Android와 iOS harness에 추가했다.
- 실기기 캡처 PNG 두 장에 Android 알림 shade가 찍혔다. 해당 두 화면 파일을 제거하고 capture runner가 screenshot 직전에 status bar를 접도록 보완했다. 최종 실기기 네 화면을 다시 확인했다.
- Rust runtime 입력·계산·revision에서 추가 결함은 발견하지 않았다. runtime focused tests 10/10, FFI mixed/bounded gate tests, Rust fmt/Clippy, Android API 37 AVD·SM-S731N 실기기·iOS 26.2 Simulator 네 장면 실행을 통과했다.

## 남은 확인 범위

이 검토는 고정 fixture와 화면/trace 기준이다. PixelCopy/Metal 표본, PNG texel, iOS compositor 후 출력, 임의 CSS·nested item·async image·WPT 및 전체 C11.4는 검증하지 않았다.
