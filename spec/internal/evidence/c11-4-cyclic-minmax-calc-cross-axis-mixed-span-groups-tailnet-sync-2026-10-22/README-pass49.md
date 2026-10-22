# C11.4 mixed cross-axis span groups Tailnet 미리보기 · pass49

## 빌드와 동기화

- RSPress `2.0.22`로 선택 문서 사이트를 빌드했다. 원본 빌드 요약은 [`selected-build-summary-pass49.json`](./selected-build-summary-pass49.json)에 있다. 처음 생성한 사이트는 6,997개 파일·388,574,684 bytes였다.
- Tailnet 미리보기의 6,997개 경로를 해시로 대조했고, 전체 응답 6,997/6,997과 백업 446/446이 일치했다. 동기화 중 preview 경로 삭제는 없었다. 전체 업로드 결과는 [`final-verification-pass49.json`](./final-verification-pass49.json), HTTP와 route overlay 결과는 [`served-summary-pass49.json`](./served-summary-pass49.json)에 기록했다. 6,997개별 파일 원장은 자동 생성 행이 많아 PR에는 넣지 않고 로컬 evidence에 보존했다.
- 공개 roadmap의 runtime 진행 항목과 계획·상태·실행 근거 링크를 Agent Browser로 확인했다. 원래 로드맵 화면은 [`tailnet-roadmap-agent-browser-pass49.png`](./tailnet-roadmap-agent-browser-pass49.png)에 있다.

## 실행 근거 페이지 정정

- 실행 근거 Markdown에서 이미 완료한 구현/runtime 20개와 PR diff 30개 검토가 “남은 검증”에 잘못 표시된 것을 발견해 `검증 상태`로 정리했다. 남은 Android definite draw/PixelCopy, 4개 AVD scene 행렬과 C11.4 회귀 matrix는 미완료로 남겼다.
- RSPress가 수정 전 route bundle을 hydrate해 이전 화면을 다시 표시했다. HTML만 고치면 부족해 HTML, 두 경로의 route bundle, 검색 인덱스, Markdown 복사본을 함께 정정했다. 최초 HTML 백업과 후속 생성 파일 백업은 별도 task backup 아래 보존했다. 두 적용 모두 쓰기 전에 해당 파일의 열린 handle이 없음을 확인했고, 미리보기 경로는 삭제하지 않았다.
- 최종 Agent Browser 확인은 [`agent-browser-verification-pass49.json`](./agent-browser-verification-pass49.json)에 있다. 검증 상태와 완료한 리뷰가 보이고 오래된 문구가 사라졌으며, Android 제한과 두 플랫폼 화면 이미지도 유지된다. Tailnet HTML 응답 SHA-256은 `807372a0a6e99f40f2cb70157af8a32830ffdb999e53ebf99a0277947fbdfd5c`다.
- 이 단일 route 수정은 선택 RSPress 전체 재빌드가 아니라 기존 pass49 출력의 국소 overlay다. 따라서 [`selected-build-summary-pass49.json`](./selected-build-summary-pass49.json)의 전체 사이트 tree hash는 최종 overlay 상태를 나타내지 않는다. 다음 전체 RSPress 빌드가 route·검색 자산의 content hash 이름을 다시 생성해야 한다.

## 종료 상태

- Agent Browser 세션 0개, 해당 브라우저 및 실행으로 생성한 Chrome 프로세스 0개를 확인했다. 기존 Chrome PID 20311과 Tailnet 미리보기 서버 PID 1191은 그대로 두었다.
- 공식 C11.4 체크와 완료 수 `77/213`은 유지한다. Android definite presentation과 기존 C11.4 전체 회귀 matrix가 남아 있다.
