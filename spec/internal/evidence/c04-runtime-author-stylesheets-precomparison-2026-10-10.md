# C04.11 사전 Chromium 기준

## 고정 입력

- fixture: `C04.11-runtime-author-stylesheets-v1`
- Chromium: Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 실행 파일 SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- 환경: macOS `26.5.1` (`25F80`), arm64, Node `v26.7.0`, viewport `301×100` CSS px, scale `1`, locale `en-US`, time zone `UTC`, light, coarse pointer, no hover
- reference ID: `chromium-darwin-arm64-Chrome-154.0.8037.98-537d9bc14eb2-a5c07a922ae5-ccffd5c5fe77`
- reference SHA-256: `0f039d30e2c5517b2166d6f7eac1c6cbfd76a159672160910454bf8c77c4d94b`
- inventory SHA-256: `537d9bc14eb234c2d14423becc240532ab0249b735f203ee0bec7420eaf586e5`
- HTML SHA-256: `360f458bda07eb93add583a1af471cf996e5f09962d7191669c367915e89815a`

## 관찰값

| 노드 | display | width | height | gap | background-color | CSS px rect `(x,y,w,h)` |
|---|---:|---:|---:|---:|---|---|
| `app` | `flex` | `301px` | `100px` | `11px` | `rgb(18, 52, 86)` | `(0,0,301,100)` |
| `sheet-one` | `none` | `auto` | `auto` | — | — | `(0,0,0,0)` |
| `first` | `block` | `47px` | `14px` | — | `rgb(51, 102, 255)` | `(0,0,47,14)` |
| `sheet-two` | `none` | `auto` | `auto` | — | — | `(0,0,0,0)` |
| `second` | `block` | `43px` | `14px` | — | `rgb(51, 102, 255)` | `(58,0,43,14)` |

이 입력은 두 style 요소의 순서, type·media 표면, type/class/ID/descendant selector, author stylesheet source order, stylesheet `!important`, inline normal/important 우선순위, stylesheet custom property와 `var()`를 관찰한다. 기존 C01/C04 reference는 수정하지 않았다.

## 측정 결과와 한계

`node tools/css-reference/capture-c04-runtime-author-stylesheets.mjs`가 고정 입력에서 reference를 생성했다. `node --test tools/css-reference/c04-runtime-author-stylesheets.test.mjs`가 input·tool·Chromium hash와 사전 판정값을 통과했다. 이 기록은 Chromium 기대값만 고정한다. Spinon collector·Stylo·Taffy·GPU 결과, Android/iOS 표시, 외부 stylesheet fetch 정책의 실행 검증은 아직 아니다.
