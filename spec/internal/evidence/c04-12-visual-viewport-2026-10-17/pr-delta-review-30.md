# C04.12 visual viewport PR diff 검토 30개

계획 검토 20개와 구현 검토 20개를 재사용하지 않고 최종 PR diff 전체를 별도 검토했다. 각 관점은 서로 다른 코드·ABI·fixture·문서 실패 경로를 대상으로 한다.

| # | 실패 관점 | 최종 diff 확인 결과 |
| --- | --- | --- |
| 1 | viewport height 계산이 visual height 공식을 그대로 남김 | resolved layout width와 surface aspect ratio의 곱으로 바뀌었다. 통과. |
| 2 | layout width가 `initial-scale` 후 screen 폭보다 작아짐 | `max(requested_width, screen_css_width / page_scale)` 규칙이 유지된다. 통과. |
| 3 | metadata 없는 모바일 overview 값이 바뀜 | 기존 기본 width/scale 값을 유지하고 기대 테스트를 보강했다. 통과. |
| 4 | `device-width` 초기 scale 1에서 visual snapshot 누락 | layout과 visual 크기가 각각 390×844로 반환된다. 통과. |
| 5 | 화면보다 좁은 fixed width 처리 회귀 | existing narrow fixed-width test에서 화면 폭으로 확장한다. 통과. |
| 6 | 화면보다 넓은 fixed width layout height 회귀 | Chrome fixture의 500×1082.0513 결과와 일치한다. 통과. |
| 7 | scale 0.5에서 page scale과 DPR 혼합 | layout/visual 크기, DPR 3, scale 0.5를 별도 assertion한다. 통과. |
| 8 | scale 2에서 visual viewport가 layout viewport와 같아짐 | visual 195×422, layout 390×844를 단위·앱 행렬에서 확인한다. 통과. |
| 9 | surface 종횡비 계산이 landscape에서도 수학적으로 반전 | 계산식은 입력 surface width/height 비율을 사용한다. 별도 landscape Chrome·Simulator run은 없고 계획 경계에 기록했다. |
| 10 | `(width)` media query가 visual 폭을 읽음 | CSS `CssViewport`에는 layout 폭이 계속 들어간다. 기존 projection test 통과. |
| 11 | `(min-height)`가 visual height를 읽음 | 500px fixed-width fixture에서 `123px` rule을 선택한다. Rust와 두 앱 fixture 통과. |
| 12 | `(max-height)`가 visual height를 읽음 | 같은 fixture에서 `456px` rule을 선택하지 않는다. Rust와 두 앱 fixture 통과. |
| 13 | 여러 viewport meta에서 요소 선택 순서 변경 | 마지막 요소 우선 기존 Chrome fixture/test를 보존한다. 통과. |
| 14 | scale 값 범위와 descriptor parse 범위가 넓어짐 | parser allowlist 및 0.01–10 scale 제한은 변경하지 않았다. 통과. |
| 15 | 0/NaN/무한 surface 입력이 snapshot을 게시 | 기존 surface 검증에 더해 visual 결과를 유한·양수로 확인하고 오류를 반환한다. 통과. |
| 16 | f32 height 곱셈 overflow가 Stylo에 전달 | 높이 `is_finite` 검사를 계산 직후 수행한다. 코드 검토 통과; 극단 입력 별도 실행은 없음. |
| 17 | cascade update가 layout 높이 대신 이전 visual 값을 사용 | session fixture가 media result를 통해 새 layout height를 확인한다. 통과. |
| 18 | environment revision이 layout 입력 차이를 반영하지 않음 | runtime의 기존 revision 경로를 유지하며 FFI Canvas key와 environment revision 일치를 확인한다. 통과. |
| 19 | Rust snapshot 필드가 일부 초기화 경로에서 빠짐 | snapshot은 하나의 resolver 생성 경로에서 네 크기 필드를 모두 채운다. 단위 테스트 통과. |
| 20 | Rust `repr(C)`와 C header field 순서/타입 불일치 | 두 쪽 모두 layout width/height, visual width/height, DPR, page scale, scheme 값, revision 순서다. Android/iOS native 빌드 통과. |
| 21 | C ABI setter가 visual 필드를 복사하지 않음 | 세 case 모두 C ABI setter output을 비교한다. Android/iOS 앱 fixture 통과. |
| 22 | C ABI Canvas nested environment 응답이 다름 | 세 case 모두 nested environment와 revision key를 비교한다. Android/iOS 앱 fixture 통과. |
| 23 | Android bridge 선언·로그 포맷이 새 struct와 맞지 않음 | API 37.2 ARM64 AVD native build와 app run이 통과하고 logcat은 `c04_12_environment_abi=PASS`다. |
| 24 | iOS bridge 선언·로그 포맷이 새 struct와 맞지 않음 | iOS 26.2 Simulator arm64 build와 app fixture pass 로그를 확인했다. |
| 25 | fixture symbol이 default artifact에 들어감 | 기본 FFI 정적 라이브러리 `nm`에서 두 fixture symbol이 없고 opt-in iOS library에는 있다. 통과. |
| 26 | fixture가 여러 오류 반환에서 session을 누수 | C ABI output 검증은 결과를 저장한 뒤 success/error 모두에서 `spinon_runtime_session_free`를 호출한다. 코드 검토 통과. |
| 27 | fixture가 성공 문자열만 보고 실패 status를 감춤 | native caller는 status 0과 `c04_12_root_html_asset=PASS`를 모두 요구하고 세부 행렬 실패는 음수 status로 반환한다. 통과. |
| 28 | malformed/oversized asset validation이 regression | fixture boundary의 UTF-8, null pointer, size 초과 테스트 5개가 통과한다. |
| 29 | Chrome emulation의 모순된 `innerWidth`, no-meta 캡처를 근거로 사용 | layout은 `clientWidth/clientHeight`와 `matchMedia`로 판정하며 no-meta 불일치·pixel bbox 제외를 문서화했다. 통과. |
| 30 | PASS 화면을 HTML rendering 완료로 읽거나 C04.12를 완료 처리 | Simulator 캡처는 앱 fixture report라고 표기하며 body paint·page-scale transform·실기기를 제외한다. STATUS의 C04.12 부모는 unchecked다. 통과. |

## 변경분 검증

- `cargo fmt --all -- --check`, runtime root_html 30 tests, feature-gated FFI root_html 5 tests, feature-gated FFI Clippy, workspace Clippy와 fixture boundary 6 tests가 통과했다.
- Android API 37.2 ARM64 AVD `emulator-5554`와 iPhone 17 Pro/iOS 26.2 Simulator 빌드·실행이 통과했다. 물리 Android/iOS 기기는 연결하거나 사용하지 않았다.
- Chrome oracle, 앱 캡처와 simulator 로그는 `SHA256SUMS`로 검증했다.
- Rspress docs build는 기존 전체 저장소 dead links 때문에 실패한다. 이번 viewport 문서에서는 추가 dead link가 보고되지 않았다.
