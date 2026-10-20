# C11.4 직접 이미지 multi-span Chrome 사전 비교

## 입력과 환경

- 입력: [`grid-track-sizing-c114-image-multispan-precomparison.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-image-multispan-precomparison.html)
- 기준: Agent Browser에서 띄운 Google Chrome `155.0.8059.85`, macOS ARM, DPR 1
- 실행 결과: [padding-root Chrome 관찰 JSON](./chrome155-observation-padding-root.json) · [Agent Browser screenshot](./chrome155-agent-browser-padding-root.png)
- 입력 SHA-256: `29e01c88179e6f0bd3f0bb10f4c3e07ae813c3f0cbce4cea98ffeaaef9162ff9`
- 관찰 SHA-256: `21ad9bd50f6c02776def4955f0964ac8e473f8480ef0ec53dd95d698c448628b`
- 화면 SHA-256: `13293ef295b27a6ac303788f78357a49c937c90858b650718707ea9d8a6292a3`

이미지 item의 `justify-self`와 `align-self`를 `auto`로 두고 다시 캡처했다. root는 1px padding을 사용해 초기 border 입력과 같은 38×58 content box를 유지한다. 네 장면의 computed tracks와 intrinsic frame은 border 입력 관찰과 동일했다. 앱 런타임은 item의 `start` 명시값을 거부하므로 앱과 oracle 입력을 모두 `auto`로 맞췄다.

## Chrome 관찰

네 장면 모두 PNG decode 후 natural size 24×16px임을 확인했다. `height` 16/32/48px와 `width:auto`의 used image width는 24/48/72px이다.

| Scene | `gridTemplateColumns` | root frame | item frame (root-local coordinates) |
| --- | --- | --- | --- |
| `image-span2-forward` | `12px 24px 24px 0px` | 40×60 | 24×16 at (1,1); 48×32 at (13,21) |
| `image-span2-reversed` | `12px 24px 24px 0px` | 40×60 | 같은 두 frame, DOM 순서만 역전 |
| `image-span2-before-span3` | `16px 28px 28px 0px` | 40×60 | 24×16 at (1,1); 48×32 at (17,21); 72×48 at (1,41) |
| `image-span3-before-span2` | `16px 28px 28px 0px` | 40×60 | 같은 세 frame, DOM 순서만 역전 |

Root width 40px보다 intrinsic track 합계가 커 두 span-2 item과 span-3 item이 root 오른쪽으로 overflow한다. span-3 item은 60px root 높이보다 아래로도 이어진다. DOM source order가 바뀌어도 computed tracks와 item geometry는 바뀌지 않았다.

입력은 40×60 border-box에 1px padding을 사용해 content box를 38×58로 유지한다. 초기 실기기 PixelCopy에서 CSS border 크기는 layout offset에 반영됐으나 green border 색은 SurfaceView에 그려지지 않아, border paint는 이번 slice에서 주장하지 않는다. Native sample은 root interior/exterior와 root 밖으로 넘친 item을 검증한다.

## 규범 계산 분리

CSS Grid §6.6의 replaced item size suggestions와 §11.5.1의 span distribution을 구분한다. spanned-item group에서는 각 item의 현재 spanned track size 합을 뺀 extra space를 track별로 계산하고, 각 track에는 그 group에서 얻은 최대 planned increase를 적용한다. 따라서 span-2 그룹의 두 기여 24px·48px은 같은 초기 track 배열에서 각각 `[12,12,0,0]` 및 `[0,24,24,0]`의 계획 증가를 만든다. track별 최대 증가 `[12,24,24,0]`을 적용한 뒤 72px span-3 기여를 나누면 각 track에 4px씩 늘어 `[16,28,28,0]`이 된다. 이 계산은 Chrome 관찰표와 따로 보관한다. 자세한 계획 검토는 [20개 실패 관점](./plan-review-20.md)에 기록했다.

이 사전 비교는 고정 Chrome browser의 computed track/frame 관찰이며 Spinon 구현·앱 runtime·WPT 결과가 아니다.
