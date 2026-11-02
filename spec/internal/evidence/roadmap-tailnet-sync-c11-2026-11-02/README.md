# C11 병합 로드맵 Tailnet 미리보기 동기화

2026-11-02 현재 `origin/main` HEAD `00230cda828254b21ddee42351aed3bddceaab8a`의 로드맵과 C11 설명을 Tailnet 미리보기로 반영했다. 공식 대장에는 C11 상위 항목이 미완료로 남고 전체 집계는 `72/213`이다. PR #1038은 제한된 parsed Root HTML 정수 `repeat()` 구현으로 기록되며 전체 Grid 지원으로 확대하지 않았다.

## 게시 범위와 결과

- Tailscale은 `Running`/online이었다. PID 28040이 `127.0.0.1:8768`에서 기존 미리보기 서버로 동작했고 재시작하지 않았다.
- RSPress 2.0.22로 42개 선택 입력을 렌더링했다. 게시 후보는 로드맵, 공식 상태, 내부 계약 0059, C11 계획과 PR #1038 관련 precomparison/runtime 근거 및 필요한 공통 asset으로 제한했다.
- 후보는 124개 경로, 9,125,844 bytes다. 기존 경로 15개를 해시 확인 후 백업하고 교체했고 89개를 생성했으며 20개는 이미 같은 내용이었다. 삭제는 없었다.
- preflight에서 교체 경로를 여는 process가 없는 것을 확인했다. 백업은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-2026-11-02/pass01`에 두었다.
- 반영 후 선택한 124개 경로 모두 local preview와 HTTPS Tailnet에서 HTTP 200 및 source SHA-256 일치를 확인했다. 백업한 15개도 이전 해시와 일치했다.
- 전체 저장소 `docs:build` 시도는 이번 선택 범위 밖의 기존 evidence/spec 링크 여러 건을 dead-link 오류로 보고해 실패했다. 별도 selected-root 빌드는 필요한 루트만 렌더링하며 전역 dead-link 검사를 끈 설정으로 성공했다. 따라서 이를 저장소 전체 문서 빌드 성공으로 해석하지 않는다. PR #1038 카드의 로컬 링크 8개는 모두 publication manifest에 존재하는지 별도 검사했다.
- Agent Browser로 실제 Tailnet 페이지의 PR #1038 merge SHA와 `72/213` 표기를 확인했다. Android `1080×2400`, iOS `1206×2622` 화면 링크를 브라우저에서 각각 열어 로드 완료를 확인했다. desktop/mobile 페이지 캡처는 이 폴더의 PNG 파일과 `agent-browser-verification.json`에 기록했다.
- 확인 후 Agent Browser 세션을 닫았고 세션 Chrome process가 남지 않았다. Tailnet 미리보기 서버는 유지했다.

## 재현·검토 자료

- [선택 root 준비](prepare-selected-root.mjs) · [선택 RSPress 설정](rspress.config.ts) · [빌드 결과](selected-build-result.json)
- [publication staging](stage-publication.mjs) · [publication 경로 및 SHA-256 목록](selected-publication.json)
- [preflight](preflight.json) · [백업/복사 결과](copy-and-backup.json) · [local/HTTPS 검증](served-verification.json)
- [Agent Browser 확인과 화면 hash](agent-browser-verification.json)
- [desktop 화면](tailnet-roadmap-agent-browser-desktop.png) · [mobile 화면](tailnet-roadmap-agent-browser-mobile-pass04.png)
- [이 문서 변경의 독립 PR diff 실패 관점 검토 30개](pr-diff-review-30.md)

이 갱신은 사용자가 지정한 개발 미리보기의 반영이다. GitHub Pages나 공식 제품 배포는 실행하지 않았다.
