# C05.2 `@property` 구현 검토와 시뮬레이터 실행 근거

**내부 계약:** [0034 `0.1.0` 고정](../0034-c05-runtime-registered-properties.md) · **계획:** [C05.2](../../../plan/c05-runtime-registered-properties.md) · **계획 검토:** [실패 관점 기록](c05-runtime-registered-properties-plan-review-2026-10-10.md) · **Chromium 기준:** [고정 reference](../../../tests/fixtures/css/references/c05-runtime-registered-properties-v1.json)

## 구현 변경에서 확인한 사항

- 새 등록 CSS profile의 기능 설정 인자가 Clippy 인자 수 제한을 넘었다. 개별 boolean 인자들을 `AuthorFeaturePolicy`로 묶고, profile 경계가 이름으로 드러나게 정리했다.
- 등록 사용자 지정 속성 결과가 computed-style profile에서 Taffy layout projection과 typed GPU paint까지 전달되도록 연결했다. layout-only profile은 일반 `background-color`를 계속 거부한다.
- 처음 검토에서 새 C ABI fixture 심볼은 내부 전용이라는 경계만 있고 정확한 C 서명·출력 buffer 조건·호출 executor·반환 상태가 명세에 없었다. 내부 계약 0034에 이를 추가하고 C04.10 공유 상태 코드 계약으로 연결했다.
- stylesheet source 순서가 이동·분리·재삽입 뒤에도 현재 HostDocument snapshot을 반영하도록 매 요청에서 현재 연결 `<style>`을 수집해 새 Stylo registry를 구성한다. 이전 revision 등록을 별도 cache에 보존하지 않는다.
- 유효한 알 수 없는 descriptor의 Stylo 진단만 무시하도록 제한하고, 필수 descriptor 오류 및 다른 stylesheet 진단은 실패 처리한다.
- Android·iOS 재실행에서 JavaScript `const` 재선언과 노드 중복 생성을 피하도록 fixture를 전역 상태에 저장하고, 재실행에서는 등록 초기값과 stylesheet 순서를 갱신한다.
- 화면 제목이 iOS에서 잘리는 것을 확인해 두 플랫폼 모두 `SPINON · C05.2 @property`로 줄였다.
- 적대적 검토에서 C05.2 CSS profile이 일반 GPU host 생성 경로와 분리되지 않은 연결 결함을 찾았다. C05.2 전용 host 생성 C ABI를 추가하고 Android/iOS 검증 모드에서만 호출하도록 수정했다. C04.10, C04.11, C05.1의 일반 host 생성 경로는 기존 profile을 사용한다.
- 재적용 때 전체 runtime 진단문을 화면 상태 라벨에 넣어 iOS 버튼과 겹치는 문제를 재현했다. 상세 원문은 log에 남기고 화면에는 `status`, layout, box 수와 revision만 표시하도록 두 플랫폼을 수정했다.

