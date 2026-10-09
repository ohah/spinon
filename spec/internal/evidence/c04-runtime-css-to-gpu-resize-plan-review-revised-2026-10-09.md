# C04.10 surface resize 계획 재검토

- 검토 대상: [`C04.10 구현 계획`](../../../plan/c04-runtime-css-to-gpu.md)
- 계획 SHA-256: `e6893181941a2820f1db29782a7520703aa2e30d9af3288546ec5a478b79d199`
- 기준: [Chromium resize precomparison](c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md)
- 재검토 계기: Android emulator에서 resize API와 새 viewport 계산이 성공해도 화면 색상 영역이 이전 표면 크기에 남았다. native window·WGPU texture·scene·screenshot을 분리 측정했다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검토 결과와 계획 반영 |
|---:|---|---|
| 1 | Android `surface.configure`가 성공하면 새 화면 크기도 반영됐다고 간주하는가 | 이전 크기 장면이 남는 현상을 재현했다. 성공 코드 대신 실제 획득 texture와 캡처 경계를 완료 조건에 추가했다. |
| 2 | WGPU acquired texture가 native window와 다른 크기인데 알아채지 못하는가 | 요청·native window·texture pixel 크기를 각각 기록한다. |
| 3 | Rust layout viewport만 새 값이고 GPU scene은 이전 값일 수 있는가 | 계산 root, render viewport, render root와 실제 draw 로그를 함께 대조한다. |
| 4 | GPU scene은 최신이어도 screenshot이 이전 프레임일 수 있는가 | 캡처 시점을 draw 완료 로그 이후로 두고 동일 프로세스 로그와 캡처를 보존한다. |
| 5 | Android renderer 재생성 중 이전 context가 새 surface를 참조하는가 | 동일 serial render queue에서 이전 renderer를 파괴한 후 최신 SurfaceView로 새 renderer를 만든다. |
| 6 | renderer 재생성이 V8 host와 DOM tree까지 버리는가 | GPU renderer만 교체하고 `RuntimeGpuHost`·V8 session·HostDocument는 유지한다. |
| 7 | 이전 generation의 recreate task가 새 크기를 덮는가 | 매 task에서 캡처한 surface generation을 확인하고 이전 값은 조용히 중단한다. |
| 8 | 빠른 크기 왕복에서 두 재생성이 역순으로 끝나는가 | render queue를 직렬로 두고 완료 직전 현재 generation을 재검사한다. 빠른 연속 전환을 구현 검증 항목에 남긴다. |
| 9 | 같은 크기의 중복 `surfaceChanged`가 불필요한 renderer 재생성을 일으키는가 | 이전 유효 크기와 실제 새 크기가 다를 때만 재생성한다. 중복 callback은 현재 renderer 경로에서 처리한다. |
| 10 | 첫 surface 생성에서 이전 크기가 없는데 renderer를 파괴하는가 | 크기 전환이 아니라 초기 surface 생성일 때는 최초 renderer를 만든다. |
| 11 | recreate 중 `surfaceDestroyed`가 오면 오래된 native window가 살아남는가 | destroy와 callback drain을 같은 render queue로 직렬화하고 window owner를 renderer 종료까지 유지한다. |
| 12 | shutdown 중 대기 중인 새 renderer가 생성되는가 | `closing`, surface availability와 generation을 모두 확인한 뒤 create를 허용한다. |
| 13 | 새 renderer가 만들어지기 전에 새 environment를 적용하거나 draw하는가 | 재생성된 renderer가 준비된 뒤 최신 environment를 적용하고 성공한 scene만 draw한다. |
| 14 | Android dp의 정수 반올림이 Chromium 기준을 넘는가 | 341 dp가 895 pixel / 2.625 = 340.952 CSS px가 된다. 허용 오차 0.5 CSS px 안에서 비교하고 원시값도 보존한다. |
| 15 | Android backing pixel과 CSS viewport에 density를 두 번 적용하는가 | CSS viewport는 surface pixel/density, WGPU surface는 원시 pixel로 각 한 번만 계산한다. |
| 16 | iOS point 크기와 Metal drawable pixel을 혼용하는가 | `UIView.bounds` point는 CSS viewport, point×scale은 drawable pixel로 기록한다. |
| 17 | iOS resize 중 UIKit layer configure가 render queue에서 실행되는가 | configure는 main thread, 기존 draw와는 serial queue barrier로 분리한다. |
| 18 | 테스트 실행 인자가 사용자 런타임 동작으로 노출되는가 | `--spinon-c0410-auto-resize`는 Simulator 전용 재현 인자이며 public API·앱 설정이 아니다. |
| 19 | 화면이 커졌지만 CSS `100vw/100vh` root가 이전 크기면 성공으로 처리하는가 | Chromium의 두 기준과 root computed/frame, render scene root, surface 전체 색상 영역을 교차 확인한다. |
| 20 | 내부 API 계약 버전이 resize 구현만으로 증가하는가 | 계획과 내부 계약은 출시 전 `0.1.0`으로 유지한다. 제품 버전 변경은 별도 release 결정이 있어야 한다. |

계획의 핵심 수정은 Android에서 크기 변경 시 기존 renderer의 `resize` 성공만 신뢰하지 않고 renderer를 같은 host 안에서 재생성하도록 한 점이다. 실제 화면 전체가 새 크기로 채워지는지 native window·WGPU texture·CSS scene·캡처로 각각 증명한다. renderer 재생성 비용과 빠른 연속 크기 변경은 구현 검증에서 추가로 측정한다.
