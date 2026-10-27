# C11.4 generic mixed row-axis 사전 비교 · 2026-10-28

## 범위

이 사전 비교는 PR #710의 identity-independent direct mixed text/image column adapter를 행 축 입력으로 확장하는 후속 계획의 기준이다. 기존 fixed-ID row scenes를 새로 계산하거나 이전 결과를 덮어쓰지 않는다. 새 Generic row candidate의 구현은 아직 시작하지 않았다.

입력과 기존 관측 원본은 [2026-10-25 row-axis precomparison](../c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)에 있다. 2026-10-28에는 Agent Browser CLI 0.31.1의 HeadlessChrome 156에서 저장된 동일 HTML을 다시 열어 글꼴, 이미지, computed tracks, item geometry와 source order를 확인했다. [재관측 JSON](agent-browser-observation-2026-10-28.json)은 이 실행 결과다. 기존 입력 HTML SHA-256은 `ea2dad9dda66b4d46e889750672d30b0a79e529995cc4d13a2f7a604dadb5122`, 기존 Agent Browser 화면 SHA-256은 `e8a09fd96ec58d81039634b90db3b1bfd1b33cd6a357c2a9c4f9223f265ed923`다.

## 독립 기대값

CSS Grid 2025 CRD §11.5의 non-spanning intrinsic track sizing은 한 track에 span 1인 item들의 관련 contribution 최댓값으로 track을 계산한다. CSS Box Sizing 2026 Working Draft §§2.2–2.3의 contribution 용어를 사용한다. 두 입력은 각각 직접 [CSS Grid 명세](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)와 [CSS Box Sizing 초안](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/)에서 확인했다. CSS Box Sizing 문서는 Working Draft이며 진행 중인 문서다.

고정 columns `[50,24]px`에서 Ahem `10px/10px` text `XX XXXXX`는 50px column 안에서 두 line box로 감겨 block contribution 20px이 된다. Replaced PNG의 natural block size는 16px이다. 두 direct child가 모두 한 row에 non-spanning으로 배치되므로 independent row expectation은 `max(20,16)=20px`이다. 이 동일한 input에서 min-content와 max-content row 값이 모두 20px이므로 이 matrix는 두 track sizing algorithm을 서로 구별하지 않는다.

## 재관측 요약

| Scene | DOM 순서 (sizing child만) | Columns | Row | Root | Text | Image |
|---|---|---:|---:|---|---|---|
| `min-forward` | text → image | `[50,24]px` | `20px` | `74×20px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `min-reversed` | image → text | `[50,24]px` | `20px` | `74×20px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `max-forward` | text → image | `[50,24]px` | `20px` | `74×20px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `max-reversed` | image → text | `[50,24]px` | `20px` | `74×20px` | `(0,0) 50×20px` | `(50,0) 24×16px` |

The reference HTML also contains a direct-child absolute `.track-probe` to read the row rectangle. It is out of flow and is not one of the two intrinsic sizing items. The runtime fixture must check both mixed inputs and must not include the absolute probe as an in-flow contribution. In the page, roots are stacked at y=0/60/120/180; runtime assertions compare root-relative frames.

## 증거 경계

Agent Browser observation is a Chrome reference, not a Spinon runtime result. It confirms the stored baseline still reproduces; it does not prove novel HostNode IDs, Android/iOS projection, simulator behavior, pixel equality, general CSS admission, WPT conformance, or C11.4 completion. The original screenshot remains linked from the source README; a duplicate screenshot was not created.
