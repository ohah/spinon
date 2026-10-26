# C11.4 row-axis span-stage 계획 PR diff 검토 30관점

**대상:** `origin/main` `3fc9802900f9f3346f4dc8392cfe032f162a8c91` 대비 계획 PR staged diff 12개 경로와 연결 자료.

| # | 검토 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 기준 branch가 최신 `origin/main`인가? | HEAD와 merge-base가 기준 SHA에 일치한다. **PASS** |
| 2 | unrelated unstaged 작업이 섞였는가? | 검토 당시 변경 12개 경로는 모두 staged였고 범위 밖 변경은 없었다. **PASS** |
| 3 | 계획 PR에 product/runtime/FFI 구현이 섞였는가? | 변경은 문서, 고정 비교자료 및 CSS HTML fixture에 한정된다. **PASS** |
| 4 | simulator 실행을 생략한 범위 설명이 타당한가? | 새 앱/runtime route가 없는 pure Rust trace test 계획이라 플랫폼 실행 주장을 하지 않는다. **PASS** |
| 5 | 공식 완료 수가 잘못 증가했는가? | `72/213`을 유지한다. **PASS** |
| 6 | C11/C11.4 상위 항목을 완료로 바꿨는가? | 공식 상태 checkbox가 미완료다. **PASS** |
| 7 | 상태 대장 기준일과 새 기록 날짜가 맞는가? | 기준일을 `2026-10-27`로 맞췄다. **PASS** |
| 8 | #664 merge time/SHA가 사실과 일치하는가? | UTC 시각과 merge SHA가 상위 계획 및 HEAD 이력과 일치한다. **PASS** |
| 9 | #669 merge time/SHA가 사실과 일치하는가? | UTC 시각과 merge SHA가 상위 계획 및 HEAD 이력과 일치한다. **PASS** |
| 10 | 신규 계획이 구현 완료나 C11.4 완료를 암시하는가? | 계획·precomparison이고 구현과 상위 완료 게이트는 남았다고 명시한다. **PASS** |
| 11 | 상위 계획·상태 대장·근거 자료 링크가 연결되는가? | row-axis 계획과 precomparison 상대 경로가 맞는다. **PASS** |
| 12 | 계획 PR diff 검토 링크가 존재하는가? | 계획 61행의 상대 링크가 이 30관점 보고서로 연결된다. **PASS** |
| 13 | row-axis slice 필요성이 Stage B 범위에 연결되는가? | 기존 Columns-only oracle과 양 축 요구의 공백을 설명한다. **PASS** |
| 14 | Rows 축이 실제 trace API에서 선택 가능한가? | 기존 trace에 `LayoutGridTrackSizingAxis::Rows`가 보존된다. **PASS** |
| 15 | 새 Rust 테스트가 test-only 모듈 아래에 제한되는가? | 계획은 기존 test-only 모듈의 child만 추가하며 product/FFI 변경은 제외한다. **PASS** |
| 16 | 논리 row contribution이 올바른 dimension에 연결되는가? | horizontal writing mode에서 item heights를 row contribution으로 둔다. **PASS** |
| 17 | root와 explicit row shape가 고정되는가? | definite `220×220` root와 네 `minmax(0,max-content)` row를 고정한다. **PASS** |
| 18 | fixed Columns sizing이 Rows 관찰과 섞이지 않는가? | 단일 220px column과 early-return control을 별도로 확인한다. **PASS** |
| 19 | HTML item shape가 기존 HostDocument builder와 맞는가? | fixture의 빈 `div`와 builder의 item element 구조가 맞는다. **PASS** |
| 20 | span-2 A/B가 실제로 한 row를 공유하는가? | A는 rows 1–2, B는 2–3이라 row 2에서 겹친다. **PASS** |
| 21 | span-3 C가 네 번째 row까지 잘못 확장되는가? | C의 explicit placement는 rows 1–3뿐이다. **PASS** |
| 22 | reversed source order가 placement 자체도 바꾸는가? | item ID별 placement와 크기는 유지하고 DOM 순서만 바꾼다. **PASS** |
| 23 | span-2 뒤 span-3 순서의 규범 근거가 직접 연결되는가? | README가 §11.5의 작은 span 우선 순서, §11.5.1의 planned increase, §11.6 maximize를 각각 링크하고 요약한다. **PASS** |
| 24 | 같은 span group에서 겹친 planned increase를 합산하는가? | 겹친 row는 item별 증가 합이 아니라 최댓값 `[40,60,60,∞]`을 사용한다. **PASS** |
| 25 | span-3 contribution이 앞 batch 결과 뒤에 적용되는가? | 기존 160px 합에서 남은 40px을 세 row에 균등 분배한다. **PASS** |
| 26 | 독립 산술이 Spinon trace를 역산하거나 fixture와 동기화됐다고 과장하는가? | exact `Fraction` 산술은 hardcoded input이며 HTML 자동 parsing을 하지 않는 한계를 명시한다. **PASS** |
| 27 | batch 종료 snapshot을 transient subpass로 과장하는가? | 계획은 각 batch 종료 state만 검사하고 intra-batch transient 값은 제외한다. **PASS** |
| 28 | serializer `None`을 실제 positive infinity 증명으로 취급하는가? | non-finite 값 합침으로 인한 구분 한계를 기록한다. **PASS** |
| 29 | track/gutter/contribution shape가 assertion 범위에 있는가? | row count, gutter/boundary flags, zero-gap, 세 contribution 종류를 검사한다. **PASS** |
| 30 | 규범적 thirds, Chrome CSSOM/DOMRect 정밀도, frame 및 hash 증거를 혼동하는가? | 각 정밀도를 분리하고 root/item frame 및 5개 hash를 보존한다. **PASS** |

## 수정 및 재검토

초기 독립 검토는 **28 PASS / 1 PARTIAL / 1 FAIL**이었다. 계획-review 파일 누락과 §11.5 규범 링크 누락을 수정한 뒤 affected-view 재검토를 수행해 두 관점 모두 PASS로 확인했다. 최종 결과는 **30 PASS / 0 PARTIAL / 0 FAIL**이다. STATUS 기준일·#664/#669 merge 메타데이터·README 입력 동기화 한계 및 항목 들여쓰기도 재확인했다.

이 보고서는 계획 PR diff만 다룬다. 계획 자체 검토 20관점과 추후 구현/runtime 검토 20관점을 대신하지 않는다. PR 본문은 검토 횟수 대신 범위, 근거 및 남은 한계를 설명한다.
