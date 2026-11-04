# C11.1 Root HTML direct-text aspect-ratio 값 확장 계획 PR diff 검토 20개

2026-11-05 plan PR 후보의 전체 staged diff를 검토했다. 이 PR은 plan, Chrome precomparison fixture/evidence 및 리뷰 기록만 추가하며 제품 코드나 구현 상태 대장을 변경하지 않는다.

| # | PR diff에서 확인할 실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | 문서 제목·본문이 C11 전체 완료로 오해될 수 있음 | 통과 — 제목과 목표를 C11.1 parsed Root HTML direct-text 제한 slice로 표현한다. |
| 2 | 기존 PR #1161의 `2/1` 시작 경계가 확인되지 않음 | 통과 — plan은 병합된 narrow slice와 해당 코드 gate를 시작점으로 지목한다. |
| 3 | HTML fixture가 설명된 14개 장면과 다름 | 통과 — fixture, expected JSON, Chrome observation에 14개 scene ID가 같은 순서로 있다. |
| 4 | precomparison 입력을 나중에 바꿔 oracle을 재현할 수 없음 | 통과 — fixture SHA-256이 README와 observation JSON에 기록되고 manifest에서 검증된다. |
| 5 | Agent Browser 관찰이 터미널 브라우저나 다른 Chrome 입력에서 나옴 | 통과 — evidence는 Agent Browser 0.31.1/HeadlessChrome 156.0.0.0와 고정 viewport를 기록한다. |
| 6 | full screenshot이 모든 scene 또는 overflow boundary를 포함하지 않음 | 통과 — 1280×853 전체 화면에 14개 scene이 있고 5/2 target이 Grid 끝 밖으로 보인다. |
| 7 | ratio numerator/denominator 방향이 기대값과 반대 | 통과 — width/height 정의와 양 축 변환 기대값이 모두 plan에 있다. |
| 8 | 단일값 `1.5`와 pair `3/2`의 CSSOM 문자열을 잘못 동일시 | 통과 — 관찰은 `1.5 / 1`과 `3 / 2`를 분리하고 typed ratio/frame 동등성만 비교한다. |
| 9 | 10/1 width-locked 결과를 transferred 6px로만 판정 | 통과 — transferred 6px와 auto minimum 10px을 분리해 expected와 plan에 적었다. |
| 10 | 1/2 height scene이 intrinsic minimum에 의해 기대값과 달라짐 | 통과 — height 80px로 fixture를 고정하고 Rust Ahem minimum과 연결한다. |
| 11 | 시스템 font intrinsic 폭을 simulator 간 정답으로 사용 | 통과 — auto control intrinsic 폭은 플랫폼 간 exact 비교에서 제외한다. |
| 12 | height-locked 5/2를 70px Grid에 맞춰 잘라냄 | 통과 — 75px frame, 오른쪽 15px overflow, visible overflow를 분리해 관찰한다. |
| 13 | 기존 2/1 동작 또는 `auto` baseline을 빠뜨림 | 통과 — 두 축 기존 회귀와 양 축 auto control이 독립 scene이다. |
| 14 | f32 ratio arithmetic에 비현실적인 exact-equality 요구 | 통과 — Rust ratio-derived geometry는 0.01 CSS px, simulator node geometry는 0.5 CSS px 허용으로 구분한다. |
| 15 | finite ratio가 transfer 결과를 inf/0으로 만들어 partial layout을 반환 | 통과 — 표현 불가능한 전이는 전체 layout 실패로 닫도록 plan에 요구한다. |
| 16 | `auto <ratio>`, degenerate, constraint 값이 성공 경로에 섞임 | 통과 — `ContentBox`, `0/1`, min/max 및 typed-state 오류를 negative boundary로 둔다. |
| 17 | projection이 CSS serialization을 다시 파싱해 typed cascade를 우회 | 통과 — typed ratio만 sizing source로 사용하고 serialization은 numeric-present/diagnostic 확인에 제한한다. |
| 18 | 기본 Root HTML candidate gate가 다른 profile/topology까지 넓어짐 | 통과 — root/item identity, fixed tracks, item 수와 direct HTML text 경계를 유지한다. |
| 19 | simulator draw/present를 NodeId frame 또는 surface pixel로 확대 주장 | 통과 — 계획이 실제 측정 경계를 기록하고 관측하지 않은 frame/readback 주장을 금지한다. |
| 20 | 계획 PR에서 공식 status/완료 수를 부당하게 앞당기거나 검증을 과장 | 통과 — STATUS와 완료 수는 유지하며 simulator/runtime 실행은 구현 PR의 미완료 수락 기준으로 남긴다. |

**판정:** 모든 변경 파일과 독립 근거를 20개 별도 관점으로 검토했다. 계획 PR은 구현 승인용 사양·oracle 준비만 다루며 제품 기능 완료 판정은 포함하지 않는다.
