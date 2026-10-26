# C11.4 image auto/max mixed-track border overflow 사전 비교

이 고정 fixture는 direct 24×16 PNG가 `minmax(auto,max-content) 17px` columns의 첫 track에 자동 배치될 때, 12×40 border-box Grid의 10px content width를 넘어가는지 확인한다. 이는 C11.4 전체, 일반 CSS overflow, PNG app painting 또는 WPT 적합성 근거가 아니다.

## 기준 source와 파일

- Plan source base: `origin/main` commit `29449e4f794e53255909b58ed8fb2fd4995dd5d2`.
- HTML input: [`grid-image-auto-max-border-overflow-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-image-auto-max-border-overflow-c114-precomparison.html).
- Image asset: `tests/fixtures/resources/c14-1-png-basic.png`, 24×16 RGBA8, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`.
- Chrome observation: [`chrome-agent-browser-observation.json`](./chrome-agent-browser-observation.json).
- Browser metadata: [`chrome-agent-browser-metadata.json`](./chrome-agent-browser-metadata.json) and [`capture-time-utc.txt`](./capture-time-utc.txt).
- Screenshot: [`chrome-agent-browser.png`](./chrome-agent-browser.png), 64×48 PNG, DPR 1, SHA-256 `9d77fb3d907a57347e0bc82e060869e72cb21c16e3e80f3854a6a1c35fc22364`.
- Independent expected geometry and sampled RGBA: [`spec-derived-expectations.json`](./spec-derived-expectations.json).

## 환경과 관측

Agent Browser `0.31.1`로 전용 session `spinon-c114-image-auto-max-border-overflow-20261026`에서 Chrome `156.0.8078.48` / Chromium `156.0.8078.48`, macOS `26.5.1` ARM64, DPR 1, viewport `64×48`을 사용했다. 실행 중 브라우저는 같은 session 안에서 fixture와 CSS Grid 2025 CRD를 열었다. User-Agent, full version, viewport 및 raw DOM 결과를 JSON에 저장했다.

Chrome의 DOM 결과:

- root border-box rect `(0,0,12,40)`, computed border `1px`, `clientWidth=10`.
- computed columns `24px 17px`, rows `16px`.
- PNG natural size `24×16`; item rect `(1,1,24,16)`, 오른쪽 끝 `x=25`.
- `elementFromPoint(12.5,8)`의 CSS point target은 `image`다.
- image computed overflow는 `clip/clip`; 이는 item의 own image content 값이다. Parent Grid에 clipping을 적용한 입력은 아니다.
- Chrome screenshot RGBA 좌표는 0-based bitmap pixel index다. PNG가 root border를 넘어서는 index `(12,8)`의 CSS pixel center `(12.5,8.5)`는 `[255,255,64,255]`; image 바깥 index `(26,8)` center `(26.5,8.5)`는 `[16,24,32,255]`; image가 겹치지 않는 좌·우 root border index `(0,30)`/`(11,30)` center `(0.5,30.5)`/`(11.5,30.5)`는 각각 `[34,197,94,255]`다.
- 고정 PNG 원본의 local `(11,7)`도 `[255,255,64,255]`다. 이는 browser screenshot의 page `(12,8)`과 같은 pixel이다.

전용 browser session을 닫은 뒤 session info는 `pageCount=0`, `browserLaunched=false`였다. 이어 확인한 session PID `98080`은 `ps`에 없고 `agent-browser-chrome-*` process도 없었다. 별도 PID `3348`의 agent-browser daemon은 이 전용 session과 연결된 근거가 없어 그대로 보존했다.

## 독립 계산

CSS Box Sizing에 따라 12px border-box에서 좌우 1px border를 빼면 content width는 10px다. CSS Grid §6.6의 auto minimum 조건은 충족되고, natural-size replaced item의 content-based minimum은 24px다. §11.5의 span-1 auto-min base와 max-content maximum contribution은 모두 24px다. 두 번째 fixed track 17px과 5px gutter를 합친 사용 track 너비는 46px로 content width보다 36px 크다. Root border right edge `x=12` 뒤로 item은 13px 이어진다. 이 값은 Chrome/Taffy 관측으로부터 역산하지 않았다. 상세 수치는 JSON에 있다.

규범 원문: [CSS Grid §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [CSS Grid §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [CSS Box Sizing §3](https://www.w3.org/TR/css-sizing-3/#box-sizing-property), [CSS Images §4.1](https://www.w3.org/TR/2023/CRD-css-images-3-20231218/#natural-dimensions).

## 앱 surface 표본 경계

Chrome의 `(26.5,8.5)` 표본은 문서 body background `[16,24,32,255]`다. 앱 fixture의 같은 좌표는 root CSS box 바깥이다. C11.4 fixture는 `root_html_default_render=false`로 공용 runtime GPU renderer를 사용하며, 해당 경로의 surface clear 색은 opaque black `[0,0,0,255]`다. 따라서 앱 표본은 Chrome body background와 대조하지 않고 surface clear와 대조한다. 이번 oracle 교정 뒤 iOS 첫 capture에서도 `(26.5,8.5)` Metal 표본이 실제 `[0,0,0,255]`였다. 자세한 고정 로그는 [runtime evidence README](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)에 기록한다.

## 한계

이 사전 비교의 원래 관측은 Chrome oracle의 CSSOM geometry와 screenshot이다. 이 문서에서 Stylo 및 Android를 실행하지 않았다는 표현은 사전 비교 시점에 한정한다. Android API 37 AVD 캡처는 이후 완료했고 결과는 [runtime evidence](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)에 기록했다. App fixture에서는 fixed image natural-size snapshot을 사용하지만 PNG texel은 그리지 않으므로, simulator capture에서 item 배경색 표본과 browser PNG texel 표본을 서로 다른 expected value로 기록한다. 기존 C114 runner의 `xCssPx/yCssPx`는 정수 CSS pixel index를 받는다. CSS pixel center는 `index + 0.5`이며 surface pixel은 `floor((originCssPx + index + 0.5) * scale)`로 구한다. 이 fixture의 app sample runner index는 `(12,8)`, `(26,8)`, `(0,30)`, `(11,30)`이며 CSS center는 각각 `(12.5,8.5)`, `(26.5,8.5)`, `(0.5,30.5)`, `(11.5,30.5)`다. 현재 runner origin은 `(0,0)`이다. iOS 최초 capture는 기존 `[16,24,32,255]` app 기대값으로 실패했으며 observed black surface clear `[0,0,0,255]`와 불일치했다. App expectation은 Chrome body background와 구분해 검정으로 고정한다. OS compositor, hardware GPU 및 physical device 동작은 이 자료로 추론하지 않는다.
