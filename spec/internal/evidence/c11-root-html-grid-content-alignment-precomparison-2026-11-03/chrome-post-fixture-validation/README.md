# 최종 동시 정렬 fixture의 Chrome 관찰

Agent Browser `HeadlessChrome/156.0.0.0`, DPR 1, viewport 1280×633에서 최종 fixture SHA-256 `95d57100a0f24d353911934922ba7832f6461cd39586e76b8bb92fc1a7f2e132`를 열었다. computed content alignment는 inline `space-evenly`, block `space-around`다.

240×180px container, columns `[40, 60]px`, rows `[30, 50]px`, 양 축 10px gap에서 Chrome item 시작점은 다음과 같다(CSS px).

| Item | x | y |
|---|---:|---:|
| a | 43.328125 | 22.5 |
| b | 136.65625 | 22.5 |
| c | 43.328125 | 107.5 |
| d | 136.65625 | 107.5 |

이 값은 독립 산식 및 Taffy runtime frame test와 1/64 CSS px 안에서 비교한다. 음수 free-space 대조는 80×70px container다. `unsafe center`는 첫 column 시작을 −15px로 두고 `safe end`는 첫 row 시작을 0px로 둔다.

- [최종 fixture positive screenshot](positive-space-evenly-around.png), SHA-256 `a61913a8bf67420471ad23378b9266beeebbe43de49c6cf21c4ea9a8a107da90`.
- [최종 fixture negative screenshot](negative-safe-unsafe.png), SHA-256 `b2b6e4493453bee3f93e359ab89396f7315df2c2a8d176819ce967e4064fd166`.

캡처는 Chrome 관찰이며 Spinon 적합성, WPT 통과 또는 플랫폼 표면 픽셀 결과가 아니다.
