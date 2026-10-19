# C11.4 PR #349 뒤 Tailnet 미리보기 동기화

**기준 원본:** PR #349 병합 커밋 `6867c2af5de4c3a953c2e846160334fd6d9590ee`.<br>
**범위:** 선택 Rspress 문서 집합의 재빌드와 기존 Tailnet 미리보기 경로의 정적 파일 갱신. 문서 미리보기 반영이며 앱 실행이나 제품 검증을 추가하지 않는다.

## 갱신 내용

- 로드맵의 C11.4 요약을 단일 scene에서 Ahem text intrinsic 10-scene 결과로 갱신했다. PR #349 병합, 양 플랫폼 frame·trace 결과, pixel 표본과 미완료 범위를 함께 적었다.
- 공식 상태 대장과 C11.4 계획은 PR #349 병합본의 설명을 유지하고, 내부 색인에서 최신 실행 근거와 이번 Tailnet 동기화를 연결했다.
- 새 실행 근거 페이지는 [C11.4 text intrinsic matrix](../c11-4-text-intrinsic-matrix-2026-10-19/README.html)다. 제한 fixture 결과이며 C11.4 완료나 전체 CSS 지원을 뜻하지 않는다.

## 선택 빌드

RSPress 2.0.22로 `0001-conformance`, `STATUS`, C11 Grid 계약·계획, 내부 색인과 선택된 C11.4 실행 근거를 빌드했다. 전체 문서 사이트 빌드는 하지 않았다. 이 선택 빌드에서 dead-link/image 검사는 꺼져 있으므로 별도 링크 점검과 Agent Browser 화면 확인을 기록한다. GitHub Pages 배포는 하지 않았다. 새 sync 근거와 Tailnet 정적 복사본에서는 호스트 전용 절대 경로를 가렸다. PR #349에 포함된 원본 동작 근거는 수정하지 않았다. pass-3 해시 목록은 가리기 전 파일의 이력용이며, 현재 파일은 pass-4 해시 목록으로 검증한다.

재현용 설정은 [Rspress 설정](./rspress.tailnet-sync.config.ts), 최종 선택 빌드 출력은 [빌드 로그](./selected-build-pass4-final.log)에 있다.

## Tailnet 복사와 제공 확인

최종 pass-4의 사전 인벤토리, 복사 결과, 교체 파일 백업, HTTP 응답·SHA-256 결과는 각각 [사전 확인](./preflight-pass4.json), [복사 확인](./copy-verification-pass4.json), [백업 확인](./backup-verification-pass4.json), [제공 응답 확인](./served-verification-pass4.json)에 기록한다. 전체 evidence 파일 목록과 SHA-256은 [해시 목록](./SHA256SUMS-pass4.txt)에 있다. 동기화는 새 파일을 추가하거나 같은 경로의 파일을 교체하며, 기존 preview 파일은 삭제하지 않는다.

Agent Browser의 [로드맵](./tailnet-roadmap-agent-browser.png), [상태 대장](./tailnet-status-agent-browser.png), [C11.4 계획](./tailnet-plan-agent-browser.png) 화면 캡처는 갱신된 Tailnet URL을 직접 연 결과다. 브라우저 세션과 그 프로세스는 캡처 뒤 종료한다.

## 검토와 범위

- [Tailnet 동기화 실패 관점 검토 20개](./sync-review-20.md)
- [PR 변경 적대 검토 30개](./pr-delta-review-30.md)

이 작업은 정적 문서 미리보기만 갱신한다. Android AVD·iOS Simulator 앱을 이 sync에서 실행하지 않았고, C11.4 완료 수를 올리지 않았다. 앱 실행 근거는 [PR #349](https://github.com/ohah/spinon/pull/349)의 별도 evidence를 따른다.
