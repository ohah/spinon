# Tailnet roadmap pass66 증거 PR diff 검토 30개

이 PR은 병합된 PR #541의 anchor correction을 Tailnet에 적용한 단일 파일 pass66 근거다. 새 구현·공식 상태·계약은 변경하지 않는다.

1. **revision 정확성** — 기록한 source revision이 PR #541 merge인가? `986d596fdea4f9235671acc4f9bdd6c13dc3a06e`를 쓴다.
2. **source checkout** — local `HEAD`와 `origin/main`이 같은가? pass66 script가 둘 다 고정 SHA와 대조한다.
3. **tracked blob** — roadmap source가 병합 main의 tracked file인가? `HEAD:path`와 `origin/main:path` blob hash가 일치해야 한다.
4. **source size/hash** — manifest에 정확한 size와 SHA-256이 남는가? preflight에서 두 값을 기록한다.
5. **기존 target 검증** — pass65 뒤 수동 변경을 덮어쓰는가? 이전 target hash가 pass65 manifest와 다르면 중단한다.
6. **변경 범위** — 예상 밖 문서를 바꾸는가? 대상은 `roadmap.html` 한 파일이다.
7. **백업 분리** — 이전 pass65 backup을 재사용하는가? 새 `pass66/roadmap.html`로 보존한다.
8. **백업 선행** — 원본을 바꾸기 전에 복구본이 존재하는가? 교체 전에 hardlink backup을 만든다.
9. **백업 hash** — 보존 파일이 기존 Tailnet bytes와 같은가? 이전 SHA-256을 즉시 확인한다.
10. **source content** — 올바른 status anchor가 포함되는가? `#css-구현-체크리스트`가 없으면 실행을 거부한다.
11. **stale fragment** — PR #539 paragraph에 이전 `#c11-css-grid`가 남았는가? script가 그 paragraph에서 검사한다.
12. **원자 교체** — preview server가 쓰다 만 파일을 읽을 수 있는가? temp hardlink 후 rename으로 교체한다.
13. **동일 볼륨** — hardlink를 지원하지 않는 볼륨에서 부분 동작하는가? hardlink 실패 시 중단하고 파일 복사 fallback은 사용하지 않는다.
14. **삭제 위험** — sibling route나 output을 삭제하는가? 동기화는 삭제하지 않는다.
15. **로컬 target size** — 최종 bytes가 source와 같은가? verify에서 byte count를 비교한다.
16. **로컬 target hash** — 최종 content hash가 source와 같은가? verify에서 SHA-256을 비교한다.
17. **backup 보존** — 백업이 최종 source로 덮이는가? 이전 hash로 다시 검증한다.
18. **원격 URL** — preview mount prefix를 정확히 쓰는가? `/spinon/roadmap.html`을 사용한다.
19. **HTTP 상태** — 오류 페이지도 성공으로 인정되는가? status 200만 성공이다.
20. **원격 bytes** — HTTP body가 source file과 byte 단위로 일치하는가? 크기와 SHA-256을 함께 검사한다.
21. **cache 방지** — browser나 HTTP cache가 예전 page를 반환하는가? verifier fetch에 `no-store`를 쓴다.
22. **roadmap 표시** — 실제 HTML이 Agent Browser에서 열리는가? 제목과 #539 paragraph를 읽었다.
23. **merge reference** — roadmap 화면이 #539 SHA를 보여주는가? `0f46437c299df6f0c069b694a3f2184ac464ad3d`를 확인했다.
24. **미완료 경계** — C11.4 전체를 완료로 표시하는가? `미완료`를 확인했다.
25. **완료 수 경계** — 공식 count가 변했는가? `77/213`을 확인했다.
26. **href** — 수정된 공식 상태 link가 실제 route를 가리키는가? Agent Browser에서 rendered href를 읽었다.
27. **실제 클릭** — URL을 직접 여는 것만 확인했는가? roadmap의 해당 link를 클릭했다.
28. **fragment target** — 브라우저에 target이 있는가? decoded fragment `css-구현-체크리스트`의 element와 heading을 확인했다.
29. **증거 파일** — 최종 화면 및 process 종료가 보존되는가? roadmap/status screenshot과 close JSON을 포함한다.
30. **검증 범위** — 문서 link 검사를 runtime·simulator 테스트로 넓혀 말하는가? 앱이나 기기 검증은 하지 않았다고 명시한다.

## 결과

30개 관점을 확인했다. local, HTTPS, Agent Browser에서 한 파일 교체와 수정 fragment 이동이 일치한다. 앱·기기 테스트를 뜻하지 않는다.
