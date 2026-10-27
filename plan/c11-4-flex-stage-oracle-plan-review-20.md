# C11.4 §11.7 flexible track expansion 계획 · 실패 관점 검토 20개

**대상:** [test-only 계획](./c11-4-flex-stage-oracle.md)과 [CSS Grid 독립 산출물·Chrome 사전 관측](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/README.md)

계획을 구현하기 전에 서로 다른 입력·산술·trace·범위 실패 가능성을 검토했다. 각 항목은 현재 계획과 근거 코드/산출물에 대조했다. 이 기록은 구현/runtime 검토나 PR 변경분 검토를 대신하지 않는다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | §11.7 free-space 분배가 다른 규칙에서 유래 | **통과.** 기준은 CSS Grid 2025 CRD §§11.7–11.7.1이며 기대값은 Taffy trace나 Chrome 값이 아니라 별도 Fraction 구현에서 산출한다. |
| 2 | 정수 나눗셈·십진 반올림이 유리수 기대값을 손상 | **통과.** Python `Fraction`이 기대 배열을 만들고 JSON은 `190/3`, `380/3`을 보존한다. |
| 3 | 1fr:2fr 사례가 비율 분배를 충분히 식별하지 못함 | **통과.** 200px definite axis에서 10px gap을 빼고 두 CSS track 기대값을 각각 `190/3`, `380/3`으로 검사한다. |
| 4 | 최소 base보다 작은 첫 가설에서 재계산하지 않음 | **통과.** `frozen-row`의 1fr track은 80px base와 첫 63⅓px 배분을 비교해 동결되고, 남은 2fr track은 110px이 된다. |
| 5 | factor 합계가 1보다 작은 경우 1 floor를 놓침 | **통과.** `subunit-columns`의 0.5fr track은 85px, 고정 20px track은 20px로 기대하고 85px 잔여 공간을 별도로 기록한다. |
| 6 | 0fr을 항상 0px track으로 취급 | **통과.** `zero-fr-row`는 24px minimum을 유지하고 1fr 이웃이 166px을 받아 10px gap과 함께 200px이 된다. |
| 7 | 고정 track의 base size가 free space에서 빠지지 않음 | **통과.** sub-unit 입력은 고정 20px track을 포함하고 독립 산출 스크립트가 frozen/inflexible base를 차감한다. |
| 8 | 반대 축의 gap 또는 크기가 대상 축 기대값을 바꿈 | **수정 반영.** 계획에 target/opposite size를 모두 비교하고 지정된 대상 축에만 10px gap을 설정하며 반대 축 gap은 0으로 둔다고 명시했다. Fixture와 `expected.json`도 Columns/Rows별 대상 축만 10px이다. |
| 9 | 200px이 root에만 설정되고 viewport는 indefinite 또는 다른 크기 | **통과.** 네 경우 root와 viewport 모두 width·height를 각각 대상축 200px, 반대축 120px로 맞춘다. |
| 10 | Columns 또는 Rows 한 축만 검사 | **통과.** 두 사례씩 양 축에 분배하고 axis enum을 기대 JSON 및 trace pass와 연결한다. |
| 11 | fixture 입력과 Rust 입력의 track sizing 함수가 다름 | **통과.** 계획은 `GridTrackSize::MinMax`의 `LengthPx` minimum과 `Fraction` maximum을 실제 typed input으로 만들고 descriptor/base를 oracle JSON과 대조한다. |
| 12 | 빈 grid 외 item contribution이나 spanning이 sizing을 오염 | **통과.** 각 Rust input은 grid item을 추가하지 않으며 item contribution·span은 범위에서 제외한다. |
| 13 | JSON case 누락·중복·추가가 조용히 통과 | **통과.** test는 네 case ID의 정확한 집합과 유일성을 검사하도록 계획했다. |
| 14 | 예상 axis가 아닌 pass/stage를 선택 | **통과.** root trace, axis pass 하나, 고유 `ExpandFlexibleTracks` stage를 확인하고 vector index에 의존하지 않는다. |
| 15 | gutter를 CSS track으로 세거나 boundary sentinel을 무시 | **통과.** stage 배열의 두 CSS track, internal gap gutter, 양쪽 boundary gutter와 collapsed sentinel을 각각 검사한다. `calc_tree.rs`는 Taffy trace의 gutter/collapsed flag를 보존한다. |
| 16 | 이전 또는 이후 stage 결과를 expansion 결과로 착각 | **통과.** 계획은 `MaximizeTracks` 뒤, `StretchAutoTracks` 앞에 있는 고유 expansion snapshot을 검사한다. |
| 17 | 1e-4 tolerance 때문에 NaN/무한대나 잘못된 rational 분모가 수용 | **통과.** 정수 또는 `numerator/denominator`만 허용하고 양의 분모와 finite `f32` 변환을 확인한 뒤 CSS px 절대오차를 적용한다. |
| 18 | 내부 growth limit/flex fraction을 규범 기대값으로 과장 | **통과.** CSS 기대값 판정은 최종 track `base_size`에 한정하고 `growth_limit`, `infinitely_growable`, 선택 fr fraction은 주장하지 않는다. |
| 19 | Chrome quantization이 규범 기대값을 덮거나 Chrome과 동일성을 주장 | **통과.** 독립 유리수 결과와 Chrome DOMRect를 별도 표에 두며 Chrome 대조는 1/64px까지의 browser control로 제한한다. |
| 20 | 네 test-only 사례를 app/runtime 또는 C11.4 전체 완료로 확대 | **통과.** 계획은 simulator, 일반 CSS admission, WPT 및 app runtime을 제외하고 C11.4 checkbox와 공식 `72/213`을 유지한다. |

**판정:** 검토 중 반대 축 gap 경계를 명확히 하는 수정 한 건을 계획에 반영했다. 수정된 계획과 fixture/JSON 입력을 다시 대조했으며, 구현을 막는 미해결 결함은 발견하지 못했다. 계획 PR 변경분은 별도 문서에서 새 30개 관점으로 검토한다.
