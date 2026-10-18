# 선택 문서 렌더 결과

- Rspress: `@rspress/core 2.0.22`
- 설치: `mise exec -- bun install --frozen-lockfile` 완료
- 빌드: `mise exec -- env RSPRESS_PERSISTENT_CACHE=false ./packages/docs/node_modules/.bin/rspress build --config packages/docs/rspress.selected.config.ts`
- 선택 root: C13.3 계획·상태·내부 인덱스/계약·실행 근거·Tailnet 동기화 문서 55개 파일
- HTML route: 13개(404 fallback 1개 포함). 새 동기화 README와 build/PR 검토 기록도 별도 route로 렌더한다.
- 새 동기화 README route: `internal/evidence/c13-3-ios-app-event-boundary-tailnet-sync-2026-10-18/README.html`
- Rspress 기록: Pages rendered, web/node build 성공

선택 root는 저장소 전체 문서 집합이 아니다. 선택 root 밖의 상대 링크와 이미지는 빌드 입력에 없으므로 `markdown.link.checkDeadLinks` 및 `markdown.image.checkDeadImages`를 해당 선택 빌드에서 비활성화했다. 그 결과는 전체 링크 검사나 전체 docs build 성공을 뜻하지 않는다. 저장소의 임시 선택 config는 빌드 후 제거했다.
