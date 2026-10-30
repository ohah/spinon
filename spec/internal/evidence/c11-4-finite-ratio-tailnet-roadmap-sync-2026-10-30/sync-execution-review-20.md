# Tailnet 동기화 실행 검토 20개

각 항목은 이번 문서 출판·Tailnet 파일 반영에서 독립된 실패 경로 또는 증거 경계를 확인한다.

1. **소스 revision 고정** — `HEAD`와 `origin/main`이 출판 manifest의 `sourceHead`와 같아야 한다. 동일한 `9a7d04bf21d832672c4c204fb9cceafc29cdb654`를 확인했다.
2. **tree provenance** — 선택 루트와 출판 manifest의 tree hash가 같은지 확인한다. 둘 다 `b7a608a5cdc05c564e4194e23c9ac09391da4815`다.
3. **선택 루트의 범위** — 임의 저장소 파일 복사를 막고 356개/13,960,527 bytes의 선정 결과만 빌드했다.
4. **RSPress 버전** — 빌드가 고정된 `2.0.22`를 사용하며 lockfile 의존성을 바꾸지 않는다.
5. **필수 경로 집합** — roadmap, status, 계획, 내부 계약, 리뷰와 두 플랫폼 overflow screenshot을 포함하는 13개 route가 manifest에 있다.
6. **PR 표식** — HTML 링크 경계에서 `PR #842` 텍스트가 분리될 수 있어 실제 HTML marker `#842`를 검증하도록 했다.
7. **오래된 주장 제거** — roadmap에 후보 실행 전 상태 문구가 남으면 선택을 거부한다.
8. **상태 수 보존** — roadmap과 status HTML에서 `72/213` 및 미완료 문구를 확인한다.
9. **merge SHA 연결** — `78fd0b84e4132e1f91bd293735e97b33cbf0d9a0`이 roadmap, status와 계획에 있다.
10. **경로 순회 차단** — 출판 경로의 절대 경로, 역슬래시, 빈 조각, `.`과 `..`을 거부한다.
11. **symlink 차단** — 목적지 루트와 각 기존 목적지 구성 요소의 symlink를 거부한다.
12. **출판 내용 고정** — 사전 점검과 복사 단계마다 manifest 크기 및 SHA-256을 재확인한다.
13. **사전 점검 분류** — 실제 실행 전 create 346, replace 18, same 55를 분류해 보고했다.
14. **기존 내용 보존** — 18개 교체 파일 각각을 덮어쓰기 전에 5,826,707 bytes의 별도 backup으로 검증했다.
15. **열린 목적지 파일** — 교체 대상에 대해 예상 preview server 외 프로세스가 파일을 잡고 있으면 중단한다.
16. **임시 파일 원자 교체** — 임시 파일을 목적지와 같은 디렉터리에 만들고 rename 뒤 바이트/hash를 재확인한다.
17. **실행 간 경합 감지** — 복사 직전 목적지의 존재 여부와 이전 hash가 사전 점검 기록과 다르면 중단한다.
18. **삭제 금지** — 선택 목록 밖 경로를 탐색하거나 삭제하지 않는다. preflight/copy/verify 모두 `no_deletes: true`다.
19. **로컬 사후 점검** — 선택된 전체 419 route의 크기/hash와 18개 backup의 크기/hash를 확인했다.
20. **원격 사후 점검** — 각 Tailnet HTTPS 응답이 HTTP 200이고 기대 크기/SHA-256과 일치하는지 전체 419 route에서 확인했다.

판정: 현재 실행은 20개 조건을 통과했다. 첫 복사 호출은 preview server의 순간적인 활성 TCP 연결을 listener 수로 오인해 변경 전에 중단했다. listener 확인을 예상된 단일 `127.0.0.1:8768` listen socket에 한정한 뒤 복사를 수행했으며, 파일 홀더 검사는 그대로 적용했다.
