# C11.3.3 unknown-area implicit-line PR diff 검토 · 실패 관점 30개

**대상:** C11.3.3 전용 trailing implicit named-line resolver, Grid 제한 profile, Android/iOS fixture route와 캡처, 증거·상태 문서. 각 항목은 별도 실패 조건을 확인한다. 공식 C11.3.3 완료 판정은 이 PR 범위에 포함하지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 제한 fixture 결과가 공식 C11.3.3 완료 체크나 집계를 올린다 | **PASS.** C11/C11.3.3 체크는 미완료이며 공식 완료 수는 변경하지 않는다. |
| 2 | 기본 runtime/API 경로가 새 resolver를 암묵적으로 호출한다 | **PASS.** profile과 projection은 C11.3.3 fixture-test feature에서만 활성화된다. |
| 3 | 존재하지 않는 bare ident가 기존 authored occurrence보다 우선한다 | **PASS.** authored named line 조회가 먼저이고, 없는 bare ident만 제한 resolver로 간다. |
| 4 | explicit area edge나 `-start`/`-end` alias가 새 fallback에 가려진다 | **PASS.** explicit/implicit area alias 처리가 missing-name 생성보다 먼저다. |
| 5 | named span이 반대쪽 definite line 대신 잘못된 축/방향으로 확장된다 | **PASS.** 공통 resolver가 axis별 metadata와 방향을 받고 forward shortage만 생성한다. |
| 6 | malformed template area가 새 implicit track 생성으로 cascade fallback을 우회한다 | **PASS.** malformed area fixture는 기존 fallback 결과를 유지한다. |
| 7 | 일반 numeric positive line이 새 mode로 implicit extent를 얻는다 | **PASS.** 생성된 named-line 결과가 없는 numeric extent는 계속 거부된다. |
| 8 | negative 또는 leading extent가 trailing-only 구현을 우회한다 | **PASS.** leading span과 negative placement 경계가 유지된다. |
| 9 | `grid-auto-rows:auto`가 implicit row sizing으로 수용된다 | **PASS.** root node와 `grid-auto-rows`를 지정한 오류로 거부한다. |
| 10 | `grid-auto-columns:auto`가 implicit column sizing으로 수용된다 | **PASS.** columns도 독립 검사하며 auto 값을 거부한다. |
| 11 | 한 축만 고정 px인 조합이 두 축 고정 조건을 통과한다 | **PASS.** row/column auto-track 오류를 각각 검사하고 양쪽 fixed-px positive control을 둔다. |
| 12 | line 산술 overflow나 Taffy track 상한을 넘긴 입력이 layout으로 전달된다 | **PASS.** checked conversion, 기존 line validation, `GRID_MAX_TRACKS` 경계가 유지된다. |
| 13 | implicit line을 만든 item이 sibling의 unmarked numeric extent까지 허용한다 | **PASS.** 생성 표시는 placement 결과에 한정되고 sibling extent를 별도 검증한다. |
| 14 | 새 placement mode가 기존 auto-size/minimum 제약을 다른 profile까지 바꾼다 | **PASS.** `TemplateGridImplicitTracks`에만 분기하고 다른 mode 조건을 유지한다. |
| 15 | nested/hidden node나 DOM source·paint order가 바뀐다 | **PASS.** 고정 matrix에서 nested-hidden 장면, frame ID, paint order와 draw count를 확인한다. |
| 16 | Android/iOS scene allowlist나 run ID 검증이 잘못된 route를 허용한다 | **PASS.** 고정 scene allowlist를 사용하고 미등록 scene은 runtime/draw 이전에 거부된다. |
| 17 | PixelCopy/Metal sample이 다른 scene·node·좌표·RGBA를 측정한다 | **PASS.** 기본 표본과 root 밖 item 표본을 scene별 allowlist 및 예상 픽셀로 대조한다. |
| 18 | 기존 `auto` rejection이 새 fixed-px positive scene으로 바뀌어 버린다 | **PASS.** 기존 거부 scene과 새 positive overflow scene이 별도 route·fixture다. |
| 19 | Android physical matrix가 emulator, 다른 모델 또는 API 대상을 성공으로 기록한다 | **PASS.** explicit selector, model `SM-S731N`, API 36, `qemu=false`를 확인했다. 실기기 결과는 26 scene과 overflow 3/3 sample이다. |
| 20 | 실기기 전체 화면의 알림/개인 화면이 evidence에 들어간다 | **PASS.** 잘못 캡처된 최초 PNG는 제거했고, 전체 화면 저장을 AVD에만 허용한다. 실기기는 앱 SurfaceView PixelCopy만 남긴다. |
| 21 | 연결된 기기가 없거나 여러 대일 때 capture가 임의 기기를 고른다 | **PASS.** physical selector는 명시 대상만 받아들이고 누락·중복·offline 및 AVD 대상을 거부하는 테스트가 있다. |
| 22 | 측정한 APK와 evidence에 기록한 APK hash가 다르다 | **PASS.** matrix capture는 build 직후 APK hash를 기록하고 같은 경로의 APK를 설치해 실행한다. Android hash는 `482e5b5452b3d62f8ad20f51247d6f93622b43c0d3db3ed7fa8d79e6c3122470`이다. |
| 23 | 공개 metadata가 사용자의 ADB serial을 노출한다 | **PASS.** evidence JSON의 serial 값은 `[REDACTED]`이며 capture 도구도 저장 전에 값을 가린다. model/API/renderer/hash는 남긴다. |
| 24 | 물리 기기의 알림 화면을 surface sample 이미지와 혼동한다 | **PASS.** 저장 PNG는 앱의 고정 Grid surface sample이고, 물리 기기 전체 화면 hash는 `null`이다. |
| 25 | Android software renderer와 실기기 renderer 결과를 하나의 hardware claim으로 합친다 | **PASS.** Android 17/API 37 AVD SwiftShader와 Android 16/API 36 SM-S731N Xclipse/Vulkan을 별도 실행 기록으로 나눈다. |
| 26 | iOS drawable 결과를 compositor 이후 화면이나 실기기 결과로 확대 해석한다 | **PASS.** 문서는 iPhone 17 Pro/iOS 26.2 Simulator의 Metal drawable이며 compositor 제출 전이라고 명시한다. |
| 27 | native backdrop과 Chrome viewport 색 차이가 Grid pixel 실패/성공으로 섞인다 | **PASS.** host-owned backdrop sample은 exact set에서 제외하고 known-area 및 overflow item sample을 따로 검사한다. |
| 28 | FFI archive 결과가 APK/iOS release bundle 검증으로 확대 해석된다 | **PASS.** default/opt-in 정적 archive 경계만 주장하고 최종 app bundle 검증이 아님을 기록한다. |
| 29 | 전체 bun suite의 기존 assertion 실패를 숨기거나 이번 변경 실패로 오인한다 | **PASS.** C04, C11.3.2, C13.3의 네 변경 외 assertion 실패를 기록하고 전체 suite 통과로 보고하지 않는다. |
| 30 | 구현 후보를 공개 지원이나 C11.3.3 전체 완료로 게시한다 | **PASS.** 공식 체크와 count는 유지되고 지원 한계 및 미검증 플랫폼을 문서에 남긴다. |

확인 결과, PR 범위에서 고칠 추가 결함은 없었다. 이전 계획 review와 implementation review는 이 검토 횟수에 합산하지 않았다.
