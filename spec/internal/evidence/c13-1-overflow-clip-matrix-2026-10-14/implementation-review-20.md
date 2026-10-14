# C13.1 clip 축·margin 행렬 구현 검토 · 20개 · 2026-10-14

대상은 `clip-y`, `clip-margin-border`, `clip-margin-hidden` 세 행의 `RuntimeRenderSnapshot` clip geometry와 고정 Chrome hit range 비교 테스트다. 앱 surface 검증으로 확대하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | y-only clip이 x축까지 제한 | `clip-y`에서 paint와 hit x interval이 모두 `None`인지 확인한다. |
| 2 | y clip paint가 border edge까지 열림 | Chrome y=clip의 paint bound `88..176`을 고정한다. |
| 3 | y clip hit가 padding edge로 잘림 | Chrome positioned parent hit bound `84..180`을 별도 검사한다. |
| 4 | x축은 범위 끝 표본 하나만 일치 | target의 모든 정수 x query를 고정 y에서 순회한다. |
| 5 | y축은 범위 끝 표본 하나만 일치 | target의 모든 정수 y query를 고정 x에서 순회한다. |
| 6 | clip interval의 max를 inclusive로 착각 | Chrome 정수 hit 범위의 마지막 좌표와 바로 다음 좌표를 검사한다. |
| 7 | margin이 visual border box가 아닌 padding box 기준으로 계산 | `border-box 8px`의 x/y clip bound를 border edge에서 양방향 확장한 값으로 고정한다. |
| 8 | margin offset이 한쪽 방향에만 적용 | x와 y 각각 min/max 네 경계를 모두 assert한다. |
| 9 | positioned clip의 hit clip이 paint clip과 달라짐 | border-box case에서 둘의 실제 Chrome 관찰값이 동일한지 확인한다. |
| 10 | `hidden`에도 clip margin이 적용 | Chrome `hidden` hit 범위가 border-box+8 확장값까지 가지 않는 것을 대조한다. |
| 11 | `hidden` paint edge를 border edge로 둠 | 실제 runtime paint clip rect가 padding edge `420..532`, `424..512`인지 확인한다. |
| 12 | `hidden` hit edge를 padding edge로 둠 | hit-test clip은 border edge `416..536`, `420..516`인지 확인한다. |
| 13 | hidden의 paint/hit 차이를 하나의 비교값으로 합침 | `clip_css_px`와 `hit_test_clip_css_px`를 독립 assertion으로 둔다. |
| 14 | 대상 frame 위치가 clip 정답과 우연히 맞음 | 세 Chrome 행의 x/y/width/height/right/bottom을 observation에 고정하고 runtime frame 오차를 검사한다. |
| 15 | user-agent가 다른 Chrome 결과를 고정값으로 사용 | 테스트가 HeadlessChrome 155 UA와 제품 버전 문자열을 assertion한다. |
| 16 | 원본 fixture가 바뀐 뒤 과거 observation을 재사용 | 원본 HTML SHA-256을 테스트에서 확인한다. |
| 17 | 정수 좌표 범위를 반올림으로 축소 | 시작은 `floor(min)`, 끝은 `ceil(max)-1`로 browser collector와 동일하게 계산한다. |
| 18 | viewport 밖 좌표가 `None` 때문에 거짓 통과 | 1280×633 viewport에서 Chrome 대상 frame 전체 좌표를 조회한다. |
| 19 | 비대상 node의 fallback hit ID까지 검증했다고 주장 | README에 target hit 여부만 검사한다고 한계를 기록한다. |
| 20 | geometry/hit 테스트를 surface pixel 또는 WPT 통과로 홍보 | evidence와 테스트 설명에서 앱 실행·pixel·DOM event·WPT는 미실행으로 분리한다. |

검증 중 테스트 helper가 accessor의 값 반환/참조 타입을 혼동해 생긴 컴파일 오류를 수정했다. 이후 세 matrix test가 통과했다. 제품 동작의 별도 실패를 재현한 것으로 보고하지 않는다.
