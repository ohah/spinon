# C11.4 anonymous Grid border overflow 사전 비교

Chrome fixture는 [grid-track-sizing-c114-anonymous-text-multi-image-border-overflow.html](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-multi-image-border-overflow.html)이다. Agent Browser 0.31.1과 고정 Chrome for Testing `155.0.8059.85`, macOS ARM64, viewport `1280×700`, DPR 1에서 실행했다. 원본 JSON은 [chrome155-observation.json](./chrome155-observation.json), 화면은 [chrome155-agent-browser.png](./chrome155-agent-browser.png)다. 실행 후 해당 Agent Browser 세션과 Chrome 프로세스를 닫았다.

| scene | CSS used columns | root border/content width | second image | border 밖 overflow |
| --- | --- | --- | --- | --- |
| `anonymous-runs-two-images-forward` | `[12,28,20,20]` | `40px / 36px` | `(14,62) 48×32px` Grid-local | `22 CSS px` |
| `two-images-anonymous-runs-reversed` | `[12,28,20,20]` | `40px / 36px` | `(14,62) 48×32px` Grid-local | `22 CSS px` |

두 scene 모두 Ahem `10px`이 준비되고 PNG 네 decode가 `24×16`으로 완료된 뒤 관찰했다. 두 `XX` text fragment는 columns 3·4에서 각각 `20×10px`이었다. Fragment 위치는 anonymous item box frame 또는 Spinon 구현 결과로 취급하지 않는다.

독립 기대값은 CSS Grid 2025 CRD §6, §8.5, §11.4, §11.5.1에 따라 계산했다. 먼저 24px image span이 두 track에 12px씩 기여한다. span-1 text는 column 3·4를 20px로 만든다. 다음 48px image span에서 columns 2·3의 현재 32px 합에 필요한 16px을 growth-limit 순서에 따라 column 2에 더하면 최종 `[12,28,20,20]`이 된다. 규범 계산은 Chrome observation에서 역으로 만들지 않았다.

Root의 2px border는 40px border-box 안에 있어 content width가 36px이다. used track 합 80px은 content width보다 크다. image B의 오른쪽 frame edge는 root border-box right edge보다 22 CSS px 바깥이다. DPR 1 Chrome screenshot의 Grid-local `(51,71)` sample은 PNG의 불투명 `#00ff40` pixel이고 top border `(10,0)` sample은 `#f0c674`이다. 한 점만 확인하며 full PNG texel matrix는 아니다. Screenshot은 browser precomparison이며 Spinon 동작 근거가 아니다.

이 계획은 C11.4 fixture/runtime 범위를 정한다. Android/iOS 앱 실행, border PixelCopy/Metal sample, actual image texture, OS compositor 이후 표면 및 WPT는 구현 검증에서 아직 수행하지 않았다. 전체 C11.4나 공식 완료 집계는 바꾸지 않는다.

## 입력·관찰 hash

| 자료 | SHA-256 |
| --- | --- |
| Chrome fixture | `58cb85846e8b0f14610db7f6271c5cda8b67c32ad56d3ec542d22809510c258f` |
| Chrome observation JSON | `3485da4820070ade0133398f062d618a2252852cc85da67d576f1fab4a19d887` |
| Chrome Agent Browser PNG | `9379144769ac25357ffe62688f40db76b268dbecd89aa2f9a8f7bd760473f511` |
| Ahem font | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| fixed PNG asset | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
