# C11.4 negative implicit Tailnet sync PR diff 검토 30관점

범위는 pass19 동기화 스크립트, input/build/copy/served/browser evidence와 이 문서다. PR #380의 roadmap source diff와 PR #379 구현 diff는 별도 검토 기록을 따른다.

| # | 실패 가능성 | 확인 결과 |
| ---: | --- | --- |
| 1 | sync가 PR #380 merge commit과 무관한 source를 썼다고 오해 | preflight source HEAD는 `f0a00bf501f5764f7d5c7da672403aaeadd7fb63`이다. |
| 2 | source commit이 아직 main에 없었음 | PR #380은 pass19 실행 전에 merge됐다. |
| 3 | selection root 이름이 run id와 다름 | config, prepare script, sanitize, sync script 모두 `pass19`로 고정됐다. |
| 4 | 잘못된 preview target에 쓰기 | target과 backup 경로를 사용자 SpinonPreview 경로로 제한한다. |
| 5 | static site 갱신 과정에서 GitHub Pages에 배포 | workflow는 Tailnet static root만 쓰며 Pages 배포 명령이 없다. |
| 6 | 전체 source tree를 선택 입력으로 주장 | prepare manifest가 선택 source/destination 142개를 나열한다. |
| 7 | route 중복을 source file 중복으로 혼동 | duplicate route mapping과 고유 input file 1,447개를 별도로 센다. |
| 8 | untracked simulator retry capture를 preview에 섞음 | 223개 untracked source file을 제외하고 그 목록을 기록한다. |
| 9 | manifest와 prepare 단계에서 추적 기준 불일치 | 두 단계 모두 selected directory를 `git ls-files --cached`로 확장한다. |
| 10 | directory symlink를 타고 root 밖으로 나감 | staging과 sync scripts가 symlink path와 entry를 reject한다. |
| 11 | user host path가 static JS/HTML에 남음 | sanitizer가 경로 1개를 제거하고 잔여 경로 0개를 보고한다. |
| 12 | build 실패를 감춤 | 초기 실행 실패 로그와 최종 성공 로그를 서로 다른 파일로 보존한다. |
| 13 | 설치 과정에서 dependency version을 갱신 | frozen lockfile 설치를 사용했고 lock/package 파일 변경이 없다. |
| 14 | Rspress output이 빈 경로에 잘못 생성 | 성공 build가 2,841 output file을 만들고 log에 plan/STATUS routes가 있다. |
| 15 | 기존 preview file set의 제거 누락을 delete로 처리 | preflight와 copy summary 모두 `no_deletes=true`다. |
| 16 | target 변화 후 오래된 preflight로 덮어씀 | copy 직전 target bytes/size/SHA를 다시 대조한다. |
| 17 | source output 변화 후 다른 bytes를 복사 | copy 단계가 preflight file set과 source hashes를 재검사한다. |
| 18 | backup이 target replacement 이후 생성 | 교체 전 previous bytes를 backup에 생성하고 hash 확인 후 rename한다. |
| 19 | 열린 server file을 truncate해 요청을 끊음 | PID 1191의 열린 5개 대상도 same-directory atomic rename을 썼다. |
| 20 | backup root가 공용 cache나 기존 run을 덮음 | 전용 `c11-4-negative-tailnet-sync-2026-10-20/pass19` 경로를 강제한다. |
| 21 | backup copy가 맞는 bytes를 보존하지 않음 | 기존 preview 교체 파일 backup은 103/103 SHA 검증됐다. |
| 22 | copy JSON의 일부 파일만 확인 | 2,841 entries 전체가 verified이고 mismatch가 없다. |
| 23 | HTTP 200만 보고 stale bytes를 놓침 | served verifier가 local과 served byte length 및 SHA-256을 비교했다. |
| 24 | roadmap 링크가 source path에만 존재 | Agent Browser에서 rendered roadmap 문단과 네 실제 href를 확인했다. |
| 25 | contract fragment가 다른 heading으로 이동 | rendered internal contract에서 exact ID가 H2를 가리킨다. |
| 26 | 공식 완료 수를 implementation slice 수로 오해 | rendered roadmap counter는 72/212이며 61/212가 아니다. |
| 27 | 제한 implementation이 완료로 체크됨 | rendered STATUS C11.4 checkbox의 `checked=false`를 확인했다. |
| 28 | screenshot URL이 있어도 image는 로드되지 않음 | Android 1080×2400 및 iOS 1206×2622 이미지의 `complete`와 natural size를 확인했다. |
| 29 | 30관점 review route가 생성되지 않음 | Agent Browser가 rendered review table 30행을 확인했다. |
| 30 | browser cleanup이 다른 user session/process를 종료 | pass19 isolated session은 inactive/PID null, process match 0이며 shared PID 67317은 유지됐다. |

Tailnet 문서 화면과 simulator capture는 별개다. runtime 결과의 hardware-device, full WPT, border stroke 또는 compositor-after 범위는 이번 sync로 확대하지 않는다.
