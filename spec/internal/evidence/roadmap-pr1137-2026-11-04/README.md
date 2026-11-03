# PR #1137 병합 후 상태·로드맵 문서 빌드 근거 · 2026-11-04

## 변경 기록

구현 PR [#1137](https://github.com/ohah/spinon/pull/1137)은 `6af14f69512c0c73d890efc728751e0ca342f9c0`로 병합됐다. 이 문서 PR은 해당 제한 slice의 결과를 `spec/STATUS.md`, 전용 계획, runtime evidence와 공개 `spec/public/roadmap.html`에 반영한다. C11.1/C11 체크와 공식 `72/213`은 유지한다.

## 선택 문서 route 빌드

전체 `mise exec -- bun run docs:build`는 변경하지 않은 기존 evidence Markdown의 dead link에서 실패했다. 예를 들어 C13.3 recognizer/event pairing과 C11 named-area runtime 자료에서 link-check 오류가 보고됐다. 이 실패를 전체 문서 빌드 통과로 세지 않는다.

변경한 계획·STATUS·구현 evidence·30개 diff review에 필요한 입력만 `prepare-selected-root.mjs`로 복사하고, 저장소와 같은 Rspress 2.0.22로 선택 root를 빌드했다. dead-link/dead-image 검사는 선택 build에서 끄므로 전체 링크 적합성은 판정하지 않는다. 선택 입력 목록과 SHA-256은 [selected-root manifest](./selected-root-pass03.json)에 있다.

- 준비 명령: `mise exec -- node spec/internal/evidence/roadmap-pr1137-2026-11-04/prepare-selected-root.mjs`
- 빌드 명령: `mise exec -- packages/docs/node_modules/.bin/rspress build --config spec/internal/evidence/roadmap-pr1137-2026-11-04/rspress.config.ts`
- 출력: `build/spinon/roadmap-tailnet-sync-pr1137-2026-11-04/pass03/site/`
- 필요한 plan, STATUS, C11.2.1, evidence README와 review HTML routes가 생성됐다. Simulator PNG 두 개도 bundled image asset으로 build됐다.

Standalone `spec/public/roadmap.html`은 Rspress page가 아니므로 빌드된 Markdown route와 별도로 미리보기 root의 `roadmap.html`로 복사한다. Tailnet 미리보기 반영은 이 문서 PR 병합 뒤 source hash·기존 파일 backup·열린 file handle을 확인하는 별도 단계이며, 이 build 결과는 게시 완료를 뜻하지 않는다. GitHub Pages workflow는 실행하지 않는다.

## 변경 검토

[상태·로드맵 후속 변경에 대한 적대 검토 30개](./roadmap-pr-diff-review-30.md)는 병합 사실, 제한 범위, 테스트와 simulator 경계, CI 상태, 공식 집계 및 published route를 각각 확인한다.
