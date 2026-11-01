# PR #1002 병합 후 C11 Root HTML gap 로드맵 동기화 · 2026-11-02

## 현재 C11 상태

C11 상위 체크박스와 전체 공식 완료 수는 바꾸지 않았다. 제한 profile 가운데 C11.2.2와 C11.2.3은 공식 상태 대장에서 체크되어 있고, C11.3·C11.4 및 C11 상위는 미완료다. C11.1 Root HTML 기본 runtime 경로는 PR #1000으로 연결됐고, finite non-negative fixed-px gap 후속 PR #1002가 `12b554c5e1a1946e7267545d5d5092ba33211af9`로 병합됐다. 공식 집계는 `72/213`이다.

## Tailnet 반영

Tailnet backend는 `Running`이었고 기존 preview server PID 28040은 `127.0.0.1:8768`에서 HTTP 200을 반환했다. 변경 전 preview의 `roadmap.html` hash는 `e79f12e69cb2edf58f1723de6f86d7d0f04406a695ebd86664ab2f8ec64007cc`였다. 이 파일은 변경 전 저장소 source에서 새 #1002 문단만 뺀 내용과 byte-for-byte 동일했다. 그래서 기존 route 한 개만 교체했다.

변경 전 파일은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1002-2026-11-02/pass01/roadmap.html`에 보존했다. 열린 파일 핸들 0개를 확인하고 동일 파일시스템 임시 파일에서 atomic rename으로 바꿨다. 삭제나 서버 재시작은 하지 않았다. 저장소 source, 로컬 HTTP response와 Tailnet HTTPS response는 모두 492,256 bytes이며 SHA-256 `cd18c1fc39ae0c43bd431758883805e498b4b0519d612f3956e7ff8d041570bf`로 일치한다. 세부 내용은 `preflight.json`, `copy-and-backup.json`, `served-verification.json`에 있다.

## Agent Browser

Agent Browser로 Tailnet URL을 desktop 1440×1000과 mobile 390×844에서 확인했다. PR #1002·merge SHA·공식 `72/213`을 읽었으며 mobile document/body width는 viewport와 동일한 390px, 첫 두 progress card는 각 362px였다. 캡처와 기계 판독 결과는 [desktop](./tailnet-roadmap-agent-browser-desktop.png), [mobile](./tailnet-roadmap-agent-browser-mobile.png), [desktop 결과](./agent-browser-desktop.json), [mobile 결과](./agent-browser-mobile.json)에 저장했다. 확인 후 `agent-browser close --all`을 실행했고 Agent Browser/Chrome headless 프로세스가 남지 않았다. 기존 Tailnet preview server는 계속 실행 중이다.

## 근거 파일

- [C11.1 fixed-px gap 계획](../../../../plan/c11-root-html-grid-gap.md)
- [공식 상태 대장](../../../STATUS.md)
- [내부 계약 0059](../../0059-c11-grid-layout.md)
- [PR #1002](https://github.com/ohah/spinon/pull/1002)
- [이번 문서·Tailnet 변경 적대 검토 30개](./pr-diff-review-30.md)
