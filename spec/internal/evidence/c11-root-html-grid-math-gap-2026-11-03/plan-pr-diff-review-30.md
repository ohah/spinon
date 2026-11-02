# C11 parsed Root HTML Grid typed math gap 계획 PR diff 적대 검토 30개

검토 대상은 계획 PR의 실제 변경분이다: 2개 HTML fixture, Chrome 156 관측 JSON/화면, 사전 비교 README, 계획 본문 및 계획 20개 리뷰. 계획 20개와는 별도 관점으로, 문서 간 참조·입력 구조·oracle 무결성·후속 실행 제약을 살폈다.

| # | PR diff 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 계획 링크가 잘못되어 사전 비교 근거로 이동할 수 없음 | 통과 — plan의 `../spec/internal/evidence/.../README.md` target이 존재한다. |
| 2 | plan의 runtime fixture 상대 링크가 저장소 밖을 가리킴 | 통과 — `../tests/fixtures/css/c11/grid-math-gap-runtime-v1.html` target이 존재한다. |
| 3 | evidence README에서 multi-scene fixture 링크가 끊김 | 통과 — `../../../../tests/fixtures/css/c11/grid-math-gap-v1.html`이 fixture로 해석된다. |
| 4 | plan review가 본문 계획이 아닌 다른 작업 문서에 연결 | 통과 — 같은 plan directory의 typed math gap 계획을 연결한다. |
| 5 | multi-scene fixture ID 중복으로 observation case를 특정할 수 없음 | 통과 — CSS selector/report가 각 scene id를 key로 삼고 19개 ID가 유일하다. |
| 6 | 5%/10% expected 계산이 width와 height basis를 섞음 | 통과 — fixture는 200px column basis와 100px row basis를 분리한다. |
| 7 | base scene의 row/column 트랙 합이 fixed root에 맞지 않아 gap만 분리되지 않음 | 통과 — 10px 두 트랙과 definite box에서 second item origin이 트랙+gap을 직접 드러낸다. |
| 8 | pure px expression이 percentage basis에 영향을 받도록 잘못 지정 | 통과 — `calc-absolute`는 pure length 12px control이고 plan은 gap-only basis 0 규칙을 요구한다. |
| 9 | zero width scene이 실제 definite 0 basis가 아님 | 통과 — root width와 두 columns가 0px이고 Chrome second item x=2는 절대 성분만 남긴다. |
| 10 | zero-basis의 `75%+2px` 계산이 fixture/result에서 불일치 | 통과 — raw observation은 `calc(75% + 2px)`와 x=0/2, independent arithmetic은 2px다. |
| 11 | `min()`/`max()` 결과가 percentage basis가 바뀌어도 같은 수치로 우연히 나옴 | 통과 — 폭 200px의 15%=30px와 20px bound가 max/min을 서로 다른 20/30px로 판별한다. |
| 12 | clamp reversed-order semantics가 예시 입력과 관측에 반영되지 않음 | 통과 — `clamp(30px, 5%+2px, 10px)`은 Chrome x=40이고 계획은 lower bound 우선을 명시한다. |
| 13 | `calc(5%*2)`가 percentage 곱셈보다 다른 단위 산술을 테스트 | 통과 — Chrome computed `10%`, width 200px의 second item x=30가 독립 20px gap과 일치한다. |
| 14 | divide control이 number/length dimension rule을 우회 | 통과 — `24px/2`의 computed `12px`와 x=22를 따로 보존한다. |
| 15 | negative computed expression을 invalid literal gap과 혼동 | 통과 — fixture는 valid `calc(2px-5%)`를 쓰고 기존 literal-negative 회귀를 별도 requirement로 남긴다. |
| 16 | >100% math gap에 overflow가 가려져 percentage clamping을 관찰할 수 없음 | 통과 — x=131 및 scroll width141은 121px gap 사용을 드러낸다. |
| 17 | Infinity stress 결과를 Chrome의 exact normal geometry parity라고 오해 | 통과 — README가 track collapse/coordinate clip을 기록하고 normal-range assertion에서 제외한다. |
| 18 | NaN stress가 root item origin으로 0 clamp를 판별하지 못함 | 통과 — computed `0px`와 second item x=10을 함께 보존한다. |
| 19 | implicit-row case에서 explicit row만 있고 implicit track이 없음 | 통과 — 4개 아이템, 2개 explicit columns와 25% auto row로 두 번째 row를 생성한다. |
| 20 | implicit-row gap과 implicit track 크기를 혼동 | 통과 — gap 7px, track 25px, 다음 row y=27px를 각각 계산한다. |
| 21 | column-flow item이 두 번째 column에 진입하지 않아 implicit gap이 검증되지 않음 | 통과 — 세 아이템, 두 explicit rows로 third item이 implicit column x=52에 놓인다. |
| 22 | named-area span이 gap을 두 번 넣거나 누락해도 결과가 같음 | 통과 — header width128, lower side/content x/y가 track span과 gutter를 구분한다. |
| 23 | multi-scene browser oracle을 단일-area 제품 입력에 사용 | 통과 — 별도 runtime fixture로 분리하고 named-area 단일 Grid gate와 충돌하지 않게 했다. |
| 24 | runtime fixture에 JavaScript가 있어 product HTML 실행 경로를 불필요하게 바꿈 | 통과 — runtime fixture는 static HTML/CSS이며 report는 Agent Browser eval로 외부에서 수집했다. |
| 25 | runtime fixture의 Chrome observation hash가 다른 입력에 대응 | 통과 — observation JSON의 geometry는 static runtime fixture의 2×2 scene과 일치하고 hash를 README에 결합했다. |
| 26 | Android/iOS간 플랫폼 evidence가 다른 HTML 입력을 쓸 여지 | 통과 — plan은 두 simulator에 동일한 단일 fixture bytes/hash를 사용하도록 명시한다. |
| 27 | Chrome screenshot을 layout oracle 단독 증거로 취급 | 통과 — exact 수치는 JSON observation 및 independent arithmetic에 있고 screenshot은 보조 관측이다. |
| 28 | screenshot을 native surface readback 또는 post-compositor equality로 과장 | 통과 — plan은 draw/present와 screenshot을 시각 smoke로만 분류한다. |
| 29 | fixture/report hash가 README에 적힌 값과 다름 | 통과 — SHA-256 재계산이 multi-scene 및 runtime fixture/JSON/PNG의 기록과 일치한다. |
| 30 | 계획 PR이 기능 구현 또는 공식 완료 수치 변경처럼 보임 | 통과 — status ledger는 건드리지 않았고 plan은 구현 미완료, C11.1/C11 unchecked, 72/213 유지와 version 0.1.0을 명시한다. |

결론: 이 계획 PR 변경분에서 별도 수정이 필요한 차단 결함은 발견하지 못했다. 다음 구현 리뷰는 이 30개 PR 문서 검토를 재사용하지 않는다.
