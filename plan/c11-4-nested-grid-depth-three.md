# C11.4 · depth-three nested Grid intrinsic sizing 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [depth-two 양 축 계획](./c11-4-nested-grid-two-axis-depth-two.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 및 두 Simulator 검증 완료. 계획 PR [#457](https://github.com/ohah/spinon/pull/457)이 Rebase and merge된 뒤 같은 checkout의 구현 브랜치에서 고정 depth-three slice를 연결했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 8개 control을 포함한 10/10 scene을 통과했다. C11.4 checkbox와 공식 완료 수 72/213은 변경하지 않는다.

## 목적과 범위

[직전 depth-two slice](./c11-4-nested-grid-two-axis-depth-two.md)는 root → middle → inner Grid의 두 Grid-to-Grid 경계에서 min-content/max-content width, height, column, row track을 함께 계산했다. 이번 slice는 같은 제한 입력을 root → middle → branch → inner로 한 단계 더 연결해 intrinsic contribution이 세 중첩 경계를 통과하는지 확인한다.

고정 scene은 min-content와 max-content 두 개다. 각 장면에서 네 Grid는 폭·높이와 단일 column·row track에 같은 intrinsic keyword를 사용한다. 각 Grid는 직계 자식 하나만 가지며 가장 안쪽 Grid의 유일한 자식은 Ahem text leaf다. 새 scene ID는 다음 둘로 제한한다.

- nested-depth-three-min-content-two-axis
- nested-depth-three-max-content-two-axis

기존 depth-one·depth-two 양 축 여덟 장면은 회귀 controls로 함께 실행한다. 새 opt-in fixture adapter만 10개 ID를 허용한다. 제품 profile·일반 style admission·공개 API를 넓히지 않으며 내부 계약 0059의 숫자 버전도 올리지 않는다.

## 고정 입력과 기대값

재현 가능한 HTML은 [depth-three fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-three-precomparison.html)다. 저장소 Ahem을 10px/10px로 사용해 XX XXXXXXX를 한 text item으로 배치한다. 모든 Grid는 margin, padding, border, gap 0, align-items start이며 추가 min/max 제약은 없다. 기존 inline-run과 image fixture는 섞지 않는다.

CSS Grid Layout Module Level 1 2025 CRD의 [§5.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-container-sizing), [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content) 순서에 따라 먼저 inline contribution을 구하고 그 결과로 row sizing을 판정한다. XX XXXXXXX의 Ahem max-content advance는 100px, 가장 긴 공백 없는 조각의 min-content contribution은 70px다. 70px에서 text는 10px line 두 개를 만들고 100px에서는 한 줄에 놓인다. 각 부모 Grid에는 intrinsic child가 하나뿐이므로 이 고정 사례에서는 자식 Grid의 contribution이 다음 부모의 단일 intrinsic track과 container size로 반복 전달된다. Chrome 수치에서 규범 기대값을 역산하지 않는다.

| 장면 | 각 Grid column / row track | 각 Grid와 text 크기 |
| --- | --- | --- |
| nested-depth-three-min-content-two-axis | 70px / 20px | 70×20px |
| nested-depth-three-max-content-two-axis | 100px / 10px | 100×10px |

독립 기대값 JSON과 Chrome observation JSON은 [사전 비교 evidence](../spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/README.md)에 분리되어 있다. Agent Browser의 HeadlessChrome 156 관측은 이 fixed input에서 위 두 track·frame 기대값과 일치한다. page-level absolute y 좌표는 fixture 배치이므로 판정하지 않는다. dimensions와 tracks 허용 오차는 1/64 CSS px다.

## 구현 및 판정 절차

