# C11.4 six-fit row-span 계획 실패 관점 검토 20개

검토 대상은 [six-fit row-span 계획](./c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/README.md)다. 제품 코드 변경 전에 입력 shape, oracle, independent expected와 거부 경계를 각각 확인했다.

| # | 실패 관점 | 계획 검토 결과 |
| ---: | --- | --- |
| 1 | six-fit list가 5개/7개 member를 허용하거나 잘못 거부 | 정확히 6 member와 전부 fit-content인 단일 profile로 고정했다. |
| 2 | 여섯 cap 중 중복·누락·미등록 값이 통과 | `[0px,10px,20%,30px,40px,50px]` typed multiset만 인정하고 720 order를 따로 검증한다. |
| 3 | authored cap 순서와 track position이 섞임 | 모든 cap order, computed list, used row와 trace index를 한 scene revision에서 비교한다. |
| 4 | `20%`가 indefinite basis에서 계산됨 | definite 220px block basis를 고정하고 20%의 expected basis를 44px로 둔다. |
| 5 | fit-content cap을 fixed max track으로 취급해 min-content floor를 자름 | W3C §6.6 note와 §11.2 typed min function을 기준으로 삼고 Chrome 사전 비교에서 baseline/candidate를 나눴다. |
| 6 | span item의 scrollable overflow를 eligible로 취급 | profile은 `overflow:visible`로 고정하고 scroll container 입력은 negative matrix에 둔다. |
| 7 | 여러 track span에서 flexible track 조건을 놓침 | 여섯 모든 track은 fit-content이며 flex track/type은 전부 거부한다. |
| 8 | two-topology 중 span-2/3의 row contribution을 누락 | span-2 다섯 item과 span-2/3 네 item topology를 각각 관측하고 covered rows 합을 독립 검산한다. |
| 9 | 명시 row start가 잘못 해석되어 다른 implicit rows 생성 | start line과 span을 fixture에 고정하고 실제 seven used rows를 검사한다. |
| 10 | empty explicit row가 baseline과 candidate 사이에 변함 | control은 항상 min-height 0, 10px frame으로 고정한다. |
| 11 | baseline과 candidate가 서로 다른 DOM/스타일 scene | 동일 scene key의 minimum만 바꾸고 5,760 pairs로 비교한다. |
| 12 | row-span frame을 단순 text height와 혼동 | 각 item frame과 실제 covered track 합, payload frame과 line boxes를 따로 관측한다. |
| 13 | DOM order가 item placement/geometry를 바꿈 | 두 topology에서 forward/reverse를 개별 실행하고 item ID별 frame을 비교한다. |
| 14 | stretch가 fit-content track을 auto track처럼 키움 | start와 stretch 결과를 따로 기록한다. Chrome six-fit span 관측 차이는 0이다. |
| 15 | baseline floor overflow를 Grid 끝 overflow로 오인 | item payload-floor shortfall과 220px grid-end overflow를 다른 counter로 둔다. |
| 16 | sample screenshot이 실제 matrix 측정을 바꿈 | 모든 관측을 먼저 저장한 뒤 별도 sample 표시만 2.5배 확대하고 화면은 계산 입력으로 쓰지 않는다. |
| 17 | CSS list admission만 성공하고 runtime owner/revision이 달라짐 | 구현 관문에 owner, style/layout revision, atomic rejection과 same-session recovery를 포함한다. |
| 18 | six-fit auto-placement profile이 span input까지 우연히 허용 | direct child count, placement predicate와 track-list predicate를 분리하고 기존 9-child profile과 회귀를 확인한다. |
| 19 | browser oracle을 Spinon 앱/플랫폼 결과로 확대 | plan은 Chrome-only precomparison으로 표기하고 app smoke를 feature geometry로 세지 않는다. |
| 20 | 제한 profile을 전체 C11.4 완료로 표시 | 공식 `78/214`, C11.4/C11 checkbox와 contract `0.1.0`을 유지한다. |

## 결론

계획은 exact six-cap row-span shape, eligibility, baseline/candidate pair, independent geometry, fail-closed negative path, ownership/revision 회복과 증거 한계를 고정했다. 이 검토는 제품 구현 검증이나 실제 PR diff 검토를 대체하지 않는다.
