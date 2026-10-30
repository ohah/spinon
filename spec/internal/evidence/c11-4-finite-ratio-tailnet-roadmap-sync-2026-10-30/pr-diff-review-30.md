# PR diff 적대 검토 30개

이 검토는 Tailnet 동기화 evidence와 일회성 검증 도구 diff를 대상으로 한다. 제품 CSS admission이나 C11 완료 판정은 변경하지 않는다.

1. **목적지 범위** — 복사 대상이 `SpinonPreview` 하나로 고정되어 있고 다른 사용자 파일 루트를 가리키지 않는가? 통과.
2. **backup 범위** — 백업이 날짜 지정한 `SpinonPreviewBackups/.../pass01` 아래에만 만들어지는가? 통과.
3. **main revision** — 오래된 로컬 branch를 우연히 출판하지 않도록 `HEAD == origin/main == sourceHead`를 강제하는가? 통과.
4. **tree와 source 혼동** — manifest는 main tree와 별도 선택 root를 모두 기록하고 함께 확인하는가? 통과.
5. **RSPress drift** — 빌드 버전이 프로젝트가 고정한 2.0.22와 같은가? 통과.
6. **build side effect** — 선택 빌드가 저장소 main 파일을 생성/변경하지 않고 `build/spinon/.../pass02`에 출력되는가? 통과.
7. **첫 validator 오판** — `PR #842`의 연속 문자열 검사가 링크 anchor 경계에서 깨지는 경우를 `#842`와 merge SHA 검사로 대체했는가? 통과.
8. **신선한 재시도** — 첫 validator 실패 산출물을 재사용하지 않고 pass02 root·site·publication manifest를 따로 만들었는가? 통과.
9. **필수 파일** — 이미지가 없는 상태에서도 URL만 확인해 성공 처리하지 않고 두 simulator evidence screenshot의 바이트를 manifest에 포함하는가? 통과.
10. **HTML stale marker** — 이전 후보 계획 문구가 roadmap에 남아 있는지 선택 전에 검사하는가? 통과.
11. **status marker** — status에서 PR/merge SHA와 공식 집계가 모두 렌더된 HTML에 존재하는지 확인하는가? 통과.
12. **계획 marker** — 집중 계획에서 PR/merge SHA 및 C11.4 상태 표식을 확인하는가? 통과.
13. **path traversal** — 출력 manifest 경로를 `..`, 절대 경로, 역슬래시와 빈 경로 조각으로 탈출시킬 수 없는가? 통과.
14. **symlink traversal** — 목적지의 중간 경로 symlink를 따라 쓰지 않는가? 통과.
15. **비파일 대상** — 디렉터리 또는 특수 파일을 정규 파일처럼 교체하지 않는가? 통과.
16. **source TOCTOU** — preflight 뒤 source 내용이 바뀌면 복사 전에 크기/hash 검사로 거부하는가? 통과.
17. **target TOCTOU** — preflight 뒤 목적지 파일의 존재나 hash가 바뀌면 교체를 거부하는가? 통과.
18. **backup 덮어쓰기** — 기존 backup directory가 있으면 재사용/덮어쓰기하지 않고 중단하는가? 통과.
19. **backup order** — 기존 바이트를 검증한 backup에 기록한 뒤 destination을 바꾸는가? 통과.
20. **atomicity** — 임시 파일이 대상과 같은 filesystem 경로에 있고 rename 후 destination을 다시 검증하는가? 통과.
21. **부분 실패 복구 경계** — 여러 파일 복사 중 중단돼도 이미 교체한 파일의 backup이 남는가? 통과. 자동 rollback은 없고 backup과 manifest를 사용한 수동 복구가 필요하다는 점을 README에 명시했다.
22. **요청 중 파일 핸들** — preview server가 읽던 파일을 원자 rename으로 바꿔도 열린 descriptor는 기존 inode를 읽고 새 요청은 새 파일을 받는가? 통과.
23. **server identity** — PID가 8768 listener를 소유하고 실제 `tailnet-preview-server.mjs` command인지 확인하는가? 통과.
24. **listen scope** — 단일 loopback listener인지 확인하며 외부 listen을 허용하지 않는가? 통과.
25. **활성 연결과 listener 구분** — 일반 lsof 네트워크 핸들 개수가 잠깐 늘어도 listen socket 자체를 잘못 판정하지 않는가? 초기 guard는 연결 중단을 일으켰고 listen-only 확인으로 수정 후 통과.
26. **대상 파일 홀더** — server가 아닌 프로세스가 교체 대상 파일을 잡고 있으면 중단하는가? 통과.
27. **HTTP 상태와 redirect** — redirect를 자동으로 따라가서 다른 origin/page의 200을 오해하지 않는가? `redirect: manual`과 최종 URL 일치를 확인하고 HTTP 200을 요구하도록 보강했다.
28. **캐시 회피** — 원격 검사에 고유 query와 `cache-control: no-cache`를 보내고 실제 바이트 hash를 확인하는가? 통과.
29. **정확한 검증 수** — 실제 route 목록의 모든 419개를 로컬과 원격에서 각각 비교하고 18개 백업도 검증하는가? 통과.
30. **판정 범위와 자원 정리** — 화면/HTML 확인이 전체 WPT나 제품 완료를 뜻하지 않게 제한을 적고 Agent Browser/HeadlessChrome 프로세스를 닫았는가? 통과.

검토 중 자동 rollback 한계와 redirect 추적 경계를 확인해 README와 검증 도구를 보완했다. 수정 후 전체 원격 검증을 다시 수행했다.
