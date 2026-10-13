# C11.2.3 iOS 소스 복구 구현 검토

## 확인한 실패 경계와 대응

1. C11.2.3의 flag와 scene 인수를 `AppDelegate`가 받지 않아 흰 화면을 여는 결함을 확인했다. route 조건을 보완한 후보에서 전체 runtime marker를 수집했다.
2. boolean flag 없이 scene 인수만 있는 유효 실행을 확인했다. `row-sparse-baseline`이 6개 node frame과 draw까지 완료했다.
3. scene 없이 boolean flag만 있는 입력을 확인했다. C0410 화면으로 진입해 `scene_id=`를 포함한 명시적 argument rejection을 기록했다.
4. C11.2.2 scene 인수가 C11.2.3 화면으로 잘못 분류되는지 공격했다. 수정 후보 C11.2.2 16-scene 회귀 행렬이 통과했다.
5. C11.2.1·일반 C11 Grid·C04.10 route 분기가 영향을 받는지 살폈다. 수정은 C11.2.3 인수 조건 두 개만 추가하며 그 외 조건을 제거하지 않는다.
6. Objective-C 선언 selector가 구현과 다른지 확인했다. 누락된 C11.2.3 host 생성 및 surface readback selector를 기존 `.mm` 구현과 일치시켰다.
7. 반환값·handle·표본 포인터 타입이 ABI와 맞는지 대조했다. 기존 C11.2.3 FFI 구현 타입을 그대로 선언했다.
8. 선언이 다른 header copy에만 들어가 빌드가 계속 실패하는 위험을 공격했다. 격리 후보에서 실제 Xcode simulator build와 25-scene matrix가 진행됐다.
9. 긴 Swift 중첩 조건식이 compiler type-check 한계를 다시 넘는지 확인했다. 기존 분기 우선순위를 보존한 순차 조건식 후보가 빌드됐다.
10. 분기 정리 중 C11.2.3이 C11.2.2보다 우선되지 않는지 살폈다. 기존 지원 경로의 조건 순서를 유지하고 C11.2.3 경로를 기존 C11.2.2보다 앞서 명시했다.
11. Swift optional 결과를 강제 unwrap하는 회귀가 있는지 확인했다. 후보는 기존 `nil` 처리와 실패 marker를 유지한다.
12. launch argument가 process에만 전달되고 view controller는 열리지 않는지 재현했다. 수정 전 후보는 흰 화면·무표식 timeout, 수정 후보는 C1123 frame/draw marker를 남겼다.
13. matrix scene 사이의 오래된 process state가 결과를 오염하는지 확인했다. 25개 장면을 모두 새 앱 process에서 실행했다.
14. run ID가 이전 scene과 섞이는지 확인했다. runtime summary·node frame·pixel matrix marker가 현재 장면 run ID와 일치했다.
15. 성공 scene을 rejection으로 오판하거나 rejection을 timeout으로 통과시키는지 확인했다. 성공 19/19와 기대 거부 6/6이 분리 집계됐다.
16. geometry를 screenshot만으로 주장하는 위험을 확인했다. 107 node geometry는 Chrome 기준과 비교해 최대 오차 0 CSS px였고 허용 오차는 0.5 CSS px다.
17. OS compositor 이후 화면 parity라고 과장할 위험을 확인했다. iOS 결과는 Metal drawable 표본 99개이며 compositor 이후 픽셀은 미검증으로 남겼다.
18. 표본 하나가 빠지거나 RGBA 채널 오차를 허용하는지 확인했다. 성공 19 scene·99/99 sample exact, 최대 채널 오차 0이었다.
19. 기존 C11.2.2 scene route가 손상되는지 확인했다. 수정 후보의 회귀 행렬은 16 scene, 성공 11/11·기대 거부 5/5, 69 node, geometry 0 CSS px, drawable 51/51 exact였다.
20. 임시 후보 결과를 병합된 `origin/main`의 증거로 잘못 취급할 위험을 확인했다. C11.2.3은 수정 PR 병합과 `origin/main` 재실행 전까지 미완료 상태로 유지한다.

## 실행 근거

- [C11.2.3 25-scene 수정 후보](./c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv6mhrrq-785dbe7a/README.md)
- [C11.2.2 16-scene 회귀](./c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1122-ios-regression-mv6mtjb4-6dbbbe77/README.md)
- [scene-only 및 flag-only route 경계](./c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv6mhrrq-785dbe7a/route-edge-cases.log)
- [수정 전 흰 화면 negative control](./c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv6mhrrq-785dbe7a/route-missing-negative-control/README.md)

이 검토는 임시 소스 후보의 iOS Simulator 결과다. iOS 실기기, compositor 이후 화면, release 성능, 전체 CSS Grid 및 WPT 적합성은 판정하지 않는다.
