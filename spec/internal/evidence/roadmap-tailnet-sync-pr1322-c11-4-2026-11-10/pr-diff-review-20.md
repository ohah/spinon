# PR #1323 Tailnet roadmap sync · PR diff 적대 검토 20개

검토 대상은 C11.4 상태·계획·공개 roadmap 갱신, pass04 Tailnet 게시 스크립트와 SHA 보고서, Agent Browser 캡처·수명 기록이다. 구현 PR #1322의 상태는 GitHub에서 다시 조회했고, 게시 원본은 clean tracked tree인 `730066df57c72076b061644a78aa05b6b0047dd0`에 고정했다.

1. **병합 식별자 혼동:** GitHub PR #1322의 merge SHA, merge 시각 및 원격 PR 상태를 대조했다. `727229aea05e33537962fe9953ee206a37befecb`, `2026-11-10T00:37:12Z`, merged로 일치한다.
2. **공식 집계 부풀림:** STATUS와 public roadmap 숫자를 대조했다. 공식 완료는 계속 `78/214`다.
3. **상위 항목 조기 완료:** CSS 구현 목록과 계약 기록의 C11/C11.4 체크 상태를 확인했다. 둘 다 미완료로 남았다.
4. **계약 버전 유출:** 내부 Grid 계약과 구현 계획을 살폈다. API/FFI 변경을 주장하지 않고 숫자 버전 `0.1.0`을 유지한다.
5. **matrix 분모 오독:** 구현 근거, 상태, roadmap의 비교 분모를 대조했다. Candidate 1,440과 paired baseline 1,440 scene을 별도로 표기한다.
6. **content-floor 수치 과장:** 600 recovered occurrence를 원래 1,920 fit-row occurrence 설명과 비교했다. count 단위와 paired 조건이 유지된다.
7. **overflow 결과 일반화:** `visible overflow 0`을 전체 CSS Grid 주장으로 읽지 않도록 경계를 확인했다. 이 값은 고정 profile에만 연결된다.
8. **시뮬레이터 증거 오분류:** Android/iOS 문구를 구현 README와 대조했다. visual smoke, llvmpipe software renderer, readback·실기기·hardware GPU 한계를 명시한다.
9. **최신 카드 오래된 PR:** public roadmap 최상단과 최신 이력의 PR/SHA를 확인했다. #1322와 merge SHA를 가리키며, #1318은 과거 이력에만 남는다.
10. **미선정 계획 생성:** 칸반의 다음 항목을 확인했다. 후속 C11.4 slice를 정하지 않았다고 표시하고 가상의 계획을 연결하지 않는다.
11. **칸반 링크 미게시:** 최신 카드 세 local href를 생성된 publication manifest와 대조했다. 계획, 구현 근거, STATUS 경로가 모두 있다.
12. **RSPress 설정 드리프트:** 프로젝트 package와 실행 로그를 확인했다. RSPress `2.0.22`로 선택 문서 빌드가 성공했다.
13. **게시 입력이 dirty tree:** prepare script는 root를 만들기 전에 tracked status와 `git diff`가 깨끗한지 검사한다. report의 source commit/tree가 선택 source와 일치한다.
14. **경로 탈출·symlink:** source/route 경계, 부모 디렉터리와 파일 종류 검사, `..`·절대 경로·symlink 차단을 살폈다. 선택 source symlink는 0개다.
15. **조용한 route 덮어쓰기:** selected-root 및 publication builder의 충돌 처리를 확인했다. 같은 경로의 서로 다른 SHA는 거부하고 임시 staged file은 기존 파일을 덮지 않는다.
16. **사용자 landing route 삭제:** publish walker가 기존 `index.html`·`404.html`을 제외하고 sync가 source 목록 밖 route를 삭제하지 않는지 확인했다. 두 route 보존, `noDeletes=true`다.
17. **잘못된 서버에 게시:** preflight가 Tailscale `Running`, 8768의 단일 listener, `tailnet-preview-server.mjs` 명령과 `127.0.0.1` 주소를 확인한다. 기록된 PID는 90711이고 서버 재시작은 없다.
18. **읽는 중인 파일 교체·경쟁 수정:** preflight에서 교체 예정 파일의 `lsof`를 확인하고 copy 직전 target/source hash와 열린 파일을 다시 검사한다. 변경 대상 열린 handle은 0개다.
19. **롤백 불가능한 교체·잘못된 응답:** 기존 파일을 전용 pass04 폴더에 먼저 백업하고 hash를 확인한 다음 temp file을 rename한다. 로컬·HTTPS 각각 203/203 route hash와 48/48 백업 hash가 일치한다.
20. **화면·프로세스 수명 착오:** Agent Browser live page에서 desktop 1440×1000과 mobile 390×844를 확인했고 mobile document width는 390px다. PR #1323 본문 이미지 두 장도 각각 1440×1000과 390×844로 완전히 로드됐다. 두 세션을 닫았다. 오래된 외부 Terminal Browser 프로세스는 다른 저장소의 기존 프로세스라 종료하지 않았다.

판정: 이 20개 관점에서 PR diff 범위의 수정이 필요한 결함은 발견되지 않았다. 전체 문서의 모든 링크 dead-link 검사는 실행하지 않았다. 전체 검증 범위는 pass04 선택 경로, public roadmap 최신 카드의 세 local link, Agent Browser에서 확인한 roadmap 화면이다.
