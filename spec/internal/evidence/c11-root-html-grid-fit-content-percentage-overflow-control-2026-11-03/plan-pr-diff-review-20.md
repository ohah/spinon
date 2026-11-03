# C11.1 `fit-content(%)` 계획 PR 변경 검토 20개

검토 대상은 새 `fit-content(%)` 계획, 계획 자체의 20개 review, STATUS 후보 행, Chrome percentage matrix와 root-border overflow evidence다. 이는 구현 review가 아니다.

| # | PR diff 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 기존 #1093의 범위를 소급해 넓혔다고 잘못 설명한다 | **PASS** — #1093 계획에서 percentage/calc를 제외했음을 새 후속 계획에 명시했다. |
| 2 | 새 계획이 이미 구현된 기능처럼 STATUS에 적힌다 | **PASS** — STATUS는 계획 후보라 표시하고 구현 미시작을 썼다. |
| 3 | C11 완료 또는 72/213 집계를 변경한다 | **PASS** — checkbox와 집계 문자열은 변경하지 않고 유지한다. |
| 4 | 규범 formula가 browser output에서 역산된다 | **PASS** — independent expected JSON은 Chrome observation과 별도 파일이며 사전 계산 출처를 기록한다. |
| 5 | percentage 입력을 fraction 값으로 변환하는 단위가 모호하다 | **PASS** — `20% of 200px = 40px`와 nine-scene resolved cap을 함께 제시한다. |
| 6 | column과 row가 같은 축 basis를 잘못 공유한다 | **PASS** — row case는 100px height basis를 별도로 고정한다. |
| 7 | 0% 및 zero basis에서 invalid/indefinite를 혼동한다 | **PASS** — 0%와 definite zero basis가 별도 matrix case다. |
| 8 | 100% 초과 cap을 clamp할 수 있다 | **PASS** — 120% case에서 resolved limit과 max-content result를 따로 기록한다. |
| 9 | automatic minimum floor를 제외해 cap이 너무 낮아진다 | **PASS** — below-minimum cases와 fixed-size floor를 acceptance에 둔다. |
| 10 | max-content 이하/이상 cap을 구분하지 않는다 | **PASS** — below/intermediate/above max-content cases를 각각 가진다. |
| 11 | integer repeat과 auto-repeat을 같은 문법으로 admit한다 | **PASS** — 기존 positive integer expansion만 포함하고 auto-repeat을 제외한다. |
| 12 | percentage를 implicit `grid-auto-*`로 전파한다 | **PASS** — explicit template axes만 긍정하고 implicit sizing을 제외한다. |
| 13 | named-area support가 percentage fit-content와 함께 확장된다 | **PASS** — 조합은 scope 밖이며 기존 px area path regression만 요구한다. |
| 14 | calc/minmax/line names가 직접 percentage 허용에 따라 들어온다 | **PASS** — 각 변형을 명시적인 negative gate로 둔다. |
| 15 | indefinite/cyclic axis가 임의값으로 계산된다 | **PASS** — Root HTML profile에서 definite 해당 axis를 요구한다. |
| 16 | border overflow는 screenshot만 있고 frame 근거가 없다 | **PASS** — root/child boxes, 28px overflow, border-edge marker와 pixel samples를 JSON/PNG에 보존한다. |
| 17 | screenshot sample이 DOM hit-test를 pixel 값으로 오인한다 | **PASS** — sample RGBA는 저장 PNG에서 읽었고 HTML hit-test 값을 사용하지 않았다. |
| 18 | Chrome fixture/screenshot이 수정 후 stale hash로 남는다 | **PASS** — fixture·expected·observation·screenshot hash를 manifest에 고정했다. |
| 19 | Ahem Chrome 기준을 실제 mobile system-font runtime 근거로 확대한다 | **PASS** — Ahem layout matrix와 fixed-box Root HTML runtime을 분리했다. |
| 20 | AVD/iOS simulator sample을 실기기 또는 C11 전체로 보고한다 | **PASS** — 플랫폼 제한과 미지원 scope를 acceptance 및 끝부분에서 명시한다. |

20개 항목을 실제 staged PR diff와 연결해 확인했다. 발견 결함은 없었다.
