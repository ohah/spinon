# C11.4 anonymous Grid text computed `text-transform: full-size-kana`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Text Level 3 CRD 2026-08-14 §2.1 및 Appendix G](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#small-kana)

**상태:** 계획 후보. `text-transform: full-size-kana`는 CSS Text Level 3의 Candidate Recommendation Draft에 정의되어 있으나 at-risk로 표시된다. 이 계획은 해당 draft의 고정 매핑 58개를 opt-in C11.4 anonymous direct-text Grid fixture 경로에서 구현하고, 공개 CSS 지원이나 안정된 상호운용 계약으로 승격하지 않는다.

## 기준과 관찰

CSS Text §2.1은 `full-size-kana`가 모든 small Kana를 동등한 full-size Kana로 바꿔 표시한다고 정의한다. Appendix G는 Unicode 15.0 기준 58개 매핑을 열거한다. 변환은 underlying document text를 바꾸지 않고, white-space Phase I 뒤·Phase II 및 line breaking 전의 text transformation 순서에 놓인다. [매핑 원본 및 추출 결과](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-size-kana-precomparison-2026-11-03/README.md) · [고정 매핑표](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-size-kana-precomparison-2026-11-03/mapping-table.json)

Agent Browser의 HeadlessChrome 156은 `CSS.supports('text-transform', 'full-size-kana') == false`를 반환하고 지정값을 computed `none`으로 처리했다. 따라서 이 브라우저는 변환 의미의 positive oracle이 아니다. 사전 비교 페이지는 small-kana 원문, full-size literal control, 미지원 선언 결과를 나란히 기록하며, Noto Sans KR 10px 후보의 폭은 글꼴/측정 환경을 확인하는 관찰값으로만 사용한다. [Chrome 관찰 JSON과 화면](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-size-kana-precomparison-2026-11-03/)

## 제한 profile

- opt-in `c11-4-text-transform-unicode-fixture-test`와 `c11-4-grid-track-sizing-fixture-test` 경로에만 연결한다. 일반 CSS admission, 기본 runtime 동작, 새 FFI/API는 추가하지 않는다.
- 정확한 computed `text-transform: full-size-kana`, `lang=ja`, horizontal-tb/LTR, 기존 anonymous direct-text Grid 구조와 같은 revision의 단일 paragraph input을 요구한다. 중첩·inline style/run·불일치 owner·stale snapshot은 거부한다.
- Appendix G의 58개 코드 포인트만 표에 따라 치환한다. 표에 없는 문자는 그대로 둔다. Unicode 정규화, 호환 분해, 문장/언어 추정, authored CSS 문자열 기반 추론은 하지 않는다.
- 변환은 기존 white-space normalization 이후 intrinsic shaping·line breaking 전에 수행한다. HostDocument 원문과 UTF-16 source range는 유지한다. supplementary small kana에서 source와 output UTF-16 길이가 다르므로 range mapping을 별도 검사한다.
- `full-size-kana`와 `full-width`, case transform 등 복합 값을 조합하는 경로는 이 계획에 포함하지 않는다. CSS computed value가 계획의 단일 값과 다르면 성공으로 축약하지 않는다.

## 고정 입력과 판정

사전 비교 fixture는 [고정 HTML](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform-full-size-kana.html)이다. Browser가 직접 비교할 수 없는 변환 정답은 CSS Text Appendix G의 각 코드 포인트 쌍으로 독립 고정한다. 공통 hiragana·katakana·halfwidth literal은 Noto Sans KR 10px에서 폭 차이가 없거나 반올림 범위 내였다. 이 수치는 변환 성공 판정이나 Chrome 호환성 기준이 아니다.

| 입력 그룹 | 원문 | Appendix G 기대 변환 |
| --- | --- | --- |
| Hiragana | `ぁぃぅぇぉっゃゅょゎ` | `あいうえおつやゆよわ` |
| Katakana | `ァィゥェォッャュョヮ` | `アイウエオツヤユヨワ` |
| Halfwidth | `ｧｨｩｪｫｯｬｭｮ` | `ｱｲｳｴｵﾂﾔﾕﾖ` |
| Supplementary kana | `𛄲𛅐𛅑𛅒𛅕𛅤𛅥𛅦𛅧` | `こゐゑをコヰヱヲン` |
| Pass-through | `あカンABC 12` | `あカンABC 12` |

## 구현·검증 순서

1. Appendix G 58행을 코드/단위 회귀에 고정하고 중복 source, 잘못된 destination, 누락 행을 검증한다. CSS mapping JSON의 Unicode code point와 Rust 변환표를 직접 대조한다.
2. `TextTransformMode`와 computed-style fixture adapter에 `FullSizeKana` 경로를 추가한다. 기본 feature에서는 기존 fail-closed 동작을 유지한다.
3. 모든 mapping과 비매핑 문자 identity를 검사한다. 특히 supplementary source의 UTF-8 경계, 2-unit source에서 1-unit destination으로 변환할 때 source range가 원문을 가리키는지 확인한다.
4. direct-text runtime matrix에 표의 네 입력 그룹, `none` control 및 malformed owner/language/style/revision negative controls를 연결한다. intrinsic columns/rows, item frames, source node/ranges, typed computed value를 비교한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source와 fixture hash의 전체 matrix를 확인한다. 화면은 layout smoke이며 glyph raster나 compositor 이후 pixel equality 증거가 아니다.
6. 기본/product artifact에서 fixture 전용 경로가 제외되고 opt-in fixture artifact에서 포함되는 경계를 확인한다. focused Rust/runtime/FFI 검증, scoped Clippy, `cargo fmt --all -- --check`, Android/iOS Simulator 빌드 결과를 실행한 범위대로 기록한다.
7. 구현/runtime 실패 관점 20개와 별도의 PR diff 적대 검토를 진행한다. C11.4/C11 체크박스와 공식 완료 `72/213`은 이 제한 slice만으로 변경하지 않는다.

## 제외 범위

일반 CSS author admission, arbitrary document/runtime input의 전체 text-transform 지원, 다른 locale/profile, inline formatting, combined transform values, glyph paint 정확도, full WPT conformance, 실기기·hardware GPU, iOS compositor 이후 pixel equality 및 C11.4/C11 완료 판정은 포함하지 않는다. CSS Text CRD의 at-risk 분류가 바뀌면 범위를 다시 판단한다.
