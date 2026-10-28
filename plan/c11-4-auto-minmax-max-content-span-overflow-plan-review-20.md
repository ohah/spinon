# `minmax(auto, max-content)` two-track span 계획 · 20개 독립 실패 관점

검토 범위는 [집중 계획](./c11-4-auto-minmax-max-content-span-overflow.md), [독립 기대값](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/expected.json), [Chrome 156 observation](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/chrome/chrome156-observation.json), [fixture](../tests/fixtures/css/c11/grid-auto-minmax-max-content-two-track-span-overflow-c114-precomparison.html)와 CSS Grid 2025 CRD §6.6/§11.5/§11.5.1/§11.6이다. 각 항목은 서로 다른 입력 또는 계약 실패를 따로 점검했다.

| # | 실패 관점 | 검토와 조치 | 판정 |
| ---: | --- | --- | --- |
| 1 | span item이 실제로 `auto` min track을 포함하지 않음 | Grid에는 정확히 두 `minmax(auto,max-content)` columns만 있고 item은 `1 / span 2`로 placement한다. | 통과 |
| 2 | implicit/auto placement 때문에 span 경계가 달라짐 | 고정 explicit line placement를 expected JSON과 구현 gate에 적었다. | 통과 |
| 3 | test가 결과 track 개수만 확인해 multi-span을 놓침 | 두 target base sizes와 `spans_multiple_tracks=true`를 별도로 요구한다. | 통과 |
| 4 | indefinite root가 intrinsic sizing 경로를 바꿔 결과를 우연히 맞춤 | root의 width와 height를 모두 definite `80×20px`로 고정한다. | 통과 |
| 5 | 여러 줄로 wrap되어 min-content가 100px이 아님 | Ahem의 unbreakable `XXXXXXXXXX`, `white-space:normal`, fixed 10px font로 min-/max-content를 100px로 만든다. | 통과 |
| 6 | OS/system font fallback이 glyph advance를 바꿈 | 저장소 Ahem TTF를 fixture에 포함하고 browser에서 font loaded/check true를 observation에 남긴다. | 통과 |
| 7 | text intrinsic input과 item content width를 혼동 | 사전 비교는 `scrollWidth=100px`을 보조값으로 기록하고 normative contribution은 별도 expected JSON으로 계산한다. | 통과 |
| 8 | `visible` 또는 `clip`을 scrollable로 잘못 분류 | CSS Grid §6.6 및 CSS Overflow §3.1의 분류를 고정하고 두 case 모두 content-based minimum 100px으로 예상한다. | 통과 |
| 9 | `hidden`, `auto`, `scroll` 중 하나가 빠지거나 잘못 묶임 | 세 값을 각각의 독립 case ID와 0px automatic minimum assertion으로 유지한다. | 통과 |
| 10 | overflow X/Y computed-value 상호작용이 target axis를 오염 | Y authored `visible`을 고정하고 browser의 computed Y 전환(`auto`)을 관측에 보존한다. Rust 검증은 X only로 명확히 제한한다. | 통과 |
| 11 | flex track 예외 때문에 content-based minimum이 0이 되어야 함 | 두 tracks 모두 non-flexible `max-content` maximum이며 trace에서 `spans_flexible_max_track=false`를 요구한다. | 통과 |
| 12 | span-two minimum contribution이 두 base에 고르게 나뉘지 않음 | CRD §11.5.1의 equal distribution을 적용해 `[50,50]px` 두 final bases를 각각 기대한다. | 통과 |
| 13 | 같은 aggregate width를 다른 per-track 배치가 가림 | expected JSON과 계획은 `[50,50]` 및 `[40,40]` 배열을 보존하고 sum-only 비교를 금지한다. | 통과 |
| 14 | scrollable 0 minimum과 max-content growth limit을 혼동 | 0px Minimum source와 100px MaxContent contribution을 별도 trace로 assert하고 50px growth limits 뒤 80px available width로 maximize한다. | 통과 |
| 15 | positive free space가 정확히 80px이 아님 | root `80px`, zero gap, no border/padding을 고정하고 final `[40,40]px`를 기대한다. | 통과 |
| 16 | non-scrollable item frame이 track sum인 100px에서 줄어듦 | content-based minimum에 따라 `100×20px` item frame을 기대하고 root width 80px과 분리한다. | 통과 |
| 17 | scrollable item frame과 content width가 같은 것으로 오인 | expected item frame `80×20px`과 `scrollWidth=100px`을 구분한다. Scroll UX 자체를 검증하지 않는다고 명시한다. | 통과 |
| 18 | CSS clip 또는 scroll 동작을 layout 결과에서 추정 | scope limits에서 clip pixels, scroll offsets, scrollbars와 paint를 명시적으로 제외한다. | 통과 |
| 19 | 이전 single-track matrix 또는 product code를 변경할 우려 | 별도 `#[cfg(test)]` submodule만 추가하는 범위로 두고 기존 single-track tests, CSS admission, FFI, ABI, app route, Taffy source는 유지한다. | 통과 |
| 20 | Chrome 관측을 규범 기대값 또는 Spinon 결과로 오인 | 독립 expected JSON을 browser 관측과 분리하고 screenshot/README에 Chrome baseline임을 명시한다. | 통과 |

계획에서 예상 수치, fixture 축 경계, axis-computed-value 관측, zero-tolerance 검증 대상과 제품 범위 제외를 고정했다. 미결 계약은 없으며 구현 착수 기준은 계획의 focused/package/fmt/Clippy gate와 새 20-view implementation review다.
