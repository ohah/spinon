# PR #1192 병합 후 Tailnet roadmap evidence PR diff review 30개

- 기준 구현 PR: #1190 merge `950c858182810e27c39048c5bb252a4aed4810af`
- 정정 PR: #1192 merge `957f44ef7594098c146dcb814c95b7e91f4f18f6`
- 최종 source SHA-256: `4cf96a01cf0ecfe0c71a1fd453410541f752abc432135fa66b6d3d10f2fe4f54`
- 범위: Tailnet 제공본, Agent Browser desktop/mobile/counter capture, status/contract evidence links

## 실패 관점

1. **원본 branch 상태** — source는 PR #1192 merge 뒤 main HEAD에서 읽었다. **PASS**
2. **서버 source identity** — 기존 Tailnet server PID 28040이 예상 listener인지 검사했다. **PASS**
3. **server lifecycle 변경** — 기존 서버를 종료하거나 재시작하지 않았다. **PASS**
4. **target file identity** — `roadmap.html` 하나만 일반 파일인지 검사했다. **PASS**
5. **symlink 대체 경로** — preview root와 target이 symlink가 아님을 확인했다. **PASS**
6. **open target handle** — copy 전 target file handle PID가 0개였다. **PASS**
7. **backup 선행** — 이전 pass01 제공본을 pass02 backup에 먼저 보존했다. **PASS**
8. **backup hash** — backup hash는 이전 pass01 hash `be9d9361…`과 일치한다. **PASS**
9. **누락 파일 제한** — 교체 목록은 `roadmap.html` 한 개다. **PASS**
10. **삭제 부작용** — sync script가 파일을 삭제하지 않는다. **PASS**
11. **atomic publish** — 같은 directory 임시 파일 뒤 atomic rename을 사용했다. **PASS**
12. **source 바이트** — 581,932 bytes를 대상으로 삼았다. **PASS**
13. **source SHA** — merge source의 SHA-256은 `4cf96a01…`이다. **PASS**
14. **local HTTP status** — `/spinon/roadmap.html`은 HTTP 200이다. **PASS**
15. **remote HTTPS status** — Tailnet URL은 HTTP 200이다. **PASS**
16. **served bytes** — local과 remote 각 581,932 bytes가 source와 맞는다. **PASS**
17. **served hash equality** — source/local/remote SHA-256이 모두 일치한다. **PASS**
18. **counter correctness** — Agent Browser live counter와 status note는 모두 `78/214`를 표시한다. **PASS**
19. **stale static count** — 최신 #1187·#1190 card에 `72/213`이 남지 않았다. **PASS**
20. **historical evidence integrity** — pass01 mismatch screenshot/JSON은 수정하지 않고 보존했다. **PASS**
21. **desktop document overflow** — 1440px viewport에서 document/body width가 1440px이다. **PASS**
22. **desktop card geometry** — #1190 card x=90px, width=928px다. **PASS**
23. **mobile document overflow** — 390px viewport에서 document/body width가 390px이며 horizontal overflow가 없다. **PASS**
24. **mobile card edge** — #1190 card x=14px, width=362px, right=376px다. **PASS**
25. **card content visibility** — desktop/mobile screenshot에서 date, PR, count, links가 읽히도록 보인다. **PASS**
26. **link set loss** — browser DOM에서 #1190 card의 링크 9개를 확인했다. **PASS**
27. **image target drift** — Android/iOS source screenshots는 implementation merge SHA `950c8581…`에 고정돼 있다. **PASS**
28. **status/contract cross-link** — 공식 상태와 internal contract는 같은 Tailnet evidence README를 가리킨다. **PASS**
29. **process cleanup** — `agent-browser close` 뒤 `pgrep -alf agent-browser`는 비었다. **PASS**
30. **claims exceed capture** — browser evidence는 roadmap display/overflow와 provided hashes만 주장하며 C11.4 product conformance를 주장하지 않는다. **PASS**

## 판정

pass02는 live `78/214` count를 포함한 수정 source와 일치한다. C11/C11.4는 계속 미완료다.
