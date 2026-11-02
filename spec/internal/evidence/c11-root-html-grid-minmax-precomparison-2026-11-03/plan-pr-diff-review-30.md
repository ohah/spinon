# C11.1 Root HTML Grid `minmax()` 계획 PR diff 적대 검토 30개

검토 대상은 계획 PR의 전체 변경 파일이다: 계획, 계획 검토 20개, 사전 비교 README/expected/Chrome observation/screenshot, 고정 HTML fixture 및 이 검토 기록. 이 검토는 계획 실패 관점 20개와 별개이며 구현 검토가 아니다.

| # | PR diff 독립 점검 | 판정 |
|---:|---|---|
| 1 | 추가 파일이 C11.1 `minmax()` 범위와 직접 연관되는가 | **PASS** — 계획, precomparison fixture/evidence와 검토 기록만 추가했다. |
| 2 | C11 공식 checkbox나 `72/213`을 잘못 올리는가 | **PASS** — 계획과 fixture는 구현 결과가 아니며 승격 금지를 명시한다. |
| 3 | 구현이 이미 반영됐다고 독자에게 오해시키는가 | **PASS** — plan status와 README 모두 제품 runtime이 아직 없다고 밝힌다. |
| 4 | 기준 문서가 최신 임시 draft 또는 변경 가능한 editor copy인가 | **PASS** — CSS Grid 2025-03-26 CRD 고정 URL을 참조한다. |
| 5 | `minmax` grammar 위치와 W3C anchor가 잘못됐는가 | **PASS** — §7.2.1 `track-sizing-functions`와 §11.6/§11.7.1 anchor를 명시했다. |
| 6 | max<min 설명이 반대로 기록됐는가 | **PASS** — max가 min으로 floor됨을 expected와 계획에서 동일하게 적었다. |
| 7 | flex breadth의 min/max 위치를 혼동하는가 | **PASS** — max `Fraction`만 허용하고 flex minimum은 CSS-invalid로 둔다. |
| 8 | 브라우저 observation을 normative expected로 중복 복사했는가 | **PASS** — 두 JSON을 분리하고 normative/source 역할을 설명했다. |
| 9 | browser version이나 user agent가 재현에 부족한가 | **PASS** — HeadlessChrome 156.0.0.0과 user-agent를 기록했다. |
| 10 | fixture가 수정된 뒤 이전 hash를 가리키는가 | **PASS** — expected와 observation의 input SHA-256이 현재 fixture와 일치한다. |
| 11 | screenshot hash가 실제 capture와 다른가 | **PASS** — evidence README와 observation의 SHA-256이 image bytes와 일치한다. |
| 12 | JSON expected와 observed scene 개수가 서로 모순되는가 | **PASS** — 16 positive + 4 controls가 20 observation과 일치한다. |
| 13 | used track size와 item origin을 같은 frame 의미로 혼동하는가 | **PASS** — tracks, root bounds, local item origins를 각각 저장한다. |
| 14 | minimum-overflow는 실제 root 경계 밖에 item을 두는가 | **PASS** — 30px root 밖 x=40..48px의 marker를 관측했다. |
| 15 | fixed tracks의 max-growth 계산이 gap을 누락하는가 | **PASS** — two-bounds와 flex-max scene에서 gap과 second item origin을 함께 고정한다. |
| 16 | percent maximum이 inline basis를 사용하는가 | **PASS** — 200px definite width에 대한 10/30/40/60% case를 포함한다. |
| 17 | percent minimum의 max-floor가 검증되는가 | **PASS** — percentage/percentage 및 px/percentage floor scene이 있다. |
| 18 | percentage row basis를 column width와 혼동하는가 | **PASS** — 독립 100px height 기준 row percentage 장면을 고정한다. |
| 19 | definite zero basis 또는 0fr이 누락됐는가 | **PASS** — 둘 다 별도 positive expected와 browser result가 있다. |
| 20 | fixed integer repeat을 auto-repeat로 넓혀 해석하는가 | **PASS** — `repeat(2, …)`만 positive로 두고 auto-repeat은 명시 제외한다. |
| 21 | invalid flex minimum을 valid bounded track으로 처리하는가 | **PASS** — browser CSS-invalid control을 product positive matrix와 분리한다. |
| 22 | invalid negative max의 fallback implicit track을 MinMax 성공으로 계산하는가 | **PASS** — 8px control을 invalid declaration aftermath로 설명한다. |
| 23 | 유효한 `auto`/intrinsic breadth를 syntax-invalid로 분류하는가 | **PASS** — 두 valid CSS controls를 별도 보존하고 slice rejection과 구분한다. |
| 24 | supported min/max breadth 허용 목록이 구현 단계에서 모호한가 | **PASS** — min/max의 typed variants와 finite/non-negative 조건을 열거한다. |
| 25 | percentage/fr gate가 nested breadth를 놓칠 위험을 계획이 무시하는가 | **PASS** — root_html_grid_tracks axis/range validator를 별도 수정 대상으로 지정한다. |
| 26 | fixed named-area, implicit track 또는 column-flow contract에 묵시적으로 새어 나가는가 | **PASS** — 각각 기존 guard 유지와 fail-closed를 적었다. |
| 27 | line-name/repeat quotas 또는 allocation safety가 없어지는가 | **PASS** — cap/quota와 allocation failure atomicity를 유지하도록 요구한다. |
| 28 | precomparison screenshot을 product runtime/PixelCopy 증거라고 부르는가 | **PASS** — browser capture로만 분류하고 simulator evidence를 미래 acceptance로 남겼다. |
| 29 | simulator·hardware·WPT 범위를 지나치게 넓혀 보이는가 | **PASS** — AVD/iOS drawable 시점과 미검증 hardware/WPT 범위를 분리한다. |
| 30 | repository relative links가 계획/evidence 위치 변경으로 깨지는가 | **PASS** — 문서 경로를 검토했고 fixture, internal evidence, C11 plan 경로를 서로 연결했다. |

검토에서 발견된 수정 필요 사항은 없다. 구현/runtime 검토는 계획 PR 병합 뒤 새 관점으로 별도 수행한다.
