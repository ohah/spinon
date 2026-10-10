# C09.3 `flow-root` 구현 변경 실패 경로 검토

- 대상: C09.3 `display: flow-root`의 CSS cascade → typed layout style → Taffy tree → runtime GPU scene 변경
- 비교 기준: 고정 Chromium 154.0.8037.98, C09 reference v2, viewport 320×240 CSS px, DPR 1·2
- 수치 기준: C09.3 4개 case·16개 node, computed style과 x/y/width/height 필드별 최대 오차 0.5 CSS px
- 구현 기록을 반영한 계획 SHA-256: `5263b70feff19c73ba068d133329991d7d5e1252298aec6a488381e67994c8df` · 최초 계획 검토 당시 hash는 별도 [계획 검토 기록](./c09-3-flow-root-plan-review-2026-10-10.md)에 보존
- 플랫폼 범위: Android API 37 emulator 및 iPhone 17 Pro / iOS 26.2 Simulator. 실기기·성능·hardware GPU 결과는 주장하지 않음.
- 검토 목적: 구현 계획 검토와 별개로 코드, 오류 경계, 회귀 경로, 플랫폼 연결에서 구현을 깨뜨릴 수 있는 관점 20개를 확인함.

| # | 실패 관점 | 확인 결과와 조치 |
| --- | --- | --- |
| 1 | typed style에서 `flow-root`가 `block`으로 뭉개짐 | `LayoutDisplay::FlowRoot`를 별도 값으로 유지하고, 고정 reference의 computed `display` 비교로 값 보존을 확인했다. |
| 2 | Taffy style 변환이 값 매핑을 빠뜨림 | Taffy 0.14.0 `Display::FlowRoot` 매핑을 추가했다. Android·iOS 네이티브 링크가 새 분기를 포함해 빌드됐다. |
| 3 | 사용자 정의 tree dispatcher가 새 formatting context를 만들지 않음 | 자식이 있는 FlowRoot에 `compute_block_layout(..., None)`을 전달하고, 4개 pinned Chromium case가 내부·외부 margin 관계를 통과했다. |
| 4 | 일반 Block의 부모 BFC 전달이 바뀜 | `Display::Block`은 기존 `block_context` 경로를 유지한다. C09.1·C09.2 regression suite를 전체 workspace에서 다시 실행했다. |
| 5 | 빈 FlowRoot가 Block 알고리즘만 호출해 실패함 | 자식 없는 FlowRoot leaf 경로 테스트가 없어 추가했다. 고정 width/height 60×40 layout이 통과했다. |
| 6 | viewport root인 FlowRoot의 auto width가 definite로 인식되지 않음 | `percentage_basis.rs`의 BlockFormatting root 축 판정을 수정했다. viewport 폭 320px, 자손 10% margin의 x=32px test가 통과했다. |
| 7 | FlowRoot 안 auto-width Block 자손의 inline size가 indefinite로 처리됨 | FlowRoot를 Block containing context로 취급하도록 보정했다. 폭 200px FlowRoot 안 auto Block과 10% margin 자손의 x=20px test가 통과했다. |
| 8 | 첫 자식의 top margin이 FlowRoot 밖으로 탈출함 | `c093-flow-root-internal`에서 첫 자식 y offset 20px을 검사했고 고정 Chromium과 일치했다. |
| 9 | 마지막 자식의 bottom margin이 FlowRoot 높이에서 누락됨 | 같은 case에서 auto-height FlowRoot 높이와 자식 frame을 대조했다. 고정 Chromium 값 60px과 일치했다. |
| 10 | FlowRoot의 own top margin이 비-BFC 부모 첫 자식과 collapse하지 않음 | `c093-flow-root-parent-margin`에서 부모·FlowRoot·손자 y를 각각 비교했고 pinned reference와 일치했다. |
| 11 | FlowRoot own bottom margin이 부모의 마지막 자식 관계에서 누락됨 | `c093-flow-root-parent-last-child`의 FlowRoot 높이와 다음 형제 y를 비교했다. |
| 12 | 앞선 같은 BFC 형제의 bottom margin을 FlowRoot top margin과 collapse하지 않음 | `c093-flow-root-sibling-margin`에서 앞 상자와 FlowRoot y를 비교했다. |
| 13 | 다음 같은 BFC 형제와 FlowRoot bottom margin 결합이 틀림 | 같은 case에서 다음 형제 y와 부모 높이를 비교했다. |
| 14 | CSS source scanner가 computed cascade 전에 합법적인 선언을 거부함 | inline style과 nested author stylesheet에서 `flow-root`를 받아들인다. author stylesheet runtime test에서 computed value와 frame을 확인했다. |
| 15 | 새 허용 규칙이 미지원 display를 함께 통과시킴 | `grid`, `flex`, `inline`, `table`, `inline flow-root`, `var()` negative cases는 계속 fail-closed다. |
| 16 | C09 외 profile로 새 지원이 번짐 | `RuntimeBlockPaintV1`의 명시적 거부 테스트가 유지된다. source 검사 허용만으로 layout projection을 통과하지 않는다. |
| 17 | C09.1 Block width·containing block regression | 기존 Chromium 10개 case·30개 node를 재실행했고 workspace tests가 통과했다. |
| 18 | C09.2 signed margin collapse regression | 기존 Chromium 16개 case·57개 node를 재실행했고 workspace tests가 통과했다. |
| 19 | preorder·computed value·CSS px·DPR 비교 경계가 약해짐 | 4개 C09.3 case·16개 node의 부모 ID, computed styles, 네 frame 필드를 DPR 1·2로 비교했다. 두 scale의 CSS geometry와 styles가 같고 각 값이 0.5 CSS px oracle 안에 있다. |
| 20 | FFI나 플랫폼 실행기가 다른 fixture·잘못된 route를 사용함 | 같은 V8 fixture를 FFI부터 Android JNI/실행 인자와 iOS Objective-C++/Swift 실행 인자까지 연결했다. 두 simulator 모두 5개 box를 V8→Stylo→Taffy→WGPU로 계산·표시했고 각 node CSS frame이 일치했다. PNG와 로그는 이 폴더에 저장했다. 화면은 숫자 oracle을 대신하지 않는다. |

