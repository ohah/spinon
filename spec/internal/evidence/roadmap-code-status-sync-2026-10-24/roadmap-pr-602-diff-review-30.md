# PR #602 로드맵 동기화 diff 실패 관점 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 C11.4 계획-only 문단이다. 계획 자체 검토 및 계획 PR 검토와 별도로, 공개 로드맵에서 상태·링크·증거의 의미가 틀리거나 기존 항목을 훼손하는 30개 실패 경로를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 병합 전에 PR을 병합 완료로 표시 | GitHub PR #602의 live state가 `MERGED`이고 merge time은 `2026-10-24T13:52:35Z`다. |
| 2 | 다른 revision의 merge SHA를 표시 | 로컬 `main`과 `origin/main`이 `6a70d3e97613f71dc74852ca275d8020138c1f1a`이며 PR merge SHA와 같다. |
| 3 | 기능 구현 PR처럼 오인 | 항목 제목과 본문에 `계획`을 표시하고 계획-only·구현 미시작을 명시했다. |
| 4 | 계획 병합만으로 C11.4 완료 처리 | C11.4 미완료 및 checkbox 유지 문구를 기록했다. |
| 5 | 전체 공식 완료 수 증가를 잘못 암시 | 현재 공식 수 `72/213`과 동일하게 표시한다. |
| 6 | 범위가 계획 문서와 다름 | 같은 axis의 fixed-repeat + auto-repeat + `grid-template-areas` 조합이라는 제한을 계획 제목·계약과 대조했다. |
| 7 | 지원 grammar를 넓게 주장 | 임의 CSS admission이 아니라 제한 slice 계획이라고 썼다. |
| 8 | Chrome 관찰을 Spinon 동작으로 승격 | Chrome 156을 사전 비교 oracle로 명시하고 Spinon 구현 결과가 아니라고 적었다. |
| 9 | simulator 증거가 있는 것처럼 표현 | simulator 결과가 없다고 직접 표시했다. |
| 10 | 9 scene 수치가 fixture와 불일치 | 고정 fixture와 observation의 scene ID를 대조해 9개임을 확인했다. |
| 11 | 30 child frame 수치가 observation과 불일치 | precomparison validator 결과 `9 scenes, 30 child boxes`와 일치한다. |
| 12 | independent expected values를 Chrome에서 역산 | README는 CSS Grid 2025 CRD 계산과 Chrome 교차 관찰을 분리한다. |
| 13 | rows/columns transpose 누락 | 두 축 장면이 고정된 9-scene inventory에 포함된다. |
| 14 | `auto-fill`/`auto-fit` 범위 누락 | 양 repeat mode와 empty/occupied auto-fit 경계를 README에서 확인했다. |
| 15 | area/template dimension 경계 누락 | expanded template보다 크거나 작은 matrix cases가 포함된다. |
| 16 | named-line 충돌을 지원 완료로 주장 | collision은 precomparison 기준 사례이며 구현 완료 주장은 없다. |
| 17 | overflow 수치를 과장 | 20px은 root-relative geometry 결과이며 border pixel 통과를 주장하지 않는다. |
| 18 | 계획 PR 링크가 다른 PR을 가리킴 | 외부 PR URL은 #602다. |
| 19 | 계획 페이지 route 오탈자 | 링크 target `/spinon/plan/c11-4-fixed-auto-repeat-template-areas.html`가 새 문서 stem과 일치한다. |
| 20 | Chrome README 링크 route 누락 | evidence README 경로와 새 precomparison 디렉터리를 대조했다. |
| 21 | PNG 링크 route 누락 | canonical `chrome156-fixed-auto-repeat-areas.png` 경로를 대조했다. |
| 22 | review 링크가 없는 파일을 가리킴 | `plan-pr-diff-review-30.md`가 source tree에 존재한다. |
| 23 | 공식 상태 링크가 오래된 heading fragment | 현재 RSPress `STATUS.html` route의 `css-구현-체크리스트` fragment를 대조했다. |
| 24 | base path `/spinon/` 누락 | 내부 페이지 URL은 기존 로드맵과 같은 `/spinon/` prefix를 쓴다. |
| 25 | HTML tag nesting 오류 | 추가 문단의 `<p>`, `<strong>`, `<a>`, `<code>` 태그가 닫힌다. |
| 26 | 텍스트가 코드 영역으로 잘못 해석 | inline literals만 `<code>`로 감쌌고 인용문·raw markup은 추가하지 않았다. |
| 27 | 링크 이름이 접근 불가능하거나 모호 | 계획, Chrome 사전 비교, Chrome 화면, PR 검토, 공식 상태에 설명 텍스트가 있다. |
| 28 | 일자순 문맥에서 항목이 숨겨짐 | 2026-10-24 #600 fixed-repeat named-lines 항목 바로 뒤, 다른 활성 백로그보다 앞에 둔다. |
| 29 | 기존 C13 비활성 backlog를 덮거나 활성화 | 기존 C13 문단은 동일 내용으로 다음 문단에 보존했다. |
| 30 | 변경이 로드맵 밖 파일까지 포함 | 제품 소스 수정은 로드맵 HTML의 새 문단 한 곳뿐이다. 나머지는 이 검토 기록과 Agent Browser의 화면·검증 근거다. |

## 결과

현재 diff는 계획 PR #602와 공식 `72/213`을 정확히 연결하며 구현/runtime 완료를 주장하지 않는다. Tailnet 반영은 이 PR의 최신 main build를 선택 경로로 별도 hash 검증한 뒤 확인한다.
