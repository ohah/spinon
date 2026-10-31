# C11.4 anonymous Grid text computed `word-break` 계획

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [CSS 규범·독립 글꼴 advance 기대값·Chrome 156 관측 12/12](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/README.md) · [고정 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-word-break.html) · [계획 실패 관점 검토 20개](./c11-4-anonymous-grid-text-computed-word-break-plan-review-20.md) · [계획 PR 변경 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/plan-pr-diff-review-30.md)

## 목표와 범위

기존 opt-in anonymous direct-text Grid adapter가 익명 Text의 상속 스타일 소유자인 동일 Grid container의 computed `word-break`와 언어 메타데이터를 paragraph text input 및 Parley intrinsic sizing에 전달한다. `normal`, `break-all`, `keep-all`을 지원하고 기존 `WordBreakMode`에 Parley 0.12.0이 이미 표현하는 `KeepAll` 변형을 추가한다. 한국어 대조 장면은 바깥 `<html lang="en">`과 별도로 해당 Grid container에 직접 둔 `lang="ko"`를 읽어 Parley `Locale`로 전달한다. 이 고정 범위는 언어 상속이나 `xml:lang` 처리를 추가하지 않는다. CSS Text Level 3의 deprecated `break-word`는 `overflow-wrap:anywhere` 결합 동작이 현재 paragraph profile에 없으므로 명시적으로 거부한다.

고정 입력은 horizontal-tb/LTR, normal whitespace, `line-break:auto`, `overflow-wrap:normal`, Ahem 10px/400 Latin `XXXX`·`XX XX`, 그리고 고정된 Noto Sans KR 2.004 10px/400 한국어 `가나다라`다. `keep-all` 비교의 Grid root에는 `lang="ko"`를 둔다. Grid root에 direct Text child 하나, 0 gap/edge, 120px definite width, 한 개의 `min-content` 또는 `max-content` column과 auto row를 사용한다. 총 12개 기준 scene은 [사전 비교 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/README.md)에 고정되어 있다.

일반 요소 Grid item의 paragraph 경로는 이미 computed `word-break:break-all`을 변환하지만 anonymous direct text child는 별도 `prepare` 경로를 사용한다. 이 계획은 그 별도 경계를 확장한다. 한국어 `keep-all` 대조는 익명 Text의 정확한 Grid style owner에 명시된 `lang=ko`만 허용하고, owner에 `xml:lang`이 있거나 `lang`이 누락·변형되면 거부한다. 바깥 문서 요소의 `lang=en`은 owner의 `lang=ko`를 대체하지 않아야 한다. 일반 언어 상속 기능은 선언하지 않는다. 새 장면과 Noto Sans KR bytes는 `c11-4-grid-track-sizing-fixture-test`로만 opt-in한다. 일반 앱 작성 API, 임의 글꼴 fallback, CSS `break-word`, `overflow-wrap`, 임의 `word-break` 조합, 모든 언어·line-break 조합, WPT 전체, glyph/surface paint 및 전체 C11.4/C11 완료는 선언하지 않는다. 내부 계약 숫자 `0.1.0`과 공식 완료 수 `72/213`을 유지한다.

## 규범과 독립 기대값

