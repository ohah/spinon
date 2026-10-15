# C13.3 Tailnet 로드맵 동기화 · 2026-10-16

## 범위

[PR #196](https://github.com/ohah/spinon/pull/196) 병합 뒤 C13.3 네이티브 wheel·trackpad 입력 수집 계획을 [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에 반영했다. `spec/public/roadmap.html`의 현재 C13.3 설명에 [입력 수집·단위 정규화 계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c13-3-native-wheel-input.html) 링크를 추가했다. 동기화 기준 main commit은 `9ad66ef64f57e275477c5553789dac1bfd030c90`이다.

이 변경은 문서 미리보기 갱신이며 native 입력 구현이나 완료 판정 변경이 아니다. `spec/STATUS.md`는 수정하지 않았다.

## 생성과 반영

- 저장소에 고정된 Rspress `2.0.22`로 C13.3 계획·관련 근거·상태 문서의 제한 staging 빌드를 통과시켰다. 임시 설정에서만 dead-link·dead-image 검사를 끄고 저장소 Rspress 설정은 바꾸지 않았다. [공백을 정리한 빌드 출력](./rspress-scoped-build-output.txt)
- 전체 문서 빌드는 저장소에 이미 있던 여러 dead link에서 실패했다. 전체 빌드 성공으로 보고하지 않는다.
- Tailnet 정적 미리보기 후보 42개를 반영했다: 신규 21개, 교체 7개, 그대로 둔 파일 14개. 교체 전 원본 7개를 미리보기 경로 밖에 백업했다. 파일은 임시 파일을 만든 뒤 교체했다. 기존 파일은 삭제하지 않았다.
- GitHub Pages workflow를 실행하거나 미리보기 서비스를 재시작하지 않았다.
- 이전·새 파일 SHA-256, 크기와 변경 상태는 [동기화 기록](./manifest.json) 및 machine-specific 경로를 제외한 [Tailnet manifest 사본](./tailnet-manifest.json)에 있다. 원본 manifest의 SHA-256과 Agent Browser가 다시 가져온 후보 파일 결과는 [served 검증 기록](./served-verification.json)에 보존했다.

## Agent Browser 확인

- Tailnet 로드맵에서 **입력 수집·단위 정규화 계획** 링크의 실제 목적지가 `https://macstudio.tailed42f2.ts.net/spinon/plan/c13-3-native-wheel-input.html`임을 확인했다. 계획 페이지의 제목과 `normalization-oracle.json` 링크도 표시됐다.
- 계획의 구현 전 비교 페이지가 열리고, 기존 Chrome 155 기준 이미지가 `400×240`으로 정상 로드됐다. 정규화 JSON과 동기화 manifest를 포함한 42개 후보 파일 모두 HTTP `200` 및 예상 SHA-256 일치였고, manifest 자체도 HTTP `200`과 SHA-256 일치였다.
- 로드맵 확인 시각인 2026-10-16 00:17:14 KST에 GitHub `main`의 공식 상태 대장은 **61/197개 완료**를 표시했다. 완료 61개 중 상위 항목은 **4/78개**, 세부 체크는 57개다. 이번 링크 추가는 상태 수를 바꾸지 않았다.
- [Tailnet 로드맵 화면](./tailscale-roadmap-agent-browser.png)과 [계획 페이지 화면](./native-wheel-plan-agent-browser.png)은 Agent Browser에서 캡처했다.

## 완료 범위

이 미리보기는 계획 탐색을 돕는다. 실제 Android/iOS mouse wheel 또는 trackpad 입력, OS별 단위 정규화 실행, 공개 `WheelEvent`·scroll API, 실기기 동작이나 compositor 이후 표시는 검증하지 않았다. C13.3 또는 공개 CSS 지원 완료를 뜻하지 않는다.
