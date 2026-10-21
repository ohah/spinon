# C11.4 depth-three nested Grid anonymous text + image 사전 비교

## 고정 입력

- HTML: [재현 fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-nested-depth-three-anonymous-image-precomparison.html)
- 구조: root → middle → branch → inner Grid. 마지막 Grid의 직접 자식은 anonymous Ahem text item `XX XXXXXXX`와 자연 크기 HTML `img` 하나다.
- 장면: min-content / max-content 각 두 장면이며 source order를 서로 바꿔 총 네 장면이다.
- 글꼴: 저장소 Ahem, `10px/10px`; PNG: `24×16px`; 모든 Grid gap과 box edge는 0px이다.
- 브라우저: Agent Browser `0.31.1`, HeadlessChrome `156.0.8078.48`, macOS ARM, viewport `1280×633`, DPR 1.
- 실행 시 Ahem `document.fonts.check("10px Ahem")`과 이미지 decode/natural-size 확인이 모두 참이었다.

## 독립 규범 기대값

[CSS Grid 2025 CRD §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items)은 각 in-flow child와 각 non-whitespace direct text sequence를 Grid item으로 만든다. [§6.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-display)은 Grid item의 display blockification을 정의하고, [§8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)는 source order에 따른 auto-placement를 정의한다. [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)의 intrinsic track sizing 절차를 이 무-span 두-column 입력에 적용한다.

Ahem `XX XXXXXXX`는 max-content 한 줄 폭이 100px이고 가장 긴 공백 없는 조각의 min-content 폭은 70px이다. 70px에서는 10px line-height의 두 줄(20px)이 되고 100px에서는 한 줄(10px)이다. 자연 PNG item은 24×16px다. 따라서 안쪽 min-content Grid track은 `[70,24]` 또는 source order를 바꾼 `[24,70]`, row는 `[20]`이다. max-content는 `[100,24]` 또는 `[24,100]`, row는 `[16]`이다. 각 바깥 Grid에는 해당 안쪽 Grid 하나만 있고 단일 intrinsic track을 가지므로 이 입력의 네 Grid frame은 min-content에서 모두 `94×20px`, max-content에서 모두 `124×16px`이다. Chrome 값으로 규범 값을 역산하지 않았다.

자세한 기대 배열·frame·허용 오차는 [독립 기대값 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-precomparison-2026-10-22/spec-derived-expectations.json)에, Chrome 관측은 별도 [observation JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-precomparison-2026-10-22/chrome156-observation.json)에 보관했다.
계획 PR 자체의 30개 독립 diff 관점 검토는 [plan-pr-diff-review-30.md](./plan-pr-diff-review-30.md)에 기록했다.

## Chrome 관측

| scene | 각 outer Grid | inner Columns / Rows | text / image |
| --- | --- | --- | --- |
| min-content text → image | `94×20px`, Columns `[94]`, Rows `[20]` | `[70,24]` / `[20]` | text `(0,0) 70×20`; image `(70,0) 24×16` |
| min-content image → text | `94×20px`, Columns `[94]`, Rows `[20]` | `[24,70]` / `[20]` | image `(0,0) 24×16`; text `(24,0) 70×20` |
| max-content text → image | `124×16px`, Columns `[124]`, Rows `[16]` | `[100,24]` / `[16]` | text `(0,0) 100×10`; image `(100,0) 24×16` |
| max-content image → text | `124×16px`, Columns `[124]`, Rows `[16]` | `[24,100]` / `[16]` | image `(0,0) 24×16`; text `(24,0) 100×10` |

Chrome의 실제 관측은 위 고정 입력에서 독립 기대값과 일치한다. Text line fragments, Grid parent chain과 direct child order도 JSON에 기록했다. [Chrome Agent Browser screenshot](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-precomparison-2026-10-22/chrome156-depth-three-mixed-scenes.png)은 레이아웃 oracle 자료일 뿐 Spinon 앱, glyph pixel equality, GPU paint 또는 전체 CSS Grid 지원 근거가 아니다.

## SHA-256

이 문서와 두 oracle, 재현 HTML, screenshot의 byte identity는 [SHA256SUMS](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-precomparison-2026-10-22/SHA256SUMS)에서 확인한다.
