# C11.4 row-axis stage JSON 연결 보강 계획 검토 20개

검토 대상: [후속 보강 계획](./c11-4-row-axis-span-stage-oracle.md#2026-10-27-후속-보강-계획-독립-기대값-json을-test-matcher에-연결), [독립 기대값 JSON](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/expected.json), 기존 test helper와 `serde_json` dev dependency. 이는 구현 전 계획 검토이며 코드를 수정하거나 테스트를 실행하지 않았다.

| # | 독립 실패 관점 | 계획 검토와 통제 | 판정 |
| ---: | --- | --- | --- |
| 1 | 기대값 JSON이 Chrome 관측에서 역산될 수 있음 | 기준은 이미 저장된 CSS Grid 2025 CRD 유리수 계산 JSON이고 Chrome은 frame control로 남긴다. | PASS |
| 2 | JSON parser가 expected fixture와 무관한 파일을 읽음 | `include_str!` 경로를 단일 checked-in `expected.json`으로 고정하고 변경 근거도 같은 파일에 연결한다. | PASS |
| 3 | 새 테스트 값만 JSON과 비교하고 실제 runtime matcher는 옛 상수를 계속 쓸 수 있음 | 기존 공통 stage assertion을 expectation struct를 받도록 좁게 일반화하고 row-axis actual trace 비교에 JSON 유래 값을 직접 넘긴다. | PASS |
| 4 | scene 하나가 빠지거나 중복되어 부분 matrix가 성공함 | 전체 result 수를 네 case와 대조하고 ID set 및 uniqueness를 검사한다. | PASS |
| 5 | JSON source order와 실제 document order가 달라도 성공함 | 각 semantic item ID 배열을 `RowAxisItem` authored order와 직접 대조한다. | PASS |
| 6 | row contribution JSON이 fixture item 높이와 달라도 성공함 | case별 A/B/C contribution map의 key set과 numeric values를 입력 item ID/height에 맞춘다. | PASS |
| 7 | normative batch order가 실제 span placement와 어긋남 | JSON의 `span2`/`span3` 목록과 `expected_span_batches`, 실제 Rows batch stage를 모두 일치시킨다. | PASS |
| 8 | Initialize/batch/final snapshot 중 하나만 JSON에서 읽음 | Initialize, 모든 batch, IntrinsicSizingComplete, MaximizeTracks의 base·growth-limit 배열을 expectation으로 만든다. | PASS |
| 9 | 유리수 문자열이 부동소수점 문자열로 잘못 읽히거나 일부만 파싱됨 | 분자·분모 전체 파싱, 분모 양수, 최종 finite 검사 후 `1e-4 CSS px` tolerance를 적용한다. | PASS |
| 10 | 정수 숫자나 큰/non-finite JSON 값이 유효한 stage 값으로 통과 | JSON 숫자와 rational 모두 finite representable `f32`인지 검증하고 overflow는 field-context assertion failure로 만든다. | PASS |
| 11 | `null` growth limit을 실제 positive infinity라고 과장 | trace/API의 non-finite 축약 의미를 유지하고 `None`으로만 비교한다. | PASS |
| 12 | 잘못된 배열 길이에서 일부 track만 검사 | 네 track 정확한 길이를 강제한다. | PASS |
| 13 | stage snapshot 순서나 중복 stage를 JSON 처리 중 놓침 | 기존 unique-stage 및 complete stage sequence 검사를 유지하고 batch order만 oracle expectation에서 공급한다. | PASS |
| 14 | 남은 공간이 JSON과 별도로 하드코딩되어 drift | MaximizeTracks base sum은 JSON의 available size 및 remaining space 값과 함께 비교한다. | PASS |
| 15 | Chrome frame expectation과 normative stage expectation이 한 oracle로 섞임 | JSON에는 stage 기준값만 공급하고 `chrome_local_y`는 별도 Chrome observation control로 유지한다. | PASS |
| 16 | fixed Columns 회귀 대조군이 기대값 refactor로 바뀜 | 기존 Columns caller는 static helper 경로를 유지하고 row-only JSON caller만 추가한다. | PASS |
| 17 | test dependency가 제품 artifact에 포함될 수 있음 | `serde_json`은 이미 dev dependency이고 JSON include는 `#[cfg(test)]` child module에만 둔다. | PASS |
| 18 | malformed JSON이나 unsupported rational의 오류가 모호함 | 필요한 field별 parsing helper에 case/path를 포함해 test 실패를 진단 가능하게 한다. 제품 runtime 오류 계약으로 확대하지 않는다. | PASS |
| 19 | 앱 simulator/FFI gate를 불필요하게 생략하거나 반대로 실행 근거로 주장 | 순수 Rust trace expectation test만 바꾸므로 simulator/FFI 결과는 계획 acceptance에서 제외한다. | PASS |
| 20 | follow-up test를 Stage B/C11.4 완료 또는 72/213 증가로 표시 | 변경은 test-oracle 동기화만 다루고 공식 checkbox와 `72/213`을 유지한다. | PASS |

## 판정

**PASS 20/20, 계획 blocker 0개.** JSON은 행 축 stage 기대값의 직접 입력이 되고, test는 fixture configuration과 runtime trace 양쪽에서 JSON과의 일치를 검사한다. Chrome frame control과 trace `None` 의미는 별도 경계로 유지한다. 구현/runtime 검토 20개와 PR diff 검토 30개는 코드 변경 뒤 새 관점으로 별도 수행해야 한다.
