# C11.4 `tab-size` 계획 PR diff 별도 실패 관점 검토 20개

**검토일:** 2026-10-31

**검토 범위:** 계획 PR에 포함할 plan, C11.4 overview entry, precomparison HTML/JSON/screenshot/comparator, plan review/evidence 문서. 제품 runtime 소스는 변경하지 않는다.

**분리:** 계획 자체의 실패 관점은 [별도 문서](../../../../plan/c11-4-anonymous-grid-text-pre-tab-size-values-plan-review-20.md)에 기록했다. 여기서는 repository diff와 review payload 경계를 검사한다.

| # | PR diff 실패 관점 | 판정과 확인 |
| ---: | --- | --- |
| 1 | 사용자 수정·이전 증거까지 우연히 PR에 들어갈 수 있음 | **PASS** — 해당 브랜치에서 의도한 plan, overview, fixture, 비교기와 전용 evidence 경로만 명시적으로 stage한다. |
| 2 | 제품 코드가 계획 PR에 섞여 계획 이후 구현 분리가 사라질 수 있음 | **PASS** — `crates/`·app·platform source 변경은 없고 plan/evidence만 포함한다. |
| 3 | 계획 후보가 공식 구현완료로 승격될 수 있음 | **PASS** — `spec/STATUS.md`와 C11.4/C11 checkbox 및 `72/213`을 변경하지 않는다. |
| 4 | overview 문구가 구현 완료 또는 simulator 통과를 암시할 수 있음 | **PASS** — “후속 계획”, “계획 PR 병합 전 구현 시작 금지”로 기록한다. |
| 5 | plan과 overview가 scene 수를 다르게 말할 수 있음 | **PASS** — 14 values × 10 positive inputs = 140, plus 14 suppressed controls = 154로 서로 대조했다. |
| 6 | tab-only JSON 기대값이 실제 Grid item으로 잘못 해석될 수 있음 | **PASS** — `ignored-whitespace-only-anonymous-grid-text`로 분류하고 README·plan에서 runtime positive 제외를 설명한다. |
| 7 | decimal 기대값을 Chrome geometry와 직접 비교해 false failure를 만들 수 있음 | **PASS** — ideal advance를 보존하고 comparator에서 1/64px outward geometry conversion만 적용한다. |
| 8 | comparator가 실패를 숨기거나 성공 코드로 끝날 수 있음 | **PASS** — 모든 불일치를 모아 출력하고 하나라도 있으면 nonzero exit한다. 현재 output은 mismatch 0이다. |
| 9 | 비교기가 repository 외부 파일이나 network에 의존할 수 있음 | **PASS** — Python standard library와 같은 evidence 디렉터리의 JSON만 읽는다. |
| 10 | raw observation을 계산 기대값으로 오표기할 수 있음 | **PASS** — Chrome 관측 JSON과 산술 기대값 JSON을 별도 파일·해시로 둔다. |
| 11 | 1/64px 동작을 CSS 표준 규칙으로 잘못 설명할 수 있음 | **PASS** — Blink geometry 관측 보정으로 한정하고 CSS tab semantics와 분리한다. |
| 12 | unpinned Chromium mainline을 Chrome 156 소스 revision으로 오인할 수 있음 | **PASS** — README가 revision을 별도로 고정하지 않았음을 분명히 적는다. |
| 13 | CSS `tab-size` 규범 링크의 section/hash가 깨질 수 있음 | **PASS** — CSS Text Level 3 §4.1.2/§4.2 anchor를 직접 사용한다. |
| 14 | tab-only ignored control의 W3C 근거가 빠질 수 있음 | **PASS** — CSS Grid Layout Module Level 2 §6를 별도로 연결한다. |
| 15 | screenshot이 glyph pixels 또는 simulator 증거인 것처럼 쓰일 수 있음 | **PASS** — browser full-page screenshot으로 표시하고 glyph/platform pixel proof가 아니라고 제한한다. |
| 16 | browser 환경 식별자가 재현에 부족할 수 있음 | **PASS** — HeadlessChrome 156.0.0.0, viewport, DPR, Ahem load와 metrics를 기록한다. |
| 17 | 재현 명령이 eval payload 없이 실행 불가능할 수 있음 | **PASS** — Agent Browser open/wait/eval/screenshot/close 명령 및 JSON unwrap 경로를 README에 명시한다. |
| 18 | Terminal Browser 사용 또는 실행 뒤 브라우저 잔류가 발생할 수 있음 | **PASS** — Agent Browser만 사용하고 `close --all`을 실행했으며 이후 프로세스 검색 결과가 비었다. |
| 19 | hash가 최신 파일과 일치하지 않을 수 있음 | **PASS** — HTML, Ahem, expected JSON, comparator, observation과 screenshot SHA-256을 재계산해 README 표와 대조한다. |
| 20 | PR template의 상태 ID·명세·동작 근거·남은 범위를 채울 수 없을 수 있음 | **PASS** — C11.4, CSS Text §§4.1.2/4.2, internal contracts 0059/0070, precomparison evidence 및 미완료 범위를 PR 본문에 명시한다. 상태 대장 변경은 해당하지 않는다. |

**결과:** 계획 payload diff를 20개 서로 다른 변경·증거 경계로 검토했다. 발견한 tab-only 분류, non-binary decimal, geometry quantization, hash와 reproduction 문제를 수정한 뒤 모두 PASS다. 이 문서는 계획 자체 20개 review나 이후 구현/runtime 20개 review를 대신하지 않는다.
