# C04.12 기본 root HTML 계획 Tailnet 미리보기 갱신

PR #321에서 병합한 기본 root HTML 렌더링 계획과 PR #322의 공개 로드맵 항목을 Tailnet 미리보기에 반영했다. 기준 main commit은 `887f1660444c542ae207da89a0b16778db34262b`이며 미리보기 주소는 `https://macstudio.tailed42f2.ts.net/spinon/`이다.

## 갱신 범위

- 선택 Rspress 2.0.22 빌드에서 최신 `STATUS.html`, 새 계획 페이지와 근거 페이지를 만들었다. 이 subset 빌드는 기존 테마를 사용하고 dead-link/dead-image 검사는 끈 상태다. 전체 문서 빌드 성공이나 링크 전체 적합성을 뜻하지 않는다.
- 로드맵·계획·근거 화면과 링크된 파일을 포함해 총 64개 경로, 5,428,395 bytes를 동기화했다. 신규 39개, 교체 5개, 이미 같은 해시라 그대로 둔 경로 20개다. 삭제는 없다.
- 교체 대상은 `roadmap.html`, `spec/public/roadmap.html`, `STATUS.html`, `STATUS.md`, `spec/STATUS.md`다. 기존 다섯 파일은 각각 `backup/c04-12-root-html-default-rendering-tailnet-sync-2026-10-18/` 아래 보존했고 백업 크기와 SHA-256을 원본과 대조했다.
- PR #321의 계획 항목이 들어간 최신 상태 대장을 복사했다. 공식 완료 수나 C04.12/C15/C16/C19 완료 상태는 바꾸지 않았다.
- 미리보기 서버 PID `1191`과 `127.0.0.1:8768` listener를 유지했다. 서버를 재시작하거나 tmux를 조작하지 않았다.

## 확인 결과

- `selected-build.log`: Rspress `2.0.22`, 페이지 렌더링 성공. 빌드된 전체 문서 집합과 dead-link 검사는 이번 범위가 아니다.
- `preflight.json`: 쓰기 전 대상별 존재 여부, 크기와 이전 해시를 기록했다.
- `backup-verification.json`: 교체 대상 다섯 백업이 기존 SHA-256과 모두 일치한다.
- `sync-manifest.json`: 64개 경로의 출처, 최종 크기·해시와 변경 상태를 기록했다.
- `served-verification.json`: 로컬 HTTP와 Tailnet HTTPS에서 64/64 경로가 HTTP 200이고 응답 바이트의 SHA-256이 manifest와 일치한다.
- Agent Browser `0.31.1`에서 Tailnet 로드맵 새 항목과 계획 페이지를 확인했다. 캡처는 각각 `1280×633px`이며 `browser-verification.json`에 해시를 기록했다. 확인 뒤 브라우저를 닫았고 해당 PID가 더는 존재하지 않았다.

## 한계

이번 작업은 개발 중인 Tailnet 미리보기 사본만 갱신했다. GitHub Pages를 배포하거나 Android/iOS Simulator를 실행하지 않았다. 새 계획은 구현 완료가 아니며 C04.12/C15/C16/C19는 계속 미완료다.

`SHA256SUMS`는 이 증거 묶음의 manifest, 검증 JSON, 빌드 로그와 Agent Browser 캡처를 묶는다.
