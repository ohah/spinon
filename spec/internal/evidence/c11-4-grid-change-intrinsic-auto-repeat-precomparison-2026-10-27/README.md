# C11.4 auto-repeat intrinsic-size 변화 사전 비교

## 범위

WPT checkout revision c999c58338ee1d223df5ad62e48be1202ced0d37의 grid-change-intrinsic-size-with-auto-repeat-tracks-001.html을 고정했다. 테스트는 같은 grid에서 item의 width를 100px, 80px, 15px, 120px로 바꾸고, max-content와 min-content 두 상태에서 auto-fill 반복 수·track 폭·item 및 absolute probe 위치가 다시 계산되는지 8회 검사한다.

Agent Browser 0.31.1이 실행한 HeadlessChrome 156에서 WPT harness는 8 Pass를 보고했다. [관측 JSON](./chrome-observation.json), [독립 기대값](./expected.json), [WPT 입력·helper hash](./source-manifest.json), [재현 검증기](./validate_precomparison.py), [max-content 100px 화면](./chrome-max-content-item-100-full.png), [min-content 15px 화면](./chrome-min-content-item-15-full.png)을 보존한다. 전체 WPT test가 Spinon에서 실행됐다는 뜻은 아니다.

## 독립 기대값

고정 CSS는 grid-template-columns: repeat(auto-fill, 20px) minmax(min-content, 40px), grid-auto-columns:20px, border 2px, top padding 10px, min-width:30px다. Grid 안쪽 width는 min-content/max-content이며 in-flow item은 grid-column:1 / -1, 높이 30px, 순서대로 네 폭을 받는다. 네 out-of-flow a1–a4는 각 authored line pair를 probe한다.

CSS Grid Level 1 §7.2.3.2의 auto-repeat 계산과 §11.3–§11.8 track sizing을 CSS Sizing Level 3의 min-content/max-content intrinsic contribution과 분리해 계산했다. 마지막 minmax(min-content,40px) track은 max-content root에서 40px, min-content root에서는 15px item의 min-content case에서 10px이다. 따라서 고정 20px repeat track 개수는 item 폭 100,80,15,120px에 대해 각각 3,2,1,4이고, root content width는 max-content에서 [100,80,60,120]px, min-content에서 [100,80,30,120]px다. 모든 root content height는 30px이고 border-box는 content에 horizontal border 4px, vertical border 14px을 더한다.

표의 track size와 frame은 Chrome 관측값을 복사하지 않고 위 규칙과 고정 입력으로 독립 계산했다. Chrome은 계산 결과를 대조하는 관찰이다. WPT helper의 pixel coordinate assertion도 같은 expected table에 포함한다. python3 validate_precomparison.py는 8개 관측 상태, track sequence, repeat count, root/item/probe frame과 WPT harness 결과를 비교한다.

## 구현 경계

이 자료는 WPT 하나의 고정 입력·oracle이다. 향후 C11.4 fixture 구현은 8개 상태 각각의 used columns, repeat count, root/item geometry, a1–a4 probe geometry와 trace를 고정한다. WPT가 쓰는 absolute probes는 track sizing input에 기여하지 않으므로, 앱 구현에서 이를 누락하거나 정적 위치 검사를 생략하지 않는다.

이 단일 WPT 입력의 성공은 auto-fill 전체 grammar, auto-fit, arbitrary script/DOM mutation, 일반 CSS admission, 전체 WPT, C11.4 또는 C11 완료를 뜻하지 않는다. C11.4 공식 체크와 완료 집계는 바꾸지 않는다.

[계획 실패 관점 20개](../../../../plan/c11-4-auto-repeat-intrinsic-recompute-plan-review-20.md) · [계획 PR 변경분 30관점 검토](./plan-pr-diff-review-30.md)
