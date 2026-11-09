# C11.4 반복 텍스트 자동 배치와 fit-content 위치 사전 비교

**상태:** Chrome 독립 기준 관측과 CSS Grid 관계 검증 완료 · 제한 구현 전 · 2026-11-09

## 입력과 환경

- Fixture: [`root-html-grid-repeated-text-autoplacement-fit-content-positions-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-fit-content-positions-precomparison.html)
- Fixture SHA-256: `0829c8dc6d39a2cc20fea772479b732e17b44ee070555cef8e5c48a0e016a2af`
- 브라우저: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, macOS arm64, DPR 1
- Grid: definite `40×220px`, `40px` column 하나, `10px` explicit row 하나, gap 0, row auto-flow, `align-content:start|stretch`, visible overflow
- 자식: 빈 explicit-row item 뒤에 auto-placement item 여덟 개가 오며 각 item은 direct text payload 하나를 포함한다. Payload 줄 수는 `[2,2,2,3,2,2,3,1]`이다.
- Track list: `fit-content(cap)` 하나, `min-content ×2`, `max-content ×2`, `auto ×1`; 고유 tail permutation 30개; fit position 1–6; cap `20px`, `20%`, `0%`, `120%`.

첫 시도에서는 총 36줄 payload를 사용했다. implicit track이 definite Grid 높이를 크게 넘어 fit-content 위치 4–6이 모두 0으로 수렴했고, 해당 위치의 cap을 구별하지 못했다. 현재 fixture는 총 17줄 text를 사용하며 여섯 위치 모두 cap에 따라 track이 달라진다. 첫 시도는 최종 관측 corpus에 포함하지 않았다.

## 관측과 독립 판정

고정 matrix는 `6 positions × 30 tail orders × 4 caps × 2 alignments = 1,440` scenes이며 fit-content 각 위치마다 240 scene이다. 전체 [Chrome 관측값](./chrome157-matrix-observation.ndjson)은 metadata 한 줄과 scene별 한 줄을 둔 1.7 MB NDJSON이다. [검증 스크립트](./verify-chrome-matrix.py)는 관측값을 고정 입력의 CSS Grid 관계와 대조한다. 브라우저의 원시 측정값을 독립 알고리즘 oracle로 취급하지 않는다.

독립 검사는 다음 관계를 확인한다.

- 선언·계산된 여섯-member 순서와 정확한 multiplicity, 30개 고유 permutation.
- implicit-row cycle은 explicit row 뒤에서 시작해 여섯 행마다 반복한다. 여덟 implicit row에서는 위치 1·2가 두 번, 위치 3–6은 한 번씩 나타난다.
- 이 고정 폭 fixture에서 각 fit-content used row는 `10px × text 줄 수`와 cap 중 작은 값이다. definite 220px block-size에서 percentage cap은 44px, 0px, 264px로 계산된다. `min-height:0`은 track 최소값을 0으로 둔다.
- item의 top/height와 used row track의 일치, payload의 top·40px width·line-height geometry·측정 줄 수.
- `align-content:stretch`의 positive free space는 생성된 `auto` row에만 분배되며 다른 row는 변하지 않는다.
- Grid의 220px border box 아래로 나오는 visible text payload overflow.

1,440개 scene 구조, 1,920개 fit-content cycle 발생, 720개 `start`/`stretch` 쌍이 모두 통과했다. 위치 4에서는 3줄 기여를 확인한다. 해당 row는 cap `20px`에서 20px, `20%`/`120%`에서 30px, `0%`에서 0px다. 첫 cycle의 나머지 위치는 2줄 기여로 같은 cap에서 각각 20px, 20px, 0px가 된다. 위치 1·2의 반복 track은 implicit row 7·8도 확인한다. 54개 scene에서 text payload가 Grid 바깥으로 나왔고 최대 초과는 20px다.

재현 가능한 요약은 [matrix-verification.json](./matrix-verification.json)에 있다. [SHA-256 manifest](./SHA256SUMS)는 fixture, observation, verifier, 요약과 screenshot을 고정한다. [표본 화면](./chrome157-sample.png)은 시각적 참고이며 geometry oracle은 아니다.

이 자료는 브라우저 사전 비교에 한정된다. Spinon runtime 동작, native rendering, author CSS admission, WPT 적합성 또는 C11.4/C11 완료를 입증하지 않는다.
