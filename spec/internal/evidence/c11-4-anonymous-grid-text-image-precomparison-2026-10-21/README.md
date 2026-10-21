# C11.4 anonymous Grid text와 replaced image 사전 비교

## 실행 조건

- Chrome for Testing `155.0.8059.85` (Agent Browser `navigator.userAgentData.uaFullVersion`), macOS ARM, DPR 1
- viewport `1280×577`
- Ahem `10px/10px`: `document.fonts.check('10px Ahem') = true`
- 고정 PNG decode 완료: natural size `24×16`
- 입력: [비교 fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-image-precomparison.html)
- 원본 observation: [chrome155-observation.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/chrome155-observation.json)
- 화면: [chrome155-agent-browser.png](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/chrome155-agent-browser.png)

## 규범 기준과 독립 기대값

CSS Grid §6은 in-flow 직접 자식을 Grid item으로 만들고, 연속 text sequence는 익명 block-container Grid item으로 감싼다. collapsible whitespace만 든 sequence는 item을 만들지 않는다. Grid item의 display는 blockify되므로 기본 inline인 `img`가 text run을 분리하는 직접 Grid item으로 동작한다. §11.5.1은 spanned track에 남은 기여분을 균등 분배하며 item 순서가 결과를 바꾸지 않도록 planned increase를 쓴다. 기준 문서는 [CSS Grid §6](https://www.w3.org/TR/css-grid-1/#grid-items), [§11.5.1](https://www.w3.org/TR/css-grid-1/#algo-spanning-items)이다.

Ahem에서 `X`는 10 CSS px이므로 `XX`의 min-content 기여는 20px, comment 양쪽의 `XX`를 합친 `XXXX`는 40px이다. PNG는 24×16이고 CSS `height:48px;width:auto`에서 transferred inline size가 72px이다. gap이 없는 네 개의 `min-content` 열에서 span-2 image의 72px 기여는 두 열에 36px씩 들어간다. 아래 track 값은 이 규칙에서 먼저 계산한 기대값이며 Chrome observation으로 조정하지 않았다.

| 장면 | Grid 직접 자식 순서 | 독립 track 기대값 | Chrome 155 track | grid-local image x | text Range x |
| --- | --- | --- | --- | ---: | --- |
| `anonymous-text-image-text` | `XX`, image, `XX` | `[20,36,36,20]` | `20px 36px 36px 20px` | 20 | 0, 92 |
| `image-before-comment-group` | image, `XX` + comment + `XX` | `[36,36,40,0]` | `36px 36px 40px 0px` | 0 | 72, 92 |
| `comment-group-before-image` | `XX` + comment + `XX`, image | `[40,36,36,0]` | `40px 36px 36px 0px` | 40 | 0, 20 |
| `whitespace-around-image` | collapsible whitespace, image, whitespace + `XX` | `[36,36,20,0]` | `36px 36px 20px 0px` | 0 | 72 |
| `hidden-child-around-image` | `XX`, hidden span, image, `XX` | `[20,36,36,20]` | `20px 36px 36px 20px` | 20 | 0, 92 |

Range x 좌표는 glyph fragment 관찰값이고 익명 item box 또는 glyph ink의 대체값이 아니다. 런타임 검증은 별도로 text intrinsic input의 source node 목록·anchor·owner·source order와 anonymous item frame을 읽는다.

## 입력 hash

| 입력 | SHA-256 |
| --- | --- |
| `tests/fixtures/resources/c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| `grid-track-sizing-c114-anonymous-text-image-precomparison.html` | `c4b1cc75da51498a190cac1a2d8fa5436526e6b39a876e232efff5fd735f5737` |
| `chrome155-agent-browser.png` | `445518ac96ffd1c014e33c93cd6e83c1d2ebff6fb1794455bf34503e93437aef` |

이 자료는 Chrome 기준만 고정한다. Spinon 레이아웃·런타임·Android/iOS 동작은 아직 비교하지 않았다.
