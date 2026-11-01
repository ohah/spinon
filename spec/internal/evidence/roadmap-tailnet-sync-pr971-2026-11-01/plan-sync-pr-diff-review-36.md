# Tailnet sync 후속 계획·근거 PR diff 적대 검토 30개 · 2026-11-01

## 검토 범위

현재 PR 변경 파일은 양 축 C11.3.3 계획의 병합·검증 상태 보정과 Tailnet 동기화 근거, Agent Browser 확인 JSON 및 캡처다. 이 검토는 코드/제품 동작의 재검증이 아니다. 기존 C11 공식 checkbox와 `72/213`을 유지하며, CI 결과나 전체 문서 링크 검증을 확대 주장하지 않는지 확인했다.

초안 대조에서 계획의 전체 `spinon-style-to-layout` 결과가 `275 passed`로 남은 점, Agent Browser 표본 경로의 파일명이 실제 `android-screen.png`/`ios-screen.png`와 다른 점, `/STATUS.html` 경로 접두가 실제 표기와 다른 점을 찾아 수정했다. 또한 원시 preflight JSON이 호스트명·Tailnet IP·개인 홈 경로를 포함하므로 PR에서 제외하고 필요한 검증 요약만 남겼다.

## 독립 검토 관점

| # | 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | 구현 PR SHA가 다른 병합에 연결됨 | PR #970 merge SHA `dd178e…b8fd`와 계획 문서 일치 |
| 2 | 상태 문서 PR SHA가 잘못됨 | PR #971 merge SHA `1a862a…6def`와 selected source HEAD 일치 |
| 3 | 계획이 #971을 아직 pending으로 표시 | pending 표현 제거, 병합 및 Tailnet pass01 기록 |
| 4 | PR 병합 시각의 timezone 혼선 | GitHub merge time을 UTC 표기로 기록 |
| 5 | 전체 구현 집계가 잘못 갱신됨 | 공식 `72/213` 유지 |
| 6 | C11 완료 여부가 잘못 표시됨 | C11 미완료를 명시하고 checkbox 변경 없음 |
| 7 | C11.3.3 전체와 제한 fixture slice가 혼동됨 | 제한 slice 병합을 전체 C11.3.3/C11 완료로 확대하지 않음 |
| 8 | unrelated suite failure가 누락됨 | 최신 기록의 276 passed·1 unrelated failed로 일치 |
| 9 | 테스트 집계가 전체 성공으로 오해됨 | 1개 실패와 비통과 해석을 그대로 둠 |
| 10 | Android simulator가 실기기로 서술됨 | API 37 AVD/SwiftShader 한계를 명시 |
| 11 | iOS drawable 표본이 compositor 이후 결과로 서술됨 | compositor 제출 전 한계를 명시 |
| 12 | GitHub check rollup 부재가 CI 성공으로 취급됨 | 두 PR의 빈 rollup을 성공으로 세지 않는다고 기록 |
| 13 | publication 입력에 미추적 workspace 파일이 섞임 | 선택 입력은 Git index 기반 149 tracked files라고 기록 |
| 14 | route manifest 수가 불일치함 | selected publication과 summary 모두 224 routes |
| 15 | publication byte 수가 틀림 | 두 보고서 모두 13,391,160 bytes |
| 16 | 새 파일·교체·동일 route 합계가 틀림 | 195 + 11 + 18 = 224 |
| 17 | 기존 파일 백업이 누락됨 | 11 replacements와 11 backups를 함께 기록 |
| 18 | 백업 보존을 hash 검증 없이 주장함 | summary는 backup hash 검증 결과를 별도 boolean으로 기록 |
| 19 | preview 파일 삭제가 발생했는데 숨겨짐 | `deletedFiles: 0`, `noDeletes: true` 일치 |
| 20 | 일부 로컬 route가 선택 hash와 다름 | 224개 local target 전체 일치 기록 |
| 21 | 일부 HTTPS route가 실패함 | 224개 전체 HTTP 200 기록 |
| 22 | 서버 응답 내용 hash가 다름 | served hash 전체 일치를 별도 기록 |
| 23 | preview server를 재시작했다고 오기 | 재시작하지 않았다고 기록 |
| 24 | 전체 docs dead-link 확인을 완료로 과장 | 검사하지 않았다고 README와 JSON에 명시 |
| 25 | roadmap 내부 링크 수가 전체 docs 링크 수로 오해됨 | 11개 선택 링크만 확인했다고 한정 |
| 26 | roadmap hash가 게시 결과와 다름 | 474,950 bytes와 SHA-256이 publication manifest와 일치 |
| 27 | screenshot이 repository에 존재하지 않음 | 상대 링크 대상 PNG가 있고 크기 1280×633 |
| 28 | Android screenshot route가 틀림 | 선택 manifest의 `android-screen.png` 경로와 일치 |
| 29 | iOS screenshot route가 틀림 | 선택 manifest의 `ios-screen.png` 경로와 일치 |
| 30 | image dimensions가 simulator artifact와 불일치 | Android 1080×2400, iOS 1206×2622로 기록 |
| 31 | 상태 페이지 경로의 접두가 틀림 | `/spinon/STATUS.html`로 정정 |
| 32 | Agent Browser 검사를 terminal-browser로 보고 | 도구 이름과 세션 `spinon-tailnet-pr971`을 명시 |
| 33 | browser 종료가 확인되지 않음 | close 성공 및 post-close process count 0을 기록 |
| 34 | preview process 종료를 브라우저 종료와 혼동 | 기존 Tailnet preview server는 계속 동작했다고 분리 기록 |
| 35 | local host/IP 정보가 공개 PR에 노출됨 | 원시 preflight report를 제외하고 필요한 요약만 커밋 대상에 포함 |
| 36 | 변경으로 공개 CSS/Grid 지원이 선언됨 | public admission/WPT/full Grid 범위가 아니라는 제한 유지 |

## 판정

36개 관점에서 문서 diff와 연결된 manifest를 대조했다. 위에서 고친 세부 경로·집계·개인 환경 정보 외에 남은 불일치는 찾지 못했다. 전체 docs 링크 검사 및 CI 성공은 주장하지 않는다.
