# C11.1 10 scene 모바일 표면 픽셀 구현 검토 · 2026-10-12

검토 입력은 현재 작업 트리의 generator, Android/iOS 인수 처리, PixelCopy/Metal readback, 독립 비교기와 실제 AVD/Simulator 실행이다. 계획 검토를 재사용하지 않았다.

| # | 실패 관점 | 검증 결과 |
|---:|---|---|
| 1 | Chrome reference가 바뀌었는데 예전 표본을 재사용하는가 | reference·PNG SHA-256을 생성 fixture에 고정하고 실제 PNG를 검사했다. |
| 2 | HTML inventory에서 node가 빠지거나 순서가 바뀌는가 | reference node preorder와 inventory ID 집합을 생성기에서 검사한다. inventory 변조 negative control이 실패했다. |
| 3 | screenshot 자체가 바뀌는데 해시 검사가 놓치는가 | PNG bytes 변조 negative control이 fixture 생성에서 실패했다. |
| 4 | viewport 크기와 sample 좌표 영역을 혼동하는가 | oracle screenshot은 360×800, runtime sample viewport는 320×240으로 별도 고정했다. 좌표는 runtime 범위 안이다. |
| 5 | 경계·anti-alias 픽셀을 내부 표본으로 오인하는가 | 표본마다 3×3 patch가 같은 RGBA인지 확인하고 실패하면 생성하지 않는다. |
| 6 | 구조 부모의 중앙점이 자식 경계에 걸리는가 | 부모 전체를 표본화하지 않고 색상 item과 내부 track 배경만 뽑는다. 문제 장면을 고친 후 10 scene·36 sample 생성이 통과했다. |
| 7 | 장면·표본 개수 누락을 부분 성공으로 처리하는가 | fixture summary를 10 scene·36 sample로 고정하고 양 플랫폼 collector가 모든 장면과 sample 집합을 검사한다. |
| 8 | 표본 ID나 좌표가 중복되는가 | generator 및 플랫폼 인수 파서에서 중복 ID/좌표를 거부한다. |
| 9 | CLI 모듈을 테스트에서 import하면 실제 파일을 덮어쓰는가 | import 시 CLI가 실행되던 결함을 direct-execution guard로 고쳤다. generator test를 import해서 실행했다. |
| 10 | Android Java 소스 수준과 구현 문법이 맞는가 | 첫 AVD build가 Java 8에서 pattern-matching 문법으로 실패했다. parser를 Java 8 문법으로 바꾸고 AVD 앱 빌드·10 scene 실행을 통과시켰다. |
| 11 | Android intent가 임의 장면 또는 다른 scene index를 받는가 | scene allowlist, run ID, URL-safe Base64 크기, scene/index 일치를 검사하고 오류는 host 초기화 전에 기록한다. |
| 12 | Android JSON에 NaN·소수·범위 밖 좌표가 들어가는가 | parser가 finite 정수·320×240 경계·고유 ID/좌표와 1–64 sample 수를 검사한다. |
| 13 | Android readback이 재생성된 다른 surface에서 오는가 | PixelCopy 요청 전후 renderer handle·generation·크기를 검사한다. begin/capture/sample/summary generation을 비교기에 연결했다. |
| 14 | sample 로그를 다른 process의 실행과 합치는가 | Android sample log에 PID와 generation을 추가했다. 비교기는 begin·capture·sample PID와 generation의 일치를 요구한다. |
| 15 | PixelCopy 파일이 로그와 다르거나 경로를 탈출하는가 | 앱 cache 디렉터리·scene/run filename을 제한하고 PNG SHA-256, PNG 크기, 로그 RGBA를 실제 PNG pixel과 교차 검사한다. |
| 16 | CSS 좌표의 density 변환에서 반 픽셀·축 계산이 틀리는가 | PixelCopy와 비교기가 같은 `floor((css + 0.5) × density)` 좌표를 독립 계산하고 PNG pixel을 읽는다. |
| 17 | Android 비교기가 누락·중복·unknown·stale sample을 허용하는가 | run ID, scene, ID 집합, 표본 수, 좌표, RGBA, PNG 손상/hash 변조 등 22개 comparator 검사가 모두 통과했다. |
| 18 | iOS drawable이 CSS viewport·revision과 다를 때 표본을 받는가 | Metal 경로가 320×240 viewport, 같은 drawable 세대, 동일 x/y scale, document/render/environment revision과 1–64 sample을 검사한다. |
| 19 | unified log에 여러 앱 실행이 섞이면 comparator가 중복 실행으로 오판하는가 | 기존 comparator가 전체 로그 marker를 세던 결함을 발견해 run ID로 분리했다. 섞인 실행을 포함한 회귀 검사가 통과했다. |
| 20 | 결과를 WPT·실기기·최종 display로 과장하는가 | Android는 SwiftShader AVD, iOS는 compositor 이전 Simulator drawable로 제한해 기록했다. WPT runner, 실기기 및 전체 CSS 지원은 미완료로 유지한다. |

## 실행 결과

- Android API 37 AVD: 10/10 scene·36/36 PixelCopy pixel exact, 각 surface PNG·로그를 보존했다.
- iPhone 17 Pro / iOS 26.2 Simulator: 10/10 scene·36/36 Metal drawable pixel exact, 통합 로그와 장면별 보조 screenshot을 보존했다.
- Android/iOS comparator negative control과 Chromium fixture generator 검사는 `bun run test` 안에서 통과했다. 전체 workspace는 596 passed·3 ignored, CSS reference 298/298, Clippy·rustfmt 통과다.
- Android 실기기는 이 실행 동안 ADB에 연결되지 않았고 iOS 실기기 검증도 하지 않았다. Spinon WPT runner 및 C11 전체 CSS 지원은 이 검토 결과로 닫지 않는다.
