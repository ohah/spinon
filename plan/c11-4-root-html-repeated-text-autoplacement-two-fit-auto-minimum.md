# C11.4 · 두 fit-content 행 자동 배치의 min-height:auto

- **상태:** Chrome 157 사전 비교 2,880/2,880 · 계획 실패 관점 20개 검토 완료 · 구현 전 · 공식 78/214 유지
- **선행 자동 배치:** [두 fit-content 행](./c11-4-root-html-repeated-text-autoplacement-two-fit-content.md) · 구현 PR [#1314](https://github.com/ohah/spinon/pull/1314)
- **선행 row-span:** [row-span 두 fit-content 행](./c11-4-root-html-repeated-text-row-span-two-fit-content.md) · 구현 PR [#1311](https://github.com/ohah/spinon/pull/1311)
- **Chrome 사전 비교:** [2,880 scene, min-height:0 기준과 자동 최소 크기 관계](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-min-precomparison-2026-11-10/README.md)
- **계획 실패 관점:** [서로 다른 20개](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-plan-review-20.md)
- **계획 PR diff 적대 검토:** [서로 다른 30개](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-pr-review-30.md)

## 결손과 제한 목표

기존 두-fit auto-placement profile은 여덟 direct text item 모두 min-height:0인 경우만 허용한다. 두-fit row-span profile에는 min-height:auto와 min-content floor 확인이 있지만, 같은 스타일을 auto-placement 경로에 적용하면 projection이 거부한다.

이번 bounded slice는 기존 두-fit auto-placement 입력을 그대로 두고 여덟 text item의 min-height만 auto로 허용한다. fit-content() 행의 automatic minimum이 cap보다 큰 content-based minimum을 지키는지, 기존 min-content/max-content/auto 행과 stretch 동작을 포함해 검증한다.

## 고정 입력과 Chrome 기준

- 기본 parsed Root HTML route와 기존 9 direct Grid item profile을 사용한다. Grid는 definite 40×220px, 40px explicit column, 10px explicit row, zero gaps, row auto-flow, LTR, visible overflow다.
- explicit row에는 빈 control item 하나가 오고 뒤이어 제한 ASCII 대문자 text payload를 가진 auto-placement item 8개가 온다. Payload 줄 수는 [2,2,2,3,2,2,3,1]; font와 line-height는 10px monospace/10px다.
- grid-auto-rows는 정확히 여섯 member다: fit-content(cap) ×2, min-content ×2, max-content ×1, auto ×1. Fit 위치 1–6의 서로 다른 15쌍, 유한하고 0 이상인 대표 px/% cap assignment 8개, 남은 member의 12개 순서를 다룬다. 대표 cap pair는 0px/0%, 10px/20px, 10px/20%, 20%/120%이며 각 역순 assignment도 관측한다.
- 모든 auto-placement text item은 min-height:auto; 다른 item style, content, topology, track list와 cap 입력은 기존 min-height:0 기준에서 바뀌지 않는다. 0px와 0% typed cap, 직접 percentage와 definite 220px block basis를 계속 구별한다.
- CSS Grid [§6.6](https://www.w3.org/TR/css-grid-1/#min-size-auto)에 따라 scrollable overflow가 아니고 item이 해당 축의 auto minimum track 하나를 포함할 때 content-based automatic minimum을 검증한다. 같은 절에 따라 fit-content() limit은 그 최소 크기를 깎지 않는다. 이 범위를 임의 track list나 일반 CSS admission으로 확장하지 않는다.
- Chrome 157 Agent Browser 사전 비교는 15 × 8 × 12 × 2 = 2,880 scene이다. 기존 min-height:0의 2,880 scene과 scene index 및 authored input을 맞춰 비교한다. 전체 관측과 독립 검증 산출물은 precomparison evidence에 보존했다.

독립 검증은 모든 2,880 scene에서 여섯 typed/authored track member, explicit row 뒤 여덟 implicit row의 순환, item/payload frame과 text line count, computed min-height, fit-row content floor를 검사했다. 1,440개의 고유 list 각각에서 start 총 높이는 180px, stretch 총 높이는 220px였다. 40px 추가 공간은 반복 등장하는 auto row occurrence에만 균등하게 배분됐고 다른 행 변화는 0px였다. 기존 0px minimum 기준에서 content floor 아래였던 start-aligned fit-row occurrence 1,920개는 auto 기준에서 floor를 지켰다. 이는 Chrome oracle 결과이지 Spinon 구현 증거가 아니다.

## 허용 및 거부 경계

- 9-item 두-fit auto-placement text profile에서만 direct text item의 min-height:auto를 허용한다. 기존 min-height:0은 그대로 허용한다.
- item은 기존 단일 direct payload div, fixed 10px monospace line-height, ASCII content grammar, zero margin/padding/border, static positioning, no aspect ratio, visible overflow 조건을 유지한다.
- 여섯-member auto-row list, fit 두 개의 상이한 authored position, 정확한 tail multiplicity/order, cap와 definite percentage basis, 단일 column, explicit empty-row control, auto placement와 start/stretch만 허용한다.
- one/three fit, 중복 위치, 잘못된 multiplicity/tail, calc()·음수·비유한 cap, indefinite percentage, 다른 text/style/child topology, nested/inline/replaced content, implicit column, dense/column flow, 추가 Grid owner, unsupported alignment, stale owner/revision 또는 불완전한 track trace는 거부한다.
- 기존 one-fit auto-placement와 two-fit row-span의 min-height 경계를 각각 유지한다. Row-span topology에 auto-placement 행을 섞지 않는다. 실패 시 published layout/render revision과 frames를 부분 변경하지 않고 같은 RuntimeSession에서 유효 입력으로 회복한다.
- one-fit auto-placement의 min-height:auto는 이 slice에 포함하지 않는다. 기존 one-fit/min-height:0과 two-fit row-span/min-height:auto는 regression controls로 두며, 이 외 조합은 별도 bounded plan 전까지 거부한다.
- 일반 CSS admission, 전체 CSS Grid 적합성, WPT, native API/FFI 변경, surface pixel parity, 실기기/hardware GPU 및 C11.4/C11 완료 범위는 아니다.

## 구현 및 검증 관문

1. 구현 PR을 만들기 전 계획과 Chrome 사전 비교를 위한 별도 plan PR을 검토·병합한다. 계획 자체에는 서로 다른 실패 관점 20개를 남긴다. 계획 PR diff는 프로젝트 규칙에 따라 별도의 서로 다른 30개 관점으로 적대 검토한다.
2. Style projection은 기존 two-fit auto-placement의 정확한 track order와 caps에서만 direct text item의 min-height:auto를 추가 허용한다. One-fit auto-placement/min-height:0, one-fit auto-placement/min-height:auto rejection, two-fit row-span/min-height:auto controls를 각각 확인한다.
3. Layout sizing trace는 각 item의 automatic minimum, 대응 implicit row의 base/final size, fit cap, min-content contribution과 stretch stage를 보존한다. Cap가 content floor를 낮추지 않고 trace/owner/revision 누락은 fail-closed인지 확인한다.
4. 기본 Root HTML RuntimeSession에서 auto-minimum Chrome 2,880 scene을 scene-by-scene 대조한다. 계산 style, track order, row cycle, 9 item frame, 8 payload frame/line count, content floor, fit cap과 visible overflow를 각각 검증한다.
5. 독립 controls에는 0px/0% typed cap, cap보다 높은 content floor, 다른 유효 px/% cap, start/stretch, auto occurrence 한 번/두 번인 위치가 포함된다. Negative matrix는 1/3개 fit, duplicate position, tail/list 변화, 잘못된 min-height, invalid cap/basis, child/flow 변화, trace 오류를 포함한다. Rejection atomicity와 same-session recovery를 확인한다.
6. 기존 두-fit auto-placement min-height:0의 2,880 scene, one-fit auto-placement 1,440 scene, two-fit row-span 3,840 scene, single-fit row-span 1–3 1,920 scene 및 4–6 4,320 scene, text 810 corpus를 회귀한다. 변경 영향 crate의 focused Rust tests, scoped Clippy -D warnings, cargo fmt --all -- --check를 실행한다.
7. Android API 37.2 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 min-height:auto high-floor sentinel, zero/mixed cap, early/adjacent/separated fit-position, start/stretch를 실행해 layout-ready와 draw/present를 기록한다. Screenshot은 visual smoke로 제한한다.
8. 시작 전 Simulator, Agent Browser, 앱 프로세스와 임시 artifact 상태를 기록한다. 이번 검증에서 띄운 세션·앱·임시 simulator profile/data는 종료하고 삭제한다. 다음 slice에 사용할 공용 iPhone 17 Pro profile이나 공유 AVD는 보존한다. 앞으로 쓰지 않을 전용 임시 리소스는 현재 handle과 사용자 소유 여부를 확인한 뒤 제거한다.
9. 구현/runtime 검토는 계획 리뷰와 다른 실패 관점 20개로 수행한다. 구현 PR의 전체 변경 diff도 프로젝트 규칙에 따라 별도의 서로 다른 30개 관점에서 검토한다. 검증된 앱 screenshot이 있으면 immutable URL을 PR 본문에 올리고 merge 전후 Agent Browser로 확인한다. Terminal Browser는 사용하지 않는다.
10. PR은 Rebase and merge로 병합한다. 이후 spec/STATUS.md, 내부 계약 0059, 이 계획과 공개 roadmap을 source와 맞춘다. 공식 78/214, C11.4/C11 체크와 내부 interface 0.1.0은 범위상 유지한다. 사용자 지정 Tailnet route는 게시 source/hash를 검증한 후 동기화한다.

## 완료 판정

2,880-scene RuntimeSession 비교, content-based minimum floor와 fit cap 관계, start/stretch·row cycle·frame 독립 검증, atomic rejection/recovery, 기존 auto/span/text regressions, focused Rust/Clippy/rustfmt, Android/iOS 기본-route sentinel, 별도 implementation/runtime 실패 관점 20개와 implementation PR diff 30개 검토, merge 후 상태·spec·roadmap/Tailnet 동기화를 마친 뒤에만 이 bounded slice를 완료 처리한다. Chrome 사전 비교나 screenshot만으로 implementation 완료를 표시하지 않는다.
