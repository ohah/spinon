# C11.4 nested Grid 행 축 contribution 계획 실패 관점 검토 20개

**검토 대상:** 고정 nested Grid 입력, 독립 CSS Grid 계산, cross-axis 의존성, simulator 및 artifact gate, 미지원 경계. 구현 전에 서로 다른 계획 실패 관점으로 점검했다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 새 slice가 기존 nested Grid text/image fixture와 다른 범위를 검증하는가? | 같은 한 단계 topology에서 outer row contribution을 직접 비교하며 runtime 동작은 아직 미검증이라고 적었다. 통과. |
| 2 | row 관측이 column sizing 결과와 무관한 입력인가? | 98px definite outer width가 inner 70/24px columns를 정하고 text wrapping을 거쳐 row 높이 20px가 되도록 고정했다. 통과. |
| 3 | Ahem text의 min-content advance가 70px이 아닌가? | `XXXXXXX` 일곱 glyph × 10px로 longest word 70px이며 fixture hash/font hash를 고정했다. 통과. |
| 4 | 두 줄 text의 block height가 line-height와 맞지 않는가? | 70px column에서 두 word가 분리되고 `line-height:10px` 두 줄이 20px가 된다. 통과. |
| 5 | text의 max-content 폭이 inner max-content track 입력과 충돌하는가? | `XX XXXXXXX` unwrapped advance 100px를 별도 설명하고 min-content track만 70px로 사용한다. 통과. |
| 6 | image가 CSS width/height 때문에 natural-size 외 값이 되는가? | author dimensions를 두지 않고 PNG decode 후 natural size 24×16px를 기록한다. 통과. |
| 7 | image 또는 Ahem이 준비되기 전 측정될 수 있는가? | fixture가 font readiness와 세 image decode 완료를 기다리고 observation에 complete/natural size를 남긴다. 통과. |
| 8 | inner column과 gap 합이 98px outer width를 넘거나 모자라는가? | `70 + 4 + 24 = 98px`로 정확히 채워 nested inline size를 결정한다. 통과. |
| 9 | inner row contribution에서 row gap이 중복되거나 빠지는가? | inner rows `[20,16]`에 inner 4px gap을 한 번 포함해 40px로 계산한다. 통과. |
| 10 | outer spanning distribution에서 outer gap을 contribution으로 다시 나누는가? | 40px에서 4px outer gap을 제외해 36px을 두 track에 18px씩 배분한다. 통과. |
| 11 | spanning scene의 root height 계산에 gap이 반영되지 않는가? | tracks 18+18에 outer gap4를 더해 40px이다. 통과. |
| 12 | one-track control이 spanning distribution과 구분되지 않는가? | 같은 40px contribution을 첫 row 하나에 적용하고 두 번째 row는 0px로 고정한다. 통과. |
| 13 | one-track root height에 zero-size row와 gap 처리의 영향이 빠지는가? | `[40,0]` track과 4px row gap의 합으로 44px을 계산한다. 통과. |
| 14 | definite-flex control에 불명확한 available space가 있는가? | 60px definite block-size에서 4px gap을 빼 56px을 두 1fr에 분배한다. 통과. |
| 15 | flex control에서 §6.6 automatic minimum 조건이 성립하지 않는가? | item이 두 flexible row를 span하므로 automatic minimum 0px을 조건부로 적용한다. 통과. |
| 16 | nested grid stretch와 내부 row sizing을 혼동하는가? | nested frame은 98×60으로 stretch되지만 내부 min-content rows와 child frames는 `[20,16]`, y=0/24로 분리했다. 통과. |
| 17 | CSS Grid의 normative expected와 Chrome observation을 혼합하는가? | §6.6/§11.1/§11.5.1 독립 계산 및 Chrome 156 관측을 별도 경로로 적었다. 통과. |
| 18 | scene 좌표가 root-relative인지 page-relative인지 모호한가? | table의 모든 nested/text/image coordinates를 scene root 상대 frame으로 명시했다. 통과. |
| 19 | fixture adapter가 입력이 바뀐 경우에도 조용히 성공하거나 제품 artifact에 섞이는가? | 추가·누락 child와 unsupported style, font/image 미준비는 partial layout 없이 거부하고 `c04-runtime-gpu-fixture-test` opt-in artifact boundary도 확인하도록 계획했다. 통과. |
| 20 | 단일 fixture pass가 전체 C11.4 또는 플랫폼 적합성으로 승격되는가? | WPT·다른 browser·물리기기·arbitrary nested Grid를 제외하고 72/213 및 unchecked C11.4를 고정했다. 통과. |

이 기록은 계획 전용이다. 구현/runtime 실패 검토 20개와 PR 변경 diff 검토 20개는 각 단계에서 별도로 작성한다.
