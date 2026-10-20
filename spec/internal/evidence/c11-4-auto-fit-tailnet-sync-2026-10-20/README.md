# C11.4 auto-fit 조사 계획 Tailnet 로드맵 동기화

**기준:** PR [#370](https://github.com/ohah/spinon/pull/370) Rebase and merge, merge SHA `b7a2f495fbae7722166f5c5454bfa471eef4b286`.<br>
**범위:** PR #370에서 추가한 auto-fit 조사 계획, Chrome 사전 관찰, 공식 상태 대장 및 공개 로드맵을 기존 Tailnet 정적 미리보기 `/spinon/`에 반영했다. GitHub Pages와 앱은 실행하지 않았다.

## 반영한 내용

- Tailnet 로드맵의 C11.4 PR #368 항목에 [auto-fit min-size 조사 계획](https://github.com/ohah/spinon/blob/plan/c11-4-auto-fit-min-size-axis-20261020/plan/c11-4-auto-fit-min-size-axis.md)과 [Chrome 사전 비교](https://github.com/ohah/spinon/blob/plan/c11-4-auto-fit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/README.md) 링크가 보인다.
- 새 계획과 evidence README route를 선택 빌드에 포함했다. Tailnet Browser에서 roadmap, `STATUS`, plan, precomparison 네 route를 열었다.
- `STATUS`의 C11.4 체크박스는 꺼져 있다. 공식 완료 수와 CSS 지원 상태는 변경하지 않았다.
- 관찰은 Chrome reference이며 Spinon 앱 결과나 WPT 적합성으로 표시하지 않는다.

## 선택 빌드와 Tailnet 동기화

Rspress `2.0.22`의 저장소 고정 CLI로 선택 root를 빌드했다. 선택 root에는 59개 source mapping과 1,808개 staging file이 있었고 199개 host/worktree 경로를 제거했다. 빌드 뒤 자산 1개에서 남은 host 경로를 추가 제거했으며 최종 산출물에는 host/worktree 경로가 없다. 선택 입력의 811개 파일 SHA-256은 [manifest](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/SHA256SUMS-preview-inputs.txt)에 고정했다.

| 결과 | 파일 수 |
| --- | ---: |
| 선택 출력 | 1,543 |
| 새로 생성 | 92 |
| 기존 파일 교체 | 71 |
| 바이트 동일 | 1,380 |
| 교체 전 backup SHA-256 일치 | 71/71 |
| 교체 대상 중 서버가 열고 있던 파일 | 3 |
| HTTP 200 및 served-byte SHA-256 일치 | 1,543/1,543 |
| 삭제 | 0 |

교체 71개는 실행별 backup에 저장하고 SHA-256을 확인한 뒤 같은 디렉터리 임시 파일에서 원자 rename했다. 교체 전 Tailnet server PID 1191이 기존 C11.4 HTML 세 경로를 열고 있었으며 이 세 경로를 별도 기록했다. 서버 프로세스를 종료하지 않았고 열린 파일을 직접 truncate하지 않았다. 기존 tmux 13개 세션도 유지했다. 상세 preflight·열린 descriptor·copy/backup·served 결과는 [`preflight-pass14.json`](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/preflight-pass14.json), [`open-handles-before-copy-pass14.json`](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/open-handles-before-copy-pass14.json), [`copy-and-backup-pass14.json`](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/copy-and-backup-pass14.json), [`served-verification-pass14.json`](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/served-verification-pass14.json)에 남겼다.

## Agent Browser 확인

Agent Browser `0.31.1`에서 Tailnet `/spinon/roadmap.html`, `/spinon/STATUS.html`, 새 plan과 precomparison route를 확인했다. 로드맵에 새 계획 링크가 있고, 같은 Tailnet origin의 `/spinon/` 바깥 링크는 검사한 네 페이지에서 0개다. `STATUS` C11.4 checkbox는 `checked=false`였다. 출력 파일 1,543개는 Browser 확인과 별도로 HTTP 200 및 byte/SHA 기준 1,543/1,543 일치했다.

![Tailnet 로드맵의 C11.4 auto-fit 조사 계획 링크](https://raw.githubusercontent.com/ohah/spinon/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/tailnet-roadmap-agent-browser-pass14.png)

추가 화면은 [STATUS](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/tailnet-status-agent-browser-pass14.png), [계획](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/tailnet-plan-agent-browser-pass14.png), [Chrome 사전 비교](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/tailnet-precomparison-agent-browser-pass14.png) 페이지다. 이들은 Tailnet 문서 화면이며 Android/iOS 앱 화면이 아니다.

## 검토와 남은 범위

- [동기화 실패 관점 검토](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/sync-review-20.md) · [PR diff 검토](https://github.com/ohah/spinon/blob/docs/c11-4-auto-fit-tailnet-sync-20261020/spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/pr-diff-review-30.md)
- 선택 Rspress build, Tailnet 파일 hash와 Browser route만 확인했다. Android/iOS 앱 runtime, 시뮬레이터, 실기기, WPT, hardware GPU, iOS compositor-after 픽셀은 이번 문서 갱신에서 검증하지 않았다.
- C11.4 전체 CSS Grid 지원 및 공식 완료는 미완료다.
