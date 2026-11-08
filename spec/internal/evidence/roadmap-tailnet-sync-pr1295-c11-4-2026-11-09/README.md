# C11.4 PR #1295 병합 후 상태·로드맵·Tailnet 동기화

이 기록은 구현 PR #1295 병합 뒤 상태 대장, 내부 계약, 계획 문서, 공개 로드맵 원본과 Tailnet 미리보기의 동기화를 확인한다. 코드 동작이나 공식 완료 수를 변경하지 않는다.

## 변경 범위와 공식 상태

- PR #1295은 `2026-11-08T20:35:50Z` Rebase and merge됐고 최종 main SHA는 `40b416b4de4942c9251db0f4862626443310d092`다.
- `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `plan/c11-4-root-html-repeated-text-row-span.md`, `spec/public/roadmap.html`을 실제 병합 결과에 맞췄다. C11.4/C11 체크와 공식 `78/214`는 그대로다.
- [게시 전 preflight](./publication-preflight.json), [게시 결과와 백업](./publication-result.json), [제공 경로·SHA 검증](./served-verification.json)에 Tailnet 상태를 기록했다.
- [문서 변경 실패 관점 30개 검토](./docs-pr-diff-review-30.md)를 마쳤다.

## Tailnet 제공본

Tailscale은 `Running`, preview server는 기존 PID 28040이며 재시작하지 않았다. 게시 전 roadmap은 2026-11-08 제공본이었다. 86개 경로(12,968,031 bytes)를 선택해 64개 추가, 11개 동일, 11개 교체했다. 교체 전 파일은 `SpinonPreviewBackups/roadmap-tailnet-sync-pr1295-c11-4-2026-11-09`에 SHA 검증 후 보존했다. 바뀌는 파일은 게시 직전 모두 reader가 없었다. 삭제는 없었다.

로컬 preview 86/86 경로와 Tailnet 주요 6/6 경로가 HTTP 200 및 원본/제공본 SHA-256 일치다. 서버와 데이터는 private Tailnet preview 범위이며 GitHub Pages를 배포하지 않았다.

## 렌더 확인

선택한 상태·계획·계약·증거 문서 루트는 RSPress 2.0.22로 성공적으로 렌더링했다. 이 임시 build만 저장소의 기존 dead-link 및 dead-image 검사를 끈 별도 config를 사용했고, 저장소 설정은 바꾸지 않았다. 기본 전체 docs build는 unrelated 기존 Markdown dead links 및 `rust,ignore` fence/누락 이미지로 실패했다. 따라서 전체 docs build green으로 기록하지 않는다.

현재 로드맵 칸반은 공식 `main`의 상태 대장을 동적으로 읽는다. 문서 PR이 main에 반영되기 전까지 칸반 summary는 이전 main 데이터를 표시할 수 있으므로, 해당 동적 칸반과 최종 desktop/mobile 화면은 문서 PR 병합 후 다시 확인한다.
