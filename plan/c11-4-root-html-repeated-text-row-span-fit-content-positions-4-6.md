# C11.4 · 반복 텍스트 row-span의 fit-content 위치 4–6

- **상태:** 계획 후보 · 제품 코드 미변경 · C11.4/C11 공식 체크 미완료 · 78/214 유지
- **선행 구현:** [반복 text row-span fit-content 위치 2·3](./c11-4-root-html-repeated-text-later-fit-content-row-span.md) · PR [#1299](https://github.com/ohah/spinon/pull/1299)
- **선행 구현:** [반복 text auto-placement fit-content 위치 1–6](./c11-4-root-html-repeated-text-autoplacement-fit-content-positions.md) · PR [#1302](https://github.com/ohah/spinon/pull/1302)
- **사전 비교:** [Chrome 157 4,320 scene](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-precomparison-2026-11-09/README.md)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-plan-review-20.md)
- **계획 PR diff 검토:** [별도 30개 관점](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-precomparison-2026-11-09/plan-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML의 제한 RootHtmlFitContentAutoRowsV1에는 같은 여섯-member grid-auto-rows list를 쓰는 repeated-text row-span profile과 8개 item auto-placement profile이 있다. PR #1299는 row-span 입력에서 fit-content의 authored 위치 2·3을 추가했지만, 그 topology는 positive implicit row를 세 개까지만 만든다. PR #1302는 8개 auto-placement item에서 위치 1–6을 지원했지만 row-span profile은 확장하지 않았다.

이 계획은 실제 여섯 implicit row를 만드는 고정 row-span topology에서 fit-content 위치 4·5·6을 지원하는 bounded slice를 정한다. fit-content() 한 개, min-content 두 개, max-content 두 개, auto 한 개라는 기존 multiplicity와 direct-text grammar를 유지한다. 일반 CSS admission, 임의 row-span, WPT 또는 C11.4/C11 전체 구현을 선언하지 않는다.

## 고정 입력과 독립 비교 모델

- 기본 parsed Root HTML RuntimeRootHtmlTextPaintV1 route, definite 40×220px Grid, 40px explicit column과 10px explicit row, zero gap, LTR, visible overflow를 사용한다. 첫 direct child는 빈 explicit item row 1 / 2, column 1 / 2다. Text item은 모두 column 1 / 2에 definite row line과 span을 가진다.
- 각 scene은 explicit row 뒤에 정확히 여섯 positive implicit row를 만든다. 두 개의 고정 topology 가족을 각각 DOM source order 정방향과 역방향으로 실행한다.
  - span2-six: row 2 / span 2, row 3 / span 2, row 4 / span 2, row 5 / span 2, row 6 / span 2.
  - span2-span3-six: row 2 / span 2, row 3 / span 2, row 4 / span 3, row 5 / span 3.
- Text source는 각각 31·27·23·19·15 UTF-8 byte의 제한 ASCII 대문자/단일 공백 grammar다. 고정 폭·글꼴·line-height에서 Chrome이 관측한 줄 수는 A–E 순서로 6, 5, 4, 4, 3이다. payload와 text style은 기존 10px monospace / 10px line-height profile을 쓴다.
- 여섯-member track list는 fit-content authored 위치 4·5·6 각각에서 후행/선행 다섯 member의 30개 고유 순서를 허용한다. cap은 0px, 0%, 10px, 20px, 20%, 120%다. definite 220px block basis에서 20% = 44px, 120% = 264px다.
- align-content start|stretch와 네 topology를 포함한 Agent Browser Chrome 157 고정 비교는 3 positions × 30 tails × 6 caps × 2 alignments × 4 topologies = 4,320 scenes다. 전체 CSSOM order, seven used row sizes, item/payload frame, text line positions, screenshot, fixture hash와 독립 verifier를 보존한다.
- 독립 검증은 각 item frame의 y/height가 실제로 점유한 used row track 합과 일치하는지, source order를 뒤집어도 semantic placement 결과가 같은지, stretch가 non-auto track을 바꾸지 않는지 검사한다. 이 고정 corpus에서는 10px와 20px cap이 fit track 크기를 위치 4에서 48개, 위치 5에서 48개, 위치 6에서 24개 scene 바꾼다. 이는 해당 corpus의 관측치이지 모든 authored content에 대한 일반화가 아니다. 0px와 0%, 20px·20%·120%가 같게 나온 corpus 결과도 그대로 기록하며, fit-content used size가 항상 cap 이하라고 가정하지 않는다.
- 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items) spanning-item intrinsic track sizing과 [§10.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-align) post-sizing alignment다. Chrome observation은 독립 기준을 보완하는 고정 입력 cross-check이며 CSS 명세를 대신하지 않는다.

## 입력 경계와 실패 동작

허용할 입력은 위 Root owner, revision, Grid 크기, 한 개 explicit row/column, 여섯 implicit row topology, 여섯-member track multiset, cap, alignment, text grammar·computed style뿐이다. Row span trace의 owner, positive implicit track count, track index·size 또는 revision이 빠지거나 모순되면 scene을 거부하고 partial layout/render output을 publish하지 않는다. 동일 RuntimeSession에서 거부 뒤 다음 valid scene이 회복되는지 확인한다.

잘못된 list length/multiplicity, fit-content 중복·누락, 새 six-row topology에 1–3 위치를 결합하거나 기존 three-row topology에 4–6 위치를 결합한 입력, 3개 미만/7개 이상 implicit row, 지원 밖 span·column placement, auto placement와 span 혼합, 다른 Grid owner, nested/inline/replaced payload, text/style·writing-mode 변형, gap·border·padding, hidden/overflow 변형, 잘못된 cap, stale owner/revision, 누락·변조 track trace는 fail-closed한다. 기존 auto-placement positions 1–6과 row-span positions 1–3 profile의 허용 경계는 독립 regression으로 유지한다.

## 구현과 검증 관문

1. 계획 실패 관점 20개와 계획 PR 전체 diff에 대한 별도 30개 관점 검토를 마친 뒤에만 제품 코드를 바꾼다.
2. Style projection은 기존 track member의 타입·순서·multiplicity를 보존하며 row-span topology에서만 fit-content 위치 4–6을 허용한다. Auto-placement profile의 기존 위치 1–6 동작은 별도 regression으로 유지한다.
3. Layout은 정확히 여섯 positive implicit row를 가진 위 네 topology만 받는다. Item placement·owner·track trace·cap basis·revision을 한 snapshot에서 확인한다. 행 개수나 item y 위치를 추정해 track sizing trace를 대체하지 않는다.
4. Default Root HTML RuntimeSession route에서 4,320 scene 전부를 Chrome observation 및 독립 frame/track/order/alignment 관계와 비교한다. Unsupported scene은 atomic reject하고 같은 session에서 valid scene을 다시 실행한다.
5. 기존 row-span 1–3의 1,920-scene matrix, auto-placement 1–6의 1,440-scene matrix, 기존 repeated-text 810-scene corpus, fit-content track-list 회귀와 Grid track-sizing suite를 재실행한다. 영향 crate의 focused Rust tests, -D warnings Clippy, cargo fmt --all -- --check를 실행하고 broad-suite 기존 실패는 baseline이 입증되지 않는 한 분리해 보고한다.
6. 동일 source revision을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 실행한다. fit 위치 4–6과 두 alignment sentinel에 대해 HTML source hash, layout-ready, semantic item/row 결과, draw/present와 OS screenshot을 보존한다. Screenshot은 visual smoke이며 PixelCopy/Metal readback이나 compositor parity가 아니다. 이번 작업에서 시작한 simulator와 Agent Browser만 종료한다.
7. 새 fixture/FFI 경로는 추가하지 않는다. 필요성이 발견되면 기본 artifact에서 fixture ABI·payload 부재와 opt-in positive control을 각각 확인하기 전에는 연결하지 않는다.
8. 구현 뒤 이 계획 검토와 다른 실패 관점 20개로 구현/runtime을 검토하고, 구현 PR diff는 별도로 30개 관점으로 검토한다. PR 본문에 가능한 Android/iOS 화면을 연결하고 immutable raw URL을 Agent Browser로 확인한다.
9. 병합 뒤 spec/STATUS.md, 내부 계약 0059, 이 계획과 공개 roadmap에 bounded 결과·실제 merge SHA·실행 한계를 기록한다. C11.4/C11 체크는 미완료, 공식 78/214, 내부 숫자 계약 0.1.0은 유지한다. 승인된 Tailnet roadmap preview는 문서 merge 후 선택 경로만 백업·hash 검증을 거쳐 동기화한다.

## 완료 판정

계획된 4,320-scene RuntimeSession 비교, 독립 frame/track/source-order/alignment 관계, cap 경계 차이, atomic rejection·same-session recovery, 기존 profile 회귀, focused Rust/Clippy/rustfmt, Android/iOS default-route smoke, 별도 구현/runtime 20관점 및 PR diff 30관점 검토, 상태·명세·roadmap 동기화와 병합 결과를 확인한 경우에만 이 bounded slice를 구현 완료로 기록한다. 이 계획과 후속 구현은 전체 CSS Grid row-span, arbitrary text, WPT 적합성 또는 C11.4/C11 완료가 아니다.
