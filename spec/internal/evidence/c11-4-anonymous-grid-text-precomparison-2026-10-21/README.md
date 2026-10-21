# C11.4 anonymous Grid text item Chrome 155 사전 비교

## 목적과 oracle

CSS Grid Layout Module Level 1 §6은 Grid container의 직접 text sequence를 anonymous block Grid item으로 만들고, collapsible whitespace-only sequence를 item으로 렌더링하지 않는다. 익명 item은 placement 속성을 직접 지정할 수 없으며 텍스트는 Grid container의 font style을 상속한다. 이 사전 비교는 해당 직접 자식 생성·자동 배치 입력만 고정한다. [CSS Grid §6](https://www.w3.org/TR/css-grid-1/#grid-items).

Agent Browser Chrome `155.0.8059.85`, macOS ARM, `devicePixelRatio=1`에서 [fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-precomparison.html)를 실행했다. `Ahem.ttf`가 정상 로드됐고 정확한 case 입력·DOM child 경계·columns/rows·Range text fragment·element box frame을 [기본 네 case JSON](./chrome155-observation.json)에 보존했다. 이후 `display:none` 경계를 추가한 고정 입력을 다시 실행해 [다섯 case JSON](./chrome155-observation-display-none.json)과 [추가 Chrome 화면](./chrome155-anonymous-grid-text-display-none.png)을 남겼다. 새 화면에서 숨겨진 span은 0×0이고 양쪽 `XX` fragment는 한 줄의 연속 text sequence로 배치된다. Agent Browser 프로세스는 캡처 후 종료했다. 관찰은 Chrome 기준이며 Spinon layout/runtime 또는 WPT 실행 결과가 아니다.

기록된 viewport는 `1280×633` CSS px다. 실제 Chrome user agent와 플랫폼 문자열도 관찰 JSON에 남겼다.

## 고정 관찰값

| case | used tracks | grid-local text/element 관찰 |
| --- | --- | --- |
| `anonymous-min-content-wrap` | columns `20px 100px`, rows `20px` | `XX XX` 한 text sequence가 같은 x에서 `20×10px`인 두 Range fragment를 만든다. |
| `anonymous-comment-continuation` | columns `40px 80px`, rows `10px` | comment로 갈린 두 Text node는 같은 y에서 x `0`, `20px`에 놓여 한 inline sequence를 이룬다. |
| `anonymous-runs-around-block` | columns `20px 30px 40px`, rows `10px` | text `XX`가 x `0`, explicit block이 x `20`·`30×10px`, text `XXXX`가 x `50`·`40×10px`다. |
| `anonymous-hidden-child-continuation` | columns `40px 80px`, rows `10px` | `display:none` span frame은 0×0이고 그 앞뒤 `XX` Range fragments는 x `0`, `20px`에 같은 y로 놓인다. 숨겨진 child가 anonymous item이나 auto-placement slot을 만들지 않는다. |
| `whitespace-does-not-create-item` | columns `20px 30px 40px`, rows `10px` | collapsible whitespace Range rect는 비어 있고, 두 explicit block은 x `0`, `20px`에 있다. |

표의 x/y는 fixture 외곽 페이지 좌표에서 Grid frame 원점을 뺀 CSS px다. layout/frame 비교 허용치는 1/64 CSS px다. Range rect는 inline text fragment이며 anonymous item box나 glyph ink의 경계가 아니다.

![Agent Browser Chrome 155에서 display:none direct child 앞뒤의 text가 한 Grid item으로 이어지는 장면](./chrome155-anonymous-grid-text-display-none.png)

## 재현 입력과 무결성

- Updated Chrome fixture: `22d4d81cc4defb66ab0ae54447a4e00c687244c3033434d3c4e4aa0b9de47735`
- Ahem font: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Original four-case observation JSON: `44211bc42cc7fcf88193693f228203083c6729db1b3a52588330d1d7973a9f8e`
- Updated five-case observation JSON: `b85659ab643f07a6c8fa6bff7e9d74cdefb3e52abf0e05c6ab5898a56d987408`
- Original four-case screenshot: `516f44f8c9b9bbb0a5e8fe82cb080fb8231ad1fbbb74a74fc8b1c7c5d52452e8`
- Updated five-case screenshot: `c0fa00c0108578df4531eb31d150a587c0a193d14c1fb574c14eb2b6ea1d33c1`

고정 fixture는 Ahem `10px/10px`, `white-space:normal`, `align-items:start`, `justify-items:start`, `align-content:start`, `justify-content:start`를 사용한다. CSS track output과 Range fragment는 Chrome CSSOM/DOM API에서 읽었다. Expected values는 Spinon runtime output에서 역산하지 않았다.

## 제한

이 자료는 anonymous Grid item support, glyph paint, PixelCopy/Metal surface, Android/iOS runtime 또는 public CSS support를 증명하지 않는다. `white-space:pre`, direct inline element, nested Grid/non-leaf aggregate, `order`, dense flow, explicit placement, 다른 writing mode·font/script, WPT와 전체 C11.4는 실행하지 않았다.

## 검토

- [계획 단계 적대 검토 20개](../../../../plan/c11-4-anonymous-grid-text-plan-review-20.md)
- [계획 PR diff 적대 검토 30개](./plan-pr-diff-review-30.md)
