# C11.4 · definite minimum-size의 two-track auto-repeat 계획

**상위 원본:** [CSS Grid 계획](c11-css-grid.md) · [C11.4 track sizing 계획](c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 계획 검토·Chrome 155 사전 비교·제한 scene/harness 구현과 Android/iOS Simulator 21-scene 검증을 마쳤다. generic grid track-sizing 제품 경로는 수정할 필요가 없었고, app fixture gate·검증 표만 확장했다. 이 slice는 C11.4 또는 C11 완료 판정을 바꾸지 않는다. [실행 근거](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-2026-10-20/README.md).

## 목적

현재 C11.4 explicit-neighbor subset에서 고정 definite minimum-size와 두-track auto-repeat pattern을 조합한 앱 경로가 독립 기준에 맞는지 검증한다. 현재 13-case 앱 matrix는 one-track minimum-size와 two-track preferred-size를 각각 검증하지만 두 조건을 합친 runtime scene은 없다. 이 작업은 먼저 같은 cascade → typed projection → Taffy → app fixture 경로를 실제 입력으로 실행한다. 기존 generic code가 이미 맞으면 regression matrix와 원본 근거를 추가하고, 불일치가 재현되면 해당 경계만 수정한다. 코드 경로가 존재한다는 사실만으로 지원 완료를 주장하지 않는다.

## 계획 범위

- 한 grid axis에 `auto-fill` 하나를 둔다. `width:max-content; min-width:<px>` 또는 `height:max-content; min-height:<px>`만으로 minimum-size basis를 정한다.
- repeat fragment는 `minmax(40px, 1fr) 20px`의 두 track으로 고정한다. 첫 track의 max `1fr`는 repeat-count fallback에서 definite min `40px`을 사용하고 두 번째 fixed track은 `20px`이다.
- Prefix-only, suffix-only, 양쪽 neighbor를 각각 검사한다. Neighbor는 finite non-negative fixed `px`로 유지한다.
- columns와 rows를 mirror해서 검증한다.
- exact two-/three-/four-repetition boundaries, container/root frame, every used track, gap count, final stage trace를 비교한다.
- `auto-fit`의 minimum-size 조합도 Chrome 진단 행렬에 넣어 미해결 상호 운용 자료로 남긴다. 다만 이 slice의 acceptance와 구현 범위에는 넣지 않는다. Chrome 155는 같은 논리 입력에서 columns의 empty/span 사례는 두 번, rows는 세 번 반복했다고 보고했지만 Level 1은 빈 track 접기 전 `auto-fit` 반복 횟수가 `auto-fill`과 같다고 정한다. 이 관찰을 규범 기대값으로 복사하지 않고, 별도 조사에서 원인을 좁히기 전까지 `auto-fit` 동작은 바꾸지 않는다.

## 구현 전 oracle과 판정

[사전 비교 README](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-precomparison-2026-10-20/README.md)는 Chrome 155 observation, input hash, viewport, resolved tracks와 spec-derived expected values를 기록한다. 양쪽 neighbors가 있을 때 최소 content 사용량은 `38 + 76n` CSS px다. 따라서 min-size 190px은 2회·190px, 200px은 3회·266px, 342px은 4회·342px를 선택한다. Prefix-only 200px은 3회·248px, suffix-only 200px은 3회·238px다. Row axis는 같은 숫자를 block direction에 쓴다.

- oracle: CSS Grid Level 1 2025 CRD §7.2.3.1–7.2.3.2에서 반복 수를 도출하고, Chrome 155 Agent Browser observation으로 고정 수치·used tracks를 독립 대조한다.
- 입력: 저장된 HTML fixture, Chrome UA, macOS ARM64, DPR 1, viewport 1600×1500, 8px gap. fixture 및 JSON hash는 evidence README에 있다.
- 관찰: `gridTemplateColumns` 또는 `gridTemplateRows`, root/item rect, repeat count, gap 수, Rust final Columns/Rows stage trace, app node frames.
- 오차: 정수 track·frame은 정확 일치. Chrome의 subpixel serialization이 있는 track은 1/64 CSS px, 앱 frame은 0.05 CSS px 이내다.
- 실패: HTML declaration이 style projection에서 사라지거나 오류 없이 다른 template으로 대체되는 경우, 반복 수·track 순서·gutter count·root frame·trace 중 하나라도 어긋나는 경우.

## 구현 및 검증 경로

1. C11.4 app fixture에 계획 범위의 auto-fill scene을 추가하고 existing capture harness에 기대 root frame, track sequence 및 collapse 상태를 명시한다. Product Rust path가 기존 지원으로 정확하면 product source는 바꾸지 않고 fixture/evidence만 추가한다. Mismatch가 있으면 smallest responsible cascade/layout component를 수정한다.
2. Chrome oracle의 auto-fill cases를 app scene IDs와 1:1로 연결한다. Chrome auto-fit diagnostic은 spec mismatch를 숨기지 않도록 app acceptance matrix에서 분리한다.
3. Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator에서 모든 새 장면을 실행한다. Root/item frames, all used tracks, gap/collapse state, stage trace와 draw/present marker를 저장한다. 이 simulator 결과는 실기기·OS compositor 최종 pixel·WPT 적합성으로 확대 해석하지 않는다.
4. 구현 변경이 있으면 새롭고 독립된 실패 경로 20개를 구현 코드·런타임에 적용한다. PR diff에는 프로젝트 규칙의 30개 독립 관점을 별도로 기록한다. PR merge는 current required checks와 보호 상태를 확인한 뒤 Rebase and merge만 사용한다.
5. `spec/STATUS.md`의 C11.4/C11 체크와 공식 완료 수는 바꾸지 않는다. 이 slice는 전체 track sizing, WPT 또는 public API 지원을 닫지 않는다.

## 제외·남는 범위

Named lines/areas, percentage/calc neighbors, 두 axis 동시 auto-repeat, 여러 auto-repeat, repeated integer syntax, variable gap, other multi-track patterns, 10,000-track limit, general auto-placement/span/gutter matrix, WPT suite 및 auto-fit min-size axis discrepancy는 이번 acceptance에서 제외한다. Exclusion은 해당 기능이 미지원이라고 새로 확정하는 뜻이 아니며 각 항목은 검증 미완료로 남는다.

## 단계 관문

- 계획 자체의 서로 다른 실패 관점 20개 검토를 [plan-review-20.md](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-precomparison-2026-10-20/plan-review-20.md)에 기록하고 계획 PR을 병합한 뒤 구현한다.
- 구현 후 새로운 runtime/code 실패 관점 20개를 같은 작업 evidence에 별도 파일로 둔다.
- plan review와 구현 review를 섞거나 PR diff review를 대체하지 않는다.

## 구현 결과 2026-10-20

- Rust scene source, Android Java 및 iOS Swift app fixture의 scene allowlist와 expected root/item/node frame, box-count 판정을 8개 case에 맞췄다. product track-sizing algorithm은 수정하지 않았다.
- 첫 iOS 실행은 추가된 scene ID가 app allowlist에 없어 명시적 argument error를 냈다. 두 platform gate를 고친 뒤 같은 전체 matrix를 재실행했다. [초기 오류·수정 후 결과](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-2026-10-20/README.md).
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 21/21 기존·신규 scene을 확인했다. 신규 8개는 root frame, used tracks, 모든 active 8px gutter, collapse 상태, trace chunk 및 draw/present marker를 검사했다.
- [구현 실패 관점 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-2026-10-20/implementation-review-20.md)를 새로 수행했다. Chrome `auto-fit` definite minimum-size axis discrepancy, WPT, 실기기, hardware GPU와 iOS compositor-after 결과는 계속 남는다.

구현 PR [#365](https://github.com/ohah/spinon/pull/365)은 2026-10-20 KST Rebase and merge됐다(merge commit `a2c137213f72e82b3a8e3cc153e92c8b9db9a653`). 별도 구현 검토와 최종 PR diff 검토를 완료했지만 C11.4 check와 공식 완료 수는 그대로 미완료로 둔다.
