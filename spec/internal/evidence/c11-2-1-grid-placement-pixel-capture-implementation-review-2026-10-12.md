# C11.2.1 표면 픽셀 수집 구현 검토

## 검토 결과

검토 중 여섯 개의 실패 경로를 재현해 수정했다. 첫째, iOS host 선택 우선순위가 C11.2.1 요청보다 C11.1 표면 수집 플래그를 먼저 검사했다. 둘째, C11.2.1 host가 pixel test feature에서만 readback을 켜도록 연결되지 않았다. 셋째, iOS 빌드 도구가 해당 feature를 켜지 않아 첫 실행이 계측 비활성 상태였다. 넷째, Metal summary의 필드 순서를 캡처 대기 코드가 잘못 가정해 120초 timeout 경로에 진입했다. 다섯째, 공용 비교기가 inventory `fixtureId`를 runtime `sceneId`와 같다고 오인했다. 여섯째, iOS 실행 화면은 C11.2.1인데 완료 문구를 C11.1로 표시했다. host 선택·feature gate를 보완하고 필드 순서·scene alias 회귀 검사를 추가했으며, iOS label을 profile에서 가져오도록 고친 뒤 새 빌드와 표면 readback을 재실행했다.

Rust 단위 테스트에서 실제 `RuntimeGpuHost`를 만들려 한 시도는 플랫폼 외부 테스트 프로세스에 V8 Isolate 초기화가 없어 실패했다. 그 host-construction 단위 검사는 제거했다. 대신 feature가 켜진 iOS 앱을 재빌드해 같은 host의 Metal 표본 요청이 성공하는 통합 실행으로 readback 연결을 확인했고, feature 활성 FFI test target의 compile-only 확인도 통과했다. 따라서 이 host 설정 자체의 V8 없는 Rust 단위 검증을 통과했다고 주장하지 않는다.

## 실패 관점 20개

| # | 공격 관점 | 확인·보완 |
| ---: | --- | --- |
| 1 | Chrome reference 파일을 바꾸고 과거 측정값을 기준으로 삼는다 | reference SHA-256과 고정 reference ID를 실행 전에 검증한다. |
| 2 | 기준 PNG만 바뀌고 JSON은 그대로인 경우 | screenshot SHA-256과 PNG signature·크기·표본 RGBA를 검증한다. |
| 3 | 다른 DPR 또는 viewport에서 뽑은 표본을 섞는다 | 320×240 CSS px, Chrome DPR 1, 고정 scene ID를 확인한다. |
| 4 | 모든 표본이 같은 색이라 잘못된 배치도 통과한다 | 파랑 item, 빨강 item, 배경, column gap, row gap을 구분한다. |
| 5 | CSS 좌표와 device pixel 좌표 계산에서 반올림이 달라진다 | CSS pixel 중심을 사용한 고정 floor 변환과 실제 drawable scale을 재검증한다. |
| 6 | x/y scale이 달라졌는데 비정방 변환을 허용한다 | iOS comparator는 x/y scale 동일성을 요구하고 두 값 모두 결과에 남긴다. |
| 7 | 일부 표본이 빠지거나 중복돼 평균으로 가려진다 | sample ID 집합과 개수가 고정 fixture와 정확히 같아야 한다. |
| 8 | 오래된 앱 프로세스나 과거 실행 표본이 섞인다 | run ID와 현재 앱 PID를 begin/sample/summary 전체에서 대조한다. |
| 9 | surface 재생성 전후 표본이 한 실행으로 섞인다 | begin/sample/summary의 surface generation 일치를 검사한다. |
| 10 | 새 fixture가 아니라 이전 runtime frame의 표면을 캡처하거나 C11.1 host를 고른다 | document·render-tree revision, scene, drawable viewport를 대조하고 C11.2.1 host 선택을 C11.1 표면 플래그보다 먼저 하도록 수정했다. |
| 11 | Android AVD를 실기기로 오인한다 | `ro.kernel.qemu` 및 API/model을 확인하며 결과를 AVD로 고정 표기한다. |
| 12 | Android screenshot 좌표를 PixelCopy 실제 표면과 혼동한다 | 실제 비교 입력은 SurfaceView PixelCopy PNG이고, 전체 화면 screenshot은 시각 evidence로만 둔다. |
| 13 | Android PNG가 log marker에 보고된 표면과 다르다 | PNG 크기·SHA-256·sample RGBA를 marker 및 고정 fixture에 대조한다. |
| 14 | Metal renderer 실행 사실을 display scanout 증거로 과장한다 | iOS 결과 경계를 compositor 전 drawable texture readback으로 명시한다. |
| 15 | iOS summary의 `status`와 `run_id` 순서 차이로 성공을 timeout 처리한다 | 실제 로그 순서를 반영해 완료 matcher를 수정하고 필드 순서 회귀 테스트를 추가했다. |
| 16 | inventory 이름과 런타임 scene 별칭이 달라 성공 캡처를 거부한다 | comparator가 `sceneId`를 우선하고 없으면 `fixtureId`를 쓰도록 고쳤다. |
| 17 | summary의 mismatch나 오류를 성공으로 받아들이거나 host가 readback을 비활성 상태로 둔다 | `matched`만 성공으로 인정한다. C11.2.1 host는 pixel test feature일 때만 readback을 켜고, 실제 iOS Metal 표본 실행으로 그 경로를 확인했다. |
| 18 | 잘못된 입력으로 표본을 만들거나 실행 간 표본을 재사용한다 | run ID·scene·sample schema·oracle을 검사하며 실제 Android/iOS run ID가 각각 독립적이다. |
| 19 | 화면이 다른 단계 이름을 표시해 증거와 혼동을 만든다 | C11.2.1 profile 화면에서 C11.1 footer가 나오던 문제를 수정하고 iOS 재캡처를 시각 확인했다. |
| 20 | 단일 장면 결과를 전체 Grid·CSS 완료로 승격한다 | AVD/Simulator 구분, 표면 경계, 미실행 전체 scene matrix·실기기·WPT를 기록하고 C11.2.1은 미완료로 유지한다. |

## 실행 근거

- Android API 37 AVD PixelCopy: [5/5 exact 결과](c11-2-1-grid-placement-pixels-2026-10-12/android-avd-c1121-pixels-20261012T055042071Z-cae8d9ce/README.md).
- iPhone 17 Pro/iOS 26.2 Simulator Metal drawable texture: [5/5 exact 결과](c11-2-1-grid-placement-pixels-2026-10-12/ios-simulator-c1121-pixels-20261012T054954356Z-a1434f59/README.md).
- matcher·scene alias·기존 pixel comparator 검사: `mise exec -- bun test tools/css-reference/c11-grid-placement-pixel-samples.test.mjs tools/css-reference/compare-c11-ios-metal-surface-pixels.test.mjs tools/css-reference/compare-c11-android-surface-pixel-matrix.test.mjs` — 57 통과.
- workspace 전체: [현재 소스 테스트](c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-04/full-test.log) · [Clippy](c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-04/clippy.log) · rustfmt와 `git diff --check` 통과.

기기 경계와 누락된 전체 행렬 검증이 남아 이 구현은 완료 판정을 받지 않는다.
