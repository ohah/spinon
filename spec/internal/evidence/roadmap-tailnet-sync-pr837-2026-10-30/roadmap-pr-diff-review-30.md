# PR #838 공개 로드맵 변경 적대 검토 30개

대상 PR [#838](https://github.com/ohah/spinon/pull/838), base `12c11815165077d570d4d9c1674b0785047cba67`, 기능 문구 검토 시점 head `b60ca4964`. `spec/public/roadmap.html`의 새 progress card와 이 검토 기록을 대조했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 새 항목 날짜가 PR merge보다 앞선다 | **통과.** 2026-10-30 날짜와 merge 시각 03:35:31Z를 표기한다. |
| 2 | 링크가 PR #837 이외의 작업을 가리킨다 | **통과.** GitHub 링크는 PR #837이다. |
| 3 | merge SHA가 실제와 다르다 | **통과.** `12c11815165077d570d4d9c1674b0785047cba67`은 확인된 merge commit이다. |
| 4 | 구현 완료처럼 보이도록 제목을 붙인다 | **통과.** 카드에 `test-only`라고 명시한다. |
| 5 | C11.4 공식 체크를 완료 처리한다 | **통과.** C11.4 미완료를 적고 status 원본 체크는 바꾸지 않았다. |
| 6 | 상위 C11을 완료 처리한다 | **통과.** C11 미완료를 적는다. |
| 7 | 전체 공식 완료 수가 증가한 것처럼 쓴다 | **통과.** `72/213` 그대로임을 명시한다. |
| 8 | exact-fit/overflow basis를 잘못 기록한다 | **통과.** 15,002px과 15,005px을 명시한다. |
| 9 | 양 축 실행을 한 축으로 축소해 쓴다 | **통과.** Columns와 Rows를 함께 표기한다. |
| 10 | 입력 case 수가 틀리다 | **통과.** 두 basis × 두 축, 네 입력이다. |
| 11 | pattern 크기나 순서가 다르다 | **통과.** `[1px, 2px]` auto-fill pattern을 기록한다. |
| 12 | 10,000-track 내부 상한을 빠뜨리거나 오해시킨다 | **통과.** 10,000-track cap으로만 설명하고 W3C 보편 규칙이라고 주장하지 않는다. |
| 13 | source-order partial cut을 잘못 서술한다 | **통과.** source-order 절단을 명시하며 정확한 expected sequence는 계획과 evidence에 링크한다. |
| 14 | negative named-line 결과가 빠진다 | **통과.** negative named-line frame을 명시한다. |
| 15 | test count가 실제 로그와 맞지 않는다 | **통과.** focused 3/3, Grid 135/135, crate 314/314로 기록한다. |
| 16 | Clippy/rustfmt 범위를 workspace 전체로 확대한다 | **통과.** crate Clippy·rustfmt라고 한정한다. |
| 17 | 시뮬레이터 성공을 꾸며 낸다 | **통과.** 앱 runtime·simulator 미포함이다. |
| 18 | WPT 적합성을 암시한다 | **통과.** WPT 미포함을 명시한다. |
| 19 | 일반 author CSS 지원을 암시한다 | **통과.** 일반 author CSS admission 미포함을 명시한다. |
| 20 | API/제품 behavior가 바뀐 것으로 읽힌다 | **통과.** Rust test-only slice와 public API 범위를 분리한다. |
| 21 | 작업 계획 링크가 다른 C11.4 후보를 가리킨다 | **통과.** `/spinon/plan/c11-4-auto-repeat-pattern-ceiling.html`을 가리킨다. |
| 22 | 실행 근거 링크가 단일-track 사례를 가리킨다 | **통과.** 새 two-track precomparison/runtime 경로다. |
| 23 | 공식 상태 링크가 내부 경로 또는 다른 페이지를 가리킨다 | **통과.** `/spinon/STATUS.html`로 연결한다. |
| 24 | HTML 문단 구조가 잘못되어 앞/뒤 카드와 합쳐진다 | **통과.** 독립 `p.kanban-note.hero-progress` 요소다. |
| 25 | 주요 설명을 구조 태그 밖에 넣어 스타일·접근성을 깨뜨린다 | **통과.** 제목은 `strong`, 기술 용어는 `code`, 연결은 `a`로 둔다. |
| 26 | 링크 URL에 로컬 path나 미검증 localhost를 쓴다 | **통과.** 사이트 경로와 공식 GitHub PR만 사용한다. |
| 27 | 새 카드가 과거 항목 뒤로 들어가 최신 이력 순서가 거꾸로 된다 | **통과.** 현재 최신 #834 카드 앞에 배치했다. |
| 28 | 변경이 SVG나 상태 차트를 수정해 공식 숫자를 바꾼다 | **통과.** HTML progress 문단 한 개만 추가했고 차트·수치는 그대로다. |
| 29 | #834의 simulator evidence를 새 #837 결과로 잘못 재사용한다 | **통과.** 기존 #834 카드와 그 화면 링크를 그대로 두고 새 카드는 Rust trace라고 구분한다. |
| 30 | 새 문구가 status record와 달라 두 페이지가 상충한다 | **통과.** PR #837 이력, 72/213, C11/C11.4 미완료, test-only 한계가 맞다. |

**판정:** 30개 독립 관점을 검토했다. 변경은 로드맵 설명 문단 하나로 제한되고, 구현과 공식 완료 집계는 바꾸지 않는다. 최종 rendered page는 Tailnet 동기화 뒤 Agent Browser로 별도 확인한다.
