# C11.1 PR #1070 병합 후 로드맵·Tailnet 문서 diff 실패 관점 검토 30개

대상은 병합 메타데이터를 반영한 STATUS/계획/계약/공개 roadmap 문단과 선택 Rspress publication, Tailnet 복사·검증 도구 및 Agent Browser evidence다. 구현 PR의 30개 검토를 재사용하지 않았다.

1. **Merge SHA identity:** PR API와 fetch한 `origin/main`이 `f40cb3b93d2555dafecd352bd35fb349b6bf7a64`로 일치한다.
2. **Merge time:** GitHub의 UTC `mergedAt` `2026-11-02T16:37:30Z`를 문서·JSON·roadmap에서 그대로 사용한다.
3. **Merge method:** merge record는 프로젝트 계약의 Rebase and merge만 기록한다.
4. **공식 완료 수:** 각 공개 문서와 manifest에서 `72/213`이 유지된다.
5. **C11 완료 경계:** STATUS 체크, roadmap copy, plan과 contract가 C11/C11.1 완료를 주장하지 않는다.
6. **제한 후보 경계:** 공개 카드와 README가 no-area Root HTML direct px/%/fr implicit-row list만 적는다.
7. **실패한 넓은 회귀:** `style-to-layout root_html` 36/37 실패와 정확한 auto-column negative fallback 경계를 공개한다.
8. **회귀 원인 과장 방지:** auto-column 코드를 수정하지 않았다는 implementation review와 넓은 suite failure를 구분해 기록한다.
9. **CI 표시:** 빈 PR check rollup과 미설정 main required checks를 CI 통과로 부르지 않는다.
10. **Android 장치 종류:** API 37 ARM64 AVD와 ANGLE/SwiftShader software renderer를 명시하고 실기기로 확대하지 않는다.
11. **iOS 장치 종류:** iPhone 17 Pro/iOS 26.2 Simulator임을 유지하고 실기기로 확대하지 않는다.
12. **Pixel 증거 경계:** simulator screenshot을 visual smoke로 제한하고 PixelCopy/Metal readback으로 표현하지 않는다.
13. **Tailnet 목적지:** `SpinonPreview`만 publication target이며 GitHub Pages나 외부 문서 배포를 건드리지 않는다.
14. **Preview server 소유권:** PID·loopback port를 확인하고 이미 실행 중인 preview server를 재시작하지 않는다.
15. **Tailnet 사전 조건:** Tailscale backend `Running`, preview root real directory, 예상 server process와 local HTTP 200을 검사한다.
16. **Symlink 경계:** source와 destination route walk에서 symlink를 거부해 publication 밖으로 벗어나지 않는다.
17. **Route 경계:** 절대 경로, `..`, 빈 경로 요소와 역슬래시를 거부한다.
18. **Publication 고정성:** source route 수, bytes, SHA-256이 manifest와 같지 않으면 preflight가 중단한다.
19. **열린 파일 보호:** 바뀌는 target에 `lsof` open handle이 있으면 복사를 거부한다.
20. **동시 변경 보호:** copy 직전 대상 존재 상태와 기존 SHA가 preflight와 달라지면 덮어쓰지 않는다.
21. **기존 파일 backup:** 교체되는 5개 target을 별도 unique backup path에 복사하고 기존 SHA를 검증한다.
22. **원자 복사:** unique temporary 파일을 목적지와 같은 filesystem에 만든 뒤 rename하고 복사 후 SHA를 대조한다.
23. **삭제 없음:** copy script의 분기와 세 report가 모두 route 삭제 0개를 기록한다.
24. **생성·교체·동일 분류:** 48 created, 5 replaced, 12 identical 수가 preflight·copy summary와 일치한다.
25. **실제 링크 집합:** 새 roadmap card의 10개 local href 각각이 selected publication에 존재하고 hash를 갖는다.
26. **Rendered page routes:** 계획, precomparison/runtime README, contract와 STATUS HTML이 Rspress selected build 결과에 포함된다.
27. **Raw evidence routes:** `.md` reviews/merge record, PNG screenshots, fixture와 compressed logs가 roadmap/README 참조 경로에 대응한다.
28. **HTTP 결과:** local 65/65 hash 및 Tailscale HTTPS HTTP 200/hash 65/65와 backup 5/5를 따로 기록한다.
29. **반응형 경계:** Agent Browser에서 desktop 1280px·mobile 390px document width가 viewport와 같고 카드 bounds가 화면 안이다.
30. **검증 도구 lifecycle/build 범위:** plan 링크 실제 렌더링을 확인하고 Agent Browser를 닫았다. 선택 Rspress build는 full-site dead-link 검사가 아니며 초기 publication guard 오류는 Tailnet 파일 변경 전 수정했다.

이 검토는 문서·선택 publication과 Tailnet preview 경계에 대한 것이다. 전체 Rspress 문서 사이트, 전체 URL 집합 또는 CSS 적합성을 증명하지 않는다.
