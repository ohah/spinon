# Tailnet 로드맵 PR 변경 적대 검토 30개

검토 대상은 PR #349 이후의 로드맵 문구, 내부 색인, 선택 Rspress 빌드와 Tailnet sync 근거다. 앱 코드와 공식 C11.4 완료 판정은 변경하지 않는다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | 변경 브랜치가 #349 병합 commit에서 갈라지지 않음 | 브랜치 base는 `6867c2af5de4c3a953c2e846160334fd6d9590ee`로 기록했다. |
| 2 | PR 제목·범위가 앱 기능 구현처럼 보임 | 변경은 roadmap·색인·선택 Tailnet preview 증거로 제한했다. |
| 3 | 로드맵이 PR #349를 병합 전으로 표현함 | 현재 병합된 PR #349와 merge commit을 명시했다. |
| 4 | 10개 scene이 한 scene으로 축약됨 | 10-scene Ahem direct-leaf text 행렬임을 적었다. |
| 5 | Chrome·Android·iOS 실행 환경이 빠짐 | Chrome 155, Android API 37 ARM64 AVD, iOS 26.2 Simulator를 구분해 썼다. |
| 6 | frame 결과의 비교 범위가 모호함 | root/item geometry와 순서의 10/10 일치로 기술했다. |
| 7 | Columns/Rows trace byte 비교가 과장됨 | 의미상 10/10과 byte 동일 8/10을 각각 표시했다. |
| 8 | 두 trace 차이가 숨겨짐 | 세부 페이지가 `AutomaticMinimum` Debug 공백 차이 2건을 기록한다. |
| 9 | pixel 검증이 전체 픽셀 비교로 표현됨 | overflow scene의 표본 각 3/3으로 한정했다. |
| 10 | Metal drawable이 compositor 이후 화면으로 오해됨 | 상태·계획·evidence에서 compositor 이후 검증이 아님을 유지했다. |
| 11 | box geometry 캡처가 typography 확인으로 오해됨 | glyph paint와 text ink가 검증되지 않았다고 명시했다. |
| 12 | 일반 font·script 지원으로 확대됨 | 일반 font/script와 anonymous/nested item을 미완료 범위로 열거했다. |
| 13 | image intrinsic contribution 완료로 오해됨 | image contribution과 전체 matrix는 미완료라고 적었다. |
| 14 | WPT나 전체 C11.4 완료를 암시함 | WPT·전체 C11.4가 남았고 checkbox·완료 수를 유지한다. |
| 15 | 계획 문서가 최신 matrix를 누락함 | C11.4 계획에 10-scene 결과와 새 evidence 링크가 이미 들어 있음을 대조했다. |
| 16 | 내부 색인이 이전 Tailnet PR #347 상태만 가리킴 | 새 post-#349 sync evidence와 20·30개 review를 연결했다. |
| 17 | C11.4 실행 evidence route가 빌드에서 빠짐 | 생성된 `README.html`, review 페이지와 정적 첨부를 확인했다. |
| 18 | 상대 fixture 링크가 Tailnet URL에서 404임 | 실제 브라우저 404를 재현하고 GitHub fixture URL로 수정했다. |
| 19 | 상태 링크가 `/spinon/spec/STATUS.html`로 남음 | 실제 route `/spinon/STATUS.html`로 수정하고 화면에서 열었다. |
| 20 | 로드맵·상태·계획 페이지가 stale HTML을 제공함 | Tailnet URL과 Agent Browser 화면을 각각 확인했다. |
| 21 | 선택 build를 full-site build로 오해함 | Rspress 선택 build임을 기록하고 전체 build 미수행을 명시했다. |
| 22 | 문서 도구 버전이 달라짐 | Rspress 2.0.22를 유지했다. |
| 23 | 꺼진 link 검사에 의존함 | 정적 경로 inventory와 browser link 확인을 별도 실행했다. |
| 24 | preview 정적 자산의 상대 경로가 빠짐 | JS/CSS, matrix JSON·로그·PNG를 generated site에서 확인했다. |
| 25 | 사이트 내 파일 수와 복사 목록이 다름 | preflight와 copy report의 파일 집합·byte·SHA-256을 대조했다. |
| 26 | 기존 파일이 백업 없이 교체됨 | 모든 교체 파일을 전용 backup root에 복사하고 hash를 검증했다. |
| 27 | unrelated preview 파일이 삭제됨 | 삭제 작업을 하지 않았고 preflight에 no-delete를 기록했다. |
| 28 | HTTP 200만으로 본문을 검증함 | 제공 응답 byte 수와 SHA-256을 source와 비교했다. |
| 29 | Tailnet sync가 GitHub Pages 배포로 이어짐 | Pages workflow와 `gh-pages` 변경은 수행하지 않았다. |
| 30 | 브라우저나 listener 수명이 바뀜 | Agent Browser 종료를 확인하고 기존 preview listener·Tailscale serve를 유지했다. |

판정: 변경은 PR #349의 최신 결과를 공개 로드맵과 Tailnet 미리보기에 반영한다. 구현 상태를 완료로 승격하지 않는다.
