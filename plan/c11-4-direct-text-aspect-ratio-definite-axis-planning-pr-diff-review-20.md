# C11.4 definite-axis ratio 계획 PR diff 적대적 검토 20개

**대상:** definite-axis 계획, 별도 계획 검토표, Chrome 사전 비교 HTML·JSON·screenshot 및 evidence README.

**검토 방법:** 변경 파일을 현재 `main` source 및 서로 독립적인 fixture/observation/expected-value 자료와 대조했다. 계획의 기능 실패 관점 검토와 구별되는 이 PR diff만의 무결성·범위 확인이다.

**판정:** 변경 파일 diff 20/20 PASS. 코드를 구현하거나 시뮬레이터를 실행한 결과가 아니다.

| # | PR diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | Evidence README가 이전 fixture hash를 계속 가리킴 | PASS — HTML fixture SHA-256 `d06fd4d5…cb8b`, 3,499 bytes와 expected-values source를 대조했다. |
| 2 | Expected-values JSON source hash/bytes가 HTML과 다름 | PASS — JSON source fields가 현재 fixture SHA-256과 3,499 bytes를 가리킨다. |
| 3 | HTML은 `auto` row인데 expected input은 `max-content` row라고 기록 | PASS — HTML, JSON, README 모두 `[auto]` row로 맞췄다. |
| 4 | Chrome screenshot이 다른 viewport/DPR에서 찍힘 | PASS — observation은 1280×633, DPR 1; PNG metadata도 1280×633이다. |
| 5 | Agent Browser observation hash가 README와 다름 | PASS — JSON hash `6f736b09…f14ac`이 README에 일치한다. |
| 6 | Screenshot hash가 README와 다름 | PASS — PNG hash `f64131fb…a8e2`가 README에 일치한다. |
| 7 | Expected-values JSON hash가 README와 다름 | PASS — 최종 JSON hash `d65674d8…766b`로 맞췄다. |
| 8 | Ahem font가 로드되지 않은 fallback 결과를 사용 | PASS — observation의 `fontReady`와 브라우저 사전 비교 기록이 모두 true를 요구한다. |
| 9 | 화면상 item 좌표와 item-relative expected 좌표가 혼합 | PASS — observation은 root-relative item/text geometry, screenshot은 화면 capture로 분리된다. |
| 10 | Computed CSS `width`/`height`를 frame 결과로 잘못 대체 | PASS — JSON은 computed serialization, track sizes, border box, Range rect를 각각 저장한다. |
| 11 | Control scene의 ratio가 positive scene에 남음 | PASS — width/height controls 모두 computed `aspect-ratio:auto`이며 JSON 입력에도 반영됐다. |
| 12 | `width-ratio`에서 row expected 30px이 fixture geometry와 다름 | PASS — definite 60px / 2와 Chrome CSSOM `30px` row 및 60×30 item이 일치한다. |
| 13 | `height-ratio`에서 column expected 60px이 fixture geometry와 다름 | PASS — definite 30px × 2와 Chrome CSSOM 두 번째 column 60px 및 60×30 item이 일치한다. |
| 14 | Controls가 text intrinsic contribution을 분리하지 못함 | PASS — width control은 row10px, height control은 AAAA column40px을 유지한다. |
| 15 | Text Range를 item frame과 동일하게 가정 | PASS — 네 scene 모두 text Range 40×10과 item frame을 따로 보존한다. |
| 16 | Plan이 CSS Sizing Level 4 WD를 안정 표준처럼 부름 | PASS — 보조 근거를 Working Draft로 명시하고 일반 지원 주장을 금지한다. |
| 17 | Plan이 Chrome 관측을 Spinon/WPT pass로 확대 | PASS — browser oracle, implementation evidence, WPT를 명시적으로 구분한다. |
| 18 | Plan이 기존 C11.4 ratio 경로에 dimensions 허용을 넓힘 | PASS — 별도 opt-in profile을 요구하고 기존 auto-dimension 경로 회귀를 유지한다. |
| 19 | Plan이 unsupported CSS와 malformed ownership을 성공으로 축약 | PASS — computed/typed mismatch, wrong/multiple owner, stale revisions와 unsupported values를 fail closed 범위에 둔다. |
| 20 | PR이 계획만으로 공식 완료 숫자를 변경 | PASS — PR 변경은 planning/evidence이며 C11.4 checkbox와 `72/213`을 유지한다. |

## 검토 후 변경

PR diff 정합성 확인을 위해 HTML fixture, expected-values JSON, screenshot, Agent Browser JSON의 hash와 dimensions를 재계산해 문서 값과 비교했다. 계획 단계이므로 Rust tests, product build, Android/iOS simulator runtime은 실행하지 않았다.
