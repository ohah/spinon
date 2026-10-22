# C11.4 auto-repeat named-line implicit fallback 사전 비교

## 범위와 기준

PR #530의 auto-repeat authored line-name 보존 이후, authored name이 없거나 요청 occurrence보다 명명된 explicit line이 적을 때 named placement가 implicit grid까지 확장되는 경계를 고정한다. 이 문서는 Chrome 관찰과 CSS 규칙에 따른 독립 기대값만 기록한다. Spinon 구현 결과나 CSS 지원 완료를 뜻하지 않는다.

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement). Integer named line은 충분한 named line이 없으면 implicit grid lines를 해당 이름으로 간주한다. Named span도 검색 방향 쪽의 implicit lines를 이름 있는 line처럼 센다. §7.2.3은 반복 트랙과 line-name 그룹의 확장 규칙을 정한다.
- 고정 Taffy 후보: 저장소 vendored `taffy 0.15.0`의 `NamedLineResolverAxis`는 named line과 span을 numeric placement로 해석하고, Spinon bridge는 이를 auto-repeat expanded line metadata와 함께 호출한다. 이 코드는 후보 근거이지 Chrome/W3C 결과의 대체 기준이 아니다.
- 기준 브라우저: Agent Browser로 연 Google Chrome `156.0.8078.48`, macOS `26.5.1` ARM64, viewport `1280×633`, DPR 1. `chrome156-observation.json`과 `chrome156-browser-metadata.json`에 관찰을 저장했다.
- 고정 입력: [`grid-auto-repeat-named-line-implicit-fallback.html`](./grid-auto-repeat-named-line-implicit-fallback.html). 고정 `125px` 축에서 `repeat(auto-fill, [cell] 40px [cell])`은 3개 `40px` 반복 track을 만든다. implicit track은 `20px`, gap은 0이다. 각 scene은 단일 item을 배치한다.
- 화면: [Chrome 관찰 screenshot](./chrome156-auto-repeat-named-line-implicit-fallback.png). Raw computed track serialization, root-relative item frame, computed placement는 JSON에 보존했다.
- 계획 PR diff 검토: [서로 다른 실패 관점 20개](./plan-pr-diff-review-20.md). 계획 문서 자체를 검토한 [계획 검토 20개](../../../../plan/c11-4-auto-repeat-named-line-implicit-fallback-plan-review-20.md)와는 별도 기록이다.
- Browser 절차: Chrome fixture는 Agent Browser로 열어 수치와 screenshot을 얻었다. `terminal-browser`는 사용하지 않았다. 현재 브라우저는 비교 작업 뒤 종료한다.

## 독립 기대값과 Chrome 관측

Auto-fill의 explicit grid는 각 축에서 3×40px다. `125px / 40px`에서 4회째 반복은 들어가지 않고, 각 axis에 4개의 explicit boundary line이 남는다. §8.3의 규칙을 line group에 독립 적용하면 absent `ghost`의 positive `5/6`은 explicit end 뒤 6개의 20px implicit track을 요구하고, insufficient `cell`의 positive `5/6`은 2개를 요구한다. Negative absent `-5/-4`는 explicit grid 앞 5개 implicit track, insufficient `cell -5/-4`는 1개를 요구한다. Named span은 반대 endpoint에서 각 search direction의 named line을 센다. `span ghost`는 다음 방향 implicit line까지 1개 20px track을 포함하고, `span 2 cell`에서 explicit name이 하나만 남은 경우에는 40px explicit track과 20px fallback track을 포함한다. 여기서 산출한 root-relative frame 기대값은 다음 순서다: columns `positive-missing (220,0) 20×10`, `positive-insufficient (140,0) 20×10`, `negative-missing (0,0) 20×10`, `negative-insufficient (0,0) 20×10`, forward missing `(120,0) 20×10`, backward missing `(0,0) 20×10`, forward insufficient `(80,0) 60×10`, backward insufficient `(0,0) 60×10`; row controls are the same direction cases at `(0,220) 20×10` and `(0,0) 20×10`. The table records Chrome's independent actual track list and frame beside each target. Both must match, so a line interpretation error cannot be hidden by testing only frames or track counts.

