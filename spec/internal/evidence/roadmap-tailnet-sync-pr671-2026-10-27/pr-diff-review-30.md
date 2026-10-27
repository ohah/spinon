# 공개 로드맵 PR diff 적대적 검토 30개

검토 대상은 `spec/public/roadmap.html`의 #671 추가 항목, 이 evidence README와 Agent Browser 화면, 본 문서다. `spec/STATUS.md`와 제품 코드는 수정하지 않았다.

| # | 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | 최근 항목 순서가 병합 순서와 다른가 | #671 문단이 #669 문단 바로 뒤에 추가됐다. | PASS |
| 2 | PR 번호가 다른 계획 변경을 가리키는가 | 문단과 링크가 GitHub PR #671을 가리킨다. | PASS |
| 3 | merge 시각이 잘못됐는가 | GitHub PR metadata의 `2026-10-26T23:42:29Z`와 일치한다. | PASS |
| 4 | merge SHA가 원본 PR commit과 혼동됐는가 | GitHub merge commit `d33545c49cb9734b0099236adcc44d6dedc1dc3a`를 기록했다. | PASS |
| 5 | 명세 버전·절이 모호한가 | CSS Grid 2025 CRD §11.5를 항목에 명시했다. | PASS |
| 6 | 구현 항목처럼 표현했는가 | 제목에 `계획 PR`을 쓰고 본문에 계획·사전 비교라고 제한했다. | PASS |
| 7 | 앱 runtime 검증을 암시하는가 | Spinon runtime 및 Simulator 실행 근거가 아니라고 명시했다. | PASS |
| 8 | span 2 단독 기대값이 계획과 다른가 | 계획의 `[40,60,60,0]px`를 그대로 반영했다. | PASS |
| 9 | span 2·3 중첩 기대값이 계획과 다른가 | 정확 유리수 `[160/3,220/3,220/3,0]px`를 기록했다. | PASS |
| 10 | Chrome 값이 규범값으로 혼합됐는가 | Chrome 156 관찰을 규범 기대값과 구별한다고 썼다. | PASS |
| 11 | 비교의 stage scope를 과장했는가 | §11.5 단계별 기여 계획이라고 범위를 제한했다. | PASS |
| 12 | 계획의 fourth row 결과 설명이 누락돼 오해를 부르는가 | 이 짧은 로드맵 요약은 stage 전체를 대신하지 않고 상세 계획으로 연결한다. | PASS |
| 13 | 상위 C11.4를 완료 처리했는가 | 문단이 C11.4 미완료라고 명시한다. | PASS |
| 14 | 공식 집계가 증가했는가 | 문단은 `72/213`을 유지한다고 명시한다. | PASS |
| 15 | 구현 상태 원본에 연결하는가 | 공식 상태 HTML anchor 링크를 포함한다. | PASS |
| 16 | 계획 링크가 존재하지 않는 route인가 | `plan/c11-4-row-axis-span-stage-oracle.md` 원본이 존재한다. | PASS |
| 17 | 계획 검토 링크가 누락됐는가 | 별도 계획 review 20 문서가 존재하고 연결됐다. | PASS |
| 18 | 이번 계획 PR 검토 링크가 잘못됐는가 | 계획 전용 PR diff review 30 문서가 존재하고 연결됐다. | PASS |
| 19 | 사전 비교 링크가 없는 원본을 가리키는가 | precomparison README 원본이 존재한다. | PASS |
| 20 | 화면 이미지 링크가 오타인가 | PNG 원본이 evidence 디렉터리에 존재한다. | PASS |
| 21 | 외부 PR 링크가 잘못됐는가 | 링크 host와 repository는 `github.com/ohah/spinon/pull/671`이다. | PASS |
| 22 | 이미지가 runtime 화면처럼 오해되는가 | 이미지 링크 및 설명은 Chrome 156 사전 비교 화면으로 표시한다. | PASS |
| 23 | HTML 마크업이 내용 때문에 깨지는가 | inline code와 링크의 태그 경계를 확인했고 HTML 파서 렌더링이 성공했다. | PASS |
| 24 | 기존 스타일과 문단 class가 다르게 쓰였는가 | 인접한 날짜별 항목과 같은 `kanban-note hero-progress` class를 쓴다. | PASS |
| 25 | 기존 #669 내용이 바뀌었는가 | diff는 기존 HTML 한 줄의 끝에 새 문단만 추가한다. | PASS |
| 26 | 같은 #671 문단이 중복됐는가 | 새 marker는 원본에서 한 번만 나타난다. | PASS |
| 27 | 버전 있는 API 또는 계약 변경을 주장하는가 | API 변경을 주장하지 않고 기존 계획 문서로 연결한다. | PASS |
| 28 | Android·iOS·WPT·기기 검증을 확대 주장하는가 | 해당 검증을 주장하지 않으며 runtime 비근거를 적었다. | PASS |
| 29 | 캡처가 새 항목을 실제 보여주는가 | Agent Browser 화면에서 #669와 이어진 #671 문단 및 `72/213`을 확인했다. | PASS |
| 30 | 화면 캡처가 잘못된 해상도 또는 파일인가 | PNG는 1280×633이며 기록된 SHA-256과 일치한다. | PASS |