## 발견점과 수정

- layout 코드 대조에서 `percentage_basis.rs`가 FlowRoot containing context 안 auto-width Block을 definite width로 보지 않는 경로를 찾았다. 그 결과 실제 containing width가 있어도 자손 percentage margin/padding을 거부할 수 있었다. BlockFormatting root와 FlowRoot 내부 auto Block 판정을 보정하고 각 경계 회귀 테스트를 추가했다.
- 계획에 명시한 childless FlowRoot leaf 분기는 reference fixture에 해당 case가 없어 직접 검증되지 않았다. 고정 치수 leaf 테스트를 추가했다.
- 첫 Android 시각 캡처는 root auto height 154px만 칠해져 viewport 하단이 검은 영역으로 남았다. runtime smoke fixture의 root 높이를 viewport와 같은 240px로 고정하고 새 바이너리로 재실행·재캡처했다. node geometry assertion은 이 화면 수정과 독립적으로 고정 Chromium reference를 비교한다.

## 실행 근거

- `cargo test --locked --workspace` — 통과.
- `cargo test --locked -p spinon-layout -p spinon-runtime -p spinon-style -p spinon-style-to-layout` — 통과.
- `bun run test:css-reference` — 85 passed, 0 failed.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `git diff --check` — 통과.
- Android: `SPINON_C093_EVAL layout=ready boxes=5`, `SPINON_C0410_DRAW presented boxes=5`, root 320×240 CSS px.
- iOS: `SPINON_C0410_ENVIRONMENT layout=ready boxes=5`, `SPINON_C0410_DRAW presented boxes=5`, root 320×240 CSS px.
- [Android screenshot](./c09-3-flow-root/android.png) · [Android log](./c09-3-flow-root/android.log) · [iOS screenshot](./c09-3-flow-root/ios.png) · [iOS log](./c09-3-flow-root/ios.log).

PR #111이 2026-10-10에 리베이스 병합됐다. 공식 C09.3 완료 체크와 내부 계약·구현 인덱스를 병합 뒤 문서 동기화 PR에서 반영했다.
