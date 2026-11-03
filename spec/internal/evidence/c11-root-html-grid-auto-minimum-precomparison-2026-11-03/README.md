# C11 parsed Root HTML automatic minimum 사전 비교

## 기준 입력

- Fixture: [root-html-grid-auto-minimum-precomparison.html](../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-precomparison.html)
- 규범에서 먼저 계산한 기대값: [expected.json](expected.json)
- Agent Browser Chrome 156 관측: [chrome-observation.json](chrome-observation.json)
- 화면: [chrome156-agent-browser.png](chrome156-agent-browser.png)
- 입력·기대값·관측·화면 SHA-256: [SHA256SUMS.txt](SHA256SUMS.txt)

## 독립 기대값

각 positive axis는 definite content size 100px이다. First item의 fixed preferred size와 min-content contribution은 80px이고 second track은 fixed 40px이다. CSS Grid automatic minimum이 first track의 base를 80px로 유지하면 `minmax(auto,1fr) 40px`의 used tracks는 `[80,40]`이고 Grid content track 합은 available 100px보다 20px 크다. Root에 2px border가 있으므로 second child는 root border 끝보다 18px 더 나온다.

`minmax(0px,1fr) 40px` 대조 장면은 auto minimum을 제거한다. 100px content axis에서 fixed 40px을 빼면 flex track은 60px이 되고 used tracks는 `[60,40]`이다. First item의 authored 80px size는 그대로 유지되지만 second child는 root border 밖으로 나가지 않는다.

경계 입력 `minmax(auto,0fr) 40px`은 Chrome 장면에 추가하지 않고 규범 계산으로 분리했다. Auto minimum으로 track base는 80px이며 `0fr`은 leftover space를 요청하지 않으므로 tracks는 `[80,40]`, second item은 root border 바깥 18px으로 예상한다. 이 값은 implementation test에서 별도로 고정하고 Chrome observation으로 표시하지 않는다.

이 계산을 columns와 rows에 각각 적용했다. Items는 default row auto-placement를 사용한다. Root/item margin, padding과 gap은 0이며 item edges와 overflow는 visible이다.

## Agent Browser 관찰

Agent Browser가 실행한 `HeadlessChrome/156.0.0.0`, viewport 1280×633 CSS px, DPR 1에서 네 장면 모두 독립 기대값과 일치했다. Columns `minmax(auto,1fr)`은 `[80,40]`, 18px right overflow이고 `minmax(0,1fr)`은 `[60,40]`, 0px overflow였다. Rows는 각각 같은 track sizes와 18px bottom overflow/0px control을 보였다. Root border-box와 모든 item frame도 observation JSON에 저장했다.

Chrome은 별도 구현 관찰값이고 기대값의 출처가 아니다. 이 고정 입력은 Android/iOS runtime이나 일반 CSS/Grid 적합성을 입증하지 않는다.
