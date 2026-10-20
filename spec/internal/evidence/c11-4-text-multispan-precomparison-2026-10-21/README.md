# C11.4 direct-leaf text multi-span 사전 비교

## 입력과 환경

Chrome fixture는 [grid-track-sizing-c114-text-multispan-precomparison.html](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-text-multispan-precomparison.html)이다. Agent Browser 0.31.1로 Chrome `155.0.8059.85`, macOS `26.5.1` ARM64, DPR 1을 확인했다. `document.fonts.ready` 뒤 Ahem load 상태를 검사했다. 원본 환경과 네 scene의 track/frame은 [chrome155-observation.json](./chrome155-observation.json)에, 화면은 [chrome155-precomparison.png](./chrome155-precomparison.png)에 저장했다.

각 grid는 220×60 CSS px이고 `min-content` column 네 개, 20px row 세 개, gap 0, `align-items:start`를 사용한다. 이 제한 입력에서는 Ahem 10px min-content 폭이 `XX XXXXX` 50px, `XXXX XXXXXXX` 70px, 18개의 `X` 180px이다. 각각의 max-content 폭은 80px, 120px, 180px다.

## 독립 규범 기대값

[CSS Grid Level 1 §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)의 span 길이별 기여 분배에서 기대값을 도출했다. span-2 기여 50px와 70px은 겹친 세 track의 계획 증가 `[25,35,35]`를 만든다. span-3 item은 앞 세 track 합 95px에 남은 85px을 균등 반영하므로 ideal used tracks는 `[53⅓,63⅓,63⅓,0]`이다. Chrome의 1/64 CSS px used-value quantization과 `1/64 CSS px` 이내에서 비교한다.

| Scene | Independent expected tracks | Chrome used tracks | Root-relative item frames `(x,y,w,h)` |
| --- | --- | --- | --- |
| `text-span2-forward` | `25,35,35,0` | `25,35,35,0` | A `(0,0,60,20)`, B `(25,20,70,20)` |
| `text-span2-reversed` | `25,35,35,0` | `25,35,35,0` | B `(25,20,70,20)`, A `(0,0,60,20)` |
| `text-span2-before-span3` | `53⅓,63⅓,63⅓,0` | `53.328125,63.328125,63.34375,0` | A `(0,0,116.65625,10)`, B `(53.328125,20,126.671875,10)`, C `(0,40,180,10)` |
| `text-span3-before-span2` | `53⅓,63⅓,63⅓,0` | `53.328125,63.328125,63.34375,0` | C `(0,40,180,10)`, B `(53.328125,20,126.671875,10)`, A `(0,0,116.65625,10)` |

Forward/reverse results are expected to keep the same used tracks while preserving each item's corresponding source-order frame. All four cases use the same exact text and placement set, with only DOM order changed in the paired scenes.

## 한계

이 파일은 Chrome oracle과 spec-derived expectation만 담는다. Spinon runtime, Android/iOS, WPT, glyph paint 또는 compositor 이후 화면을 검증하지 않는다. 구현 계획은 [c11-4-text-multispan-contributions.md](../../../../plan/c11-4-text-multispan-contributions.md)에 있다. C11.4 checkbox와 공식 완료 수는 변경하지 않는다.

Observation JSON, screenshot, source fixture, plan과 review 문서 hash는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 보존한다. README와 hash manifest는 재귀 hash를 피하려고 manifest에서 제외한다.
