# Nested Grid positive implicit rows 계획 실패 관점 검토 20개

계획과 고정 입력을 구현 시작 전 별도로 검토했다. 각 항목은 서로 다른 계산·경계·실패 조건을 확인한다.

| # | 실패 질문 | 확인과 판정 |
| --- | --- | --- |
| 1 | 이미 합쳐진 nested text/image 또는 nested row slice와 같은 범위를 중복 검사하는가? | 이전 계획이 제외한 nested implicit track 확장을 명시하고, 해당 cross-product만 대상으로 삼는다. 통과. |
| 2 | 명시 track 수를 잘못 세어 implicit row가 하나만 생기는가? | explicit row 1개는 lines 1–2이고 `grid-row:3/4`는 뒤에 두 positive implicit rows를 필요로 한다. 통과. |
| 3 | `grid-auto-rows` list cycle의 시작 인덱스가 모호한가? | §7.6의 first-after-explicit 규칙과 `[min-content,max-content]`를 적고 row 2/3 각각을 입력으로 채운다. 통과. |
| 4 | 첫 implicit row가 비어 있어 `min-content` pattern 위치를 관찰하지 못하는가? | `grid-row:2/3`의 4×8px fixed block이 첫 implicit row의 크기를 보인다. 통과. |
| 5 | 두 번째 implicit row가 content 없이 max-content 크기를 얻는다고 잘못 가정하는가? | natural-size image가 `grid-row:3/4`에 놓여 max-content track을 직접 채운다. 통과. |
| 6 | explicit row의 기여를 implicit row 기여로 오인하는가? | 첫 row는 별도 24×8px child와 `grid-template-rows:min-content`로 고정했다. 통과. |
| 7 | 이미지 decode 또는 CSS size가 바뀌어 block contribution이 달라지는가? | 각 image decode 뒤 complete/natural dimensions를 검사하고 natural 24×16px를 기록한다. 통과. |
| 8 | 이미지 intrinsic ratio가 width constraint 때문에 뜻하지 않게 조정되는가? | nested/outer column과 이미지 natural width가 모두 24px이며 다른 width/ratio constraint는 없다. 통과. |
| 9 | percentage basis나 cyclic sizing이 기대값에 관여하는가? | 모든 axis sizing은 고정 px, min-content, max-content만 쓴다. percentage/calc는 제외한다. 통과. |
| 10 | inner row gap 수를 하나 또는 세 개로 세는가? | 세 row 사이 gap 두 개를 각각 4px로 적어 `8+4+8+4+16=40px`로 계산한다. 통과. |
| 11 | 내부와 outer Grid의 gap이 같은 식으로 잘못 더해지는가? | nested contribution 40px와 outer gap 4px을 별도 계산한다. 통과. |
| 12 | outer item이 두 행 span일 때 item gap을 분배 대상에 포함하는가? | outer 40px 중 gap 4px을 뺀 36px을 `[18,18]`로 분배한다. 통과. |
| 13 | 한 행 control에서 빈 두 번째 track을 잊는가? | `one-row`는 outer rows `[40,0]`, root height `40+4=44px`로 확인한다. 통과. |
| 14 | border, padding, margin 또는 auto stretch가 40px 이외의 box 크기를 섞는가? | box sizing, zero edge sizes, `align-self:start`, `align-content:start`를 고정한다. 통과. |
| 15 | scene label 또는 viewport 위치가 normative frame 비교를 오염시키는가? | 절대 y는 관찰 정보로만 저장하고 규범 판정은 nested/root 상대 크기·track에 둔다. 통과. |
| 16 | 실수 track 크기를 정수 tolerance로 숨기는가? | 고정 입력은 정수 px이며 기준 허용오차는 0px로 미리 정했다. 통과. |
| 17 | CSSOM used `gridTemplateRows`를 규범 계산 자체로 취급하는가? | browser observation과 `spec-derived-expectations.json`을 별도 산출물로 둔다. 통과. |
| 18 | draw/present 또는 geometry만으로 border 바깥 색을 증명한다고 과장하는가? | overflow가 없는 fixture이고 beyond-border pixel 주장도 명시적으로 제외한다. 통과. |
| 19 | 이 두 장면을 auto-placement, negative implicit, subgrid 또는 임의 parser로 확장 해석하는가? | 각 범위를 제외 목록과 unsupported 입력 기준에 적었다. 통과. |
| 20 | 계획 단계의 Chrome 결과를 Spinon 구현 완료나 C11.4 완료로 센다? | runtime/Simulator 미실행과 72/213 유지 및 C11.4 미완료를 상단에 명시한다. 통과. |

검토 중 수정 사항은 없었다. 이는 계획 검토이며 구현/runtime 검토나 PR diff 검토를 대신하지 않는다.
