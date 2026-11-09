# C11.4 두 `fit-content()` text row-span 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-root-html-repeated-text-row-span-two-fit-content.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/README.md)다. 각 관점은 서로 다른 입력·track sizing·runtime 경계·정리/보고 실패를 겨냥한다. Chrome observation은 계획의 사전 비교 자료이며 제품 runtime 검증과 구분한다.

| # | 공격 관점 | 실패 가능성 | 계획의 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | 두 fit 위치 조합 경계 | 15개 pair 중 누락·중복 또는 같은 위치 허용 | 1–6에서 서로 다른 위치를 모두 생성하고 15 pair를 보존 | 통과 |
| 2 | cap assignment 뒤바뀜 | authored 위치와 cap이 분리되거나 반대 할당 누락 | 네 representative cap pair마다 두 assignment를 별도 scene으로 고정 | 통과 |
| 3 | cap 단위 혼합 | px와 percentage를 한 단위처럼 비교 | 220px definite basis와 직접 resolved value를 명시 | 통과 |
| 4 | cap range 오류 | `120%` clamp 또는 0 cap 거부 | zero, positive px/px, positive px/percentage, positive percentage/percentage 입력을 대조 | 통과 |
| 5 | min-content floor 과대 가정 | fit track used size를 cap 이하로 강제 | CSS minimum floor를 cap 상한으로 오인하지 않는 독립 판정을 명시 | 통과 |
| 6 | cap 교차 오염 | 한 fit row cap이 다른 fit row remainder를 제한 | 각 row trace에 대응하는 authored track cap을 독립 적용하도록 구현 관문에 명시 | 통과 |
| 7 | intrinsic remainder tie | 1/64px 잔여 단위가 fit row 순서와 무관하게 배정 | row별 growth potential과 기존 rounding regression을 분리 확인 | 통과 |
| 8 | tail order 과장 | 네 선택 order를 전체 permutation 지원으로 기록 | 네 tail order만 한정하고 전체 track order는 제외 | 통과 |
| 9 | multiset drift | min/max/auto count가 입력마다 변함 | 정확한 `2 fit + 2 min + 1 max + 1 auto` multiset을 고정 | 통과 |
| 10 | implicit-list cycle 오프셋 | explicit row가 auto-row list 위치를 이동시킴 | explicit 10px row 뒤 여섯 implicit row index를 frame/trace에서 분리 | 통과 |
| 11 | implicit row 폭발 | 과대 span이 일곱 번째 이상 row를 만들고 성공 | 여섯-row topology만 허용하고 7개 이상은 negative로 분류 | 통과 |
| 12 | span topology 간섭 | span-2/3 item placement가 서로 다른 열에 놓임 | 모든 item의 column 1/2를 고정하고 두 topology만 사용 | 통과 |
| 13 | source order 의존 | DOM 순서만 바꿔 semantic frame이 변함 | 두 topology 각각 정방향/역방향 비교를 유지 | 통과 |
| 14 | text wrapping 변화 | payload/style/font가 scene마다 달라 cap sensitivity를 오염 | 기존 제한 grammar·고정 monospace 10/10만 사용하고 line count 검사 | 통과 |
| 15 | stretch가 intrinsic sizing을 변경 | `align-content:stretch`가 fit/min/max row를 늘림 | start/stretch 관계에서 auto row만 바뀌는지 확인 | 통과 |
| 16 | CSSOM 입력 정규화 누락 | authored six-member order가 typed projection에서 소실 | parsed order·typed cap·computed track order의 동일성 검사 | 통과 |
| 17 | trace 또는 revision 혼합 | 다른 owner/revision의 span과 track trace를 합쳐 부분 frame 공개 | trace consistency, owner/revision, atomic rejection과 same-session recovery를 gate로 둠 | 통과 |
| 18 | 기존 profile 회귀 | two-fit 경로가 one-fit row-span/auto-placement를 덮어씀 | 4,320·1,920·1,440·810 scene의 기존 회귀를 명시 | 통과 |
| 19 | 기본 route 우회 | fixture FFI success를 Root HTML 제품 동작으로 오인 | RuntimeSession 기본 Root HTML route만 positive runtime 경로로 허용 | 통과 |
| 20 | cleanup·증거 과장 | 재사용 simulator 삭제 또는 screenshot을 pixel parity로 표기 | 작업에서 띄운 프로세스/세션만 종료, reusable profile 보존, smoke와 readback 분리 | 통과 |

**결과:** 20개의 독립 실패 관점을 계획·사전 비교에 대조했다. 구현 전에 남은 핵심 불확실성은 두 fit track의 독립 cap 적용과 1/64px remainder 제한이며, 이를 Layout trace 및 RuntimeSession gates로 직접 검증하도록 계획에 포함했다. 사전 Chrome 결과만으로 Spinon runtime 동작을 주장하지 않는다.
