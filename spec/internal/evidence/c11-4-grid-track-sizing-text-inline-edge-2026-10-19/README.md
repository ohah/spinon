# C11.4 · inline text edge 사전 oracle

base plan의 Grid item `padding`·`border` case와 C15 inline span decoration은 서로 다른 측정 경계다. 이 fixture는 `<span>`의 inline padding/border를 min-content line contribution에 넣는 방법을 정하기 위해 추가했다.

## 기준 입력과 환경

- Fixture: [`grid-track-sizing-c114-inline-span-edges.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-inline-span-edges.html)
- Font: [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Browser: Agent Browser에서 실행한 `HeadlessChrome/155.0.0.0`, macOS user-agent, DPR 1, viewport `1280×633`
- `document.fonts.check("10px Ahem")`: `true`
- 각 Grid는 `width:120px`, `grid-template-columns:min-content`를 사용한다. 텍스트는 Ahem `10px/10px`이며 span은 양쪽에 각각 `4px` padding과 `1px` border가 있다.

## 관찰값

| 입력 | min-content column | row height |
| --- | ---: | ---: |
| normal · `XX XXXXXXX` | 75px | 20px |
| nowrap · `XX XXXXXXX` | 110px | 10px |
| break-all · `XXXXXXXX` | 15px | 80px |
| pre-line · `XX\nXXXX` | 45px | 20px |

inline edge는 모든 텍스트 폭에 한 번에 더해지지 않는다. normal·break-all·pre-line에서는 시작 edge가 첫 content line, 끝 edge가 마지막 content line의 contribution을 늘린다. nowrap은 두 edge가 같은 line에 있어 둘 다 한 번씩 기여한다. 고정 browser 출력은 [`chrome155-observation.json`](./chrome155-observation.json)에 있으며 SHA-256은 `3e034385f2abac97fe470be596988e88c56f815d175b4d4cc36370809301ae08`이다. fixture SHA-256은 `1dfb6c4f1eb6c4309daf97c1bdc416214874f2f44e337e88293cb13167eeed9e`이다. 전체 파일 manifest는 [`SHA256SUMS`](./SHA256SUMS)에 있다.

이 자료는 Chrome geometry oracle이다. Spinon layout·renderer·native surface 결과를 뜻하지 않는다. Agent Browser session을 `close`했고 종료 확인에서 `active=false`, `pid=null`이었다. 관찰용 localhost server도 종료했다.
