# C11.4 auto-repeat public roadmap PR diff review · 30개

대상은 `spec/public/roadmap.html`의 PR #825 기록 단락과 이 검토표다. PR #825 merge SHA·시각은 GitHub의 병합 결과와 대조했다. 아래 각 항목은 서로 다른 실패 가능성을 현재 변경에 적용했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 잘못된 PR 번호를 새 slice에 연결 | PR #825로 기록했고 제목 및 계획·근거의 링크와 일치한다. |
| 2 | Rebase and merge가 아닌 병합 방식을 표시 | GitHub merge 결과가 Rebase and merge이며 표기가 일치한다. |
| 3 | KST 날짜와 UTC 병합 시각을 혼동 | `2026-10-29T22:12:01Z`와 공개 항목 날짜 `2026-10-30`은 UTC/KST 경계를 반영한다. |
| 4 | 다른 병합 SHA를 표시 | `a252cd2b60d7b8cca699d484e3aa525b5dc0c942`가 PR merge SHA와 일치한다. |
| 5 | 시험 입력의 세 경계값을 누락 | 9,999·10,000·10,001px를 모두 적었다. |
| 6 | 입력 basis를 실제 track 개수처럼 적음 | 요청값과 결과값을 각각 표시해 10,001 요청의 10,000 cap을 구분했다. |
| 7 | Columns와 Rows 한 축만 검사한 것처럼 축약 | 두 축을 모두 명시했다. |
| 8 | `auto-fill`과 `auto-fit`을 동일 처리한다고 암시 | 두 mode를 병기하고 빈 `auto-fit` collapse를 분명히 했다. |
| 9 | 빈 `auto-fit`이 track 개수를 줄인다고 오해 | 문구는 collapse 여부와 track count 상한을 혼동하지 않는다. |
| 10 | prefix 보존 또는 suffix 절단을 빠뜨림 | 상한에서 prefix 보존·suffix 절단을 명시했다. |
| 11 | 16개 경계 입력의 근거가 없는 숫자 | 12개 matrix 입력과 4개 source-order control의 합과 일치한다. |
| 12 | 집중 테스트 개수를 경계 입력 수로 혼동 | 집중 테스트 2개와 내부 16개 입력을 별도로 기술했다. |
| 13 | Grid 선택 테스트 결과를 과장 | 134개는 evidence log와 일치한다. |
| 14 | crate 전체 테스트 결과를 과장 | 312개는 evidence log와 일치한다. |
| 15 | Clippy 결과가 다른 crate/옵션에서 나온 것처럼 표시 | 근거 README의 `spinon-layout --all-targets -D warnings` 결과만 요약했다. |
| 16 | 실행하지 않은 Android/iOS 화면이 있는 것처럼 암시 | test-only trace라고 하고 앱 screenshot이 이 slice의 결과가 아님을 밝혔다. |
| 17 | Rust unit test가 일반 author CSS admission을 증명한다고 암시 | 일반 CSS admission은 확인하지 않았다고 명시했다. |
| 18 | 내부 trace가 전체 WPT 적합성을 증명한다고 암시 | 전체 WPT를 완료하지 않았다고 명시했다. |
| 19 | 한 경계 테스트를 C11.4 완료로 승격 | C11.4와 C11 완료를 뜻하지 않는다고 썼다. |
| 20 | 공식 완료 숫자를 73으로 잘못 증가 | 상태 대장과 같이 `72/213`을 유지한다. |
| 21 | 공식 상태 checklist를 완료로 오해 | C11.4를 미완료라고 표시하고 완료 checkbox를 주장하지 않는다. |
| 22 | PR check가 없는데 CI 성공이라고 보고 | check rollup이 비어 있고 CI 통과로 기록하지 않는다고 밝혔다. |
| 23 | 상세 계획 상대 route 누락 또는 오타 | `/spinon/plan/c11-4-auto-repeat-track-ceiling.html`의 source가 존재한다. |
| 24 | 계획 검토 route 누락 또는 오타 | 20개 계획 검토 Markdown source가 존재한다. |
| 25 | 실행 evidence route 누락 또는 오타 | C11.4 auto-repeat README source가 존재한다. |
| 26 | 구현/runtime 검토 route가 다른 evidence를 가리킴 | 같은 auto-repeat ceiling evidence 디렉터리의 20개 검토 문서로 연결한다. |
| 27 | 구현 PR diff 검토를 새 로드맵 PR 검토로 잘못 표시 | 구현 PR과 로드맵 PR의 30개 검토 링크를 별도로 표시한다. |
| 28 | 공개 상태 링크가 STATUS가 아닌 계획을 가리킴 | `/spinon/STATUS.html#css-구현-체크리스트`를 사용한다. |
| 29 | 새 기록이 관련 C11.4 이력에서 분리됨 | 최근 fit-content limit slice 뒤에 연속된 `<p>` 항목으로 삽입했다. |
| 30 | HTML 문단 경계나 링크 텍스트 삽입이 깨짐 | 기존 단락 경계 뒤에 닫힌 `<p>`를 추가했고, Agent Browser 후보 화면에서 새 문단의 제목·본문·링크를 확인해 캡처했다. Tailnet 최종 route 검증은 병합 후 별도로 한다. |

**결론:** 확인한 항목에서 사실 오류를 찾지 못했다. 실제 Tailnet 반영 및 최종 화면 확인은 병합된 `main`을 기준으로 별도로 수행한다.
