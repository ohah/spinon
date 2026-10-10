# C06.4 구현 전 Chromium 비교

## 목적과 판정 범위

이 비교는 C06.4 구현 전에 브라우저의 em/rem 계산 기준을 고정한다. Spinon/Rust 구현, 모바일 runtime, 글꼴 metric 지원을 입증하지 않는다.

## 고정 입력

- Chromium: Google Chrome 154.0.8037.98, revision @b859317bf11f6be47f9b7799ec690a0a42a1fb33
- viewport: 320×800 CSS px
- DPR: 같은 page에서 1과 2를 각각 적용
- locale/timezone: en-US / UTC
- color scheme/pointer: light / coarse + no hover
- fixture: tests/fixtures/css/c06/font-relative-units.html
- inventory: tests/fixtures/css/c06/font-relative-units-inventory.json
- capture: tools/css-reference/capture-c06-font-relative-units.mjs
- 고정 결과: tests/fixtures/css/references/c06-font-relative-units-v1.json

## 관찰값

Chromium은 14개 관찰 node를 DPR 1·2에서 같은 CSS computed value와 CSS px frame으로 반환했다. 자동 검증은 모든 node의 property map과 x/y/width/height가 두 배율에서 정확히 같은지 검사한다.

| 대상 | computed value 또는 geometry |
| --- | --- |
| CSS document element html | :root의 font-size: 1.25rem → 20px |
| body | 상속 font-size 20px |
| 앱 mount subtree | 별도 font-size 10px |
| mount의 em 상속 자식 | width 2em → 20px, height 1em → 10px |
| 1.2em 부모 / 1.5em 자식 | computed font-size 12px / 18px; width 24px / 18px |
| rem-target | font-size 25px, width 40px, height 20px, margin-left 5px, padding-left 10px |
| font-size percentage | 부모 10px 기준 150% → 15px; width 1em → 15px |
| custom property | 2em → 28px; 2rem → 40px |
| spacing container | font-size 12px 기준 row-gap 3px, column-gap 6px, margin-left 6px, padding-left 3px |
| flex children | flex-basis 2em/1rem → 24px/20px |

Root의 1.25rem은 문서 루트의 font-size 계산 특례에 따라 초기 medium 16px을 기준으로 20px이 된다. 하위 mount의 10px은 rem 기준을 바꾸지 않는다. 따라서 runtime HostRoot를 문서 루트로 취급하는 방식은 이 관찰과 일치하지 않는다. 확정된 모델은 cascade 전용 합성 `<html><body>` 아래에 HostRoot mount를 두는 것이다. 합성 wrapper는 HostDocument/Taffy/GPU box가 아니므로 font-size와 inherited context만 전달하고, 무시될 html/body box·paint 입력은 오류로 거부한다. 상세 계약은 [C06.4 계획](../../../plan/c06-4-font-relative-units.md)과 [내부 계약 0040](../0040-c06-font-relative-units.md)에 있다.

flex children의 `getComputedStyle(height)`는 layout 뒤 resolved/used height를 돌려준다. 이 값은 `basis-a`에서 24px, `basis-b`에서 20px이며 각각 final rect 높이와 같다. Stylo snapshot은 cascade 단계의 `height:1rem/1em` typed value를 보존하므로 Rust 검증은 이 사전 값과 Taffy final frame을 나눠 대조한다.

## 실행

기준 생성 명령은 node tools/css-reference/capture-c06-font-relative-units.mjs다. reference 생성 당시 `node --test tools/css-reference/c06-font-relative-units.test.mjs`는 3개 통과, 0개 실패였다. 이후 같은 author CSS를 V8 runtime fixture에도 고정하는 검사를 추가해 현재 검증은 4개 테스트를 포함한다.

실행 로그는 이 결과 파일이 보존하는 Chrome binary hash·fixture hash·capture tool hash 및 DevTools observation이다. 기준 파일은 생성 후 덮어쓰지 않는다.
