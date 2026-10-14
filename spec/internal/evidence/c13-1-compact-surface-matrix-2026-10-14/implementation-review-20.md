# C13.1 compact surface matrix 구현 실패 관점 검토 · 2026-10-14

계획 검토와 분리해 구현 코드와 실제 Android/iOS runtime 결과를 다시 살폈다. 아래 결과는 compact fixture slice에 한정하며 C13.1 전체 완료 판정은 아니다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 사용자가 안내된 canonical Android flag를 주어도 build shell이 fixture를 끌 수 있다 | Gradle과 shell 모두 `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST`를 canonical로 읽는다. 실제 canonical build가 성공했고 AVD에서 flag 전달 뒤 fixture가 실행됐다. **통과.** |
| 2 | 예전 flag 이름을 쓰는 기존 명령이 갑자기 깨진다 | Gradle과 shell의 legacy alias 경로를 확인했다. legacy-only Android build log도 성공했고 canonical/native 값으로 정규화됐다. **통과.** |
| 3 | canonical·legacy alias 값이 달라 한 계층은 켜고 다른 계층은 끈다 | Gradle과 shell 모두 서로 다른 비어 있지 않은 값을 거부한다. conflict negative control은 exit 2와 명확한 진단을 남겼다. **통과.** |
| 4 | 임의 flag 문자열이나 C04 runtime GPU 없이 fixture가 부분 활성화된다 | Gradle·shell의 `0/1` 검사와 C04 의존 조건을 확인했다. invalid-string negative control은 이번 실행에서 별도 수행하지 않았으며 코드 경계로 판정했다. **통과, 음성 빌드 실행은 미수행.** |
| 5 | fixture ABI가 기본 제품 archive/header에 새어나온다 | Rust module와 C header 선언이 전용 C13.1 fixture feature에 묶였다. 기본 archive에는 새 ABI symbol/payload가 없고 feature positive control에는 둘 다 있다. **통과.** |
| 6 | JNI 또는 Objective-C++ selector가 엉뚱한 FFI 심볼을 부르거나 조건부 선언과 구현이 어긋난다 | C header, JNI, `SpinonRunner.h/.mm`, Rust export의 이름·argument를 대조했고 Android/iOS simulator build를 통과했다. **통과.** |
| 7 | null host, null output, 0-byte output buffer에서 FFI가 역참조한다 | 새 constructor/eval의 null·capacity guard와 오류 상태를 기존 host 경계와 비교했다. C ABI guard unit test 2개가 fixture test에서 통과했다. **통과.** |
| 8 | V8 host에 `document.body`가 없어 첫 fixture append가 실패한다 | Android 최초 실패 로그에서 재현한 뒤 root를 `document.body || document`로 연결했다. 같은 source가 Chrome oracle와 두 simulator runtime에서 다시 실행됐다. **통과.** |
| 9 | Android surface 재측정 때 공통 environment가 300×100을 320×240으로 바꾼다 | 초기 SurfaceView 크기, CSS viewport getter, environment update 세 경로에 compact 전용 크기를 확인했다. 최종 AVD report와 PixelCopy 모두 300×100 CSS viewport다. **통과.** |
| 10 | iOS가 화면 제약이나 environment 입력에서 simulator 크기를 fixture viewport로 오인한다 | compact route의 canvas constraint와 runtime `viewportWidth/Height`가 모두 300×100을 사용한다. 최종 runtime report의 root/render viewport도 300×100이다. **통과.** |
| 11 | 13행 중 일부가 빠지거나 15개 grid slot을 case/node로 잘못 센다 | Chrome case ID 13개, root 포함 41 frame, node 집합과 순서를 Rust oracle에서 확인하고 Android/iOS runtime의 41 frame을 확인했다. **통과.** |
| 12 | 다른 Chrome revision이나 viewport에서 수집된 oracle을 조용히 재사용한다 | Runtime validator는 Chrome 155 버전·revision·UA, 300×100/DPR1, root 크기, 13 case ID, node/sample 수를 거부 조건으로 검사한다. 변조 viewport/revision unit mutation도 실패한다. **통과.** |
| 13 | node 개수는 맞지만 layout frame과 render box의 NodeId 집합이 다르다 | Rust validator가 중복 frame ID와 render/layout node-set 차이를 오류로 처리한다. 최종 Chrome 비교와 Android/iOS 실행에서 41 frame을 통과했다. **통과.** |
| 14 | frame geometry에 NaN·누락 값·큰 오차가 있어도 완료로 보고한다 | oracle 숫자는 finite로 제한되고 x/y/width/height 차이가 0.5 CSS px를 넘으면 중단한다. 최종 runtime frame 41개가 통과했다. **통과.** |
| 15 | sample 개수만 맞고 case/role 순서, ID, 좌표가 누락·중복되거나 surface 밖이다 | case마다 고정 role 4개, 전체 ID/coordinate 52개, viewport 범위, hit ID frame membership을 검사한다. Chrome observation test의 중복 좌표·identity mutation controls와 양 플랫폼 결과가 통과했다. **통과.** |
| 16 | overflow boundary hit이 엉뚱한 node로 가도 색상만 비슷해 통과한다 | Rust는 Chrome `elementFromPoint` ID를 frame index에 매핑하고 실제 `hit_test_css_point` NodeId 위치와 비교한다. Android/iOS 모두 52/52 identity가 일치했다. **통과.** |
| 17 | 기대 RGBA의 alpha가 빠지거나 범위 밖 값이 PixelCopy를 통과한다 | oracle JSON channel 수와 0–255 범위를 검증하고 Android는 실제 RGBA 네 channel을 exact 비교한다. API 37 AVD PixelCopy에서 52/52가 exact다. **통과.** |
| 18 | density 반올림·crop 비율·오래된 Surface capture가 표본을 다른 CSS 좌표에서 읽는다 | PixelCopy는 300×100 CSS viewport를 입력받고 surface scale, capture width/height와 52 표본 전체를 검사한다. callback 전후 surface size·renderer handle·generation이 같아야 결과를 받는다. AVD 결과는 788×263, 52/52다. **통과.** |
| 19 | parent paint 보정이 hit order/clip ancestry를 섞거나 positioned 후손을 임의 재배치한다 | 보정은 auto z-index absolute/fixed parent의 단순 static-only subtree에서만 적용한다. `visible`, `clip`, `hidden`, `clip/auto` 테스트는 parent paint order와 descendant hit order를 별도로 단언한다. positioned/z-index 후손 조합은 의도적으로 보류되며 별도 Chrome 대조가 남는다. **통과, 복합 subtree는 미지원 경계로 기록.** |
| 20 | Simulator/소프트웨어 표본을 hardware GPU, 전체 surface 또는 완료 증거로 과장한다 | Android는 API 37 ARM64 AVD ANGLE/SwiftShader의 52-point PixelCopy이고 iOS는 iPhone 17 Pro Simulator의 frame/hit 및 화면 캡처뿐이다. Metal drawable readback, 전체 byte equality, 실기기, WPT, DOM event와 C13.1 완료를 명시적으로 제외했다. **통과.** |

근거: [runtime 실행·실패 진단·화면](runtime-execution.md), [Chrome 155 observation](README.md), [기본/opt-in archive feature isolation](feature-isolation.txt), [workspace test log](workspace-test.log), [Clippy log](workspace-clippy.log). 구현 검토에서 범위를 넘어선 완료 주장은 발견하지 않았다.
