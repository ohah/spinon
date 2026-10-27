# PR #685 변경분 적대적 검토 30관점

**검토 대상:** `origin/main...feat/c11-4-row-axis-stage-oracle-json-sync-20261027` 전체 변경분. 계획, Rust test-only 코드, 근거 문서와 로그를 포함한다. 검토 시 PR #685는 OPEN/MERGEABLE이었고 GitHub status-check rollup은 비어 있었다.

| # | 실패 관점 | 판정과 확인 |
|---:|---|---|
| 1 | 계획 커밋이 구현 전에 작성된 계획과 연결되지 않음 | **PASS** — 계획 보강과 20관점 계획 검토가 코드 변경 커밋보다 앞선다. |
| 2 | 계획의 목표가 독립 JSON과 matcher drift 제거를 명확히 하지 않음 | **PASS** — 이전 PR #673의 PARTIAL과 대상 파일, 기대값 소비 방법을 명시한다. |
| 3 | 계획이 Chrome 최종 geometry를 내부 stage oracle로 오인 | **PASS** — Chrome geometry와 CSS CRD stage 값의 역할을 분리한다. |
| 4 | 계획 범위가 product algorithm, FFI 또는 앱 변경으로 넓어짐 | **PASS** — test-only로 제한하고 simulator가 불필요한 이유를 기록한다. |
| 5 | 계획이 C11.4 전체 또는 완료 수를 잘못 승격 | **PASS** — C11.4 미완료와 `72/213` 유지가 반복 명시된다. |
| 6 | 계획 검토 20개가 구현 검토를 대체 | **PASS** — 구현 검토와 PR diff 검토를 별도 파일/단계로 요구한다. |
| 7 | crate root에서 새 helper가 제품 빌드에 포함 | **PASS** — 상위 모듈은 `#[cfg(test)]`로 선언되어 있다. |
| 8 | 새 serde_json 의존성이 추가되거나 제품 dependency가 됨 | **PASS** — 기존 dev dependency를 사용하고 manifest/lockfile 변경이 없다. |
| 9 | `include_str!`가 잘못된 파일을 가리킴 | **PASS** — 지정한 checked-in JSON이 컴파일 시 포함되고 focused test가 이를 읽는다. |
| 10 | malformed JSON이 조용히 기본값으로 대체 | **PASS** — decode 실패는 원인 메시지와 함께 즉시 테스트 실패가 된다. |
| 11 | JSON axis/track count/definite size가 runtime fixture와 어긋남 | **PASS** — `Rows`, 네 track, 220px 기준을 fixture와 대조한다. |
| 12 | JSON scene 누락·추가·중복 | **PASS** — 결과 수, Rust ID uniqueness, JSON ID uniqueness, 정확한 ID 순서를 각각 확인한다. |
| 13 | source order가 HTML fixture와 어긋남 | **PASS** — 각 scene의 semantic item 순서를 실제 `RowAxisItem` 배열과 비교한다. |
| 14 | contribution key 집합에 누락/추가 항목이 있음 | **PASS** — map length와 모든 fixture item key를 검사한다. |
| 15 | contribution height가 fixture 입력과 달라짐 | **PASS** — 각 item 높이를 `1e-4px` 허용오차로 비교한다. |
| 16 | span label이 모호하거나 잘못 파싱됨 | **PASS** — 양의 canonical `spanN`만 받고 malformed label 회귀 테스트가 있다. |
| 17 | normative batch order와 runtime placement batch가 다름 | **PASS** — JSON order, fixture order, actual Rows trace batch order를 모두 연결한다. |
| 18 | batch stage 문자열이 span/flexible flag를 잘못 나타냄 | **PASS** — 정확한 `IntrinsicItemBatch(span=N, crosses_flexible_track=false)`를 검사한다. |
| 19 | stage 기대값 일부가 여전히 Rust 상수로 중복됨 | **PASS** — row-axis base/growth arrays, available block-size와 remaining space를 JSON에서 만든 expectation으로 전달한다. |
| 20 | array가 짧거나 길어 일부 track만 검사 | **PASS** — JSON 배열은 정확히 네 값이어야 하고 오류 길이 회귀 테스트가 있다. |
| 21 | 유리수 parsing이 malformed/0 denominator/overflow를 허용 | **PASS** — 분자·분모 parsing, 양수 분모, finite f32 범위를 확인하고 negative test를 둔다. |
| 22 | JSON `null`을 실제 +∞로 과장 | **PASS** — `None` sentinel로 비교하고 README와 검토서가 serializer 한계를 설명한다. |
| 23 | stage 누락, 중복 또는 순서 역전을 통과 | **PASS** — 전체 stage sequence, 유일성 및 주요 stage 순서를 확인한다. |
| 24 | track entry, gutter, collapsed flag 또는 finite base가 깨짐 | **PASS** — 기존 공통 matcher의 track topology와 수치 검사를 보존했다. |
| 25 | 남은 공간을 runtime trace와 무관하게 계산/하드코딩 | **PASS** — 실제 MaximizeTracks base sum으로 공간을 계산해 JSON expectation과 비교한다. |
| 26 | helper 분리 중 기존 Columns 대조군의 기대값이 변경 | **PASS** — 고정 Columns 호출부는 정적 expectation wrapper를 사용하고 기존 회귀·negative control이 통과한다. |
| 27 | Rust file 책임이 커져 유지보수가 어려움 | **PASS** — 공유 matcher, row fixture, JSON parser를 분리했고 각 파일은 500줄 이하이다. |
| 28 | 증거 문서가 테스트 결과, hash 또는 플랫폼 범위를 과장 | **PASS** — 6개 통과 로그, source hashes, 테스트 전용 범위와 미검증 simulator 구간을 연결한다. |
| 29 | PR 본문 라벨/링크/화면 요구가 변경 성격과 불일치 | **PASS** — `chore`, `area: layout`을 붙였고 UI 변경이 없어 screenshot이 없음을 명시한다. |
| 30 | CI 미보고 상태를 성공으로 표시하거나 병합 후 상태를 혼동 | **PASS** — PR view에는 check rollup이 없고 `gh pr checks`도 보고된 check가 없다고 반환했다. 이 결과를 CI 통과로 쓰지 않는다. |

## 판정

**30 PASS / 0 PARTIAL.** PR 전체 diff에서 수정할 결함을 찾지 못했다. PR merge 전 GitHub check rollup이 비어 있음을 확인했으며, merge 상태와 SHA는 병합 뒤 별도 확인한다. 이 PR은 test-oracle 보강이며 C11.4 공식 완료 판정이 아니다.
