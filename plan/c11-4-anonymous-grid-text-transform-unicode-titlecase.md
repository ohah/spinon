# C11.4 anonymous Grid text Unicode `text-transform:capitalize`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 단계. 기존 ASCII English 제한 slice [PR #986](https://github.com/ohah/spinon/pull/986)의 다음 후속이다. 기존 runtime 행렬 12개(none/uppercase/lowercase 6개와 capitalize 6개)를 보존하고 11개 Unicode scene을 더한다. 이 계획은 C11.4/C11 완료나 공개 CSS 지원을 선언하지 않는다.

**사전 비교:** [Chrome 156 Agent Browser 관측, CSS Text·Unicode 규범 및 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/README.md) · [20개 계획 실패 검토](./c11-4-anonymous-grid-text-transform-unicode-titlecase-plan-review-20.md) · [계획 PR 변경 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/plan-pr-diff-review-30.md)

## 목표와 계약

기존 anonymous direct-text Grid fixture의 `text-transform:capitalize`를 ASCII 글자와 U+0020만 받는 경계에서 Unicode 단어 분절과 전체 titlecase 매핑으로 확장한다. [CSS Text Level 3 §1.4](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#characters-and-letters)는 typographic letter unit을 Unicode `Letter` 또는 `Number` general category에 속한 typographic character unit으로 정의한다. §2.1.1은 단어 경계를 UA 선택으로 두고 UAX #29 사용을 권고하며, Unicode full case mapping을 요구한다. `lang="en"`을 알고 있는 이 profile은 English locale titlecase mapping을 적용한다.

Chrome 156의 고정 `lang="en"` 관측은 U+00E9 `é`→`É`, hyphen 분리, apostrophe 내부, 열린 문장부호, 결합 악센트와 Number 첫 단위 `49ers`를 비교했다. Chrome은 `ßeta`와 `ßeta   éclair`의 U+00DF를 그대로 두지만 [Unicode 17.0.0 `SpecialCasing.txt`](https://www.unicode.org/Public/17.0.0/ucd/SpecialCasing.txt)는 U+00DF titlecase mapping을 `0053 0073` (`Ss`)로 정의한다. 구현 판정은 CSS Text의 full case mapping과 Unicode data를 따르고 이 Chrome 차이는 호환성 차이로 보존한다. 두 기준을 한 기대값으로 섞지 않는다.

단어 경계는 pinned `icu_segmenter = 2.3.0`의 UAX #29 version 17.0.0 `WordSegmenter`로 고정한다. 첫 typographic letter unit이 Unicode Lowercase property를 가지면 `icu_casemap = 2.3.0`의 English `TitlecaseMapper` full mapping을 적용하고 뒤 문자는 그대로 보존한다. 따라서 `ßeta`는 `Sseta`; `49ers`는 첫 typographic unit인 Number `4`가 소문자가 아니므로 그대로다. Locale, case mapping, property 및 segmenter 자료는 `c11-4-text-transform-unicode-fixture-test` opt-in feature 아래에 두고 기본 product artifact에는 새 case-mapping 데이터와 scene marker가 없어야 한다.

HostDocument의 원문 Text와 UTF-16 범위는 바꾸지 않는다. CSS Text의 처리 순서에 맞춰 `white-space:normal`의 Phase I collapse 뒤, Parley shaping과 intrinsic sizing 전에 transform한다. 하나의 원문 scalar가 여러 출력 scalar로 확장될 때 출력 각 scalar의 paragraph byte 범위가 같은 원문 UTF-16 범위를 가리켜야 한다. 인접한 direct Text node 경계는 단어 분절 경계가 아니다. style owner, direct `lang="en"`, HostDocument/style revision과 allowlisted scene을 모두 검사하고, inline run, 다른 언어, stale snapshot 및 profile 밖 입력은 layout publication 전에 거부한다.

## 비교 입력과 고정 범위

기준 fixture는 Noto Sans KR variable font `wght=400`, 10 CSS px, LTR, `white-space:normal`, 고정 Grid owner와 anonymous direct text다. 폰트 SHA-256, Chrome 156 CSSOM observations, 예상 output string, 독립 case-map 근거와 후보/Chrome 차이는 [사전 비교 README](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/README.md), [관측 JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/chrome156-observations.json), [expected JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/expected.json)에 분리해 둔다. CSSOM은 Chrome 1/64 CSS px quantization을 포함하고, 지원 값 비교 허용치는 0.03125 CSS px다. 표준·Unicode 데이터가 변환 문자열을 결정하고 Chrome candidate CSSOM 폭은 shaping 관찰값으로만 둔다. nominal `hmtx` 폭은 shaped width와 다르므로 CSSOM 기대 폭으로 취급하지 않는다. Unicode 17 full mapping에 따른 U+00DF 결과는 CSS Text·SpecialCasing으로 고정하며 Chrome의 관측 결과를 expected output으로 복사하지 않는다.

새 runtime matrix는 기존 12개 computed `text-transform` scene과 다음 11개 allowlisted Unicode scene을 합쳐 23개로 고정한다. 기존 12개는 none/uppercase/lowercase 각 min/max 6개와 ASCII capitalize 회귀 6개이며 모두 같은 source revision에서 재실행한다.

| scene | 입력·값 | 독립 expected 및 반례 |
| --- | --- | --- |
| `unicode-sharp-s-min` | `ßeta`, `min-content` | `Sseta`; 한 단어라 `min-content=max-content`, 예상 column 25.1875 CSS px. Chrome target은 20.96875px로 원문 `ßeta`와 일치해 규범 차이로 분류한다. |
| `unicode-sharp-s-max` | `ßeta`, `max-content` | `Sseta`, 예상 column 25.1875px; titlecase expansion 1→2 scalar와 원문 UTF-16 `0..1` provenance를 검사한다. |
| `unicode-source-split-max` | direct Text node `ß` + direct Text node `eta` | 하나의 anonymous item, `Sseta`; source node 두 개를 보존하고 split 경계로 capitalization이 초기화되지 않는다. |
| `unicode-accented-max` | `éclair`, `max-content` | `Éclair`; Chrome candidate track 25.953125px. |
| `unicode-combining-max` | `e` + U+0301 + `lan` | `E` + U+0301 + `lan`; source UTF-16 범위는 scalar별로 보존한다. Chrome은 decomposed/precomposed rendering의 CSSOM 폭을 구분하지 않는다. |
| `unicode-hyphen-boundary-max` | `well-known` | UAX #29 경계에 따라 `Well-Known`; Chrome candidate 56.171875px. |
| `unicode-apostrophe-word-max` | `o'neill` | 내부 apostrophe는 word를 나누지 않아 `O'neill`; Chrome candidate 30.28125px. |
| `unicode-opening-punctuation-max` | `«élan»` | punctuation 다음 word가 `Élan`; Chrome candidate 30.046875px. |
| `unicode-number-first-none` | `49ers`, `capitalize` | 첫 typographic letter unit은 Number이므로 변환하지 않고 `49ers` 유지; 같은 source의 `text-transform:none` control과 폭을 exact 비교한다. |
| `unicode-number-first-control` | `49ers`, `none` | 위 scene의 no-transform differential control; Chrome 24.984375px. |
| `unicode-collapsed-space-max` | `ßeta   éclair` | Phase I 뒤 `Sseta Éclair`; Chrome의 변환 문자열 후보는 53.359375px, target은 49.15625px로 U+00DF 차이를 보인다. |

Text-layout unit tests는 runtime 폰트 matrix와 분리해 `ǆ`→`ǅ`, supplementary `𐐨`→`𐐀`, `ß`→`Ss`, 결합 mark, 숫자·대문자 첫 unit, UAX #29 apostrophe/hyphen과 split source를 확인한다. Noto Sans KR이 지원하지 않는 glyph는 앱 glyph paint나 simulator text raster 근거로 주장하지 않는다.

## 구현과 검증 순서

1. 이 계획의 규범 문장, UCD mapping, 17-scene count, Chrome/standard 차이를 서로 겹치지 않는 20 실패 관점으로 검토한다. 계획 PR diff는 별도 30 관점으로 검토한다.
2. 계획 PR은 `Rebase and merge`하고 동일 checkout을 최신 `main`으로 fast-forward한 뒤 implementation branch를 순차로 만든다. 별도 worktree나 병렬 작업은 사용하지 않는다.
3. `spinon-text-layout`에서 `WhiteSpaceMode::Normal` Phase I 결과 문자열과 source ranges를 받아 UAX #29 word segmentation 및 ICU full titlecase를 수행한다. transform을 Parley line breaking/intrinsic sizing 전에 적용하고 output UTF-8 ranges를 다시 매긴다. 각 output scalar에는 해당 원문 scalar의 UTF-16 range를 복사해 확장 mapping을 보존한다. 다른 text-transform mode와 feature-off ASCII 동작은 회귀하지 않는다.
4. `icu_casemap`, `icu_segmenter`, `icu_properties`, `icu_locale_core`는 lockfile에 고정된 2.3.0과 필요한 compiled data를 fixture-only optional feature로 연결한다. locale parser는 정확한 `en`만 허용한다. case map data, 새로운 scene ID 및 fixture export는 default/release archive에서 제외하고 feature-positive archive에서만 포함한다.
5. anonymous Grid runtime validator, FFI source generator, 17-scene test table과 simulator capture checker를 같은 source profile로 갱신한다. ID가 style value·lang·source를 대신하지 않으며 owner/revision mismatch와 inline source는 layout 전에 거부한다.
6. `spinon-text-layout` focused/full unit test, runtime/FFI feature test, fixture-module boundary, default/opt-in archive raw marker scan, scoped Clippy, `cargo fmt --all -- --check` 및 Android/iOS 앱 builds를 같은 commit에서 실행한다. 실패 cohort를 삭제하거나 성공 재실행으로 고쳐 쓰지 않는다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 12개와 신규 11개 scene을 각각 23/23 실행한다. source revision·설치 artifact hash를 고정하고 text output, source ranges, intrinsic contribution, line count, track/frame, draw/present를 확인한다. Chrome의 U+00DF 동작 차이는 제품 mismatch로 표기하고 CSS Text+Unicode expected와 분리한다. Screenshot은 layout 상태 증거이며 glyph pixel이 아니다.
8. 구현과 runtime 결과를 계획 검토와 별개의 신규 20 실패 관점으로 검토하고 구현 PR diff를 별도 30 관점으로 검토한다. 수정한 경계는 다시 검토한다. PR 본문에는 동작 화면을 첨부하고, PR 번호/merge SHA/check rollup을 확인한 뒤 `Rebase and merge`한다.
9. 병합 뒤 내부 계약 0059, C11.4 계획, `spec/STATUS.md`, 공개 roadmap을 동일한 제한 범위·Chrome 차이·미검증 범위로 갱신한다. C11.4/C11 checkbox 또는 공식 완료 `72/213`을 바꾸지 않는다.

## 범위 밖과 완료 판정

정확한 direct `lang="en"` fixture scene 외 일반 author CSS admission, locale 상속, Turkish/Dutch 등 locale-specific casing, 모든 Unicode grapheme/shaping 조합, inline formatting, arbitrary fonts, GPU glyph paint, physical-device/hardware GPU, WPT 전체 및 C11.4 전체 completion은 이 slice가 아니다. Unit test의 `ǆ`·Deseret 변환은 normalization/source mapping만 검증하며 font rendering을 증명하지 않는다. 제품 support와 공개 API는 선언하지 않는다. 내부 계약 숫자 `0.1.0`과 공식 완료 `72/213`을 유지한다.
