# 0045 · C07.3 종횡비

**문서 ID:** `0045` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** [PR #104](https://github.com/ohah/spinon/pull/104) 구현·검증 완료, 검토 중 · **공개 API:** 아님

`0045`는 문서 ID다. 출시 전 내부 계약·Spinon crate 숫자 버전은 `0.1.0`으로 고정하며 구현이나 문서 추가로 올리지 않는다.

## 적용 범위

현재 제한 runtime layout profile의 block/flex item에 CSS `aspect-ratio`의 초기값·`auto`와 bare 양수 finite `<ratio>`를 적용한다. CSS 계산은 Stylo의 typed computed value를 사용하고 Taffy 0.14.0에 전달한다. 종횡비와 non-default `min-width`·`max-width`·`min-height`·`max-height`의 조합은 Chromium과 Taffy의 used-size 결과가 다르므로 현재 layout adapter가 해당 node와 property를 지목해 전체 계산을 거부한다. 이 내부 계약은 CSS 전체 지원이나 앱 작성자용 공개 API를 의미하지 않는다.

## 값 계약

| CSS computed 상태 | 내부 값 | 동작 |
| --- | --- | --- |
| 초기값 또는 `auto` | `None` | 선호 비율 없음 |
| bare positive finite `<ratio>` | `Some(width / height)` | 지원된 auto size에 사용. non-default min/max와 함께면 layout 오류 |
| 분자 또는 분모가 0인 degenerate ratio | `None` | CSS auto처럼 선호 비율 없음 |
| `auto <ratio>` | 오류 | 유효한 CSS이나 content-box 의미를 이 계약에서 표현하지 못함 |
| 음수·NaN·무한대·f32에서 0/무한대로 변환 | 오류 | 부분 geometry를 반환하지 않음 |

비율은 `box-sizing`에 맞춘 preferred box에 작용한다. definite width와 definite height가 모두 있으면 ratio가 그 두 지정 크기를 덮지 않는다. Taffy 0.14 leaf 알고리즘은 이 경우에도 비율로 높이를 다시 키우므로 adapter가 양 축이 definite인 leaf 계산에서 Taffy ratio를 끈다. non-default min/max constraint transfer는 구현 범위에서 제외하고 fail closed한다. aspect ratio는 상속되지 않는다.

## 계산 경계

- Stylo의 `GenericAspectRatio`에서 `auto`와 `PreferredRatio`를 직접 읽는다. computed CSS 문자열을 ratio 입력으로 재파싱하지 않는다.
- `var()`·지원되는 typed numeric expression은 Stylo가 계산한 typed ratio를 사용한다. invalid CSS 선언은 Stylo cascade의 이전 유효 winner를 보존한다.
- `LayoutStyle.aspect_ratio`에 있는 값은 finite positive `f32`여야 한다. adapter는 supported profile에서 이를 Taffy에 전달한다. 두 definite 크기를 가진 leaf는 Taffy의 ratio 후처리를 비활성화해 지정 크기를 보존한다.
- preferred ratio가 있고 네 min/max 중 하나가 initial `auto`/`none`이 아닌 node는 Taffy layout 전에 `UnsupportedAspectRatioConstraint { node, property }`로 거부한다. 이 경계는 Block·Flex 여부에 관계없이 적용한다.
- 지원 대상은 제한 runtime Block/Flex profile뿐이다. Grid·table·inline·replaced element·intrinsic content sizing은 이 계약 밖이다.
- 오류는 node와 property를 포함하며 layout 전체 계산을 실패시켜 오래된/부분 frame을 새 결과로 반환하지 않는다.

## 예제와 경계

```css
.video-frame { width: 320px; height: auto; aspect-ratio: 16 / 9; }
```

width가 320 CSS px로 확정되고 height가 auto인 경우, 빈 content box라면 preferred height는 180 CSS px다. padding·border·box-sizing·min/max가 있으면 CSS sizing 규칙에 따라 최종 결과가 달라지므로 단순 나눗셈을 일반 결과로 보장하지 않는다.

```css
.media { aspect-ratio: auto 16 / 9; }
```

이 값은 유효 CSS지만 bare ratio와 다르게 content box 기준을 요구한다. 내부 Taffy 연결에서 의미를 보존하지 못하면 오류로 거부한다. `16 / 9`로 묵시 변환하지 않는다.

## 검증과 상태

계획·Chromium 기준·구현·계획 재검토·구현 실패 경로 검토 및 Android/iOS Simulator 결과는 [C07.3 계획](../../plan/c07-3-aspect-ratio.md), [Chrome 사전 기준](evidence/c07-3-aspect-ratio-precomparison-2026-10-10.md), [Taffy 차이·시뮬레이터 결과](evidence/c07-3-taffy-differential-2026-10-10.md), [수정 계획 재검토](evidence/c07-3-aspect-ratio-plan-review-scope-followup-2026-10-10.md), [구현 검토](evidence/c07-3-aspect-ratio-implementation-review-2026-10-10.md)에서 추적한다. Chrome의 resolved `getComputedStyle()` 문자열은 Stylo computed 문자열과 직접 비교하지 않고, `computedStyleMap()`의 `aspect-ratio`·`box-sizing`와 used geometry를 구분한다. 병합 전까지 공식 완료 체크와 공개 API 목록은 변경하지 않는다.
