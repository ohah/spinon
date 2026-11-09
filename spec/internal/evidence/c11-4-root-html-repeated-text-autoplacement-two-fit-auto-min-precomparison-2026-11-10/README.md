# C11.4 반복 텍스트 자동 배치 두 fit-content 행의 자동 최소 크기 사전 비교

**상태:** Chrome 157 사전 비교와 독립 관계 검증 완료 · 구현 전 · 2026-11-10

## 입력과 도구

- Oracle fixture: oracle-fixture.html
- Fixture SHA-256: 34c6be405481f86ded711867918ef5ab90a13c828cb03fed1bf821f37f5a1876
- Browser: Agent Browser /opt/homebrew/bin/agent-browser, HeadlessChrome 157.0.0.0, MacIntel, DPR 1, viewport 1280×900
- Grid: definite 40×220px, 40px column 하나, 10px explicit row 하나, gaps 0, row auto-flow, align-content start 또는 stretch, LTR, visible overflow
- Children: explicit row를 차지하는 빈 control 하나와 여덟 auto-placement text item. 각 item은 단일 payload div를 가지며 줄 수는 [2,2,2,3,2,2,3,1]이다.
- grid-auto-rows: fit-content() 2개, min-content 2개, max-content 1개, auto 1개. fit 위치 15쌍, 순서 있는 px/% cap assignment 8개, 후행 member 순서 12개, alignment 2개를 비교했다.
- 차이는 여덟 auto-placement text item의 min-height뿐이다. explicit-row control의 min-height:0과 고정 10px 크기는 기준 fixture와 같다.

## 기준과 독립 판정

W3C [CSS Grid Layout §6.6 Automatic Minimum Size of Grid Items](https://www.w3.org/TR/css-grid-1/#min-size-auto)는 scrollable overflow가 아니고 item이 해당 축의 auto minimum track을 포함하면 item의 used automatic minimum size가 content-based minimum이라고 정의한다. 같은 절은 fit-content() 인수가 content-based minimum size를 고정 상한처럼 clamp하지 않는다고 명시한다.

Capture 전에 min-height:0 기준과 min-height:auto를 같은 Agent Browser 세션에서 한 장면으로 비교했다. fit-content(0px) 행은 첫 기준에서 0px, auto에서 두 줄 payload 높이인 20px였다. 비교용 본 행렬은 기존 기준과 동일한 2,880개 scene을 사용한다. verify-chrome-matrix.py는 capture 결과를 CSSOM/row 관계, payload line-height, auto-row stretch 관계 및 기존 min-height:0 observation과 독립 대조한다.

검증 결과:

- 2,880/2,880 scene, 고유 authored track list 1,440개, fit 위치 15쌍, cap assignment 8개, tail order 12개, start/stretch pair 1,440개를 확인했다.
- 모든 scene에서 계산된 min-height:auto, computed six-member track order, explicit row 뒤 여덟 implicit row의 순환 순서와 item/payload frame을 확인했다.
- start 배치의 총 row 높이는 모든 list에서 180px이고 각 implicit text row는 그 item의 [2,2,2,3,2,2,3,1] × 10px content height와 일치한다. 3,840개의 start-aligned fit row 중 기존 min-height:0 결과에서 내용 높이 아래로 줄었던 1,920개가 자동 최소 크기로 바닥을 지킨다. 자동 최소 크기 위반은 0개다.
- stretch는 180px에서 220px로 남은 40px를 더한다. auto row가 한 번 나타나는 list는 40px, 두 번 나타나는 list는 각 20px를 받는다. 비-auto 행의 변화는 0px다. auto-minimum 입력의 text overflow는 0개다.
- 독립 검증 결과는 matrix-verification.json; 전체 관측은 chrome-matrix-observation.ndjson; 수집기는 capture-chrome.mjs; 표본 이미지는 chrome157-two-fit-auto-min-sample.png다. SHA256SUMS가 이 산출물의 해시를 고정한다.

Agent Browser lifecycle은 opened true, closed true, close error 없음으로 기록됐다. 별도 Terminal Browser와 simulator는 사용하지 않았다. 이 결과는 Chrome 기준 관측이며 Spinon RuntimeSession 또는 앱 구현 증거가 아니다. 일반 CSS Grid 적합성, WPT, OS surface/pixel parity, 실기기 및 C11.4/C11 완료를 주장하지 않는다.
