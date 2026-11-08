# C11.4 PR #1295 병합 후 상태·로드맵·Tailnet 동기화

이 기록은 구현 PR #1295 병합 뒤 상태 대장, 내부 계약, 계획 문서, 공개 로드맵 원본과 Tailnet 미리보기의 동기화를 확인한다. 코드 동작이나 공식 완료 수를 변경하지 않는다.

## 변경 범위와 공식 상태

- 구현 PR #1295은 `2026-11-08T20:35:50Z` Rebase and merge됐고 merge SHA는 `40b416b4de4942c9251db0f4862626443310d092`다. 이 상태를 반영한 문서 PR #1296은 `2026-11-08T21:15:46Z` Rebase and merge됐으며, 최종 main SHA는 `04b152519a0336266dda86d5da5b7001fc383740`다.
- `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `plan/c11-4-root-html-repeated-text-row-span.md`, `spec/public/roadmap.html`을 실제 병합 결과에 맞췄다. C11.4/C11 체크와 공식 `78/214`는 그대로다.
- [게시 전 preflight](./publication-preflight.json), [게시 결과와 백업](./publication-result.json), [제공 경로·SHA 검증](./served-verification.json)에 Tailnet 상태를 기록했다.
- [문서 변경 실패 관점 30개 검토](./docs-pr-diff-review-30.md)를 마쳤다.

## Tailnet 제공본

Tailscale은 `Running`, preview server는 기존 PID 28040이며 재시작하지 않았다. 게시 전 roadmap은 2026-11-08 제공본이었다. 첫 게시 pass01은 86개 경로(12,968,031 bytes)에서 64개 추가, 11개 동일, 11개 교체를 반영했다. 이후 최종 pass02는 92개 경로(13,139,176 bytes)에서 9개 추가, 69개 동일, 14개 교체를 반영했다. 각 교체 전 파일은 `SpinonPreviewBackups/roadmap-tailnet-sync-pr1295-c11-4-2026-11-09/pass02`에 SHA 검증 후 보존했다. 바뀌는 파일은 게시 직전 모두 reader가 없었다. 어느 pass에서도 삭제나 서버 재시작은 없었다.

pass02 로컬 preview 92/92 경로와 Tailnet 주요 8/8 경로가 HTTP 200 및 원본/제공본 SHA-256 일치다. [pass01 게시 결과](./publication-result.json), [최종 pass02 게시 결과와 백업](./publication-result-pass02.json), [최종 제공본 검증](./served-verification-pass02.json)을 참조한다. 서버와 데이터는 private Tailnet preview 범위이며 GitHub Pages를 배포하지 않았다.

## 렌더 확인

선택한 상태·계획·계약·증거 문서 루트는 RSPress 2.0.22로 성공적으로 렌더링했다. 이 임시 build만 저장소의 기존 dead-link 및 dead-image 검사를 끈 별도 config를 사용했고, 저장소 설정은 바꾸지 않았다. 기본 전체 docs build는 unrelated 기존 Markdown dead links 및 `rust,ignore` fence/누락 이미지로 실패했다. 따라서 전체 docs build green으로 기록하지 않는다.

로드맵 칸반은 공식 `main`의 상태 대장을 동적으로 읽는다. PR #1296 병합 뒤 Agent Browser에서 Tailnet 페이지를 desktop `1440×1000`과 mobile `390×844`로 확인했다. 두 화면 모두 공식 완료 `78/214`와 `C11.4 후속 bounded slice 선정`을 표시했다. 이는 상위 C11.4/C11이 완료됐다는 뜻이 아니며, 다음 slice가 선택되지 않아 진행 칸은 `0/2`로 남아 있다. mobile에서 페이지 viewport는 `390px`이고 페이지 전체 `scrollWidth`도 `390px`이다. 상단 메뉴, 칸반, 도식만 각각 내부 `overflow-x: auto`를 사용한다. 확인에 사용한 Agent Browser 세션은 종료했다.
