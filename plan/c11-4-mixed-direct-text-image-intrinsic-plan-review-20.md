# C11.4 direct text/image non-spanning contribution 계획 실패 관점 20개

이 검토는 구현 전 계획과 사전 oracle만 대상으로 한다. 20개 관점은 각각 별도 입력 가정, 규범 단계, 변환 경계 또는 주장 과장을 공격한다. 구현/runtime 검토 및 PR diff 검토와 합치지 않는다.

| # | 실패 관점 | 계획의 방지 기준 |
|---:|---|---|
| 1 | Chrome 결과가 기대값 계산에 들어가 oracle이 자기 자신을 검증한다 | 기대값은 Ahem glyph advance, break opportunity, image natural size와 §11.5 span-1 절차로 계산하고 Chrome CSSOM은 관측에만 둔다. |
| 2 | Ahem font가 로드되지 않아 fallback glyph width가 사용된다 | browser readiness에서 `document.fonts.check('10px Ahem')`을 true로 요구한다. |
| 3 | `XX XXXXX`에 soft wrap opportunity가 없다고 가정한다 | `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`을 고정하고 5-glyph longest word와 8-glyph unbroken text를 따로 계산한다. |
| 4 | Ahem 공백 폭을 생략해 max-content를 70px로 잡는다 | 공백 glyph 하나를 포함해 `8 × 10px = 80px`로 고정한다. |
| 5 | PNG natural size가 빌드마다 달라지거나 로드 전 측정된다 | 저장소의 고정 24×16 PNG를 사용하고 decode 및 `naturalWidth/Height`를 검사한다. |
| 6 | Replaced image가 min-content에서 natural width를 보존하지 않는다 | image는 authored width/height 및 min/max constraint 없이 사용하고 CSSOM·item frame의 24×16을 관측한다. |
| 7 | 두 item이 서로 다른 column에 놓여 max aggregation을 검사하지 않는다 | 모든 scene에서 두 item을 같은 column 1, span 1에 배치한다. |
| 8 | span 2 이상의 item 처리 결과를 span-1 알고리즘과 혼동한다 | scene contract에 두 item 각각 column span 1을 고정하고 larger span은 거부 경계로 둔다. |
| 9 | row auto placement 순서 차이가 열 track 크기 차이로 오인된다 | text/image의 row는 명시한 fixed rows 1/2로 고정하고 source order만 뒤집는다. |
| 10 | source-order reverse control이 실제로 reverse되지 않는다 | observation에 scene별 direct item source order를 명시하고 forward/reversed 쌍을 비교한다. |
| 11 | min-content 기대 50px이 작은 image 24px에 의해 대체된다 | §11.5의 item contribution 최댓값으로 `max(50,24)=50`을 명시한다. |
| 12 | max-content 기대 80px이 작은 image 24px에 의해 대체된다 | §11.5의 max-content contribution 최댓값으로 `max(80,24)=80`을 명시한다. |
| 13 | definite root width가 track을 120px까지 늘릴 수 있다 | track의 max-content growth limit이 80px이고 alignment가 start임을 적고 Chrome 관측으로 확인한다. |
| 14 | 고정 row height나 grid gap이 column intrinsic 값에 더해진다 | rows 24px, gap 0, padding/border 0을 고정하고 축별 값만 비교한다. |
| 15 | item padding/border가 contribution을 바꾼다 | 모든 item edge를 0으로 고정해 content/outer-size 변환을 이 slice에서 제외한다. |
| 16 | automatic minimum 또는 aspect-ratio suggestion이 직접 기여로 오인된다 | width/height constraint와 aspect ratio를 지정하지 않으며 flex/automatic-minimum track은 명시적 제외로 둔다. |
| 17 | route 확장이 일반 CSS 지원을 열어버린다 | 고정 scene ID, DOM topology, text, image source, profile 및 style allowlist를 요구하고 다른 입력은 fail-closed한다. |
| 18 | Chrome renderer의 CSSOM computed track과 실제 item frame이 어긋난다 | 각 scene에서 CSSOM track, root, text, image frame을 모두 보존하고 독립 기대와 대조한다. |
| 19 | simulator draw/present가 pixel equality로 과장된다 | draw/present와 screenshot은 표시 경로 증거로만 사용하고 pixel claim은 별도 native readback 없이는 하지 않는다. |
| 20 | 네 고정 scene의 성공이 전체 C11.4 또는 72/213 증가로 읽힌다 | 계획과 구현 PR 모두 C11.4 checkbox와 공식 집계를 유지하고 제외 범위를 명시한다. |

## 판정

Chrome 156 Agent Browser 관측에서 네 장면은 min-content 50px, max-content 80px을 forward/reverse 모두 반환했다. 독립 Ahem/PNG 계산 검증기는 `validate-precomparison.py`로 4/4 PASS했다. 이 근거는 제한 계획 범위만 지지하며 Spinon runtime은 검증하지 않았다. 구현 진입 전에 위 경계와 runtime route를 고정한다.
