# Tailnet 로드맵 동기화 PR 변경 검토 30개

범위: Tailnet 미리보기 동기화 증거와 Agent Browser 화면. 코드 동작, C11 기능 완료 또는 전체 문서 빌드는 검토 대상으로 주장하지 않는다. 각 항목은 서로 다른 실패 관점을 확인한다.

1. **원본 PR 식별** — 게시 입력이 병합된 PR #1094 merge SHA `0326850ceb20ee19cea81ae5e0df1080cc018063`와 일치한다. 통과.
2. **Git tree 식별** — manifest의 source tree가 `36844d6e5afdad46d1ce4f744335071836c7dc7e`로 고정되어 있다. 통과.
3. **dirty tracked 변경 차단** — 원본 tracked diff SHA가 빈 diff hash이며 선택 중 작업 중 tracked 변경이 섞이지 않았다. 통과.
4. **입력 출처 제한** — 선택 root는 Git index의 tracked 파일에서만 만들어졌다. 통과.
5. **symlink 격리** — 선택 입력에 symlink가 없다. 통과.
6. **선택 파일 수 기록** — 111개 source file 수가 manifest에 기록되어 있다. 통과.
7. **선택 입력 바이트 기록** — source 파일 5,587,717 bytes를 manifest와 요약 문서에 기록했다. 통과.
8. **작업 폴더 오염 차단** — 이전 작업의 미추적 로그·캡처가 선택 입력에 포함되지 않았다. 통과.
9. **route 개수 일치** — publication과 served verification 모두 198개 route를 가리킨다. 통과.
10. **publication 크기 일치** — 게시 입력 15,835,207 bytes가 manifest와 사전 검사 결과에서 일치한다. 통과.
11. **홈 경로 선택 확인** — 새 로드맵의 `roadmap.html` route가 publication에 포함되어 있다. 통과.
12. **공식 상태 경로 확인** — `STATUS.html` route가 publication에 포함되어 있다. 통과.
13. **관련 계획·계약 경로 확인** — C11 fit-content plan과 내부 계약 0059가 게시 대상이다. 통과.
14. **runtime evidence 경로 확인** — PR #1093 실행 근거와 리뷰 페이지가 게시 대상이다. 통과.
15. **초기 원격 일치 검사** — 동기화 전 로컬·HTTPS roadmap 응답은 모두 200이고 SHA-256이 동일했다. 통과.
16. **Tailnet backend 상태** — preflight 시 Tailscale backend가 실행 중이었다. 통과.
17. **미리보기 서버 생명주기** — 서버 PID 28040을 중단·재시작하지 않고 유지했다. 통과.
18. **별도 static preview 보존** — PID 33319를 중단·재시작하지 않고 유지했다. 통과.
19. **대상 파일 잠금 확인** — 변경 route의 open handle 수가 0이었다. 통과.
20. **신규 경로 개수** — 새로 복사한 169개 route가 preflight와 copy 결과에서 일치한다. 통과.
21. **교체 경로 개수** — 교체한 11개 route가 preflight와 copy 결과에서 일치한다. 통과.
22. **무변경 경로 개수** — 동일했던 18개 route가 preflight와 copy 결과에서 일치한다. 통과.
23. **삭제 방지** — preflight와 copy 기록에서 대상 route 삭제가 없다고 확인했다. 통과.
24. **교체 전 백업** — 지정된 backup 경로가 존재하고 교체 전 파일 8개가 보존되어 있다. 통과.
25. **복사 결과 총계** — 169 + 11 + 18이 198개 route와 맞는다. 통과.
26. **HTTPS 전체 확인 범위** — served verification이 198개 route를 HTTPS로 확인했다. 명령은 오류 없이 완료됐다. 통과.
27. **핵심 URL 응답** — roadmap, STATUS, C11 plan/contract, runtime evidence 등 핵심 7개 HTTPS route가 모두 200이다. 통과.
28. **원격 바이트 동일성** — 핵심 7개 route 각각의 원격 바이트 수와 SHA-256이 로컬 결과와 일치한다. 통과.
29. **공식 진행 상태 보존** — 로드맵은 `72/213`을 보여 주고 C11 미완료 상태를 유지한다. Agent Browser DOM에서 확인했다. 통과.
30. **화면·브라우저 정리** — 1440×1000 및 390×844에서 문서 폭이 viewport와 일치한다. 두 화면을 저장하고 Agent Browser를 종료했으며 preview PID 28040/33319는 남아 있다. 통과.

## 판정

확인한 Tailnet 게시 입력과 live roadmap은 PR #1094 merge SHA 기준으로 일치한다. 이 검토는 문서 미리보기 및 evidence 동기화만 다루며, C11 완료나 전체 문서 빌드를 판정하지 않는다.
