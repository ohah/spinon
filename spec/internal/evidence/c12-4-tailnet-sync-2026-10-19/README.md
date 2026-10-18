# C12.4 Tailnet 미리보기 동기화 근거

2026-10-19에 병합된 PR [#330](https://github.com/ohah/spinon/pull/330)의 문서 변경을 Tailnet 미리보기 경로에 반영하고 응답 내용을 확인했습니다. 원본은 `main`의 `63cfe6ed1adeecac29c63bde4e31c6154c20d4c3`입니다.

## 빌드와 반영

- 저장소에 고정된 Rspress 2.0.22로 Tailnet 미리보기에 필요한 선택 문서 집합을 빌드했습니다. dead-link 검사는 이 선택 빌드에서 끄고, 결과는 [selected-build.log](selected-build.log)에 보존했습니다. 선택 빌드는 전체 문서 빌드가 아닙니다.
- 전체 문서 빌드는 PR #330에서 기록한 저장소 전반의 571개 dead-link 오류로 실패했습니다. 해당 제한은 이번 선택 빌드 성공과 별도로 유지합니다.
- 반영 대상 92개: 신규 57개, 교체 6개, 기존과 동일 29개. 삭제는 0개입니다.
- 교체한 6개 파일은 복사 전에 Tailnet 백업 경로에 저장하고 SHA-256을 대조했습니다(6/6 일치, 실패 0).
- 신규·교체 파일 63개를 복사한 뒤 전체 92개 대상의 SHA-256을 확인했습니다(92/92 일치, 실패 0).
- 원래 실행 중인 미리보기 listener PID 1191을 유지했고 재시작하지 않았습니다. 로컬 절대 경로와 비공개 Tailnet 호스트 이름은 이 근거에 넣지 않았습니다.

## 제공 응답 확인

각 92개 경로에 대해 로컬 미리보기와 Tailnet HTTPS로 요청했습니다. 총 184개 응답이 모두 HTTP 200이었고, 응답 본문의 SHA-256이 기대값과 모두 일치했습니다(184/184, 실패 0). 상세 값은 [served-verification.json](served-verification.json)에 있습니다.

Agent Browser로 Tailnet 페이지도 열어 확인했습니다.

- [roadmap 화면](tailnet-roadmap-note-agent-browser-v4.png): 2026-10-19 항목의 16 cycle × 8 session, 총 128 session 결과와 C12.4 미완료 경계가 표시됩니다.
- [상태 대장 화면](tailnet-status-c124-agent-browser-v2.png): C12.4 상태와 PR #330 근거 링크가 표시됩니다.
- 실행 근거 페이지에서 결과 이미지가 정상 로드됐고, 자연 크기는 1080×2400입니다. 원본 이미지와 실행 자료는 [C12.4 실행 근거](../c12-4-repeated-session-lifecycle-2026-10-19/c124-lifecycle-20261018T194837821Z-5af75d12/README.md)에서 확인할 수 있습니다.

화면 캡처는 Agent Browser로 얻었습니다. 브라우저 세션을 닫은 뒤 프로세스가 남지 않았는지 확인했습니다. 사이트 업데이트는 Tailnet 미리보기 경로에만 반영했으며 GitHub Pages 배포는 하지 않았습니다.

## 판정 범위

이 근거는 선택 문서 빌드, 복사 대상의 해시, 로컬·Tailnet HTTP 응답, Tailnet 페이지 렌더링을 확인합니다. 전체 문서 dead-link 검사, iOS·Android 앱 동작, 실기기 동작, 전체 CSS 지원 완료를 증명하지 않습니다. C12.4는 미완료 상태로 유지합니다.

## 파일

- [preflight.json](preflight.json): 92개 동기화 대상과 신규·교체·동일·삭제 집계
- [backup-verification.json](backup-verification.json): 교체 전 백업의 검증 결과
- [copy-verification.json](copy-verification.json): Tailnet 복사 후 전체 대상 해시 검증
- [served-verification.json](served-verification.json): 로컬 및 Tailnet HTTPS 제공 응답의 상태와 해시
- [manifest.json](manifest.json): 동기화 입력 파일별 SHA-256 manifest
- [SHA256SUMS](SHA256SUMS): 이 디렉터리의 근거 파일 manifest
