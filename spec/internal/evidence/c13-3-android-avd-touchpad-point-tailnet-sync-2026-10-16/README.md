# C13.3 ADB touchpad Tailnet 로드맵 동기화 · 2026-10-16

## 반영 범위

[PR #212](https://github.com/ohah/spinon/pull/212)의 feature branch `dd0630fc85c99e192620f3c074153ae7bbd2a01f`에서 로드맵 설명과 ADB touchpad evidence 링크, 후속 계획, C13.3 내부 목록·상태 설명과 실행 화면·로그를 Tailnet 개발 미리보기에 반영했다. PR base `main`은 `d8b85fc5a4f0e5ed7e12e7d3e704416e9c36a94e`였다. 공식 상태 카운터는 GitHub main의 `spec/STATUS.md`를 읽으며 수동 변경하지 않았다.

Rspress `2.0.22` 격리 staging 빌드에서 10개 문서 페이지와 참조 자산을 만들었다. PR 변경 검토와 이 동기화 기록의 `.html` route를 포함한다. 최초 빌드는 README의 같은 디렉터리 이미지 경로에 `./`가 빠진 점과 임시 root의 favicon 누락으로 실패했다. README 경로를 `./touchpad-origin-target.png`로 고치고 public icon/logo/roadmap 자산을 staging root에 추가한 뒤 scoped build가 성공했다. 저장소 Rspress 설정은 수정하지 않았다. [최종 scoped build 출력](./rspress-scoped-build-output.txt).

최종 preview manifest는 64개 경로다. 세 번째 pass의 62개에서 더 이상 생성되지 않는 hashed bundle 3개를 manifest에서 제외하고, 현재 scoped build의 bundle 4개와 새 README.html route 1개를 포함해 2개 순증했다. 첫 sync는 58개 중 새 파일 38개, 교체 8개, 동일 hash 유지 12개였고 기존 파일 8개를 백업했다. Rspress 재생성 뒤 일관성을 맞춘 두 번째 pass는 58개 중 새 asset 3개, 교체 10개, 유지 45개였고 11개 이전 파일을 추가 백업했다. 보고서와 supporting artifact를 넣은 세 번째 pass는 62개 중 새 파일 4개, 유지 58개였고 직전 preview manifest를 백업했다. `.html` 보고서 route를 추가한 네 번째 pass는 64개 중 새 파일 5개, 교체 12개, 유지 47개였고 교체 파일 12개와 이전 manifest 1개를 추가 백업했다. 총 33개 이전 파일을 전용 backup tree에 보존했다. 파일별 SHA-256과 backup 경로는 [최종 동기화 manifest](https://macstudio.tailed42f2.ts.net/spinon/sync-manifest-c13-3-android-avd-touchpad-point-2026-10-16.json)에 있다. 삭제는 없었다.

## Agent Browser 검증

2026-10-16 08:40 KST에 Tailnet roadmap에서 `공식 완료 65/197`과 새 `ADB touchpad raw-origin AVD 실행·화면 근거` 링크를 확인했다. 카운터는 이번 변경으로 수동 고정하지 않았으며 공식 main 상태의 동적 표시다. [로드맵 Agent Browser 캡처](./roadmap-agent-browser.png).

새 evidence 페이지는 제목과 AVD 한정 설명을 표시했고 embedded screenshot은 `complete=true`, visible, 1080×2400으로 로드됐다. [evidence page Agent Browser 캡처](./evidence-page-agent-browser.png). 최종 manifest와 64개 대상은 Agent Browser fetch와 Web Crypto SHA-256으로 확인했으며, 전체 HTTP 200·hash 일치 64/64, 실패 0이다. 검증 명령 결과는 [PR #212의 `served-verification.json`](https://github.com/ohah/spinon/pull/212)에서 볼 수 있다.

## 범위와 남은 점

이 작업은 정적 Tailnet 개발 미리보기 파일만 바꿨다. GitHub Pages workflow를 실행하거나 미리보기 서버를 재시작하지 않았다. 전체 문서 사이트 build는 실행하지 않았으며 scoped build를 전체 사이트 build 성공으로 확대해 주장하지 않는다. PR 병합 뒤 같은 제공 경로와 공식 카운터를 확인했다.
