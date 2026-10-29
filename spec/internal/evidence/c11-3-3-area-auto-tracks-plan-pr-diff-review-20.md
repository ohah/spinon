# C11.3.3 area implicit-track 계획 PR · 변경분 적대 검토 20개

**결과:** PASS 20/20. 계획·사전 비교 자료만 포함한 PR 변경분을 검토했다. 제품 코드 수정, simulator 실행 또는 C11.3.3 완료를 입증하는 검토가 아니다.

| # | 독립 변경 실패 관점 | 확인한 변경 근거 | 결과 |
|---:|---|---|---|
| 1 | follow-up 문서가 C11.3.3의 정식 후속 계획으로 연결되지 않는다 | `plan/c11-3-template-repeat-named-lines-areas.md`에 새 계획 상대 링크 추가를 확인했다 | PASS |
| 2 | 이전의 exact track-count 제한이 아직 유효 계약처럼 남는다 | 부모 계획에서 이전 구현 범위와 새 후속 범위를 구분했다 | PASS |
| 3 | area-generated tracks 때문에 기존 named line/general implicit placement도 완료 처리한다 | C11.3.1/.2의 일반 implicit line placement 제외를 명시했다 | PASS |
| 4 | 계획 문구가 C11.3.3 candidate나 C11 상위를 완료했다고 주장한다 | 계획 첫머리와 완료 조건에 candidate/상위 미완료를 유지했다 | PASS |
| 5 | 공식 완료 수가 부분 fixture 확장만으로 바뀐다 | 계획과 evidence 모두 `72/213` 유지 조건을 기록했다 | PASS |
| 6 | 규범 출처가 다른 초안 또는 불안정한 외부 설명을 가리킨다 | CSS Grid 2025 CRD §7.3, §7.3.2, §8.5 링크를 확인했다 | PASS |
| 7 | grid-template-areas 행/열의 생성 규칙이 구현 추정에만 의존한다 | W3C §7.3 설명과 Chrome 156 used-track 관측을 각각 제시했다 | PASS |
| 8 | missing row case가 missing column case로 잘못 매핑된다 | Chrome JSON에서 `short-row`/`short-col` used lists와 item frames를 대조했다 | PASS |
| 9 | `grid-auto-rows`와 `grid-auto-columns` 크기가 서로 바뀐다 | none/short-axis scenes의 `[7,7]`·`[9,9]` 결과를 확인했다 | PASS |
| 10 | `auto` track이 fixed-pixel 결과로 오인된다 | `auto` case의 content contribution `[10,20]`/`[20,30]`을 확인했다 | PASS |
| 11 | extra authored track이 area size에 맞춰 잘린다 | `long` case는 rows/columns 각 세 authored tracks를 그대로 보인다 | PASS |
| 12 | 기존 mismatch rejection의 oracle이 실제 기존 scene과 다르다 | area matrix, `40px 120px` columns, `60px 60px` rows, 0-sized final-area item을 일치시켰다 | PASS |
| 13 | mismatch scene의 0px track을 item overflow로 잘못 설명한다 | Chrome CSSOM은 `40px 120px 0px`, item frame은 x=160px·width=0px다 | PASS |
| 14 | scene count나 expected outcome 수가 산술상 맞지 않는다 | 기존 26 + 신규 5 = 31, node 65 + 25 = 90, success 25/fallback 5/rejection 1을 재계산했다 | PASS |
| 15 | 이전 26-scene evidence가 새 baseline으로 덮여 추적성을 잃는다 | 계획은 26-scene 원본을 별도로 보존하고 신규 31-scene baseline을 분리한다 | PASS |
| 16 | Screenshot/JSON이 다른 fixture revision을 가리킨다 | fixture, JSON, PNG SHA-256을 README와 실제 파일에서 재계산해 일치시켰다 | PASS |
| 17 | 계획이 지원 범위를 무심코 넓혀 auto-repeat/track-list grammar를 허용한다 | `TemplateGrid` profile, 기존 `auto`/single-px subset, auto-repeat 별도 경계를 읽었다 | PASS |
| 18 | simulator·artifact·CI 근거를 계획 PR에서 실행 완료처럼 표현한다 | 이 PR의 목적은 계획 고정이며 해당 gate는 구현 PR의 완료 조건으로 남겼다 | PASS |
| 19 | PR 20회 검토가 계획 실패 검토와 중복 집계된다 | 이 문서는 별도 plan-review-20과 다른 변경 파일·diff 경계를 검토한다 | PASS |
| 20 | 요청된 브라우저 실행 뒤 Agent Browser가 남는다 | 캡처 session을 닫고 일치하는 Chrome/Agent Browser process가 없는지 확인했다 | PASS |

## 실행 결과

- `git diff --check origin/main...HEAD`: 통과.
- 새 계획·사전 비교 문서에 추가된 local 상대 링크와 fixture/JSON/PNG SHA-256 대조: 통과.
- Chrome 156 관측 JSON: 6 scene parse 및 지정된 track-count assertions 통과.
- Android/iOS, Rust test, Clippy와 GitHub PR checks: 계획 PR 범위에서 실행하지 않았으며 구현 PR 관문으로 남긴다.

부모 C11.3 문서에는 이번 diff와 무관한 과거 증거 경로 3개가 로컬 checkout에 존재하지 않는다. 그 경로는 수정하지 않았고 새로 추가된 계획 링크는 실제 target을 확인했다.
