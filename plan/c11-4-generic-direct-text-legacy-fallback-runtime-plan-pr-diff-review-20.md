# C11.4 direct-text legacy fallback runtime 계획 PR 변경 검토 20

## 검토 대상

- 새 계획: [`c11-4-generic-direct-text-legacy-fallback-runtime.md`](./c11-4-generic-direct-text-legacy-fallback-runtime.md)
- 계획 자체 실패 관점 검토: [`c11-4-generic-direct-text-legacy-fallback-runtime-plan-review-20.md`](./c11-4-generic-direct-text-legacy-fallback-runtime-plan-review-20.md)
- 상위 인덱스: [`c11-4-grid-track-sizing.md`](./c11-4-grid-track-sizing.md)
- 범위: 고정된 13개 `TEXT_CASES`의 legacy fallback Rust dispatch와 Android/iOS app runtime evidence 계획. 구현·시뮬레이터 실행은 하지 않았다.

## 서로 다른 변경 실패 경로

| # | 실패 경로·불변 조건 | 근거 및 판정 |
| ---: | --- | --- |
| 1 | 계획이 이미 구현된 6-scene generic direct-text matrix를 새 일로 중복 등록함 | 새 범위는 one-item `TEXT_CASES` 13개이며 기존 six-scene positive matrix를 명시적으로 제외한다. PASS |
| 2 | #704의 first-case legacy fallback 테스트를 13개 완료로 잘못 계산함 | 계획 상태는 기존 증거가 첫 case 한 개라고 한정하고 나머지 12개를 matrix에 포함한다. PASS |
| 3 | scene ID 표가 실제 `TEXT_CASES`와 다름 | source array와 13개 row를 대조했다. ID·count가 일치한다. PASS |
| 4 | FFI scene source와 Rust case의 root/item/text identity가 다름 | FFI registry를 확인했고 계획은 source와 typed case identity를 함께 검증한다. PASS |
| 5 | specialized adapter가 legacy fallback 앞에서 scene을 가로채도 fallback이라 주장함 | 계획이 각 case의 dispatcher mapping 및 full prepare result를 구현 전 audit에서 확인하게 한다. PASS |
| 6 | generic collector가 `None` 반환한 것만으로 dispatcher success를 주장함 | generic `None`과 full `c11_4_text::prepare` 결과를 개별 assertion으로 분리한다. PASS |
| 7 | mutation/control에서 다른 root/item/text가 allowlist에 섞임 | 계획은 unknown/변형 identity의 partial success를 거부하고 legacy IDs를 고정한다. PASS |
| 8 | Chrome page y offset이 native root-local y로 사용됨 | 모든 expected item frame은 root-local `(0,0)`으로 정규화돼 있다. PASS |
| 9 | 10개 기본 case의 expected geometry가 원본 Chrome 근거와 불일치함 | Chrome 155 observation의 columns, rows와 root/item sizes를 대조했다. PASS |
| 10 | 세 cyclic case의 expected geometry가 Chrome 156 observation과 불일치함 | cyclic precomparison의 scene list와 dimensions를 대조했다. PASS |
| 11 | Chrome 관찰이 규범 기대값 또는 Spinon 결과로 오해됨 | 계획에서 Chrome을 기준 관찰로만 분류하고 CSS Grid/Ahem 계산과 분리한다. PASS |
| 12 | Ahem font 또는 raw text source revision이 달라 input이 바뀜 | full dispatch gate가 Ahem intrinsic values/revision과 text source를 검사한다. PASS |
| 13 | box edge/min-width/max-width/word-break case가 다른 track contribution으로 처리됨 | edge·min-width·max-width·break-all case를 고유 행으로 고정하고 typed input과 frame을 비교한다. PASS |
| 14 | `fit-content(calc())` 또는 cyclic percentage basis가 누락됨 | 기존 precomparison JSON과 computed basis/trace matcher를 재사용하도록 계획한다. PASS |
| 15 | Android matrix만 통과하거나 iOS만 통과해도 전체로 보고됨 | 각 플랫폼 독립 13/13 및 동일 source commit을 요구한다. PASS |
| 16 | duplicate/missing scene, stale trace 또는 revision mismatch가 matcher에서 통과함 | 정확한 scene set/count와 same-scene revision tuple, trace/frame rejection 조건이 적혀 있다. PASS |
| 17 | layout/draw 제출을 glyph raster 또는 pixel parity로 과장함 | glyph paint와 surface equality는 명시적 제외이며 telemetry가 0이면 보고하지 않는다. PASS |
| 18 | unchanged FFI code로 artifact exclusion을 새로 주장함 | FFI source/ABI가 바뀌지 않으면 새 artifact boundary claim을 금지한다. PASS |
| 19 | 계획 PR이 STATUS/C11.4 checkbox를 조기 완료 처리함 | 계획 및 상위 entry가 `72/213`과 C11.4 미완료를 유지한다. PASS |
| 20 | Agent Browser/CI/Tailnet 결과를 실행 전에 완료로 적음 | 아직 실행하지 않았다고 표시하고, Browser는 merge 뒤 screenshot link가 있을 때만 사용하며 empty check rollup은 pass로 치지 않는다. PASS |

## 판정

계획 변경 diff에서 범위 중복, input identity, expected geometry, runtime gate와 status claim을 대조했다. 구현이나 simulator를 실행한 것으로 표시하지 않는다. **PASS 20 / PARTIAL 0 / FAIL 0.**
