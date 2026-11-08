# C11.4 진행 표시와 로드맵 링크 확인

## 확인한 혼동

2026-11-08 Agent Browser 캡처에서는 칸반이 `진행 중 1/2`로 렌더링됐고 카드에는 `Intrinsic sizing`과 `C11.4 · ACTIVE`가 표시됐다. 진행 데이터는 있었지만 영어 제목과 일반적인 보드 안내가 한국어 로드맵에서 C11.4 작업임을 즉시 드러내지 못했다. 칸반 제목 옆에 `진행 중: C11.4 Grid 트랙 크기`를 고정 표시하고, 카드 제목과 상태도 한국어로 바꿨다.

C11.4에는 별도 상위 ID가 없으므로 공식 완료 집계 `78/214`에는 포함되지 않는다. 로드맵은 공식 완료 수와 활성 세부 작업을 각각 표시하며 둘은 서로 다른 지표다.

## 수정 내용

로드맵 칸반 제목 옆과 C11.4 활성 카드에 `진행 중`을 한국어로 표시한다. C11.4에는 별도 상위 ID가 없으므로 공식 완료 집계 `78/214`에는 포함되지 않는다. 공식 완료 수와 활성 세부 작업은 별도 지표로 설명한다.

로드맵 카드가 가리킨 계획 문서와 실행 근거가 처음에는 Tailnet 미리보기에서 HTTP 404를 반환했다. 실패 결과는 [`plan-link-verification.json`](plan-link-verification.json)과 [`runtime-evidence-link-verification.json`](runtime-evidence-link-verification.json)에 보존했다. 선택된 RSPress 경로를 빌드해 미리보기 서버에 추가했고, 바뀐 기존 문서는 원본 백업 후 같은 디렉터리에서 원자 교체했다. Preview server PID 28040과 Tailscale 연결은 유지했고 서버를 재시작하거나 파일을 삭제하지 않았다.

## 검증 결과

- [선택 빌드 로그](selected-rspress-build.log)와 [로드맵 카드 로컬 링크 감사](selected-route-audit.json): 로컬 카드 경로 5개 확인. 전체 사이트 dead-link 검사는 하지 않았다.
- [최종 선택 게시 결과](selected-publication-result-pass03.json): 변경 파일, 백업 경로, 서버 상태와 해시를 기록한다.
- [최종 로컬·Tailnet 응답 검증](served-link-verification-pass03.json): 주요 문서·이미지와 공유 CSS/JS 요청의 상태 및 게시 해시를 기록한다.
- Agent Browser 데스크톱 1440×1000과 모바일 390×844에서 `진행 중: C11.4 Grid 트랙 크기`, `진행 중 1/2`, `C11.4 · 진행 중`을 확인했다. 문서 너비는 각 viewport와 같고 가로 overflow가 없었다. 모바일 캡처는 칸반을 활성 열로 이동해 카드도 보이게 했다.
- [데스크톱 상태](final-agent-browser-desktop.json) · [모바일 상태](final-agent-browser-mobile.json) · [데스크톱 화면](final-roadmap-agent-browser-desktop.png) · [모바일 화면](final-roadmap-agent-browser-mobile.png)
- 계획·비교·실행·PR diff 리뷰·이 작업 README를 Agent Browser로 열어 제목과 본문을 확인했다. [세부 상태 기록](agent-browser-plan.json), [비교](agent-browser-precomparison.json), [실행](agent-browser-runtime.json), [PR diff 리뷰](agent-browser-pr-diff-review.json), [로드맵 증거](agent-browser-roadmap-evidence.json).
- Agent Browser 종료 기록은 최종 종료 후 [`agent-browser-lifecycle-final.json`](agent-browser-lifecycle-final.json)에 추가한다.

## 적대적 검토

- [화면·상태 실패 관점 20개](roadmap-status-review-20.md)
- [변경 diff 실패 관점 30개](roadmap-pr-diff-review-30.md)

## 기타 증거

- 이전 화면의 Agent Browser 추출: [`agent-browser-desktop.txt`](agent-browser-desktop.txt), [`agent-browser-mobile.txt`](agent-browser-mobile.txt)
- PR #1277 Android/iOS 이미지 원본 확인: [`android-image-verification.json`](android-image-verification.json), [`ios-image-verification.json`](ios-image-verification.json)
- 잘못된 iOS URL 진단 파일 [`ios-image-verification-bad-url.json`](ios-image-verification-bad-url.json)은 올바른 이미지 URL 검증과 혼동하지 않도록 보존했다.
