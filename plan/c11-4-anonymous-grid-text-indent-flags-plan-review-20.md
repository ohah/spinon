# C11.4 `text-indent` flags 계획 실패 관점 검토 20개

**대상:** [`c11-4-anonymous-grid-text-indent-flags.md`](./c11-4-anonymous-grid-text-indent-flags.md), 고정 Chrome observation, 독립 기대값 및 HTML fixture.

각 항목은 구현 전에 별도의 실패 경계로 확인했다. 계획 review는 구현/runtime review 또는 PR diff review를 대신하지 않는다.

| # | 실패 관점 | 판정 및 근거 |
| ---: | --- | --- |
| 1 | fixture 확장이 일반 CSS admission이나 제품 HTML 경로를 열 수 있는가? | 기존 opt-in anonymous direct-text route, exact DOM profile로 고정했다. **PASS** |
| 2 | `each-line`을 soft wrap 뒤에도 적용하는가? | CSS Text §8.1 경계와 `XXX X` soft-wrap controls는 subsequent soft line indent를 기대값에서 분리한다. **PASS** |
| 3 | forced line break를 ordinary soft wrap과 동일하게 취급하는가? | `XXX\nX`/`pre-line` input을 별도 profile로 두고 two-line origins를 고정한다. **PASS** |
| 4 | `hanging`이 첫 줄이 아닌 continuation line에 반영되는가? | Ahem width 30/10의 soft-wrap control은 첫 줄 `0`, 다음 줄 `10` origin과 min width 30을 고정한다. **PASS** |
| 5 | `each-line`가 forced line break 이후의 두 번째 line에 빠지는가? | Forced case의 second origin은 each-line 단독에서 10px이다. **PASS** |
| 6 | `hanging each-line` 조합의 상호작용이 누락되는가? | 4개 forced/soft × min/max control에 computed canonical value와 line origin을 별도로 고정한다. **PASS** |
| 7 | Browser output을 규범 기대값으로 그대로 복사하는가? | Ahem metrics와 CSS Text/Sizing/Grid 계산으로 expected JSON을 독립 산출하고 Chrome을 교차 확인으로 분리한다. **PASS** |
| 8 | min-content와 max-content의 content advance가 섞이는가? | `XXX X`는 min `[30,10]`, max `[50]`; forced input은 `[30,10]`으로 별도 고정한다. **PASS** |
| 9 | indent를 Parley line advance에 이미 포함된 것으로 보고 두 번 더하는가? | 계획은 `LineMetrics.advance`와 별도 `LineMetrics.indent`를 분리해 합산한다고 명시한다. **PASS** |
| 10 | 현재 line index 기반 보정이 flags를 무시하는가? | 계획은 `line_index == 0` 특례 대신 actual line indent를 intrinsic input에 사용한다. **PASS** |
| 11 | `text-indent` computed serialization의 keyword 순서가 source 문자열에 종속되는가? | scene ID inference를 금지하고 같은 computed snapshot의 canonical serialization을 검증한다. **PASS** |
| 12 | flags만으로 length 값이나 scene이 추측되는가? | computed length+flags와 exact expected scene을 따로 대조한다. **PASS** |
| 13 | Forced break의 zero-width Range rectangle이 추가 line으로 셈되는가? | Precomparison verifier는 zero-width break marker를 line text fragment에서 제외하고 root height/y origins를 함께 확인한다. **PASS** |
| 14 | multiple source, inline spans 또는 generated marker가 CSS semantics를 바꾸는가? | 고정 fixture는 direct anonymous text single source이며 inline runs/other nodes를 거부한다. **PASS** |
| 15 | line-height나 font loading 차이가 track 오류로 오인되는가? | Ahem digest, loaded font predicate, 10px line-height와 20px forced/min-content heights를 고정했다. **PASS** |
| 16 | 기존 positive/negative `text-indent` 동작이 회귀하는가? | 양수 18-scene 및 음수 12-scene matrices를 explicit regression gate로 유지한다. **PASS** |
| 17 | Computed snapshot/revision이 바뀌어 부분 layout을 게시하는가? | stale/missing revision과 mismatch는 frame/trace/draw publication 전에 거부한다. **PASS** |
| 18 | Font line indent가 CSS Grid track contribution과 분리되는가? | Expected used track, root/item geometry, text line origin과 contribution을 same revision으로 연결한다. **PASS** |
| 19 | 제한 simulator·archive 증거가 glyph/실기기/public CSS를 증명한다고 과장되는가? | AVD/iOS boundary와 archive 한계를 분리하고 glyph ink/WPT/hardware/whole C11.4는 제외했다. **PASS** |
| 20 | 제한 slice가 C11.4 또는 `72/213`을 완료 처리하는가? | Parent check와 official global count는 그대로 미완료로 둔다. **PASS** |

**결과:** plan, fixture, independent expected values 및 Chrome observation의 20개 failure perspective를 확인했다. Implementer review와 PR diff review는 이후 별도로 해야 한다.
