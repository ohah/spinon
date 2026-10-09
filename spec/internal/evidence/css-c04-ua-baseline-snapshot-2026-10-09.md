# C04.5 · 지원 HTML UA computed-style snapshot 실행·구현 검토

## 구현 결과

`compute_supported_elements_ua_cascade`와 `SupportedElementsUaV1`을 `spinon-style`에서 공개 Rust API로 추가했다. 함수는 컴파일 시 내장한 `supported-elements-v0.css`를 Stylo `Origin::UserAgent`로 등록하고, 호출자가 준 인메모리 author stylesheet와 `StyloDocumentView`의 inline `style`을 함께 cascade한다. 결과는 선택 HostRoot subtree의 모든 요소에 대해 7개 computed property만 직렬화하고 문서·스타일·viewport revision을 보존한다.

C01 고정 Chromium reference의 HTML 요소 9개·feature 19개를 CSS computed string 단위로 모두 비교했다. 출력 node 집합은 fixture의 11개 요소(html·body·대상 9개)와 일치한다. 지원 밖 CSS 구문은 기존 Stylo 진단으로 남고 호출 성공만으로 해당 입력이 지원된다고 판정하지 않는다. 네트워크 요청은 하지 않는다.

## 재현 가능한 비교 자료

| 자료 | 고정 값 |
| --- | --- |
| Chromium | Google Chrome `154.0.8037.95`, revision `@05d469856e75794131cc2e5d9b2f6b6f10a70388` |
| 실행 환경 | macOS `26.5.1` (`25F80`), arm64, 800×600 CSS px, device scale 1, en-US, UTC, screen/light |
| reference JSON SHA-256 | `e6b5d6f42100d8c8dc27874ef13ec760e99272cd8f6f97cf8a044a5e4c5656e4` |
| C01 inventory SHA-256 | `1293be438ca54d184c0f614d49fb0549bded5484806e73467a73898e6dc4fc6c` |
| C01 HTML fixture SHA-256 | `dba2edfd1268afdb9f32d9ad725191e903ef2af30e1a549e29279f1576ff3863` |
| embedded UA CSS SHA-256 | `bd15dd612a21cc8cb48e25eb38b86803868667df75cd56f111d7c476ddba3ebd` |

Rust test는 reference ID·Chromium 제품 버전·revision·fixture identity와 hash metadata를 확인하고, C01 inventory에 대응하는 feature ID·selector·node ID·computed string을 검사한다. 위 파일 byte hash는 이번 실행에서 `sha256sum`으로 별도 재계산해 기록했다. 테스트 자체가 실행 중 파일 byte hash를 재계산하지는 않는다.

## 구현 변경

- `ComputedStyleProfile::SupportedElementsUaV1`과 `compute_supported_elements_ua_cascade`를 추가하고 crate root에서 다시 export했다.
- cascade 공통 경로에 새 profile의 7개 computed property 집합을 연결했다. UA sheet 등록과 author origin 순서는 기존 공통 cascade를 그대로 사용한다.
- UA 계산 snapshot은 현재 Flex/Taffy profile과 다른 입력 계약이므로 `spinon-style-to-layout`에서 전체 거부한다.
- 기존 cascade device·fixed font metrics·thread-state 코드를 `cascade/device.rs`로 옮겨 `cascade.rs`를 449줄로 줄였다.
- Rust tests는 Chromium 값 일치, 정확한 반환 node/property 집합, author와 inline override, namespace 경계, 잘못된 입력, 진단·원격 `@import` 미요청, revision 보존을 확인한다.

