# C11.4 row-axis expected JSON 연결 구현 검토 20관점

**검토 범위:** checked-in `expected.json`을 test-only row-axis matcher의 stage oracle로 연결한 변경. 검토 기준 commit은 `d9b4bcae054af51585ced34147792c4f5941223d`이며, 세 Rust 파일의 SHA-256은 같은 디렉터리의 [validation record](./README.md)에 고정했다. 이 기록은 PR diff 검토와 별도다.

## 실패 관점

| # | 별도 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | JSON 파일이 compile-time test input과 분리되어 drift | **PASS** — `include_str!`로 checked-in 기대값을 읽어 runtime matcher까지 전달한다. |
| 2 | JSON 문법 오류나 잘못된 루트 형식 | **PASS** — JSON decode 실패는 원인 메시지와 함께 테스트를 실패시킨다. |
| 3 | 행 대신 열 기대값을 잘못 연결 | **PASS** — 최상위 `axis`가 정확히 `Rows`인지 확인한다. |
| 4 | track 수 또는 definite block-size 변경 | **PASS** — track 수 4, 상위 block-size 220px, maximize 입력 block-size를 각각 고정 검증한다. |
| 5 | scene 누락·추가로 검증 범위가 조용히 축소/확대 | **PASS** — JSON 결과 개수를 Rust case 수와 비교한다. |
| 6 | Rust fixture 안의 중복 scene ID | **PASS** — 기대 scene ID 집합의 중복을 별도로 실패시킨다. |
| 7 | JSON 안의 중복 scene ID | **PASS** — 결과 ID 중복을 별도로 실패시킨다. |
| 8 | 결과 ID나 작성 순서가 달라짐 | **PASS** — 네 ID와 배열 순서를 Rust fixture 순서와 정확히 비교한다. |
| 9 | forward/reversed HTML source order와 JSON 불일치 | **PASS** — 각 case의 실제 source-order item ID 배열과 비교한다. |
| 10 | contribution 맵의 item 누락·추가·높이 변경 | **PASS** — 정확한 item 수, 각 key, 각 높이를 fixture와 비교한다. |
| 11 | span label 오기·비표준 표기 | **PASS** — 양의 정수 `spanN` 표기만 허용하며 잘못된 형식·0·leading zero를 거부하는 테스트를 추가했다. |
| 12 | normative batch 순서가 fixture와 달라짐 | **PASS** — JSON에서 파싱한 batch 순서를 fixture의 고정 순서와 비교한다. |
| 13 | batch stage 이름·flexible-track 속성 변경 | **PASS** — 각 batch stage 문자열을 예상 span과 `crosses_flexible_track=false`까지 포함해 비교한다. |
| 14 | initialize 또는 intermediate snapshot 누락·배열 길이 오류 | **PASS** — stage 존재 여부, 네 track 배열 길이, base size와 growth limit을 검사한다. 잘못된 배열 길이 거부 테스트도 있다. |
| 15 | intrinsic sizing 완료 snapshot drift | **PASS** — JSON 배열을 기대값으로 사용해 실제 trace의 base/growth 배열과 비교한다. |
| 16 | maximize row sizes 또는 growth limit drift | **PASS** — JSON의 maximize 배열을 실제 stage와 비교하며 유리수 문자열도 처리한다. |
| 17 | 누락·중복·순서가 잘못된 실제 trace stage | **PASS** — 전체 stage sequence를 비교하고 주요 stage의 유일성과 순서를 확인한다. |
| 18 | track/gutter shape, 비유한 base size, 음수/불일치 free space | **PASS** — 기존 matcher의 track topology·유한 base·0 gutter 검사와 계산된 남은 공간 비교를 유지한다. |
| 19 | malformed/overflow rational 및 null sentinel 오해 | **PASS** — 유리수의 분모·유한 f32 범위를 검사하고 오류 입력 테스트를 추가했다. JSON `null`은 trace `None`으로만 비교하며 내부 +∞라고 단정하지 않는다. |
| 20 | 기존 column regression 또는 변경 범위 과장 | **PASS** — 기존 column trace와 negative-control 테스트가 함께 통과했다. 제품 layout, 앱 runtime, FFI는 수정하지 않았고 Chrome은 내부 stage oracle로 사용하지 않는다. |

## 결과

**20 PASS / 0 PARTIAL.** 이 검토는 stage-oracle JSON 연결의 test-only 구현을 대상으로 한다. 규범 산술의 독립성, Chrome final geometry 비교, 네 고정 scene 밖의 일반 CSS Grid 적합성은 별도 범위다. PR의 전체 diff는 병합 전에 별도의 30관점 검토가 필요하다.
