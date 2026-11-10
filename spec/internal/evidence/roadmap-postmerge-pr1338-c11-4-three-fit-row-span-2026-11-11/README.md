# PR #1338 병합 후 C11.4 상태·로드맵 정정

C11.4 bounded three-fit repeated-text row-span implementation PR [#1338](https://github.com/ohah/spinon/pull/1338)은 `2026-11-10T16:05:13Z` Rebase and merge로 병합됐다. 구현 merge SHA는 `ebcf720d65246340632273f1a40500b346cbf403`이며 PR source head는 `90daa675fd16e0ce5f2d1373dad3cd3eed3daf3b`다. 이 문서 변경은 그 제한 slice 결과를 공식 상태 대장, 내부 계약 0059, 집중 계획과 공개 roadmap에 반영한다.

## 구현 상태와 공식 집계

- C11.4/C11 완료 체크는 미완료로 유지하고 공식 완료 집계 `78/214`를 바꾸지 않는다.
- 내부 numeric contract는 `0.1.0`을 유지한다. 공개 API·FFI 변경은 없다.
- 공개 roadmap의 `C11.4 bounded` 카드를 `할 일`에서 `진행 중`으로 옮겼다. three-fit row-span bounded slice 자체는 PR #1338로 구현·병합됐고 C11.4에는 다른 미완료 범위가 남아 있다. 다음 bounded slice는 현재 코드와 계획을 다시 대조한 뒤 따로 선택한다.
- GitHub PR #1338에는 workflow run과 status check이 0건이었다. `main` branch required status checks도 설정되어 있지 않아 CI green으로 기록하지 않는다.

## 구현 범위와 근거

정확히 세 `fit-content()`와 `min-content`·`max-content`·`auto` 각 한 개로 된 six-member `grid-auto-rows` list, 여섯 row 중 모든 20 fit-position triple, 여섯 tail 순서를 고정 repeated-text row-span profile에 연결했다. Chrome 157 RuntimeSession 비교 11,520/11,520 scene, 기존 repeated-text row-span 960-scene regression, one/two-fit controls, four-fit/calc/mixed-minimum rejection와 same-session recovery가 통과했다. Three-fit auto-placement와 임의 track list/topology는 계속 거부한다.

- [집중 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md)
- [내부 Grid 계약 0059](https://github.com/ohah/spinon/blob/main/spec/internal/0059-c11-grid-layout.md)
- [플랫폼 smoke와 정리 근거](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-runtime-2026-11-10/README.md)
- [구현/runtime 검토 20관점](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-runtime-2026-11-10/implementation-runtime-review-20.md)
- [구현 PR diff 검토 30관점](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-runtime-2026-11-10/implementation-pr-diff-review-30.md)
- [이 문서 변경의 diff 검토 30관점](./post-merge-docs-pr-diff-review-30.md)

## Agent Browser 로드맵 화면

수정된 `spec/public/roadmap.html`을 작업 전용 localhost server에서 Agent Browser로 확인했다. 데스크톱과 모바일 모두 공식 상태 fetch는 `78/214`, 칸반의 `세 fit row-span`은 `진행 중` 열 `1/2`에 한 번 표시됐고 heading은 한 개다. 모바일 viewport와 문서 폭은 390px이며 Kanban만 362px 컨테이너 안에서 1,120px 가로 scroll을 갖는다.

- [데스크톱 1440×1000](./roadmap-local-desktop.png) · SHA-256 `49e608dedb8a53a874fe87c77d2b0abd44d52c53c9f41c2cbd9a3eec916aba53`
- [모바일 390×844](./roadmap-local-mobile.png) · SHA-256 `92cb83ccc7debe42e2ce10640fa9e0b0d51de5c6176f725c0544be55220020f9`

Agent Browser session을 닫고 전용 localhost server를 종료했다. 이후 `agent-browser`/server process와 iOS simulator가 실행 중이지 않은 것을 확인했다.

## 플랫폼·프로세스 정리

iPhone 17 Pro/iOS 26.2 Simulator Metal과 Android API 37.0 ARM64 AVD/SwiftShader에서 고정 fixture의 light/dark 기본 Root HTML route smoke를 수행했다. 앱 로그는 fixture hash, `layout=ready`, 14 boxes, draw/present와 OS color-scheme 전환을 기록한다. 결과는 OS screenshot smoke이며 surface readback, compositor 이후 pixel parity, 실기기/hardware GPU 또는 전체 WPT 결과가 아니다.

스모크 후 iOS simulator에 설치된 `dev.spinon.bootstrap`만 제거하고 공용 iPhone 17 Pro profile을 `Shutdown`했다. 기존 profile 데이터와 다른 simulator apps는 유지했다. S03 이름의 simulator device는 등록돼 있지 않았다. Android 기존 앱 APK와 appearance는 원래 상태로 복원했다. Agent Browser에서 PR #1338 본문의 commit-pinned 플랫폼 이미지 네 장이 모두 로드된 뒤 세션을 닫았다.

Tailnet preview는 문서 PR이 main에 병합된 뒤 갱신한다. 이 디렉터리의 Tailnet 실행·HTTP/hash·desktop/mobile 화면 증거는 별도 publication evidence로 기록한다. GitHub Pages 수동 배포는 실행하지 않는다.
