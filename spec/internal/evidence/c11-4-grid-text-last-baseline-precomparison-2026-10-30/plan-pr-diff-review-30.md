# C11.4 multiline last-baseline 계획 PR diff 적대 검토 30개

**대상:** plan branch에서 GitHub PR로 보낼 계획 문서, 20개 계획 검토, fixture, 독립 기대값, Agent Browser observation/screenshot, 비교 도구. 제품 구현 diff 검토는 아니다.

| # | PR diff를 깨뜨릴 수 있는 문제 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 새 계획의 상위 CSS Grid 상태 링크가 잘못된다. | `spec/STATUS.md#c11-css-grid` section과 경로를 확인했다. | PASS |
| 2 | CSS Grid 규범 링크가 다른 publication/version을 가리킨다. | 계획과 README가 동일한 2025 CRD §11.5 URL을 사용한다. | PASS |
| 3 | CSS Box Alignment source link가 last-baseline 규칙이 아닌 부분을 가리킨다. | README가 `#baseline-rules` 링크를 제공한다. | PASS |
| 4 | 구현 계획 링크가 존재하지 않는다. | `plan/c11-4-grid-track-sizing.md` 파일을 확인했다. | PASS |
| 5 | 내부 계약 링크가 repository root-relative path로 잘못 계산된다. | 계획에서 `../spec/internal/0059-c11-grid-layout.md`가 실제 파일로 해소된다. | PASS |
| 6 | 계획의 fixture 링크가 잘못된다. | `tests/fixtures/css/c11/grid-last-baseline-multiline-direct-text.html`가 존재한다. | PASS |
| 7 | README의 fixture link가 evidence directory 깊이에 맞지 않는다. | 네 단계 상위에서 repository root로 도달하며 `readlink -f` 결과가 fixture다. | PASS |
| 8 | README의 font link가 잘못되거나 fixture URL이 다른 bytes를 사용한다. | `tests/fixtures/resources/c15/Ahem.ttf`와 fixture 상대 URL을 확인했다. | PASS |
| 9 | README가 예상값 파일을 연결하지 않는다. | `expected-values.json`의 유효 JSON과 link를 확인했다. | PASS |
| 10 | 계획 검토 링크가 없거나 계획 review가 20개가 아니다. | `plan/c11-4-grid-text-last-baseline-plan-review-20.md`에 1–20의 별도 실패 관점이 있다. | PASS |
| 11 | Browser observation과 규범 기대값이 같은 JSON에 섞여 있다. | expected values와 Chrome observation이 서로 다른 파일이며 verifier도 별도 읽기만 한다. | PASS |
| 12 | Chrome metadata가 fixture font 미로딩을 놓친다. | observation에 font10/font20 true와 HeadlessChrome 156 UA가 있고 verifier에서 확인한다. | PASS |
| 13 | fixture가 parent auto-placement 영향으로 reverse scene을 바꾼다. | 두 item의 row와 semantic column이 explicit이며 block wrapper의 단순 context를 기록했다. | PASS |
| 14 | item semantic ID가 plan/expected/browser 간 불일치한다. | verifier는 각 semantic `small`/`large` suffix를 실제 ID와 매칭한다. | PASS |
| 15 | CSS columns, gap, width 합계가 모순된다. | `25 + 8 + 40 = 73px`; expected JSON, fixture와 Chrome observation이 일치한다. | PASS |
| 16 | multiline text가 한 줄이거나 3줄 이상으로 놓인다. | 고정 Ahem font, 25px/40px track과 `XX XX`/`XX` 입력에서 browser line rect 수 2/1을 verifier가 검사한다. | PASS |
| 17 | first/last line box top을 item-relative라고 잘못 더해 baseline을 두 번 이동한다. | verifier는 browser line rect를 root-relative로 사용하고 first/last Ahem offset을 더한다. | PASS |
| 18 | baseline offset의 font metric 계산이 기대값과 다르다. | Ahem ascent 800/UPEM 1000, 0.8em ratio와 fixed font hash를 expected JSON에 둔다. | PASS |
| 19 | last-baseline이 first-baseline처럼 start shim을 쓰도록 계획에 잘못 고정된다. | Grid §11.5 end-side shim 및 end fallback으로 end shim small 2px/large 0px, item y 0/2px, common baseline 18px 및 row 22px를 명시한다. | PASS |
| 20 | `first baseline` 회귀 control이 last와 구분되지 않거나 one-line 기존 계약을 넓힌다. | first control은 한 줄 text, group 16px, item y=8/0px, row 20px다. | PASS |
| 21 | `start` control이 baseline alignment를 우연히 상속한다. | start scene은 item y=0/0px, row 20px로 Chrome 및 expected JSON에 있다. | PASS |
| 22 | align-self scene이 부모 alignment를 검사하지 않는다. | expected/browser scene은 parent start와 두 child last-baseline 설정을 고정한다. | PASS |
| 23 | source order 역전이 DOM semantic placement까지 바꾼다. | reverse scene은 same semantic x/y/frame 및 group baseline을 기대하며 verifier가 각 semantic ID를 비교한다. | PASS |
| 24 | page offset 또는 case margin이 비교에 포함된다. | verifier는 각 scene root와 item frame, line rect만 비교하고 page y offset은 규범값으로 취급하지 않는다. | PASS |
| 25 | 구현이 없는 plan PR을 runtime proof로 읽게 된다. | 계획, README, review는 현재 증거가 Chrome-only이며 Spinon runtime 미검증임을 명시한다. | PASS |
| 26 | fixture HTML이 public author CSS 또는 계약 변경처럼 보인다. | fixture 경로는 CSS reference fixture이며 계획이 opt-in runtime-only 및 no public API를 명시한다. | PASS |
| 27 | 계획에 없는 simulator/WPT/full support를 PR 설명에 주장한다. | 구현 검증 섹션은 future gates이며 현재 C11.4/C11 completion을 배제한다. | PASS |
| 28 | 계획 후보가 구현 완료처럼 기록되거나 공식 완료 count가 바뀐다. | STATUS에는 Chrome-only plan candidate와 미구현 경계만 추가됐고 C11.4 checkbox 및 `72/213`은 유지된다. 내부 계약 `0.1.0`도 바뀌지 않았다. | PASS |
| 29 | 파일 bytes가 문서의 hashes와 다르다. | Fixture, Ahem, expected JSON, observation, screenshot과 verifier hashes를 재계산했다. | PASS |
| 30 | 계획 review 자체를 actual PR diff review와 같은 증거로 센다. | 이 30개 diff review는 20개 계획 검토와 별도 파일이며 제품 구현/runtime 검토를 대체하지 않는다고 적는다. | PASS |

## 결과

비교 스크립트 실행 결과 `PASS 5 scenes`였다. README의 evidence links, fixture/font/expected/observation/screenshot SHA-256, Chrome version/font readiness, scope 한계를 서로 대조했다. 이 검토는 계획 PR diff에만 해당하며 구현 PR에는 새 20개 implementation/runtime review와 별도의 PR diff 30개가 필요하다.
