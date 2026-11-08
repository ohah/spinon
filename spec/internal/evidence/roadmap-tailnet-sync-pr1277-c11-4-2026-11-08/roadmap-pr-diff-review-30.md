# 로드맵·Tailnet 동기화 PR 변경 실패 관점 검토 30개

각 항목은 이 diff가 만들 수 있는 별도 회귀를 검사했다. 화면 검토 20개와 구분해 변경 경로·출처·게시 안전성에 집중한다.

1. **C11.4 완료 오승격:** STATUS 체크박스를 바꾸지 않았는가? — 통과. C11.4와 C11 미완료를 유지한다.
2. **공식 수 변조:** `78/214`를 수정하지 않았는가? — 통과. 현재 count 문자열만 설명한다.
3. **기존 merge SHA 손상:** PR #1277 SHA를 정확히 남겼는가? — 통과. `8aa7570d34ac7c0ac9f93f3aac4fe555428741fa`.
4. **CI 오표기:** 빈 PR rollup을 green이라고 쓰지 않았는가? — 통과. rollup 비어 있음을 기록한다.
5. **스크린샷 근거 과장:** OS screenshot을 surface readback으로 부르지 않았는가? — 통과. Android PixelCopy/Metal surface readback 아님을 유지했다.
6. **C11.4 범위 확장:** 미구현 intrinsic 조합을 지원한다고 주장하지 않는가? — 통과. 다음 비교 예정으로 한정한다.
7. **공식 ID 혼동:** 수동 진행 카드를 공식 상태 대장 ID처럼 보이게 하지 않는가? — 통과. 별도 지표라는 주석을 추가했다.
8. **현재 항목 불일치:** 헤더와 카드가 다른 C11 ID를 가리키는가? — 통과. 둘 다 C11.4 Grid track sizing.
9. **후속 후보 확정 과장:** 다음 intrinsic 조합이 검증 전 확정된 것처럼 보이는가? — 통과. 독립 Chrome 비교 후 별도 slice로 이어간다고 쓴다.
10. **이전 구현 카드 손실:** #1274 이하 merge history를 삭제하지 않았는가? — 통과. 새 #1277 상태만 추가했다.
11. **중복 count:** 상세 카드가 공식 완료 숫자에 포함되는가? — 통과. 78/214 그대로.
12. **문서 주장 일치:** STATUS와 내부 계약의 #1277 병합 설명이 어긋나는가? — 통과. 추적된 동기화 diff에서 상태를 갱신했다.
13. **broken source route:** roadmap의 계획·evidence 로컬 URL이 RSPress에 생성되는가? — 통과. 선택된 카드 로컬 링크 5개 검사.
14. **404 흔적 제거:** 사전 실패 기록을 숨기거나 덮어썼는가? — 통과. 이전 404 JSON을 그대로 보존했다.
15. **deep-link miss:** 계획·비교·실행·review·sync README가 모두 만들어지는가? — 통과. 각 generated route와 실제 browser title을 확인했다.
16. **이미지 손상:** Android/iOS 원본 링크가 잘못된 SHA를 가리키는가? — 통과. merge SHA 원본 URL을 다시 확인하고 각각 1080×2400, 1206×2622를 기록했다.
17. **정적 evidence 누락:** README의 PNG/JSON/log 링크에 필요한 파일을 staging했는가? — 통과. 선택 evidence tree 88개 파일의 source hash를 대조했다.
18. **symlink 탈출:** source tree symlink를 따르는가? — 통과. stage와 publication 모두 symlink를 거부한다.
19. **경로 탈출:** `..`, 절대 경로 또는 역슬래시 route가 허용되는가? — 통과. 모든 route segment를 검사한다.
20. **다른 문서 덮어쓰기:** 선택하지 않은 Tailnet route를 변경하는가? — 통과. 선택된 generated pages/static과 연결 evidence만 manifest에 포함.
21. **기존 파일 유실:** 교체 문서의 원본이 백업되는가? — 통과. 다섯 기존 route를 `pass02`에 보존했다.
22. **부분 파일 노출:** 새 파일을 복사 중 읽을 수 있는가? — 통과. 같은 디렉터리 임시 파일을 fsync 후 exclusive hard link로 게시했다.
23. **기존 reader 중단:** 열린 read-only stream을 강제로 닫는가? — 통과. 서버 프로세스를 유지하고 같은 디렉터리 원자 rename을 사용했다.
24. **임시 파일 충돌:** 고정 임시 파일명이 기존 임시 파일을 덮는가? — 통과. UUID와 `wx`로 생성한다.
25. **새 target race:** preflight 뒤 같은 URL이 생성되면 덮는가? — 통과. exclusive hard-link add는 기존 target에서 실패한다.
26. **오래된 source copy:** preflight와 게시 사이 source가 바뀌어도 진행하는가? — 통과. bytes와 SHA를 게시 직전 재확인한다.
27. **오래된 target 교체:** preflight 뒤 target이 바뀌어도 덮는가? — 통과. 기존 target SHA를 재확인하고 다르면 중단한다.
28. **서버 무중단:** Preview server/Tailscale을 재시작 또는 종료하는가? — 통과. PID 28040, Tailscale Running을 확인했고 재시작하지 않았다.
29. **불완전 응답:** Tailnet reverse proxy에서 새 문서나 CSS/JS가 404인가? — 통과. local/Tailnet 양쪽 38 GET 응답 모두 200이며 바이트 SHA가 manifest와 같다.
30. **브라우저 잔류:** Agent Browser가 작업 종료 후 남는가? — 최종 close 후 `agent-browser-lifecycle-final.json`에 프로세스 검증을 기록한다.

전체 docs dead-link 검사는 실행하지 않았다. 검증 범위는 PR #1277 카드의 로컬 링크, 연결된 선택 문서, 게시한 이미지와 필요한 공유 static asset이다.
