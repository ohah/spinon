# C11.4 · nested Grid positive implicit row contribution 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR [#447](https://github.com/ohah/spinon/pull/447)이 2026-10-21 Rebase and merge됐다. 계획의 두 scene을 기존 C11.4 opt-in fixture 경로로 구현했고, direct-layout/runtime/FFI 검사와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 각각 2/2 마쳤다. outer Rows `[18,18]`/`[40,0]px`, inner Rows `[8,8,16]px`, 다섯 node frame, trace, draw/present가 일치한다. 구현/runtime 실패 관점 검토를 기록했고, 구현 PR의 diff 검토·merge는 진행 중이다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다. [사전 비교·독립 계산·화면](../spec/internal/evidence/c11-4-nested-implicit-row-precomparison-2026-10-21/README.md) · [구현·Simulator 실행 근거](../spec/internal/evidence/c11-4-nested-implicit-row-runtime-2026-10-21/README.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-nested-implicit-row-runtime-2026-10-21/implementation-review-20.md) · [계획 실패 관점 검토 20개](./c11-4-nested-implicit-rows-plan-review-20.md)

## 목표와 범위

바깥 Grid의 direct item인 일반 nested Grid 한 단계가 **양의 implicit row 두 개**를 만들 때, 내부 행 크기와 nested Grid의 block-axis intrinsic contribution이 바깥 row track sizing까지 전달되는 경로를 고정한다. 현재 text/image/nested row fixture는 이 교차 범위를 포함하지 않는다. 이 문서는 구현 완료나 일반 CSS 지원 계약이 아니다.

고정 입력은 다음 두 장면이다.

- nested Grid에 명시 행 하나(`min-content`)를 두고, `grid-row: 3 / 4`인 image가 positive implicit row 두 개를 만든다. `grid-auto-rows: min-content max-content`의 반복 패턴에서 첫 implicit row에는 8px block, 둘째 implicit row에는 natural-size 24×16px image가 놓인다. 그 사이 첫 implicit row에는 4×8px fixed block을 둔다.
- 바깥 Grid는 `min-content min-content` 행 두 개와 4px row gap을 사용한다. nested Grid는 장면 하나에서 두 행을, 다른 장면에서 첫 행 하나만 차지한다.

각 Grid의 inline-size는 24px로 고정한다. 모든 box의 margin, border, padding은 0이며, 내부와 바깥 row gap은 각각 4px이다. 내부 track 기대 크기는 `[8,8,16]px`, nested intrinsic block contribution은 `8 + 4 + 8 + 4 + 16 = 40px`이다. 독립 규범 계산과 Chrome 관찰의 파일·역할 분리는 [사전 비교 README](../spec/internal/evidence/c11-4-nested-implicit-row-precomparison-2026-10-21/README.md)에 있다.

| scene | nested outer-row placement | 규범 기대 outer rows | root / nested block-size |
| --- | --- | --- | --- |
| `span-two` | `1 / 3` | `[18,18]px` | `40px / 40px` |
| `one-row` | `1 / 2` | `[40,0]px` | `44px / 40px` |

## 사전 oracle과 판정 기준

- 규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §§7.5–7.6, 11.3–11.5.1이다. §7.5의 명시 범위 밖 배치가 implicit grid track을 만든다. §7.6의 auto-row list는 첫 명시 행 뒤에서 list의 첫 크기부터 순환한다. §11.3–11.5의 intrinsic track sizing은 각 고정 block의 contribution을 사용하고, 두 행을 span하는 outer item은 §11.5.1 규칙으로 outer row gap을 뺀 기여 공간을 분배한다.
- 독립 기대값은 CSS track pattern 및 고정 입력에서 계산해 `spec-derived-expectations.json`에 별도 저장했다. Chrome 값을 기대값으로 복사하지 않았다.
- 기준 구현은 Agent Browser 0.31.1로 연 `HeadlessChrome/156.0.0.0`이다. viewport 1280×633 CSS px, DPR 1에서 fixture를 열고 image `decode()` 완료 뒤 used `gridTemplateRows`와 모든 box frame을 읽었다. 관찰 JSON과 화면은 `chrome156-observation.json`, `chrome156-nested-implicit-rows.png`다.
- 두 scene에서 내부 used rows `[8,8,16]`, nested height 40px, 바깥 rows `[18,18]`/`[40,0]`, root heights 40/44px를 각각 정확히 비교한다. 기대값은 정수 CSS px여서 허용 오차는 0px이다. DOM frame은 root 기준 좌표로 비교한다. viewport 절대 위치는 재현 확인용 관찰이며 규범 기대값이 아니다.
- 이미지가 모두 `complete`이고 natural size가 24×16px가 아니거나, scene/frame/track이 누락되면 oracle capture는 실패다. 이 결과는 Chrome 한 버전의 CSSOM 및 geometry 관찰이며 WPT나 여러 browser 적합성 근거로 확대하지 않는다.

