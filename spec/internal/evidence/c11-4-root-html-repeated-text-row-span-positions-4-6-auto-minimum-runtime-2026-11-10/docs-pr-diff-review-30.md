# C11.4 PR #1331 병합 상태·공개 roadmap 문서 PR diff 검토 30개

검토 대상은 제한 구현 PR #1331 병합 후 수정한 `spec/STATUS.md`, `spec/0001-conformance.md`, 내부 계약 0059, 계획 문서, `spec/public/roadmap.html` 및 이 검토 근거다. 제품 코드는 변경하지 않는다. GitHub PR 상태와 merge tree를 기준으로 병합 상태·공식 완료 집계·한정 범위·검증 표현·roadmap 링크를 별도로 확인했다.

| # | 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 열린 PR을 병합 완료로 기록한다. | PASS — GitHub PR #1331은 `MERGED`이며 `mergedAt=2026-11-10T10:00:02Z`다. |
| 2 | 병합 시각의 UTC 표기를 로컬 시간으로 바꿔 혼동한다. | PASS — 네 공식 문서와 roadmap에 원본 `Z` timestamp를 유지한다. |
| 3 | PR head를 merge SHA로 잘못 표기한다. | PASS — PR head `55f40e29…`와 merge SHA `4b4f48ea…`를 구분한다. |
| 4 | 계획 PR·기준 구현 PR·이번 구현 PR을 하나로 합친다. | PASS — #1306은 기준 profile, #1331은 positions 4–6 `min-height:auto` 구현으로 구분한다. |
| 5 | Rebase and merge를 다른 merge 방식으로 기록한다. | PASS — PR merge 방식과 원격 main merge SHA를 대조했다. |
| 6 | PR check가 없는데 CI green이라고 적는다. | PASS — status, plan, contract와 roadmap에서 check 부재를 밝히고 CI 통과로 세지 않는다. |
| 7 | C11.4를 완료 checkbox로 올린다. | PASS — `spec/STATUS.md` C11.4 checkbox는 미완료다. |
| 8 | C11 상위 항목을 완료로 승격한다. | PASS — C11 checkbox를 바꾸지 않는다. |
| 9 | 공식 완료 수를 `78`에서 올린다. | PASS — 공식 집계 `78/214`를 유지한다. |
| 10 | 공식 집계 분모 `214`를 다른 값으로 바꾼다. | PASS — STATUS, 계획, conformance, contract와 roadmap에서 같은 분모를 쓴다. |
| 11 | bounded 구현을 제품 지원 완료로 계산한다. | PASS — 현재 제품 지원 완료 없음과 상위 미완료 상태를 유지한다. |
| 12 | 내부 numeric contract를 작업 병합만으로 올린다. | PASS — 숫자 버전 `0.1.0`을 유지한다. |
| 13 | 세-row positions 1–3 profile과 six-row profile을 혼동한다. | PASS — 이번 slice를 여섯 positive implicit row profile로 한정한다. |
| 14 | 모든 fit-content 위치를 허용한다고 쓴다. | PASS — authored positions 4–6 범위만 표시한다. |
| 15 | 기존 track multiset 또는 topology를 넓힌다. | PASS — 기존 bounded multiset 및 두 fixed topology family를 유지한다고 적는다. |
| 16 | explicit-row empty control에도 `min-height:auto`를 적용한다고 오해할 수 있다. | PASS — control은 `min-height:0`으로 남는다. |
| 17 | mixed item minimum, 새 grammar 또는 arbitrary row-span을 지원한다고 암시한다. | PASS — 일반 CSS admission과 unsupported combinations는 미지원으로 유지한다. |
| 18 | Chrome 사전 비교를 Spinon 제품 결과로 취급한다. | PASS — precomparison과 RuntimeSession 결과를 별도 근거로 기술한다. |
| 19 | 4,320 pairs를 4,320 total observations로 축소한다. | PASS — 4,320 baseline/candidate pairs 및 8,640 observations로 구분한다. |
| 20 | eligible item occurrence를 전체 DOM node 수나 scene 수로 잘못 해석한다. | PASS — `12,240 eligible item occurrence`로 분모 의미를 고정한다. |
| 21 | content-floor·overflow 통과 또는 source-order/stretch 비교를 누락한다. | PASS — 각 invariant와 `0 CSS px` 차이를 연결한다. |
| 22 | 기존 positions 1–3 regression 범위를 제거하거나 과장한다. | PASS — 2,880 paired scenes로 한정해 기재한다. |
| 23 | two-fit regression control을 새 기능 지원 범위로 해석한다. | PASS — 회귀 확인으로만 설명하고 admission 확대를 주장하지 않는다. |
| 24 | focused rejection/recovery와 rounding test 수를 변경한다. | PASS — focused 5개 및 layout rounding 5개를 evidence log와 대조한다. |
| 25 | scoped Clippy를 workspace Clippy로 보고한다. | PASS — 변경 crate scoped Clippy만 통과했고 workspace Clippy는 실행하지 않았다고 밝힌다. |
| 26 | iOS Simulator 또는 Android AVD를 실기기로 표현한다. | PASS — iPhone 17 Pro/iOS 26.2 Simulator와 API 37 ARM64 AVD로 구분한다. |
| 27 | visual smoke를 PixelCopy·Metal surface·compositor pixel parity로 과장한다. | PASS — surface readback, compositor parity, hardware GPU, 실기기와 WPT 한계를 명시한다. |
| 28 | plan/runtime/review/screenshot 링크가 없거나 잘못된 route를 가리킨다. | PASS — source evidence files와 commit-pinned Android/iOS screenshot 경로를 확인한다. |
| 29 | 공개 roadmap에 이전 PR #1326이 최신 병합으로 계속 표시되거나 새 card의 HTML 경계가 깨진다. | PASS — 상단 요약과 progress history를 #1331로 갱신하고 단일 닫힌 `<p>`로 구성한다. |
| 30 | 문서 PR merge 전 Tailnet preview가 동기화됐다고 주장한다. | PASS — PR에는 source roadmap update만 기록하고 Tailnet 제공 hash·브라우저 확인은 merge 후 별도 evidence에서 확인한다. |

## 판정

현재 문서 변경은 bounded implementation과 미완료 상위 상태를 구분한다. PR merge 후 Tailnet preview에 선택한 문서 경로를 반영하고 실제 HTTP 제공 hash와 desktop/mobile Agent Browser 화면을 별도 실행 근거로 기록한다.
