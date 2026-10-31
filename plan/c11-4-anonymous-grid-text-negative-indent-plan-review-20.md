# C11.4 anonymous Grid negative `text-indent` 계획 검토 20개

**검토 대상:** [계획](./c11-4-anonymous-grid-text-negative-indent.md), [고정 Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/README.md), [expected JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/expected.json), [fixed HTML fixture](../tests/fixtures/css/c11/c11-4-anonymous-grid-text-negative-indent-precomparison.html).

이 검토는 구현 전 계획·oracle만 대상으로 한다. 제품 코드나 simulator를 실행하지 않았고, PR diff 검토와 합산하지 않는다.

| # | 계획을 공격하는 실패 관점 | 판정과 계획의 통제 |
|---:|---|---|
| 1 | CSS Text 규범이 negative `<length>`를 허용하지 않을 수 있다 | CSS Text 3 §8.1 문법과 length 의미를 Agent Browser로 확인했다. -5/-10px는 유효한 absolute length다. 통과. |
| 2 | `text-indent`가 첫 formatted line만 바꾼다는 조건이 빠져 모든 line advance에서 빼게 될 수 있다 | 규범과 계획이 first formatted line만 대상으로 한다. 통과. |
| 3 | Chrome 값으로 기대값을 역산해 같은 oracle을 반복 검증할 수 있다 | Ahem의 1000 UPEM X/space metrics와 line-break 규칙으로 기대값을 별도로 산출하고 Chrome JSON을 observation으로 분리했다. 통과. |
| 4 | Ahem이 로드되지 않아 fallback font 측정이 섞일 수 있다 | 고정 SHA와 `document.fonts.check("10px Ahem") == true`를 기록했다. 통과. |
| 5 | 10px font와 10px line-height 조건이 서로 다를 수 있다 | fixture·계획·expected JSON이 모두 10px/10px을 사용한다. 통과. |
| 6 | `XXX X`의 min-content에서 긴 첫 단어와 짧은 다음 단어를 대조하지 않을 수 있다 | `max(30+d,10)` 산식과 두 negative 값 장면을 고정했다. 통과. |
| 7 | `X XXX`의 순서를 뒤집어도 결과가 동일하다고 잘못 가정할 수 있다 | 첫 단어가 짧은 대조군을 별도 입력으로 두고 min-content 30px을 고정했다. 통과. |
| 8 | `XX` 단일 단어가 음수 indent로 0 이하 폭을 만드는 상황을 candidate 값에 섞을 수 있다 | -5/-10에서는 15/10px로 양수임을 산출했다. 0 clamp가 필요한 -30px은 제외했다. 통과. |
| 9 | collapsed space advance가 `XXX X` 또는 `X XXX`의 max-content 계산에서 누락될 수 있다 | 각 단어 사이 U+0020 10px을 더해 max-content `50+d`를 산출했다. 통과. |
| 10 | min-content wrap에서 Ahem 각 glyph를 글자 단위로 나눌 수 있다고 가정할 수 있다 | `word-break:normal`의 ASCII word boundary를 고정해 두 word의 break만 허용한다. 통과. |
| 11 | 12개 scene을 6개 style/text 조합으로 축약해 Grid track function을 검증하지 않을 수 있다 | 각 조합을 `min-content`와 `max-content` column에 각각 연결해 12개 scene으로 둔다. 통과. |
| 12 | 기존 positive `text-indent` scenes의 line/track 결과가 회귀할 수 있다 | 기존 PR #930의 18 scenes를 30-scene 통합 회귀 matrix에 유지한다. 통과. |
| 13 | negative CSS value와 typed `ParagraphStyle` 값이 서로 다른 문서 revision에서 올 수 있다 | 같은 computed root/style snapshot, HostDocument revision, scene ID에서 값과 text source를 묶는다. 통과. |
| 14 | `-30px` 경계의 min/max ordering 문제를 작은 negative profile에 숨길 수 있다 | -30은 oracle-only로 구분하고, `TextIntrinsicInlineSizes`의 min≤max 관계와 Grid track floor를 별도 문제로 명시했다. 통과. |
| 15 | 일반 음수값을 허용하면서도 이름만 fixed allowlist라고 부를 수 있다 | -5/-10 및 12 exact scene만 허용하고 -7/-30 및 unlisted case는 fail-closed한다. 통과. |
| 16 | percentage의 intrinsic sizing 규칙이 fixed negative length와 혼합될 수 있다 | CSS Text의 percentage-zero intrinsic rule을 기록하되 percentage는 이번 fixture에서 거부한다. 통과. |
| 17 | `hanging`/`each-line`이 같은 first-line 계산이라고 처리될 수 있다 | 두 keyword가 영향을 받는 line set을 바꾸므로 제외하고 negative control로 확인한다. 통과. |
| 18 | AVD·Simulator 결과를 실기기·hardware GPU·glyph raster 근거로 확대할 수 있다 | 계획은 simulator만 허용하며 screenshot/frame과 glyph/surface pixel을 분리한다. 통과. |
| 19 | opt-in fixture 추가가 기본 product artifact 또는 일반 CSS admission에 새 동작을 노출할 수 있다 | 전용 fixture-test feature와 기본/positive-control archive boundary를 통과 조건으로 둔다. 통과. |
| 20 | 작은 slice 병합으로 C11.4/C11 공식 완료 수를 올릴 수 있다 | 내부 계약 0059 버전·C11/C11.4 checkbox·`72/213`은 변경하지 않는다고 명시했다. 통과. |

## 결론

20개 계획 실패 관점에서 독립 oracle·candidate 범위·negative control·artifact/platform 경계가 구분되어 있다. 계획 단계 결함은 발견되지 않았다. 구현 전 Chrome 관측과 독립 산식이 일치하며, -30px은 미해결 invariant 경계로 구현에서 제외한다.
