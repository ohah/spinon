# C11.4 `align-content:safe|unsafe end` 사전 비교

## 목적과 경계

C11.4의 다음 제한 후보를 위해 고정 `RootHtmlAutoRowStretchV1`와 기존 두 항 `calc()` row-gap shape에서 `safe end`, `unsafe end`, plain `end`를 비교했다. `safe|unsafe center`는 앞선 구현 PR #1249로 병합됐으며, 이 자료는 end 정렬의 Chrome oracle만 고정한다.

- 관측자: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0
- Viewport: 1280×633 CSS px, DPR 1
- Grid: 40×40px, 40px column, 10px explicit row, 두 implicit `auto` rows, column-gap 0, fixed empty item 하나와 direct text `A`/`B`
- Positive cases: `row-gap:0`; overflow cases: definite 40px block-size basis의 `row-gap:calc(120% - 2px)`
- Fixture SHA-256: `ae2e0e0c5c4e52b871efccc4d3826d0a485aa1419b2b0f5cf6d46251746e536d`

## 독립 기대값과 관측

Positive free-space에서 정렬 subject는 30px이고 container에는 10px가 남는다. plain·safe·unsafe end는 track group을 끝에 배치해 상대 y `[10,20,30]px`를 만든다. Overflow subject는 10px track 세 개와 46px gutter 두 개를 포함해 122px다. `safe end`는 start로 fallback해 `[0,56,112]px`; `unsafe end`와 plain `end`는 end 배치를 유지해 `[-82,-26,30]px`다. 기존 safe/unsafe center overflow control은 `[0,56,112]px`, `[-41,15,71]px`다.

Agent Browser의 computed style 및 item geometry가 독립 기대값 8/8 scene, 96/96 item-frame 좌표와 정확히 일치한다. 최대 차이는 0 CSS px이며 모든 used row는 `[10,10,10]px`다. 비교 허용치는 1/64 CSS px로 고정했다. 원 관측은 `chrome-observation.json`, 독립 계산은 `expected-values.json`, 장면별 비교는 `comparison.json`에 있다.

## 재현 자료

- [HTML fixture](./fixture.html)
- [Chrome 관측](./chrome-observation.json)
- [독립 기대값](./expected-values.json)
- [좌표 비교 결과](./comparison.json)
- [Agent Browser 캡처](./chrome-safe-end.png)

이 자료는 Chrome oracle만 검증한다. Spinon 구현, runtime 동작, WPT 적합성, 시뮬레이터·실기기 지원, PixelCopy/Metal readback 또는 C11.4/C11 전체 완료를 입증하지 않는다. Agent Browser는 캡처 후 종료했으며 프로세스 lifecycle을 별도 파일에 남긴다.
