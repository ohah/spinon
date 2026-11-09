# C11.4 · 반복 텍스트 자동 배치의 fit-content 위치 2–6

- **상태:** 제한 구현 병합 · 상태·명세·로드맵 동기화 · C11.4/C11 공식 체크 미완료 · `78/214` 유지
- **계획 PR:** [#1301](https://github.com/ohah/spinon/pull/1301) · Rebase and merge · `512f634fed403a1398d9dac3fc34995972ac2734`
- **구현 PR:** [#1302](https://github.com/ohah/spinon/pull/1302) · Rebase and merge · `2026-11-09T03:03:35Z` · `3c403dc265933407dcd34154012ceeec871d71c8`
- **구현 근거:** [1,440-scene 비교, simulator smoke, 회귀 수정과 한계](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-implementation-2026-11-09/README.md)
- **병합 후 상태·로드맵 동기화:** [기록과 문서 diff 검토](../spec/internal/evidence/c11-4-repeated-text-autoplacement-fit-content-positions-roadmap-sync-2026-11-09/README.md)
- **직전 구현:** [반복 text row-span의 후속 fit-content 위치](./c11-4-root-html-repeated-text-later-fit-content-row-span.md) · PR [#1299](https://github.com/ohah/spinon/pull/1299)
- **사전 비교:** [Chrome 157 1,440 scene와 독립 관계 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/README.md)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-autoplacement-fit-content-positions-plan-review-20.md)

## 결손과 목표

기본 parsed Root HTML의 `RootHtmlFitContentAutoRowsV1`은 여섯-member repeated intrinsic `grid-auto-rows` list를 받는다. Fit-content position 1은 9-item repeated-text auto-placement profile에서 허용한다. PR #1299는 position 2·3을 제한 direct-text row-span profile에 추가했지만, 9-item auto-placement 조합은 명시적으로 제외했다. Positions 4–6은 해당 profile에서 사용하지 않는다. 이 계획은 여덟 implicit row를 실제로 생성하는 9-item profile에 fit-content의 모든 authored position을 연결한다.

이번 제한 slice는 기존 position 1을 회귀 대조로 유지하고 positions 2–6의 다섯 순서군을 추가한다. 여섯-member 목록의 multiplicity는 `fit-content(cap)` 하나, `min-content` 둘, `max-content` 둘, `auto` 하나로 고정한다. 각 위치에서 후행 다섯 member의 30개 고유 순서만 허용한다. 새 위치 다섯 개는 `5 × 30 × 4 caps × 2 alignments = 1,200` scene을 추가하며 position 1 대조를 합친 Chrome/runtime matrix는 총 1,440 scene이다.

이 작업은 9-item auto-placement profile에 한정한다. Span topology는 첫 세 implicit row만 사용하므로 span profile의 기존 position 1–3 경계를 바꾸지 않는다. 새 공개 API·FFI, 일반 author CSS admission 또는 C11.4/C11 완료 선언은 추가하지 않는다.

## 입력과 독립 비교 모델

- 기본 Root HTML Grid는 definite `40×220px`, explicit columns `[40px]`, explicit rows `[10px]`, gap 0, static positioning, visible overflow, LTR 및 `grid-auto-flow:row`다. `align-content:start|stretch`만 허용한다.
- 직접 자식은 빈 explicit item 하나와 implicit auto-placement item 여덟 개다. 각 implicit item은 direct payload `<div>` 하나만 포함한다. 모든 자식은 하나뿐인 explicit column에 배치된다.
- Text payload 순서와 고정 content는 `A A A A A A`, `AAAA AAAA A`, `AA AA AA AA`, `AAA AAA AAA`, `AAA AAA`, `A A A A A A`, `A A A A A A A A`, `AA`다. 기존 ASCII 대문자 단어 문법, 단어당 1–6 byte, 한 칸 ASCII 공백, 총 1–32 byte 제한을 유지한다. 고정 fixture에서 Chrome 157이 관측한 줄 수는 `[2,2,2,3,2,2,3,1]`이다.
- `grid-auto-rows`는 정확히 여섯 member다. Fit-content cap은 기존처럼 유한하고 0 이상인 직접 `px|%`만 허용한다. 고정 cap은 `20px`, `20%`, `0%`, `120%`다. definite 220px content block-size 기준에서 percentage cap은 44px, 0px, 264px다.
- Chrome Agent Browser 157 사전 비교는 position 1–6 × tail 순서 30개 × cap 4개 × 정렬 2개, 총 1,440 scene이다. 전체 CSSOM order, used row 크기, item/payload frame, text 줄 수, screenshot, fixture hash와 독립 verifier를 고정한다.
- 이 고정 폭 fixture의 독립 fit-content 관계는 `used row = min(10px × text 줄 수, cap)`이다. item `min-height:0`이므로 track 최소값은 0이다. 독립 verifier는 row 7/8 반복을 포함한 fit-content 1,920개를 확인하고, item frame과 used implicit row의 일치 및 payload가 Grid 밖으로 나오는지도 검사한다.
- 독립 정렬 관계는 CSS Grid sizing 후 content alignment를 적용하는 순서다. `stretch`는 남은 양의 공간을 생성된 `auto` track에만 똑같이 더한다. 다른 track은 `start` 크기를 유지한다. geometry 오차는 0.01 CSS px 이내, authored order·줄 수·item 수·cycle index는 정확히 일치해야 한다.

위치 4에서는 row 4가 text 세 줄을 갖는다. 고정 corpus에서 예상 fit row는 cap `20px`일 때 20px, `20%`와 `120%`일 때 30px, `0%`일 때 0px다. 첫 cycle의 위치 1–3, 5, 6은 두 줄 content로 각각 20px, 20px, 0px가 된다. 위치 1은 세 줄인 implicit row 7에서 반복되고 위치 2는 한 줄인 row 8에서 반복된다. 이 반복은 explicit row를 auto-row 목록에 잘못 포함하는 오류도 잡는다.

## 허용과 거부 경계

- 여섯-member list 길이와 multiplicity를 고정한다. 유한하고 0 이상인 `fit-content(px|%)` 하나는 9-item direct-text auto-placement profile에서만 위치 1–6 중 한 곳에 허용한다.
- `40×220` Grid shape, 단일 40px column, explicit row 10px, edge/gap 0, row sparse flow, 빈 item 하나, 표시 item 여덟 개, text/payload 문법과 계산 style 제약을 유지한다.
- 기존 9-item `grid-auto-flow:row` 배치를 유지한다. item의 `y` 차이만으로 track 식별자를 추정하지 않고 final row-track trace와 명시된 owner/cycle position을 사용한다.
- 다른 list 길이/multiplicity, 누락·중복 intrinsic keyword, 복수 fit-content, 미지원 cap, indefinite percentage basis, 잘못된/빈 text, 다른 자식 수, inline/nested/replaced content, 추가 Grid owner, implicit column, 다른 placement/alignment/overflow, stale revision, 모순되거나 누락된 final track trace는 계속 거부한다.
- Span profile은 fit-content 위치 1–3 제한을 유지한다. 해당 장면은 implicit row를 세 개까지만 만들므로 3/4-item span topology에서 위치 4–6은 계속 거부한다.
- 이 slice는 임의 repeated track list, 임의 text/font shaping, 일반 row auto-placement, 일반 `fit-content()` 동작, WPT 적합성, native 전체 matrix 일치 또는 C11.4/C11 전체 지원을 뜻하지 않는다.

## 구현과 검증 관문

1. 제품 코드를 바꾸기 전에 이 계획의 서로 다른 실패 관점 20개 검토와 계획 PR의 별도 30개 diff 검토를 마친다.
2. 기존 고정 multiplicity에서 유효한 fit-content 하나가 위치 1–6 어디든 올 수 있는 Style projection predicate를 추가한다. Row-span predicate는 위치 1–3을 따로 유지하고 현재 typed CSS 값과 순서를 보존한다.
3. Layout admission은 9-item repeated-text auto-placement topology의 위치 1–6에만 확장한다. Explicit row 제외, 여섯-member cycle 계산, implicit item 8개 owner, text 문법과 final track trace 검증을 유지한다. owner·row count·trace·cap basis·revision이 누락되거나 모순되면 output 발행 전에 실패한다.
4. 모든 fit 위치, tail 순서 30개, cap, 정렬, 위치 1/2의 반복 cycle에 대해 projection/Layout/RuntimeSession 양성 검증을 추가한다. 1,440개 결과를 고정 Chrome 관측 및 독립 fit/cycle/frame/stretch 관계와 비교한다.
5. 3/4-item row-span 위치 4–6, 잘못된 visible child 수, implicit-column placement, 잘못된 track multiplicity/list 길이, 중복·누락 fit-content, 잘못된 px/%/`calc()` cap, text/style 변형, 추가 Grid owner, overflow/hidden/flow 변형, stale owner/revision, 불완전·변조 row trace에 대한 거부 사례를 추가한다. 부분 layout/render 발행이 없고 같은 session에서 다음 valid input이 복구되는지 확인한다.
6. 기존 repeated-text 810-scene corpus, row-span 1,920-scene corpus, 모든 fit-content auto-row 회귀와 Grid track sizing 검사를 재실행한다. 영향받은 crate tests, `-D warnings` Clippy, `cargo fmt --all -- --check`를 실행하고 기존 기준 실패는 별도로 기록한다.
7. 동일 source revision을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML 앱 경로에서 실행한다. 위치 1–6, cap `20px|20%|0%|120%`, `start|stretch`를 확인하고 layout-ready, exact item/track trace, draw/present와 screenshot을 저장한다. OS screenshot은 visual smoke이며 PixelCopy/Metal 표면 readback이 아니다. 이번 검증에서 시작한 simulator와 Agent Browser만 종료한다.
8. 새 fixture/FFI가 추가되면 기본 및 fixture-test artifact를 확인한다. 기본 artifact에는 fixture 전용 심볼/자료가 빠지고 opt-in positive control에는 포함되어야 한다. runtime route에서 요구되지 않으면 fixture ABI를 추가하지 않는다.
9. 구현 뒤 새 실패 관점 20개로 implementation/runtime을 검토하고 전체 구현 PR diff도 별도 30개 관점으로 검토한다. PR 본문에는 검증된 Android/iOS screenshot 링크와 정확한 근거 범위를 적고, immutable raw URL은 merge 전 Agent Browser로 확인한다.
10. 병합 뒤 `spec/STATUS.md`, 내부 계약 `0059`, 이 계획과 공개 roadmap에 bounded 결과를 기록한다. C11.4/C11 체크는 미완료, 공식 `78/214`, 내부 버전 `0.1.0`을 유지한다. 문서 변경을 병합한 다음 source/hash를 확인해 기존에 승인된 Tailnet roadmap preview를 동기화한다.

## 완료 판정

제한 slice는 1,440-scene RuntimeSession matrix, 독립 cycle/cap/frame/stretch 관계, atomic 거부·복구 사례, 기존 text/span 회귀, 집중 Rust/Clippy/rustfmt 검사, Android/iOS 기본 경로 확인, 별도 implementation/runtime 20관점 검토, 별도 PR diff 30관점 검토와 병합 후 spec/status/roadmap 동기화가 모두 끝난 뒤에만 구현 완료로 본다. Chrome 브라우저 사전 비교만으로 구현을 완료 처리하지 않는다.
