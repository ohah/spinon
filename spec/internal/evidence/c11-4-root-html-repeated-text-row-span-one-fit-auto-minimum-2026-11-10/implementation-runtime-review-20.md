# 구현·Runtime 실패 관점 검토 20개

검토 대상은 one-fit row-span `min-height:auto` 구현과 RuntimeSession 실행 경로다. 계획 검토를 반복하지 않고 실제 profile 경계, trace, 거부·복구와 기존 경로 회귀를 별도로 확인했다. 읽기 전용 검토에서 현재 diff가 일으키는 수정 필요 결함은 발견되지 않았다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | one-fit 후보에서 fit-content track이 둘 이상 허용되는가 | layout·style이 각각 정확히 하나인지 검사한다. |
| 2 | 지원하지 않는 fit 위치가 들어오는가 | three-row span profile은 1–3 위치만 허용한다. |
| 3 | implicit row 수가 늘거나 줄어드는가 | 새 후보는 implicit row 세 개로 제한한다. |
| 4 | explicit-row control이 `min-height:auto`로 바뀌는가 | 후보는 control `min-height:0`을 요구하고 거부 테스트가 있다. |
| 5 | text item들의 자동 최소 크기가 섞이는가 | all-auto 후보만 지원하며 mixed `auto|0`을 거부한다. |
| 6 | item에 지원하지 않는 text payload나 child count가 들어오는가 | 기존 direct-text payload 검증이 그대로 적용된다. |
| 7 | 지원하지 않는 span topology나 source order가 들어오는가 | 기존 제한 topology와 placement 검사에 후보를 결합한다. |
| 8 | cross-axis placement, static position, writing context가 달라지는가 | 기존 Root HTML span profile 검증이 같은 조건을 계속 강제한다. |
| 9 | scrollable overflow나 flexible track이 auto minimum eligibility를 바꾸는가 | visible overflow와 비-flex track 조건이 유지된다. |
| 10 | style projection과 layout admission의 후보 조건이 어긋나는가 | 양쪽에서 one-fit·three-row·all-auto 및 control 조건을 대조한다. |
| 11 | 다른 Grid owner나 item이 섞이는가 | 단일 owner와 item 개수 검증을 유지한다. |
| 12 | `Minimum` contribution이 fit-content cap에서 잘리는가 | vendored Taffy의 분배 helper에 min-over-cap 회귀 테스트를 추가했다. |
| 13 | `Maximum` contribution도 cap을 넘는가 | 같은 helper의 maximum cap 유지 테스트가 추가됐다. |
| 14 | zero cap에서 overlapping span minimum이 사라지는가 | Runtime trace에서 두 span의 automatic minimum과 최종 row 합을 확인한다. |
| 15 | trace가 다른 revision 또는 owner에 귀속되는가 | completed layout과 cascade key, Grid node owner를 함께 비교한다. |
| 16 | final trace stage의 track 수·순서·used rows가 어긋나는가 | Chrome 관측 row와 최종 trace row를 대조한다. |
| 17 | nearest rounding이 1/64 px 증가를 잃거나 cap을 과하게 넘는가 | cap tolerance, 반올림 가능한 행 건너뛰기, 잔여 단위 재배분과 실제 row span matrix가 확인한다. |
| 18 | item frame은 맞지만 payload의 line count나 overflow가 틀리는가 | Runtime glyph baseline 줄 수, payload frame, content bottom을 Chrome에 대조한다. |
| 19 | unsupported revision이 부분 publish되거나 같은 session에서 복구되지 않는가 | atomic reject와 color-scheme 복귀 후 same-session recovery가 통과했다. |
| 20 | matrix pairing·실행 대상이 baseline/candidate를 섞는가 | 고정 Chrome 입력의 2,880쌍/5,760 layouts를 key와 mode별로 비교한다. |

분리된 두-fit span, one-fit auto-placement 및 기존 repeated-text row-span regression 결과는 [실행 근거](./README.md)에 기록했다. 이 검토는 제품 전 범위, WPT, 실제 기기 또는 compositor 이후 pixel parity를 주장하지 않는다.
