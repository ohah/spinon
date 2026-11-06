# C11.4 `row-gap` percentage 구현·RuntimeSession 실패 관점 검토 20개

**대상:** `RootHtmlAutoRowStretchV1` 제한 profile에 percentage `row-gap`을 추가한 코드, 70-scene Chrome/독립 oracle 대조, 기본 RuntimeSession 및 Android/iOS Simulator evidence. 이 검토는 구현/runtime 경계를 확인하며 실제 PR diff 검토 30개를 대신하지 않는다.

## 검토 항목

1. **다른 Root HTML profile 오염과 기준 우회:** percentage 검사 자체는 `RootHtmlAutoRowStretchV1`에만 적용한다. 직접 typed Layout 입력도 fixed 40×40px, explicit `[40px]`/`[10px]` tracks, 2~3 visible auto-placed items를 만족해야 하며 다른 generic intrinsic-gap 경로는 유지된다. 큰 높이로 기준을 바꾸는 입력은 Layout 진입에서 거부한다. **통과.**
2. **퍼센트 기준 축:** Style projection gate는 이 profile의 고정 40×40px Grid shape와 결합해 percentage를 사용한다. 계획과 fixture 모두 definite 40px block-size를 고정한다. **통과.**
3. **typed percentage 보존:** percentage는 projection 뒤 `LayoutLengthPercentage::Percentage`로 전달되고 길이 px로 선환산하지 않는다. Layout 70-scene matrix에서 Taffy frame과 track trace로 사용값을 확인한다. **통과.**
4. **단위 경계:** 기존 fixed px `0..=7.5px` 허용과 더 넓은 finite non-negative percentage 허용이 서로 다른 match arm에 있다. **통과.**
5. **0% 기준선:** 0%는 gap 0px 독립 기대값 및 positive-space scene과 결합되며 fixed 0px 기존 동작은 남아 있다. **통과.**
6. **`normal` 계산:** Grid `row-gap:normal`은 used zero gap으로 허용하고 별도 RuntimeSession test가 15px/15px auto rows를 확인한다. **통과.**
7. **negative authored CSS 한계:** HTML `row-gap:-5%`는 현재 parser/cascade에서 fatal `OtherInvalidValue` 진단을 낸다. 이를 CSS invalid-declaration recovery parity로 주장하지 않고 runtime test와 limitation 문서에 고정했다. **범위 한계 기록.**
8. **직접 negative typed input:** projection 및 Layout helper는 음수 percentage를 거부한다. CSS parser의 authored-negative 경로와 typed boundary 검사를 혼동하지 않는다. **통과.**
9. **비유한 입력:** NaN, 양·음 infinity와 부적합 px를 projection/Layout negative controls에서 거부한다. **통과.**
10. **extent overflow:** 두 row gutters와 30px base tracks를 더한 finite `f32` 범위 guard가 있고 `row-gap` property attribution을 보존한다. `height:2e38`와 200% gap을 직접 넣는 Layout regression은 unsupported profile로 Taffy 계산 전에 거부한다. **통과.**
11. **representability 경계:** 가장 큰 representable percentage fraction을 허용하고 그 다음 `f32` 값을 거부하는 projection/Layout 경계 test가 있다. **통과.**
12. **column 축 누수:** `column-gap:0` 조건은 그대로 exact-match이며 percentage·non-zero column gap은 거부 대조다. **통과.**
13. **math 값과 원자성:** `calc(5px)`는 Layout에서 `row-gap` 오류를 내고 completed layout을 publish하지 않는다. 다음 light-mode layout 복구도 확인한다. **통과.**
14. **gutter 중복 계산:** 신규 구현은 validation만 넓히며 수동 frame offset을 더하지 않는다. Layout expected formula와 Taffy output에서 gutter가 한 번만 계산된다. **통과.**
15. **alignment 조합 누락:** 7 percentages × `normal`, `stretch`와 8개 positional/distributed values로 70 cases를 실행한다. **통과.**
16. **free-space·overflow 보존:** independent oracle은 positive, zero, negative free space를 포함한다. 120%/center에서 item frame은 root 경계로 clamp되지 않는다. **통과.**
17. **track 대 frame 오진:** matrix는 item x/y/width와 `StretchAutoTracks` 이후 non-gutter row base를 둘 다 비교한다. 고정 item height는 독립적으로 확인한다. **통과.**
18. **oracle 순환 의존:** Chrome 157 관측 JSON과 CSS Grid 계산기로 만든 기대값이 별도 파일이다. 구현 tests는 사전 고정된 기대값을 읽으며 구현 결과로 oracle을 생성하지 않는다. **통과.**
19. **기본 앱 overflow 증거 과장:** 최종 코드 diff SHA `252e392adac9a5fb392c429be626b1b7b3cf0393d55db7b8d901a7d63e45d2fa` 기준 pass03 positive-space와 pass04 120%/center overflow를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 다시 실행했다. pass04 screenshot RGB bounds에서 마지막 item이 Root bottom보다 각각 73.14/73.00 CSS px 아래 시작함을 확인했다. 이는 OS screenshot의 pixel observation이며 PixelCopy·Metal drawable readback·실기기·hardware GPU·WPT 검증이 아니다. 원래 설치 앱과 asset/build hash 복원은 두 pass에서 통과했다. **범위 기록.**
20. **상위 RuntimeSession 회귀:** 첫 broad 실행에서 기존 shape test의 “percentage row-gap 계속 거부” assertion이 새 profile admission과 충돌해 27/28이 됐다. 허용해야 하는 row-axis gap을 되돌리지 않고 percentage `column-gap` 거부 대조로 바꾸었으며, 수정 뒤 `root_html_grid_intrinsic_auto_rows` 28/28이 통과했다. **수정 후 통과.**

## 결과

이 구현/runtime review에서 찾은 stale rejection 기대값은 수정했다. Blocking finding은 남지 않았다. 현재 코드·테스트·simulator 근거는 bounded candidate를 지지하지만, PR 실제 diff review, merge, post-merge roadmap sync는 이 검토 밖에 남아 있다. C11.4와 C11은 미완료이며 공식 `78/214`를 유지한다.
