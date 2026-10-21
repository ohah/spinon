# C11.4 nested Grid의 행 축 intrinsic contribution 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 156 고정 fixture 사전 비교와 CSS Grid 2025 CRD 독립 계산을 마쳤다. 아직 Spinon runtime/trace 또는 앱 시뮬레이터 결과는 없다. 이 계획은 일반 nested Grid 지원이나 C11.4 완료 판정이 아니다.

## 목적과 경계

기존 [nested Grid text/image slice](./c11-4-nested-grid-replaced-item.md)는 한 단계 nested Grid의 Ahem text와 자연 크기 `<img>`를 outer Grid item contribution에 연결했다. 다음 slice는 같은 형태의 입력에서 **outer row track이 nested Grid의 block-axis intrinsic contribution을 받는 경우**를 고정한다. 열과 행을 함께 다루는 일반 CSS Grid 구현을 가정하지 않으며, 현재 adapter가 이 입력을 처리하는지 먼저 실행해 차이를 확인한다. 실패가 재현될 때만 제한 fixture 경로를 수정한다.

고정하는 항목은 한 단계 nested Grid, Ahem `XX XXXXXXX`, 저장소의 natural-size PNG, 두 개의 outer row, 세 개의 root scene이다. 첫 scene에서 nested item은 두 outer row를 span하고, 두 번째에서는 첫 row 하나만 차지한다. 세 번째는 definite block-size와 flexible row를 사용하는 automatic-minimum control이다. `subgrid`, 임의 중첩 깊이, 임의 child/style 수집, 일반 HTML/CSS admission은 범위에서 제외한다.

## 고정 입력과 oracle