| scene | CSS placement | Chrome used tracks | item frame (root-relative) | 확인 경계 |
| --- | --- | --- | --- | --- |
| `positive-missing` | columns `5 ghost / 6 ghost` | 3×40px explicit + 6×20px implicit | `(220,0) 20×10px` | 없는 이름의 양수 occurrence |
| `positive-insufficient` | columns `5 cell / 6 cell` | 3×40px explicit + 2×20px implicit | `(140,0) 20×10px` | existing name의 occurrence 초과 |
| `negative-missing` | columns `-5 ghost / -4 ghost` | 5×20px leading implicit + 3×40px explicit | `(0,0) 20×10px` | end edge부터 없는 이름 역방향 occurrence |
| `negative-insufficient` | columns `-5 cell / -4 cell` | 1×20px leading implicit + 3×40px explicit | `(0,0) 20×10px` | end edge부터 existing name occurrence 초과 |
| `span-forward-missing` | line 4 / `span ghost` | 3×40px explicit + 1×20px implicit | `(120,0) 20×10px` | explicit line에서 forward named span |
| `span-backward-missing` | `span ghost` / line 1 | 1×20px leading implicit + 3×40px explicit | `(0,0) 20×10px` | explicit line에서 backward named span |
| `span-forward-insufficient` | line 3 / `span cell 2` | 3×40px explicit + 1×20px implicit | `(80,0) 60×10px` | explicit name 하나 뒤 occurrence 2 fallback |
| `span-backward-insufficient` | `span cell 2` / line 2 | 1×20px leading implicit + 3×40px explicit | `(0,0) 60×10px` | explicit name 하나 앞 occurrence 2 fallback |
| `row-positive-missing` | rows `5 ghost / 6 ghost` | 3×40px explicit + 6×20px implicit | `(0,220) 20×10px` | positive fallback on row axis |
| `row-negative-missing` | rows `-5 ghost / -4 ghost` | 5×20px leading implicit + 3×40px explicit | `(0,0) 20×10px` | negative fallback on row axis |

Track count와 line/group serialization은 exact다. Item frame은 Chrome double을 JSON으로 저장하고 `1/64 CSS px` 이내에서 비교한다. Positive/negative computed placement strings는 CSSOM serialization 관찰로 보존하며 내부 resolved integer line의 직접 노출이라고 해석하지 않는다.

## 해석 한계

- 한 Chrome build의 고정 fixture다. 다른 browser engines, WPT, Spinon runtime, Android/iOS 화면은 구현 PR에서 별도로 확인해야 한다.
- 범위는 기존 제한 auto-repeat profile의 fixed-size, 단일 `auto-fill` repeat, integer named line 및 named span이다. `auto-fit`의 post-placement collapse, multiple repeat components, named areas, subgrid, writing mode, 일반 CSS admission은 다루지 않는다.
- 기존 repeated line-name occurrence, repeat boundary merging, auto-placement 충돌, Taffy track cap과 fallible allocation 경계는 그대로 유지해야 한다.
- `spec/STATUS.md` C11.4 체크와 공식 완료 집계 `77/213`은 변경하지 않는다.

## 입력 및 관측 hash

- HTML fixture: `da4285d45d3174ca17d1db0f33045605f57e66dd2d0c6ac756b94aa9e10c133a`
- Chrome observation JSON: `c3a5b2ea3876adba14eb387075a34fe13dbf1b655504a26c9ce16e713c6eadf8`
- Chrome browser metadata JSON: `53763a5c893a053b96591382cffed2ede0397d6a0092d4ce52aa66343693e065`
- Chrome screenshot: `54d4c1fb097e5739961f5b3072524ba0825dabaedc3ea186721ba5b1bf01da41`
