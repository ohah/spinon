# Public roadmap C11 plan-card PR diff review · 30 perspectives

**대상:** PR 변경분 `spec/public/roadmap.html`과 이 evidence directory. 제품 코드·지원 상태 변경은 없다.

| # | 독립 실패 관점 | 확인 기준 | 판정 |
|---:|---|---|---|
| 1 | 아직 병합되지 않은 계획을 병합 완료로 소개한다 | 카드의 PR 번호·merge 상태·시각·SHA가 병합된 #1054 메타데이터와 일치해야 한다. | 통과 |
| 2 | 계획을 구현 완료로 표시한다 | 본문은 구현과 runtime 검증이 시작되지 않았다고 명시해야 한다. | 통과 |
| 3 | 계획 PR로 C11.1 또는 C11 체크를 승격한다 | 카드에 두 상태가 미완료라고 표시하고 `spec/STATUS.md`는 수정하지 않는다. | 통과 |
| 4 | 계획 병합으로 공식 완료 수가 증가한 것처럼 보인다 | 카드가 공식 `72/213`을 유지한다고 적어야 한다. | 통과 |
| 5 | column flow의 구현 경계가 왜 필요한지 빠진다 | cascade는 값을 보존하지만 projection/layout은 거부한다고 설명해야 한다. | 통과 |
| 6 | 제한 범위가 일반 CSS Grid 지원처럼 보인다 | 명시 2×2 grid의 sparse column auto-placement만 계획한다고 한정한다. | 통과 |
| 7 | dense flow를 같은 지원으로 오해한다 | `sparse`를 카드에 남기고 dense 구현을 주장하지 않는다. | 통과 |
| 8 | implicit track 구현까지 포함한 듯 보인다 | 카드와 링크된 계획은 4-cell capacity 안으로 범위를 제한한다. | 통과 |
| 9 | Chrome 관측이 Spinon runtime 증거로 읽힌다 | 링크 이름은 “사전 비교”이고 본문은 runtime 검증 전이라고 구분한다. | 통과 |
| 10 | 고정 기대값과 지원 범위를 혼동한다 | 2×2는 계획 fixture shape이고 공개 CSS admission 주장이 아니다. | 통과 |
| 11 | 계획 링크가 base path에서 깨진다 | `/spinon/plan/c11-root-html-grid-column-flow.html` route를 생성한다. | 통과 |
| 12 | precomparison README 링크가 잘못된 prefix를 쓴다 | `/spinon/internal/evidence/.../README.html`의 RSPress route를 쓴다. | 통과 |
| 13 | 계획 검토 링크가 원본 Markdown만 가리킨다 | `.html` route가 생성될 selected root에 plan review 파일을 둔다. | 통과 |
| 14 | 계획 PR diff 검토 링크가 다른 작업을 가리킨다 | C11 column-flow plan의 정확한 review 파일을 링크한다. | 통과 |
| 15 | 동기화 증거 링크가 없는 파일을 가리킨다 | 이번 PR에서 README를 함께 추가하고 publication route에 포함한다. | 통과 |
| 16 | 카드가 기존 최신 C11 카드를 밀어내거나 삭제한다 | 새 카드만 첫 C11 카드 앞에 추가하고 뒤의 기존 HTML을 그대로 보존한다. | 통과 |
| 17 | unrelated roadmap sections가 수정된다 | diff가 roadmap 카드 삽입 및 그 주변 줄바꿈으로 한정된다. | 통과 |
| 18 | kanban 스타일을 임의로 깨뜨린다 | 기존 `kanban-note hero-progress` class를 그대로 쓴다. | 통과 |
| 19 | 카드가 문서 의미 흐름 밖에 놓인다 | 새 문단은 현재 작업 hero progress 묶음의 첫 C11 업데이트로 둔다. | 통과 |
| 20 | 링크 텍스트가 목적을 알리지 않는다 | 계획·사전 비교·20개 계획 검토·30개 PR 검토·동기화 근거를 명시한다. | 통과 |
| 21 | raw HTML attribute quoting이 손상된다 | `href`와 `code` 속성의 따옴표가 유효한 HTML 형태인지 검사한다. | 통과 |
| 22 | CSS pseudo value가 본문에서 오독된다 | `grid-auto-flow:column`은 code로 표시하고 지원 문장과 분리한다. | 통과 |
| 23 | merge metadata가 다른 merge를 가리킨다 | #1054 `mergedAt`과 merge commit은 GitHub·현재 HEAD 값과 일치해야 한다. | 통과 |
| 24 | 날짜가 잘못되어 최신순 정렬을 흐린다 | 카드 날짜는 #1054 merge의 현지 작업일 `2026-11-02`로 표시한다. | 통과 |
| 25 | plan review 수를 runtime review로 오인한다 | 링크와 텍스트는 계획 검토라고 명명하고 runtime 검토를 주장하지 않는다. | 통과 |
| 26 | 새 카드가 모바일 폭에서 고정 폭을 강제한다 | Agent Browser 390×844에서 card right edge 376px, `scrollWidth == clientWidth == 390px`를 확인하고 새 width rule은 추가하지 않았다. | 통과 |
| 27 | 외부 PR 링크가 엉뚱한 저장소를 향한다 | 링크는 `ohah/spinon/pull/1054`를 사용한다. | 통과 |
| 28 | evidence README가 동작 검증으로 과장된다 | README는 선택 문서 빌드와 Agent Browser 화면만 기록하고 기능 runtime 통과를 주장하지 않는다. | 통과 |
| 29 | public status 본문과 카드 수치가 충돌한다 | publication에서 상태 route가 `72/213` 및 C11 incomplete를 유지하는지 확인한다. | 통과 |
| 30 | selected build 또는 Agent Browser 검증 실패를 성공으로 기록한다 | 고정 Rspress 2.0.22 build와 실제 local browser 관찰만 기록한다. Tailnet HTTPS 복제 결과는 post-merge sync 단계로 분리한다. | 통과 |

이번 검토는 PR diff의 내용·링크·표현을 대상으로 한다. Agent Browser 화면은 이 PR의 pre-merge selected-site 후보에서 확인했다. Tailnet HTTPS route 검증은 병합 뒤 별도 sync 근거에 기록한다.
