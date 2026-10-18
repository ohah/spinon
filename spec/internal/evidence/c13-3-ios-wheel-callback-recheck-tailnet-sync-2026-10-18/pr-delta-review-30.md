# Tailnet 로드맵 동기화 PR 적대적 검토 30항목

기준: PR #300 병합 뒤 수행한 문서 미리보기 동기화 기록. 각 항목은 별개의 잘못된 상태·과장·파괴 경로를 검사했다.

1. **기준 커밋 일치** — PASS. 사전 inventory의 base commit은 PR #300 merge commit `5b80933639a375a4db138d80ee95416f4720af42`이다.
2. **완료 상태 비변경** — PASS. 동기화 전후 공식 완료 수가 모두 74/207이며 새 완료 ID를 만들지 않았다.
3. **로드맵 대상 수 산술** — PASS. 상위 78 + ID가 있는 하위 127 + ID 없는 하위 2 = 207이다.
4. **계획의 미완료 경계** — PASS. C13.3는 계획, 상태 대장, 미리보기에서 미완료로 남았다.
5. **선택 빌드 입력 수** — PASS. Rspress 입력 Markdown은 8개이고 기록된 HTML route 수도 8개다.
6. **정적 로드맵 산출물** — PASS. Markdown route와 별개인 `roadmap.html`도 build 산출물 manifest에 포함됐다.
7. **문서 도구 버전 고정** — PASS. 선택 렌더는 프로젝트 규칙의 Rspress 2.0.22를 사용했다.
8. **검사 비활성화 공개** — PASS. 선택 빌드에서 link/image check를 끈 사실이 요약 JSON과 README에 함께 기록됐다.
9. **전체 빌드 실패 보존** — PASS. 전체 `docs:build` 종료 코드 1을 숨기거나 성공으로 바꾸지 않았다.
10. **실패 규모 기록** — PASS. 저장소 전체 dead-link 오류 페이지 540개를 경계 요약에 남겼다.
11. **수정 페이지 귀속** — PASS. 바뀐 C13.3 evidence README의 dead-link 그룹이 0임을 전체 실패와 구별해 적었다.
12. **부분 빌드의 해석** — PASS. 선택 렌더 성공을 전체 문서·링크·이미지 적합성으로 확대하지 않았다.
13. **복사 대상 합계** — PASS. copy inventory의 총 대상 파일 수는 75개다.
14. **복사 작업 산술** — PASS. 32 추가 + 12 백업 후 교체 + 31 동일 파일 건너뜀 = 75다.
15. **삭제 경로** — PASS. 계획·실행 모두 삭제 수가 0이며 동기화는 copy-only다.
16. **교체 전 복구 지점** — PASS. 기존 파일을 덮기 전 별도 backup root에 보관했다.
17. **백업 무결성** — PASS. 교체 대상 12개의 백업 내용이 사전 SHA-256과 일치한다.
18. **파일별 대상 검산** — PASS. 추가·교체·skip 각 항목은 예상 크기와 SHA-256을 inventory에 보유한다.
19. **복사 후 무결성** — PASS. 75개 전부 preview 대상의 SHA-256이 기대값과 일치한다.
20. **HTTP 검증 범위** — PASS. 로컬과 Tailnet 각각을 대상으로 총 150개 응답을 수집했다.
21. **HTTP 상태 코드** — PASS. 150개 응답 모두 HTTP 200이다.
22. **서빙된 내용 검산** — PASS. 각 응답 body의 SHA-256이 대상 artifact hash와 일치한다.
23. **Listener 생명주기** — PASS. 기존 8768 listener를 확인했고 재시작하지 않았다.
24. **대상 경로 범위** — PASS. 로컬 및 Tailnet의 같은 상대경로를 직접 GET해 정적 preview와 외부 접근 경로를 따로 확인했다.
25. **로드맵 counter 렌더** — PASS. Agent Browser 화면에서 207개 표시 및 공식 완료 74/207을 확인했다.
26. **로드맵 note 결과 보존** — PASS. 화면의 C13.3 note에 inner-target 세 실행 결과(4/4, 3/4, 2/2)와 재현성 미확립 경계가 있다.
27. **상태 화면 경계** — PASS. 상태 대장 화면에서 후속 근거를 볼 수 있고 C13 상위 체크는 해제 상태다.
28. **스크린샷 의미와 로드** — PASS. Tailnet 문서 화면 screenshot이 2752×2064로 로드됐다. 이를 앱 입력·callback 실행 근거로 표현하지 않았다.
29. **개인정보 범위** — PASS. 기록과 화면 캡처에서 Tailnet hostname, 계정 경로, PID를 배제하거나 redacted 처리했다.
30. **브라우저 종료 및 검토 잔여** — PASS. 최종 PR 화면과 이미지 확인 뒤 Agent Browser를 닫았고, 종료 명령·세션 비활성·프로세스 부재를 별도 기록했다.

## 남은 제한

- iOS Simulator 실행에서 Run 02는 3/4 callback/process에 그쳤고 누락 원인은 미확인이다. 반복 가능한 ±X/±Y 결과는 확립되지 않았다.
- discrete mouse, Control-wheel, physical trackpad, DOM WheelEvent, S04.10 및 실기기는 이 PR에서 검증하지 않았다.
- 전체 문서 빌드는 저장소 내 540개 dead-link error page로 실패했다. 선택 8-route 빌드는 부분 렌더다.
- 이 문서는 문서 미리보기 동기화 검토이며 C13.3 구현 완료 선언이 아니다.
