# C11 Root HTML dense-placement 사전 비교

이 자료는 parsed Root HTML runtime 경로에 CSS Grid dense auto-placement를 연결하기 위한 계획 단계 oracle다. 고정 6개 장면에서 dense 배치와 sparse 대조군, definite-item 충돌 대조군을 비교한다.

## 캡처

- macOS의 Agent Browser CLI 0.31.1, HeadlessChrome 156.0.0.0, viewport 1280×633 CSS px, DPR 1.
- 브라우저에서 [fixture.html](fixture.html)을 열고 computed track 목록, `grid-auto-flow`, root frame 및 각 직접 자식 frame을 `getBoundingClientRect()`로 관측한 뒤 [chrome156-six-scenes.png](chrome156-six-scenes.png)을 캡처했다.
- 캡처 뒤 Agent Browser 세션을 닫았고 이 세션의 Agent Browser·Chrome 프로세스가 남지 않았음을 확인했다.
- 원본 관측값과 SHA-256은 [chrome156-observation.json](chrome156-observation.json)에 있다.

## 독립 기대 geometry

CSS Grid Level 1 §8.5의 auto-placement cursor 규칙을 적용했다. 두-track span item 쌍이 연속 행 또는 열의 처음 두 cell을 채운다. 마지막 한-cell item이 알고리즘 차이를 드러낸다. row dense는 root 기준 `(88, 0)`에 두고 sparse row 대조군은 `(88, 24)`에 둔다. column dense는 `(0, 56)`에 두고 sparse column 대조군은 `(48, 56)`에 둔다. 충돌 장면에서는 definite item이 앞서 첫 남은 cell을 점유한 뒤 이후 span과 작은 item을 배치한다. track 크기와 gap이 고정되어 기대 좌표는 intrinsic sizing이나 screenshot pixel에 의존하지 않는다.

Chrome 6개 관측값은 독립적으로 산출한 위치와 일치한다. 이 자료는 브라우저 기준 관측일 뿐 Spinon 동작, WPT 적합성 또는 제품 runtime 결과를 보여주지 않는다.

## 범위 경계

현재 Root HTML projection은 `row dense`와 `column dense`를 거부한다. 기존 `RuntimeBlockFlexGridAutoPlacementDenseV1`와 C11.2.3 matrix에는 별도 placement 구현이 있다. 이 사전 비교는 Root HTML 연결 계획의 기준을 제공한다. 현재 admission 계약을 넓히거나 연결이 동작함을 증명하지 않는다. Simulator 및 제품 검증은 구현 PR에서 진행한다.
