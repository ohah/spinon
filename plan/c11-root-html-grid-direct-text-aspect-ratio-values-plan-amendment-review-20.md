# C11.1 aspect-ratio 값 확장 계획 보정 검토 20개

기존 계획의 Chrome precomparison에는 height-locked `aspect-ratio:1/2; height:80px` 장면이 있지만 최초 size admission 서술은 `height:30px`만 명시했다. 이 검토는 제품 코드 변경을 재개하기 전에 그 불일치를 보정한 계획 amendment를 공격적으로 점검한다.

| # | amendment 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 사전 oracle에 없던 height 80px을 구현 범위에 몰래 추가 | 통과 — 이미 고정된 `height-half` Chrome scene이 그 입력과 frame을 가진다. |
| 2 | 높이 80px이 임의의 새 fixed-size profile로 확장 | 통과 — `height:80px`은 typed ratio가 0.5일 때만 허용한다. |
| 3 | 높이 30px 기존 값 행렬의 판정이 바뀜 | 통과 — 4/3·3/2·5/2·2/1은 그대로 `height:30px`이다. |
| 4 | height 30px의 1/2 transfer가 15px으로 단언되어 min-content clamp를 숨김 | 통과 — 해당 ratio는 높이 80px scene에만 둔다. |
| 5 | height 80px 장면의 transferred width 40px과 Ahem minimum 관계가 불명확 | 통과 — transfer 40px은 Rust 직접 등록 Ahem minimum과 일치한다고 plan에 명시한다. |
| 6 | simulator의 system monospace 폭을 Ahem과 동일하다고 가정 | 통과 — 앱 font는 플랫폼별로 기록하고 비교에서 제외한다. |
| 7 | 특별한 80px 조건이 computed CSS serialization parse를 요구 | 통과 — 조건은 typed ratio 0.5를 사용하고 serialization 재파싱은 계속 금지한다. |
| 8 | ratio 1/2의 float representation이 조건 비교에서 달라짐 | 통과 — cascade의 typed f32 ratio 0.5는 정확히 표현 가능하며 size transfer는 finite-positive 검사 대상이다. |
| 9 | height-locked `4/3` 또는 다른 ratio에서 80px을 실수로 받음 | 통과 — pair+ratio 정확한 교차 조합 외에는 negative control로 거부한다. |
| 10 | width-locked size pair까지 새 높이 예외의 영향 | 통과 — width 60px/height auto pair와 typed ratio matrix는 독립 유지한다. |
| 11 | two-column/one-auto-row topology 확장과 혼합 | 통과 — root width, tracks, row, gap, flow, alignment 조건은 그대로다. |
| 12 | owner당 ratio item 수 또는 다른 owner 격리가 변함 | 통과 — owner-scoped one item invariant는 amendment 전후 동일하다. |
| 13 | 80px ratio scene이 overflow/min-max 경계를 바꿈 | 통과 — zero edges, auto min/none max와 visible overflow 경계를 유지한다. |
| 14 | overflow 5/2 scene이 height 예외에 의존 | 통과 — 75×30과 Grid 바깥 15px은 기존 height30 scene이다. |
| 15 | Chrome expected evidence가 바뀌었지만 observation/fixture는 오래됨 | 통과 — 이미 기록된 14-scene input과 expected/observed frame을 재사용한다. |
| 16 | 새로운 계획이 14개 Rust matrix 수락 기준과 다름 | 통과 — implementation matrix는 oracle의 80px/0.5 scene을 동일 입력으로 포함한다. |
| 17 | Rust frame rounding이 exact equality로 잘못 검사 | 통과 — f32 ratio geometry tolerance 0.01 CSS px는 기존 계획에서 유지된다. |
| 18 | runtime fixture가 default Root HTML route 밖으로 전환 | 통과 — Android/iOS simulator acceptance는 같은 source hash의 기본 route로 유지한다. |
| 19 | amendment만으로 C11.1/C11 또는 공식 완료 수가 증가 | 통과 — plan/status와 public API 완료 상태를 변경하지 않는다. |
| 20 | plan correction 이후 이전 계획 검토를 재사용하며 새 실패 관점 없음 | 통과 — 이 amendment 전용 독립 20개를 추가했고 구현/PR 검토와 구분한다. |

**판정:** 명시된 한 height/ratio 조합만 추가하면 고정 Chrome matrix와 계획의 size admission이 일치한다. 제품 코드를 재개할 수 있다.
