# C11.1 Root HTML named areas × percentage tracks · 계획 실패 관점 20개

대상은 [`c11-root-html-grid-area-percentage.md`](./c11-root-html-grid-area-percentage.md), 네 개의 고정 Chrome 156 source page, Chrome observation, 독립 expected JSON, current-code rejection test다. 이 검토는 설계와 계획만 평가하며 구현·runtime 검증으로 세지 않는다.

| # | 독립 실패 관점 | 근거와 판정 |
|---:|---|---|
| 1 | 현재 code gap이 단지 추정이고 이미 지원될 수 있다 | `validate_root_html_named_area_template()`는 fixed px pair와 exact two-axis `1fr 2fr`만 허용한다. focused current-code test가 `%` named-area rejection을 실제 assertion으로 통과했다. PASS. |
| 2 | 목표가 일반 CSS Grid 전체 지원으로 오해될 수 있다 | 기본 parsed Root HTML route와 고정 2×2 area map으로 한정했다. PASS. |
| 3 | CSS area matrix와 semantic items가 다를 수 있다 | 네 Chrome source가 모두 동일한 `header/header`, `side/content` matrix와 세 대응 item을 쓴다. PASS. |
| 4 | percentage columns가 root width 아닌 viewport 기준으로 계산될 수 있다 | 독립 계산은 definite 120px Grid content width를 사용하고 Chrome computed tracks가 이를 대조한다. PASS. |
| 5 | percentage rows가 inline axis 기준으로 잘못 계산될 수 있다 | 독립 계산은 definite 70px Grid content height를 사용하고 Chrome row tracks와 item y/height를 기록한다. PASS. |
| 6 | both-axis case가 한 축 하나만 확인할 수 있다 | `both-axes`는 columns `[30,90]`, rows `[14,56]` 및 세 item frame을 함께 고정한다. PASS. |
| 7 | mixed-unit case가 percentage 때문에 생긴 크기와 px size를 분리하지 못한다 | `mixed-px-percent`는 두 축에 px/% 혼합을 사용하고 남는 20px·6px가 auto stretch되지 않음을 관찰한다. PASS. |
| 8 | percentage가 100%를 넘을 때 100%로 clamp될 수 있다 | `overflowing-columns`는 120%와 10% track, 156px total, root 밖 content origin 144px를 별도로 고정한다. PASS. |
| 9 | overflow frame이 root 크기에 clamp되어 요구 동작을 감출 수 있다 | expected item/header frame은 root보다 큰 used columns를 보존하고 `root.scrollWidth=156`을 함께 기록한다. PASS. |
| 10 | zero percentage track이 음수 또는 미정 track과 혼동될 수 있다 | `zero-track`은 authored 0%의 `[0,120]` track list와 0-width side item을 대조한다. Negative values는 별도 rejection gate다. PASS. |
| 11 | item frame이 DOM source order를 의미상 area로 오인할 수 있다 | Source order를 `content`, `header`, `side`로 뒤집고 각 `grid-area` 결과를 별도로 관찰한다. PASS. |
| 12 | gap이 percentage basis에서 빠져 expected values가 틀릴 수 있다 | 모든 신규 source에서 gap은 0이며 plan은 basis/gap 혼합을 별도 구현 범위에서 제외한다. PASS. |
| 13 | 기존 named-area fixed-px 동작을 새 validator가 깨뜨릴 수 있다 | 구현 관문에 PR #1006 fixed-px named-area regression을 넣었다. PASS. |
| 14 | 기존 named-area fractional 동작을 percentage 추가가 덮어쓸 수 있다 | 기존 `1fr 2fr` 양축 경로는 그대로 유지하고 별도 regression으로 검사한다. PASS. |
| 15 | `%`와 `fr` 동시 사용을 새 조합이 무심코 허용할 수 있다 | 새 named-area percentage mode는 두 축의 `px/%`만 허용하고 `% + fr` named-area 조합은 명시적으로 계속 거부한다. PASS. |
| 16 | indefinite axis, intrinsic sizing 또는 aspect-ratio가 잘못 지원될 수 있다 | Existing percentage-axis conditions과 negative matrix를 계획 수락 기준에 명시했다. PASS. |
| 17 | minmax/calc/repeat/line placement가 percentage 허용을 통해 우회될 수 있다 | 각 형태를 별도 fail-closed 목록에 두고 property 진단과 scene 미발행을 요구한다. PASS. |
| 18 | overflow/f32 값이 allocation 또는 partial scene 뒤에서 실패할 수 있다 | 기존 resolved-range gate를 재사용하고 오류 전에 track aggregate와 no-partial-output 검사를 요구한다. PASS. |
| 19 | Android/iOS screenshot이 pixel readback이나 실기기 근거처럼 읽힐 수 있다 | 계획은 visual evidence만 기본으로 요구하고 AVD software renderer 및 iOS capture 경계를 명시한다. PASS. |
| 20 | 제한 조합이 C11.1/C11 또는 official count를 완료 처리할 수 있다 | 계획·parent index 모두 implementation 전이며 C11 checkbox와 `72/213` 유지로 고정한다. PASS. |

결과: 20/20 PASS. 계획 자체의 범위를 평가했으며 implementation 또는 PR diff 검토를 대체하지 않는다.
