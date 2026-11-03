# C11.5 공개 로드맵 HTML 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`의 C11.5 auto-size/stretch 병합 카드와 바로 앞 fixed-size self-alignment 카드의 범위 문구, Tailnet preview sync 기록, Agent Browser screenshot이다. 스크린샷은 이 정적 로드맵 페이지의 표시 결과를 증명하며 앱 runtime 동작 증거로 취급하지 않는다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | 카드가 PR #1132를 올바르게 연결하는지 | 통과: PR #1132 링크를 사용한다. |
| 2 | merge timestamp가 GitHub UTC 값과 일치하는지 | 통과: `2026-11-03T20:03:49Z`다. |
| 3 | merge SHA가 PR #1132 실제 merge commit인지 | 통과: `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`이다. |
| 4 | auto-stretch가 계획 단계로 남아 있는지 | 통과: 제한 구현 PR 병합으로 수정했다. |
| 5 | C11.5 전체 완료를 선언하는지 | 통과: C11.5 전체가 미완료라고 쓴다. |
| 6 | C11 전체 완료를 선언하는지 | 통과: C11 전체가 미완료라고 쓴다. |
| 7 | 프로젝트 완료 수가 실제보다 증가했는지 | 통과: `72/213`을 유지한다. |
| 8 | 10 positive scene과 16 negative controls가 잘못 합쳐졌는지 | 통과: 두 집합을 별도 표기한다. |
| 9 | Android API 37 AVD를 physical device로 오해할 수 있는지 | 통과: AVD라고 명시한다. |
| 10 | Android software renderer 한계가 빠졌는지 | 통과: ANGLE Vulkan SwiftShader라고 명시한다. |
| 11 | Android PixelCopy 표본 수가 정확한지 | 통과: 11/11 exact다. |
| 12 | iOS Simulator를 실기기로 오해할 수 있는지 | 통과: iPhone 17 Pro/iOS 26.2 Simulator다. |
| 13 | iOS Metal 결과가 compositor 이후라고 오해할 수 있는지 | 통과: compositor 제출 전이라고 명시한다. |
| 14 | 기존 fixed-size 회귀 행렬을 auto-stretch와 혼동했는지 | 통과: 두 플랫폼 7/7 회귀라고 분리했다. |
| 15 | 계획 링크가 새 제한 slice 계획을 가리키는지 | 통과: 전용 auto-stretch 계획이다. |
| 16 | 실행 링크가 실제 simulator evidence를 가리키는지 | 통과: 실행 근거 README다. |
| 17 | 실패 관점 리뷰와 simulator 결과가 같은 종류의 증거로 합쳐졌는지 | 통과: 이 카드는 검증 matrix와 한계를 구분한다. |
| 18 | fixed-size 카드가 여전히 auto/stretch 미완료라고 말하는지 | 통과: stale `auto/stretch sizing` 한계를 제거했다. |
| 19 | fixed-size card의 기존 scope가 손상됐는지 | 통과: fixed-track self-alignment 설명은 유지했다. |
| 20 | intrinsic track sizing 완료를 암시하는지 | 통과: intrinsic sizing은 미완료로 남는다. |
| 21 | baseline support 완료를 암시하는지 | 통과: baseline은 미완료로 남는다. |
| 22 | 일반 CSS admission을 선언하는지 | 통과: 선언하지 않는다. |
| 23 | WPT conformance를 선언하는지 | 통과: 선언하지 않는다. |
| 24 | 전체 CSS Grid 지원을 선언하는지 | 통과: 선언하지 않는다. |
| 25 | public API/ABI를 추가했다고 암시하는지 | 통과: 그런 주장을 넣지 않았다. |
| 26 | check rollup이 빈 상태에서 CI 통과를 주장하는지 | 통과: rollup 공백과 CI 비주장을 적었다. |
| 27 | HTML markup이 기존 paragraph 구조를 유지하는지 | 통과: C11.5 카드 paragraph 구조를 유지했다. |
| 28 | 코드 요소의 `<code>` markup이 닫히는지 | 통과: 새 SHA 및 CSS value markup을 확인했다. |
| 29 | 로드맵 screenshot을 앱 runtime screenshot으로 오해할 수 있는지 | 통과: screenshot은 페이지 표시 확인에만 사용한다. |
| 30 | 변경이 C11.5 관련 두 카드 밖으로 번졌는지 | 통과: HTML은 두 progress card만 바꾸고 screenshot/sync receipt만 근거로 추가했다. |

**결론:** 공개 로드맵은 PR #1132 병합을 반영하면서 C11.5/C11의 미완료 상태와 `72/213`을 보존한다.
