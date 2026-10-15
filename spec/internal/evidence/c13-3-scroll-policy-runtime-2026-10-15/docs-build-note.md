# Rspress 문서 빌드 참고 · 2026-10-15

`bun install -F @spinon/docs --frozen-lockfile`은 성공했다. 저장소 전체 `bun run docs:build`는 C13.3 파일 외 기존 Markdown에서 참조하는 미포함 PNG 때문에 실패했다. 예: `spec/internal/evidence/s04-hit-test-platforms-2026-10-07.md`가 `s04-hit-test-ios-2026-10-07.png`를 import하며, C04/C06 evidence에도 같은 형태의 누락된 캡처가 있다. dead-link 검사를 임시로 끈 빌드도 해당 missing image import에서 종료됐다.

이 변경의 TailScale `STATUS.html`과 C13.3 `README.html`은 현재 Markdown을 `unified` + `remark-parse` + `remark-gfm` + `remark-rehype`로 정적 변환한 미리보기 산출물이다. `packages/docs/rspress.config.ts`는 원상 복구했고 tracked diff가 없다. 전체 Rspress site build 성공으로 보고하지 않는다.
