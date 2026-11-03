# C11.1 `calc()` implicit-column 사전 비교

이 기록은 parsed Root HTML `RootHtmlColumnFlowV1`의 definite-width `grid-auto-columns`에서 `<length-percentage>` math track을 추가하기 위한 구현 전 기준이다. Chrome 관측값과 명세·산술에서 계산한 독립 기대값을 분리한다.

## 입력과 관측

- 고정 입력: [fixture](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-calc-auto-columns-precomparison.html)
- Browser: Agent Browser의 `HeadlessChrome/156.0.0.0`, viewport `1280×633`, DPR 1
- Grid: content box `240×63px`, explicit columns `[30,40]px`, rows `[20,40]px`, gaps `5/3px`, 12개 고정 `10×10px` item, `grid-auto-flow:column`
- Implicit pattern: `calc(10% + 5px) calc(2% + 2px)`
- 원본 computed style·DOMRect 관측: [chrome156-observation.json](./chrome156-observation.json)
- CSS 규칙 기반 독립 계산: [expected.json](./expected.json)
- negative-result, 120% 및 definite-zero-width 관측: [range-controls.json](./range-controls.json)
- 재현 스크립트: [main observation](./collect-main-observation.js), [range controls](./collect-range-controls.js); fixture를 연 뒤 `agent-browser eval --stdin`에 각각 전달한다.
- Chrome 화면: [chrome156-calc-precomparison.png](./chrome156-calc-precomparison.png)
- Source SHA-256: `3071fc655beaae8bc7d7ef176893b1bef4844d08b40b442e5091b1c958d7185d`
- Observation SHA-256: `a971af13c478f4d6123486c5a1bbd8c2129be6efcac08dc9d9be74579a9ff268`
- Independent expected SHA-256: `0766f6332293720d61d70b2e7d20f403dc78ae2513a58b566494a156ad80a713`
- Range controls SHA-256: `95d63ddbf808767f25b94ec2c6059e609212a9e89b4e86fdf7b8192d3925be6d`
- Screenshot SHA-256: `359d0a2844605286634addf90cb2d9eb1da69ebbcae21dc7a85d328e3a181a76`

## 독립 계산

CSS Grid 2025-03-26 CRD §7.6에서 `grid-auto-columns` 목록은 만들어지는 implicit tracks에 반복 적용된다. CSS Values and Units Level 4 §§10.9, 10.12에 따라 math function은 해당 값 타입이 허용되는 위치에서 쓸 수 있고, top-level 결과는 속성 범위로 clamp된다. 이 fixture는 비음수 합만 사용한다. Definite `240px` content width를 기준으로 `calc(10% + 5px)`는 `29px`, `calc(2% + 2px)`는 `6.8px`다. 두 implicit track pattern을 두 번 적용하면 used columns는 `[30,40,29,6.8,29,6.8]px`가 된다. Gaps는 percentage basis에서 빼지 않고 인접 track 시작점에 한 번씩 더한다.

range controls에서 `calc(1px - 10%)`의 negative used result는 `0px`, `calc(120% + 1px)`는 container width를 기준으로 clamp하지 않은 `289px`, definite width `0px`에서 `calc(10% + 5px)`는 `5px`다. 기존 direct `fr`과 섞은 `calc(10% + 5px) 1fr`은 `[29,43.5,29,43.5]px`를 낸다. 이 값들은 [원 관측](./range-controls.json)에 별도로 기록했다.

Chrome은 둘째/넷째 implicit track을 `6.796875px`로 나타냈다. 이는 독립 계산과 `0.003125px` 차이며 허용 오차 `0.02px`보다 작다. 관측된 item x 좌표도 독립 기대에서 최대 `0.003125px` 차이다. 이 페이지는 Chrome oracle이며 Spinon 동작이나 앱 surface parity를 증명하지 않는다.

## 브라우저 수명

관측 후 `agent-browser close --all`을 실행했다. 종료 직후 daemon 검사에서 Agent Browser/Chrome 프로세스는 남아 있지 않았다.
