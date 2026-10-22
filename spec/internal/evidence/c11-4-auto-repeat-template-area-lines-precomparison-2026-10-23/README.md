# C11.4 auto-repeat과 template-area line 사전 비교

**상태:** Chrome 사전 관측 및 독립 규범 계산 완료. 이 자료는 Spinon 구현 결과가 아니다. C11.4 checkbox와 공식 완료 수 `77/213`은 바꾸지 않는다.

## 고정 환경과 입력

- Agent Browser `0.31.1`이 실행한 Chrome for Testing `156.0.8078.48` macOS ARM64, `HeadlessChrome/156` user agent
- viewport `1280×633 CSS px`, DPR 1, `horizontal-tb`, `ltr`
- 한 축에만 auto-repeat 한 개, 20px repeated track, 양쪽 20px fixed neighbor, 4px gap, definite `100px`/`116px`/`140px` container
- CSS 입력과 DOM 순서는 [고정 fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-template-areas-c114.html)에 있다. Chrome 원시 computed track list·area matrix·node frame은 [관측 JSON](./chrome156-observation.json), 화면은 [PNG](./chrome156-auto-repeat-areas.png), fixture·관측·화면 SHA-256은 [manifest](./SHA256SUMS.txt)에 기록했다.

## 규범 계산

[CSS Grid 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)는 definite container에서 gap을 포함해 들어가는 반복 수를 정하고, `auto-fit`은 배치 후 in-flow item이 없거나 걸치지 않는 repeated track과 그 양쪽 gutter를 0으로 collapse한다고 규정한다. [§7.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-named-lines)는 각 named area의 `name-start`/`name-end`가 생성 line names이며 authored explicit name과 겹쳐도 추가 line names로 남는다고 규정한다. [§8.3–§8.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)는 그 이름과 `grid-area` 배치 해석을 정의한다.

반복 수 산술은 `116 = 20 + 3×20 + 20 + 4×4`, `140 = 20 + 4×20 + 20 + 5×4`이다. `auto-fit`에서 첫 repeated track만 점유되면 used columns/rows는 `[20,20,0,0,0,20]`; 반복 영역 item이 없으면 `[20,0,0,0,0,20]`이다. Collapse 뒤 0px가 된 반복 track에 닿은 gutter도 collapse하므로, 시작 neighbor와 끝 neighbor 사이에는 추가 4px gutter가 남지 않는다.

area-larger의 `100px` control에서는 `20 + 2×20 + 20 + 3×4 = 92px`라서 fixed neighbors 사이에 auto-fill repeated track 2개가 들어간다. 3개는 `116px`가 필요해 들어가지 않는다. 따라서 template은 4 tracks지만 5-cell area가 5번째 explicit track을 만든다. `grid-auto-columns`/`grid-auto-rows: 20px`가 이 추가 track을 정하고, 전체 track·gutter 길이 `116px`는 100px root를 16px 넘는다.

## 고정 scene 결과

표의 frame은 각 grid root 기준 `[x,y,width,height]` CSS px이다. `hero-start`/`hero-end`는 `grid-template-areas`가 생성한 line aliases이며, area가 expanded grid보다 짧은 장면에서는 area 셀 번호를 auto-repeat의 마지막 track에 맞춰 이동시키지 않는다.

