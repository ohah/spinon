# C11.4 `fit-content(px)` implicit row 계획 실패 관점 검토 20개 · 2026-11-07

구현 전 계획·Chrome fixture·독립 판정 조건을 서로 다른 실패 경로로 살폈다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 대상이 explicit `grid-template-rows` 변경으로 구현될 수 있음 | explicit row `[10px]`과 implicit auto-row를 별도 track frame 및 뒤 probe로 고정했다. 통과. |
| 2 | `fit-content()`를 일반 `minmax()` 또는 `max-content`와 같은 값으로 취급함 | typed function identity와 cap24/room40 대조를 pass criteria에 넣었다. 통과. |
| 3 | cap 24가 실제 sizing limit으로 사용되지 않고 content 34를 그대로 허용함 | 독립 공식 `max(minimum,min(limit,max-content))`과 Chrome `[10,24,6]`을 고정했다. 통과. |
| 4 | cap보다 작은 34px contribution을 cap까지 키운다고 가정함 | room40 control은 track 34px로 두며 limit보다 content가 작을 때의 결과를 구분한다. 통과. |
| 5 | item minimum이 fit-content cap보다 낮은 우선순위로 계산됨 | min-floor scene은 `min-height:28px`, cap24와 content34를 사용해 row28을 요구한다. 통과. |
| 6 | Grid의 definite extra space가 fit-content row를 stretch함 | `align-content:start`, definite 100px root와 used-row CSSOM 값을 pass condition에 둔다. 통과. |
| 7 | 뒤 probe가 sizing item으로 함께 계산되어 cap row contribution을 변경함 | Probe는 세 번째 independent row의 6px child이며 middle item과 분리된 순서를 고정했다. 통과. |
| 8 | 두 번째 implicit row sizing이 첫 implicit row 크기와 혼동됨 | probe y coordinate가 middle row의 끝과 일치하는지 비교해 track frame을 독립 추론한다. 통과. |
| 9 | explicit row 높이 또는 column 폭이 implicit-row 비교를 바꿈 | 단일 40px column 및 10px explicit row를 모든 scene에서 고정했다. 통과. |
| 10 | percentage cap이 fixed px와 같은 basis로 잘못 승인됨 | scope는 `LengthPx`만 허용하고 percentage는 negative control로 거부한다. 통과. |
| 11 | `calc()` 또는 cyclic basis가 단일 px cap 경로로 유입됨 | plan은 calc/cyclic limit을 분리된 unsupported computed value로 거부한다. 통과. |
| 12 | mixed/multi track list가 singleton route를 우회함 | 정확한 singleton shape만 허용하고 여러 track 및 mixed list를 거부하도록 했다. 통과. |
| 13 | intrinsic explicit column과 fit-content row가 함께 admission됨 | explicit columns/rows는 positive fixed px singletons이며 intrinsic 조합은 rejection matrix에 있다. 통과. |
| 14 | dense/column flow 또는 implicit column이 row candidate를 바꿈 | sparse row, auto `grid-auto-columns`에 고정하고 다른 flow/implicit column을 거부한다. 통과. |
| 15 | areas, line names 또는 auto-repeat의 generated track이 검증을 피함 | 해당 metadata, auto-repeat과 names를 candidate 조건에서 배제했다. 통과. |
| 16 | style projection이 CSSOM 값을 cap 없이 `Auto`로 낮춤 | computed typed `FitContent(LengthPx)` 및 projection 결과를 개별 확인하도록 계획했다. 통과. |
| 17 | Layout admission만 바꾸고 placement/track capacity가 candidate를 거부함 | projection, layout, implicit placement, capacity를 개별 검증 대상으로 지정했다. 통과. |
| 18 | unsupported computed style 실패 후 partial layout이나 draw가 publish됨 | RuntimeSession property/node 오류, `completed=None`, same-session recovery를 관문에 넣었다. 통과. |
| 19 | Chrome 사전 관측을 Spinon 구현 통과로 확대 해석함 | Chrome oracle, 독립 계산, Rust, Android/iOS Simulator 결과를 서로 다른 단계로 분리했다. 통과. |
| 20 | 이 제한 profile이 일반 CSS/Grid 또는 C11.4 완료로 공표됨 | STATUS 완료 체크와 `78/214`를 유지하고 일반 admission/WPT/실기기를 미지원으로 명시했다. 통과. |

계획 및 사전 비교에서 구현 착수 전에 수정할 미해결 결함은 발견하지 못했다. 구현 결과는 별도 20개 실패 관점으로 새로 검토한다.
