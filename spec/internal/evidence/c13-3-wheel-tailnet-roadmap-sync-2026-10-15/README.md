# C13.3 Tailnet 로드맵 동기화 · 2026-10-15

## 범위

PR [#194](https://github.com/ohah/spinon/pull/194)이 main에 병합된 뒤, 별도 Tailnet 정적 미리보기 `https://macstudio.tailed42f2.ts.net/spinon/`에 해당 C13.3 로드맵과 실행 근거를 반영했다. 문서 기준 main SHA는 `28b3ba214fc681b6327da607288a990e55e96f06`이다. 이 동기화는 GitHub Pages 배포가 아니며 서비스 재시작도 하지 않았다.

PR #171의 Android, iOS, Chrome 이미지 URL은 Agent Browser에서 모두 정상 로드됐다. Tailnet에서 발견한 별도 문제는 wheel 실행 근거 Markdown의 상대 이미지 경로 여섯 개였다. Rspress가 이미지 파일을 찾을 수 있도록 `./`를 붙였고, 같은 파일에서 실제 존재하는 PR delta 검토 문서 링크도 연결했다.

## 생성 및 반영

- Rspress `2.0.22`로 제한 staging tree를 빌드했다. 임시 설정과 산출물은 저장소 밖/ignored build 디렉터리에 뒀다.
- 기존 파일 중 `roadmap.html`, `plan/c13-3-scroll-gestures.html`, `spec/STATUS.html`, `spec/internal/0063-c13-3-scroll-gestures.html`, `spec/internal/README.html`, `spec/internal/README.md` 여섯 개를 갱신했다. 각 원본은 덮기 전에 SHA-256을 기록하고 대상 트리 밖에 백업했다.
- C13.3 wheel 계획·실행 근거 HTML, 원본 evidence 파일, 새 fingerprint 정적 리소스를 추가했다. 기존 `index.html`, 다른 페이지와 파일은 건드리거나 삭제하지 않았다.
- 파일별 이전/새 SHA-256, 바뀐 경로, HTTP 상태 및 served hash는 [manifest](./manifest.json)에 있다. 로컬 복구용 백업은 `/private/tmp/spinon-tailnet-c13-3-wheel-sync-20261015/backup`에 보존했다.

## Agent Browser 확인

- Tailnet 로드맵에서 C13.3 wheel 항목과 실행 근거 링크를 열었다.
- 실행 근거 페이지의 Android/iOS 화면 6개가 모두 로드됐다. Android 원본은 각 1080×2400, iOS 원본은 각 1206×2622이며 `naturalWidth > 0`을 확인했다.
- 페이지에 연결된 로그·JSON·XML·텍스트 자료 12개가 HTTP 200을 반환했다.
- 동기화 manifest에 나열된 모든 경로를 Tailnet에서 가져와 HTTP 200과 SHA-256 일치를 확인했다. manifest 파일 자체도 HTTP로 다시 가져와 본문 일치를 확인했다.
- [로드맵 화면 캡처](./tailscale-roadmap-agent-browser.png)는 업데이트된 C13.3 wheel 설명과 근거 링크를 보여준다. 전체 실행 화면은 PR #194 본문과 Tailnet 실행 근거 페이지에 있다.

## 동작 범위

이 문서는 이미 합쳐진 C13.3 synthetic normalized wheel fixture 근거의 정적 미리보기만 기록한다. OS wheel/trackpad 입력 수집, 플랫폼별 단위 정규화, 공개 `WheelEvent`/callback, 실제 기기 및 compositor 이후 출력은 여전히 미검증이다. 전체 C13.3 완료를 뜻하지 않는다.
