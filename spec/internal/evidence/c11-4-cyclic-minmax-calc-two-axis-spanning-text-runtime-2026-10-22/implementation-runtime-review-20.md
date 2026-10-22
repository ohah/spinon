# C11.4 양축 cyclic `minmax(calc())` text runtime 구현 적대 검토 20개

계획 검토와 분리해 구현 코드와 앱 실행 경로의 실패 관점을 확인했다. 각 항목은 고정 fixture 코드, Rust 추적 또는 Android/iOS Simulator 결과와 대조했다. 이 검토는 네 scene의 제한 profile에만 적용하며 일반 Grid 적합성 판정은 아니다.

1. **기본 runtime으로 fixture 코드가 유입되는가?** `spinon-runtime` 경로는 `c11-4-grid-track-sizing-fixture-test`에 묶여 있다. 기본 `spinon-ffi` archive에서 네 scene ID, 오류 marker, Ahem 등록 marker가 없고 opt-in positive control에 포함됨을 artifact 경계 기록으로 확인했다.
2. **기존 cyclic calc 거부 경로가 새 scene을 먼저 가로채는가?** 새 네 ID 분기를 기존 cyclic calc handler 앞에 두었다. Android와 iOS 각각 네 scene 모두 성공해 dispatch 순서가 실제 앱 경로와 맞는다.
3. **scene ID를 재사용한 잘못된 root가 통과하는가?** root `div`, 문서 root 직속 소유, 고정 ID, 빈 `data-node`, track-sizing profile을 검사한다. 조건이 어긋나면 `unsupported_c114_two_axis_span_text_fixture`로 실패한다.
4. **추가 item이나 잘못된 자식 순서가 contribution 계산을 바꾸는가?** root는 direct child 하나, item은 direct Ahem text node 하나만 허용한다. text content와 `HostParent`도 검사한다.
5. **네 scene 이외의 width/변형이 허용되는가?** FFI·runtime 두 층에서 고정 scene 목록만 노출한다. allowlist 단위 테스트는 세 가지 비등록 ID를 거부한다.
6. **track 문법이 느슨하게 해석되는가?** typed style은 각 축 두 track, `calc(10% + 2px)`와 `max-content` 조합만 통과시키며 auto-repeat, template areas, line names는 거부한다. Android/iOS 결과 모두 Columns/Rows에 두 track만 기록했다.
7. **cyclic과 definite control의 입력이 우연히 같아지는가?** 두 cyclic 장면은 intrinsic width, 두 control은 10px/60px definite width를 각각 고정한다. 네 장면의 track·frame 결과는 대응하는 control과 같고 각자 예상한 content-box 크기를 낸다.
8. **text가 이미지/nested Grid 측정 입력으로 오인되는가?** 직접 text node만 읽고 `Ahem`을 system font 없이 등록한다. Rust 테스트와 runtime diagnostics에서 text source, min-content 10px, max-content 60px를 확인했다.
9. **word wrapping 결과가 column width와 무관하게 재사용되는가?** `break-all`, 10px line-height를 고정하고 Columns pass 이후 Rows pass 순서를 검사한다. 10px item 폭에서는 row contribution 60px, 60px item 폭에서는 10px로 캡처됐다.
10. **양축 span이 한 축에만 적용되는가?** item placement는 두 축 모두 line 1→3이며 intrinsic span은 `[2]`다. Rust trace는 Columns와 Rows 각각 span-2 min/max contribution을 확인한다.
11. **percentage rerun이 앞서 얻은 content base size를 낮추는가?** Taffy rerun은 percentage 최소 함수와 intrinsic 최대 함수 조합에서 기존 base를 보존한다. cyclic 장면의 최종 track은 `[4,4]`/`[29,29]`, row는 `[29,29]`/`[4,4]`로 검증했다.
12. **percentage 최대 함수에도 보존 규칙이 새어 나가는가?** 보존 분기는 최대 함수가 intrinsic일 때만 실행된다. 기존 cyclic calc focused 회귀 테스트 7개가 통과했다.
13. **root min-content를 다른 root 유형이나 축에 적용하는가?** available-space 변환은 root `display:grid`와 `width:min-content`일 때만 작동하고 전달된 height는 유지한다. projection은 track-sizing profile의 root Grid 경계로 입력을 제한한다.
14. **gutter와 border-box 계산이 item frame을 밀어내는가?** 2px row/column gap, 2px root border, 0 padding/margin을 고정했다. 네 scene 모두 item origin `(2,2)`이고 border-box root는 `14×64` 또는 `64×14`다.
15. **Android 표본이 border 안쪽만 재서 overflow 경계를 놓치는가?** PixelCopy는 scene마다 left/item/right/outside/bottom/past-root 6점을 확인한다. 성공 run에서 네 scene 24/24 표본이 exact다.
16. **iOS 표본이 다른 surface나 compositor 결과로 오인되는가?** 고정 iPhone 17 Pro/iOS 26.2 Simulator의 Metal drawable에서 scene마다 6점을 읽고 surface generation을 연결했다. 24/24 exact이며 compositor 제출 전 readback이라고 한정했다.
17. **scene 완료 전에 draw가 누락되거나 이전 generation이 섞이는가?** Android/iOS 네 scene 모두 `drawPresented=true`, `supersededRetries=0`, `boxes=2`다. 각 scene의 실행 로그에 layout generation·document revision을 기록했다.
18. **기기 설치본과 검사한 앱이 다른가?** Android API 37 ARM64 AVD의 local/installed APK SHA-256이 일치한다. iOS 결과는 같은 iPhone 17 Pro Simulator 앱 SHA-256을 manifest에 기록했다.
19. **픽셀 통과를 glyph raster 또는 실기기 결과로 확대 해석하는가?** 앱 화면은 빨간 item box와 sample 요약을 보여주며 runtime log의 `runtime_text_count=0`이라 glyph paint를 입증하지 않는다. AVD/Simulator 결과만 기록하고 실기기·hardware GPU·iOS compositor 이후 결과는 주장하지 않는다.
20. **C11.4 전체 완료로 오해할 상태 변경이 있는가?** 이 slice는 고정 2×2 explicit Grid와 한 text item에 한정한다. 상태 대장의 C11.4 체크박스와 `77/213` 집계는 그대로 유지한다.

## 재현 경로

- Android API 37 ARM64 AVD run: `android-avd-c114-cyclic-minmax-calc-two-axis-spanning-text-android-avd-20261022T074336156Z-30579381/`
- iPhone 17 Pro / iOS 26.2 Simulator run: `ios-simulator-c114-cyclic-minmax-calc-two-axis-spanning-text-ios-simulator-20261022T074352914Z-9d6794aa/`
- Default/opt-in archive 비교: `artifact-boundary/`
- 구현 전 oracle: `chrome-agent-browser/precomparison.png` 및 `observation.json`
