# C11.4 PR #385 Tailnet 문서 동기화 실패 관점 검토 20개

검토 대상은 병합된 PR #385 상태와 근거를 기존 Tailnet /spinon/ 정적 미리보기에 반영한 선택 빌드·파일 sync·화면 확인이다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | sync 대상이 저장소 경로나 다른 사용자 폴더로 바뀌는가 | script가 승인된 ~/Library/Application Support/SpinonPreview 경로만 허용한다. source와 backup 경로도 고정 검증한다. 통과. |
| 2 | 실행 중인 Tailnet server가 새 경로를 제공하지 않는가 | server PID 1191의 8768/18789 listener를 확인했고, HTTPS origin에서 1,611개 route file의 HTTP 결과를 검증했다. 통과. |
| 3 | 선택 빌드가 잘못된 commit 또는 branch에서 만들어지는가 | source는 main의 a623819ff9148f12a1d784eaa9739043618ddf1f이며 PR #386 merge 이후다. 통과. |
| 4 | 이전 auto-fit 선택 source만 복사하고 PR #385 문서를 놓치는가 | 선택 root에 auto-repeat/negative-line 계획, 계약 0059, STATUS, 공개 roadmap, PR #385 실행 근거와 PR #386 sync 근거가 포함됐다. 통과. |
| 5 | 실패한 중간 캡처나 무관한 C12.4 untracked evidence를 preview에 포함하는가 | source mapping은 성공 근거의 특정 README·리뷰·대표 PNG만 지정하고 실패 캡처 및 C12.4 폴더를 선택하지 않는다. 통과. |
| 6 | 최신 Rspress 외부 설치나 lockfile 변경이 생기는가 | 저장소에 설치된 Rspress 2.0.22 CLI를 재사용했고 dependency 설치/lock 변경은 없다. 통과. |
| 7 | 선택 root가 다른 설정 또는 출력 디렉터리로 빌드되는가 | 별도 pass16 config가 검증된 선택 root와 고유 output path를 고정한다. Rspress build가 exit 0이다. 통과. |
| 8 | 선택 파일에 host/worktree 절대 경로가 남는가 | root에서 199개를 제거하고 빌드 뒤 추가 1개를 제거했다. 마지막 검색에서 잔여 파일 0개다. 통과. |
| 9 | Rspress route가 plan과 신규 evidence README를 누락하는가 | roadmap, STATUS, 계획, 계약, 양 evidence README의 생성 파일과 Agent Browser route를 확인했다. 통과. |
| 10 | public roadmap가 이전 PR만 보이거나 #385 note 순서가 틀리는가 | Tailnet roadmap 화면 첫 C11.4 note에 #385 merge 문구, 124/124 행렬과 제한 범위가 표시된다. 통과. |
| 11 | 공개 roadmap의 plan href가 404 또는 잘못된 경로인가 | GitHub main의 plan source를 Agent Browser에서 열었고 제목과 문서 내용이 표시된다. 통과. |
| 12 | PR #385 기록에서 merge SHA 또는 simulator 수치가 달라지는가 | 화면과 생성 HTML에 SHA 078f5708..., 124/124, trace 676/1,284 및 세 overflow scene 표본 5/5가 표시된다. 통과. |
| 13 | STATUS의 C11.4 또는 C11 완료 상태가 동기화 중 켜지는가 | C11.4 intrinsic track sizing checkbox를 직접 조회해 checked=false를 확인했다. 통과. |
| 14 | contract page가 새 제한 범위나 0.1.0 버전을 누락하는가 | 계약 0059 Tailnet route에 PR #385, 124/124, 내부 버전 0.1.0이 보인다. 통과. |
| 15 | implementation evidence 이미지 URL이 깨지거나 다른 자산을 가리키는가 | Agent Browser에서 Android 1080×2400과 iOS 1206×2622 PNG가 각각 로드됐다. 통과. |
| 16 | 공개 note의 내부 링크가 아직 없는 local route를 가리키는가 | 계약, 실행 README, 구현 검토 20, PR diff 30 네 링크가 각각 HTTP 200이다. 통과. |
| 17 | preview 교체가 전체 폴더 삭제를 포함하거나 기존 데이터를 잃는가 | 사전 비교 결과 생성 113, 교체 90, 동일 1,408, 삭제 0이다. 통과. |
| 18 | 기존 open handle이 있는 파일을 truncate하거나 backup 없이 바꾸는가 | PID 1191의 열린 HTML 한 파일을 특정했다. 이전 bytes SHA가 backup 90/90 검증에 포함되고 같은 디렉터리 atomic rename을 사용했다. 통과. |
| 19 | 복사 후 대상 파일이 source와 다르거나 서버가 stale bytes를 반환하는가 | 로컬 copy 확인과 HTTP served verification 모두 1,611/1,611 SHA 일치, HTTP 실패와 mismatch는 0이다. 통과. |
| 20 | 브라우저나 무관한 tmux/server process가 남거나 종료되는가 | Agent Browser 세션 전체를 닫았고 PID 77078은 종료 확인했다. Tailnet server와 tmux session은 종료하지 않았다. 통과. |

## 최종 판정

동기화 실패 관점에서 병합 blocker는 발견하지 않았다. 실제 제공 byte와 Agent Browser 화면이 모두 갱신된 C11.4 제한 범위 및 공식 미완료 상태를 보여 준다. 앱과 WPT 실행은 이번 Tailnet sync의 범위가 아니다.
