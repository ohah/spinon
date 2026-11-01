# PR 변경 검토 30개: full-width Tailnet 로드맵 동기화

대상은 PR #995/#996 병합 뒤 생성한 선택 문서 publication, Tailnet 복사·backup 원장, Agent Browser 확인 자료다. 각 항목은 이번 동기화의 별도 실패 경로나 상태 혼동을 확인한다.

1. **기준 HEAD:** selected-root와 publication이 문서 PR #996 merge HEAD `c2a30611244a9bb2d73ff135387157d5fe92109c`를 기록한다. 통과.
2. **source tree:** root/publication 원장이 같은 tree `a36d01a7f805948ff5cee68276ee2d153e15ecd0`를 가리킨다. 통과.
3. **구현 merge identity:** 로드맵 카드에 PR #995 merge SHA `772df994a5fba4aa8d405b3f42792c281150f88c`가 보인다. 통과.
4. **문서 merge identity:** 상태 문서의 source가 PR #996 merge HEAD에 맞는다. 통과.
5. **official count:** 로드맵과 상태 대장은 `72/213`을 유지한다. 통과.
6. **부모 상태 과장:** C11 및 C11.4 상위 checkbox를 완료로 바꾸지 않았다. 통과.
7. **fixture 경계:** full-width 지원 범위가 정확한 직접 `lang=ja`, 폰트·크기·source 제한 opt-in fixture로 남아 있다. 통과.
8. **Android 범위:** 기록은 API 37 ARM64 AVD 4/4 layout scene이며 실기기 결과라고 하지 않는다. 통과.
9. **iOS 범위:** 기록은 iPhone 17 Pro/iOS 26.2 Simulator 4/4이며 iOS 실기기라고 하지 않는다. 통과.
10. **화면 주장:** `layout=ready` 캡처를 glyph pixel 증거로 주장하지 않는다. 통과.
11. **Chrome oracle:** unsupported value의 computed `none` 관측을 full-width 기대 결과와 구분한다. 통과.
12. **일반 CSS admission:** 고정 fixture 통과를 일반 CSS admission으로 확장하지 않는다. 통과.
13. **Unicode 범위:** 임의 Unicode 구현이나 전체 text-transform 지원으로 표현하지 않는다. 통과.
14. **WPT 범위:** Spinon WPT 적합성을 주장하지 않는다. 통과.
15. **플랫폼 범위:** 실기기 및 hardware GPU 검증을 했다고 적지 않는다. 통과.
16. **선택 build 버전:** RSPress 2.0.22를 기록하며 업데이트하지 않는다. 통과.
17. **링크 검사 범위:** 선택 build에서 dead-link/dead-image 검사가 비활성화된 점을 보고한다. 전체 문서 링크 검증으로 확대하지 않는다. 통과.
18. **기존 publication 보존:** PR #990 시점 1,253개 기존 route를 입력 기준으로 썼다. 통과.
19. **route 수 산술:** publication은 1,463개, 추가 route는 210개로 manifest에 기록된다. 통과.
20. **변경 route 수:** 기존 publication과의 내용 변경 296개가 manifest에 기록된다. 통과.
21. **삭제 방지:** publication과 Tailnet 복사 모두 route 삭제가 없다. 통과.
22. **copy 전 open handle:** 변경 대상 파일 open handle 0개를 확인했다. 통과.
23. **Tailscale 상태:** copy 전 backend `Running`을 확인했다. 통과.
24. **preview identity:** 예상 Tailnet preview server PID 28040과 실행 스크립트를 확인했다. 통과.
25. **server lifecycle:** 기존 preview server를 재시작하지 않았다. 통과.
26. **원자 교체:** 기존 route 95개를 임시 파일 후 rename으로 교체하고 별도 backup을 만들었다. 통과.
27. **backup 무결성:** 교체 전 backup 95/95 hash가 기존 target hash와 일치한다. 통과.
28. **publication 무결성:** served route 1,463/1,463 hash가 publication manifest와 일치한다. 통과.
29. **핵심 HTTP:** 로드맵 및 연결 문서 18개가 local과 Tailnet HTTPS에서 각각 200/hash 일치다. 통과.
30. **실제 화면 및 정리:** Agent Browser에서 PR #995 내용과 72/213을 확인했다. 390px viewport document/body 폭도 390px였고, 세션 종료 후 browser process가 남지 않았다. 통과.

## 발견한 한계

전체 dead-link/dead-image build, GitHub CI pass, C11/C11.4 상위 완료, glyph pixel, 실기기/hardware GPU 증거는 이 작업으로 만들지 않았다. PR merge와 simulator evidence의 의미를 이 동기화 범위에 섞지 않는다.
