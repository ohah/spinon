# C15.3 HostDocument Tailnet 선택 렌더 기록

## 입력과 명령

- 기준 main: `6c5edcdadfcf55ec01e381964acc02a66b880c89` (PR #264 구현 및 PR #265 공개 roadmap 갱신 병합 후).
- Rspress: lockfile의 `2.0.22`.
- `/tmp/spinon-c15-3-hostdocument-docs-input-20261017/spec`에 Tailnet에서 갱신할 상태 대장, C15 계획·계약·근거 문서만 복사했다. 전체 `spec/`은 371 MB이므로 복사하지 않았고 선택 입력에는 119개 파일이 있다.
- 입력 복사본에서 `STATUS.md`의 `../plan/`을 `plan/`으로, plan의 `../spec/internal/`을 `../internal/`로, internal index의 `../../plan/`을 `../plan/`으로 조정해 Tailnet route에 맞췄다. 저장소 원본은 바꾸지 않았다.
- 선택 입력만 Rspress route로 만들었다. 전체 docs strict build와 Markdown 전체 링크 검사를 통과했다고 주장하지 않는다. 선택 route 밖의 참조는 link/image check를 끄고, 실제 게시할 165개 경로를 별도 HTTP body/hash로 확인했다.
- 실행 명령: `SPINON_DOC_ROOT=/private/tmp/spinon-c15-3-hostdocument-docs-input-20261017/spec SPINON_DOC_OUT_DIR=/private/tmp/spinon-c15-3-hostdocument-docs-out-20261017 RSPRESS_PERSISTENT_CACHE=false bun run --filter @spinon/docs docs:build --config /private/tmp/spinon-c15-3-unicode-shaping-impl-20261017/packages/docs/.rspress.tailnet.config.ts`.
- 결과: exit code 0. 출력은 2.9 MB, 선택 Rspress `static/`은 29개 자산이다. [전체 build log](selected-build.log).

## 게시 경계

Tailnet listener는 `/Users/yoonhb/Library/Application Support/SpinonPreview`를 정적 root로 제공한다. URL의 `/spinon/` prefix는 Tailscale Serve route에서 처리하므로 파일은 그 root에 상대 경로로 복사했다. 잘못된 `SpinonPreview/spinon/` 하위 디렉터리를 만들지 않았다.

`roadmap.html`, `public/roadmap.html`, `STATUS.html`/`.md`, C15 plan·내부 index·계약·구현 evidence route, 직접 연결된 source evidence, 선택 Rspress static assets, C15.1 HTML fixture와 Ahem을 대상으로 삼았다. 이전 내용과 다른 13개 파일은 전용 backup root에 bytes/hash를 검증한 뒤 먼저 보존했다. preview 파일 삭제는 없고 listener를 재시작하지 않았다.
