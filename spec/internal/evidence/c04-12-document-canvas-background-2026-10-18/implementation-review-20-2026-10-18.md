# C04.12 문서 Canvas 배경 구현 후 실패 관점 검토 20회

**검토 대상:** root/body computed background 선택, system-color 사전 검사, cascade snapshot 식별, 오류 전달, 앱 content root 연결.
**근거:** [`cargo-test.log`](cargo-test.log), [`cargo-clippy.log`](cargo-clippy.log), [Android 실행](android-logcat.txt), [iOS 실행](ios-simulator.log), [Chrome oracle](README.md).

| # | 독립 실패 관점 | 확인 결과와 근거 |
|---:|---|---|
| 1 | 불투명 html과 불투명 body의 우선순위 역전 | html 색을 먼저 고르는 resolver와 `root_html_canvas_prefers_opaque_html_background_then_propagates_body_background` 테스트에서 html `(18,52,86)`이 선택됐다. |
| 2 | 투명 html에서 body 색 전파 누락 | stylesheet 및 `var()`를 통한 body 값이 computed snapshot을 거쳐 Canvas 결과 `(30,140,70)`으로 연결되는 세션 테스트가 통과했다. |
| 3 | body 노드가 없는 입력의 fallback | resolver의 `None` body 분기에서 선택 scheme 색을 사용한다. HTML parser는 일반 document에서 body를 삽입하므로 직접 unit 입력으로 이 경계를 검사한다. |
| 4 | html과 body가 투명할 때 light/dark 반전 | 두 scheme의 fallback이 각각 `#ffffff`와 `#121212`로 고정됐고 resolver 테스트가 모두 통과했다. |
| 5 | 선택되지 않는 body partial alpha가 html Canvas를 막음 | 초안 resolver가 root가 opaque여도 body alpha를 검사해 실패하는 결함을 발견했다. body 검사를 root-transparent 분기로 이동하고 실제 `RuntimeSession` 회귀 테스트를 추가해 html `(18,52,86)`이 계속 선택됨을 확인했다. |
| 6 | alpha 0 body 값을 opaque 색으로 취급 | sRGB의 zero-alpha body 색을 투명으로 분류하고 dark fallback `#121212`를 쓰는 새 end-to-end 세션 테스트를 추가했다. |
| 7 | 선택되는 root/body partial alpha의 임의 반올림 | 선택 후보의 translucent variant는 `UnsupportedComputedBackgroundColor`(-15)로 끝나며 output 색을 만들지 않는다. |
| 8 | f32 오차를 RGBA8 채널의 잘못된 반올림으로 확대 | 정수 byte 경계 허용 검사를 확인했다. Chrome fractional sRGB sample은 pixel channel 229와 CSSOM 환산 230이 달라 지원 후보에서 제외됐다. |
| 9 | HSL/HWB/wide-gamut 입력을 sRGB로 조용히 변환 | Stylo의 computed source color-space tag를 검사해 sRGB가 아닌 입력을 거부한다. HSL, HWB, display-p3 계열은 지원 결과로 노출하지 않는다. |
| 10 | NaN·무한대·범위 밖 component가 byte 변환에 도달 | `ComputedCssColor::from_absolute_color`의 source/변환 유한성 검사와 Canvas 경로의 alpha/channel 범위 검사를 확인했다. |
| 11 | Stylo가 계산 중 제거한 CSS system color 키워드 | computed value만 보는 대신 parsed declaration을 검사한다. `Canvas`, `CanvasText` 및 custom property 전달 사례가 stylesheet·inline 테스트에서 거부됐다. |
| 12 | `var()` 등 함수 안에 숨긴 system-color identifier | `cssparser` nested function/block 재귀가 포함돼 있다. malformed token/block은 허용하는 대신 fail-closed한다. |
| 13 | 문자열·주석의 `Canvas` 글자를 실제 색상으로 오인 | 문자열 token 내부를 검색하지 않는 회귀 테스트가 통과했다. |
| 14 | media query 내부 stylesheet 선언 누락 | `CssRule::Media` 자식 규칙까지 검사한다. 활성·비활성 분기에서 system color 입력을 안전하게 거부하는 보수적 profile이다. |
| 15 | inline style의 system color가 stylesheet 검사만 통과 | 별도 inline traversal가 `background-color`와 custom property를 검사하고, 두 유형의 거부 테스트가 통과했다. |
| 16 | gradient/image를 평탄한 Canvas 단색으로 취급 | background-image는 root profile의 allowlist 밖이다. fixture는 linear-gradient가 cascade failure가 되는 것을 확인한다. |
| 17 | foreign namespace나 HTML 외 요소를 body로 선택 | body ID 추출은 HTML namespace의 root 직속 `body`만 인정한다. 노드가 없으면 scheme fallback으로 내려간다. |
| 18 | synthetic html/body box validator의 과도한 완화 | 배경 허용은 `RuntimeFlexCustomPropertiesMediaEnvironmentV1` root HTML profile에 한정됐다. 다른 layout profile의 검증 경로는 그대로 유지된다. |
| 19 | 이전 document/style/environment snapshot과 새 색 혼합 | cascade key와 style snapshot의 generation·document·render-tree·style·environment revision을 모두 비교한 뒤에만 선택한다. |
| 20 | 실패 뒤 C ABI output buffer 부분 갱신 | FFI는 session resolver 성공 뒤에만 `canvas.write`를 한다. 새 -15와 헤더/0077 설명을 연결했고, 오류 반환 시 출력 구조체에 쓰지 않는 흐름을 확인했다. |

## 판정

20개 관점을 확인했다. opaque root와 partial-alpha body가 함께 있을 때 body alpha가 선택되지 않는 root Canvas까지 차단하던 결함을 수정했다. alpha 0 body 검증도 추가했다. 관련 runtime/style 전체 테스트와 Clippy가 통과했다. Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator에서 최신 구현을 실행해 Canvas 색과 화면 표본을 확인했다. 전체 HTML body box paint, system UI theme, compositor equivalence, 실기기는 이 검토의 통과 조건이 아니다.
