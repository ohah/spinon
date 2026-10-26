# PR #669 이후 로드맵/Tailnet 문서 변경 30관점 검토

검토 대상은 공개 로드맵 한 항목, C11.4 집중 계획의 병합 상태, 선택 Rspress root·route 구성과 백업/복사 경계다. 앱 구현 PR의 30관점 검토를 재사용하지 않고, 공식 상태·링크·문서 빌드와 미리보기 게시 실패 경로를 따로 확인한다. Tailnet HTTP hash와 Agent Browser 관찰은 실행 후 별도 원본 JSON과 캡처로 기록하며, 이 표만으로 게시 성공을 주장하지 않는다.

| # | 독립 실패 경로 | 결과와 근거 |
|---:|---|---|
| 1 | 구현 PR의 실제 merge SHA나 시각을 잘못 기록하는가 | **통과** — GitHub PR #669의 `mergedAt`과 merge commit을 조회해 `2026-10-26T22:28:31Z`, `3e46d12c1b5df26a0c06507defe3826e2f8b4fdb`로 대조한다. |
| 2 | 집중 계획이 PR을 아직 대기 중으로 표시하는가 | **통과** — 상태 줄과 작업 6을 PR #669 Rebase and merge 완료로 맞춘다. |
| 3 | 빈 check rollup을 CI 성공으로 설명하는가 | **통과** — 계획·로드맵은 check rollup이 비었고 main required status checks가 없었다고 분리해 쓴다. |
| 4 | C11.4 전체 완료를 잘못 암시하는가 | **통과** — 제한 fixture 한 장면이라고 명시하고 전체 C11.4 미완료를 유지한다. |
| 5 | 공식 완료 수가 무심코 증가하는가 | **통과** — `spec/STATUS.md`를 편집하지 않고 `72/213`을 고정한다. |
| 6 | C11.4 공식 체크박스를 체크하는가 | **통과** — 상태 대장 체크박스는 변경하지 않는다. |
| 7 | 균등 auto 행 수치가 독립 oracle과 다른가 | **통과** — 행 크기 `[32.5,42.5,5]px`, 세 번째 fixed row 유지 내용을 구현 기록과 비교한다. |
| 8 | Android 대상을 실기기나 hardware GPU로 잘못 부르는가 | **통과** — API 37 ARM64 AVD이며 software renderer라는 한계를 쓴다. |
| 9 | iOS 표본을 compositor 이후 pixel 결과로 부르는가 | **통과** — Metal 표본은 제출 전 drawable이라고 쓴다. |
| 10 | PixelCopy/Metal 표본 수를 전체 화면 비교처럼 과장하는가 | **통과** — 5/5 지점 표본의 일치로 범위를 한정한다. |
| 11 | WPT 또는 일반 CSS admission을 확인했다고 쓰는가 | **통과** — WPT와 일반 CSS admission은 미검증으로 표시한다. |
| 12 | viewport 또는 root sizing의 플랫폼 parity를 주장하는가 | **통과** — 계획의 viewport 차이와 row-axis slice 범위를 유지한다. |
| 13 | 수정된 runner guard의 시험 결과를 C11.4 지원으로 확대하는가 | **통과** — CLI guard는 fixture 범위의 회귀 방지로 설명한다. |
| 14 | 로드맵의 최신 날짜 항목 앞에 새 기록이 삽입되는가 | **통과** — 2026-10-27 #664 기록 다음에 #669를 추가한다. |
| 15 | 로드맵 링크가 PR 자체가 아닌 다른 작업을 가리키는가 | **통과** — `https://github.com/ohah/spinon/pull/669`를 명시한다. |
| 16 | 새 계획 링크가 RSPress route와 다른 파일명을 가리키는가 | **통과** — `plan/c11-4-align-content-normal-runtime.html` 경로를 원본 계획과 대조한다. |
| 17 | runtime README 링크가 새 증거 폴더를 가리키는가 | **통과** — README의 실제 source 경로와 공개 route를 비교한다. |
| 18 | Android 화면 링크가 이전 실행이나 다른 scene을 가리키는가 | **통과** — 최종 `222150868Z-94e8b191` fixture PNG를 사용한다. |
| 19 | iOS 화면 링크가 이전 실행이나 다른 simulator를 가리키는가 | **통과** — 최종 `222207272Z-dce6182a` fixture PNG를 사용한다. |
| 20 | 새 문서 검토 링크를 미리보기 root에서 찾지 못하는가 | **통과** — 선택 root에 이 문서의 생성 route를 포함한다. |
| 21 | relative `spec/STATUS.md` 링크가 사라지는가 | **통과** — root alias와 `spec/STATUS.md` route를 별도로 포함한다. |
| 22 | §11.8 precomparison 링크가 깨지는가 | **통과** — precomparison README와 raw evidence를 두 공개 path alias에 포함한다. |
| 23 | source fixture 경로가 선택 root에서 빠지는가 | **통과** — 계획이 참조하는 고정 HTML fixture를 route asset에 추가한다. |
| 24 | 전체 문서 폴더를 빌드해 보존된 archived config가 다시 문제를 일으키는가 | **통과** — Rspress는 좁은 선택 root를 사용하고 전체 `docs:build` 성공을 주장하지 않는다. |
| 25 | 선택 build의 dead-link 검사 비활성화를 전체 링크 검증으로 오해하는가 | **통과** — 설정은 선택 route build 전용이며 README에서 검증 범위를 한정한다. |
| 26 | 선택 root에 임의 repo 파일이나 symbolic link가 섞이는가 | **통과** — 준비기는 선언된 source map만 복사하고 symlink를 거부한다. |
| 27 | staged route가 Rspress 결과와 다르게 변하는가 | **통과** — stage manifest는 byte length와 SHA-256을 기록하고 복사 뒤 다시 대조한다. |
| 28 | 미리보기 서버가 내려가 있거나 엉뚱한 root를 덮어쓰는가 | **통과** — 사전 단계는 Tailscale backend, PID/listener, realpath와 기존 HTTP 200을 요구한다. |
| 29 | 열린 파일을 덮거나 복구본 없이 route를 바꾸는가 | **통과** — 변경된 open handle은 거부하고 기존 파일은 별도 backup 후 atomic rename한다. |
| 30 | 실패 경로에서 preview 파일을 삭제하거나 서버를 재시작하는가 | **통과** — sync 경로에 delete/restart가 없고 backup/hash/served response를 따로 기록한다. |

**결과:** 문서 상태·선택 build·복사 코드의 독립 실패 관점 30/30을 검토했다. 실제 게시 성공은 `tailnet-copy.json`, `tailnet-verify.json`, Agent Browser 관찰과 화면으로 별도 입증하며 이 PR 리뷰 표와 혼동하지 않는다.
