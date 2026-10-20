# C11.4 auto-fit Tailnet 동기화 실패 관점 검토 20개

**검토 대상:** PR #370 뒤 정적 Tailnet 문서 선택·빌드·복사·제공 및 Agent Browser 확인. 제품 runtime 구현 검토가 아니다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | 저장소 전체나 무관한 문서를 선택 root에 무심코 넣는가? | **통과.** `prepare-root-pass14.json`은 59개 선택 source mapping과 1,808개 staging file을 기록한다. |
| 2 | 신규 plan/evidence route가 선택 누락으로 로드맵에서 404가 되는가? | **통과.** `plan/c11-4-auto-fit-min-size-axis.html`과 precomparison README route를 Browser에서 열었다. |
| 3 | 저장소 고정 Rspress 버전이 아닌 임의 최신 CLI로 빌드되는가? | **통과.** 최종 선택 빌드는 저장소의 `@rspress/core` 2.0.22 CLI이며 성공했다. |
| 4 | evidence config가 package resolution을 우회해 pinned base config를 잘못 로드하는가? | **검토에서 발견·수정.** evidence config의 중복 `defineConfig` import를 제거하고 docs package에서 제공하는 base config object를 사용했다. 최종 build가 성공했다. |
| 5 | staging 또는 산출물에 host/worktree 절대 경로가 남는가? | **통과.** staging에서 199개 경로를 지웠고 build output의 남은 1개를 제거했다. 최종 검사 `remaining_host_path_files=[]`다. |
| 6 | 선택 source 파일이 바뀐 채 Tailnet target과 다른 산출물이 복사되는가? | **통과.** preflight와 copy 단계에서 file set, byte 수, SHA-256을 다시 대조했다. |
| 7 | sync가 기존 Tailnet 파일을 삭제하거나 전체 preview를 정리하는가? | **통과.** 결과는 create 92, replace 71, same 1,380이며 `no_deletes=true`다. |
| 8 | preview root가 server의 실제 정적 root와 다르게 잡히는가? | **통과.** PID 1191의 cwd/open file path에서 `SpinonPreview`를 확인했고 helper의 exact target guard가 통과했다. |
| 9 | 선택 target이 아닌 경로에 backup이 쓰이는가? | **통과.** PR #370 전용 `SpinonPreviewBackups/c11-4-pr370-tailnet-roadmap-sync-2026-10-20/pass14` 경로를 사용했다. |
| 10 | replace 대상 기존 바이트가 backup 전에 소실되는가? | **통과.** 변경 전 backup SHA-256 71/71 일치 후 same-directory temporary file을 rename했다. |
| 11 | copy 전에 Tailnet server가 열어 둔 file descriptor를 놓치는가? | **통과.** 바로 전 `lsof` scan에서 71 replace 대상 중 열린 기존 HTML 3개와 PID 1191을 기록했다. |
| 12 | 문서 동기화를 위해 Tailnet server나 사용 중인 tmux session을 종료하는가? | **통과.** PID 1191과 tmux 13개 session을 동기화 전후 유지했다. |
| 13 | 새 페이지 URL만 200이고 다른 선택 자산이 stale하게 남는가? | **통과.** 선택된 모든 1,543 URL의 served byte와 SHA-256을 비교해 1,543/1,543 일치했다. |
| 14 | 로드맵이 page base `/spinon/` 밖으로 같은-origin 링크를 내보내는가? | **통과.** Browser에서 roadmap, STATUS, plan, precomparison을 열어 같은-origin out-of-base 링크 0개를 확인했다. |
| 15 | official C11.4 status가 문서 동기화 과정에서 완료로 바뀌는가? | **통과.** Tailnet `STATUS` checkbox는 `checked=false`이고 로드맵이 완료를 주장하지 않는다. |
| 16 | Chrome 관찰을 Spinon runtime evidence로 오인시키는가? | **통과.** precomparison route의 Chrome-reference 한계 문구를 Browser에서 확인했다. |
| 17 | PR screenshot이 앱 실행 화면처럼 설명되는가? | **통과.** 네 캡처는 Tailnet 문서 페이지이며 앱 화면이 아니라고 기록했다. |
| 18 | PR screenshot의 이미지 URL이 branch 정리 뒤 깨지는가? | **통과.** `deleteBranchOnMerge=false`를 확인했고 raw GitHub screenshot URL을 Agent Browser에서 실제 PR 본문에 로드했다. |
| 19 | Tailnet 화면이 복사 직후가 아니라 이전 cache 결과를 보여주는가? | **통과.** sync 뒤 Agent Browser가 새 로드맵 링크와 plan/evidence page를 표시했고, 서버 응답 SHA도 현재 local file과 일치했다. |
| 20 | Tailnet 문서 관찰을 앱·WPT·실기기 검증으로 확대하는가? | **통과.** 보고서와 PR 본문은 문서 route 및 hash만 검증했다고 한정한다. |

문서 빌드는 선택 Rspress root에서 성공했다. 이 검토는 Android/iOS 앱이나 WPT 실행을 포함하지 않는다.
