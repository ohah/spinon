# C11.4 구현 로드맵 Tailnet 미리보기 갱신

**날짜:** 2026-10-19

C11.4 제한 후보의 현재 결과와 미완료 범위를 구현 상태 대장, 세부 계획, 내부 계약과 웹 로드맵 미리보기에 반영했다. 이 작업은 문서 미리보기 갱신이며 앱 배포나 C11.4 완료 판정이 아니다.

## 빌드와 반영

- Rspress 2.0.22 선택 빌드가 새 출력 디렉터리에서 성공했다. 선택 문서의 서로 연결된 계획·상태·계약·증거 페이지를 렌더링했다.
- 전체 문서 빌드는 C11.4와 무관한 기존 dead link와 이미지 import 오류가 있어 성공하지 않았다. 선택 빌드 설정에서 dead-link 검사를 끄고 대상 경로를 Agent Browser로 직접 확인했다.
- 선택 경로 10개와 필요한 정적 자산 6개, 총 16개 변경 파일을 기존 SpinonPreview 루트에 원자적으로 복사했다. 기존 파일은 복사 전 lsof 검사 후 별도 백업에 보존했고, 모든 대상의 SHA-256을 확인했다. 상세 old/new hash와 백업 위치는 manifest.json에 있다.
- Terminal Browser는 사용하지 않았다. Agent Browser로 확인했고, 기존 Tailnet 미리보기 서버 PID 1191은 그대로 유지했다.

## Agent Browser 확인

- /spinon/roadmap.html: C11.4 Taffy 0.15.0, Chrome 155 비교, Android/iOS Simulator와 남은 미완료 범위를 표시한다.
- /spinon/STATUS.html: 기준 날짜 2026-10-19이며 C11.4 행은 unchecked 상태와 미완료 설명을 유지한다.
- /spinon/plan/c11-4-grid-track-sizing.html: 최신 코드 검토 20개와 PR diff 검토 30개 링크를 표시한다.
- PR diff 검토 페이지에서 본문 표 30행과 미완료 결론을 확인했다.
- auto-repeat Simulator 페이지에서 Android 1080×2400, iOS 1206×2622 캡처가 모두 complete=true로 로드됐다.
- 확인 중 JavaScript 오류는 없었다. 캡처는 이 폴더의 tailnet-*-agent-browser.png 파일이다.
- 검증 뒤 브라우저를 닫았고 agent-browser session list는 빈 목록이었다. Agent Browser/Chromium 프로세스도 남지 않았다. 미리보기 서버는 계속 실행 중이다.

스크린샷과 선택 빌드 로그는 Tailnet 미리보기의 문서 렌더링 근거다. 앱 화면 동작은 별도 Android API 37 AVD·iOS 26.2 Simulator 근거를 따른다.
