# C11.4 Root HTML `align-content:baseline` 계획 실패 관점 검토 20개

계획·고정 fixture·Agent Browser 관측·독립 기대값을 대조해 서로 다른 실패 경로를 검토했다. 계획 검토는 구현 또는 구현 PR 변경분 검토로 재사용하지 않는다.

| # | 실패 관점 | 확인·차단 방법 | 판정 |
|---:|---|---|---|
| 1 | 일반 Grid 전체가 baseline을 지원하는 것으로 범위를 넓힌다. | `RootHtmlAutoRowStretchV1` fixed profile만 목표로 명시하고 다른 profile 거부를 수용 관문에 둔다. | PASS |
| 2 | 부모 Flex/Grid baseline-sharing context가 없는 조건을 쓰지 않는다. | 단일 top-level Root HTML Grid와 block root 입력만 고정한다. | PASS |
| 3 | Grid `align-content`를 item의 `align-self` baseline과 혼동한다. | Grid tracks를 content-alignment subject로 분리하고 `align-items:normal`을 고정한다. | PASS |
| 4 | computed `baseline`의 CSSOM serialization을 빠뜨린다. | Browser observation과 projection input에서 exact `baseline`을 확인한다. | PASS |
| 5 | authored `first baseline`을 computed serialization으로 직접 허용한다. | CSSOM canonical serialization 규칙을 명시하고 computed `baseline`만 allowlist에 넣는다. | PASS |
| 6 | Taffy의 FirstBaseline mapping을 제품 baseline algorithm으로 과장한다. | mapping은 start fallback과 좌표가 같은 제한 profile에서만 재사용하며 nested baseline group은 제외한다. | PASS |
| 7 | baseline에서 implicit `auto` track를 stretch한다. | expected rows `[10,10,10]`과 stretch-stage 미실행을 별도 수용 기준으로 둔다. | PASS |
| 8 | explicit row 및 implicit text rows가 다른 높이를 만든다. | explicit 10px row와 10px line-height direct text `A`/`B`를 고정한다. | PASS |
| 9 | positive free space에서 baseline이 start와 다르게 배치된다. | Browser 관측과 독립 fallback 계산이 모두 `[0,10,20]`을 예상한다. | PASS |
| 10 | row-gap overflow에서 baseline fallback이 안전하지 않다. | `calc(120% - 2px)`의 46px gutter와 y `[0,56,112]`를 고정한다. | PASS |
| 11 | overflow control이 baseline fallback 차이를 검출하지 못한다. | `end`/`unsafe end`의 `[-82,-26,30]`를 별도 negative control로 둔다. | PASS |
| 12 | safe end control의 positive/overflow 계산이 섞인다. | positive `[10,20,30]`, overflow `[0,56,112]`를 독립 값으로 기록한다. | PASS |
| 13 | row-gap 계산 basis가 width나 viewport로 잘못 잡힌다. | Grid definite block-size 40px에 대해 `1.2×40−2=46px`를 산출한다. | PASS |
| 14 | computed grid tracks를 item-frame 좌표와 혼동한다. | `[10,10,10]` used rows와 item frame 네 값 각각을 검사한다. | PASS |
| 15 | 2-item Root HTML single-row legacy shape도 새 allowlist를 얻는다. | positional/distributed alignment와 마찬가지로 정확한 3-item shape에서만 통과시키고 2-item 회귀를 둔다. | PASS |
| 16 | `last baseline`을 FirstBaseline으로 조용히 변환한다. | last-baseline layout 표현이 없고 Chrome 157 관측도 `normal`로 돌아가므로 지원 범위에서 제외한다. | PASS |
| 17 | invalid `safe baseline`/`unsafe baseline`이 허용된다. | profile allowlist에서 modifier 조합을 거부하고 negative tests로 고정한다. | PASS |
| 18 | `place-content`로 justify 축까지 새 값이 흘러간다. | computed longhand별 검사와 justify-content fixed control을 유지한다. | PASS |
| 19 | 잘못된 입력이 partial frame/draw를 내보내거나 RuntimeSession을 오염시킨다. | atomic rejection, `align-content` 오류 귀속 및 same-session valid recovery를 확인한다. | PASS |
| 20 | fixture 성공을 public CSS·WPT·C11.4 완료로 확대 해석한다. | 내부 fixed-profile 계약, 미검증 범위, `0.1.0`, `78/214` 및 상위 미완료를 명시한다. | PASS |
