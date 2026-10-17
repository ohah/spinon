# C15.3 non-zero scroll ancestor text transform 계획 검토 20개

**검토 시점:** 코드 구현 전. Chrome 사전 비교, 현재 runtime scroll snapshot 계약, 범위 제외 항목을 대조했다.

| # | 실패 관점 | 계획의 방어·판정 기준 |
|---:|---|---|
| 1 | `scrollLeft`를 glyph device pixel로 바로 빼 CSS geometry와 raster 좌표를 혼합한다. | offset·origin은 CSS px로 계산하고 DPR 배율은 WGPU 정점 생성 경계에서만 적용한다. Chrome 기준 x delta와 0.05 CSS px를 비교한다. |
| 2 | scroll 양수 방향에서 text origin을 잘못 더해 content가 스크롤 방향과 함께 움직인다. | horizontal `0→3`에서 parent-relative x `+2→-1`을 고정 관찰값으로 둔다. |
| 3 | horizontal offset이 text의 y origin이나 line baseline을 바꾼다. | horizontal fixture는 y와 height 불변, vertical fixture는 x와 width 불변을 함께 판정한다. |
| 4 | vertical offset 부호 오류로 text가 scroll viewport에서 멀어진다. | vertical `scrollTop=18`에서 parent-relative y `+23→+5`를 acceptance로 둔다. |
| 5 | 중첩 scroll ancestor 하나를 빠뜨리거나 같은 ancestor offset을 두 번 더한다. | nested `4+5`의 총 이동 `9px`, 변경 `2+3`의 총 이동 `5px`를 각각 검사한다. |
| 6 | 내부 scroller의 clip이 자기 content와 같이 이동해 경계가 풀린다. | 각 scrollport clip은 viewport에 고정하고, 내부 scroller의 viewport만 바깥 ancestor transform을 따른다. 바깥 hit은 text를 반환하면 안 된다. |
| 7 | scroll 변경을 현재 origin에 누적해 업데이트 순서에 따라 위치가 달라진다. | `3→6`, `18→7`, `4+5→2+3`에서 base origin과 현재 전체 offset을 재계산한다. |
| 8 | offset을 0으로 돌려도 이전 delta가 남는다. | 세 fixture 모두 `→0` 결과가 시작 시 parent-relative rect와 같아야 한다. |
| 9 | `scrollWidth`를 border-box width로 취급해 max offset을 잘못 계산한다. | border-box, `clientWidth/Height`, `scrollWidth/Height`를 별도로 관찰하고 clamp는 runtime container의 max offset을 따른다. |
| 10 | `NaN`, 무한대, 음수 요청이 glyph 좌표나 clip에 들어간다. | invalid scroll request는 snapshot 생성 전에 error로 닫고 기존 immutable snapshot은 보존한다. |
| 11 | ancestry가 가리키는 scroll container가 없어 보이지 않는 content를 성공 처리한다. | owner metadata의 모든 ancestor가 runtime container로 존재해야 하며 없으면 `InvalidScrollState`다. |
| 12 | text node가 스스로 scroll owner일 때 같은 경로에 억지로 포함한다. | text self-owned scroll container는 기존 `UnsupportedTextScroll` 경계를 유지한다. |
| 13 | 오래된 scroll revision에서 offset을 적용해 새 문서 장면을 덮는다. | 기존 `StaleScrollRevision` 검증을 유지하고 invalid update negative control을 둔다. |
| 14 | 중복 owner update의 순서에 따라 결과가 비결정적이다. | update 목록의 중복 owner는 기존대로 오류이며 sequence에 넣어 회귀 검증한다. |
| 15 | text는 움직였지만 WGPU clip이 옛 위치에 남거나 hit clip이 갱신되지 않는다. | render text는 active owner paint clip과 교차하고 outside sample은 stage background; owner hit-test의 active clip도 그대로 확인한다. |
| 16 | owner box hit-test를 glyph별 text hit-test 완료로 확대 해석한다. | 이번 관찰은 Chrome `elementFromPoint` oracle이며 Spinon 결과는 owner hit/clip만 판정하고 glyph hit·DOM event는 제외한다고 명시한다. |
| 17 | clipping된 glyph bitmap을 transform 때 재 rasterize하거나 atlas cache를 깨뜨린다. | 고정 Ahem bitmap을 재사용하고 CSS origin만 이동한다. interior ink exact와 clip edge sample을 분리한다. |
| 18 | alpha, paint order, sibling overlay가 scroll support 때문에 바뀐다. | 기존 `DrawText` 순서, 색/alpha, box의 active scroll transform을 회귀 기준으로 유지하며 새 fixture에는 text 외 paint command를 넣지 않는다. |
| 19 | 명시적 Ahem font load 실패 뒤 system fallback이 oracle을 가장한다. | `document.fonts.status`와 `document.fonts.check()`를 보존하고 둘 중 하나가 실패하면 관찰 전체를 invalid로 판정한다. |
| 20 | LTR 단일 고정 fixture의 통과를 RTL, writing mode, scroll bar paint, 일반 CSS text 지원으로 확대한다. | 보고·상태·PR에서 이 제한 sample만 다루고 RTL/logical scroll, scrollbars, DOM admission, style projection, C16 및 C15 완료는 명시 제외한다. |

**판정:** 20개 관점을 계획에 반영했다. 코드 작업은 이 표와 별개의 새 구현 검토를 만든 뒤에만 시작한다.