## 구현 후 적대 검토 20개 관점

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | computed snapshot profile과 UA CSS 자원 버전을 혼동 | `SupportedElementsUaV1`과 `spinon-html-ua/0.1.0-draft`의 식별 목적이 다름을 계약에 명시했다. |
| 2 | 제한 API를 공개 DOM/CSSOM 또는 FFI 기능으로 오인 | Rust crate 내부 스타일 계산 진입점이며 JS·C ABI 노출이 없음을 명세했다. |
| 3 | 내장 UA CSS가 Author origin으로 잘못 등록 | 공통 registry가 내장 sheet를 `UserAgent`로 등록하는지 코드와 기존 registry test를 확인했다. |
| 4 | 호출자가 내장 UA 입력을 Author sheet 목록으로 위장 | `UserAgent` 입력을 `InvalidStylesheetOrigin`으로 거부하는 새 profile test를 통과했다. 현재 `CssOrigin`은 `User` origin을 표현하지 않는다. |
| 5 | author normal rule이 내장 UA 기본값에 지지 않음 | `ul`에 author `display:flex`·padding 값을 적용해 두 값이 UA를 덮는지 확인했다. |
| 6 | inline `style`이 author stylesheet보다 낮은 우선순위로 계산 | 같은 `ul`에 author `flex`와 inline `block`을 적용해 최종값이 inline 값인지 확인했다. |
| 7 | 같은 local name의 SVG/외부 namespace 요소에 HTML UA 선택자가 적용 | 외부 namespace `div`의 기본 `inline` 값으로 HTML namespace 경계를 확인했다. |
| 8 | 다른 root 또는 root 바깥 node가 계산 대상에 들어옴 | view의 명시 HostRoot 계약과 기존 invalid/foreign-generation DOM adapter tests를 함께 확인했다. |
| 9 | 결과에서 root·body 또는 하위 대상 요소를 조용히 누락 | 테스트에서 view subtree의 11개 요소 ID 전체와 결과 집합을 정확 비교한다. |
| 10 | 내부 profile이 지정한 property 밖의 값을 출력 | 각 결과 요소의 property key 집합이 정확히 7개인지 확인한다. |
| 11 | C01 fixture의 selector 9개 중 하나를 빠뜨려 부분 통과 | reference와 inventory 각각 9개, selector 집합의 정확한 일치를 검사한다. |
| 12 | feature 누락·중복·다른 node 연결을 성공 처리 | 19개 feature ID의 유일성, property 대응, fixture ID·HTML namespace와 node mapping을 대조한다. |
| 13 | reference 파일이 다른 Chrome·viewport 결과로 바뀜 | reference ID·Chrome 제품/version/revision·fixture hash metadata·viewport를 고정 확인하고 19개 값을 정확 비교한다. |
| 14 | 0·NaN·무한대·device-pixel overflow viewport가 cascade에 진입 | 네 invalid 형태가 모두 `InvalidViewport`로 종료되는지 검사한다. |
| 15 | 계산 결과에 다른 viewport/environment revision을 붙임 | 입력 `CssViewport`가 결과에 그대로 남는지 equality 검사한다. |
| 16 | view의 문서·렌더 revision 또는 호출 style revision을 바꿔 기록 | generation·document/render revision과 style revision echo를 fixture에서 확인한다. |
| 17 | stylesheet 진단을 버리거나 원격 `@import`를 실제로 요청 | `@import` 진단의 source ID 보존을 확인하고 실행기는 네트워크 loader를 갖지 않는 것을 확인했다. |
| 18 | 빈·중복 sheet ID 또는 잘못된 base URL 오류를 성공으로 둔갑 | 함수가 registry `Result` 오류를 그대로 전파함을 확인하고 registry 실패 원자성·URL 테스트를 workspace 회귀로 실행했다. |
| 19 | 새 profile이 기존 Flex/Taffy profile을 넓히거나 지원 밖 profile이 투영됨 | 전체 workspace regression을 통과했고 새 UA profile을 Taffy 경로에 넘기면 `UnsupportedProfile`로 거부하는 회귀 테스트를 추가했다. |
| 20 | 동기 계산·캐시·media environment·mobile 실행 범위를 실제 보장보다 넓게 설명 | 호출 thread에서 동기 계산, 호출별 registry/stylist/subtree walk, 캐시·증분 invalidation 미제공을 계약에 적었다. pointer 기본값은 Stylo target별이며 완전한 author `@media` parity를 보장하지 않는다. iOS·Android는 target compile만 실행했고 앱 runtime·성능·GPU 화면은 주장하지 않는다. |

검토 중 test에서 소유자 ID를 잘못 지정한 픽스처 오류를 바로잡았다. 또 현재 pinned Stylo가 `display:grid` inline declaration을 unsupported diagnostic으로 반환하는 것을 확인해 우선순위 test 입력을 지원되는 `block` 값으로 바꿨다. 이는 C04.5의 CSS 지원 범위가 Grid까지 확장되었다는 근거가 아니며, 해당 한계는 별도 C04/Grid 작업에 남는다.

## 실행 결과

- `cargo test --locked --workspace` — 통과, workspace 전체 201개 단위·통합 테스트.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `cargo fmt --all -- --check` — 통과.
- `git diff --check` — 통과.
- `cargo check --locked -p spinon-style -p spinon-style-to-layout --target aarch64-apple-ios` — 통과.
- 같은 check를 `aarch64-apple-ios-sim`과 `aarch64-linux-android`로 실행 — 모두 통과.

## 미완료 경계

이 내부 API는 runtime stylesheet set, worker scheduling, 제품 owner/root·viewport 및 complete media environment 공급, CSS invalidation/cache, Taffy의 Block/inline/list marker/Grid, text shaping, GPU scene, Android/iOS 앱 화면에 연결되지 않았다. 동기 계산 비용을 측정하거나 병렬 실행의 안전성을 입증하지 않았다. 실기기 R05 직접 입력 block 수집은 현재 연결 불가 상태라 백로그에 두며 이 작업은 해당 수집을 기다리지 않는다.
