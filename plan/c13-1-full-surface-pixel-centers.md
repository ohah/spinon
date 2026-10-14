# C13.1 Android 전체 CSS surface pixel-center 대조 계획 · 2026-10-14

## 목표

[PR #161](https://github.com/ohah/spinon/pull/161)이 추가한 13-case Android AVD surface를 기존 52개 선택점에서 전체 300×100 CSS pixel-center grid로 확장한다. 실제 WGPU `SurfaceView`를 `PixelCopy`하고, Chrome 155의 고정 300×100 DPR1 PNG와 모든 30,000 CSS pixel center의 RGBA를 exact 비교한다.

이 단계는 Android API 37 ARM64 AVD의 software backend에서 관찰한 fixture 결과다. 788×263 physical `PixelCopy`의 모든 하드웨어 pixel byte equality, iOS Metal drawable readback, 제품 CSS 지원, DOM event/CSSOM, WPT 또는 C13.1 완료를 주장하지 않는다.

## 비교 oracle과 좌표 변환

- 기준은 [PR #161 Chrome 155 PNG](../spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/chrome155-surface.png)와 고정 observation JSON이다. Chrome 155.0.8059.40/revision, `300×100`, DPR1, scroll `(0,0)`, fixture source/hash를 그대로 사용한다.
- 대상은 `spinon_api37_1_compat16k`, Android API 37 / ARM64 AVD의 compact route에서 생성한 전체 `SurfaceView PixelCopy` PNG다. 매 capture는 현재 renderer handle·surface generation·size가 같은지 확인한다.
- AVD physical surface는 기존 측정값 788×263이다. CSS pixel `(x,y)` 중심은 기존 helper와 동일하게 `floor((x+0.5)×surfaceWidth/300)`, `floor((y+0.5)×surfaceHeight/100)`로 대응시킨다. x/y scale을 따로 계산하고 1.0으로 가정하지 않는다.
- 300×100 각 좌표에서 Chrome PNG와 대응 PixelCopy RGBA 네 채널을 비교한다. 판정은 30,000개 모두 exact이며 색상 오차 평균/허용치는 두지 않는다. 물리 surface의 선택하지 않은 추가 subpixel은 이 대조 범위 밖이다.
- 비교 산출물은 원본 Chrome/PixelCopy PNG와 input SHA, dimensions, x/y scale, 총 좌표 수, exact/mismatch 수, mismatch 좌표별 기대/실제 RGBA JSON을 포함한다. 별도 파생 CSS-grid PNG는 만들지 않는다. 불일치 전체 좌표는 보존하고 콘솔은 요약만 출력한다.

## 현재 발견한 실패 입력

PR #161에 보존된 Chrome PNG와 AVD PixelCopy PNG를 CSS pixel-center mapping으로 전수 대조한 사전 분석에서 30,000점 중 860점이 달랐다. 불일치는 `(200,60)`–`(263,73)`의 positioned-escape tile에만 모였고 Chrome의 box 배경/테두리 색이 AVD에서 root 배경색으로 덮였다. 52개 선택점은 이 영역의 box 배경/테두리를 포함하지 않아 누락했다. 기존 Android paint order는 absolute parent의 static-only 전체 subtree일 때만 static descendants를 parent 뒤로 이동하며, sibling positioned escape target이 있는 경우 parent 전체를 건너뛴다. 이 원인 가설을 renderer paint order와 Chrome full-surface oracle로 확인한다.

## 구현 범위

1. renderer 단위 fixture에서 `relative`/`absolute`/`fixed` auto `z-index` parent, in-flow static box, positioned sibling escape target의 paint order를 분리해 검증한다. static child는 parent 배경 뒤에 오고 escaped positioned target은 기존 stacking phase·tree order를 유지해야 한다. hit order와 clip ancestry는 독립적으로 확인한다.
2. stacking-order 보정은 단순 static-only 전체 subtree 조건 때문에 놓친 in-flow static descendant를 처리하되, positioned 또는 별도 stacking context descendant를 parent의 static group으로 끌어들이지 않는다. visible/clip/hidden/mixed clip 회귀와 existing C13.1 matrix를 유지한다.
3. PNG comparator와 단위 mutation controls를 추가한다. Chrome/PixelCopy 크기 불일치, invalid/duplicate mapping, 1개 이상 RGBA 차이, alpha 차이에서 실패해야 한다.
4. 실제 Android API 37 AVD에서 앱 route를 다시 실행해 41/41 frame, 52/52 hit identity, 52/52 고정 sample, 그리고 30,000/30,000 CSS pixel-center RGBA exact를 확인한다. Android 전체 화면과 raw PixelCopy PNG/hash를 보존한다.
5. iPhone 17 Pro / iOS 26.2 Simulator에서 동일 renderer 변경 뒤 41/41 frame·52/52 hit identity와 화면을 회귀 확인한다. drawable readback은 이 계획에 포함하지 않는다.
6. state ledger·C13 plan·C13.1 internal contract에 실제 관찰 범위만 갱신한다. C13.1 전체는 미완료로 유지한다.

## 검증 및 통과 기준

- Chrome capture는 agent-browser만 사용한다. 기존 pinned PNG/observation을 재사용하고 재수집이 필요하면 source/version/hash를 다시 고정한다.
- 관련 style-to-render paint/hit tests, PNG comparator tests, 기존 Chrome observation tests, Rust workspace tests, Clippy, rustfmt, Android build/AVD 실행, iOS Simulator build/실행을 수행한다.
- 30,000개 좌표 exact RGBA 0 mismatch와 별도 negative mutation controls가 모두 통과해야 이 slice를 표면 결과 통과로 기록한다. 고정 52-point 성공만으로 전체 grid를 통과 처리하지 않는다.
- Android AVD는 ANGLE/Vulkan/SwiftShader software backend다. iOS는 Simulator 화면 및 runtime query만이다. hardware GPU·실기기·Metal drawable readback·WPT·event/CSSOM scroll·float parity는 미실행 또는 별도 gate다.

## 제외

전체 physical-pixel 788×263 byte parity, 세로/가로·density 변형 전체, iOS drawable readback, 일반 사용자 CSS 문법, scroll state/DOM event/CSSOM, C13.1 WPT와 C13.1 완료 판정은 다루지 않는다.

## 실행 결과 · 2026-10-14

- 수정 전 비교는 30,000 CSS pixel center 중 860 mismatch를 재현했다. 위치는 `(200,60)`–`(263,73)`이고 Chrome box background 719점과 gold border 141점이 Android에서 root background로 덮였다. [수정 전 JSON과 PixelCopy](../spec/internal/evidence/c13-1-full-surface-pixel-centers-2026-10-14/README.md).
- renderer paint-order 수정 뒤 Android API 37 ARM64 AVD PixelCopy에서 30,000/30,000 CSS center와 RGBA 120,000/120,000 channel exact, mismatch 0을 확인했다. x/y scale은 `2.6266666666666665` 및 `2.63`이다.
- Android AVD에서 41/41 frame, 52/52 hit identity, 52/52 고정 PixelCopy point도 통과했다. iPhone 17 Pro / iOS 26.2 Simulator는 41/41 frame, 52/52 hit query와 화면을 확인했다. Metal drawable readback은 없다.
- Node reference/비교기 tests 8/8, workspace 779/779 (4 ignored), opt-in FFI oracle tests 2/2, Clippy, rustfmt, Android/iOS Simulator builds가 통과했다. 적대 점검에서 `relative; z-index:auto`의 static 자식 paint 순서도 재현·수정했다. 구현 실패 관점 20개와 PR delta 관점 30개는 별도 evidence 파일에 둔다.
- [실행 근거·화면·원본 로그·PNG](../spec/internal/evidence/c13-1-full-surface-pixel-centers-2026-10-14/README.md). 이 결과는 AVD software backend의 30,000 CSS pixel-center 범위다. 788×263 전체 physical pixel bytes, 실기기/hardware GPU, drawable readback, event/CSSOM, WPT와 C13.1 완료는 별도 gate다.
- 구현 후 별도 [20개 실패 관점 검토](../spec/internal/evidence/c13-1-full-surface-pixel-centers-2026-10-14/implementation-review-20.md)와 [PR delta 30개 검토](../spec/internal/evidence/c13-1-full-surface-pixel-centers-2026-10-14/pr-delta-review-30.md)를 보존한다. PR 게시 후 화면 URL과 CI/mergeability 검토 결과를 해당 기록에 반영한다.
