# C11.4 양축 cyclic `minmax(calc())` minimum PR diff 적대 검토 30개와 후속 확인

계획 자체의 20개 검토와 구현/runtime의 20개 검토를 재사용하지 않고, 이번 PR diff에서 서로 다른 계약·실패 경계를 대조했다. 검토 대상은 Rust fixture 입력·trace 검사, FFI/runtime scene 등록, Android/iOS 앱 검증기, simulator capture, 문서와 근거 자료다.

| # | 실패 관점 | 대조와 결과 |
| --- | --- | --- |
| 1 | column과 row percentage가 하나의 공통 basis를 잘못 공유하는가? | CSS Grid 기대값은 content inline `50px`, block `10px`으로 분리했다. 최종 track `[104] / [22]`를 Rust 및 두 앱에서 확인했다. |
| 2 | 2px border가 percentage basis에 포함되는가? | root border box `54×14px`와 content basis `50×10px`을 분리했다. Chrome 사전 관측, Rust frame 및 두 simulator가 일치한다. |
| 3 | intrinsic sizing 중 cyclic minimum을 root 크기보다 먼저 resolve하는가? | 사전 계산은 cyclic minimum을 intrinsic phase에서 auto처럼 다루고 이후 content basis로 resolve한다. 양축 track trace와 root frame이 이 분리된 결과를 보였다. |
| 4 | `calc(200% + 4px)`의 column 산술이 다른 축의 길이 상수를 쓰는가? | column 입력과 독립 expected value를 고정했다. 양축 scene과 column-only cyclic scene 모두 `[104px]`다. |
| 5 | `calc(200% + 2px)`의 row 산술이 column 상수를 잘못 쓰는가? | row 입력·expected 값을 따로 고정했다. 양축 scene의 row `[22px]`, row-only cyclic scene의 `[42px]`가 일치한다. |
| 6 | definite control이 cyclic 경로를 우연히 재사용하는가? | 양축 definite root control은 `[24] / [22]`, `14×14px` root로 두 축 minimum floor를 확인한다. |
| 7 | 단일 definite-row 대조가 column 외 결과까지 바꾸는가? | `10px` definite row control에서 columns `[104]`, rows `[10]` 및 item `104×10px`을 두 플랫폼에서 확인했다. |
| 8 | 단일 definite-column 대조가 row wrapping을 생략하는가? | `10px` column에서 Ahem `aa aa`가 두 줄 intrinsic block size를 만들고 row `[42]`가 된다. item frame은 `10×42px`다. |
| 9 | text intrinsic 기대가 시스템 폰트에 따라 달라지는가? | Rust test는 system font 없이 Ahem fixture를 등록한다. inline min/max-content `20/50px`를 각각 assert한다. |
| 10 | 공백 collapse나 wrapping 설정 때문에 의도한 두 단어 입력이 달라지는가? | fixture의 `white-space-collapse`, `text-wrap-mode`, line-height와 text `aa aa`를 고정하고 Chrome/Rust/runtime 입력을 대조했다. |
| 11 | Rust direct-layout test가 최종 stage가 아닌 중간 track 크기를 읽는가? | 각 축의 마지막 pass와 마지막 stage를 골라 base size를 확인한다. gutter/collapsed track을 제외한 track 수가 정확히 1개여야 한다. |
| 12 | trace에 누락된 축이 있어도 test가 통과하는가? | 네 scene 모두 Columns와 Rows trace를 별도로 요구한다. 축별 expected size 비교 오차는 `0.05px` 이하로 제한한다. |
| 13 | item frame은 맞지만 root border box가 틀리는 경우를 놓치는가? | Rust test, Android node frame, iOS node frame에서 root와 item의 x/y/width/height를 각각 검사한다. |
| 14 | FFI HTML source가 track CSS나 root dimension을 잘못 serialize하는가? | 여덟 scene의 FFI source test가 scene ID별 columns, rows, root sizing, border, text와 두 element 수를 확인한다. |
| 15 | Rust string formatter가 text를 JavaScript 문자열로 깨뜨리는가? | 기존 단어 scene은 `a`, 신규 scene은 `aa aa`를 예상 source와 대조한다. 이번 고정 text에는 quote/escape 문자가 없으며 임의 사용자 문자열 admission은 범위 밖이다. |
| 16 | 새 Rust layout test가 제품 경로에 들어가는가? | 모듈 선언은 `c11-4-grid-track-sizing-fixture-test` feature로 gate되어 있다. 기본 release archive 검사와 opt-in positive control을 함께 기록했다. |
| 17 | scene ID 중복이나 배열 크기 갱신 누락으로 일부 scene이 shadow되는가? | runtime `SCENES` ID 집합은 길이와 유일성을 assert한다. FFI scene list와 capture scene registry도 네 신규 ID를 각각 포함한다. |
| 18 | 미등록 sibling scene이 허용 목록을 우회하는가? | exact scene ID lookup과 미등록 ID negative assertion을 유지한다. 신규 definite-column cyclic prefix는 runtime에서 명시적으로 unsupported 처리한다. |
| 19 | C11.4 runtime parser에는 등록했지만 앱 route에서 거부되는가? | Android 및 iOS의 C11.4 route allowlist에 같은 네 ID를 등록하고 실제 simulator 실행에서 모두 dispatch했다. |
| 20 | Android root expectation이 definite control에서 공통 fallback을 사용해 오판하는가? | 첫 전체 행렬이 `100×40px` fallback 기대값 오류를 드러냈다. `14×14px` 분기 수정 후 새 APK로 전체 4 scene을 재실행했다. |
| 21 | iOS root expectation이 Android와 다른 geometry를 허용하는가? | Swift expected root와 Rust/Java 값이 네 ID 각각에서 일치하고 iPhone 17 Pro Simulator 4/4가 통과했다. |
| 22 | item dimensions는 맞지만 node origin 또는 box count가 다른가? | 양 플랫폼 node frame에는 root `(0,0)`과 item `(2,2)`를 포함한다. 네 scene 모두 정확히 2개의 box를 요구한다. |
| 23 | Android PixelCopy 표본이 root/item target ID를 바꾸거나 중복시키는가? | 표본 validator가 scene별 ID, target node, CSS 좌표, 중복 및 누락을 거부한다. 최종 실행은 28/28 exact다. |
| 24 | iOS Metal 표본 목록과 색 기대가 Android 표와 어긋나는가? | Swift 표본별 node/좌표/RGBA를 캡처 script와 대조했다. drawable readback은 compositor 제출 전 28/28 exact다. |
| 25 | red 표본이 실제 item 내부에서만 찍혀 overflow를 검증하지 못하는가? | item의 오른쪽·아래쪽 root border 구간과 root 바깥의 실제 item paint를 별도로 표본화한다. 문서에서는 border stroke 색 검증과 혼동하지 않는다. |
| 26 | item 직후의 검정 표본 좌표가 아직 item 내부인가? | 이전 `x=25`가 24px definite item 안쪽임을 실제 표면에서 확인했다. 양 플랫폼 및 script에서 첫 outside coordinate `x=27`로 고쳐 재실행했다. |
| 27 | scene 표본이 화면 밖 좌표 또는 잘못된 sample node로 통과하는가? | Android와 iOS sample target은 root/item frame 및 허용 scene과 연결된다. definite control의 right/bottom outside 표본은 각각 `x=27`, `y=25`다. |
| 28 | 이전 APK를 최신 소스 결과로 오인하는가? | 초기 AVD가 stale APK SHA를 실행한 것을 기록했다. fixture build 후 local/installed APK SHA가 `b4586f…610b`로 같은 최종 run만 통과 근거로 채택했다. |
| 29 | fixture 경계 검사가 전체 앱 번들 제외를 과장하는가? | 기본 `spinon-ffi` archive의 문자열 검사와 opt-in positive control만 주장한다. 이를 APK/IPA fixture 제외로 확대하지 않는 제한을 artifact JSON과 README에 적었다. |
| 30 | 부분 fixture 결과를 일반 CSS 지원·WPT·전체 C11.4 완료로 표현하는가? | STATUS와 실행 README는 고정 fixture 및 simulator 증거로 한정하고 WPT, 일반 CSS admission, 실기기, hardware GPU, iOS compositor 이후 픽셀을 미검증으로 유지한다. C11.4 체크와 `72/213` 완료 수는 바꾸지 않았다. |

**판정:** 검토 중 발견한 stale APK 실행 경로, Android pixel-scene 등록 누락, definite control root 기대값 및 item 밖 경계 좌표를 수정한 뒤 영향받는 전체 4-scene Android/iOS 행렬을 다시 통과시켰다. 남은 범위는 30번에 적은 대로 고정 fixture 밖의 동작이다.

31. **PR 생성 후 roadmap 참조와 공개 screenshot URL이 실제 PR을 가리키는가?** PR #493 계획 링크가 해당 slice를 가리키고, Agent Browser에서 Android 1080×2400 및 iOS 1206×2622 이미지 두 개의 `complete=true`와 `naturalWidth>0`을 확인했다. cleanup 뒤 지정 namespace의 session 목록은 비었다.
