# C11.4 Tailnet 동기화 PR 변경 실패 관점 검토 30개

대상: 이 PR의 preview evidence 문서·manifest·Agent Browser 캡처 및 PR 설명.

1. PR 제목과 브랜치 이름이 문서 배포 범위를 정확히 말하는가 — Tailnet preview sync만 다루도록 `docs:` 범위를 쓴다.
2. PR body의 source revision이 main에 실제 병합된 커밋인가 — 0d35a10...를 README·manifest에 고정했다.
3. PR body screenshot의 raw branch URL이 유효한가 — branch push 뒤 HTTP 응답을 받고 다운로드 이미지와 commit PNG의 SHA-256이 같음을 확인했다.
4. screenshot을 앱 실행 결과로 오해할 수 있는가 — roadmap/문서 페이지 캡처라고 명시하고 simulator 근거와 분리한다.
5. screenshot이 로드맵 변경을 보여주는가 — C11.4 행과 72/212 카운터가 함께 보이는 Agent Browser 화면을 선택했다.
6. PNG가 빈 페이지나 이전 scroll 위치를 담는가 — 이미지 자체를 열어 텍스트와 체크 상태를 육안 확인했다.
7. 상태 대장에서 C11.4가 완료로 승격되는가 — count를 바꾸지 않았고 완료 체크도 건드리지 않았다.
8. 계약 숫자 버전을 문서 작업만으로 올리는가 — 0.1.0을 유지했다.
9. 새 검토 페이지의 주장과 실제 증거가 다른가 — review page 화면과 source markdown을 모두 보존했다.
10. full site build 실패를 감추거나 성공으로 보고하는가 — exit 1 log를 포함하고 사전 dead link 문제로 범위를 구분한다.
11. 선택 빌드에서 꺼 둔 링크 검사 사실이 숨겨지는가 — config 설정과 제한을 README에 명시했다.
12. 변경된 문서에 신규 dead link가 추가되는가 — targeted audit에서 1,045 relative link 중 새 누락 0을 확인했다.
13. 기존 누락 링크를 이번 PR이 새로 만들었다고 오해하는가 — 남은 일곱 경로가 base commit에도 있었음을 JSON으로 연결했다.
14. PR 본문에서 evidence README와 핵심 원자료를 찾을 수 있는가 — README가 render, hash, browser, cleanup 기록을 연결한다.
15. 문서 페이지 경로와 static asset 경로가 혼동되는가 — Markdown route, raw Markdown, public asset을 별도 항목으로 manifest에 남겼다.
16. 정적 asset hash가 빌드 산출물과 다른가 — 80 served bodies의 길이와 SHA-256이 모두 source와 같다.
17. HTTP 200만 보고 stale cache를 지나치는가 — hash 검증 요청에 source hash 일부를 query로 넣고 response body를 대조했다.
18. 리다이렉트가 다른 content를 숨기는가 — 최종 요청에서 HTTP 200 및 정확한 본문 hash가 나왔다.
19. copy가 미리보기 밖 파일을 덮어쓰거나 삭제하는가 — root-relative target만 갱신했고 delete count 0이다.
20. open 404.html descriptor를 무시하는가 — 별도 lsof 확인 뒤 해당 파일은 제외했다.
21. backup이 target directory 안에 있어 같이 노출되는가 — 18개 백업은 served root 밖에 두었다.
22. 복사 직전에 열린 reader를 무시하는가 — 각 replace 직전에 lsof를 재확인하는 copy procedure를 사용했다.
23. 백업 hash가 존재만 하고 사전 hash와 다를 수 있는가 — source/backup SHA-256 18개를 각각 비교했다.
24. copy verification이 add·replace·same을 구분하지 않는가 — 18 replace, 22 add, 40 same으로 보고한다.
25. 삭제하지 않은 stale hashed asset을 실수로 제거하는가 — static old assets 정리는 하지 않았다.
26. agent-browser screenshot이 sync 이전 캐시를 찍는가 — 먼저 HTTPS hash 확인 뒤 새로고침/경로 이동해 캡처했다.
27. Tailnet preview 증거를 Terminal Browser로 얻었는가 — Agent Browser 0.31.1 경로만 사용했다고 기록했다.
28. 브라우저 창을 닫고 daemon/Chrome을 남기는가 — close 후 session list와 OS process 목록이 비어 있는지 확인했다.
29. 이 PR이 Android/iOS simulator 확인을 했다고 과장하는가 — 문서 preview scope라 앱·simulator를 수행하지 않았다고 쓴다.
30. PR이 아직 checks 대기 중인데 병합 준비라고 기록하는가 — 생성 뒤 live PR checks와 merge state를 확인하고 규칙에 맞게 병합한다.
