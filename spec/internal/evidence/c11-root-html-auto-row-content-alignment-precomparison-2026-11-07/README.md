# C11.4 Root HTML implicit auto-row content alignment 사전 비교

## 고정 입력과 환경

기존 [canonical HTML](../../../../tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html)을 사용했다. SHA-256은 `44a3423cd3be20935b2436d7f9f3701e580e6d39205b445b5b2a8158ca3e7308`이다. 한 40×40px Grid에는 40px column, fixed 10px explicit row, 두 10px direct-text implicit auto rows와 0px gaps가 있다. HeadlessChrome 157.0.0.0 / Agent Browser 0.31.1, viewport 1280×633 CSS px, DPR 1에서 DOM의 inline `align-content` 값만 변경했다.

## 독립 기대값

CSS Grid 2025 CRD §§10.5, 11.8의 track sizing/alignment와 CSS Box Alignment 3 §§4.1, 4.3의 positional/distributed alignment를 적용했다. Direct text `A`와 `B`는 각각 10px block minimum contribution을 주어 initial rows `[10,10,10]px`를 만든다. 40px definite block axis의 remaining free space는 10px이다.

- `normal|stretch`: auto max tracks each grow 5px; rows `[10,15,15]`, y `[0,10,25]`.
- `start|flex-start`: unchanged rows, y `[0,10,20]`.
- `end|flex-end`: unchanged rows, y `[10,20,30]`.
- `center`: unchanged rows, y `[5,15,25]`.
- `space-between`: unchanged rows, y `[0,15,30]`.
- `space-around`: unchanged rows, y `[5/3,15,85/3]`.
- `space-evenly`: unchanged rows, y `[2.5,15,27.5]`.

유리수 기대값과 Chrome 관측은 `expected-values.json`과 `chrome-observation.json`에 저장했다. Chrome의 `space-around` item positions는 `[1.65625,14.984375,28.3125]`로 양자화됐으며 유리수 기대값과 `1/32 CSS px` 이내에서 비교한다. Browser는 관측자이며 규범 출처가 아니다.

## 화면과 한계

`chrome/align-content-start.png`과 `chrome/align-content-stretch.png`는 변경하지 않은 canonical fixture에서 computed alignment를 각각 `start`, `stretch`로 명시한 뒤 Agent Browser로 캡처했다. 첫 화면은 뒤쪽에 남는 10px free space를 보여 주고, 둘째 화면은 auto tracks가 Grid를 채우는 모습을 보여 준다. 두 파일의 SHA는 [기계 판독 관측](./chrome-observation.json)에 기록했다. Spinon 출력·WPT 근거·플랫폼 간 pixel 증거가 아니다. 이 사전 비교에서는 runtime 구현을 실행하지 않았다.
