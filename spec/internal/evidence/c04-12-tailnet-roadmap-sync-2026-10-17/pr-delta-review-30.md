# C04.12 Tailnet 로드맵 동기화 PR 변경 검토 30개

이번 변경은 로드맵 HTML과 배포·브라우저 검증 기록이다. 각 항목은 독립적인 상태·문서·배포 실패 경로를 확인한다.

1. **이전 완료 수가 현재 값으로 남는가?** `61/197`을 이전 스냅샷으로 표시하고 현재 수를 상태 대장 집계에서 읽는다. 통과.
2. **공식 수가 페이지의 동적 값과 일치하는가?** Agent Browser가 `206개 표시 · 공식 완료 73/206`을 읽었다. 통과.
3. **C04.12 상위 항목이 하위 작업 때문에 완료 처리되는가?** STATUS의 상위 체크는 unchecked다. 통과.
4. **viewport 하위 항목만 체크됐는가?** STATUS의 해당 하위 체크는 checked이며 다른 C04.12 하위 항목과 혼동되지 않는다. 통과.
5. **새 문장이 renderer 작업을 완료로 확대하는가?** renderer transform과 일반 HTML paint를 미지원으로 명시한다. 통과.
6. **고정 폭 viewport 숫자가 layout/visual 값과 뒤바뀌는가?** `500×1082.0513` layout과 `390×844` visual을 구분한다. 통과.
7. **Simulator 화면이 HTML body 출력으로 오해되는가?** PASS 보고 화면이라고 캡션과 본문에서 제한한다. 통과.
8. **기기 유형이 부풀려지는가?** AVD와 iOS Simulator만 적고 실기기 동작은 미검증으로 남긴다. 통과.
9. **Chrome 기준이 구현 근거처럼 표시되는가?** Chrome은 oracle로, app fixture는 별도 실행 근거로 나눈다. 통과.
10. **새 viewport 계획 링크가 preview에서 404인가?** Agent Browser에서 계획 페이지를 열어 내용을 확인했다. 통과.
11. **STATUS 링크와 계획 route 경로가 다른가?** 외부 Tailnet 계획 URL과 배포 경로가 일치한다. 통과.
12. **viewport evidence README의 `.html` 경로가 누락되는가?** 첫 복사에서 발견한 404를 추가 복사로 해결하고 두 호스트에서 재확인했다. 통과.
13. **내부 계약 링크가 낡은 내용 또는 404로 연결되는가?** 계약 0077 route의 제목과 범위 문장을 확인했다. 통과.
14. **Markdown 계획의 evidence 상대 경로가 `spec/`를 중복하는가?** preview용 원본 Markdown에서 경로를 `../internal/`로 맞춘다. 통과.
15. **Markdown 원본과 HTML route 중 하나만 게시되는가?** STATUS·계획·계약·근거 페이지의 원본과 렌더본을 함께 게시한다. 통과.
16. **README 이미지 참조가 파일 이름 변경으로 깨지는가?** Android·iOS screenshot의 자연 크기와 loaded 상태를 확인했다. 통과.
17. **브라우저 화면 증거가 앱 화면으로 혼동되는가?** Tailnet 문서 화면 캡처와 PR #277 Simulator report 캡처를 별도 설명한다. 통과.
18. **선택 빌드가 저장소 전체 문서 build로 오해되는가?** 선택 root와 `checkDeadLinks=false`를 명시하고 전체 build 실패 로그를 별도로 보존한다. 통과.
19. **전체 build의 기존 누락 자산이 새 파일 원인으로 오인되는가?** 실패 로그의 깨진 PNG import는 기존 다른 evidence 디렉터리에서 발생한다. 통과.
20. **선택 Rspress sidebar/search 범위를 숨기는가?** 다섯 문서에 한정된 범위를 README와 build 결과에 밝힌다. 통과.
21. **부분 검색 index가 전체 site 검색으로 오해되는가?** 선택 문서 전용 index만 사용하고 전체 문서 검색 완료를 주장하지 않는다. 통과.
22. **기존 preview의 경로를 지우거나 비우는가?** 동기화 목록 외에는 삭제하지 않는다. 기록상 삭제 0개다. 통과.
23. **기존 Tailnet 파일을 백업 없이 덮어쓰는가?** 교체 전 바이트와 SHA-256을 기록하고 백업을 보존한다. 통과.
24. **동일한 파일도 불필요하게 재기록되는가?** SHA-256이 동일한 경로는 copy 결과에서 `identical`로 구분한다. 통과.
25. **로컬 Serve 응답과 Tailnet 응답이 달라지는가?** 두 endpoint에서 HTTP 200과 payload hash를 비교한다. 통과.
26. **고정 asset 이름의 다른 내용이 기존 페이지를 깨뜨리는가?** 경로 충돌을 사전 대조하고 hash 이름이 달라진 bundle은 기존 asset을 보존한다. 통과.
27. **Tailnet listener가 재시작되거나 다른 포트를 사용하는가?** `127.0.0.1:8768` PID `1191`을 그대로 둔다. 통과.
28. **사용자 터미널 상태가 변경되는가?** tmux와 기존 terminal-browser를 조작하지 않는다. 통과.
29. **새 Simulator 또는 실기기 실행을 이 sync가 암시하는가?** 이 PR의 화면 검증은 Agent Browser 문서 화면이며 앱 재실행은 없다. 통과.
30. **PR 본문의 screenshot URL이 branch 삭제 뒤 끊기는가?** 병합 뒤 screenshot URL을 `main` 경로로 정정할 수 있도록 파일을 저장소 evidence에 포함한다. 통과.
