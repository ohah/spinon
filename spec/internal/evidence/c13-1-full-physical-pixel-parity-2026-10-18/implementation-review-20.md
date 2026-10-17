# C13.1 전체 물리 픽셀 구현 실패 관점 검토 20개 · 2026-10-18

대상은 commit `85f289938b5445ff2187e8b93644d9fa9e3ab5af`의 capture/comparator/runtime 경로와 이 commit에서 나온 Android AVD·iOS Simulator 실행이다. 계획 문서의 20개 검토는 재사용하지 않았다.

| # | 독립 실패 관점 | 코드·실행 대조 및 판정 |
|---:|---|---|
| 1 | 다른 viewport나 oracle PNG 크기를 같은 비교로 통과 | comparator가 Chrome DPR 3 observation과 `900×300` PNG를 확인한다. 두 앱 결과 dimensions도 `900×300`이며 통과. |
| 2 | CSS-size PNG를 확대해 실제 readback처럼 제출 | Android PixelCopy와 iOS Metal RGBA 파일을 직접 비교한다. screenshot은 첨부 증거만으로 분리했다. |
| 3 | edge band나 허용 오차가 mismatch를 숨김 | channel tolerance와 crop 보정 없이 전체 byte equality, 최대 오차 0으로 세 번씩 통과했다. |
| 4 | 알파 채널 생략으로 불투명/투명 차이를 누락 | 비교 cardinality는 270,000×4=1,080,000 channel이고 alpha를 포함해 mismatch 0이다. |
| 5 | fixture border 밖 paint를 surface 비교에서 잘라냄 | `(270,72)` / physical center `(811,217)`이 전체 surface 안에 포함되며 sample metadata가 Chrome 색과 일치한다. |
| 6 | PixelCopy 대신 status bar가 든 화면 screenshot을 comparator에 전달 | runner는 `android-api37-avd-pixelcopy.png`만 비교하고 전체 Android screenshot은 별도 보존한다. 통과. |
| 7 | Android density가 목표 scale에 도달하지 못함 | 모든 final run이 `420→override 480`, `scale_x/y=3`, `900×300`을 기록한다. |
| 8 | density 조작이 physical/logical display size를 변경 | `wm size` 출력은 전·중·후 `1080×2400`; 실행기는 값 동일성을 assert하고 size를 설정하지 않는다. |
| 9 | 실패 경로에서 density override가 남음 | runner cleanup은 capture/build 이후 예외 경로에서도 이전 override 또는 reset을 실행한다. 세 성공 run은 `420` 복원을 기록했다. |
| 10 | 앱이 density 복구 중 재기동되어 capture가 계속됨 | cleanup은 force-stop하고 `pidof`가 비는지 확인한 뒤 density 복원 결과를 검증한다. 세 Android run 모두 종료 확인. |
| 11 | 지정하지 않은 두 번째 emulator나 실기기를 수정 | 실행 인자는 `emulator-5554`로 고정되고 qemu/API37/ARM64 assertion이 있다. `localhost:5555`는 조작하지 않았다. |
| 12 | 같은 앱 process 또는 stale PixelCopy 결과를 반복 재사용 | 세 run ID와 PID `9243/9447/9646`가 다르고 각 PixelCopy begin log가 PID·generation `2`를 연결한다. |
| 13 | Android 전체 surface는 exact지만 기존 center 경로가 회귀 | 각 run에서 별도 30,000 center pixels·120,000 channels, 52 fixed samples, 41 frames, 30,000 hit identities 모두 통과했다. |
| 14 | comparator가 run ID의 포맷을 앱과 다르게 강제 | 초기 Android hex ID 검사가 고정 32자라 유효한 `12`자리 run ID를 거부했다. elapsedRealtime ID 규칙으로 수정했고 세 run 모두 통과했다. |
| 15 | 새 iOS capture argument가 앱 시작 gate에서 누락 | 첫 iOS build/runtime 검토에서 argument dispatch 경계의 빠진 allowlist를 발견해 `AppDelegate` 및 route validator를 함께 수정했다. 이후 fresh-source 세 run이 통과했다. |
| 16 | iOS batch가 row-major 순서를 누락·중복·재배열 | 4,219 batch 요청과 각 64개/마지막 48개 크기, coordinate sequence 및 최종 270,000 byte order를 검사했다. 세 run 통과. |
| 17 | Metal의 256-byte row padding이 RGBA artifact에 섞임 | readback마다 네 color byte만 packed `.rgba`에 기록하며 comparator test가 padded row artifact를 거부한다. |
| 18 | BGRA/RGBA 변환이나 alpha 보정이 한 번 더 적용됨 | manifest format `Bgra8Unorm`, artifact order `RGBA8`; sRGB/premultiply correction 없이 Chrome byte와 exact다. |
| 19 | batch 중 drawable generation 또는 format이 바뀌어 다른 frame을 조합 | capture state는 첫 generation/format을 고정하고 매 report에서 재확인한다. iOS 세 run은 generation 1, `Bgra8Unorm`으로 끝났다. |
| 20 | 결과 summary가 center oracle/다른 source를 가리켜 full parity로 오인 | summary 수정 뒤 Android full comparison은 Chrome 155.0.8059.85 DPR3 oracle hash와 source commit `85f2899…`를 가리킨다. iOS도 같은 Chrome hash/source revision에 묶였다. |

마지막으로, comparator 7개 mutation/shape tests, iOS host-init 전 argument·route conflict rejection, Android density/size/process cleanup, default/opt-in archive positive control이 서로 다른 경계로 확인됐다. Simulator/AVD 결과를 C13.1 제품 지원이나 C13 완료로 승격하지 않는다.