규범 기대값은 [CSS Grid Layout Module Level 1, 2025 CRD §6.6, §11.1, §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에서 독립 계산한다. §11.1에 따라 각 Grid의 columns sizing을 rows sizing보다 먼저 수행하고, 행 기여를 계산할 때 이미 계산된 column size를 사용한다. §11.5.1의 spanning item 분배는 outer row gap을 기여값에 중복 포함하지 않는다. §6.6은 definite-flex control의 spanning automatic minimum 조건에만 적용한다. Chrome은 비교 관찰자이며 규범 기대값의 근거가 아니다.

공통 nested Grid와 leaf 입력은 다음과 같다.

- outer Grid content width `98px`, columns `98px`, `row-gap:4px`, padding/border `0`.
- nested Grid는 outer Grid의 direct item이며 width/height는 `auto`; 내부 columns `min-content max-content`, rows `min-content min-content`, 각 gap `4px`.
- Ahem 10px text `XX XXXXXXX`, `line-height:10px`, normal wrapping. 고정 font에서 longest word는 `70px`, unwrapped advance는 `100px`이다.
- 두 번째 inner row/column의 direct HTML image는 저장소 PNG의 natural size `24×16px`이다.
- font가 ready이고 이미지 세 개가 모두 decode된 뒤에만 세 scene을 측정한다. 원본 HTML, font, PNG, 관측 JSON, 화면의 SHA-256과 Chrome metadata는 [사전 비교 근거](../spec/internal/evidence/c11-4-nested-grid-rows-precomparison-2026-10-21/README.md)에 둔다.

## 독립 계산과 사전 관측

열 pass에서 text min-content track `70px`, image max-content track `24px`, column gap `4px`가 `98px` outer width를 채운다. text는 70px column에서 두 줄이 되어 `70×20px`가 된다. inner min-content rows는 `[20,16]px`이고 inner row gap을 더한 nested Grid의 block contribution은 `40px`이다.

| 장면 | outer rows와 root height | nested frame | text frame | image frame |
| --- | --- | --- | --- | --- |
| `row-span-two` | `[18,18]px`, `40px` | `(0,0) 98×40px` | `(0,0) 70×20px` | `(74,24) 24×16px` |
| `row-one-track` | `[40,0]px`, `44px` | `(0,0) 98×40px` | `(0,0) 70×20px` | `(74,24) 24×16px` |
| `row-definite-flex-control` | `[28,28]px`, `60px` | `(0,0) 98×60px` | `(0,0) 70×20px` | `(74,24) 24×16px` |

첫 scene은 40px nested contribution에서 outer 4px row gap을 빼고 남은 36px을 두 spanning min-content row에 균등하게 나눠 `[18,18]px`를 만든다. 두 번째 scene은 같은 40px 기여를 첫 row 하나에 반영하고 뒤 row는 0px으로 남긴다. root height에는 4px outer gap이 포함되어 44px이다. control은 root height `60px`, 두 `minmax(auto,1fr)` rows와 4px gap을 쓴다. §6.6에 따라 두 flexible row를 span하는 item의 automatic minimum은 0px이다. 56px의 자유 공간을 두 flex track이 나눠 `[28,28]px`가 되고 nested Grid는 60px로 stretch된다.

Agent Browser `HeadlessChrome/156.0.0.0` 관측은 위의 세 track 배열, root 크기, nested/text/image frame과 일치했다. 이것은 단일 Chrome build의 관찰일 뿐이며 browser 간 적합성·Spinon 동작을 증명하지 않는다. 실제 JSON과 고정 화면은 [사전 비교 근거](../spec/internal/evidence/c11-4-nested-grid-rows-precomparison-2026-10-21/README.md)에 보존했다.

## 구현 및 판정 절차

1. 고정 HTML, Ahem font, PNG와 Chrome observation의 SHA-256을 다시 확인한다. Chrome observation과 독립 기대값을 서로 대체하지 않는다.
2. 기존 nested Grid fixture adapter를 살펴 세 root scene을 고정 allowlist의 한 revision 입력으로 추가한다. root/nested/text/image node identity, font/image intrinsic source, document/style revision과 행·열 track trace를 기록한다.
3. Runtime focused tests에서 outer row tracks, inner tracks, 모든 node frame, contribution source/axis/span, scene revision을 각각 비교한다. image/font 미준비, 추가·누락 child, 변경된 unsupported style은 부분 layout을 내지 않고 명시적으로 거부한다.
4. `c04-runtime-gpu-fixture-test` opt-in fixture 앱으로 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 세 scene을 실행한다. 기대 track/frame, trace, draw/present, 앱 산출물과 설치 산출물 SHA-256을 묶어 기록한다. 선택된 환경은 simulator이며 Android 실기기는 이 slice의 gate가 아니다.
5. 기본 제품 archive에는 전용 fixture symbol·scene 문자열·Ahem/PNG payload가 없는지, `c04-runtime-gpu-fixture-test` archive에는 positive control로 포함되는지 각각 확인한다. Android/iOS fixture 빌드와 Rust focused tests, `cargo fmt --all -- --check`, workspace Clippy를 수행한다. 실행하지 않은 검사·플랫폼은 통과로 기록하지 않는다.
6. 구현/runtime 결과는 이 계획 검토와 다른 실패 관점 20개로 검토한다. 최종 PR diff도 현재 사용자 지시에 따라 독립 실패 관점 20개로 검토한다. 차이를 발견하면 수정 후 영향을 받는 관점을 다시 확인한다.
7. PR 본문에는 계획/구현 상태 ID, 남은 범위, 재현 화면과 증거 링크를 넣는다. 현재 GitHub required check 상태와 PR 상태를 새로 확인한 뒤 Rebase and merge한다. PR에 올린 screenshot image가 Agent Browser에서 실제로 로드되는지 확인하고 브라우저 프로세스를 종료한다.

## 범위 한계

- 세 고정 scene과 이 입력의 한 단계 nested Grid만 대상으로 한다. arbitrary nested/anonymous Grid, subgrid, 임의 CSS parsing, 임의 child count와 다른 writing mode는 포함하지 않는다.
- Ahem 고정 font와 natural PNG 한 종류만 사용한다. 일반 font/shaping, glyph paint, aspect-ratio transfer, author sizing constraint, async image completion은 검증하지 않는다.
- Browser observation은 Chrome 156 한 버전이다. WPT, 다른 browser, CSS Grid 전 범위 적합성을 주장하지 않는다.
- 계획 단계에서는 Android/iOS 앱 실행을 하지 않았다. 구현 이후 증거는 Android API 37 ARM64 AVD와 iOS 26.2 Simulator로 제한하며 물리기기·hardware GPU·iOS compositor 이후 pixel을 증명하지 않는다.
- 이 slice는 C11.4 전체 완료가 아니다. `spec/STATUS.md`의 C11.4 미완료 checkbox와 공식 완료 수 `72/213`을 유지한다. 내부 계약 0059 숫자 버전은 `0.1.0`으로 고정한다.

## 계획 근거

- [계획 실패 관점 20개](./c11-4-nested-grid-rows-plan-review-20.md)
- [Chrome 156 사전 비교·독립 계산·SHA-256](../spec/internal/evidence/c11-4-nested-grid-rows-precomparison-2026-10-21/README.md)
- [계획 PR diff 실패 관점 20개](../spec/internal/evidence/c11-4-nested-grid-rows-precomparison-2026-10-21/plan-pr-diff-review-20.md)
- [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)
