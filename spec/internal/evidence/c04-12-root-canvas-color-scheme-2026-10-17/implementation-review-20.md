# C04.12 root Canvas color-scheme 구현 실패 관점 검토

- **대상:** root metadata 추출, CSS cascade 결과 선택, runtime wait/revision 검사, FFI와 Android/iOS 적용 경로
- **검토 범위:** 계획 검토 20개를 재사용하지 않고 변경 코드와 현재 fixture·Simulator 원본을 대조
- **통과 범위:** native content root 배경색 2종과 OS scheme 전환만 확인. 일반 HTML layout·text·paint 완료 판정은 아님

## 검토 항목

| # | 독립 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | OS dark 선호만으로 root Canvas를 dark로 바꾸는가? | computed root가 `normal`이고 metadata도 없으면 light를 선택한다. environment 테스트와 Chrome 비교가 이 경계를 확인한다. |
| 2 | 첫 `meta color-scheme=dark`가 OS light에서 무시되는가? | root가 `normal`이면 단일 metadata supported list를 사용한다. environment parser 테스트와 Chrome light case가 dark를 선택한다. |
| 3 | `light dark` 지원 목록이 OS 값을 따르지 않는가? | 둘 다 지원하면 `ColorSchemeMetadata::resolve`가 preference를 선택한다. metadata fallback 테스트와 Chrome 양 scheme 행을 대조했다. |
| 4 | `only dark`를 OS light에 맞춰 light로 되돌리는가? | `only`는 지정 scheme을 반환한다. 기존 scheme fixture의 parser/selection 테스트가 유지된다. |
| 5 | 명시적 root CSS `dark`보다 meta `light`가 우선하는가? | 실제 HTML root computed `color-scheme`을 먼저 해석하고 metadata는 `normal`일 때만 사용한다. session precedence 테스트가 dark RGBA를 확인한다. |
| 6 | root CSS의 값 대신 body·첫 자식 style을 읽는가? | completed root snapshot의 `root_node_id`와 일치하는 element만 선택한다. 다른 node 값은 Canvas 선택 경로에 들어오지 않는다. |
| 7 | cascade 계산 중 이전 revision의 색을 성공으로 반환하는가? | `requested == completed.key`, 환경 revision, generation/document/render-tree/style/environment key를 모두 비교한다. 하나라도 어긋나면 stale 오류다. |
| 8 | 빈 문서·root 목록 불일치에서 임의 색을 반환하는가? | root 개수가 정확히 1이 아니면 `MissingRoot`; root style/property가 없으면 `MissingRootStyle`이다. 기본 root HTML session의 completed root만 허용한다. |
| 9 | cascade pending이 timeout 뒤에도 light 성공으로 바뀌거나 큰 Rust timeout에서 panic하는가? | pending이면 `CascadeTimeout`을 반환한다. zero-timeout/느린 fake computer 테스트는 이전 completion 재사용 없이 pending을 확인한다. 검토에서 `Instant + Duration::MAX` overflow 경로를 찾아 `checked_add`와 60초 단위 condition wait로 바꾸고 같은 테스트에서 `Duration::MAX`를 호출해 확인한다. |
| 10 | cascade worker 실패를 default light로 숨기는가? | failed state는 `CascadeFailed`로 종료한다. 색 바이트는 성공 snapshot이 있을 때만 생성된다. |
| 11 | 단일 invalid metadata 뒤에 다른 값이나 OS theme를 잘못 물려받는가? | invalid metadata는 supported list가 없는 상태로 남고 `normal` root의 fallback은 light다. Chrome 표와 Rust 환경 fixture를 대조했다. |
| 12 | invalid metadata 다음의 두 번째 meta를 임의 선택하는가? | metadata 요소 수가 둘 이상이면 Canvas API는 `DuplicateColorSchemeMetadata`를 반환한다. invalid-first/valid-second fixture와 session 거부 테스트가 이를 확인한다. |
| 13 | duplicate count가 많은 입력에서 overflow되어 duplicate를 잊는가? | `u8::saturating_add`를 사용하고 1 초과 여부만 판정하므로 255개 초과에서도 fail-closed가 유지된다. |
| 14 | metadata `only` 문법이 조합 scheme으로 잘못 해석되는가? | `only` 뒤 한 토큰만 허용하고 추가 supported token은 parse 실패한다. 기존 allowlist 테스트가 malformed `only dark light`를 거부한다. |
| 15 | Stylo가 출력한 미지원 computed token을 임의 밝기로 해석하는가? | `normal`과 제한 scheme grammar만 해석하며 unknown token은 `UnsupportedComputedColorScheme`이다. environment unit test가 `sepia`를 거부한다. |
| 16 | Canvas 색을 display profile이나 OS system palette의 보편값으로 오인하는가? | 반환은 고정 Chrome 155 sRGB `#ffffff`/`#121212`의 opaque RGBA 2종이다. 명세와 evidence는 다른 control/system 색을 제외한다. |
| 17 | FFI 오류·잘못된 입력에서 output struct가 일부 기록되는가? | session/canvas null, 알 수 없는 scheme, timeout 60초 초과는 쓰기 전에 실패한다. 성공에만 완성된 environment·key·RGBA 구조체를 한 번 기록한다. |
| 18 | Android JNI cascade 대기가 main thread를 막거나 오래된 변경을 화면에 적용하는가? | 초기·변경 계산은 `bootstrapExecutor`에서 수행하고 UI 적용은 main으로 보낸다. session id와 request generation 재검사 후 decor background를 바꾼다. |
| 19 | Activity 종료 중 JNI session free와 대기 중 environment 호출이 경합하는가? | `onDestroy`가 bootstrap executor 종료를 기다린 뒤 root session을 free한다. 종료 race는 코드 순서에서 fail-safe를 확인했으며 이 실행의 Simulator 로그는 theme 전환만 다룬다. |
| 20 | iOS theme/surface update가 직렬 순서 없이 적용되거나 종료 중 session을 해제하는가? | 전용 serial queue에서 resolve하고 main queue에서 generation/session을 다시 검사한다. terminate는 해당 queue를 drain한 뒤 session을 free한다. iOS Simulator 전환 로그가 `0→1→2`를 기록한다. |

## 판정

20개 관점에서 새 Canvas 경로가 이전 CSS/media 결과를 색으로 게시하거나 실패를 성공으로 숨기는 경로는 발견되지 않았다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 light→dark→light 전환이 각각 `#ffffff → #121212 → #ffffff`였고, PNG 중앙 raw RGB가 runtime report와 일치했다. 이 근거는 native content root background 표본이다. body/text, HTML box layout·paint, system UI/control palette, hardware GPU, 실기기와 compositor 뒤 픽셀은 검증하지 않았다.
