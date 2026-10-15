# C13.3 복합 wheel 근거 Tailnet 로드맵 반영 · 2026-10-16

[PR #202](https://github.com/ohah/spinon/pull/202)에서 병합된 Android API 37 AVD 복합 source 결과를 정적 Tailnet 로드맵과 연결했다. 로드맵 원문은 [spec/public/roadmap.html](https://github.com/ohah/spinon/blob/main/spec/public/roadmap.html)이다. C13.3은 iOS native callback, 물리 복합 입력장치, 제품 입력 경로 등의 조건이 남아 있어 미완료로 유지하며 완료 체크를 바꾸지 않았다.

- 로드맵: <https://macstudio.tailed42f2.ts.net/spinon/roadmap.html>
- 새 근거 페이지: <https://macstudio.tailed42f2.ts.net/spinon/spec/internal/evidence/c13-3-native-wheel-composite-avd-2026-10-16/README.html>
- Agent Browser에서 로드맵의 새 요약과 링크, 공식 카운터 `197개 표시 · 공식 완료 61/197`을 확인했다.
- 근거 페이지에서 `0x10200a` composite source union과 `0x2002` mouse ACTION_SCROLL을 확인했다. 네 screenshot이 모두 1080×2400으로 로드됐고 깨진 이미지는 없었다.
- Tailnet 정적 루트에 103개 경로만 복사했다: 16개 교체, 42개 추가, 45개 내용 동일. 교체 전 파일은 `/private/tmp/spinon-tailnet-c13-3-roadmap-composite-sync-20261016/backup/`에 SHA-256과 함께 백업했다. 파일 삭제, 서버 재시작, GitHub Pages 배포는 없었다.

## 확인 이미지와 기계 판독 근거

- [로드맵 요약](./roadmap-agent-browser.png)
- [복합 AVD 근거 페이지](./composite-evidence-agent-browser.png)
- [로드맵 공식 완료 수](./roadmap-count-agent-browser.png)
- [Agent Browser 결과](./tailnet-served-verification.json)
- [복사 전후 SHA-256 manifest와 백업 경로](./tailnet-sync-manifest.json)
- PR #203 본문 이미지도 Agent Browser에서 확인했다. 세 개의 `raw.githubusercontent.com` 이미지는 각각 1280×633으로 로드됐고 깨진 이미지가 없었다. [PR 이미지 렌더링 화면](./pr-203-images-agent-browser.png) · [검증 JSON](./pr-body-image-verification.json).

- [PR 변경 적대 검토 20개](./pr-delta-review-20.md) · [PR 본문 이미지 배치 화면](./pr-203-images-agent-browser.png) · [PR 이미지 검증 JSON](./pr-body-image-verification.json).
