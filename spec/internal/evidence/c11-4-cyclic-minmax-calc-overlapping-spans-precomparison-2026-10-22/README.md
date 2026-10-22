# C11.4 cyclic `minmax(calc())` 겹치는 span 기여 사전 비교

이 사전 비교는 cyclic percentage minimum을 가진 세 explicit track에서 span-2 item 둘과 span-3 item 하나가 겹칠 때 §11.5.1의 span 그룹 분배와 source-order 독립성이 유지되는지 고정한다. 작은 definite-width control은 item text가 Grid border box 밖으로 나오는 구간도 포함한다. 계획·Chrome 관찰이며 Spinon runtime 성공을 뜻하지 않는다.

## 입력과 환경

- HTML fixture: [`grid-cyclic-minmax-calc-overlapping-spans-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-overlapping-spans-c114-precomparison.html)
- Font: 저장소 Ahem, `10px/10px`; Chrome 관찰에서 `document.fonts.status=loaded`, `document.fonts.check("10px Ahem")=true`.
- 브라우저: Agent Browser `0.31.1`, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.3–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing).
- 독립 기대값: [spec-derived-expectations.json](./spec-derived-expectations.json). 기대값은 Chrome 결과를 베끼지 않고 Ahem advance와 규범적 track 단계에서 계산했다.
- Chrome 관찰: [chrome156-observation.json](./chrome156-observation.json). 화면: [chrome-agent-browser-pass01.png](./chrome-agent-browser-pass01.png).
- Source/font/observation/screenshot/expectation SHA-256: [SHA256SUMS.txt](./SHA256SUMS.txt).
- Agent Browser 실행은 전용 session `spinon-c114-overlap-final-oracle-e00ea27987c0`, PID `48316`에서 마쳤고 해당 session을 닫은 뒤 PID 종료를 확인했다. 기존 다른 브라우저를 닫거나 전체 종료 명령을 쓰지 않았고, terminal-browser를 사용하지 않았다.

## 규범에서 독립적으로 계산한 기대값

모든 scene은 Ahem `10px/10px`, `white-space:nowrap`, 2px column gap, 2px root border를 쓴다. 세 track은 `minmax(calc(10% + 2px), max-content)`다. 각 text의 min-content와 max-content contribution은 줄바꿈이 없으므로 각각 70px, 80px, 140px이다.

CSS Grid §7.2.1에서 cyclic percentage는 Grid의 intrinsic size를 계산할 때 `auto`처럼 취급하고, 결과 Grid size로 최종 resolve한다. §11.5.1은 span 크기순으로 그룹을 처리하고 각 track의 planned increase를 분리해 item 순서 의존성을 없앤다. 두 span-2 item에서 첫 item은 `70 − 2px gutter = 68px`을 두 track에 나눠 track 1·2에 34px씩 계획한다. 두 번째는 `80 − 2px gutter = 78px`을 나눠 track 2·3에 39px씩 계획한다. 그룹의 track별 최대 planned increase는 `[34,39,39]`이다.

그 다음 span-3 item의 140px contribution에서 현재 track 합 112px과 내부 gutter 4px을 빼면 24px이 남는다. 이를 세 track에 8px씩 나누면 max-content growth limit은 `[42,47,47]`, 합과 gutter를 더한 intrinsic content width는 140px이다. cyclic root에서 final minimum은 `10% × 140 + 2 = 16px`로 resolve되어 각 track의 max-content upper size보다 작다. 따라서 final Columns도 `[42,47,47]`이다. 같은 140px definite control은 같은 결과를 가져야 한다.

100px definite control에서는 minimum이 12px이다. track sizing의 남은 60px을 세 track에 균등하게 나누므로 Columns는 `[32,32,32]`다. span-3 item box는 100px이지만 nowrap text는 140px이며 content box를 40px 넘는다. root border box의 오른쪽 끝은 root 기준 x=104이고 text 끝은 x=142다. planned native surface 표본 `(126,7)`은 root 밖 Ahem text `#e5484d`, `(150,7)`은 text 다음 host clear를 가리킨다. CSS border stroke는 이 fixture 결과로 주장하지 않는다.

| scene | Columns | root border box | span-2 left frame | span-2 right frame | span-3 frame / text |
|---|---:|---:|---:|---:|---:|
| cyclic forward/reverse | `[42,47,47]` | `144×14px` | `(2,2) 91×10px` | `(46,2) 96×10px` | `(2,2) 140×10px` |
| definite 140px forward/reverse | `[42,47,47]` | `144×14px` | `(2,2) 91×10px` | `(46,2) 96×10px` | `(2,2) 140×10px` |
| definite 100px forward/reverse | `[32,32,32]` | `104×14px` | `(2,2) 66×10px`, text 70px | `(36,2) 66×10px`, text 80px | `(2,2) 100×10px`, text 140px |

Chrome 156의 여섯 장면에서 이 배열·frame이 독립 기대값과 일치했다. forward는 span-2 left → span-2 right → span-3, reverse는 반대 순서다. 관찰 JSON에는 DOM 순서 원본과 browser document-y 좌표가 보존되어 있고, 후속 runtime 비교는 root-relative frame을 사용한다.

## 구현 계획과 범위

계획은 정확히 여섯 scene ID와 세 direct text item의 위 grid shape만 기존 C11.4 opt-in fixture-test 경로에 허용한다. style projection에서 세 identical calc-minimum track, fixed 10px row, 2px gap, numeric explicit column spans를 검사한다. 미등록 scene·DOM shape·track list·span·writing mode는 fail-closed 상태를 유지한다. 계산기나 일반 CSS admission을 바꾸지 않는다. generic Taffy 경로가 규범 결과와 다를 때만 root cause를 분리해 수정한다.

구현 판정은 Chrome·독립 기대값 여섯 scene, forward/reverse 순서 일치, typed used track list와 root/item frame, three span contributions와 stage trace, definite 100px text overflow, default/fixture FFI artifact 분리, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 각 6/6 장면이다. 100px control은 Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 root-relative `(126,7)` red text 및 `(150,7)` host clear를 표본화한다. border stroke·WPT·실기기·hardware GPU·iOS compositor 이후 pixels는 범위 밖이다.

독립 구현/runtime 실패 관점 20개, 별도 PR diff 실패 관점 20개를 수행한다. C11.4 checkbox와 공식 완료 수 `77/213`은 그대로 둔다.
