# PR #918 runtime evidence link review 30

검토 대상은 C11.3.3 runtime README의 Android/iOS 39-scene matrix 링크 두 개다. GitHub PR 이후 생성되는 RSPress route를 가리키며, 실행 결과의 의미와 공식 상태는 바꾸지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 두 링크가 HTTPS 외부 주소가 아닌 사이트 내부 경로인가 | PASS — `/spinon/` root-relative |
| 2 | 두 링크 모두 표준 공개 route prefix를 쓰는가 | PASS — `/spinon/spec/` |
| 3 | 렌더링 route가 `.html`로 지정됐는가 | PASS |
| 4 | Android 링크가 올바른 run ID를 가리키는가 | PASS — API 37 AVD C11.3.3 matrix |
| 5 | iOS 링크가 올바른 run ID를 가리키는가 | PASS — iPhone Simulator C11.3.3 matrix |
| 6 | Android source README가 저장소에 있는가 | PASS |
| 7 | iOS source README가 저장소에 있는가 | PASS |
| 8 | Android result manifest가 같은 run에 존재하는가 | PASS |
| 9 | iOS result manifest가 같은 run에 존재하는가 | PASS |
| 10 | Android 링크가 이전 run ID로 잘못 고정되지 않았는가 | PASS — 2026-10-31 source-bound run |
| 11 | iOS 링크가 이전 run ID로 잘못 고정되지 않았는가 | PASS — 2026-10-31 source-bound run |
| 12 | 두 run의 source commit이 C11.3.3 implementation commit과 같은가 | PASS — `b815f483a253072cbbca1c0cd473d519fc95a220` |
| 13 | 링크가 source-only untracked artifact를 가리키지 않는가 | PASS — tracked README sources |
| 14 | raw `.md` URL로 route resolution을 건너뛰지 않는가 | PASS — `.html` |
| 15 | matrix link label이 platform과 역할을 명확히 나타내는가 | PASS |
| 16 | Android sample을 physical-device run으로 오해시키지 않는가 | PASS — AVD 명칭 유지 |
| 17 | Android API level이 37로 표시되는가 | PASS |
| 18 | Android software backend 한계를 보존하는가 | PASS — SwiftShader |
| 19 | Android surface sampling 경계를 보존하는가 | PASS — PixelCopy |
| 20 | iOS sample을 physical-device run으로 오해시키지 않는가 | PASS — Simulator 명칭 유지 |
| 21 | iOS device profile이 iPhone 17 Pro로 표시되는가 | PASS |
| 22 | iOS runtime이 26.2로 표시되는가 | PASS |
| 23 | iOS renderer/readback 경계가 Metal로 표시되는가 | PASS |
| 24 | iOS post-compositor 제한을 지우지 않는가 | PASS |
| 25 | matrix 규모가 각 39 scenes로 유지되는가 | PASS |
| 26 | matrix node 수가 각 126으로 유지되는가 | PASS |
| 27 | geometry 비교 결과가 0 CSS px로 유지되는가 | PASS |
| 28 | 변경이 C11.3.3 candidate를 공식 완료로 승격하지 않는가 | PASS |
| 29 | 변경이 공식 완료 72/213을 수정하지 않는가 | PASS |
| 30 | 두 링크가 build route에서 열리고 실행 행렬이 맞는가 | PASS — Agent Browser에서 Android/iOS 각 200, 39 scenes·126 nodes, broken image 0; [측정·hash](runtime-matrix-link-fix-browser-verification-pass01.json) |

검토 범위는 해당 문서 diff와 고정 run 경로다. 이 검토는 앱 테스트나 CI 결과를 대신하지 않는다.
