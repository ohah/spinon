# C11.4 `safe|unsafe flex-end` 계획 실패 관점 검토 20개

계획과 Chrome 157 사전 비교 입력을 대상으로, 구현 전에 distinct failure 경로와 검증 경계를 검토했다. 브라우저 관측 6 scene은 독립 계산으로 고정한 뒤 비교했다.

| # | 계획 위험 | 판정 기준·계획의 대응 | 결과 |
|---:|---|---|---|
| 1 | `flex-end`의 non-flex 의미를 잘못 해석할 수 있음 | CSS Box Alignment §4.1의 Grid 외부 alias는 `end`이며 precomparison에서도 end control과 대조 | 해결 |
| 2 | safe와 unsafe가 같은 overflow 배치를 낸다고 가정할 수 있음 | 독립 기대값에서 safe `[0,56,112]`, unsafe `[-82,-26,30]`을 분리 | 해결 |
| 3 | positive free space에서 safe fallback이 잘못 적용될 수 있음 | end·safe·unsafe positive를 모두 `[10,20,30]`으로 고정 | 해결 |
| 4 | percentage `row-gap`가 예상 basis에서 resolve되지 않을 수 있음 | definite 40px block basis와 `calc(120% - 2px) = 46px`를 입력에 고정 | 해결 |
| 5 | overflow subject 크기가 잘못 계산될 수 있음 | rows 3×10px + gutters 2×46px = 122px 산식을 기록 | 해결 |
| 6 | unsafe negative coordinates를 screen screenshot으로 오판할 수 있음 | off-screen frame은 Rust coordinates로 판정하고 screenshot은 visible bounds 전용 | 해결 |
| 7 | implicit row sizing이 alignment에 따라 변할 수 있음 | 모든 scene의 used rows `[10,10,10]px`를 기대값에 포함 | 해결 |
| 8 | gutter가 alignment 값에 따라 달라질 수 있음 | overflow trace에서 `[46,46]px` gutter 불변 조건을 구현 gate에 명시 | 해결 |
| 9 | CSS computed string에서 safety modifier가 유실될 수 있음 | `safe flex-end`/`unsafe flex-end` exact serialization 확인 | 해결 |
| 10 | `safe flex-end`만 추가되고 unsafe 누락 또는 반대가 될 수 있음 | 두 값을 각각 positive/overflow scene에 포함 | 해결 |
| 11 | plain `end` 회귀가 새 gate에 섞일 수 있음 | end control 및 existing regression gate를 명시 | 해결 |
| 12 | plain `flex-end`와 safety-modified values를 혼동할 수 있음 | plain flex-end는 기존 지원; 새 scope는 exact safety modifier 두 값으로 제한 | 해결 |
| 13 | 이전 `safe|unsafe flex-start` 지원이 회귀할 수 있음 | 이전 slice 회귀를 별도 검증 게이트에 남김 | 해결 |
| 14 | safe/unsafe center·end 값이 회귀할 수 있음 | 기존 bounded values와 focused regressions를 유지 | 해결 |
| 15 | style projection과 layout gate가 다른 집합을 허용할 수 있음 | 두 gate의 exact values 일치 검사 요구 | 해결 |
| 16 | unsupported values가 generic parser까지 통과할 수 있음 | 일반 parser/admission 및 baseline·distributed safety는 미지원으로 고정 | 해결 |
| 17 | runtime failure가 잘못된 property로 귀속될 수 있음 | 오류가 `align-content`에 귀속되는지 확인하도록 고정 | 해결 |
| 18 | negative request가 partial snapshot을 노출할 수 있음 | atomic rejection과 same-session recovery 요구 | 해결 |
| 19 | Chrome observation이 독립 oracle과 혼합될 수 있음 | expected coordinates를 독립 산식으로 먼저 기록하고 별도 compare script 실행 | 해결 |
| 20 | bounded result를 전체 C11.4 완료로 과장할 수 있음 | status checkbox·`78/214`·contract version 변경 금지 명시 | 해결 |

W3C Working Draft 상태와 `<overflow-position>` at-risk 표기는 계획에 명시했다. 실행 전 비교는 Spinon 구현이나 모바일 결과를 대체하지 않는다.
