# C11.2.1 모바일 표면 픽셀 행렬 · 구현 실패 관점 검토

검토 범위는 sample matrix 생성기, Android/iOS capture runner와 scene allowlist, PixelCopy/Metal comparator, 각 플랫폼의 10-scene 실행 로그, 통합 test·build 결과다. 서로 다른 실패 상황 20개를 각각 공격했다.

| # | 공격 관점 | 확인 결과와 증거 |
| ---: | --- | --- |
| 1 | 오래된 C11.1 reference나 다른 Chrome run을 기대값으로 끼워 넣는다. | reference ID, inventory·HTML·runtime matrix·generator hash와 DPR 1 screenshot hash를 고정했다. 생성기 재현 test와 `--check` 통과. |
| 2 | scene screenshot을 다른 ID에 연결한다. | inventory·runtime matrix·Chrome observation·screenshot 목록의 scene order와 node preorder를 생성 단계에서 일치시킨다. ID 변조 test는 실패한다. |
| 3 | 입력 HTML 또는 palette가 바뀌어 RGBA oracle만 조용히 바뀐다. | palette는 fixed map과 대조하고 HTML hash drift를 거부한다. palette 변조 negative control 통과. |
| 4 | 이미지 생성이 비결정적이라 같은 입력에서 다른 좌표를 만든다. | node rect 중심에서 가까운 3×3 균일색 pixel을 고르고 동률은 y, x 순으로 정한다. byte-for-byte fixture regeneration 통과. |
| 5 | 360×800 Chrome screenshot의 잘린 아래·오른쪽을 320×240 GPU viewport에서 읽는다. | 생성기가 CSS viewport와 screenshot 크기 양쪽을 확인하며 모든 sample 좌표가 320×240 안이다. |
| 6 | anti-alias 경계나 색상 edge의 우연한 단일 pixel을 exact sample로 쓴다. | fixture test에서 각 sample 주위 3×3 RGBA 동일성을 검사한다. 20개 sample 전부 통과. |
| 7 | overlap 장면에서 가려진 파란 아래 item이 보인다고 거짓 주장한다. | pinned Chrome은 `c1121-overlap-first`를 완전히 가린다. 해당 node는 `occludedNodeIds`로 기록하고 위쪽 빨간 item을 실제 sample로 비교했다. |
| 8 | 중첩 Grid에서 부모와 자식의 sample 좌표가 중복되거나 다른 색을 가리킨다. | ID·좌표 uniqueness를 검사한다. 중첩 장면 7개 sample 모두 두 플랫폼에서 exact. |
| 9 | node가 zero-size·hidden·viewport 밖이어도 painted item으로 취급한다. | 유한한 geometry, scene node identity, viewport 교집합, Chrome screenshot의 실제 palette pixel을 검사한다. 대응 sample을 못 찾으면 생성 실패. |
| 10 | Android 로그의 RGBA만 믿고 PixelCopy PNG 자체는 확인하지 않는다. | PixelCopy PNG SHA-256과 PNG 내 실제 픽셀을 로그·고정 Chrome RGBA와 각각 대조한다. Android 20/20 exact. |
| 11 | Android density나 PixelCopy 좌표 환산 오류가 같은 좌표로 우연히 통과한다. | begin/capture density·size·generation을 묶고 CSS center 좌표의 device pixel 변환을 독립 대조했다. |
| 12 | 이전 scene의 PixelCopy 파일 또는 다른 process marker를 현재 run으로 재사용한다. | scene별 새 process와 run ID를 요구하고 PNG cache 경로·PID·scene·generation·sample 집합을 검증한다. |
| 13 | iOS Simulator screenshot을 Metal surface readback으로 바꿔 부른다. | 비교값은 WGPU draw report의 Metal drawable texture에서 읽었다. Simulator screenshot은 시각 증거로만 보존한다. iOS 20/20 exact. |
| 14 | drawable 크기나 scale 불일치 상태에서 iOS의 인접 pixel을 읽는다. | viewport·drawable 크기, x/y scale, CSS→device 좌표를 검증하고 모든 sample이 drawable 내부에 있는지 확인한다. |
| 15 | surface 재생성이나 다른 render revision 뒤의 값을 현재 frame 표본으로 채택한다. | iOS summary/begin의 PID·generation·document/render/environment revision을 같은 run 안에서 대조한다. |
| 16 | 6개 성공 scene만 실행하고 나머지 4개를 누락해도 전체 행렬이 통과한다. | 실행기와 summary가 10 scene, 성공 6, 기대 거부 4, pixel scene 6, 표본 20개를 요구한다. 두 플랫폼 각각 10/10 실행. |
| 17 | 실패 scene을 거부로 부르기만 하고 partial frame이나 non-empty present를 허용한다. | runtime comparator가 정확한 오류 code·NodeId·property, frame 0, positive draw 0을 검사했다. 거부 4/4 통과. |
| 18 | 지원 밖 scene이나 scene/sample ID 불일치로 표면 capture를 시작한다. | Android는 scene allowlist와 C11/C11.2.1 scene 일치를 요구하고 PixelCopy 미시작 negative control을 통과했다. iOS parser도 allowlist 밖 scene의 Metal readback을 거부했다. |
| 19 | sample 누락·중복, 다른 run, 잘못된 RGBA 한 채널을 성공 summary에 합친다. | 기존 Android PixelCopy·iOS Metal comparator negative control 전체와 새 fixture mutation test 9개가 통과했다. matrix summary는 비교 6개와 표본 20개가 모두 있어야 작성된다. |
| 20 | AVD/Simulator 결과를 실기기·compositor 최종 표시·WPT 적합성으로 과장한다. | Android device report는 ANGLE/Vulkan/SwiftShader AVD를 기록한다. iOS는 compositor 전 drawable이다. Android 실기기, iOS 실기기, WPT와 전체 CSS 지원은 미완료로 남겼다. |

두 플랫폼 모두 동일한 고정 fixture에서 성공 장면 geometry 최대 오차 0 CSS px, surface 표본 최대 채널 오차 0으로 실행됐다. 전체 `bun run test`는 CSS reference 336/336, Android touch tools 29/29, Rust workspace 618 passed·4 ignored, release depth test 1/1이었다. `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`도 통과했다.

## 미확정 경계

Android AVD는 소프트웨어 renderer다. Android 실기기 full-scene surface PixelCopy, iOS 실기기, compositor 후 최종 표시, Spinon WPT 적합성은 이 구현 검토로 완료 처리하지 않는다.
