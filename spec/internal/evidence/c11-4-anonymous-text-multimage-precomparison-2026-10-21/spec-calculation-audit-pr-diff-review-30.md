# C11.4 anonymous text/image 표준 계산 수정 PR diff 적대 검토 30개

**검토 대상:** Chrome 155/156 관측과 독립 CSS Grid 2025 CRD 계산 사이의 문서 불일치 수정. 코드는 바꾸지 않았고 C11.4 상태와 공식 완료 수는 유지했다.

| # | 독립 실패 관점 | 대조와 판정 |
| --- | --- | --- |
| 1 | 고정 입력 fixture가 검토 도중 달라졌는가? | fixture SHA-256 `5710b2726af62529944b21f03b35212d38d4caa90a0774a24f694e837a435c5b`를 재확인했다. 통과. |
| 2 | Chrome 155 관측 JSON이 수정되어 차이를 감췄는가? | 원본 JSON을 수정하지 않고 원본 해시를 그대로 유지했다. 통과. |
| 3 | Chrome 155 screenshot이 새 기대값에 맞게 다시 만들어졌는가? | 원본 screenshot 파일과 해시를 유지했다. 통과. |
| 4 | 계산 기준 날짜·문서가 잘못 연결됐는가? | 2025-03-26 CSS Grid CRD와 §11.4–§11.5.1 링크를 대조했다. 통과. |
| 5 | 모든 `min-content` track의 초기 base size를 0으로 보지 않았는가? | §11.4 초기화 규칙과 계산 설명이 일치한다. 통과. |
| 6 | 모든 intrinsic max sizing function의 growth limit 초기값을 잘못 유한값으로 가정했는가? | columns 1–2의 초기 무한 growth limit을 명시했다. 통과. |
| 7 | span-1 text의 Ahem min-content 폭이 다른 입력에서 왔는가? | 고정 Ahem `XX`와 20px 기여를 fixture 및 결과 표와 대조했다. 통과. |
| 8 | anonymous text item이 명시 배치됐다고 잘못 계산했는가? | CSS Grid §6·§8.5에 따른 auto-placement row 1, column 3과 기록을 대조했다. 통과. |
| 9 | 세 image의 natural ratio 또는 높이 입력이 다른가? | 고정 PNG `24×16`과 세 높이 입력을 확인했다. 통과. |
| 10 | span-2 첫 image의 inline contribution이 24px이 아닌가? | ratio 및 고정 높이 16px로 24px을 확인했다. 통과. |
| 11 | span-2 둘째 image의 contribution이 48px이 아닌가? | ratio 및 고정 높이 32px로 48px을 확인했다. 통과. |
| 12 | span-3 image의 contribution이 72px이 아닌가? | ratio 및 고정 높이 48px로 72px을 확인했다. 통과. |
| 13 | text가 있는 track 3의 base size와 growth limit이 잘못됐는가? | non-spanning Ahem text가 둘 다 20px으로 만든다는 기록을 확인했다. 통과. |
| 14 | 첫 span-2 image의 기여가 columns 1–2 밖으로 적용됐는가? | 고정 placement와 두 track의 12px씩 증가를 확인했다. 통과. |
| 15 | 첫 image 후 track 합과 contribution이 다르게 쓰였는가? | `12+12=24`로 양쪽 기록을 대조했다. 통과. |
| 16 | 첫 image 후 columns 1–2 growth limit이 이미 12px이라고 가정했는가? | growth-limit pass의 무한값 치환과 0 남은 공간 설명을 추가했다. 통과. |
| 17 | 둘째 image가 columns 2–3 이외의 track을 span하는가? | fixture CSS placement를 확인했다. 통과. |
| 18 | 둘째 image 전의 base 합 32px 계산이 틀렸는가? | `12+20=32`를 재계산했다. 통과. |
| 19 | 둘째 image의 남은 기여가 16px이 아닌가? | `48-32=16`을 계산 설명 및 표와 대조했다. 통과. |
| 20 | track 3이 20px growth limit을 넘도록 up-to-limit 단계에서 키웠는가? | track 3의 남은 capacity가 0임을 명시했다. 통과. |
| 21 | 둘째 image의 16px이 다른 track에 배분됐는가? | 무한 growth limit인 column 2에 전부 배분해 track `[12,28,20,0]`을 얻는다. 통과. |
| 22 | 둘째 image 뒤 columns 1–2 growth limit이 유한해졌다고 잘못 서술했는가? | `12+12=24`, `28+20=48`인 growth-limit pass 잔여 0을 기록했다. 통과. |
| 23 | 셋째 image의 span 또는 base 합이 틀렸는가? | 첫 세 track만 span하고 `12+28+20=60`임을 확인했다. 통과. |
| 24 | 셋째 image에서 필요한 추가 공간이 12px이 아닌가? | `72-60=12`를 확인했다. 통과. |
| 25 | up-to-limit 분배 대상의 현재 capacity를 놓쳤는가? | columns 1–2는 무한, column 3은 20px limit에 도달한 상태로 분리했다. 통과. |
| 26 | 두 대상 track의 증가량을 각 6px로 계산하지 않았는가? | 12px을 두 무한 track에 균등 배분해 `[18,34,20,0]`을 얻는다. 통과. |
| 27 | beyond-limit 단계가 필요하다고 잘못 기록했는가? | up-to-limit 단계에서 12px 전부 소진되어 다음 단계 잔여가 0임을 명시했다. 통과. |
| 28 | reverse-order Chrome 장면을 규범 결과와 혼동했는가? | 두 source order의 규범 기대와 Chrome 관측을 서로 다른 표 열에 유지했다. 통과. |
| 29 | Chrome 155와 HeadlessChrome 156의 역할·버전이 합쳐졌는가? | 고정 155 기록과 Agent Browser 156 재실행을 분리하고, 156 JSON에 실제 UA를 기록했다. 통과. |
| 30 | CSSOM track 관찰을 Blink 내부 원인 또는 전체 브라우저 적합성 결론으로 과장했는가? | exact 155 source revision·중간 state·전반적 적합성을 입증하지 않는다고 명시했다. 통과. |

## 반영 상태

- 오래된 사전 비교 README의 세-image 독립 기대값을 `[18,34,20,0]`으로 정정했다.
- Chrome 155 및 HeadlessChrome 156의 `[16,32,24,0]` 관측과 원본 screenshot/JSON은 보존했다.
- 계획과 `spec/STATUS.md`를 같은 판정으로 맞췄다. C11.4 checkbox 및 공식 `72/213` 완료 수는 변경하지 않았다.
- 앱 코드·runtime fixture·공개 API는 변경하지 않았다.
