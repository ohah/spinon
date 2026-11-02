# C11.1 Root HTML explicit `fit-content(px)` 계획 검토 20개

구현 전에 계획/기준 입력의 failure mode를 서로 다른 20개 관점에서 검토했다.

| # | 실패 관점 | 검토 결과 및 방지 기준 |
| --- | --- | --- |
| 1 | Chrome 결과를 규범으로 취급 | W3C §7.2.1의 sizing formula로 기대값을 독립 계산하고 Chrome은 교차 확인으로만 둔다. |
| 2 | Ahem이 로드되기 전 intrinsic width 측정 | `document.fonts.ready`, `fonts.check`, loaded status와 고정 font hash를 확인한다. |
| 3 | min-content가 cap보다 커 cap 동작을 구분하지 못함 | cap 10px/40px/120px 세 장면을 두어 floor·clamp·max-content 사례를 분리한다. |
| 4 | item의 authored width가 track 폭을 대신 결정 | fit-content item에 width를 주지 않고 computed track과 item frame을 따로 기록한다. |
| 5 | maximum cap만 보고 minimum floor를 생략 | 10px column cap과 20px row cap에서 min-content floor를 확인한다. |
| 6 | row/column 축을 혼동 | 별도 `height:min-content` probe와 row scene에서 20px inline-size, 4×10px Ahem line, 40px block minimum을 각각 기록한다. |
| 7 | fixed `repeat()` 결과를 단일 track으로 오해 | 두 repeat occurrence의 used size와 item origins를 각각 고정한다. |
| 8 | repeat seam에 gap이 빠짐 | 기준 장면은 gap 0을 명시하고 gap 동작을 이번 slice에 섞지 않는다. 기존 gap regression을 유지한다. |
| 9 | auto minimum을 항상 min-content와 같다고 가정 | W3C의 auto-min 예외를 확인했다. Positive scenes는 비-scrollable plain Ahem text에 한정하고 overflow/aspect-ratio 영향은 범위 밖으로 둔다. |
| 10 | negative/NaN/infinity가 typed projection까지 통과 | admission gate는 finite 및 non-negative를 확인하고 raw typed values의 rejection tests를 둔다. |
| 11 | percentage argument가 px로 잘못 인정 | `Percentage`는 positive list에서 빼고 explicit rejection case로 고정한다. |
| 12 | calc 인자에서 해석된 px만 검사해 범위가 넓어짐 | `Calc` 전체를 거부하며 별도 계획 없이 산술 평가를 추가하지 않는다. |
| 13 | explicit 지원이 implicit tracks로 새어 나감 | `grid-auto-rows`/`grid-auto-columns` 목록은 현재 allowlist를 유지하고 rejection을 확인한다. |
| 14 | auto-repeat/line-name/area-created track을 우연히 허용 | 기존 auto-repeat·line-name·area 검사를 유지하고, positive named-area map은 authored track 수와 정확히 일치시킨다. |
| 15 | `repeat()`에서 값이 admission 전에 절단/복제 | 확장 후 전체 track list와 allocation bound를 확인하고 두 track expected를 비교한다. |
| 16 | 선언 fallback이 invalid CSS를 성공으로 보임 | parser cascade와 property-bound projection error를 구분하고 invalid/unsupported declaration control을 둔다. |
| 17 | 다른 profile에도 admission이 퍼짐 | 변경 위치를 Root HTML profile gate로 한정하고 track-sizing profile 기존 동작을 회귀 확인한다. |
| 18 | 모바일 화면 캡처만으로 geometry를 주장 | Rust/runtime frame, `layout=ready`, draw/present와 surface sample을 별도 artifact로 보존한다. |
| 19 | AVD/Metal 표본을 실기기 pixel로 과장 | AVD software renderer와 iOS pre-compositor drawable 한계를 명시한다. |
| 20 | bounded pass를 C11 완료로 승격 | C11/C11.1 checkboxes와 official counter를 유지하고 제한 범위를 plan/status/evidence에 기록한다. |

계획은 구현, 테스트, simulator 실행이 시작되기 전에 이 20개 관점을 반영한다. 계획 PR의 변경 diff review는 이 문서와 별도 20개 항목으로 수행한다.
