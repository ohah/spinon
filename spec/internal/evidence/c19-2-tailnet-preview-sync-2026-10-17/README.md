# C19.2 Tailnet 미리보기 동기화 및 제공 검증 · 2026-10-17

이 기록은 PR #232의 C19.2 결과와 PR #233·#234 문서 변경을 Tailnet 미리보기 경로에 동기화하고, 실제 Agent Browser가 새 내용을 받는지 확인한다. 공식 완료 상태를 바꾸는 작업은 아니다.

## 입력과 빌드

- 기준 `main`: `9b8d74f210f9419820cff25fbce2a1c5db300622`, tree `21915e498add5f2a569832fa7033ea711864689d`.
- 격리 Rspress `2.0.22` build는 C19.2 상태·계약·실행 근거, C19.1 색 근거, 로드맵과 내부 검토 페이지 범위에 한정했다. 복사한 104개 입력은 기준 tree 파일과 byte 단위로 일치한다. build는 0.61초에 완료됐다.
- 전체 문서 build와 전체 dead-link 검사는 실행하지 않았다. 기존 무관 링크 때문에 dead-link 검사는 임시 scoped config에서만 비활성화했다. 여섯 개 PNG import는 실제 hashed `static/image` 출력물로 생성됐다.
- 저장소의 Rspress 설정과 lockfile은 수정하지 않았고 GitHub Pages도 배포하지 않았다. 입력 경계와 결과는 [build 기록](./scoped-build-record.json), 출력 로그는 [scoped-build.log](./scoped-build.log)에 있다.
- 이 PR의 새 evidence 페이지와 내부 인덱스도 별도 scoped config로 다시 build했다. 관련 세 route가 생성되고 build가 성공했다. [PR build 결과](./pr-build-check.json)와 [출력 로그](./pr-build-check.log)를 남겼다.

## 복사 결과

미리보기 서버는 저장소를 직접 제공하지 않고 `~/Library/Application Support/SpinonPreview`를 제공한다. 그래서 저장소 파일을 바꾸는 대신, 생성된 사이트 파일만 기존 preview root에 복사했다. [manifest](./manifest.json), [사전 검사](./preflight.json), [복사 후 검사](./copy-verification.json)는 이 복사를 재현하고 확인할 수 있게 한다.

| 결과 | 수치 |
|---|---:|
| 전달 파일 | 108개 · 5,615,740 bytes |
| 새 파일 / 교체 / 동일 | 81 / 14 / 13 |
| 삭제 | 0 |
| 교체 파일 백업 | 14개, 전부 해시 확인 |
| manifest SHA-256 | `e940d9062d556d273983d976e81bcf877a739dc1170b6d789be7aea076075dc0` |

복사 후 Agent Browser에서 108개 URL을 각각 가져와 HTTP 200, 바이트 수, SHA-256을 모두 manifest와 대조했다. 108개가 전부 일치했다. Tailnet Serve의 기존 `/` 경로와 PID 1191 preview 서버는 재시작하지 않았다.

## 실제 화면 확인

Agent Browser에서 다음 Tailnet URL을 열었다. 스크린샷도 이 브라우저 세션에서 저장했다.

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html): `203개 표시 · 공식 완료 71/203`. C19와 C19.2 체크박스는 모두 비활성·미체크 상태다.
- [공식 상태 대장](https://macstudio.tailed42f2.ts.net/spinon/spec/STATUS.html): C19 및 C19.2가 아직 미완료임을 확인했다.
- [C19.2 Simulator 근거](https://macstudio.tailed42f2.ts.net/spinon/spec/internal/evidence/c19-2-runtime-simulators-2026-10-17/README.html): Android 1080×2400, iPhone 1206×2622, iPad 2064×2752 캡처 여섯 장이 모두 로드됐다.
- PR #235 본문에 넣은 세 화면 캡처도 Agent Browser로 열어 모두 로드됐음을 확인했다. 화면은 각각 1680×1200이다. 확인 후 `spinon-c19-roadmap-pr-20261017` 브라우저 세션을 닫았고 session PID 15294가 사라졌다. 기존 Tailnet 서버 PID 1191은 그대로 실행 중이며 8768 포트에 LISTEN만 남았다.

이 화면의 count는 `spec/STATUS.md`에서 동적으로 만든다. 이전 61/197 또는 62/197 수치는 이전 상태의 snapshot이며 현재 값은 71/203이다. 이 PR은 count를 HTML에 고정하지 않으며 `spec/STATUS.md`를 변경하지 않는다.

주요 화면: [로드맵](./roadmap-agent-browser.png), [동적 count](./roadmap-counter-agent-browser.png), [C19 상태](./roadmap-c19-status-agent-browser.png), [C19.2 대장](./status-c19-2-agent-browser.png), [시뮬레이터 근거](./runtime-evidence-agent-browser.png).

## 판정 범위

이 기록은 복사된 정적 파일의 Tailnet HTTP 응답과 브라우저 표시만 확인한다. 전체 문서 build, Pages 배포, Android 실기기, Android hardware GPU, iOS compositor 이후 출력은 검증하지 않는다. C19와 C19.2는 계속 미완료다.
