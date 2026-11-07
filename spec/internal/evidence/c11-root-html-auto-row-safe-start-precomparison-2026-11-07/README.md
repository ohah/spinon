# C11.4 `align-content:safe|unsafe start` 사전 비교

## 목적과 경계

C11.4 `RootHtmlAutoRowStretchV1`의 다음 alignment 후보로 computed `safe start`와 `unsafe start`를 `start` control과 비교했다. 관측은 Agent Browser 0.31.1 / HeadlessChrome 157.0.0.0, 1280×633 CSS px viewport, DPR 1에서 수행했다. Fixture SHA-256은 `e6eb051fdd1d03ca5c5cb516af24b5c5e452fc327d1ed40fdd8450e1a081378b`다.

고정 Grid는 definite 40×40px content box, 40px explicit column, 10px explicit row, 두 implicit `auto` rows, column-gap 0, 하나의 40×10px empty item과 두 direct-text item `A`/`B`로 구성했다. Positive scene은 `row-gap:0`; overflow scene은 definite 40px block-size basis에서 `calc(120% - 2px)`다.

## 기대값과 관측

`start`, `safe start`, `unsafe start` 모두 positive scene에서 rows `[10,10,10]px`, item y `[0,10,20]px`를 낸다. Overflow scene에서도 모두 시작점 정렬을 유지해 rows `[10,10,10]px`, item y `[0,56,112]px`를 낸다. Agent Browser의 computed value는 authored value를 각기 `start`, `safe start`, `unsafe start`로 보존했다.

비교기는 6/6 scene과 72/72 item-frame 좌표를 독립 기대값과 비교했다. 최대 차이는 0 CSS px이며 허용치는 1/64 CSS px다. Computed `row-gap`은 positive에서 `0px`, overflow에서 `calc(120% - 2px)`다. [관측](./chrome-observation.json), [독립 기대값](./expected-values.json), [비교 결과](./comparison.json), [재현기](./compare.py), [Agent Browser 화면](./chrome-safe-start.png)을 보존한다.

이 결과는 Chrome oracle 관측이다. Spinon 구현, runtime 동작, WPT 적합성, mobile, physical device, hardware GPU, PixelCopy/Metal readback 또는 C11.4/C11 전체 완료를 주장하지 않는다. Agent Browser는 관측 뒤 닫았고 `agent-browser`/`HeadlessChrome` 프로세스가 남지 않은 것을 확인했다.
