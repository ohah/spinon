# C11.4 두 anonymous text run + 세 image bordered Grid 구현 실패 관점 검토 20개

**검토 대상:** 제한 runtime fixture, FFI resource attachment, Android/iOS scene gates, simulator 실행·surface 결과와 feature boundary. 계획 검토 20개와 분리해 최종 구현 및 실행 기록을 점검했다.

| # | 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 알려지지 않은 scene이 일반 runtime 동작으로 조용히 통과하는가? | Runtime fixture의 네 ID allowlist와 접두어별 fail-closed 경로를 유지했다. FFI도 unknown ID에서 source를 만들지 않는다. 통과. |
| 2 | 앱 fixture가 계획 밖 DOM topology를 받아들이는가? | 직접 자식 두 text run과 두/세 image 구조, ID 순서 및 exact style을 검사한다. 통과. |
| 3 | source order만 다른 두 scene에서 image placement나 크기도 달라지는가? | Rust·FFI·Android·iOS 기대값을 대조했다. 두 source order 모두 동일한 image geometry와 track을 갖는다. 통과. |
| 4 | span-3 image가 span-2로 축소되어 테스트되는가? | Runtime style, FFI source, Java/Swift trace expectation 모두 `column 1 / span 3`과 span `[1,2,3]`을 명시한다. 통과. |
| 5 | 세 image scene에서 node/image count가 두-image 전용으로 고정됐는가? | Scene child 수, resource IDs, draw count, box count와 pixel sample 수를 3-image case로 검사한다. 각 platform의 `runtime_image_count=3 image_draw_count=3`이다. 통과. |
| 6 | PNG natural size나 ratio가 달라져 contribution·frame이 잘못될 수 있는가? | 고정 24×16 PNG와 image 높이 16/32/48의 natural width 24/48/72를 입력 및 결과에서 비교한다. 통과. |
| 7 | 두 anonymous text run이 한 text item으로 병합되는가? | 두 `XX` text source의 node ID, source order, 20×10 intrinsic input과 각 frame을 확인한다. forward/reverse 모두 두 run이다. 통과. |
| 8 | browser 관측을 normative expected로 혼동하는가? | Chrome 156 `[16,32,24,20]`은 CSS Grid 2025 CRD 독립값 `[18,34,20,20]`과 사전 비교에서 분리했다. Simulator checks는 독립 expected를 쓴다. 통과. |
| 9 | span-3 분배가 columns 1–3 이외 track을 늘리는가? | 네 columns를 `[18,34,20,20]`로 확인하고 third track 20px 및 fourth text track 20px을 유지한다. 통과. |
| 10 | rows 또는 root의 border-box가 바뀌는가? | 모든 scene에서 rows `[60,60,60]`, root frame `40×184`, 2px border와 content width 36px을 대조한다. 통과. |
| 11 | overflow 계산이 content edge를 기준으로 잘못 측정되는가? | 이미지 C right edge x=74와 root border-box edge x=40을 비교해 34 CSS px을 확인했다. old image는 22px이다. 통과. |
| 12 | overflow sample이 root border 안쪽의 색을 읽는가? | 새 scene은 root-local `(62,128)`을 사용한다. 이 위치는 오른쪽 border edge보다 22px 바깥이고 이미지 C 내부다. 통과. |
| 13 | Android surface transform이 CSS 좌표를 잘못 변환하는가? | API 37 AVD scale 2.625에서 PixelCopy border와 overflow sample 8/8 exact를 확인했다. 통과. |
| 14 | iOS 결과를 compositor 이후 화면이라고 과장하는가? | iPhone 17 Pro drawable scale 3.0 readback만 주장하며 compositor 이전 한계를 README에 명시했다. 통과. |
| 15 | Android가 새 장면 ID를 앱 진입부에서 거부하는가? | 첫 실행에서 Java allowlist 누락을 발견해 ID를 추가했다. 수정 빌드에서 네 장면 모두 허용되고 frame/trace 검증을 통과했다. |
| 16 | iOS가 같은 장면 ID를 runtime route에서 거부하는가? | Swift allowlist와 frame/track/span checks를 확장했다. iOS build 및 네 장면 캡처가 통과했다. |
| 17 | PixelCopy sample이 실제 기대 색상과 비교되지 않는가? | 새 sample ID 누락으로 생긴 `expected=-1,-1,-1`을 Android RGB map에 등록했다. 최종 두 sample ID가 각 scene에서 exact다. |
| 18 | 기존 bordered two-image route가 회귀하는가? | 최종 동일 앱 빌드에서 old 두 장면도 Android/iOS 각각 재실행해 columns, frame, overflow sample을 통과했다. 통과. |
| 19 | fixture ABI/resource가 기본 제품 archive에 섞이는가? | 기본/opt-in macOS ARM64 static archive의 symbol, byte marker와 PNG payload를 비교했다. 기본 archive 누락, fixture positive control 존재다. 통과. |
| 20 | 제한 fixture 결과가 일반 CSS/Grid 완료를 암시하는가? | 명시적 `overflow:visible` parser 경계, 단일 PNG sample, no-WPT/no-physical/no-compositor-after/no-full-C11.4 한계를 기록하고 공식 72/213을 유지한다. 통과. |

두 플랫폼 공통 scene gate와 sample expectation의 빠진 항목은 최종 네 장면 matrix를 실행하기 전에 수정하고 다시 검증했다. 최종 결과는 Android API 37 AVD 4/4, iOS 26.2 Simulator 4/4, Android PixelCopy 8/8, iOS Metal drawable 8/8 exact다. 이 검토는 코드/runtime 범위이며 PR 전체 변경의 별도 30관점 검토는 [PR diff review](./pr-diff-review-30.md)에 둔다.
