# C11.4 visibility-hidden border overflow · 구현 PR diff 실패 관점 30개

이 검토는 계획 diff나 runtime 20개 표를 반복하지 않고, 변경 파일 사이의 연결·근거·artifact·PR 표시 경계를 대상으로 한다. 최종 GitHub PR screenshot 렌더 항목은 PR 생성 뒤 Agent Browser에서 확인해 닫는다.

| # | 변경 diff에서 확인한 실패 경로 | 결과 |
|---:|---|---|
| 1 | C11.4 FFI 생성기에 네 ID 중 하나가 빠져 앱 stage만 scene을 요청한다. | `runtime_source` exact ID table 및 FFI eight-scene route test가 네 ID 모두 포함한다. 통과. |
| 2 | 기존 `visibility:hidden` 4개 scene이 새 문자열 분기 때문에 기존 HTML을 생성하지 않는다. | 기존 scene branches와 신규 branches가 공존하고 전체 matrix에서 기존 14개가 다시 통과한다. |
| 3 | visible control ID가 hidden span의 CSS 값으로 생성된다. | `visibility`는 ID에서 결정하며 capture manifest와 sample 색이 hidden/control을 구분한다. 통과. |
| 4 | border shorthand가 parser에 지원되지 않는 border-image 값을 만들어 fixture가 실패한다. | HTML source는 `border-width/style/color` longhand만 사용하며 computed border 계약을 검사한다. 통과. |
| 5 | root border width/height를 content size와 혼동한다. | Independent expected JSON, runtime root frame, Python matcher가 border box와 content track size를 각각 기록한다. 통과. |
| 6 | root border 없는 legacy route에서 `116px` fixed cross track을 허용한다. | Root style contract의 `border_overflow`는 exact scene mapping에서 전달되고 양방향 legacy-style mutations가 거부된다. |
| 7 | C11.4 fixture geometry가 scene ID 밖의 스타일을 통해 일반 author CSS API로 노출된다. | Public API/stylesheet admission을 추가하지 않고 기존 opt-in FFI route 아래 exact fixture only로 제한했다. |
| 8 | root style 검증 확장으로 mixed flex fixture 책임 파일이 500줄을 넘는다. | root contract를 별도 내부 모듈로 분리했다. 부모 398줄이며 helper는 private 경계를 유지한다. |
| 9 | FFI unknown ID가 fallback CSS source를 받아 generic layout으로 성공한다. | FFI exact route test가 unknown/suffixed ID를 거부하고 전체 run manifest도 rejected route controls를 확인한다. |
| 10 | 신규 Rust test 하위 모듈이 컴파일 또는 capture hash 입력에서 빠진다. | `tests/visibility_hidden/border_overflow.rs` module path와 capture source hash list를 함께 갱신했다. focused test 통과. |
| 11 | Clippy 수정을 위해 unrelated fixture operation 순서나 값이 바뀐다. | `map_identity` iterator만 제거하고 opt-in runtime Clippy 및 focused tests를 다시 통과했다. |
| 12 | `TextLayoutHeightMismatch` 예외가 일반 block text나 non-zero grid cell까지 넓어진다. | Guard는 content height 0, `grid_items` membership, parent `TrackSizing` mode를 모두 요구한다. Layout 308개와 Row fixture test 통과. |
| 13 | 앱 C11.4 dispatcher가 새로운 IDs를 native stage에 전달하지 않는다. | Android/iOS dispatcher allowlists를 각각 같은 네 ID로 갱신하고 두 앱에서 18/18 실행했다. |
| 14 | Java와 Swift가 신규 root size 또는 item frame을 서로 다르게 기록한다. | 두 platform table이 columns `100×120` / rows `120×100`, item frames 및 5 frame records로 일치한다. |
| 15 | visible control의 zero-area C item 때문에 expected draw box 수가 하나 증가한다. | Hidden 3 / visible 4 draw count를 frame count 5와 분리했고 PixelCopy/Metal presentation assertion을 통과했다. |
| 16 | Border scenes에서 fixed-axis pass가 옛 `120px`을 기대한다. | Java/Swift fixed-axis matcher는 신규 scene에서 `116px`이고 final trace matcher도 양 축에 이를 확인한다. |
| 17 | 신규 tracks를 기존 visible scene의 `[150,40,90]`로 잘못 읽는다. | 두 platform flexible tables, expected JSON 및 trace verifier가 `[150,40,0]`을 요구한다. |
| 18 | Negative remaining free space를 0으로 clamp해 계산 오류를 숨긴다. | 독립 checker는 content `96 - 150 - 40 - 20 = -114px`를 보존하고 실제 trace와 비교한다. |
| 19 | Rust Debug 직렬화의 무해한 공백 차이가 iOS만 실패하거나, 반대로 모든 텍스트를 느슨하게 비교한다. | AutomaticMinimum 필드만 whitespace-normalize하고 enum/field/value는 exact 비교한다. 최종 양 플랫폼 matrix 통과. |
| 20 | Overflow PixelCopy가 hidden span 자체를 target node로 지정해 숨김 성공을 오판한다. | Hidden 표본의 target은 A item, visible control 표본은 span이며 node ID와 expected RGBA를 모두 matcher가 검사한다. |
| 21 | Border 표본이 item에 가려진 오른쪽/아래 edge에서 합성 색으로 달라진다. | Border sample은 가리지 않는 왼쪽/위쪽 edge `(1,60)` / `(60,1)`을 사용한다. 네 값 모두 exact. |
| 22 | Root 밖 sample의 CSS viewport 좌표를 drawable 좌표로 그대로 사용한다. | PixelCopy/Metal log의 viewport·drawable scale로 root-local point를 변환하고 정확한 pixel coordinates를 다시 검증한다. |
| 23 | iOS drawable 검증을 compositor 이후 화면 또는 전체 화면 parity로 과장한다. | README와 plan은 제출 전 Metal readback의 두 point samples로 범위를 제한한다. |
| 24 | Android scene 화면 캡처가 다른 설치 APK 또는 stale source에서 나온다. | Manifest는 APK와 설치본 SHA 일치, V8 revision, source commit/tree/diff 및 file-set hash를 기록한다. |
| 25 | iOS simulator 실행이 다른 기종/iOS runtime에서 얻어진다. | Manifest는 iPhone 17 Pro/iOS 26.2 UDID와 app SHA, pre-boot 상태를 기록한다. |
| 26 | Capture harness가 기존 14개만 선택해 신규 scene을 pass로 누락한다. | Script가 precomparison `expected.json`의 네 scene을 추가하고 모든 scene을 선택해 18/18 manifest를 만든다. |
| 27 | Python verifier가 14개 ID set만 받아 new screenshot/trace를 무시한다. | `SCENE_IDS`를 exact 18개로 확장하고 두 final manifest, frames, target tracks, trace, sample 및 screenshot SHA를 검사한다. |
| 28 | Verifier negative controls가 오직 오래된 hidden overlap만 검사한다. | 신규 hidden border-outside blue→purple 누출과 1px sample 이동을 별도 negative controls에 추가했다. 17 controls가 거부됐다. |
| 29 | Fixture marker가 기본 product static archive에 들어가거나 positive control에도 없다. | `artifact-boundary.json`은 기본 archive 0 marker/0 symbol, opt-in 4/4 marker/2 symbol을 hash와 함께 기록한다. APK/IPA 제외라고 확대하지 않는다. |
| 30 | PR 본문 screenshot URL이 깨졌거나 이미지가 표시되지 않은 채 merge된다. | PR #747을 Agent Browser로 열어 두 이미지의 `complete=true`, 표시 상태, natural dimensions를 확인했다. Android `1080×2400`, iOS `1206×2622`; 두 raw URL 모두 정상 로드. [브라우저 확인 기록](./pr-747-agent-browser-image-verification.json). 통과. |

30개 diff 실패 관점 검토를 마쳤다. PR merge 전 required checks와 merge 정책은 별도로 확인한다.
