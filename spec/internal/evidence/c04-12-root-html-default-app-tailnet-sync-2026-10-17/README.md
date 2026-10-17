# C04.12 기본 앱 루트 HTML Tailnet 로드맵 동기화

**상태:** PR #272 병합본의 문서 미리보기 동기화·제공 검증 · C04.12 미완료
**기준 커밋:** `54a39a3e7403dd7fc9178de17daeafec4f00ec88` · **실기기:** 사용하지 않음

## 동기화 범위

PR #272에서 갱신한 로드맵, 상태 대장, 내부 계약, C04.12 계획, 기본 앱 Android/iOS Simulator 근거와 관련 검토 문서를 Tailnet 미리보기에 반영한다. Rspress 2.0.22는 선택한 문서만 임시 문서 트리에서 렌더링했다. 전체 `docs:build`의 기존 dead-link 및 code-fence 언어 오류는 이 빌드 결과에 포함되지 않으며, 이번 선택 빌드는 저장소 전체 링크 검사를 대신하지 않는다.

미리보기 주소는 [스피논 Tailnet 문서](https://macstudio.tailed42f2.ts.net/spinon/)다. 업데이트된 로드맵의 C04.12 항목은 미완료이며 공식 완료 집계는 상태 대장을 읽는다. 결과 수치와 각 파일의 바이트·SHA-256은 [preflight](preflight.json), [copy verification](copy-verification.json), [served verification](served-verification.json)에 기록한다.

## 복사·제공 검증

- 동기화는 복사 전용이다. 기존 Tailnet 경로를 삭제하지 않는다.
- 교체 대상은 별도 로컬 백업 경로에 먼저 보존하고 백업 hash를 검증한다. manifest는 복사·교체·동일 파일을 분리한다.
- 로컬 미리보기와 Tailnet endpoint에서 대상 경로별 HTTP status, byte length, SHA-256을 비교한다.
- 로드맵의 C04.12 미완료 표기와 공식 `205개 표시 · 공식 완료 71/205` counter는 [Agent Browser 화면](tailnet-roadmap-agent-browser.png)에서 확인한다. 상태 대장의 C04.12 체크가 꺼져 있는 모습은 [별도 캡처](tailnet-status-agent-browser.png)로 보존한다.
- Agent Browser 세션을 닫은 뒤 비활성 및 PID 부재를 확인한다. 기존 Tailnet Serve listener는 재시작하지 않고 tmux 상태도 바꾸지 않는다.

## 근거 파일

- [선택 Rspress 빌드 결과](render-build-result.md), [빌드 로그](selected-build.log)
- [복사 전 파일 목록과 hash](manifest.json), [백업 검증](backup-verification.json)
- [HTTP 제공 검증](served-verification.json), [Agent Browser 확인 값](browser-verification.json)
- [PR 변경 실패 관점 30개](pr-delta-review-30.md), [근거 파일 SHA-256](SHA256SUMS)

이 미리보기는 문서 배포 확인이다. Simulator의 빈 native host 캡처는 HTML UI 시연이 아니며, viewport/page-scale·OS 색상 선호·`prefers-color-scheme`·CSS `color-scheme` 효과와 HTML 화면 렌더는 여전히 미구현이다.
