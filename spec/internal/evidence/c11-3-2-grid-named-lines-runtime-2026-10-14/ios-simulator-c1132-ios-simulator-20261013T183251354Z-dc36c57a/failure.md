# C11.3.2 iOS 구조화 geometry 로그 실패 기록

- 실행 ID: `c1132-ios-simulator-20261013T183251354Z-dc36c57a`
- AppDelegate route 수정 뒤 named-line 성공·기대 거부 장면과 WGPU draw가 실행됐다.
- Metal drawable 표본은 2/2 RGBA exact였다.
- 검증 도구는 Android용 `SPINON_C1132_GRID_NODE_FRAME` 로그만 찾아 geometry 단계에서 실패했다. iOS runtime 보고서는 성공했지만 비교기가 읽을 구조화 frame 줄이 없었다.
- 수정: iOS fixture adapter가 runtime 보고서에서 frame을 파싱해 `SPINON_C1132_GRID_NODE_FRAME_RUN`으로 기록하고, 캡처 스크립트가 Android/iOS 두 로그 형식을 읽게 한다. 실패 시 앱 종료와 `failure.txt` 저장도 보장한다.
- 이 시도는 geometry gate 통과 근거가 아니다.
