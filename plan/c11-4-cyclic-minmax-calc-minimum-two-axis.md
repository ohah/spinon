# C11.4 · 양축 cyclic percentage `minmax(calc())` 최솟값 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 단일 축 최솟값 계획](./c11-4-cyclic-minmax-calc-minimum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획·사전 비교·고정 runtime fixture와 양 플랫폼 Simulator 검증 완료. 구현 PR [#493](https://github.com/ohah/spinon/pull/493)은 2026-10-22 02:41:32 UTC에 Rebase and merge됐고 merge SHA는 `3ffd2b5a39f27dc45d8235bdf830678e4d3c0a15`이다. 병합 당시 PR status check rollup은 비어 있었고 `main` required status checks는 설정되지 않았다. 구현 전용 fixture 조각이며 C11.4 공식 체크와 완료 수 `72/213`은 유지한다.

## 목적과 범위

직전 slice는 `minmax(calc(200% + length), max-content)`를 column 또는 row 한 축씩 적용했다. 이번 slice는 두 축에서 그 minimum을 동시에 사용하고, 각 축의 cyclic basis가 서로 섞이지 않는지 fixed runtime fixture로 확인한다. 양축 cyclic 장면은 Ahem `aa aa`로 column max-content basis `50px`, row basis `10px`을 만들어 최종 최소 track `[104px]`과 `[22px]`을 구분한다. 단일 축 cyclic control과 두 축 definite control을 함께 둔다.

새 ID 네 개만 `c04-runtime-gpu-fixture-test` C11.4 경로에 허용한다. HTML Grid root 하나와 direct text item 하나, horizontal writing mode, 한 column·한 row, gap 0, `2px` solid border, Ahem `10px/10px`을 고정한다. 일반 CSS admission, 여러 track/item, span, nested/anonymous/replaced item, writing mode 변형, WPT, 제품 CSS 지원, 내부 계약 버전 변경은 포함하지 않는다.

## 기준과 독립 기대값

기준 fixture는 [HTML 입력](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-two-axis-c114-precomparison.html)이다. 규범 출처는 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 [Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)이다. cyclic percentage는 Grid container의 intrinsic size 산정에서 `auto`처럼 취급한 뒤 결과 container size를 기준으로 layout할 때 resolve한다. 두 축은 각각 inner inline size와 inner block size를 basis로 쓴다. `minmax()` maximum이 minimum보다 작으면 minimum까지 올라간다.

독립 계산은 브라우저 관측과 분리해 고정했다. Ahem `aa aa`의 max-content inline size는 50px이며, 폭 10px에서 두 단어는 두 줄을 이루어 block size 20px이다. 양축 cyclic scene은 폭 50px·높이 10px의 intrinsic root를 만든다. 최종 column minimum은 `200% × 50 + 4 = 104px`, row minimum은 `200% × 10 + 2 = 22px`이다. Row-only scene은 definite 10px column에서 두 줄이 되어 intrinsic height 20px를 만들고, 최종 row minimum은 `200% × 20 + 2 = 42px`이다. 각 percentage는 content box 기준이며 2px border는 basis에 포함하지 않는다.

| Scene ID | Root sizing | Column / row track | Root content / border box | Item, root-relative |
| --- | --- | --- | --- | --- |
| `cyclic-minmax-calc-min-both-axes` | `max-content / max-content` | `[104] / [22]` | `50×10 / 54×14` | `(2,2) 104×22` |
| `cyclic-minmax-calc-min-column-definite-row` | `max-content / 10px` | `[104] / [10]` | `50×10 / 54×14` | `(2,2) 104×10` |
| `definite-column-cyclic-minmax-calc-min-row` | `10px / max-content` | `[10] / [42]` | `10×20 / 14×24` | `(2,2) 10×42` |
| `definite-minmax-calc-min-both-axes` | `10px / 10px` | `[24] / [22]` | `10×10 / 14×14` | `(2,2) 24×22` |

`2px` border를 뺀 content basis, 양축 동시 resolution, row text wrapping, root 밖 overflow를 각각 검사한다. 전체 독립 계산, Chrome 156 관측과 capture 환경은 [사전 비교 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-precomparison-2026-10-22/README.md)와 JSON에 기록한다.

## 구현과 검증 관문

