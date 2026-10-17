# 0076 · C19.3 hit-test·WCAG 대비 계산 기반

- **계약 버전:** `0.1.0` · 출시 전 고정
- **상태:** C19.3의 첫 제한 조각 구현 중 · C19.3/C19 전체는 미완료
- **상위 계획:** [C19 기본 페인트 속성](../../plan/c19-basic-paint-attributes.md)
- **사전 비교:** [hit-test·contrast 사전 비교](./evidence/c19-3-hit-test-contrast-precomparison-2026-10-17.md)

## 범위

이 조각은 renderer hit-test가 source color alpha 및 opacity group과 독립적이라는 불변 조건, 그리고 이미 합성된 sRGB 쌍의 WCAG 2.2 contrast ratio 계산 함수만 추가한다. 공개 DOM/CSS API나 제품 접근성 기능을 추가하지 않는다. text glyph의 computed `color` 전달과 C15/C16 text painter 연결은 후속 작업이다.

## Hit-test 계약

- Hit target은 viewport 안의 box geometry, hit-test clip, hit-test order로 결정한다. `RuntimePaint::Color`의 alpha와 `RuntimePaintCommand::BeginGroup` opacity는 target 선정에 영향을 주지 않는다.
- 같은 box·clip·order 입력은 중첩 opacity group 각각의 opacity `{0, 0.5, 1}` 및 source alpha `{0, 0.4, 1}` 조합 모두 같은 target을 반환한다. 시작 경계 포함, 오른쪽·아래쪽 끝 경계 제외도 각 조합에서 확인한다.
- 기존 좌표 기준을 유지한다: frame과 clip은 시작 경계를 포함하고 끝 경계를 제외한다. paint clip과 hit-test clip은 별도로 유지한다.
- 비교 기준은 고정 Chrome 155 `document.elementFromPoint()`의 3×3 alpha/opacity matrix다. 좌표, 아래쪽 target, 오른쪽 반열린 경계 및 browser metadata는 사전 비교 파일에 고정했다.

## WCAG 계산 계약

```rust,ignore
pub struct FinalOpaqueSrgb { /* alpha 합성 뒤 encoded sRGB 채널 3개 */ }
pub fn wcag_22_contrast_ratio(
    foreground: FinalOpaqueSrgb,
    background: FinalOpaqueSrgb,
) -> f64;
```

- `FinalOpaqueSrgb::new` 입력은 유한한 encoded sRGB component 3개이며 각 값은 `[0, 1]`이다. `FinalOpaqueSrgbError`는 비유한 값과 범위 밖 채널을 거부한다.
- WCAG 2.2 상대 휘도 식을 사용한다. 각 채널은 `0.04045` 이하에서 `c / 12.92`, 그 외에는 `((c + 0.055) / 1.055)^2.4`로 linearize한다. 상대 휘도는 `0.2126R + 0.7152G + 0.0722B`, 비율은 `(lighter + 0.05) / (darker + 0.05)`다.
- 입력 타입은 alpha와 premultiplied color를 갖지 않는다. 호출자는 foreground와 background 각각의 실제 backdrop 합성을 먼저 수행하고 최종 불투명 encoded sRGB component를 전달해야 한다. 이 함수는 CSS alpha, opacity group 또는 blend/composite 절차를 구현하지 않는다.
- 반환값은 비율일 뿐이다. `4.5:1` threshold 판정은 호출자 책임이며, 이 API·fixture는 WCAG 적합성, 실제 anti-aliased glyph 대비, 접근성 트리나 사용자 경험을 보증하지 않는다.

## 검증 기준

- Chrome 155 독립 JavaScript oracle의 `#000/#fff = 21`, `#767676/#fff = 4.542224959605253`, `#777/#fff = 4.478089453577214`와 정확 경계 및 양쪽 소수 입력은 [사전 비교 파일](./evidence/c19-3-hit-test-contrast-precomparison-2026-10-17.md)에 고정한다.
- alpha/opacity matrix는 중첩 두 group의 27개 opacity/source-alpha 조합에서 overlap target, under-only target, 양쪽 반열린 끝 경계를 각각 확인한다. opacity 0을 miss로 바꾸는 부정 대조는 실패해야 한다.
- WCAG fixture는 black/white 극값, 세 primary 색별 휘도 가중치, sRGB piecewise 선형화 경계 양쪽, `4.5:1` 상하의 8-bit 회색, 소수 채널의 정확 경계, 채널 유효성, 대칭성과 동일색 `1:1`을 확인한다. sRGB 선형화나 가중치를 바꾼 부정 대조는 실패해야 한다.
- C15/C16 text paint common-revision 통합, CSS alpha compositing, Android/iOS surface glyph output은 이 조각의 완료 기준이 아니다. C19.3 전체 또는 C19 parent 완료 상태로 승격하지 않는다.

## 근거

- 대비 수식과 4.5:1 정상 텍스트 기준: [W3C WCAG 2.2 이해 문서 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum)
- Chrome hit-test와 contrast 사전 관찰, 입력 환경 및 기대값: [사전 비교 기록](./evidence/c19-3-hit-test-contrast-precomparison-2026-10-17.md)
