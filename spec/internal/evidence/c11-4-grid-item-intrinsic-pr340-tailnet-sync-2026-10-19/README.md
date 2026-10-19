# C11.4 PR #340 Tailnet 문서 미리보기 동기화

**날짜:** 2026-10-19
**문서 원본 커밋:** `d8881d9818e3629f6159637f40021ee0a95a4fdf` (PR #340 리베이스 병합)

## 범위

PR #340 병합 뒤 `spec/STATUS.md`, C11.4 계획·검토 문서와 로드맵을 작업 중인 Tailnet 미리보기에 반영했다. 공개 로드맵 수는 기존처럼 **공식 완료 72/212**이며 이번 C11.4 raw intrinsic input 변경으로 완료 수를 올리지 않았다. 상태 대장도 C11.4를 미완료로 유지한다.

이 디렉터리는 문서 미리보기 배포·관찰 근거다. 모바일 앱 화면, 앱 런타임, Android/iOS 시뮬레이터나 실기기 동작을 검증한 자료가 아니다.

## 렌더와 동기화

Rspress `2.0.22`로 Markdown 경로 15개를 선택 렌더링하고 로드맵 및 연결된 정적·근거 자산을 포함해 77개 배포 대상을 준비했다. 렌더 명령은 성공했다. 전체 문서 빌드·링크 검사 통과를 주장하지 않는다. 선택 렌더의 범위와 출력은 [render-build.log](render-build.log)에 있다.

사전 조사·복사 결과는 다음과 같다.

- 총 대상 77개: 기존 파일 53개, 교체 19개, 신규 24개, 동일 34개
- 삭제 0개, 복사 후 불일치 0개
- 미리보기 바깥의 백업 파일 20개를 SHA-256으로 확인했고 불일치 0개
- `404.html`은 서버에서 읽기 전용으로 열려 있어 교체하지 않았다. 복사 대상 집계에서 제외했고 기존 백업과 해시가 같은 상태로 남았다.
- Tailnet HTTPS에서 77개 대상 모두 HTTP 200을 반환했고 77개 모두 원본 렌더 파일과 SHA-256이 같았다.

원자료는 [preflight.json](preflight.json), [backup-verification.json](backup-verification.json), [copy-verification.json](copy-verification.json), [served-verification.json](served-verification.json)에 있다. 미리보기 호스트와 로컬 절대 경로는 기록에서 가렸다.

## Agent Browser 화면 확인

Agent Browser로 로드맵, 상태 대장, C11.4 계획 및 PR diff 검토 경로를 열어 화면에 표시되는 문구를 확인하고 캡처했다. 각 경로·확인 문구는 [browser-verification.json](browser-verification.json)에 적었다.

- [로드맵의 완료 수](tailnet-roadmap-counter-agent-browser.png)
- [상태 대장의 C11.4 범위](tailnet-status-agent-browser.png)
- [상태 대장의 PR #340 링크](tailnet-status-pr340-agent-browser.png)
- [C11.4 계획](tailnet-plan-agent-browser.png)
- [PR diff 검토 문서](tailnet-pr-review-agent-browser.png)

확인 뒤 이 작업에서 연 Agent Browser 세션을 닫았다. 도구의 세션 목록에는 닫힌 세션 이름이 비활성 기록으로 남았지만, `session info`는 `active=false`, `pid=null`을 반환했고 브라우저 프로세스 확인도 비어 있었다. 종료 근거는 [browser-session-shutdown.json](browser-session-shutdown.json)에 있다. Terminal Browser는 사용하지 않았다.

## 남은 범위

이 기록은 선택된 문서 경로의 렌더·전송·브라우저 표시만 확인한다. 전체 사이트 dead-link 검사, 전체 Rspress 사이트 생성, 앱 동작, CSS 호환성, Android/iOS runtime 또는 시뮬레이터/실기기 확인을 뜻하지 않는다. C11.4의 CSS contribution 계산, track sizing 및 나머지 계획 범위도 완료로 바뀌지 않았다.
