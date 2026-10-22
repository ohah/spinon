# C11.4 cyclic `fit-content(calc())` 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 별도로 최종 구현 코드와 현재 소스에서 실행한 Android/iOS runtime 결과를 대조했다. 검토 중 발견한 C11.4 전용 painter 준비 코드의 위치를 공통 renderer 입력 구성과 fixture별 validator로 나눠 `fixtures/` 경계에 맞췄다. 영향받은 fixture test, Clippy, platform builds와 capture를 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 |
| ---: | --- | --- |
| 1 | 미등록 scene ID가 비슷한 prefix만으로 실행되는가 | FFI와 runtime allowlist는 네 ID만 등록한다. 미등록 fit-content calc ID는 명시 오류, 다른 ID는 다른 fixture 경로로 남는다. |
| 2 | root 식별이 임의 태그나 namespace를 허용하는가 | scene 판별은 HTML namespace와 exact root ID를 확인한다. 다른 namespace/non-element은 false로 빠진다. |
| 3 | root ownership나 `data-node` 누락으로 node-id 계약이 바뀌는가 | runtime fixture validator가 root owner, marker, direct parent와 node identity를 확인한다. |
| 4 | 여러 direct Grid item이 단일 text contribution으로 오인되는가 | root direct child가 정확히 하나인 경우만 통과시킨다. |
| 5 | item 아래 comment/element/여러 text child가 하나의 Ahem run으로 합쳐지는가 | 직접 text node 하나와 scene별 고정 문자열을 검증한다. |
| 6 | `fit-content(calc())`가 percent/length를 잃거나 단순 px로 취급하는가 | computed style의 typed `FitContent(calc())` percentage와 absolute length를 검증하고 style-to-layout 결과의 track을 확인한다. |
| 7 | cyclic column에서 intrinsic basis를 definite root처럼 잘못 적용하는가 | 독립 기대 29px과 Columns `[29]`, root `50×20`, item `29×20`이 Android/iOS에서 일치했다. |
| 8 | cyclic row에서 block-axis basis를 놓치거나 column만 계산하는가 | Rows `[22]`, root `20×40`, item `20×22`가 Android/iOS에서 일치했다. |
| 9 | definite width 40px control이 cyclic branch에 잘못 들어가는가 | definite column은 limit 24px, Columns `[24]`, root `40×20`으로 양 플랫폼에서 통과했다. |
| 10 | definite width 10px에서 track clamp와 item min-size가 섞이는가 | `min-width:0`을 검증하고 track/item 폭 9px을 양 플랫폼에서 확인했다. |
| 11 | Ahem fallback이 host/system font에 따라 intrinsic contribution을 바꾸는가 | 저장소 Ahem font를 등록하고 system font 없는 engine을 써서 50/20px intrinsic metrics를 고정한다. |
| 12 | white-space, wrap, weight, line-height 변형이 control을 바꾸는가 | scene validator가 Ahem 10px, weight 400, normal style, 10px line-height와 wrapping properties를 고정한다. |
| 13 | planned overflow point가 실제 item/root 바깥이 아닌가 | small-limit `(15,5)`는 item 9px 및 root 10px 모두 바깥이고 red exact였다. |
| 14 | overflow 뒤 임의 위치에도 글리프가 남는가 | `(25,5)`는 black exact였다. surface text 범위를 넘어 무제한 red paint가 되지 않는다. |
| 15 | 계산된 text layout이 GPU render scene에서 빠지는가 | 첫 PixelCopy 시도에서 이 누락을 재현했다. 고정 scene만 기존 text painter에 연결했고 최종 capture에서 text count/batch와 overflow sample이 통과했다. |
| 16 | renderer glue가 일반 runtime이나 다른 fixture에 영향을 주는가 | adapter 호출은 runtime paint 활성 상태와 네 exact root ID일 때만 생긴다. 일반 CSS admission/profile은 바꾸지 않았다. |
| 17 | profile에서 빠진 computed color가 엉뚱한 paint color로 대체되는가 | fixture computed property의 `rgb(229, 72, 77)`을 검증한 뒤 고정 sRGB 값을 사용한다. 다른 값은 fail-closed다. |
| 18 | duplicate style, stale generation, invalid DPR, detached text owner가 render 입력에 들어가는가 | fixture input builder가 style 중복, profile/revision/DPR, owner style와 handle 소속을 확인하고 기존 text attach가 source revision을 검증한다. |
| 19 | clipping 설정이 아이템 경계에서 글리프를 잘라낸 채 geometry만 통과하는가 | PixelCopy와 drawable 표본에서 item 바깥 x=15 red, 더 바깥 x=25 black을 확인해 paint 결과까지 검증했다. CSS border stroke는 별도다. |
| 20 | 두 플랫폼에서만 통과하고 fixture가 제품 archive에 포함되는가 | Android API 37 SwiftShader PixelCopy와 iOS 26.2 Metal drawable은 각 9/9 exact다. 기본 release archive는 marker가 없고 fixture-test positive archive에는 marker가 있다. |

**범위:** Android 물리 기기/GPU와 iOS compositor 이후 pixel, WPT, 임의 CSS input은 이 검토로 입증되지 않는다. C11.4 전체 상태와 `72/213`은 바꾸지 않았다.
