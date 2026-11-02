# Tailnet 로드맵 공식 상태 집계 identity 복구

**범위·사전 비교·실행 근거:** [evidence README](../spec/internal/evidence/roadmap-status-counter-identity-2026-11-03/README.md) · **계획 실패 관점 검토:** [20개](./roadmap-status-counter-identity-plan-review-20.md) · **구현 실패 관점 검토:** [20개](../spec/internal/evidence/roadmap-status-counter-identity-2026-11-03/implementation-review-20.md) · **PR diff 적대 검토:** [30개](../spec/internal/evidence/roadmap-status-counter-identity-2026-11-03/pr-diff-review-30.md)

## 현행 결함

Tailnet `/spinon/roadmap.html`은 현재 `213개 표시 · 공식 완료 72/213`을 보여 준다. 공식 원본 [상태 대장](../spec/STATUS.md)을 ID와 제목으로 대조하면 완료 체크 행은 78개다. 페이지는 상태 Map을 ID 단독으로 구성해 서로 다른 제목의 중복 `C04.12` 행이 마지막 `[ ]` 값으로 덮어쓰인다. 또한 ID 문법이 숫자 점 번호로 끝난다고 가정해 `C06.6a`·`C06.6b`를 읽지 못한다. 현재 DOM에서 C04.12 7개가 모두 미완료로 보이고 C06.6 하위 두 ID는 빠져 있다.

## 기대 동작

- `spec/STATUS.md`는 상태의 유일한 원본이며 화면은 읽기 전용이다.
- ID를 영문자 접미까지 읽고, 공식 task row마다 `ID + Markdown 정규화 제목` key를 부여한다. 같은 ID를 가진 서로 다른 행은 각자의 체크 상태와 고유 DOM id를 갖는다.
- base task는 먼저 ID로 원본 행을 찾는다. ID가 한 번만 나오면 코드의 요약 제목을 허용하고 그 row status를 base UI key에 연결한다. ID가 반복되면 정규화 제목이 같은 원본 행 하나만 허용한다. 누락·모호·중복 identity는 전체 sync 실패로 표시한다.
- 상세 stable ID 중 base row에 대응하지 않는 행과 기존 `미구현 하위 항목` 두 행은 각각 한 번씩 표시한다. 기준 대장에서는 총 215개 표시 row, 78개 완료가 기대된다. 이 수는 코드의 고정 상수로 두지 않고 파싱 결과에서 계산한다.
- 검색, 전체/미완료 필터, 칸반 상태와 접근성 이름이 새 identity key를 사용해 일치해야 한다.
- 공식 checkboxes, WIP 범위와 개인 브라우저 저장 상태는 수정하지 않는다.

## 재현·판정 자료

수정 전에는 Agent Browser 기준 URL에서 counter, status note, 전체 checked 수와 C04.12/C06.6 행을 저장했다. 세부 값은 [사전 비교](../spec/internal/evidence/roadmap-status-counter-identity-2026-11-03/README.md)와 JSON·PNG에 있다. 독립 기준은 같은 시점의 `spec/STATUS.md` ID/title/check 값이다.

## 구현 후 확인

1. status parser가 213개 ID row와 2개 지원 중단 child row를 distinct key로 읽고, base row 78개를 ID별 uniqueness 또는 duplicate ID의 제목 일치 규칙으로 연결하는지 확인한다.
2. duplicate ID, alphabetic suffix, duplicate identity, missing base identity, malformed ID-row marker, malformed no-ID child marker, fetch/HTTP/timeout failure를 positive/negative case로 검증한다. ID형 체크 행 marker 수와 파싱된 stable row 수, 특수 하위 행 marker 수와 파싱된 child row 수가 각각 같아야 한다. 공식 원본에서 행 자체가 삭제된 경우에는 원본 변경으로 취급해 파싱 결과에서 denominator를 다시 계산한다.
3. Agent Browser에서 수정 전과 동일한 페이지 입력을 로드해 총 215개·완료 78개, C04.12 제목별 원본 상태, C06.6a/b 표시, search/open filter와 칸반 상태를 확인한다.
4. desktop 및 mobile viewport에서 가로 overflow, 잘림, 중복 DOM id를 검사한다. 사용한 Agent Browser 프로세스는 확인 뒤 종료한다.
5. PR 전 실제 변경을 서로 다른 실패 경로 20개로 검토하고, 저장소 PR gate인 별도 patch 검토 30개를 수행한다. 결함 수정 뒤 관련 브라우저 확인과 검토 항목을 다시 실행한다.
6. 병합 후 GitHub `main`의 served `spec/STATUS.md`, Tailnet page hash·counter·화면을 Agent Browser로 재확인한다. 체크된 기능 항목은 추가하지 않는다.

이 변경은 공식 구현 상태를 승격하지 않으며 Tailnet 표시 결함만 수정한다.
