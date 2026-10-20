# C11.4 negative implicit Tailnet sync 검토 20관점

검토 범위는 선택 Rspress build, static root copy, backup/served hash와 Agent Browser evidence다. 앱 동작이나 전체 문서 적합성을 재판정하지 않는다.

| # | 실패 가능성 | 확인 결과 |
| ---: | --- | --- |
| 1 | source가 PR #380 merge 이전 commit | 준비 결과 source HEAD `f0a00bf501f5764f7d5c7da672403aaeadd7fb63`이 PR #380 merge SHA와 일치한다. |
| 2 | 전체 source tree를 preview에 무심코 복사 | 선택 root mapping 142개와 산출물 manifest의 source 목록이 기록되어 있다. |
| 3 | untracked capture를 evidence로 섞음 | 선택 디렉터리의 미추적 파일 223개를 manifest와 staging에서 제외했다. |
| 4 | input hash가 실제 build source와 다름 | 81개 고유 source의 tracked file 1,447개가 SHA-256 manifest에 들어 있다. |
| 5 | 선택 directory recursion이 untracked 파일을 읽음 | prepare와 input manifest 모두 `git ls-files --cached`로 tracked source만 읽는다. |
| 6 | symlink가 target 밖 파일을 포함 | 선택 root/target/backup symlink를 거부하며 tracked source symlink도 거부한다. |
| 7 | 문서 generator 버전이 drift | Rspress `2.0.22` 고정 패키지와 frozen lockfile을 썼다. |
| 8 | dependency 설치 중 lockfile 수정 | `bun.lock`과 `packages/docs/package.json` 변경이 없다. |
| 9 | 빌드 실패를 성공으로 기록 | 초기 executable-resolution 실패를 보존하고 최종 Rspress build log를 별도로 기록했다. |
| 10 | 빌드 산출물에 host/worktree path가 남음 | sanitizer가 1개 generated JS에서 host path를 정리했고 잔여 경로는 0개다. |
| 11 | preview root 전체를 지우거나 덮어씀 | 2,841개 대상만 create/replace/same으로 비교하고 delete는 없다. |
| 12 | 대상 경로가 preview 외부 | sync script가 정확한 `SpinonPreview`와 전용 pass19 backup root를 요구한다. |
| 13 | 기존 bytes 없이 replace | 103개 교체 파일을 전용 backup에 저장해 103/103 SHA를 확인했다. |
| 14 | open handle을 가진 파일을 in-place truncate | server PID 1191이 열어 둔 5개 파일은 backup 후 same-directory atomic rename으로 교체했다. |
| 15 | 변경 중 source file set 또는 hash가 바뀜 | copy script는 preflight identity, 전체 path set, byte length와 SHA-256을 재검사한다. |
| 16 | copy 성공인데 destination hash 불일치 | create/replace 234개와 전체 2,841개 copy entry가 검증됐다. |
| 17 | 파일이 target에 있으나 HTTPS server가 구버전을 제공 | Tailnet HTTP 200·bytes·SHA-256이 2,841/2,841 일치한다. |
| 18 | roadmap link 또는 contract fragment가 틀림 | Agent Browser에서 계획/계약/evidence route를 열고 negative contract `anchor_found=true`를 확인했다. |
| 19 | 문서 screenshot이 경로만 있고 image는 404 | Android 1080×2400 및 iOS 1206×2622 PNG가 `complete=true`로 로드됐다. |
| 20 | browser/preview/tmux 정리로 기존 사용자 상태 손실 | 격리 Agent Browser session만 종료했다. 공유 PID 67317, preview PID 1191은 유지했고 tmux를 건드리지 않았다. |

이 검토는 pass19 선택 동기화 근거에 한정한다. status counter는 72/212이고 C11.4는 미완료이며 sync는 GitHub Pages 배포가 아니다.
