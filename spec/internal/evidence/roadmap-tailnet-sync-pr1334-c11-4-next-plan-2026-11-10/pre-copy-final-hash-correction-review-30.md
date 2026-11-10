# PR diff 적대 검토 30개 관점

**대상:** #1336 Tailnet 동기화 근거에서 copy 직전 target 값과 copy 뒤 publication 값을 구분해 기록하는 문서 수정.

1. `preflight.json`을 동기화 후 결과로 오해할 수 있는가 — PASS: README에서 해당 route 값이 copy 전 target임을 명시한다.
2. 이전 target byte 수가 manifest와 맞는가 — PASS: 662,675 bytes는 preflight response의 길이와 같다.
3. 이전 target hash가 manifest와 맞는가 — PASS: `1c89b4d26c71b9ec3d3609031e91f2ca985ed03118bd0eded3b550dc97c682e0`는 preflight의 `localRoadmapSha256`이다.
4. 최종 source byte 수가 선택 publication과 맞는가 — PASS: 663,150 bytes를 선택 route의 값으로 표기한다.
5. 최종 source hash가 선택 publication과 맞는가 — PASS: `fa7c84c0faf71225598d999da9381bcfc870c6f5c983965a4645ee45010a01f5`를 사용한다.
6. 최종 served byte 수가 사후 검증과 맞는가 — PASS: `served-verification.json`은 663,150 bytes를 기록한다.
7. 최종 served hash가 사후 검증과 맞는가 — PASS: 사후 HTTP route hash가 최종 source hash와 같다.
8. copy report의 previous hash를 final hash로 혼동하는가 — PASS: README는 previous 값은 preflight와, final 값은 publication/served 결과와 연결한다.
9. 475-byte 차이를 우발적인 전송 손상으로 부르는가 — PASS: 663,150−662,675 byte 차이를 이전 target에서 새 source로 교체한 결과로 설명한다.
10. 서버가 동기화 뒤 임의로 내용을 바꿨다고 주장하는가 — PASS: 근거는 이전 target과 선택된 source의 차이로 한정한다.
11. 263개 route count가 바뀌는가 — PASS: route 수는 사전·사후 manifest에서 263으로 유지된다.
12. create/replace/same 집계가 변하는가 — PASS: 70/29/164 집계를 유지한다.
13. 29개 backup 수가 변하는가 — PASS: 사전·사후 backup hash 검증 29/29를 유지한다.
14. copy 전 열린 handle이 있었다고 오인시키는가 — PASS: 교체 대상 open handle 수 0을 그대로 기록한다.
15. 원본을 backup하기 전 덮어썼다고 오인시키는가 — PASS: copy report previous hash와 보관 backup hash의 역할을 구분한다.
16. 파일 삭제를 수행했다고 보이게 하는가 — PASS: 삭제 0건 기록을 유지한다.
17. 잘못된 route의 hash를 roadmap 값으로 사용했는가 — PASS: 값은 `roadmap.html` 항목의 byte/hash에만 연결한다.
18. JSON manifest를 임의로 고치는가 — PASS: preflight, publication, copy, served 원본 JSON은 바꾸지 않고 설명만 고친다.
19. 검증 시각 이후에 Tailnet이 바뀌었다고 암시하는가 — PASS: 사전 target과 사후 source/served 단계를 구분한다.
20. 현재 공개 route가 저장소 원본과 다른가 — PASS: 최종 HTTPS response는 `spec/public/roadmap.html`과 byte/hash가 같다.
21. preview server를 다시 시작할 필요가 있는가 — PASS: 현재 route hash가 이미 최종 source와 일치하므로 서버 조작을 하지 않는다.
22. 추가 sync로 backup을 덮어쓸 위험이 있는가 — PASS: 이 수정은 로컬 README 한 줄 설명만 바꾸며 sync script를 실행하지 않는다.
23. Tailnet preview root 외의 경로를 삭제하는가 — PASS: 파일 시스템 변경 대상은 저장소의 해당 README다.
24. 완료 수나 task 상태에 영향을 주는가 — PASS: 공식 `78/214`, C11.4/C11 미완료 상태를 바꾸지 않는다.
25. 제품 구현 또는 platform runtime 완료를 새로 주장하는가 — PASS: 문서는 배포 측정값만 다룬다.
26. 전체 문서 링크 검사를 완료했다고 암시하는가 — PASS: `fullDocsLinkValidation: false` 한계를 유지한다.
27. PR #1336의 이미지를 제거하거나 바꾸는가 — PASS: 캡처·본문 URL은 변경하지 않는다.
28. 모바일·데스크톱 viewport 근거를 섞는가 — PASS: 화면 증거는 이 수정 범위 밖이며 기존 크기 그대로 보존된다.
29. CI 또는 제품 테스트 결과를 만들어 내는가 — PASS: 새 CI·앱·runtime 실행을 주장하지 않는다.
30. 정정이 전후 근거를 역추적할 수 없게 하는가 — PASS: preflight, copy, selected publication, served verification 네 manifest의 연결을 README에 명시한다.

## 검토 후 반영

- README가 `preflight.json`의 copy 전 target을 final route 결과로 잘못 설명한 것을 발견했다.
- 사전 값 662,675 bytes/hash와 사후 값 663,150 bytes/hash를 역할별로 명확히 구분했다. 네 JSON manifest와 현재 Tailnet HTTP response는 일치하며 재동기화는 하지 않았다.
