# C11.4 anonymous Grid text computed `text-transform: full-width`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 새 제한 fixture 계획. `full-width`는 CSS Text Level 3 CRD 2026-08-14에 정의되어 있지만 at-risk로 표시된다. Chrome 156 Agent Browser는 이 값을 거부하고 computed `none`을 반환한다. 구현은 CSS Text와 Unicode mapping을 기준으로 하는 opt-in fixture 후보이며 Chrome의 text-transform 지원을 주장하지 않는다. [사전 비교와 관측 한계](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/README.md) · [계획 실패 관점 20개](./c11-4-anonymous-grid-text-transform-full-width-plan-review-20.md) · [계획 PR diff 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/plan-pr-diff-review-30.md)

## 목표와 범위

기존 anonymous direct-text Grid의 computed `text-transform` 검증 경로에 `full-width`의 고정 입력 subset을 추가한다. Chrome이 해당 값을 지원하지 않으므로 independent normative expectation은 CSS Text Level 3 §2.1.1과 UAX #11의 `<wide>` decomposition으로 결정하고, Chrome 156의 literal candidate measurement는 고정 font advance와 min-content wrapping만 관찰한다. 이 둘을 하나의 browser oracle로 합치지 않는다. [CSS Text CRD §2.1.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-transform-mapping) · [UAX #11](https://www.unicode.org/reports/tr11/)

정확히 다음 source만 허용한다.

- Grid style owner의 computed `text-transform`은 scene에 지정한 `none` 또는 `full-width`와 일치한다.
- 직접 `lang="ja"`, fixed Noto Sans KR `wght=400`, 10 CSS px, LTR, horizontal writing mode, `white-space:normal`을 사용한다.
- Text는 `AB 12` 또는 `AB   12`뿐이다. 대문자 ASCII `A`, `B`, ASCII digits `1`, `2`, U+0020만 변환 후보로 받는다. 다른 punctuation, lowercase, non-ASCII source, inline run, ancestor-owned text, language inheritance는 fail closed한다.
- `AB 12`는 `ＡＢ　１２`로 변환한다. `AB   12`는 Phase I U+0020 collapse 뒤 `ＡＢ　１２`로 변환한다. full-width 문자 수와 UTF-16 source range는 원본 문자 단위와 대응한다.
- Transform은 normalized Phase I text 뒤, line breaking과 intrinsic sizing 전에 적용한다. HostDocument의 원문과 UTF-16 범위는 수정하지 않는다.

## 고정 사전 비교

HTML fixture, Chrome observation, standard-based expected strings, screenshot과 입력 hash는 [precomparison evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/README.md)에 둔다.

| Scene | Expected normalized text | Independent target | Chrome 156 evidence |
|---|---|---|---|
| `none-max` | `AB 12` | max-content `25.984375px`, one line | target와 literal text candidate가 일치 |
| `full-width-max` | `ＡＢ　１２` | max-content `50px`, one line | target computed `none`; literal candidate는 `50px` |
| `full-width-min` | `ＡＢ　１２` | min-content `20px`, two lines | target computed `none`; literal candidate min track은 `20px`/2 lines |
| `full-width-collapsed-spaces` | `ＡＢ　１２` | max-content `50px`, one line | target computed `none`; one ideographic-space candidate `50px`, three-space candidate `70px` |

허용 폭 오차는 `0.03125 CSS px`이다. Chrome target의 computed `none`은 unsupported-value negative control이다. Chrome screenshot과 CSSOM은 full-width 변환 성공 증거가 아니며 제품과 Chrome 간 호환성 주장을 하지 않는다. Unicode expected는 `UnicodeData.txt`의 `<wide>` compatibility decomposition 및 CSS Text mapping 규칙을 입력의 고정 문자에 적용해 만든다. CSS Text CRD의 at-risk 표시는 명세가 CR 기간에 이 기능을 제거할 수 있음을 의미하므로 남은 범위에 유지한다.

## 구현·검증 순서

1. 이 계획과 precomparison을 20개의 서로 다른 실패 관점으로 검토하고 계획 PR 변경을 별도 30 관점으로 확인한다.
2. 계획 PR을 `Rebase and merge`한 후 현재 checkout에서 최신 `main`으로 진행해 순차 구현 branch를 만든다. 별도 worktree와 동시 구현은 사용하지 않는다.
3. `TextTransformMode::FullWidth`의 layout-buffer 변환을 추가한다. 원본 허용 집합과 phase order를 layout 전에 검증하고, 각 output scalar의 paragraph byte range를 대응하는 원본 UTF-16 range와 연결한다. 변환 후 Noto shaping 결과로 intrinsic min/max contribution과 line break를 계산한다.
4. 기존 `c11-4-text-transform-unicode-fixture-test` opt-in의 anonymous Grid runtime allowlist에 `lang=ja` 고정 scene을 추가한다. Scene ID는 computed value, exact Grid owner, source text, language, document/style revision을 대신하지 않는다. Unsupported style 값, wrong/missing/inherited `lang`, punctuation, 다른 source, inline run과 stale snapshot은 layout publication 전에 거부한다.
5. 동일한 4 scene을 text-layout, runtime adapter, FFI scene source 및 fixture dispatcher에 연결한다. fixture-only scene IDs와 source strings의 기본 archive 부재 및 opt-in positive control을 검사한다. 신규 scene에 대한 default CSS author admission이나 공개 API는 추가하지 않는다.
6. focused/full relevant Rust suites, fixture-boundary checks, feature-scoped Clippy, `cargo fmt --all -- --check`, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator를 같은 source revision에서 검증한다. Chrome unsupported-value 결과는 제품 mismatch 수정으로 다루지 않는다.
7. 구현/runtime 동작을 새 20 관점으로, 구현 PR diff를 별도 30 관점으로 검토한다. PR에는 Chrome unsupported-value 화면과 CSS Text/Unicode 판정 기준을 구분해 적는다. Agent Browser 사용 후 browser/session process가 없는지 확인한다.
8. 구현 병합 뒤 내부 계약 0059, 상위 계획, `spec/STATUS.md`와 공개 Tailnet roadmap에 제한 범위와 at-risk/Chrome 한계를 동기화한다. C11.4/C11 checkboxes와 공식 `72/213`은 전체 항목 기준이 충족될 때까지 유지한다.

## 제외 범위

일반 CSS author admission, 임의 CSS Text `full-width` 문자 집합, punctuation·lowercase·non-ASCII mapping, 모든 Unicode wide/narrow pair, 다른 locale/source owner/inline formatting, `full-size-kana`, Chrome/WPT interoperability, glyph pixels, 실기기/hardware GPU, 전체 C11.4 또는 C11 완료는 이 제한 slice가 아니다. CSS Text CRD의 at-risk 상태가 바뀌면 계획·구현 범위를 다시 판정한다.
