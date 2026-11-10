# PR 변경분 실패 관점 검토 30개

대상은 공개 roadmap #1329 카드, 선택 publication 경로, Tailnet 동기화 근거와 Agent Browser 화면이다. 각 검토는 다른 실패 경계를 확인했다.

1. **계획 PR 병합 오인** — GitHub #1329가 `MERGED`이며 merge SHA `e3a1b09e…f888f1c94`인지 대조했다. 계획을 구현 merge로 표기하지 않는다.
2. **후보/구현 혼동** — 카드에서 4,320 scene을 Chrome 사전 비교라고 명시하고 Spinon 구현 결과가 아니라고 적는다.
3. **공식 완료 수 오기** — roadmap 문구, STATUS source 및 브라우저 화면에서 `78/214`를 확인했다.
4. **상위 완료 체크 오기** — `C11.4`와 `C11`이 미완료 상태로 남는지 확인했다.
5. **계약 버전의 우발 상승** — 내부 숫자 계약 `0.1.0`을 유지한다고 표시한다.
6. **PR 상태 허위 표시** — #1329 check rollup 0개를 기록하고 CI green으로 부르지 않는다.
7. **다음 plan route 오타** — RSPress publication에서 `plan/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum.html`가 생성됐다.
8. **비교 근거 route 누락** — precomparison README가 선택 route manifest에 있고 hash가 고정됐는지 확인했다.
9. **raw image mutable branch link** — Chrome reference image URL이 #1329 merge commit SHA를 경로에 쓴다.
10. **raw image 404 또는 깨진 응답** — Agent Browser에서 image가 complete 상태로 `1280×900` intrinsic size를 보였다.
11. **desktop wrong viewport** — screenshot과 Agent Browser 확인값이 `1440×1000`인지 확인했다.
12. **desktop document overflow** — desktop `scrollWidth`가 `1440px` viewport와 같은지 확인했다.
13. **mobile wrong viewport** — screenshot과 확인값이 `390×844`인지 확인했다.
14. **mobile document-level overflow** — `document.scrollWidth`가 viewport와 같은 `390px`인지 확인했다.
15. **칸반이 잘려 사용 불가** — mobile `.kanban-frame`의 `overflow-x:auto`, `1120px` content, `362px` viewport 및 `scrollLeft 0→300` 변화를 확인했다.
16. **새 plan이 화면에 없음** — desktop/mobile card screenshot에 #1329 계획 문단, merge SHA, `78/214`가 보인다.
17. **잘못된 브라우저 증거 종류** — 페이지와 이미지 확인은 Agent Browser 결과로 남기며 simulator surface 증거로 부르지 않는다.
18. **요청한 브라우저 종류 위반** — 이번 공개 페이지 확인은 Agent Browser CLI로 수행했고 Terminal Browser를 실행하지 않았다.
19. **사용 후 브라우저 잔류** — 전용 session을 닫고 session info가 inactive/null PID, task Chrome profile과 config가 제거됐음을 확인했다.
20. **다른 세션 파괴** — 종료 후 session 목록은 기존 `zntc4603-root`만 남아 있다. 그 세션의 browser process는 유지했다.
21. **계획 전용 simulator 불필요 실행** — 이번 문서 pass에서는 iOS/Android simulator나 AVD를 시작하지 않았고 새 profile도 만들지 않았다.
22. **S03 profile 임의 삭제** — S03 이름이 CoreSimulator inventory에 없음을 확인했다. 공유 profile과 HWPJS 소유 profile은 건드리지 않았다.
23. **선택 root에 dirty tracked source 혼입** — root manifest가 source HEAD `0ff0f9d2…`와 빈 tracked diff hash를 기록한다.
24. **source symlink 경유** — root generator가 source 경로의 symlink를 거부하며 manifest에 symlink 0개다.
25. **생성 root의 경로 충돌·덮어쓰기** — pass01 root와 manifest의 기존 존재를 거부하고 write-once 생성했다.
26. **잘못된 문서 빌드 버전** — 성공 로그의 RSPress가 pinned `2.0.22`인지 확인했다.
27. **선택 빌드로 전체 링크 검사 주장** — 제한된 route corpus 때문에 dead-link 검사를 끈 설정과 전체 문서 검사를 안 했다는 제한을 README에 썼다.
28. **route 누락·중복·path traversal** — 302 publication route와 manifest를 일대일 비교하고 safe-route 검사를 거쳤다.
29. **미리보기 파일의 무검증 교체** — preflight가 변경 파일 open handle 0개를 확인한 뒤 교체 전 69개를 고유 backup에 복사하고 SHA를 대조했다.
30. **최종 제공 stale/corrupt 또는 불필요 삭제** — local/HTTPS 302/302 HTTP 200 및 hash 일치를 확인했고 backups 69/69 일치, route deletion 0, server restart false다.

검토 결과에서 수정이 필요했던 것은 첫 RSPress 선택 build의 기본 dead-link 검사였다. 이 선택 publication은 문서 전체가 아니므로 기존 preview 설정처럼 `checkDeadLinks: false`로 다시 빌드하고, 이를 전체 링크 검증으로 확대하지 않았다. 성공 결과와 해당 제한을 기록했다.
