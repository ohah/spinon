# PR #1042 Tailnet 동기화 증거 · PR diff 실패 관점 검토 30개

검토 대상은 `README.md`, publication·preflight·copy·served-verification JSON, 선택 build/publication script, sync script와 Agent Browser capture/report다. 각 항목은 다른 실패 경로 또는 증거 경계를 확인한다.

1. 소스 revision이 PR #1044 병합 후 `main`을 가리키는가 — PASS, `734663a19b72b59ff422993a258359b4ab46f970`.
2. manifest의 source tree가 해당 revision과 연결되는가 — PASS.
3. tracked diff가 없는 시점의 source로 publication을 만들었는가 — PASS, empty tracked diff SHA-256.
4. 변경되지 않은 untracked raw 로그를 무단 publication하지 않았는가 — PASS, manifest에 `untrackedEvidencePublished=false`.
5. 공개 roadmap card가 PR #1042를 정확히 연결하는가 — PASS, 8개 local link route 전부 manifest에 존재.
6. official completion count를 잘못 증가시키지 않았는가 — PASS, `72/213`.
7. C11 또는 C11.1을 완료로 바꾸지 않았는가 — PASS, `c11Complete=false`; Agent Browser에서 C11 checkbox unchecked.
8. Tailscale backend를 copy 전에 확인하는가 — PASS, preflight에서 `Running`.
9. preview root가 예상 경로이며 symlink가 아닌지 확인하는가 — PASS, sync script에서 `lstat` 검사.
10. preview listener 개수가 하나인지 확인하는가 — PASS, PID `28040`.
11. listener가 예상 loopback 주소·port인지 확인하는가 — PASS, `127.0.0.1:8768`.
12. listener process가 Tailnet preview server인지 확인하는가 — PASS, `tailnet-preview-server.mjs`.
13. copy 전 기존 local roadmap 응답을 확인하는가 — PASS, HTTP 200.
14. 변경 route의 열린 file handle을 거부하는가 — PASS, 대상 0개.
15. 이전 파일 backup 디렉터리를 고유 경로로 보존하는가 — PASS, pass01에 11 route 백업.
16. 기존 route hash가 preflight와 copy 사이에 바뀌면 거부하는가 — PASS, copy 직전 hash를 재확인.
17. 신규·교체 파일을 temporary path에서 atomic rename하는가 — PASS, sync script.
18. 각 replace 전 backup hash를 확인하는가 — PASS, copy 후 검증 11/11.
19. 각 route의 source size/hash가 manifest와 같은가 — PASS, 103/103.
20. local preview copy 뒤 각 route hash를 비교하는가 — PASS, 103/103.
21. HTTPS 응답 상태를 모든 selected route에서 확인하는가 — PASS, HTTP 200 103/103.
22. HTTPS 응답 body hash를 source와 비교하는가 — PASS, SHA-256 103/103.
23. copy 범위가 새 `roadmap.html`과 실제 card link route를 포함하는가 — PASS.
24. `STATUS.html`이 새 main 상태를 표시하는가 — PASS, browser text와 count `72/213`.
25. 새 C11 plan route가 render되는가 — PASS, Agent Browser에서 제목·`grid-auto-rows` 확인.
26. 새 runtime README route가 render되는가 — PASS, 20/20·Android·iOS marker 확인.
27. responsive check가 document-level overflow와 card visibility를 구분하는가 — PASS, 390px document width는 390px이며 card visible; overview SVG 일부가 crop되는 점도 보고서에 기록.
28. Android·iOS PR screenshot을 실제 browser image load로 확인하는가 — PASS, 각각 1080×2400·1206×2622.
29. browser 종료를 확인하는가 — PASS, `close --all` 뒤 두 번째 process scan 0개.
30. 결과를 full-site validation 또는 app pixel equality로 확대 해석하지 않는가 — PASS, README가 선택 route·Agent Browser 웹 capture의 범위와 한계를 명시.

## 판정

103개 선택 route를 안전하게 교체·생성하고 local·HTTPS hash를 대조했다. Tailnet preview server는 재시작하지 않았고 파일을 삭제하지 않았다. 모바일 overview SVG crop은 관측된 시각 한계로 기록했으며 전체 문서 링크나 native app pixel parity를 주장하지 않는다.
