# C11.4 preserved-tab 계획 PR diff 적대 검토 30개

**대상 변경:** 계획, 계획 실패 관점 20개, Agent Browser 사전 비교 fixture 및 evidence. 구현 코드 변경은 없다. 계획 검토 표는 [별도 기록](./c11-4-anonymous-grid-text-pre-tabs-plan-review-20.md)이며 여기서는 PR에 제출할 파일 전체의 정확성·연결·범위 경계를 새 관점으로 검사한다.

| # | 적대적 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | 기존 C11.4/C11 ID를 완료 처리하는 계획인가 | 공식 체크/72/213 유지가 명시되어 있다 — PASS |
| 2 | fixture·JSON·PNG 링크가 문서 build에서 raw file 대신 존재하지 않는 `.json.html` route로 바뀌는가 | 첫 docs build가 신규 README의 상대 asset links를 잡았다. fixture와 raw evidence link를 GitHub blob URL로 바꿨다. 재실행 build에서 이번 변경 경로의 dead-link 항목은 0개다 — PASS |
| 3 | CSS Text Level 3 링크가 목표 §4.2를 가리키는가 | `#tab-size-property` URL과 captured page 제목을 확인했다 — PASS |
| 4 | preserved-tab 배치 근거가 block ancestor origin을 보존하는가 | 규범 문구를 기록하고 fixture x=0 동치 및 offset 제외를 밝혔다 — PASS |
| 5 | `tab-size` 기본값 8의 근거가 있는가 | Agent Browser로 규범 초기값과 computed value를 관찰했다 — PASS |
| 6 | 0.5ch exact-stop 예외를 놓쳤는가 | 계획과 `XXXXXXXX\tX` 장면이 다음 stop을 요구한다 — PASS |
| 7 | W3C 요구를 Chrome 결과만으로 정의했는가 | 독립 10px/80px 계산과 Chrome cross-check를 분리했다 — PASS |
| 8 | Chrome screenshot을 Spinon 결과로 오인하는가 | README와 계획은 oracle로만 분류한다 — PASS |
| 9 | 브라우저/font 환경이 재현 가능한가 | HeadlessChrome 156, Ahem hash, font-loaded 상태를 보존했다 — PASS |
| 10 | fixture source hash가 README와 같은가 | HTML SHA-256을 다시 계산해 기록값과 일치했다 — PASS |
| 11 | fixture의 tab literals가 escape text로 잘못 기록됐는가 | 실제 U+0009와 forced newline이 source에 있으며 observation text에도 반영됐다 — PASS |
| 12 | Ahem load 실패가 유효한 관측으로 통과할 수 있는가 | browser receipt에서 `document.fonts.check`가 true다 — PASS |
| 13 | default stop after two columns 계산이 빠졌는가 | Chrome/independent expected track 100px로 고정했다 — PASS |
| 14 | leading tab이 trim되는 경로를 놓쳤는가 | 90px root/line 기대값과 별도 ID가 있다 — PASS |
| 15 | consecutive tabs가 같은 stop에 겹칠 수 있는가 | 80px 다음 160px, 총 170px로 계산했다 — PASS |
| 16 | line break 뒤 이전 줄 tab cursor를 물려받을 수 있는가 | 100px/90px 두 line과 20px row를 따로 기록했다 — PASS |
| 17 | trailing tab intrinsic contribution이 사라지는가 | 80px line/track과 preserved endpoint를 포함했다 — PASS |
| 18 | min-content가 `pre`에서 soft-wrap하는가 | 100px min-content track 한 줄 oracle가 있다 — PASS |
| 19 | 25px track 안에서 text overflow를 root 폭과 혼동하는가 | root/track 25px, Range line 100px를 분리해 기록했다 — PASS |
| 20 | `tab-size:2` oracle를 지원 장면으로 오인하는가 | 60px 관측은 oracle-only 음성 경계로 분류됐다 — PASS |
| 21 | plan이 custom value를 동작 API로 몰래 추가하는가 | runtime는 computed 8만 받고 number/length 일반 지원을 제외한다 — PASS |
| 22 | runtime이 custom tab-size를 놓치고 8로 처리할 수 있는가 | computed longhand 노출 및 2 거부를 구현 관문에 포함했다 — PASS |
| 23 | normalizer 탭 cursor가 paragraph text node 경계에서 초기화되는가 | 여러 TextSource의 provenance/cursor 연속 요구를 계획했다 — PASS |
| 24 | expanded spaces가 원본 tab provenance를 잃는가 | 한 UTF-16 code unit과 전체 output byte range 대응을 명시했다 — PASS |
| 25 | CRLF가 이중 줄바꿈 또는 provenance 단일화가 되는가 | 한 forced break, 두 source range 테스트를 계획했다 — PASS |
| 26 | tab expansion으로 기존 `PreWrap` 동작이 바뀌는가 | 기존 PreWrap code path와 fixtures를 보존하는 조건을 썼다 — PASS |
| 27 | profile mismatch에서 partial intrinsic/final result가 노출되는가 | 동일 error/fail-closed 요구를 두 API 모두에 적었다 — PASS |
| 28 | scene count가 계산과 맞는가 | existing 31 + 8 success + 1 negative = 40으로 산술 확인했다 — PASS |
| 29 | product artifact 또는 platform proof 범위를 과장하는가 | archive 경계, AVD/Simulator 범위와 glyph/compositor 한계를 구분했다 — PASS |
| 30 | 제출 뒤 미검토 파일이나 별도 변경이 섞일 수 있는가 | 최종 PR path set·head patch를 로컬 staged review와 재대조하도록 merge gate에 넣었다 — PASS |

## 결론

계획 PR diff는 구현 전 기준, 제한 profile, 재현 가능한 Chrome oracle, 계획 리뷰 및 이후 구현/PR 검증 관문을 분리한다. 구현·simulator 검증·CI·공식 완료 상태를 증명하지 않는다. 병합 전 PR의 최종 head가 위에서 읽은 변경 파일과 일치하는지 다시 검사한다.

문서 build는 수정 전 새 evidence README의 JSON/PNG/fixture 상대 링크를 잡아 실패했다. 해당 링크를 GitHub blob 경로로 고친 후 재실행했을 때 이번 변경 파일에서는 dead link가 나오지 않았다. 전체 build는 저장소의 다른 문서에서 발견된 1,116개 dead-link 페이지 때문에 여전히 exit 1이다. 이 PR은 전체 docs build 성공을 주장하지 않는다.
