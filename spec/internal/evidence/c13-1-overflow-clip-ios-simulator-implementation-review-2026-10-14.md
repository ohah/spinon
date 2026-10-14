# C13.1 iOS Simulator integration · 구현 실패 관점 20개

대상은 C13.1 opt-in Cargo/Xcode 연결, Swift fixture route, 보고 검증, Simulator evidence다. Rust production layout code는 변경하지 않았다. 아래 판단은 [iOS Simulator 실행 근거](c13-1-overflow-clip-ios-simulator-2026-10-14/README.md), default/feature-on archive 대조, 현재 source diff를 사용했다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | 기본 빌드 설정이 C13.1 feature를 실수로 켠다 | PASS — Debug/Release Xcode default가 0이고 새 build env flag도 0을 기본값으로 쓴다. Cargo feature 목록에는 flag=1일 때만 추가된다. |
| 2 | `true`, 빈 값, `2` 등 잘못된 flag가 켜진 것으로 처리된다 | PASS — simulator script는 0/1 외 입력을 exit 2로 거부했다. |
| 3 | C13.1이 C04.10 runtime GPU 없는 구성에서 링크된다 | PASS — simulator/native scripts가 C04 prerequisite를 검사하고 두 negative run이 exit 2였다. ObjC++ host/eval도 두 macro를 함께 요구한다. |
| 4 | `mise` 유무에 따라 xcodebuild에 flag 전달이 달라진다 | PASS — 두 xcodebuild 분기에 같은 설정을 전달한다. |
| 5 | iOS native build가 C13 fixture source를 Cargo에 전달하지 않는다 | PASS — C13=1 build log의 ffi feature 목록에 `c13-1-overflow-clip-fixture-test`가 나타났다. |
| 6 | C11/C04 fixture를 포함하는 이유로 C13 payload도 기본 archive에 섞인다 | PASS — feature 없이 빌드한 iOS archive에서 C13 FFI symbol 2개와 fixture string이 모두 없고 opt-in archive에는 모두 있다. |
| 7 | Rust feature 이름과 Xcode C macro 이름이 어긋난다 | PASS — opt-in build log에 Cargo feature와 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1` compile define이 각각 확인됐다. |
| 8 | C header는 Rust symbol 선언을 숨기지만 ObjC++가 무조건 링크한다 | PASS — header declaration 및 wrapper reference가 전처리 macro 안에 있고, default archive에는 대상 symbol이 없다. |
| 9 | simulator build와 native build가 C13 flag를 서로 다르게 해석한다 | PASS — simulator env flag를 native build phase로 전달하고 Cargo feature를 같은 값으로 선택한다. |
| 10 | C13 실행 인자가 기존 C0410 route에서 무시된다 | PASS — `AppDelegate`가 해당 인자를 runtime GPU controller 진입 조건으로 인식한다. 실행 로그에 C13 host가 생성됐다. |
| 11 | C13 인자와 C12/C11 인자를 함께 주면 우선순위에 따라 다른 fixture가 실행된다 | PASS — Swift 초기화가 충돌하는 `--spinon-c*` 인자를 host 생성 전에 거부했다. C12.3 동시 선택은 negative log에서 확인했고 C13 host는 생성되지 않았다. |
| 12 | 우연한 다른 fixture boolean이 C13 host를 앞선다 | PASS — exclusive route check 뒤 C13 host와 eval selection이 가장 먼저 처리된다. |
| 13 | iPhone logical size가 Chrome reference viewport와 달라진다 | PASS — C13 route는 canvas/viewport를 320×240 CSS px로 고정했고 runtime log에 같은 값이 남았다. |
| 14 | display scale 3을 CSS viewport 크기로 잘못 섞는다 | PASS — viewport는 320×240 CSS px이고 renderer surface는 960×720 texel로 별도 기록됐다. Chrome geometry 대조는 CSS px로 13/13 exact였다. |
| 15 | report buffer가 hit-test JSON을 담지 못해 성공 report가 잘린다 | PASS — 실제 FFI eval이 status 0으로 13 frame과 15 sample을 전달했고 Swift가 sample JSON 전체를 decode했다. |
| 16 | frame log parser가 뒤의 paint/hit JSON 괄호까지 frame으로 읽는다 | PASS — C++ frame helper는 `node_frames_css_px` 뒤 첫 `]`를 기준으로 자르고, Swift helper도 같은 종료 규칙을 쓴다. |
| 17 | `boxes=13`만 검사해 실제 frame 누락을 숨긴다 | PASS — Swift route는 `boxes=13`과 frame list count 13을 따로 요구하고, 독립 parser도 frame inventory 전체를 Chrome과 비교한다. |
| 18 | hit sample 1개가 빠지거나 순서/ID가 달라도 summary가 통과한다 | PASS — 15개 고정 ID 순서, sample count, 각 `matched` 상태를 검사한다. 독립 대조는 ID·좌표·element ID를 Chrome JSON과 비교했다. |
| 19 | viewport 밖 결과가 root element여도 count만 맞아 통과한다 | PASS — 네 고정 outside sample의 element가 `null`이어야 Chrome oracle 대조와 FFI matched 검증이 통과한다. 실행 결과는 4/4 null이다. |
| 20 | Simulator 화면 캡처를 drawable readback/scanout 증거로 과장한다 | PASS — evidence는 surface draw와 화면 캡처로 범위를 제한하고 RGBA readback, compositor 완료, scanout을 명시적으로 제외한다. |

남은 범위는 전체 C13.1 비교 matrix, broader paint/hit/event negative controls, WPT 분류, scroll/CSSOM과 실제 display/readback 관측이다. 이 검토는 C13.1 또는 C13 완료 판정이 아니다.
