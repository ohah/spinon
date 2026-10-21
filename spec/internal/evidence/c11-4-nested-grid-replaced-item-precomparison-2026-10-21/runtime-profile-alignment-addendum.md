# runtime profile alignment 보충 관찰

초기 사전 비교는 nested Grid에 `justify-items:start`를 지정했다. opt-in runtime test에서 해당 값이 현재 `RuntimeBlockFlexGridTrackSizingV1` 지원 profile 밖이라 fail closed 되는 것을 확인했다. 일반 replaced item에 대한 CSS 초기값 `justify-items:normal`로 입력을 조정하고 Agent Browser / HeadlessChrome 156에서 다시 측정했다. 자식 item의 `align-self`·`justify-self`도 초기값을 사용한다.

최종 oracle 파일은 `chrome-nested-replaced-runtime-profile-oracle.html`이며 SHA-256은 `647f86aef074122f18ee3d063f77115f33ad7ac73b90124ae3940acb585195c9`다. Agent Browser DOM/CSSOM 관찰은 `chrome156-runtime-profile-alignment.json`에 보존했으며 SHA-256은 `d1cad132e2aaef820824f68873fd8fa8cccc856a94b55e0cbebece72269a2faa`다. 화면 SHA-256은 `b97133e07382458db9931de9f25da070e73a9ebab23964a80ee645ef540e8f34`다.

![HeadlessChrome 156 runtime-profile alignment observation](./chrome156-runtime-profile-alignment.png)

| 장면 | outer frame / columns | nested frame / columns | text frame | image frame / natural size |
| --- | --- | --- | --- | --- |
| span-two | `98×20`; `47/47px` | `98×20`; `70/24px` | `70×20` | `(74,0) 24×16`; `24×16` |
| one-track | `102×20`; `98/0px` | `(0,28) 98×20`; `70/24px` | `(0,28) 70×20` | `(74,28) 24×16`; `24×16` |
| definite-flex-control | `120×20`; `58/58px` | `(0,56) 120×20`; `70/24px` | `(0,56) 70×20` | `(74,56) 24×16`; `24×16` |

Ahem font loaded and all images decoded with natural size `24×16`. `justify-items` computed to `normal`; the measured geometry matches the initial oracle. This verifies the adjusted planning input in Chrome and does not replace the CSS Grid normative calculation or prove product-wide profile support. Agent Browser `session info --json` reported `active:false` after `agent-browser close`.
