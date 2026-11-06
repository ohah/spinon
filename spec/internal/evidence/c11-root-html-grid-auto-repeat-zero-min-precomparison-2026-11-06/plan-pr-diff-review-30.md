# PR #1215 C11.1 auto-repeat zero-min 계획 · PR diff 적대 검토 30개

초기 검토 대상은 PR #1215의 기능 계획·fixture·비교 evidence 11개 변경 파일(head `0a93b69bd92499d4b3eb89aa372b6a27bb097334`)과 본문 screenshot link다. 후속 검토에서 이 기록, 이미지 로드 기록, README와 manifest 변경도 확인했고, checksum·실제 이미지 표시 관련 관점은 항목 27·29에서 재검증했다. 각 항목은 별도 실패 경계다.

| # | 서로 다른 실패 관점 | 판정 |
|---:|---|---|
| 1 | `plan/c11-css-grid.md`가 PR #1179를 plan-only로 남겨 중복 구현을 유도할 수 있음 | PASS — 실제 병합 PR·시간·merge SHA로 문구를 바로잡았다. |
| 2 | #1179의 기록이 실제 auto-repeat implementation PR과 다를 수 있음 | PASS — linked plan은 PR #1179 및 merge SHA `7c042f4ac5080609927fb26760112c5a88a56d3c`를 기록한다. |
| 3 | 공식 상태 대장이 계획만으로 candidate를 완료 처리할 수 있음 | PASS — `spec/STATUS.md`는 바꾸지 않고 계획 PR임을 명시했다. |
| 4 | `78/214`가 계획 추가만으로 바뀔 수 있음 | PASS — counter와 C11.1/C11 status는 그대로라고 문서화했다. |
| 5 | C11.1 제한 slice가 C11 전체 지원처럼 해석될 수 있음 | PASS — parent completion과 제외 범위를 분명히 남겼다. |
| 6 | C11.1 auto-repeat grammar가 definite `0px` minimum을 배제한다고 잘못 처리할 수 있음 | PASS — fixed-size grammar 근거와 현재 positive-only 경계를 연결했다. |
| 7 | 1px floor 권고를 CSS 명세의 의무값으로 설명할 수 있음 | PASS — UA 선택값이며 1px은 suggested라고 구분했다. |
| 8 | count floor를 actual track minimum으로 적용해 `minmax(0px,1fr)` 의미를 바꿀 수 있음 | PASS — sizing minimum 0px을 보존한다고 계획과 expected data에 분리했다. |
| 9 | 두 track repeat pattern의 internal gutter를 누락할 수 있음 | PASS — formula의 `P × G`로 repeated pattern track 수를 반영한다. |
| 10 | 반복 경계 gutter를 중복 계산하거나 빼먹을 수 있음 | PASS — formula numerator `A + G`가 전체 gutter inequality에서 도출된다고 설명한다. |
| 11 | container가 한 track minimum보다 작을 때 repetition 0을 반환할 수 있음 | PASS — 최소 repetition 1 fallback을 독립 expected와 구현 acceptance에 둔다. |
| 12 | 0, finite non-negative px 외 percentage·calc·non-finite 값이 새 profile로 새어 들어갈 수 있음 | PASS — 기존 제한을 유지하고 그 입력은 fail closed하도록 한다. |
| 13 | 2개 repeat track으로 확장할 때 repetition 수는 limit 안이지만 expanded tracks 수가 넘을 수 있음 | PASS — `repetition × pattern length` 후행 quota 검증을 요구한다. |
| 14 | count 곱셈이나 free-space 덧셈이 overflow한 뒤 partial frame이 생길 수 있음 | PASS — overflow-safe preflight와 atomic rejection을 acceptance로 요구한다. |
| 15 | auto-fill의 빈 track이 auto-fit처럼 collapse될 수 있음 | PASS — auto-fill track 보존 장면을 따로 둔다. |
| 16 | auto-fit이 placement 전에 비어 있다고 판정해 occupied cell을 collapse할 수 있음 | PASS — placement 뒤 collapse 순서를 명세한다. |
| 17 | 모든 cell이 빈 auto-fit에서 gutter 또는 nonzero tracks가 남을 수 있음 | PASS — 11개 0px track expected를 별도 고정했다. |
| 18 | 점유 auto-fit에서 첫 track의 max track flex sizing이 달라질 수 있음 | PASS — single-track와 mixed pattern의 occupied output을 고정했다. |
| 19 | rows 구현이 columns와 다른 basis/gap을 사용해 축별 결과가 달라질 수 있음 | PASS — 두 rows scene과 별도의 container/gap 값이 있다. |
| 20 | 두 zero minimum을 가진 pattern이 한 track처럼 count될 수 있음 | PASS — 5 repetitions와 10 expanded tracks를 직접 검증한다. |
| 21 | zero/positive mixed pattern에서 count basis와 final sizes가 뒤섞일 수 있음 | PASS — count용 `[1,2]` basis와 used `[1,2]` values를 분리한다. |
| 22 | 0.5px gap 또는 1.125px track이 정수로 반올림돼 경계 수가 달라질 수 있음 | PASS — subpixel scene과 exact computed values를 고정했다. |
| 23 | expected track output이 Chrome 관찰에서 복사되어 독립 oracle이라고 오인될 수 있음 | PASS — expected JSON은 CSS rule과 독립 산식으로 별도 작성하고 Node 비교기가 두 파일을 읽는다. |
| 24 | 브라우저가 테스트한 버전과 계획 문서의 Chrome version이 다를 수 있음 | PASS — raw observation은 HeadlessChrome 157 UA를 기록하고 verifier가 version prefix를 검사한다. |
| 25 | syntax 지원 여부만으로 actual repeat count가 검증됐다고 볼 수 있음 | PASS — 10개 case에서 resolved track list와 item dimensions를 별도 비교했다. |
| 26 | evidence fixture가 제품 implementation fixture로 오인될 수 있음 | PASS — fixture는 `tests/fixtures`의 precomparison input이며 제품 지원 증거로 주장하지 않는다. |
| 27 | screen image 또는 JSON이 바뀌어도 해시가 따라오지 않을 수 있음 | PASS — fixture와 evidence 파일의 SHA256SUMS를 재생성하고 검증했다. |
| 28 | compare script가 입력 누락·중복 또는 track 수 어긋남을 놓칠 수 있음 | PASS — observation case count, ID, independent repeat count, expanded track count와 expected values를 확인한다. |
| 29 | PR 본문의 raw screenshot 경로가 잘못되어 이미지가 깨질 수 있음 | PASS — Agent Browser에서 immutable commit URL의 이미지가 complete였고 1280×3664로 로드됐다. 브라우저는 검증 후 닫혔다. |
| 30 | 계획 PR이 simulator, Android hardware, WPT, CI 또는 완료 상태까지 수행했다고 읽힐 수 있음 | PASS — 남은 범위에 구현 미시작·simulator 미검증·WPT 미검증·상태 미승격을 적었다. |

## 최종 판정

30개 관점을 검토했고 blocking defect는 찾지 못했다. 이 결과는 계획 PR diff 검토다. 제품 구현/runtime 20개 검토, 이후 구현 PR diff 검토 및 Android/iOS Simulator gate는 별도다. GitHub PR status rollup은 비어 있으므로 CI 성공으로 세지 않는다.