## 구현 변경에 대한 적대적 검토

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 구현이나 내부 문서 추가만으로 출시 전 숫자 버전이 오르는가 | crate와 내부 계약은 `0.1.0` 유지. `0034`는 문서 일련번호이며 릴리스 버전 변경이 아니다. |
| 2 | 새 `@property` 허용이 C05.1 또는 다른 기존 computed-style profile로 새어 가는가 | 계산 profile 분리와 별도 C05.2 host 생성자를 확인했다. Android/iOS는 C05.2 실행 인자가 있을 때만 전용 생성자를 호출하고 일반 모드는 기존 생성자를 유지한다. 일반 GPU profile 거부 회귀 테스트를 포함한다. |
| 3 | 등록 규칙 허용이 일반 CSS 선언 allowlist까지 넓히는가 | layout 속성 allowlist는 유지되고 paint profile에만 기존 허용 색상 속성을 쓴다. `color`와 그 밖의 속성은 거부된다. |
| 4 | `@media`, `@supports`, `@layer`, `@import` 또는 중첩 규칙을 통해 조건부 기능이 들어오는가 | profile 검사에서 최상위 `@property`만 허용한다. 조건부·중첩 규칙은 거부 테스트를 둔다. |
| 5 | 알 수 없는 descriptor 때문에 유효한 등록 전체가 실패하는가 | Stylo 0.22의 `Unsupported @property descriptor declaration:` 진단만 제거한다. `vendor-extension` fixture가 등록과 초기값 계산을 유지한다. |
| 6 | 다른 parser diagnostic도 문자열 접두부와 혼동되어 무시되는가 | 해당 접두부 외 모든 author stylesheet diagnostic은 실패로 남는다. |
| 7 | 필수 `syntax` 누락을 성공으로 오인하는가 | 필수 descriptor 누락을 실패 fixture로 확인한다. |
| 8 | 초기값이 등록 syntax와 맞지 않는데 잘못된 값으로 계산되는가 | `<length>`에 `red`를 준 초기값을 실패 처리한다. |
| 9 | 등록 syntax와 맞지 않는 사용값이 그대로 layout에 흘러가는가 | `<length>` 속성에 `red`를 지정한 노드는 등록 초기값 `23px`로 계산되고 frame도 비교한다. |
| 10 | 지정되지 않은 등록 속성이 `initial-value`를 적용하지 않는가 | `<length>` 기본값 `23px`, 뒤늦게 선언한 등록의 `31px`를 Chrome과 정확 비교한다. |
| 11 | `inherits:false` 속성이 조상값을 잘못 상속하는가 | 조상의 `71px` 대신 등록 초기값 `23px`가 자식에게 적용되는지 확인한다. |
| 12 | `inherits:true`가 상속 계산을 빠뜨리는가 | 조상의 `19px`가 자식의 computed width 및 Taffy frame에 전달된다. |
| 13 | 사용 규칙 아래쪽에 있는 등록 규칙을 놓치는가 | 등록 순서와 관계없이 `<style>` 전체의 등록을 처리하고 Chromium reference와 비교한다. |
| 14 | 중복 이름 등록 승자가 stylesheet 순서와 달라지는가 | 두 stylesheet의 `<length>`·`<number>` 충돌과 계산값을 pinned Chrome과 대조한다. |
| 15 | stylesheet 이동이 DOM source order와 계산에 반영되지 않는가 | 두 번째 stylesheet를 앞에 이동한 뒤 등록 승자와 frame을 다시 비교한다. |
| 16 | stylesheet 본문 변경 뒤 이전 `initial-value`가 남는가 | `23px → 37px` 본문 변경에서 새 snapshot의 computed 값이 갱신된다. |
| 17 | stylesheet 분리 뒤 등록값 또는 스타일이 남는가 | 첫 stylesheet 분리 뒤 연결된 두 번째 source만 적용되며 계산 결과가 바뀐다. |
| 18 | 분리한 stylesheet 재삽입 뒤 예전 등록 순서가 잘못 복구되는가 | 새 DOM 순서 기준으로 등록 결과를 재계산하며 Chromium 이동·재삽입 reference와 맞춘다. |
| 19 | Android/iOS 반복 실행이 `const` 재선언 또는 중복 노드로 실패하거나 UI 로그가 컨트롤과 겹치는가 | 실제 V8 두 번째 평가가 `status=0`, document node 수 16개 유지, layout box 수 11→12로 완료된다. 상세 log는 보존하고 상태 라벨은 요약해 재적용 화면에서 겹치지 않는다. |
| 20 | 새 FFI 심볼의 인자·버퍼·대기·오류 계약이 빠지거나 공개 API로 오해되는가 | 계약 0034에 전용 host 생성·fixture 평가 C signature, 일반 host와 profile 분리, live-host/serialized-call/background executor 조건, NUL 출력 buffer와 상태 코드를 기록했다. 두 심볼은 검증 모드용이며 public API가 아니다. |
| 21 | 한 HostRoot의 연결 stylesheet 등록이 다른 HostRoot의 fragment cascade에서 사라지는가 | pinned Chrome 보조 fixture에서 둘째 root의 `width:19px`와 `19×14` rectangle을 관찰했다. Runtime Rust 테스트도 둘째 root의 computed width `19px`를 확인하고 기존 다중 root layout 거부 코드를 별도 assertion 한다. |

## 자동 검증