1. 계획 PR을 main에 Rebase and merge한 뒤 같은 checkout에서 최신 main 기준 구현 브랜치를 만든다. 새 worktree를 만들지 않는다.
2. 현재 C11.4 depth-two adapter와 FFI fixture scene dispatch를 읽고, 두 새 ID만 depth 3의 root/middle/branch/inner 순서로 생성한다. node ID, 부모 소유권, 단일 자식, direct text leaf와 source revision을 확인한다.
3. projection·layout 경로의 기존 opt-in two-axis allowlist를 재사용한다. 허용 범위를 넓힐 필요가 생기면 정확한 호출 경계와 미등록 입력 거부 테스트를 먼저 제시하고, broad profile 변경을 피한다.
4. layout·runtime·FFI assertions에서 새 두 장면의 네 Grid와 text node 각각에 대해 부모·자식 순서, Columns·Rows used track, width·height, root-relative frame, source revision, trace completeness를 확인한다. 잘못된 scene ID, depth-four, 추가 자식, 잘못된 intrinsic 조합은 부분 output 없이 거부한다. 기존 여덟 controls도 동일 경로에서 회귀 실행한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator fixture 앱에서 10 scene 전체를 실행한다. 두 플랫폼 모두 node identity/order, 두 축 track/frame, drawPresented, local/installed artifact hash를 보존한다. Android 실기기는 사용하지 않는다.
6. cargo fmt, focused layout/runtime/FFI tests, workspace all-target Clippy -D warnings, Android/iOS fixture builds를 실행한다. 기본 product archive에서 새 scene ID와 고정 Ahem payload가 빠지고 opt-in fixture-test archive에서는 positive control로 포함되는지 확인한다. 실행하지 않은 검사는 통과로 쓰지 않는다.
7. 구현 완료 후 계획 검토와 관점을 겹치지 않는 새 구현/runtime 실패 관점 20개를 기록한다. 최종 PR diff는 필수 프로젝트 규칙대로 별도 30개 관점으로 검토한다. Android/iOS Simulator 화면을 PR 본문에 넣고 이미지 링크가 실제로 보이는지 Agent Browser로 확인한 뒤 브라우저 프로세스를 정리한다.
8. 현재 PR의 merge state와 required checks를 확인하고 Rebase and merge한다. 병합 뒤 Tailnet preview를 선택 build, 백업·SHA 검증, Agent Browser 페이지 확인으로 동기화하고 browser를 닫는다.

## 구현 결과

두 depth-three scene에서 root → middle → branch → inner 네 Grid와 Ahem direct-text leaf의 parent/order, 두 축 intrinsic track, frame 및 Grid contribution trace를 확인했다. min-content는 각 Grid와 text가 `70×20px`, Columns `[70px]`, Rows `[20px]`; max-content는 `100×10px`, Columns `[100px]`, Rows `[10px]`다. Android와 iOS Simulator가 각각 기존 여덟 controls를 포함해 10/10 scene 및 draw/present gate를 통과했다.

[Android/iOS 실행·화면·artifact hash·source revision 및 제한](../spec/internal/evidence/c11-4-nested-grid-depth-three-runtime-2026-10-22/README.md) · [구현/runtime 실패 관점 검토 20개](./c11-4-nested-grid-depth-three-implementation-review-20.md) · [최종 PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-4-nested-grid-depth-three-runtime-2026-10-22/pr-diff-review-30.md). 초기 Android 캡처는 이전 8-scene fixture APK를 사용해 신규 ID에서 멈췄다. fixture APK 재빌드 뒤 10/10을 통과했고 로컬·설치 APK hash가 일치한다. 실패했던 개별 시도는 구현 성공 근거로 세지 않는다.

## 범위 밖과 완료 기준

- depth-four 이상, arbitrary nesting, multiple/spanning children, multiple tracks, anonymous text runs, replaced items, subgrid, gap, inline descendants, writing mode, percentage/calc, authored min/max, font/layout mutation은 제외한다.
- screenshot은 화면 표시 보조 자료다. glyph pixel equality, border paint, iOS compositor-after pixels, WPT, Android hardware GPU, 실기기 Android 및 전체 CSS Grid 지원을 주장하지 않는다.
- 두 새 fixed scene, 기존 controls, Rust·FFI 경계, 두 Simulator, product/fixture artifact 경계와 분리된 리뷰 근거가 모두 통과해야 이 slice의 구현 근거로 기록한다.
- C11.4 전체 상태는 미완료로 두며 공식 72/213 완료 수를 바꾸지 않는다.

## 검토 기록

- [계획 실패 관점 20개](./c11-4-nested-grid-depth-three-plan-review-20.md)
- [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/plan-pr-diff-review-30.md)
- [Chrome 156 사전 비교·기대값·화면·SHA-256](../spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/README.md)
