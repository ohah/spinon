# C11.4 익명 Grid 텍스트 `hyphens:auto` 제한 profile 계획

**상태:** 구현 및 양 플랫폼 simulator 검증 후보. 기존 [`hyphens:none|manual` 제한 profile](./c11-4-anonymous-grid-text-hyphens.md)은 PR #977로 병합됐다. 이 계획의 제한 `auto` 구현은 현재 작업 브랜치에서 30-scene Android/iOS Simulator 검증을 통과했으며 아직 PR로 병합되지 않았다. C11.4/C11과 공식 `72/213`은 미완료다.

## 목표와 규범 범위

[CSS Text Module Level 3 §5.3](https://www.w3.org/TR/css-text-3/#hyphenation)은 `hyphens:auto`의 break opportunity를 언어에 맞는 resource에서 정하고, 그 기회를 min-content intrinsic size에 포함한다. CSS Text는 정확한 breakpoint 알고리즘을 정하지 않는다. User agent 간 dictionary 또는 line-break 위치의 일치를 완료 조건으로 둘 수 없다.

추가할 범위는 C11.4 `RuntimeBlockFlexGridTrackSizingV1` fixture-test 경로에서 다음 조건을 모두 만족하는 익명 Grid direct-text item이다.

- 콘텐츠 언어가 정확히 `en`으로 확인되고 computed `hyphens`가 `auto`다. 상위 language를 확정할 수 없거나 `en-US`, `fr` 등 다른 tag인 경우 이번 고정 profile은 layout 전에 거부한다.
- Ahem 10px/10px, `horizontal-tb`, LTR, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`, 기본 letter/word spacing과 text-indent만 허용한다.
- 자동 사전 분절 대상은 `hyphenation`, `internationalization`, `representation` 세 소문자 ASCII 단어다. 다른 단어, 추가 문장부호·공백·inline run, U+00AD를 포함한 source text는 거부한다. `hyphens:none|manual`의 기존 profile 동작은 유지한다.
- 자동 분절은 source/document 문자열이나 DOM range에 삽입하지 않는다. normalized layout text에만 U+00AD 기회를 추가하며, line이 그 synthetic marker에서 끝날 때 Ahem U+2010 advance 10px를 포함한다.
- min-content pass에서는 자동 기회를 visible U+2010 advance로 측정한다. 기대 min-content inline size는 `hyphenation` 50px, `internationalization` 50px, `representation` 40px다. max-content에는 dictionary marker의 추가 advance를 반영하지 않으며 각각 원문폭 110px, 200px, 140px다.
- 50px definite track의 Hypher 기반 예상 줄 수는 각각 3, 7, 5다. per-line advances는 독립 Ahem 산식과 고정 pattern breakpoints로 확인한다. Chrome 156의 줄 수·breakpoints는 관측표로 기록하되 정확한 breakpoint equality는 요구하지 않는다.

## 구현 설계와 의존성 경계

- [`hypher`](https://docs.rs/hypher/0.1.8/hypher/) `=0.1.8`을 exact pin으로 추가하고 `default-features = false`, `features = ["english"]`만 사용한다. Crate manifest는 `MIT OR Apache-2.0`이며 기본 `full` feature를 사용하지 않아 다른 언어의 pattern set을 끌어오지 않는다.
- `hypher`는 optional dependency가 되어야 한다. `spinon-text-layout`의 전용 `c11-4-hyphens-auto-fixture-test` feature를 통해 기존 opt-in C11.4 runtime fixture feature에서만 켜고, default/product build에서는 dictionary code/data가 빠지는지 archive positive/negative controls로 확인한다.
- 허용된 단어는 ASCII 45바이트 이하이다. Hypher no-alloc API 호출 전에 byte length와 허용 단어를 검사해 그보다 긴 word가 crate 내부 panic 경계에 도달하지 않게 한다. 사전이 빈 분절 또는 원문 전체를 반환하는 등 예상하지 않은 결과도 layout 성공으로 낮추지 않고 fail closed한다.
- pattern breakpoint는 UTF-8/UTF-16에서 동일한 ASCII boundary에만 추가한다. 각 Hypher segment는 비어 있지 않아야 하고 순서대로 이어 붙인 byte string이 입력 단어와 같아야 한다. synthetic U+00AD는 source UTF-16 range가 0인 mapping 또는 기존 source range 사이의 명시적 mapping gap으로 표시한다. 뒤쪽 `TextSourceRange`, line range, owner NodeId, document/style revision이 밀리거나 원문을 소비하지 않는지 검증한다.
- max-content pass는 원래 paragraph와 동일한 원문 폭을 유지한다. min-content pass와 definite-width line layout은 같은 normalized marker positions를 사용하고, 끊긴 line에만 U+2010 advance를 적용한다. line이 우연히 다른 source 경계에서 끝났을 때 hyphen을 추가하지 않는 회귀를 포함한다.
- Stylo의 fixture 전용 `hyphens` alias cascade에서 computed `auto`를 읽는다. 선언 원문 추측, DOM scene ID에 따른 값 대입, 알 수 없는 language의 English fallback은 금지한다.

## 비교 근거와 고정 행렬

[Chrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-precomparison-2026-11-01/README.md)는 Ahem/Agent Browser 환경과 UA 관측을 기록한다. Chrome은 `hyphenation`에서 min-content 50px 및 50px 폭 3줄을, `internationalization`에서 min-content 50px 및 7줄을 관측했다. Hypher breakpoints는 Chrome과 다르므로 고정 구현 oracle은 versioned dictionary plus Ahem width calculation이다. `extraordinary`는 Chrome min-content 110px 대 Hypher predicted 50px의 차이를 보여 첫 구현 matrix 밖에 둔다.

구현 matrix에는 다음을 고정한다.

1. 세 단어별 `hyphens:none|manual|auto` × min-content, max-content, definite 50px track. 기존 word fixtures는 기존 선언/cascade adapter와 연결하되 `auto`만 세 word allowlist에 허용한다.
2. `auto`의 en language owner/cascade positive control과 language 없음, `fr`, 허용되지 않은 단어, U+00AD 포함 source의 expected reject.
3. 한 단어가 한 줄에 있을 때 synthetic hyphen이 추가되지 않고 source/document UTF-16은 원래 길이를 보존하는 max-content 대조군.
4. generated break마다 line-ending marker, discretionary U+2010 advance, line count, min/max contribution, Grid track, output revision을 검사한다. 세 단어의 `none/manual` 회귀도 현재 기대값과 일치해야 한다.

## 검증 및 완료 관문

- 구현 전: 위 독립 Ahem 산식과 Chrome 관측, fixture HTML/source/font hash 및 기준 checkout을 사전 비교 evidence에 고정한다.
- Rust: focused text-layout, style cascade, runtime fixture, FFI matcher 및 관련 regression suites; `cargo fmt --all -- --check`; feature-scoped Clippy를 실행한다. 전체 suite 실패는 변경 전 기준과 분리한다.
- 앱: Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 pinned source·app hash로 세 word × 세 mode 행렬과 expected rejects를 실행한다. geometry·track·trace·draw/present 결과를 확인한다. AVD software renderer와 iOS compositor 제출 전 표본은 해당 범위 그대로 적는다.
- artifact: 기본/product `spinon-ffi` release static archive에는 Hypher English pattern marker, auto-only fixture symbols와 dictionary bytes가 없어야 한다. opt-in fixture-test archive에는 각각 positive control이 있어야 한다. Archive 검사는 APK/IPA 전체 fixture exclusion을 대신하지 않는다.
- 구현/runtime 후 새 실패 관점 20개와 실제 최종 PR diff 별도 실패 관점 30개를 기록한다. 결함을 고치면 영향 관점을 다시 확인한다.
- 일반 CSS author admission, 임의 English text, 다른 언어, U+00AD와 자동 break 혼합 fallback, spelling-changing hyphenation, `hyphenate-limit-*`, general WPT, glyph painting, hardware GPU/실기기는 이 slice의 증거가 아니다. 공식 `72/213` 및 C11.4/C11 체크를 올리지 않는다.

## 2026-11-01 구현 후보 실행

고정 matrix 30개(기존 `none|manual|initial` 회귀, Auto 9개, expected reject 2개)가 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 30/30 통과했다. Simulator trace에서 computed mode, document revision, min/max intrinsic, line advance, track/frame, 원문 source 및 각 synthetic marker의 0-length UTF-16 mapping을 검사했다. 설치 APK·iOS executable hash가 build hash와 같다. [실행·artifact 경계와 남은 범위](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-runtime-2026-11-01/README.md) · [archive feature boundary](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-runtime-2026-11-01/artifacts/README.md).

Text-layout 96, Style cascade/CSS adapter 7, Runtime 394 passed/3 ignored, FFI 188 passed/2 ignored, fixture module-boundary 32, fixture-enabled FFI Clippy 및 `cargo fmt --all -- --check`가 통과했다. 초기 capture에서 두 harness expected-value 결함을 찾아 수정하고 전체 양 플랫폼 행렬을 다시 실행했다. 새 실패 관점의 [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-runtime-2026-11-01/implementation-runtime-review-20.md)와 실제 최종 diff의 [PR 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-auto-runtime-2026-11-01/pr-diff-review-30.md)는 완료했다. PR 생성·GitHub check 확인·병합이 남아 있다. 이 후보 자체는 공개 CSS 지원이나 전체 C11.4 완료를 선언하지 않는다.
