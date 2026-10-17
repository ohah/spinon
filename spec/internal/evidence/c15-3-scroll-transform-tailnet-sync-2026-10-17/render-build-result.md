# Tailnet C15.3 선택 문서 렌더 결과

## 입력과 명령

- 원본 `origin/main`: `740b64032a68cd24ae258d341cefcfc2a3fe8984`.
- Rspress 버전: `2.0.22` (lockfile 고정).
- 선택 입력: `/private/tmp/spinon-c15-3-scroll-tailnet-input-20261017/spec`.
- 선택 출력: `/private/tmp/spinon-c15-3-scroll-tailnet-selected-20261017`.
- 입력에는 최신 `STATUS.md`, C15 계획·내부 색인·계약 0072–0074, C15.3 scroll precomparison 및 implementation 근거를 넣었다. `spec/public/roadmap.html`은 custom static HTML이므로 Rspress build에서 제외하고 원본을 두 Tailnet route에 복사한다.
- 실행 명령: `bun run --filter @spinon/docs docs:build --config /private/tmp/spinon-c15-3-scroll-tailnet-sync-20261017/packages/docs/.rspress.tailnet.config.ts`.
- 선택 입력 밖의 문서·이미지 링크가 섞이는 것을 막기 위해 이 build에서 `markdown.link.checkDeadLinks`와 `markdown.image.checkDeadImages`를 끄고, C15 route closure만 명시적으로 검사한다.

## 결과

Rspress는 선택된 16개 문서 route를 생성했고 종료 코드 0을 반환했다. 출력에는 `STATUS.html`, `plan/c15-inline-text-layout.html`, `internal/README.html`, 계약 0072–0074, 두 C15.3 실행·사전 비교 README와 검토 문서, 동기화 근거와 review route, 연결된 image 및 Rspress assets가 포함된다. 정확한 build output은 [selected-build.log](selected-build.log)다.

Rspress build log의 정렬용 줄 끝 공백만 제거해 Git diff 검사를 통과시켰다. 결과 문구·route 수·종료 코드는 유지했고, 원본 및 정규화 SHA-256을 `selected-build-log-normalization.json`에 기록한다.

선택 build는 저장소 전체 988개 Markdown 문서의 dead link/image 검사나 full docs build를 대신하지 않는다. Tailnet에 복사할 경로는 선택 output과 그 문서가 직접 가리키는 C15.3 evidence assets로 제한한다. 기존 preview 파일을 삭제하지 않는다.
