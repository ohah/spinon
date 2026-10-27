# PR #711 diff 검토 30개 · 2026-10-28

검토 범위는 PR #710 병합 뒤 상태 대장·집중 계획·공개 로드맵 변경과 Tailnet 게시 근거 파일이다. PR #710 구현 코드는 이 PR의 변경 범위가 아니다. 각 항목은 서로 다른 오류 경로나 불변 조건을 확인했다.

| # | 검토 관점 | 결과와 근거 |
|---:|---|---|
| 1 | PR #710의 실제 병합 상태 | GitHub에서 `MERGED`, Rebase merge SHA `58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94`로 확인했다. |
| 2 | 병합 시간의 정확성 | GitHub `mergedAt`은 `2026-10-27T20:00:08Z`; STATUS와 계획·로드맵이 같다. |
| 3 | 상태 대장의 삽입 위치 | C11.4 기존 기록의 날짜 순서에서 #706 기록 다음에 새 slice를 기록했다. |
| 4 | 공식 완료 수 보존 | 변경된 C11.4 문장과 로드맵 항목 모두 `72/213`이다. |
| 5 | 상위 C11.4 체크 보존 | diff에 완료 체크나 상위 체크 변경이 없다. 항목은 미완료로 명시한다. |
| 6 | 제한 fixture와 제품 지원의 구분 | 새 항목은 opt-in fixture로 한정하고 일반 CSS admission을 검증하지 않았다고 썼다. |
| 7 | CI 상태의 과장 방지 | PR check rollup이 빈 배열이고 main required checks 조회가 404였다. 어떤 문서도 CI 통과라고 주장하지 않는다. |
| 8 | merge commit과 source commit의 혼동 | 문서는 병합 SHA를 명시하고, Tailnet 게시 보고서는 게시 입력인 docs source commit `610fa07c`를 별도로 기록한다. |
| 9 | PR 링크의 목적지 | 로드맵의 구현 링크는 실제 GitHub PR #710 URL을 가리킨다. |
| 10 | 집중 계획 상대 경로 | 로드맵 집중 계획 링크가 `/spinon/plan/c11-4-generic-mixed-direct-text-image-intrinsic.html`에서 열렸다. |
| 11 | 실행 근거 상대 경로 | 로드맵 실행 근거 링크가 새 C11.4 evidence README로 열렸다. |
| 12 | 공식 상태 링크와 fragment | 상태 대장 링크가 `STATUS.html#css-구현-체크리스트`로 열렸고 페이지 내 C11.4와 `72/213`을 확인했다. |
| 13 | Android 이미지의 실제 게시 경로 | 로드맵의 href를 그대로 열어 PNG가 로드됐다. manifest의 accepted AVD run 경로와 일치한다. |
| 14 | iOS 이미지의 실제 게시 경로 | 로드맵의 href를 그대로 열어 PNG가 로드됐다. manifest의 accepted Simulator run 경로와 일치한다. |
| 15 | Android 이미지 치수 | Agent Browser가 `1080×2400` 이미지를 열었고 DOM natural size가 일치한다. |
| 16 | iOS 이미지 치수 | Agent Browser가 `1206×2622` 이미지를 열었고 DOM natural size가 일치한다. |
| 17 | 이미지 문서 404 탐지 | 첫 수동 주소 조립은 `/runs/`를 빼 404였다. 실제 로드맵 href에서 경로를 다시 가져와 둘 다 성공했다. 최종 링크는 manifest와 대조했다. |
| 18 | 캡처가 수정된 로드맵 항목을 보여 주는지 | Agent Browser 화면에서 #706과 #710 항목, #710 merge SHA, 그 아래 검증 관문 제목을 시각 확인했다. 캡처는 `tailnet-roadmap-pr710-agent-browser.png`이다. |
| 19 | 계획 페이지 내용의 최신성 | 페이지 본문에서 PR #710, merge SHA, 미완료 상태, `72/213`이 확인됐다. |
| 20 | 실행 근거와 계획의 역할 분리 | 실행 근거는 Android/iOS 각각 8/8 simulator scene과 layout-box 한계를 말하며 전체 C11.4 지원을 주장하지 않는다. |
| 21 | 게시 입력 commit 재현성 | selected root와 publication manifest는 source HEAD `610fa07c42d6bcacd3aa419f552c19aac9d08f89`, tree `9fb7e0c5…`를 기록한다. |
| 22 | 사전 route 집합 크기 | 게시 manifest는 431개 route, 합계 12,800,039 bytes다. |
| 23 | 사전 비교의 개수 산술 | 409 create + 7 replace + 15 same = 431; 값이 preflight와 copy report에서 일치한다. |
| 24 | 대상 파일 삭제 방지 | copy report의 `noRoutesDeleted`가 true이며 동기화 스크립트에는 route 삭제 경로가 없다. |
| 25 | 기존 페이지 복구 가능성 | 교체 전 7개 파일, 3,176,637 bytes를 별도 backup root에 저장했다. |
| 26 | 백업 손상·누락 | 재검증 스크립트가 7개 backup 모두의 크기와 SHA-256을 다시 계산해 일치시켰다. |
| 27 | 교체 중 동시 사용 안전성 | preflight에서 교체 대상 7개의 open-handle 검사가 닫힘으로 기록됐고 sync 직전 `lsof` 검사도 스크립트에 있다. |
| 28 | 로컬 게시본 응답과 콘텐츠 | 431개 local route 전부 HTTP 200이고 manifest 크기·SHA-256과 일치한다. |
| 29 | Tailnet 게시본 응답과 콘텐츠 | 431개 Tailnet route 전부 HTTP 200이고 manifest 크기·SHA-256과 일치한다. |
| 30 | 브라우저 프로세스 종료 | Agent Browser session 목록이 비었고 실행 중인 agent-browser 프로세스가 없는 것을 확인했다. |

검증 결과는 로컬 문서와 인증된 private Tailnet 미리보기에 한정된다. PR CI 또는 공개 문서 호스팅 성공으로 확대 해석하지 않는다.