[CSS Text Level 3 §5.1](https://www.w3.org/TR/css-text-3/#word-break-property)은 이 상속 속성의 `normal`, `break-all`, `keep-all`, deprecated `break-word` 값을 정의하고 그 효과가 intrinsic size에 반영된다고 명시한다. Grid track 판정에는 [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)과 [CSS Grid Level 1 §11](https://www.w3.org/TR/css-grid-1/#algo-track-sizing)을 적용한다.

Chrome 156과 글꼴 테이블에서 독립적으로 만든 기대값은 [12개 장면 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/README.md)에 있다. Ahem에서 `normal`의 `XXXX`/`XX XX` min-content는 40px/20px이고 `break-all`은 10px/10px이다. 각각의 max-content는 두 mode 모두 40px/50px다. Noto Sans KR 400의 `가나다라` glyph advance는 각 9.2px다. `lang=ko`의 `normal` min-content는 9.2px, `keep-all`은 36.8px이며 max-content는 둘 다 36.8px다. line-height 10px를 적용하면 해당 min-content row 높이는 각각 40px·20px·40px·10px이다. Chrome track 값은 1/64px layout unit까지 기록하며 최대 양자화 차이는 0.0125px다. Runtime의 Noto geometry는 독립 글꼴 테이블 기대값에서 0.05 CSS px 이내, Chrome oracle은 0.02 CSS px 이내로 판정한다. computed keyword와 Ahem geometry는 정확히 대조하고, runtime line count는 고정 10px line-height에서 독립 기대 row 높이와 일치해야 한다.

## 구현과 검증 순서

1. 계획과 독립 기대값을 구현과 별도로 서로 다른 실패 관점 20개로 검토한다. 누락을 발견하면 계획·scene·기대값을 먼저 수정한다.
2. 계획 변경 PR diff를 별도 30개 관점으로 검토하고 사전 비교 화면을 본문에 첨부할 수 있게 한다. 계획 PR을 `Rebase and merge`한 뒤 최신 `main`에서 구현 브랜치를 만든다.
3. `spinon-text-layout::WordBreakMode`에 `KeepAll`을 추가하고 Parley 0.12.0의 `WordBreak::KeepAll`로 연결한다. `ParagraphStyle`에는 optional language tag를 추가해 기본값 `None`을 보존하고, 명시 값은 Parley `StyleProperty::Locale`로 전달한다. 기본 `Normal`, 기존 `BreakAll`, paragraph validation, normal line wrapping과 intrinsic sizing 회귀를 보존한다. 고정 Noto Sans KR font bytes는 opt-in C11.4 fixture path에만 등록한다.
4. anonymous text fixture 하위 `word_break` module에서 scene별 computed serialization allowlist를 관리한다. 익명 Text의 정확한 Grid style owner와 같은 revision snapshot에서 `normal`·`break-all`·`keep-all`만 typed paragraph style에 연결한다. Korean scene은 그 Grid owner에 직접 설정된 HTML `lang="ko"`, XML `lang` 부재, computed Noto font identity를 확인한다. owner의 locale 누락·변형·충돌, 바깥 `<html lang>`만으로 locale을 추론하는 입력, scene/value 불일치, `break-word`, 미지원 값, missing property, stale revision 및 wrong owner를 layout publication 전에 거부한다. Parley language tag 파싱 실패도 거부한다.
5. opt-in source route에 12개 unique scene을 등록한다. Rust expected record는 computed CSS, Grid owner language attribute/style revision, font identity, `WordBreakMode`, paragraph intrinsic min/max, line count/advance, item frame 및 Columns/Rows final track을 scene별로 1:1 대조한다. Noto expected geometry는 9.2/36.8px 산식에서 만들고 Chrome 1/64px serialization 차이를 허용 오차와 분리해 기록한다.
6. `spinon-text-layout`에서 Ahem `break-all`, Noto Sans KR `normal`과 `keep-all`의 줄바꿈·내재 크기 차이를 고정 font bytes로 검증한다. `lang=ko`가 Parley Locale metadata로 전달되는지 확인하되, pinned Parley 0.12.0의 `line-break:auto` strict segmenter에서는 한국어 locale이 줄 분할을 바꾸지 않음을 no-locale 대조로 확인한다. `keep-all`은 no-wrap fallback이 아니라 `normal`과 비교해 min-content track 및 row height를 바꾸어야 한다. [고정 Parley line segmenter 구현](https://github.com/linebender/parley/blob/v0.12.0/parley_engine/src/analysis.rs).
7. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각 12/12 장면을 실행한다. Rust/runtime trace와 설치 artifact hash를 연결하고 root track, row, revision과 지원·기대 거부를 확인한다. 검정 WGPU 렌더러와 simulator screenshot만으로 glyph paint나 compositor 이후 픽셀 일치를 주장하지 않는다.
8. 기본 제품 archive와 C11.4 fixture-test 양성 대조군을 따로 빌드한다. 신규 scene marker 및 10MB Noto Sans KR payload는 기본 artifact에서 없어야 하며 fixture-test artifact에는 예상 bytes/hash와 scene만 있어야 한다. static archive 검사만으로 APK/IPA 전체 경계를 주장하지 않는다.
9. `cargo fmt`, 관련 text-layout/runtime/FFI tests, CSS fixture-module boundary, feature-scoped Clippy, Android/iOS simulator builds를 실행한다. 구현 전 실패 기준선과 새 실패를 분리하고 실행하지 않은 gate를 통과로 기록하지 않는다.
10. 구현 코드와 실제 runtime을 계획 검토와 겹치지 않는 신규 실패 관점 20개로 검토한다. 구현 PR diff는 별도 30개 관점으로 검토한다. 결함을 수정하면 영향받은 관점을 다시 확인한다.
11. 구현 PR에는 Chrome 사전 비교와 가능한 Android/iOS 동작 화면을 실제 이미지로 첨부한다. 이미지는 GitHub PR 본문에서 로드되는지 확인하고, 화면이 증명하지 못하는 glyph/surface 범위를 적는다. 현재 required checks와 PR check rollup을 재조회한 뒤 `Rebase and merge`하고 최종 merge SHA와 상태를 확인한다.
12. 구현 PR에서 내부 계약 0059에 고정 profile과 API metadata 경계를 기록한다. 병합 뒤 `spec/STATUS.md`, C11.4/상위 계획과 공개 roadmap을 실행 결과·제외 범위에 맞춰 동기화한다. 이 slice로 C11.4/C11 checkbox나 `72/213`을 승격하지 않는다.

## 완료 판정

이 계획의 완료는 세 가지 `WordBreakMode` 값 중 `normal`·`break-all`·`keep-all`을 정확한 12개 opt-in anonymous-text scene에서 처리하는 제한 구현, 별도 `break-word` 거부, 독립 oracle, Noto/Ahem text-layout regression, Android/iOS Simulator 행렬, 기본/fixture artifact 경계 및 분리된 계획·구현·PR 검토 근거가 모두 확인된 상태다. 일반 CSS author admission, 전체 CSS Text `word-break`, 임의 언어/글꼴, WPT, glyph/surface paint, C11.4 또는 C11 전체 완료는 포함하지 않는다.