## 구현 순서

1. 계획 PR #447을 Rebase and merge했다. 계획 검토 20개와 계획 PR diff 검토 근거는 이 계획과 사전 비교 evidence에 보존한다.
2. direct-layout fixture에서 기존 `LayoutInput`, Taffy track sizing, contribution 경로가 독립 기대값과 일치함을 확인했다. 제품 알고리즘 변경 없이 회귀 fixture를 추가했다.
3. 기존 C11.4 opt-in 경계 안에 두 scene과 strict runtime adapter를 추가했다. exact HTML topology, nested ownership, allowlisted 24×16 PNG resource, 두 positive implicit row 순서, outer placement를 검증하고 미등록 scene은 거부한다.
4. direct-layout/runtime 검사에서 root, nested Grid, 세 child frame, inner/outer Rows trace와 outer contribution item·kind·value·span을 확인했다. Android/iOS adapter는 두 Grid의 rows/columns, gap gutter 및 intrinsic source도 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 두 scene을 실행해 frame·used rows·trace·draw/present가 각각 2/2 통과했다. 실기기는 연결하지 않았다. 이 입력에는 border 바깥 overflow가 없고 Surface/Metal pixel readback도 없어 해당 주장을 하지 않는다.
6. 기본·fixture-test static archive를 빌드해 scene ID와 C14 PNG payload의 제외/포함을 positive/negative control로 확인했다. focused tests, workspace Clippy, fixture-module boundary 검사, Android/iOS Simulator app build가 통과했다. 전체 workspace test suite는 실행하지 않았다.
7. 구현/runtime 실패 관점 20개를 새로 검토했다. 구현 PR의 독립 diff 검토 30개, 발견 사항 반영 뒤 최종 diff 확인과 merge가 남아 있다. 계획 검토 표는 재사용하지 않는다.
8. PR에 Chrome 사전 비교와 가능한 Android/iOS Simulator 화면을 첨부한다. live required check과 merge state를 확인한 뒤 Rebase and merge한다. 비어 있는 status rollup을 CI 통과로 표시하지 않는다.
9. 계획, `spec/STATUS.md`, 내부 계약 0059와 공개 roadmap을 실제 결과에 맞춰 갱신한다. C11.4 parent checkbox와 `72/213` 집계는 전체 완료 기준이 충족될 때까지 유지한다. Tailnet 정적 preview는 병합 뒤 최신 main source hash를 확인해 갱신한다.

## 명시적 제외

- 자동 배치, negative implicit rows, auto-repeat, 2개를 넘는 implicit rows, 임의 auto-row list, named line/area, subgrid, 다단계 nesting
- percentage, `calc()`, flexible row, writing mode, baseline, aspect-ratio transfer, text shaping, async image completion
- border/overflow paint, physical Android, hardware GPU, iOS compositor 이후 surface, WPT와 browser 간 일반 적합성
- 일반 CSS admission/parser·공개 CSS API 지원, C11.4 또는 C11 전체 완료

## 완료 조건

두 고정 scene의 독립 계산, Chrome oracle, direct-layout trace, Android API 37 AVD와 iOS Simulator의 frame/track/draw/present, artifact 경계, 구현 검토와 PR diff 검토가 모두 결과 근거로 연결돼야 이 slice를 닫는다. 이는 해당 fixture 입력의 확인이며 일반 nested Grid나 전체 C11.4 지원을 뜻하지 않는다.