1. 계획 실패 관점 20개를 기존 계획과 다른 질문으로 검토하고, 발견한 누락을 구현 전에 계획·fixture·판정 기준에 반영한다.
2. 양 축의 `GridTrackMinSizingFunction::Calc`가 distinct axis-owned `LayoutCalcId`와 percent basis를 유지하는지 style projection 및 layout regression으로 고정한다. 각 scene에서 both-axis trace, intrinsic contribution, root/item frame, min/max flooring과 실패 원자성을 확인한다.
3. 네 ID, CSS math handle owner/axis, DOM topology, text, style, track expression만 허용하는 opt-in adapter를 추가한다. FFI source allowlist와 별도로 존재하는 runtime C11.4 text-scene registry에도 네 ID를 각각 등록하고 등록 집합 테스트를 확장한다. 다른 ID·node shape/order·axis handle·expression은 부분 결과 없이 거부한다.
4. fixture-module boundary, Rust focused tests, rustfmt, workspace all-target Clippy와 Android/iOS fixture builds를 실행한다. 기본 product archive에서는 신규 symbol·scene ID·marker가 빠지고 fixture-test positive control에서는 포함되는지 비교한다. 이 결과를 APK/IPA 전체에서 fixture가 빠졌다는 주장으로 넓히지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 네 장면의 두 축 track, root/item frame, stage trace와 draw/present를 확인한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 border control, item interior, border overpaint, root 밖 item, item 끝/바깥을 표본화한다. CSS border stroke가 포함되지 않으면 border 색상 검증으로 표현하지 않는다.
6. 구현/runtime은 계획 검토와 별도로 새 실패 관점 20개로 리뷰한다. 최종 PR diff는 project rule의 30개 독립 관점으로 검토한다. 발견한 문제를 고친 뒤 영향받은 layout, artifact, simulator와 pixel 표본을 다시 확인한다.
7. PR은 C11.4를 완료 처리하지 않고 지원 경계와 미검증 범위를 적는다. PR screenshot은 Agent Browser에서 실제 표시를 확인한 뒤 해당 session을 종료한다. Live required checks와 merge state를 확인하고 Rebase and merge한다.

## 미지원과 판정

이 slice는 고정 fixture에서 cyclic percentage가 포함된 minimum `calc()`를 양 축에 함께 투입한 결과다. 일반 author CSS admission, WPT 적합성, arbitrary track grammar, Android 실기기·hardware GPU, iOS compositor-after pixel 및 전체 C11.4를 증명하지 않는다. 공식 체크 `C11.4`와 `72/213`을 그대로 둔다.

## 근거

- [독립 기대값](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-precomparison-2026-10-22/spec-derived-expectations.json)
- [Chrome Agent Browser 관측·screenshot·pixel 표본](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-precomparison-2026-10-22/README.md)
- [계획 실패 관점 검토 20개](./c11-4-cyclic-minmax-calc-minimum-two-axis-plan-review-20.md)
- [CSS Grid 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) · [Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)

## 구현 결과

네 scene을 기존 opt-in C11.4 fixture 경로에 연결했다. 양축 cyclic, column-only, row-only와 definite control이 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 4/4 통과했다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable 표본은 각 28/28 exact이며 root/item frame·양축 track trace·intrinsic input·draw/present가 기대값과 일치한다.

첫 AVD 실행에서 오래된 APK가 새 scene argument를 거부해 opt-in APK를 명시 재빌드했다. 재실행 중 Android expected-root helper의 definite-control fallback과 right-edge 표본 좌표가 각각 잘못된 것을 찾아 고치고 최종 전체 행렬을 두 플랫폼에서 다시 통과시켰다. 기본 release FFI archive 문자열 검사에는 신규 ID/payload와 C11.4 fixture ABI 이름이 없고 positive control에는 포함된다. archive 검사는 전체 APK/앱 번들 제외 증명이 아니다.

고정 fixture 이외의 일반 CSS admission, WPT, 실기기/hardware GPU 및 iOS compositor 이후 pixels는 미검증이다. [실행·화면·artifact 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-runtime-2026-10-22/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-runtime-2026-10-22/implementation-runtime-review-20.md) · [PR diff 적대 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-runtime-2026-10-22/pr-diff-review-30.md) · [로드맵 반영 PR diff 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-two-axis-runtime-2026-10-22/roadmap-pr-diff-review-30.md) · [계획 검토 21개](./c11-4-cyclic-minmax-calc-minimum-two-axis-plan-review-20.md).
