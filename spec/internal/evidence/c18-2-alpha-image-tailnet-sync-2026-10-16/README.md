# C18.2 Tailnet 미리보기 동기화 근거

## 대상과 결과

C18.2 구현 PR [#221](https://github.com/ohah/spinon/pull/221)의 병합 결과 커밋 `06f181b064f7d23644231133dae4253d145d4bf7`을 기준으로 Tailnet 문서 미리보기를 동기화했다. 이 자료는 미리보기 동기화와 브라우저 확인 근거이며 구현 완료 판정을 추가하거나 공식 상태 대장을 수정하지 않는다.

로드맵 [화면](./roadmap-c182-agent-browser.png)에서 C18.2 완료 항목과 `공식 완료 69/200`을 확인했다. 당시 표시는 상위 78개, 세부 ID 120개, ID 없는 하위 체크 2개를 합산한 200개 중 69개 완료였다. 이 숫자는 `main`의 `spec/STATUS.md`를 읽어 만드는 동적 표시이며 이 동기화에서 숫자를 직접 입력하지 않았다. [별도 카운터 화면](./roadmap-count-agent-browser.png)에도 같은 값이 보인다.

## 빌드와 미리보기 반영

- Rspress `2.0.22`로 C18.2 계획·상태/적합성·내부 명세와 근거에 한정한 미리보기 산출물 빌드를 완료했다. 저장소 문서 설정은 수정하지 않았다. 전체 사이트 릴리스 빌드나 GitHub Pages 배포를 의미하지 않는다.
- 빌드 입력에 사전 비교의 `chrome155.png`가 빠진 첫 스테이징 시도를 보완한 뒤 최종 빌드가 통과했다. 최종 빌드 로그는 [`rspress-scoped-build-output.txt`](./rspress-scoped-build-output.txt)에 있다.
- 매니페스트는 165개 경로, 6,162,931바이트로 기록한다: 새 파일 141개, 교체 12개, 기존과 동일 12개. 삭제는 0개이며 교체된 기존 파일 12개는 미리보기의 날짜별 백업 경로에 보존했다.
- 체크인한 [매니페스트](./manifest.json)는 서버 미리보기의 로컬 절대 경로를 제거하고 미리보기 URL과 상대 백업 경로만 남긴 공유용 사본이다. 서버에서 제공된 원본 매니페스트는 HTTP 200이었고 SHA-256은 `d8ffd426727aec34b07023ef7286c0134ce2b457c8d62e7dd8449cff23fcee2b`였다.
- Agent Browser의 fetch와 Web Crypto SHA-256으로 매니페스트에 있는 165개 URL을 확인했다. 모두 HTTP 200 및 해시 일치, 실패 0개였다. [검증 기록](./served-verification.json)에 시각·URL·집계값을 남겼다.
- 동기화는 복사만 수행했고 삭제를 하지 않았다. 미리보기 서버를 재시작하지 않았고 GitHub Pages를 배포하지 않았다.

## 실제 페이지 확인

2026-10-16 18:54 KST에 Agent Browser에서 다음 라우트를 각각 순서대로 열어 제목과 C18.2 내용을 확인했다.

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html): 제목, C18.2 완료 표시, 공식 카운터 `69/200`.
- [작업 계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c18-2-static-alpha-image-compositing.html): C18.2 계획 제목과 본문.
- [내부 명세](https://macstudio.tailed42f2.ts.net/spinon/spec/internal/0065-c18-2-alpha-image-compositing.html): caller-owned RGBA8 PNG와 source-over 계약.
- [상태 대장](https://macstudio.tailed42f2.ts.net/spinon/spec/STATUS.html): C18.2 상태 항목.
- [Android 전체 화면 캡처](https://macstudio.tailed42f2.ts.net/spinon/spec/internal/evidence/c18-2-alpha-image-android-api37-2026-10-16/full-screen.png): 로드 완료, 1080×2400.
- [iOS 전체 화면 캡처](https://macstudio.tailed42f2.ts.net/spinon/spec/internal/evidence/c18-2-alpha-image-ios-simulator-2026-10-16/full-screen.png): 로드 완료, 1206×2622.

두 캡처는 각각 Android API 37 ARM64 AVD의 PixelCopy 결과와 iPhone 17 Pro iOS 26.2 Simulator의 Metal 결과다. 이번 확인에는 실기기를 사용하지 않았다. 브라우저에서 보고된 깨진 이미지도 없었다. 브라우저 화면 캡처는 [`roadmap-c182-agent-browser.png`](./roadmap-c182-agent-browser.png), [`roadmap-count-agent-browser.png`](./roadmap-count-agent-browser.png), [`android-screenshot-agent-browser.png`](./android-screenshot-agent-browser.png), [`ios-screenshot-agent-browser.png`](./ios-screenshot-agent-browser.png)에 보존했다.

## 연결된 근거

- [C18.2 구현 PR #221](https://github.com/ohah/spinon/pull/221)
- [동기화 매니페스트](./manifest.json)
- [서빙·해시·라우트 검증 기록](./served-verification.json)
- [최종 범위 빌드 로그](./rspress-scoped-build-output.txt)
- [Tailnet 반영 변경 30개 관점 검토](../c18-2-alpha-tailnet-sync-pr-delta-review-30-2026-10-16.md)