| scene | area / expanded track | 규범 기대 frame: prefix, hero, suffix, tail |
| --- | --- | --- |
| `columns-fill` | area 5열 / expanded 5열 | `[0,0,20,20]`, `[24,0,68,20]`, `[96,0,20,20]` |
| `columns-fit` | area 6열 / expanded 6열, 반복 3개 collapse | `[0,0,20,20]`, `[24,0,20,20]`, `[44,0,20,20]` |
| `columns-area-shorter` | area 5열 / expanded 6열 | `[0,0,20,20]`, `[24,0,68,20]`, `[96,0,20,20]`, `[120,0,20,20]` |
| `columns-area-larger` | 100px root, template 4열 / area가 늘린 explicit grid 5열 | prefix `[0,0,20,20]`, hero `[24,0,68,20]`, suffix `[96,0,20,20]`; 16px overflow |
| `columns-area-line-collision` | area 5열 / expanded 5열, authored `hero-start`/`hero-end` 중복 | area `hero` `[0,0,92,20]`, 두 번째 named-line occurrence `[24,0,68,20]` |
| `columns-fit-empty-area` | area 6열 / expanded 6열, hero item 없음·반복 4개 collapse | prefix `[0,0,20,20]`, suffix `[20,0,20,20]` |
| `rows-fill` | area 5행 / expanded 5행 | `[0,0,20,20]`, `[0,24,20,68]`, `[0,96,20,20]` |
| `rows-fit` | area 6행 / expanded 6행, 반복 3개 collapse | `[0,0,20,20]`, `[0,24,20,20]`, `[0,44,20,20]` |
| `rows-area-shorter` | area 5행 / expanded 6행 | `[0,0,20,20]`, `[0,24,20,68]`, `[0,96,20,20]`, `[0,120,20,20]` |
| `rows-area-larger` | 100px root, template 4행 / area가 늘린 explicit grid 5행 | prefix `[0,0,20,20]`, hero `[0,24,20,68]`, suffix `[0,96,20,20]`; 16px overflow |
| `rows-fit-empty-area` | area 6행 / expanded 6행, hero item 없음·반복 4개 collapse | prefix `[0,0,20,20]`, suffix `[0,20,20,20]` |
| `columns-fit-numeric`, `rows-fit-numeric` | areas 없는 숫자 line 대조 | prefix 0, hero 24, suffix 44 (해당 반복 축 좌표) |

Chrome `auto-fit`의 occupied suffix는 columns/rows 모두 root 기준 48px에, area 없는 numeric control도 48px에 놓였다. 모든 반복 track이 빈 scene에서도 Chrome suffix는 24px였다. 규범 계산값은 각각 44px와 20px다. 따라서 관측 차이는 이 fixture에서 area alias의 차이가 아니라, areas 없는 numeric controls에도 같은 4px가 남는 Chrome 156 auto-fit gutter 관측 차이다. 이 차이는 독립 기대값을 대체하지 않는다. 계획/구현의 pass 기준은 규범 계산과 고정 Spinon scene이며 Chrome 값은 별도 진단으로 남긴다.

`columns-area-shorter`와 `rows-area-shorter`는 area matrix가 expanded grid보다 작을 때 area alias가 authored cell 경계에 고정되고, 마지막 fixed track이 별도로 남는 것을 확인한다. `columns-area-line-collision`은 Chrome에서 `grid-area: hero`가 첫 `hero-start` line부터 마지막 `hero-end` line까지 놓이며, 명시 occurrence 배치는 두 번째 `hero-start`를 선택하는 것을 확인한다. CSSOM이 반환한 line-name/track ordering과 각 child rect는 JSON에 보존했다.

`columns-area-larger`와 `rows-area-larger`에서는 Chrome이 template의 4개 track 뒤에 area가 요구한 5번째 explicit track을 사용한다. CSSOM track list는 각 축 `20px` 다섯 개를 반환하며, area가 늘린 마지막 track은 `grid-auto-* : 20px`로 크기가 정해진다. suffix가 root 기준 96px에서 시작해 116px까지 그려지는 것을 확인했다. 이는 area dimension을 template repeat fragment에 맞춰 재해석하지 않는 경계다.

## 증거의 범위

Chrome 관측은 브라우저 동작일 뿐 WPT 결과나 Spinon 결과가 아니다. 4px gutter 차이가 있는 `auto-fit` suffix frame을 정답으로 복사하지 않는다. 여러 auto-repeat, 양 축 동시 auto-repeat, intrinsic/percentage track, non-default writing mode/direction, WPT, simulator 및 실기기 표면은 이 사전 비교에 없다. Chrome fixture에는 13개 고정 scene이 있고 구현/runtime 결과는 아직 없다. 화면은 CSS 색 사각형과 track 배치를 보여주며 app draw, border stroke 또는 compositor 이후 pixels를 검증하지 않는다.
