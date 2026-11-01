# C11.1 구현·runtime 실패 관점 검토 20개

**대상:** base `f15b5de56669acc47afc6f26ca9707bf23c3cea5` 위의 Root HTML fixed-px Grid gap 후보 및 최종 pass03 Android/iOS Simulator 실행. 계획 검토 20개와 구분한 구현 후 검토다. 병렬 reviewer를 사용하지 않고 primary agent가 코드·테스트·앱 실행을 대조했다. 각 항목은 다른 입력·경계 또는 실패 경로를 확인한다.

| # | 공격·실패 관점 | 실제 확인과 결과 |
| --- | --- | --- |
| 1 | Root HTML 경로 대신 작업별 fixture FFI가 화면을 통과시킨다. | 두 플랫폼 앱 build flags에서 C04 runtime GPU는 켜고 C04 runtime GPU fixture-test 및 C11 Grid fixture는 껐다. 로그의 route는 `source=index.html`이고 `root_app_marker=ready`다. 기본 `new_runtime_gpu_with_root_html()` integration test도 동일 entry를 호출한다. 통과. |
| 2 | `gap` allowlist 추가가 다른 runtime profile의 author admission까지 넓힌다. | property 추가는 `RuntimeRootHtmlTextPaintV1` 목록에 한정된다. 진단 완화도 cascade profile이 정확히 이 값일 때만 작동한다. 코드 경계 확인. |
| 3 | `gap` 첫 번째 값을 column에 연결해 축을 뒤집는다. | 독립 기대값 및 assertion에서 첫 값은 row 5px, 둘째는 column 10px다. computed typed values와 네 item frame 모두 exact. 통과. |
| 4 | `row-gap`·`column-gap`이 shorthand와 다른 typed longhand에 연결된다. | 두 longhand로 교체한 동일 fixture가 shorthand와 같은 네 frame을 만든다. 통과. |
| 5 | 양수 gap이 parse recovery의 negative 예외에 잘못 들어가 버려진다. | diagnostic helper는 fixed-px 값 중 음수가 있을 때만 매치하며 positive declaration은 negative case test와 별도로 거부-control이다. 실제 positive shorthand/longhand 통과. |
| 6 | `gap` 이외 CSS 진단까지 recovery가 무시한다. | `color: bad`, `display: grid`의 다른 parser error, negative `em`, `calc()`는 helper negative control에서 매치하지 않는다. 다른 parser 진단은 기존 fail-closed cascade로 전달된다. 통과. |
| 7 | CSS-invalid `gap: -1px 10px`에서 두 번째 유효해 보이는 값이 column gap으로 새어 나온다. | 선언 전체를 버린 뒤 두 번째 item x=40px임을 확인한다. invalid shorthand의 10px 성분은 publish되지 않는다. 통과. |
| 8 | invalid inline negative gap 진단을 무시하면서 정상 author stylesheet gap도 함께 지운다. | stylesheet shorthand는 유지하고 inline `gap: -1px 10px`만 버린다. 네 frame이 원래 5px/10px 장면으로 남고 computed diagnostics는 비어 있다. 통과. |
| 9 | CSS property나 `!important` 대소문자 차이가 음수 선언 복구를 우회한다. | uppercase `GAP`, `PX`, `!IMPORTANT` unit control 및 실제 integration source를 넣었다. 선언은 버려지고 zero-gap 장면이 유지된다. 통과. |
| 10 | 값 중간 CSS comment가 negative declaration 진단 matcher를 우회한다. | `gap: -1px/**/ 10px`에서 실제 runtime 테스트가 cascade failure를 재현했다. 진단의 CSS comment를 구분자로 제거한 뒤 값을 판별하도록 수정했고 같은 입력의 zero-gap scene이 통과한다. 실패 경로와 수정 확인. |
| 11 | CSS comment로 감싼 `!important` 토큰 때문에 음수 선언이 남는다. | helper unit diagnostic `gap: -1px 10px !/**/IMPORTANT`가 recovery 대상으로 분류된다. 통과. |
| 12 | percentage gap을 px로 오인하거나 다른 axis 오류로 보고한다. | `column-gap: 10%`는 `unsupported_computed_value` 및 `column-gap` diagnostic으로 거부되고 partial scene이 없다. 통과. |
| 13 | math-backed `calc()`가 resolved px처럼 projection을 통과한다. | `row-gap: calc(1px + 1%)`는 `row-gap` diagnostic과 함께 layout publication 전에 거부된다. projection은 `layout_math_values`도 확인한다. 통과. |
| 14 | 계산 스타일에 이미 음수 px가 들어왔을 때 `gap >= 0` 조건이 누락된다. | factored admission predicate의 직접 단위 테스트에서 `LengthPx(-0.5)`를 거부한다. 통과. |
| 15 | NaN 또는 무한대가 px gap으로 Taffy 입력에 들어가 track 좌표를 오염시킨다. | 같은 predicate test에서 NaN과 positive infinity를 거부한다. 허용 px는 finite 값뿐이다. 통과. |
| 16 | 기존 `normal` 및 zero-gap 장면이 새 authored 값 처리로 바뀐다. | predicate에서 `normal`과 0px을 허용하고 zero-gap runtime control의 네 frame을 기존 값과 exact 비교했다. 통과. |
| 17 | gap을 item frame에 사후 더해 Taffy track 계산과 중복 적용한다. | projection은 computed typed row/column gap을 `LayoutGap`에 한 번 전달하고, 별도 frame 보정은 없다. 독립 root/frame 기대값에서 track 합·gutter가 정확히 한 번 반영된다. 통과. |
| 18 | authored ID/class 또는 DOM 앞의 숨김 노드를 기준으로 하드코딩해 실제 selector/source order에 실패한다. | fixture selector와 ID를 class로 바꾸고 hidden prefix를 삽입해도 네 semantic paint frame이 그대로다. 통과. |
| 19 | 반복·implicit track·placement·nested Grid 같은 범위를 gap 지원과 함께 실수로 허용한다. | 기존 fail-closed matrix가 explicit placement, non-default auto-flow, repeat, `fr`/percentage track, named areas/lines, implicit row, nested Grid와 flex item 등을 각각 예상 코드로 거부하며 partial scene이 없다. 통과. |
| 20 | Host test만 통과하고 플랫폼 기본 앱 route나 실제 draw가 실패한다. | 최종 소스 build의 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 모두 layout ready/root 65px/7 boxes, draw/present 7 boxes를 기록했다. screenshots와 APK/executable SHA-256은 [platform observation](./platform-runtime-observation.json)에 연결했다. 캡처는 display screenshots이므로 surface pixel equality로 확대하지 않는다. |

## 판정

20개 관점에서 수정이 필요한 결함 1개(#10)를 고치고 동일 입력 runtime 검사를 통과시켰다. #19의 범위 제한과 #20의 캡처 한계는 그대로 유지한다. 이 review는 C11.1 전체나 상위 C11을 완료 처리하지 않으며 공식 완료 `72/213`을 변경하지 않는다.
