# C10.3.3 Flex Box Alignment 사전 비교

**대상:** [C10.3.3 계획](../../../plan/c10-3-flex-order-alignment.md) · Chrome 기준 fixture와 독립 geometry 기대값
**판정:** 이 문서는 구현 전에 수행한 첫 비교 기록이다. 당시 fixture는 30개 case·84개 node였다. overflow cross-axis auto-margin 경계를 보강해 현재 fixture/reference는 50개 case·134개 node로 확장했다. 최신 입력과 digest는 아래에 갱신했으며 추가 결과 및 구현 판정은 [구현 검토](./c10-3-3-flex-alignment-implementation-review-2026-10-11.md)와 [계약 0052](../0052-c10-3-3-flex-box-alignment.md)를 기준으로 한다.

## 고정 환경

- Oracle: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`.
- viewport: `320×240` CSS px, DPR `1`·`2`, locale `en-US`, timezone `UTC`, light scheme, no forced colors, coarse pointer/no hover, `horizontal-tb`, LTR.
- fixture/inventory: `flex-alignment.html`, `flex-alignment-inventory.json`; 현재 최종 reference는 50 case·134 unique node다. Reference는 computed longhand와 node별 `x/y/width/height`를 기록한다.
- 기준 허용치: 구현 비교 시 각 node·각 geometry field의 최대 절대 오차 `0.5 CSS px`. 평균으로 단일 실패를 상쇄하지 않는다. 이 사전 비교에서는 Chrome의 DPR 1·2 결과가 CSS px 단위로 정확히 일치하는지도 확인했다.
- WPT revision `d5a765f1089ce6d3f72300281481edf3dddff7f3`에서 아래 Flexbox test 경로가 존재하는 것을 확인했다. 경로 존재 여부만 확인했으며 WPT 테스트를 실행하거나 WPT 적합성을 주장하지 않는다.
  - [`align-content-001.htm`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/align-content-001.htm)
  - [`align-content-wrap-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/align-content-wrap-001.html)
  - [`align-items-006.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/align-items-006.html)
  - [`align-self-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/align-self-001.html)
  - [`align-self-013.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/align-self-013.html)

## Chromium에서 고정한 동작

- `align-items: normal` computed string은 `normal`이고 fixed-size item의 시작 좌표는 `(0,0)`이다. `align-self:auto` computed string은 `auto`다.
- LTR row에서 `align-self:start|self-start|flex-start`는 cross-axis `y=0`, `end|self-end|flex-end`는 `y=50`, `center`는 `y=25`다.
- cross-size가 container보다 큰 item에서 `safe center`와 `safe flex-end`는 `y=0`; `unsafe center`는 `y=-10`이다.
- cross-axis `margin-top:auto`가 있으면 `align-self:flex-end`보다 먼저 공간을 차지한다. 위쪽 auto margin 하나는 `y=80`, 위·아래 둘 다 auto는 `y=40`이다.
- `align-content:center`는 `flex-wrap:nowrap` 단일 line에서 frame을 바꾸지 않는다. `wrap`인데 실제 line이 하나인 case에서는 item이 `y=40`으로 이동한다.
- 2-line `wrap-reverse`에서 `align-content:flex-start`는 item `y=[80,60]`, `align-content:start`는 `[20,0]`이다. reverse line stacking과 writing-mode-relative `start`는 같은 값이 아니다.
- `align-content:safe center` overflow는 line `y=[0,20]`, `unsafe center`는 `[-5,15]`다. `space-between`와 `row-gap:10px` case는 item `y=[0,0,80]`이다.
- `align-content:stretch`의 두 auto-height item은 각 `50px` 높이다. 한 item의 `min-height:30px; max-height:50px; padding-top:5px; border-top:2px` case도 최종 border-box 높이 `50px`이다.
- `place-items:end center`는 computed `align-items:end`, `justify-items:center`다. Flex item frame은 `(0,50,20,10)`이라 paired `justify-items`가 main-axis 배치를 바꾸지 않는다.
- `place-self:safe end center`는 item computed `align-self:safe end`, `justify-self:center`이며 frame은 `(0,50,20,10)`이다.
- `place-content:end`는 `align-content:end`와 `justify-content:end`를 모두 설정한다. 두 value `place-content:center space-between`은 두 축의 computed longhand와 2-line frame을 각각 보존한다.
- `justify-content:normal`과 `stretch`는 single item x=`0`; row의 `left/right`는 x=`0/80`; column의 `left/right`는 y=`0/0`이다. overflow에서 `safe right`와 `unsafe right`는 x=`0/-30`이다.
- 잘못된 `align-items`, `align-content`, `justify-content` 선언은 앞선 유효 값을 바꾸지 않는다. custom property를 통한 `safe flex-end`, `space-evenly`, `safe center`는 computed string에 보존된다. Block 부모의 align 속성은 block 자식 y 순서를 바꾸지 않는다.

## 반복 방법과 산출물

```sh
mise exec -- bun run css:reference:c10-3-3-alignment
node --test tools/css-reference/c10-3-3-flex-alignment.test.mjs
```

- 비교기준 JSON: [Chrome reference](../../../tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json)
- 최종 입력 SHA-256: HTML `cd8923d6ea6d106daf427776a45df0ce942cd4306033602564824ff1598f59c9`, inventory `05e896b7a584a84742b82c07cc8265226f52b4760720158447d4883289021cf4`, runtime fixture `2d95bad3017de95daf2c8d498f668a51107dab2b1fd0d15b202b7cab16093081`, capture helper `190307e38f8277bd4bb07e69304c66123fe29077de0562e7f537d79d1931452a`.
- Reference는 Chrome 실행 파일·fixture·inventory·capture helper의 digest를 담고, 생성기는 기존 JSON을 덮어쓰지 않는다. WPT inventory는 고정 경로 연결만 기록하며 WPT 실행은 `not-run`이다.
