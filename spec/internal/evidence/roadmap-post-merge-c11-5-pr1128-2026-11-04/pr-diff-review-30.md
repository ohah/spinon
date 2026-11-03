# C11.5 병합 후 로드맵 동기화 PR 변경분 적대 검토 30개

각 항목은 PR #1129의 문서·로드맵·동기화 근거에서 별도의 실패 경로나 불변 조건을 확인한다. 코드를 바꾸는 구현 PR 검토가 아니라 병합 후 상태 동기화 검토다.

1. **PR 식별자** — 로드맵과 상태 대장이 제한 구현 PR #1128을 가리키며, 문서 동기화 PR #1129와 혼동하지 않는지 확인한다.
2. **병합 결과** — #1128의 Rebase and merge 시각 `2026-11-03T17:52:20Z`가 세 문서의 기록과 일치한다.
3. **merge SHA** — `0f704151cfd805ad2fef2e12535739f0d3a109dd`가 PR, 상태 대장, 내부 계약, 로드맵에서 동일하다.
4. **체크 상태** — check rollup이 비어 있다는 기록을 CI 성공으로 부풀리지 않았는지 확인한다.
5. **required checks** — main의 required status check 부재와 CI 결과 부재를 구분한다.
6. **공식 완료 수** — 변경 전후 공식 완료가 `72/213`으로 유지되는지 확인한다.
7. **상위 C11 상태** — C11 체크박스가 unchecked로 남고 C11 완료로 오인할 문구가 없는지 확인한다.
8. **C11.5 범위** — 제한된 Root HTML slice와 전체 C11.5 계약을 구분한다.
9. **입력 제한** — 기존 eligible fixed-track Grid, direct empty `div`, fixed-size item 제한을 일반 Grid 지원으로 확장하지 않는다.
10. **지원 속성** — `align-self`·`justify-self` 위치 정렬 설명이 plan 및 내부 계약과 일치한다.
11. **CSS admission** — 일반 CSS authoring 지원으로 승격하거나 공개 지원 표에 추가했다고 암시하지 않는다.
12. **WPT 경계** — pinned Chrome 비교를 Spinon WPT 적합성 결과로 표현하지 않는다.
13. **플랫폼 경계** — Android AVD/SwiftShader 결과를 실기기 또는 hardware GPU 결과라 부르지 않는다.
14. **iOS 경계** — iOS Metal drawable 표본이 compositor 제출 전이라는 제한을 유지한다.
15. **Chrome 결과** — 19 scene 19/19를 Chrome 156 사전 비교 결과로 정확히 귀속하고 플랫폼 통과와 합산하지 않는다.
16. **Android 표본** — PixelCopy 7/7은 API 37 ARM64 AVD 결과이며 별도 OS 합성 PNG와 혼동하지 않는다.
17. **iOS 표본** — Metal drawable 7/7을 OS compositor 이후 광학 화면 표본으로 오인하지 않는다.
18. **기본 artifact 경계** — opt-in fixture adapter/payload가 기본 산출물에 없다는 증거를 positive artifact 포함 사실과 구분한다.
19. **광범위 테스트 1** — `spinon-layout` 423 passed/1 failed 기록을 누락하거나 모두 통과로 바꾸지 않는다.
20. **광범위 테스트 2** — `spinon-style-to-layout` 372 passed/1 failed 기록이 실행 근거와 일치한다.
21. **광범위 테스트 3** — `spinon-style` 283 passed/1 failed 기록이 실행 근거와 일치한다.
22. **광범위 테스트 4** — `spinon-runtime` 261 passed/2 failed/3 ignored 기록이 실행 근거와 일치한다.
23. **문서 교차 일관성** — STATUS, internal contract 0059, 계획 문서와 공개 로드맵에서 scope·counts·limitations가 서로 모순되지 않는다.
24. **계약 버전** — 제한 구현 병합만으로 내부 계약 버전 `0.1.0`을 올리지 않는다.
25. **카드 순서** — 새 C11.5 병합 기록이 최신 진행 공지 위치에 있고 기존 roadmap 항목을 덮어쓰지 않는다.
26. **링크 대상** — 새 문서 링크가 `main`의 GitHub blob 경로를 사용해 Tailnet의 미생성 라우트에 의존하지 않는다.
27. **링크 파일 존재** — plan, contract, precomparison, simulator evidence, review 및 STATUS 링크의 대상 파일이 저장소에 존재한다.
28. **반응형 데스크톱** — 로컬 1280px 및 Tailnet 1440px viewport에서 문서 너비가 viewport와 일치하고 카드가 화면 밖으로 나가지 않는다.
29. **반응형 모바일** — 로컬 및 Tailnet 390px viewport에서 body/document 너비 390px, 카드 362px로 수평 overflow가 없다.
30. **동기화 무결성·수명** — pass02 source/local/remote SHA와 byte count 일치, 이전 사본 백업, atomic rename, no-delete, preview PID 28040 무재시작이 근거 파일에 남는다.

## 검토 판정

30개 관점에서 공식 완료 수, C11 미완료 표기, 구현 범위, 광범위 테스트 결과, 링크 및 Tailnet 제공 무결성을 대조했다. 확인한 화면은 로드맵 문서 표시만 대상으로 하며 전체 Grid 호환성이나 앱 런타임 증거로 취급하지 않는다.
