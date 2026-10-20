# C11.4 음수 implicit track 계획 PR diff 적대 검토 30개

**검토 대상:** `plan/c11-4-negative-implicit-track-sizing-20261020`의 plan/evidence/fixture 변경. 계획 적대 검토 20개와는 별도의 PR 변경 검토다. 코드·runtime 검토와 구현 PR diff 검토도 별도로 새 관점을 사용한다.

| # | 변경 실패 관점 | exact diff에서 확인한 것 | 결과 |
|---:|---|---|---|
| 1 | 잘못된 파일 유입 | stage된 경로가 계획 2개, 별도 계획/PR 검토 기록 2개, precomparison 자료 7개, CSS fixture 1개로 제한된다. | 통과 |
| 2 | 제품 코드 조기 변경 | Rust/Android/iOS runtime 파일과 status/roadmap 파일이 이번 계획 PR에 섞이지 않는다. | 통과 |
| 3 | dirty 원본 checkout 오염 | 이 branch base는 `31a19e0a`; 원래 checkout의 미완성 작업 경로는 변경 대상이 아니다. | 통과 |
| 4 | 오래된 main 기준 | `git ls-remote origin refs/heads/main`이 branch base와 같은 `31a19e0abd88abb6622b7411a951ed833fc0d44c`다. | 통과 |
| 5 | 미추적/임시 캡처 파일 | `.raw.txt`, temp html, browser profile을 stage 목록에서 검색하고 모두 제외했다. | 통과 |
| 6 | fixture font 상대경로 | fixture 위치에서 `../../resources/c15/Ahem.ttf`가 저장소 고정 font를 가리키며 Agent Browser에서 Ahem load check가 true다. | 통과 |
| 7 | font fallback이 기대값을 오염 | environment JSON은 `ahemLoaded=true`; min/max-content Ahem sizes와 fixture font hash를 함께 보존한다. | 통과 |
| 8 | 관찰 환경 기록 부족 | Agent Browser 0.31.1, Chrome 155.0.8059.85, macOS 26.5.1 arm64, headless, viewport/DPR를 기록한다. | 통과 |
| 9 | screenshot과 observation 불일치 | screenshot은 같은 33-case fixture를 연 Agent Browser session에서 `screenshot --full`로 생성했다. | 통과 |
| 10 | root 밖 scene이 화면에서 사라짐 | 전체 screenshot의 fixed-root 가로·세로 overflow 행과 밖으로 나온 마지막 item을 눈으로 확인했다. | 통과 |
| 11 | fixture case 수 누락 | HTML case array, environment `caseCount`, observation row count, expectations count가 모두 33이다. | 통과 |
| 12 | case id 중복/누락 | validator가 id uniqueness와 각 기대 id의 presence를 확인하고 33/33으로 종료한다. | 통과 |
| 13 | observation의 축 혼합 | 모든 row에서 columns와 rows 값을 별도 JSON field로 보존하고 validator가 축별로 비교한다. | 통과 |
| 14 | single negative cycle off-by-one | one-leading-track case의 left-to-right expected size는 마지막 authored list item과 일치한다. | 통과 |
| 15 | multi-track cycle 오프셋 | 2–4개 negative tracks와 positive companion cases가 reverse prefix와 forward suffix를 각각 고정한다. | 통과 |
| 16 | zero-size track 제거 | zero-cycle expected array는 `0px` 위치와 track 수를 모두 보존한다. | 통과 |
| 17 | implicit-only grid의 명시 track 혼동 | `columns:none` case는 computed used columns를 관찰해 negative·positive sequence를 각각 고정한다. | 통과 |
| 18 | span·reversed placement 기대 누락 | boundary span, reversed lines, negative definite line에 닿는 span case가 별도 ID로 존재한다. | 통과 |
| 19 | hidden item이 extent를 넓힘 | hidden control은 display 상태와 visible computed track lists를 독립 검사한다. | 통과 |
| 20 | auto-flow 방향 경로 공백 | row/column 및 dense/sparse로 분류된 auto-placement case를 양 축에서 포함한다. | 통과 |
| 21 | dense와 sparse 결과가 같은 약한 oracle | 같은 negative-column 입력의 paired case에서 third item과 auto item의 row가 각각 24px과 1px로 갈린다. | 통과 |
| 22 | placement geometry가 page 위치에 종속 | validator는 item DOMRect에서 grid DOMRect를 빼 상대 좌표를 비교한다. | 통과 |
| 23 | browser fractional rounding | 좌표와 used tracks 모두 각 원소별 CSS px `1/64` 오차만 허용한다. | 통과 |
| 24 | overflow가 root sizing으로 가려짐 | root width/height 100px assertion 뒤 마지막 item edge가 각각 172px, 108px 초과하는지 검사한다. | 통과 |
| 25 | 기대값을 oracle에서 복사 | 기대 JSON에는 규범 설명과 수기로 적은 배열이 있고 observation에서 자동 생성하지 않는다. | 통과 |
| 26 | validator가 환경을 확인하지 않음 | Chrome 155 UA, DPR 1, Ahem true, case count 33을 먼저 assert한다. | 통과 |
| 27 | validator가 malformed track을 숫자로 삼킴 | `px` token 정규식, array length, per-item tolerance를 모두 검사한다. | 통과 |
| 28 | 로컬 실행 재현성 | validator는 Node built-in module만 사용하고 repo root에서 실행해 33/33 pass했다. | 통과 |
| 29 | SHA manifest stale/경로 오류 | plan, fixture, font, env, observation, screenshot, expectation, validator hash가 root 기준으로 저장되고 `shasum -c` 전체 통과했다. | 통과 |
| 30 | 문서 연결/PR scope 오해 | plan/positive plan, evidence README/fixture/plan review link를 확인했고, PR은 구현·완료 수를 주장하지 않는다. | 통과 |

## 검토 결과

30개 관점에서 unresolved diff defect가 없다. 최종 PR body는 기존 PR template를 따르고 `planning`·`area: docs` label을 사용한다. Chrome 155 화면은 branch의 PNG를 raw GitHub image URL로 본문에 연결한다. 이 검토표는 merge 전 현재 branch revision에 대해 한 번 더 재실행하며, 수정이 생기면 영향 관점을 다시 검사한다.
