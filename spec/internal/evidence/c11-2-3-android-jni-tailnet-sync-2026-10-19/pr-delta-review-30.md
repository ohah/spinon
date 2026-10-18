# Tailnet sync PR 변경 적대 검토 30개

기준은 이 PR의 동기화 기록, 빌드 요약, Tailnet 검증 보고서, Agent Browser 캡처다. 각 검토는 현재 diff와 기록된 실행 결과를 대상으로 했다.

1. **병합 출처** — PR #328의 실제 merge commit `d50c6cc...`를 기준으로 기록했다. PASS.
2. **작업 날짜** — 로드맵 note와 sync 증거가 2026-10-19를 가리킨다. PASS.
3. **제품 코드 범위** — Tailnet 반영 기록만 추가하며 제품 동작을 바꿨다고 주장하지 않는다. PASS.
4. **완료 과장** — C11.2.3 제한 profile과 공개 CSS Grid 범위가 그대로라고 명시했다. PASS.
5. **로드맵 링크** — 브라우저에서 렌더된 note에 PR #328 링크를 확인했다. PASS.
6. **공식 상태 근거** — `STATUS.html`에서 PR 기록과 제한 profile 상태를 별도로 확인했다. PASS.
7. **선택 빌드 버전** — Rspress 2.0.22를 빌드 로그와 결과에 기록했다. PASS.
8. **선택 빌드 범위** — STATUS, 계획, 근거 문서, roadmap과 자산으로 범위를 한정했다. PASS.
9. **전체 빌드 구분** — 전체 저장소 빌드도 성공했다고 섞어 쓰지 않았다. PASS.
10. **dead-link 진단 의미** — 전체 빌드의 569건을 source-file 진단으로 기록했고 링크 개수라고 표현하지 않았다. PASS.
11. **진단 로그 개인정보** — 원시 bundler trace의 기기 절대 경로를 공개하지 않고 경로를 축약한 요약만 남겼다. PASS.
12. **선택 링크 검사 한계** — selected config가 dead-link/image 검사를 끈 사실과 전체 적합성이 아님을 밝혔다. PASS.
13. **빌드 산출물 수** — 404를 제외한 파일 34개와 2,078,379 bytes를 출력에서 재계산했다. PASS.
14. **미리보기 경로 범위** — `/spinon` 아래 경로만 동기화 대상으로 삼았다. PASS.
15. **사전 검사 합계** — 78 create, 4 replace, 11 same으로 93개가 합산된다. PASS.
16. **삭제 방지** — 사전 검사와 복사 보고서 모두 삭제 0개다. PASS.
17. **교체 백업** — 기존 대상 4개를 덮기 전에 별도 저장하고 hash를 검증했다. PASS.
18. **복사 확인** — 93개 대상이 원본 byte 수와 SHA-256에 맞는다고 기록됐다. PASS.
19. **외부 응답 확인** — Tailnet에서 93개 모두 HTTP 200이다. PASS.
20. **served hash 확인** — Tailnet 응답 93개 모두 원본 SHA-256과 일치한다. PASS.
21. **검증 실패 반환** — served verification helper의 실패 종료 코드를 확인하도록 고친 뒤 최종 검증을 재실행했다. PASS.
22. **로컬 주소 노출** — 공개 보고서의 미리보기 root와 Funnel hostname을 가렸다. PASS.
23. **화면 증거 출처** — roadmap 캡처는 Tailnet URL을 연 Agent Browser에서 생성했다. PASS.
24. **캡처 내용** — 캡처에 C11.2.3 note와 PR #328이 보인다. PASS.
25. **캡처 크기** — 저장 이미지가 1280×633임을 파일에서 확인했다. PASS.
26. **계획 경로** — 계획 페이지의 제목, fixture gate 내용, 실행 근거 링크를 브라우저에서 확인했다. PASS.
27. **근거 경로** — 실행 README와 build/log/ELF 파일 링크가 렌더됐다. PASS.
28. **기존 화면 자산** — 링크된 API 37 PNG를 열었고 1080×2400으로 표시됐다. PASS.
29. **범위 고지** — 네 route와 PNG를 확인했으며 전체 사이트 모든 링크를 검증했다고 말하지 않는다. PASS.
30. **세션 정리** — Agent Browser 종료 후 해당 세션 프로세스 0개를 확인했고, 미리보기 listener와 `tmux spinon`은 유지했다. PASS.

## 판정

30개 관점에서 차단 이슈 없음. 남은 범위는 전체 저장소 dead-link 오류 569건을 별도 정리하는 일이며 이 PR의 Tailnet 동기화와는 무관하다.
