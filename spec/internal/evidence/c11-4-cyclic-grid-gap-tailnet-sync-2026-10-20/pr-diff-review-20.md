# C11.4 Tailnet 문서 PR diff 적대 검토 20개

**대상:** 공개 roadmap/상태 대장 문구와 선택 빌드·동기화 helper 변경. 아래 검토는 PR 변경 diff와 생성 근거를 기준으로 각각 다른 실패 경로를 확인한다.

| # | 적대 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11.4 공식 완료 체크나 완료 수를 실수로 올렸나? | `STATUS.md` diff는 C11.4 아래 Tailnet 기록 한 줄을 추가한다. Tailnet checkbox는 `false`로 렌더됐다. |
| 2 | 공개 roadmap 설명이 PR #359의 좁은 cyclic gap 결과를 넘어서지 않나? | 문구는 intrinsic 0 basis, `calc()` 길이 항, 최종 content-box resolve, 고정 2×2 fixture를 구분한다. |
| 3 | Chrome 155와 Simulator 결과를 혼동하지 않나? | roadmap은 Chrome oracle와 Android/iOS simulator frame·surface sample을 따로 기술한다. |
| 4 | AVD PixelCopy와 iOS pre-compositor 표본을 실기기 결과라 부르지 않나? | Android software AVD, iOS compositor 전 drawable이라는 한계를 새 문구에 남겼다. |
| 5 | roadmap 링크가 병합 PR과 현재 상태·계획을 가리키나? | Agent Browser에서 PR #359 링크 한 개와 공식 상태·계획 링크를 확인했다. |
| 6 | C11.4 계획 route가 선택 build에 포함되나? | `/spinon/plan/c11-4-grid-track-sizing.html`가 존재하고 새 근거 링크가 해당 route에서 열렸다. |
| 7 | roadmap HTML 한 줄 편집이 다른 항목을 훼손하지 않았나? | diff는 PR #357 바로 뒤 새 C11.4 note 한 개만 삽입하며, 렌더 화면에서 #357·#359 이웃 항목을 확인했다. |
| 8 | source Markdown 상대 링크를 배포 편의 때문에 직접 바꾸지 않았나? | 원본 `STATUS.md`에는 새 근거 한 줄만 추가했다. 나머지 선택 Markdown rewrite는 staging tree에서만 수행한다. |
| 9 | subset 밖 링크가 `/spinon/` 바깥 same-origin 경로로 빠지지 않나? | 36개 HTML route에서 same-origin out-of-base 링크는 0개다. |
| 10 | 중복 source mapping이 주요 근거 링크를 오도하지 않나? | pass13 Browser가 Markdown 보고서의 네 screenshot source 누락을 찾아 GitHub main 링크가 깨짐을 확인했다. pass14 helper selection에 네 PNG source를 넣고 `/spinon/` image href 및 4/4 load를 검증한다. |
| 11 | 이전 Tailnet evidence를 덮지 않나? | 새 폴더 `c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20`와 작업 전용 backup root를 썼다. 기존 preview 경로만 선택 파일을 교체한다. |
| 12 | 임시 pass10과 최종 pass11 대상이 충돌하지 않나? | 두 pass의 staging root·site root·backup 하위 경로를 별도 이름으로 검증한다. |
| 13 | 생성 build 출력이나 절대 host 경로가 PR에 포함되나? | `build/`는 무시된다. pass11에서 공백 포함 절대 경로가 어색하게 잘리는 현상을 발견해 README에서 경로를 제거했다. pass12 Browser에서 generic 루트 설명이 보이고 잘린 문구·절대 사용자 경로가 사라진 것을 확인한 뒤 이 확정 문구를 pass13에 게시한다. |
| 14 | 선택 manifest가 자기 자신을 해시하거나 입력 변경을 숨기나? | preview-input manifest와 최종 `SHA256SUMS.txt`는 self-reference를 제외한다. pass11 manifest는 pass10 captures/logs를 포함한다. |
| 15 | preview sync가 선택 밖 파일을 삭제할 수 있나? | copy helper는 create/replace/same만 처리하고 delete 동작을 하지 않는다. pass10 preflight의 `no_deletes`는 true다. |
| 16 | 교체 전 원본 backup과 atomic replacement가 보장되나? | pass10 교체 31건 모두 사전 backup의 byte 수·SHA가 일치했고, 교체는 같은 디렉터리 임시 파일 rename을 썼다. |
| 17 | server의 열린 파일을 강제로 닫거나 server/tmux를 중지하나? | 교체 경로 lsof 0개, PID 1191의 8768 listen 유지가 기록됐다. helper는 kill/tmux 명령을 포함하지 않는다. |
| 18 | verifier가 임의 HTTP path를 origin처럼 허용하나? | verifier는 HTTPS origin만 받고 path/query/fragment가 있는 origin을 거부한다. |
| 19 | 로컬 복사 성공만으로 Tailnet 제공 완료를 주장하나? | 실제 URL 282개에서 HTTP 200·byte·SHA-256이 모두 일치했다. |
| 20 | sync 검토와 PR diff 검토를 같은 자료로 중복 계산했나? | 이 표는 변경 코드·문서 경계를 다루고, 별도 sync 표는 실제 preflight/backup/server/browser 운영 경계를 검토한다. 각 표에 서로 다른 20개 항목이 있다. |

## 검토 경계

pass10 selected build, 36개 route 검사, 실제 Tailnet 응답과 캡처한 화면을 기준으로 이 PR diff를 검토했다. pass11–13에서 helper 선택목록과 문서 이미지 링크 경계를 Agent Browser로 점검해 두 결함을 찾았고 각각 pass12·pass14로 수정·재검증한다. C11.4 런타임 구현의 범위를 다시 확장하는 검토는 아니다.
