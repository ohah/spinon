# C11.4 anonymous Grid text `pre-wrap` 계획 PR diff 검토 30개

**검토일:** 2026-10-31
**대상:** 계획 후보의 staged diff. 이 검토 기록 파일 자체는 대상 diff에서 제외했다.
**판정 범위:** 계획·로드맵 상태·Chrome 사전 비교 fixture와 증거가 서로 모순되거나 일반 지원 완료를 과장하는 실패 관점을 검토한다. 구현 동작 검증은 아니다.

| # | 독립 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | `spec/STATUS.md`가 C11.4 또는 C11 checkbox를 완료로 바꾸거나 공식 집계를 올리는가? | PASS — checkbox와 `72/213`을 유지한다. |
| 2 | 계획 문구가 fixture candidate를 일반 CSS 지원 또는 C11.4 완료라고 주장하는가? | PASS — 제한 fixture slice와 미완료 범위를 반복해서 구분한다. |
| 3 | pre-wrap 계획 PR 병합 전 구현이 시작된 것처럼 기록하는가? | PASS — 계획 PR 뒤 별도 구현 PR에서 시작한다고 명시한다. |
| 4 | 계획의 12개 Chrome scene과 독립 기대값 scene ID 집합이 불일치하는가? | PASS — fixture, expected JSON, observation JSON ID 집합은 일치한다고 evidence에 기록됐다. |
| 5 | Chrome 관측을 독립 CSS 기대값인 것처럼 섞어 적는가? | PASS — `expected-values.json`과 Agent Browser observation JSON을 분리한다. |
| 6 | screenshot 또는 Range rectangle을 glyph pixel/GPU 표면 증거라고 과장하는가? | PASS — Range는 fragment 위치이며 screenshot의 증명 범위를 한정한다. |
| 7 | 브라우저 종류·버전·viewport·DPR·font readiness 근거가 누락되는가? | PASS — Agent Browser 0.31.1, HeadlessChrome 156, viewport/DPR와 Ahem readiness를 기록한다. |
| 8 | scene source가 바뀌어도 source hash 증거를 추적할 수 없는가? | PASS — fixture source hash가 관측 JSON에 연결되고 README가 해당 증거를 안내한다. |
| 9 | 이번 변경이 Rspress에 새 dead link를 추가하는가? | PASS — 새 source/JSON/image 참조와 status의 plan 참조는 candidate branch GitHub URL을 사용한다. |
| 10 | `pre-wrap`이 `break-spaces`와 같다고 잘못 설명하는가? | PASS — 줄 끝 preserved-space hanging과 두 모드 차이를 명세 참조와 함께 구분한다. |
| 11 | `normal` 공백 축약 max-content 기준값 50px이 틀렸는가? | PASS — Ahem 10px 기준 `XX  XX`를 50px로 계산한다. |
| 12 | `pre-wrap` 공백 보존 max-content 기준값 60px이 틀렸는가? | PASS — 두 source U+0020을 보존한 60px 기대값을 둔다. |
| 13 | normal/pre-wrap min-content 대조가 soft-wrap opportunity를 놓치는가? | PASS — 두 단어 경계 기준 column 20px, 두 줄 결과를 별도 고정한다. |
| 14 | 25px soft-wrap 장면에서 줄 끝 공백의 hanging을 track contribution에 잘못 더하는가? | PASS — 25×20 결과와 fragment 관측의 한계를 별도 기록한다. |
| 15 | LF forced break 장면이 soft wrap과 혼동되거나 제거되는가? | PASS — `XX\nXX`를 max-content 20×20 두 줄 장면으로 분리한다. |
| 16 | comment가 나눈 두 DOM Text node를 두 anonymous Grid item으로 잘못 취급하는가? | PASS — 단일 anonymous item과 node별 source provenance 요구를 함께 둔다. |
| 17 | whitespace-only direct text가 anonymous item 또는 paragraph로 생성되는가? | PASS — Grid §6 기준으로 0×0/no-item negative control을 둔다. |
| 18 | whitespace-only sequence가 뒤 block의 auto-placement cell을 소비하는가? | PASS — marker가 첫 cell `(0,0)`에 놓이는 scene을 별도 둔다. |
| 19 | leading preserved spaces가 max-content에서 사라지는가? | PASS — 앞 공백 20px과 text 20px, 합계 40px을 고정한다. |
| 20 | trailing preserved spaces가 max-content에서 사라지는가? | PASS — 뒤 공백 두 개를 포함한 40px 결과를 독립 scene으로 둔다. |
| 21 | Parley 0.12 tab 동작이 CSS `tab-size:8`을 자동 만족한다고 가정하는가? | PASS — 구현 계획은 이 가정을 금지하고 `XX\tXX` 100px 경로를 요구한다. |
| 22 | authored `tab-size` 지원이 제한된 fixture 계약에 포함된 것처럼 보이는가? | PASS — 고정 기본값만 범위에 두고 author 설정은 제외한다. |
| 23 | CR, CRLF, form-feed line break 정규화에서 원본 source range를 잃는가? | PASS — LF 정규화와 CRLF 양 code unit provenance 보존 요구를 명시한다. |
| 24 | 새 mode 도입이 기존 Normal/NoWrap/PreLine 정규화 동작을 바꿀 수 있는가? | PASS — 기존 mode 보존과 회귀 assertion 요구를 계획한다. |
| 25 | `PreWrap` allowlist가 anonymous C11.4 fixture 경계 밖으로 넓어지는가? | PASS — opt-in C11.4 경로만 열고 order 및 paragraph profile 제한을 유지한다. |
| 26 | root HTML, arbitrary cascade, public CSS API 또는 사용자 요청의 root HTML parity가 완료된 것처럼 기록되는가? | PASS — 해당 범위를 명시적으로 제외하고 별도 요청은 미완료로 남긴다. |
| 27 | 앱 scene 수 계산이 기존 9개와 신규 7개를 중복/누락하는가? | PASS — final matrix를 16개로 고정하고 신규 scene 이름을 열거한다. |
| 28 | Android/iOS 플랫폼 gate가 빠지거나 지원 플랫폼 범위를 과장하는가? | PASS — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각각 16/16을 요구한다. |
| 29 | frame, track trace, draw/present, surface sample이 같은 검증 층으로 취급되는가? | PASS — 계획은 각 산출물을 분리 기록하고 renderer/compositor 한계를 남긴다. |
| 30 | fixture marker가 기본 product artifact에 들어가도 검출되지 않는가? | PASS — 기본 archive negative와 fixture-test positive control을 함께 요구한다. |

## 최종 판정

계획 PR diff에서 수정이 필요한 차단 문제를 발견하지 않았다. 이 검토는 계획 문서와 사전 비교 증거의 범위만 판정하며, 구현·테스트·simulator·제품 CSS 지원을 승인하거나 완료로 표시하지 않는다. 후속 구현 PR에는 별도의 20관점 구현/runtime 검토와 30관점 PR diff 검토가 필요하다.

`mise exec -- bun run docs:build`는 저장소의 기존 dead link 다수로 실패했다. 실패 목록에서 새 사전 비교 README와 이번 변경의 STATUS 계획 링크는 나타나지 않았다.
