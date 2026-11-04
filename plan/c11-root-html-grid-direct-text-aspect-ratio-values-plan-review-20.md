# C11.1 Root HTML Grid direct-text aspect-ratio 값 확장 계획 검토 20개

이 검토는 구현 전에 계획의 별도 실패 경로와 계약 경계를 점검한다. 기존 `2/1` 계획 검토나 구현 코드 검토를 재사용하지 않는다.

| # | 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 작업이 기본 parsed Root HTML route 대신 C11.4 opt-in profile을 넓힘 | 통과 — 대상 profile을 `RuntimeRootHtmlTextPaintV1`로 고정하고 C11.4 변경을 금지했다. |
| 2 | 숫자 ratio 순서를 height/width로 뒤집음 | 통과 — 모든 기대값을 width/height로 계산하고 양 축 장면을 독립 기대값으로 둔다. |
| 3 | 단일 숫자 표기 `1.5`와 `3/2` pair를 다르게 처리 | 통과 — Chrome computed ratio와 두 frame이 같은 결과인지 비교하고, f32 전이를 쓰는 Rust frame/auto row 허용 오차 0.01 CSS px를 별도로 둔다. |
| 4 | ratio transfer 이후 text automatic minimum을 무시 | 통과 — 10/1의 transferred 6px와 used 10px를 따로 고정한다. |
| 5 | 5/2 height-locked target을 70px Grid track으로 clamp | 통과 — 기대 frame 75×30과 visible right overflow 15px를 명시한다. |
| 6 | `height:30px; ratio:1/2`에서 intrinsic min-width를 무시해 잘못된 15px 기대 | 통과 — 이 조건을 fixture에 쓰지 않고 height 80으로 바꾸어 width 40이 Ahem 최소폭과 맞게 한다. |
| 7 | 플랫폼 시스템 monospace intrinsic width를 portable oracle로 오인 | 통과 — auto-height-control 폭 비교는 제외하고 Chrome 폭은 관찰값으로만 둔다. |
| 8 | CSSOM serialization을 typed ratio source로 사용 | 통과 — 숫자 sizing은 typed `ComputedLayoutAspectRatio`에서 오며 projection 재파싱을 금지한다. |
| 9 | `auto <ratio>`의 content-box semantics를 숫자형 preferred ratio로 취급 | 통과 — `ContentBox` sizing box는 성공 조건에서 배제한다. |
| 10 | degenerate `0/1`을 값 없는 `auto`로 조용히 받아들임 | 통과 — degenerate/unsupported computed values를 fail-closed negative control로 둔다. |
| 11 | finite ratio의 전이 결과가 f32 범위를 벗어나 partial geometry를 생성 | 통과 — transfer 결과가 finite하게 표현되지 않으면 전체 출력을 거부하도록 요구한다. |
| 12 | 두 ratio-bearing item이 같은 owner 안에서 상호 track sizing | 통과 — owner당 하나 제한 및 다중 item 전체 거부를 유지한다. |
| 13 | typed ratio 값과 NodeId/owner snapshot의 관계가 바뀜 | 통과 — 기존 identity, owner-scoped projection, contextual error를 유지한다. |
| 14 | 2/1 회귀·`auto` control을 새 일반 값에 덮어씀 | 통과 — 두 방향 기존 회귀와 각 축 `auto` 대조군을 수락 기준에 남겼다. |
| 15 | CSS-invalid 선언의 cascade fallback을 valid-but-unsupported 오류와 혼합 | 통과 — 두 경우를 별도 controls로 명시한다. |
| 16 | min/max, box edge, placement 또는 arbitrary Grid topology까지 암묵 지원 | 통과 — 기존 strict topology와 sizing constraints를 고정하고 제외 항목으로 반복 명시한다. |
| 17 | oracle geometry를 raster, PixelCopy 또는 compositor 결과로 확대 해석 | 통과 — 독립 frame oracle과 표면 readback 증거를 구분했다. |
| 18 | Rust 단위 검사 font와 앱 기본 Root HTML font 차이를 숨김 | 통과 — Ahem은 Rust test 직접 등록으로만 사용하며 앱은 기본 font 경로로 실행한다. |
| 19 | simulator의 draw/present를 실제 NodeId frame 혹은 surface pixel 측정으로 오기록 | 통과 — 실제 측정 경계를 기록하고 harness 미제공 값을 추정하지 않도록 했다. |
| 20 | bounded ratio slice로 C11.1/C11/공식 완료 수를 앞당김 | 통과 — 내부 계약·STATUS·Tailnet 동기화는 merge 뒤이며 상위 완료 상태는 그대로 둔다. |

**판정:** 20개 독립 관점 모두 계획 기준을 통과했다. 이 기록은 구현 또는 PR 검토를 대신하지 않는다.
