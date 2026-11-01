# PR #987 Tailnet roadmap 문서 변경 검토: 30개 관점

이 검토는 PR #987 merge state 문서와 Tailnet publication·브라우저 증거를 확인한다. 구현 PR #986의 코드 검토를 대신하지 않는다.

1. **Source HEAD:** publication은 PR #987 merge 이후 `0f8d7d708d10d25e92391cb50adc0cd9204d5945`로 고정했다.
2. **Prior publication:** source root는 직전 확인된 #982 publication manifest와 hash를 비교해 사용한다.
3. **Overlay scope:** 상태, roadmap, C11 plan/index, contract 0059와 해당 slice 증거만 선택 root에 overlay한다.
4. **Symlink handling:** 기존 root·site·새 source의 symlink를 거부해 임의 외부 파일을 route에 넣지 않는다.
5. **Path traversal:** publication route에서 absolute path, `.`/`..`, 역슬래시를 거부한다.
6. **Source hashes:** overlay와 selected root manifest는 모든 파일의 byte 수와 SHA-256을 기록한다.
7. **Rendered links:** PR #986 roadmap card의 내부 링크가 모두 publication route에 존재하는지 검사한다.
8. **Merge claim:** roadmap HTML에 PR #986의 Rebase and merge, merge SHA, UTC 시각을 확인한다.
9. **Status claim:** 로드맵과 상태 대장의 공식 `72/213`이 바뀌지 않았는지 확인한다.
10. **Parent state:** C11과 C11.4 intrinsic track sizing checkbox는 모두 unchecked다.
11. **Completed children:** C11.2.2와 C11.2.3만 checked로 읽혀 status 원본과 맞는다.
12. **No delete:** 이전 931 route 전체를 복사한 뒤 추가·교체만 수행하고 삭제하지 않는다.
13. **No clobber:** publication, reports, backup root가 이미 있으면 덮어쓰지 않는다.
14. **Server identity:** Tailscale backend `Running`, port 8768 단일 listener와 예상 preview server command를 확인한다.
15. **Server lifecycle:** sync 중 preview server PID 28040을 재시작하거나 종료하지 않는다.
16. **Open handles:** 변경 대상 파일의 open handle 수가 0인 경우에만 교체한다.
17. **Backup integrity:** 바뀌는 기존 route는 교체 전에 backup하고 backup hash를 검증한다.
18. **Atomic copy:** 임시 파일을 같은 디렉터리에 쓴 뒤 rename해 route 단위로 교체한다.
19. **Preflight race:** copy 직전 target 존재 여부와 hash가 preflight 결과와 동일한지 다시 확인한다.
20. **After-copy local files:** 1,133개 route 모두 publication hash와 local preview root hash가 같은지 확인한다.
21. **Backup replay:** 기존 파일 61개 각각의 backup hash가 preflight의 원 hash와 맞는지 확인한다.
22. **Local HTTP:** 필수 17 route가 preview server에서 HTTP 200 및 정확한 hash로 제공된다.
23. **Tailnet HTTPS:** 동일 17 route가 사용자 지정 Tailnet hostname에서 HTTP 200 및 정확한 hash로 제공된다.
24. **Desktop browser:** Agent Browser 1280×720에서 merge card와 `72/213`을 읽고 문서 너비 1280px를 확인한다.
25. **Mobile browser:** Agent Browser 390×844에서 merge card와 `72/213`을 읽고 document/body 너비가 390px임을 확인한다.
26. **Mobile overflow:** 페이지 전체 가로 넘침이 없어 별도 수평 페이지 스크롤이 발생하지 않는다.
27. **Screenshot provenance:** desktop/mobile 캡처는 Tailnet URL을 연 Agent Browser에서 생성한 파일이다.
28. **Browser cleanup:** `close --all` 후 session은 inactive이고 Agent Browser executable process는 남지 않았다.
29. **Failed attempt accounting:** 첫 staging 시도의 anchor text marker mismatch와 preview copy 이전 실패를 보존하고 pass02 결과와 분리한다.
30. **Build claim:** 선택 RSPress build가 통과했어도 dead-link/dead-image 검사를 끈 범위만 주장하며 전체 문서 검증으로 확대하지 않는다.

**판정:** PR #986 및 PR #987이 반영된 선택 publication이 Tailnet에 도달했고 1,133 route·백업·17 critical HTTPS route·desktop/mobile roadmap 상태가 확인됐다. 삭제나 server restart는 없었다. 전체 문서 dead-link 검증은 이번 범위에 포함되지 않는다.