- `mise exec -- cargo test --locked --workspace` — C05.2 검증 fixture 추가 뒤 전체 Rust workspace 통과. Runtime 단위 테스트는 91개이며 새 다중 HostRoot 등록 경계 테스트도 포함한다.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` — 인자 과다 검출을 `AuthorFeaturePolicy`로 수정한 뒤 전체 workspace 통과.
- `cargo fmt --all -- --check` 및 `git diff --check` — 통과.
- `bun run test:css-reference` — 전체 27개 CSS reference 검사 통과. `node tools/css-reference/verify-c05-runtime-registered-properties-multi-root.mjs` — Chrome `154.0.8037.98`에서 `rootCount=2`, `width=19px`, rectangle `19×14` 확인.
- `bun run test:js` — 2개 통과. `bun run test:benchmark:r05-physical-touch` — 29개 통과.
- `mise exec -- env SPINON_V8_DIR=… SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android` 및 `… bun run build:ios-sim` — Android ARM64 APK와 iOS Simulator 앱 빌드 성공.

## Android API 37 emulator

- 기기: `emulator-5554`, ARM64, Android API 37 emulator. 실기기는 사용하지 않았다.
- V8 첫 실행: `status=0`, `layout=ready`, 11개 render box, document revision 57, DOM node 16개.
- 재실행: `status=0`, `layout=ready`, 12개 render box, document revision 59, DOM node 16개. 노드는 늘지 않고 등록 초기값·source order 변화가 다시 계산됐다.
- WGPU renderer 로그는 `backend=Gl`; ANGLE이 Vulkan 1.3 SwiftShader를 사용한다. 하드웨어 Vulkan/실기기 성능 근거가 아니다.
- [초기 화면](c05-runtime-registered-properties-android-initial-2026-10-10.png) · [재적용 화면](c05-runtime-registered-properties-android-2026-10-10.png) · [원시 실행 로그](c05-runtime-registered-properties-android-2026-10-10.log)
- profile 격리 수정 뒤 새 APK로 재실행: C05.2는 `SPINON_C052_HOST` 전용 host에서 최초·재적용(`boxes=11→12`, `document_revision=57→59`, node 16 유지)이 모두 성공했다. C04.11 및 C04.10+C05.1 모드는 `SPINON_C0410_HOST` 기존 host에서 성공했다. [C04.11 기존 host 경로 로그](c05-runtime-profile-isolation-android-c0411-2026-10-10.log) · [C05.1 재적용 로그](c05-runtime-profile-isolation-android-c051-2026-10-10.log)

## iOS Simulator

- 기기: iPhone 17 Pro Simulator, iOS 26.2. 실기기는 사용하지 않았다.
- V8 첫 실행: `status=0`, `layout=ready`, 11개 render box, document revision 57, DOM node 16개.
- 재실행: `status=0`, `layout=ready`, 12개 render box, document revision 59, DOM node 16개.
- `CAMetalLayer` WGPU 표면의 draw 로그가 두 실행 모두 status `0`을 기록한다. [C04.11 실행 근거](c04-runtime-author-stylesheets-2026-10-10.md)와 같은 iOS Simulator Metal 경로다.
- [초기 화면](c05-runtime-registered-properties-ios-initial-2026-10-10.png) · [재적용 화면](c05-runtime-registered-properties-ios-2026-10-10.png) · [원시 실행 로그](c05-runtime-registered-properties-ios-2026-10-10.log)
- profile 격리 수정 뒤 새 앱으로 재실행: C05.2가 `SPINON_C052_HOST`로 최초 계산을 완료했고, C04.11은 기존 `SPINON_C0410_HOST` 경로로 `status=0`, `layout=ready`, 3개 box를 발행했다. [host 선택 원시 로그](c05-runtime-profile-isolation-ios-2026-10-10.log)

Xcode 빌드는 성공했지만 Rust static library 사이 `___REGISTER_CLASS_RawWindowMetalLayer` 중복 심볼 및 V8 archive debug-map timestamp 경고를 출력했다. 같은 링크 산출물에서 iOS Simulator가 초기 장면과 재적용 장면을 모두 그렸다. 이 링크 경고의 소유권 정리는 C05.2 기능 계약과 별도 작업이다.

## 남은 범위

이 검증은 고정 Chrome fixture에 대한 제한된 `<length>`, `<color>`, `<number>` 등록·상속·source order 및 기존 제한 Flex/paint 경로다. 다중 HostRoot 비교는 등록 cascade만 확인하며 해당 장면의 runtime layout은 여전히 `multiple_host_roots`로 거부된다. 전체 CSS·일반 stylesheet at-rule·CSSOM·`CSS.registerProperty()`·외부 CSS 로딩·증분 재계산·실기기·하드웨어 성능 비교는 포함하지 않는다. C05 부모와 CSS 전체 지원 완료를 뜻하지 않는다.
