# C11.4 표준 계산 정정 및 후속 계획 PR diff 적대 검토 30개

**검토 범위:** CSS Grid §11.4–§11.5.1 계산 정정, 기존 Chrome 관측 보존, 새 two-run/three-image fixture 계획, 공식 상태/로드맵 기록, 입력·결과 hash 및 PR screenshot 경로. 각 항목은 별도 실패 경로를 확인한다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 이전 계산 오류가 원본 Chrome 관측값까지 덮어쓰는가? | Chrome 155의 `[16,32,24,0]` CSSOM은 그대로 두고 독립 기대값만 고쳤다. 통과. |
| 2 | 규범 기대값이 beyond-limit 단계부터 계산되는가? | 두-image 뒤 infinite growth limit을 보존하고 span-3 up-to-limit을 먼저 적용한다. 통과. |
| 3 | 두 span-2 image가 앞선 track growth limit을 유한값으로 만든다고 가정하는가? | growth-limit pass에서 남은 공간이 없어 columns 1–2 limit은 무한이다. 통과. |
| 4 | span-3 잔여 공간의 시작 합계가 틀리는가? | 첫 세 track base 합 60px과 image contribution 72px에서 12px 잔여를 얻었다. 통과. |
| 5 | text track의 20px growth limit을 놓치는가? | 단일-span anonymous text가 column 3을 20px로 고정한다. 통과. |
| 6 | 세 번째 track에도 up-to-limit 공간을 잘못 배분하는가? | 20px limit에 도달한 column 3은 보존하고 columns 1–2에 6px씩 배분한다. 통과. |
| 7 | 네 번째 track의 anonymous text contribution을 빠뜨리는가? | 두 번째 run이 column 4의 20px min-content를 만든다. 통과. |
| 8 | 두 source order가 서로 다른 표준 기대값을 갖는가? | 양쪽 모두 `[18,34,20,20]`으로 고정했다. 통과. |
| 9 | Chrome 155 관측에 없는 column 4를 부정확하게 취급하는가? | 기존 3-image 기록의 0px 네 번째 track은 보존하고 새 fixture는 text 두 번째 run으로 20px을 추가한다. 통과. |
| 10 | Chrome 156 관측을 규범 정답으로 승격하는가? | CSSOM `[16,32,24,20]`과 CRD 계산을 별도 표기했다. 통과. |
| 11 | 공개 Blink main 소스가 Chrome 155 내부 원인을 증명한다고 과장하는가? | 고정 바이너리 revision/intermediate state를 증명하지 않는다는 제한을 적었다. 통과. |
| 12 | 두 run이 실제 별도 anonymous item으로 나뉘지 않을 수 있는가? | 직접 text sequence 사이에 `img`가 있어 두 text node를 별도 sequence로 둔다. 통과. |
| 13 | reversed scene에서 placement가 source order와 함께 바뀌는가? | image별 explicit row/column을 유지하고 DOM 순서만 뒤집는다. 통과. |
| 14 | 명시 배치 이미지가 anonymous auto-placement를 가리는가? | image 세 칸을 먼저 점유한 뒤 row 1 column 3/4에 text item을 놓는 oracle을 기록한다. 통과. |
| 15 | PNG natural dimensions와 contribution이 불일치하는가? | 고정 24×16 PNG 및 16/32/48px 높이에서 24/48/72px contribution을 연결했다. 통과. |
| 16 | Ahem 미로드 상태를 예상 입력으로 세는가? | precomparison JSON은 `fontReady`와 이미지 decode/natural size를 저장하고 fixture가 readiness를 기다린다. 통과. |
| 17 | image decode 실패가 성공 관측에 섞였는가? | 양 장면의 여섯 image decode 상태와 natural size가 모두 기록돼 있다. 통과. |
| 18 | fixture 수정 후 CSSOM JSON이 이전 실행과 섞이는가? | fixture SHA-256이 README에 고정돼 JSON 및 screenshot과 연결된다. 통과. |
| 19 | screenshot이 다른 fixture 실행에서 나온 파일일 수 있는가? | README의 screenshot hash가 실제 파일 SHA-256과 일치한다. 통과. |
| 20 | text DOM Range rect를 anonymous item frame이라고 오인하는가? | 사전 비교는 text range와 image rect를 관찰로만 기록하며 런타임 계획은 item frame 별도 검증을 요구한다. 통과. |
| 21 | 34px overflow가 border box를 기준으로 잘못 계산되는가? | root-local image `[2,74]`, border-box 오른쪽 40px이므로 overflow는 34px이다. 통과. |
| 22 | 색 표본이 overflow 영역 밖 또는 투명한 PNG texel인가? | `(62,128)`은 root-local 오른쪽 border에서 22px 바깥이며 대응 source texel `(20,2)`는 opaque `#00ff40`이다. 통과. |
| 23 | CSS viewport 좌표 변환에 root origin을 누락하는가? | root `(4,22)`를 더한 sample 위치 `(66,150)`을 기록했다. 통과. |
| 24 | border 색 표본과 overflow 표본이 동일 좌표로 혼동되는가? | 계획은 top-border `(10,0)`과 overflow `(62,128)`을 별도 표본으로 지정한다. 통과. |
| 25 | image가 border나 padding 때문에 예상 frame과 다른가? | fixture에서 `box-sizing:border-box`, image margin/padding/border 0을 고정했다. 통과. |
| 26 | outer root의 40px 폭을 content width로 오해하는가? | 계획은 2px border 양쪽을 제외한 36px content width를 명시한다. 통과. |
| 27 | scene ID와 evidence scene 이름이 어긋나는가? | 계획과 JSON의 forward/reversed scene ID가 동일하다. 통과. |
| 28 | 새로운 계획이 구현 완료나 C11.4 완료로 승격되는가? | implementation 미시작 및 C11.4/72-of-213 미변경을 status와 plan에 적었다. 통과. |
| 29 | 플랫폼 결과가 Simulator를 넘어서는가? | Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator, compositor 전 Metal 한계를 지정했다. 통과. |
| 30 | 리뷰 기록이 계획 전용 20개와 PR diff 검토를 혼동하는가? | 계획 검토 20개와 본 PR 전체 diff 30개를 별도 파일로 둔다. 통과. |

검토 중 blocker는 발견하지 못했다. 이 기록은 계획 PR에만 해당하며 구현 코드/runtime의 새 20관점 검토를 대신하지 않는다.
