# C11.4 anonymous Grid text computed `text-transform` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Text Level 3 §2.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-transform-property)

**상태:** 제한 fixture 구현 후보. ASCII layout-only mapping, source preservation 및 style snapshot revision guard가 연결됐다. `spinon-text-layout` 86/86, feature-enabled `spinon-runtime` 379 passed·3 ignored와 19 integration passes, feature-enabled FFI 182 passed·2 ignored, fixture boundary 30/30, targeted Clippy와 Android API 37 ARM64 AVD/iPhone 17 Pro iOS 26.2 Simulator 각 6/6이 통과했다. 기본/opt-in FFI archive marker 경계를 대조했고 구현/runtime 실패 관점 검토 20개를 마쳤다. 실제 PR diff 실패 관점 검토 30개를 마쳤다([검토 기록](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/pr-diff-review-30.md)). [PR #952](https://github.com/ohah/spinon/pull/952)에는 보고된 check이 없어 CI 성공으로 기록하지 않는다. 병합 검토가 남아 있으며 이 slice와 공식 `72/213`은 미완료 상태다. [Chrome oracle와 독립 Noto Sans KR advance](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.md) · [현재 구현·시뮬레이터 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/README.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/implementation-runtime-review-20.md) · [계획 실패 관점 검토 20개](./c11-4-anonymous-grid-text-computed-text-transform-plan-review-20.md)

## 목표와 범위

기존 opt-in C11.4 anonymous direct-text Grid adapter가 정확한 Grid style owner에서 읽은 computed `text-transform`을 `ParagraphStyle`과 intrinsic text layout에 전달한다. `none`, `uppercase`, `lowercase`만 허용하고 source string `IIII ii`의 ASCII 문자만 layout용으로 변환한다. Noto Sans KR 10px, weight 400, direct `lang="en"`을 사용해 대소문자 변환에 따른 min-/max-content 크기 차이를 관찰한다.

CSS Text Level 3 §2.1에서 `text-transform`은 상속되며 underlying content를 바꾸지 않는 presentation 속성이다. 따라서 HostDocument 원문과 text source node/UTF-16 range는 보존하고 Parley가 shaping할 normalized text만 변환한다. non-ASCII, locale-sensitive casing, Unicode 확장·다중 code point casing, `capitalize`, `full-width`, `full-size-kana`, CSS-wide value와 inline run은 이번 fixed fixture의 입력이 아니다. 정확한 root `lang="en"`을 요구해 상속/언어별 casing 추론을 막는다.

이 변경은 기존 `c11-4-grid-track-sizing-fixture-test` gate에서만 여섯 개 fixed scene을 등록한다. 일반 CSS author admission, document-wide text rendering, arbitrary family/weight, Unicode text transformation, text selection/copy behavior, WPT, glyph painting 또는 전체 C11.4/C11 지원을 선언하지 않는다. 내부 계약 `0.1.0`과 공식 완료 `72/213`은 유지한다.

## 고정 oracle와 기대값

[Agent Browser Chrome 156 oracle](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.md)는 동일한 원문 `IIII ii`를 `none|uppercase|lowercase`와 `min-content|max-content` column으로 조합한 여섯 scene을 고정한다. Noto Sans KR `wght=400`, `lang=en`의 independent shaped glyph advances로부터 예상한 inline contributions는 다음과 같다. CSSOM expected 값, Chrome 1/64px quantization 및 0.03125px tolerance는 JSON으로 보존한다.

| computed value | shaped layout text | min-content column / auto row | max-content column / auto row |
| --- | --- | --- | --- |
| `none` | `IIII ii` | `11.734375px / 20px` | `19.46875px / 10px` |
| `uppercase` | `IIII II` | `11.734375px / 20px` | `19.84375px / 10px` |
| `lowercase` | `iiii ii` | `11px / 20px` | `18.71875px / 10px` |

## 구현과 실패 경계

1. `spinon-text-layout`에 `TextTransformMode::{None, Uppercase, Lowercase}`를 추가하고 `ParagraphStyle`에 저장한다. 현재 text fixture 기본값은 `None`을 유지한다.
2. anonymous-text adapter는 exact Grid root의 same-revision computed `text-transform`을 읽는다. scene ID만으로 CSS value를 정하지 않으며 `none|uppercase|lowercase` 및 direct `lang=en`의 일치를 확인한다.
3. normalization 단계에서 ASCII letters만 case-map한다. 변환은 one-to-one ASCII라 normalized UTF-8/UTF-16 source positions와 text-node provenance가 보존되어야 한다. `HostDocumentSnapshot`, 원본 Text node 값, source ranges는 수정하지 않는다. non-ASCII source, unknown scene/value, missing/wrong language, stale revision, wrong style owner는 layout publication 전 거부한다.
4. text-layout tests는 source text 유지, 변환된 layout text, source-to-layout range, whitespace collapse 뒤 결과, min/max intrinsic contribution과 line advance를 검사한다. runtime scene validators는 computed serialization·typed style·owner·revision·source text·transformed text·intrinsic input/size·line metrics·frame·Columns/Rows trace를 함께 비교한다.
5. fixed six-scene oracle과 기존 anonymous Grid text matrix를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. Screenshots, installed artifact hashes, layout trace를 연결한다. renderer text area가 glyph를 그리지 않는 경우 screenshot은 glyph-pixel evidence로 해석하지 않는다.
6. 신규 fixture marker와 scene payload가 default/product artifact에서 빠지고 opt-in fixture-test positive control에만 포함되는 경계를 검사한다. Static archive evidence를 APK/IPA 전체 bundle exclusion으로 확대하지 않는다.
7. 계획 검토와 별도로 구현/runtime 실패 관점 20개, 구현 PR diff 실패 관점 30개를 검토한다. PR 본문에 가능한 Android/iOS 화면을 immutable commit image link로 넣고 Agent Browser에서 표시를 확인한다. 현재 required check과 check rollup을 다시 조회한 뒤 `Rebase and merge`한다.
8. 병합 후 상태 대장, 내부 계약 0059, 구현 계획 및 public roadmap에 실제 통과 scene과 제외 범위를 동기화한다. C11.4/C11 checkbox와 `72/213`은 그대로 둔다.

## 완료 판정

여섯 fixed scene의 computed transform, layout-only case mapping, source text/provenance, intrinsic min/max column, auto row, line advance, frame과 track trace가 Chrome/independent values와 허용 오차 안에서 일치해야 한다. Unsupported Unicode/value/owner/revision controls는 부분 결과를 남기지 않고 거부되어야 한다. Android/iOS Simulator, focused Rust/runtime/FFI checks, formatter/Clippy, feature boundary, independent 20-point implementation review와 별도 30-point PR diff review가 모두 통과해야 제한 fixture slice가 완료된다. 이는 일반 CSS text-transform, WPT, glyph painting, hardware device, iOS post-compositor pixels 또는 전체 C11.4 completion이 아니다.

## 근거

- [사전 비교·Chrome screenshot·font/hash](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.md)
- [선택 문서 build와 roadmap viewport 확인](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/selected-docs-build-plan-pr951.json)
- [계획 실패 관점 검토 20개](./c11-4-anonymous-grid-text-computed-text-transform-plan-review-20.md)
