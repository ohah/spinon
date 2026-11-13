# C11.4 three-fit row-span mixed minimum 계획 PR diff 검토 30개

대상 diff: three-fit row-span item별 mixed `min-height:0|auto` 계획, Chrome 사전 비교 자료, `spec/STATUS.md`, `spec/public/roadmap.html`, C11.4 계획 인덱스. 이 검토는 계획 PR의 변경분을 대상으로 하며 제품 구현이나 RuntimeSession·모바일 동작을 검증한 것이 아니다.

| # | 실패 관점 | diff 확인 및 판정 |
|---:|---|---|
| 1 | 계획을 C11.4 전체 구현 완료로 읽을 수 있는가 | 문서와 공개 칸반은 후보를 계획 완료·구현 전으로 표시한다. C11.4/C11 체크는 미완료다. 통과. |
| 2 | 제한 slice만으로 공식 완료 수를 올리는가 | 상태 대장과 칸반 모두 `78/214`를 유지한다. 통과. |
| 3 | numeric internal contract가 범위 확장과 함께 상승하는가 | 계획과 상태 대장은 `0.1.0`을 그대로 둔다. 통과. |
| 4 | CSS 표준 판본을 확정 명세로 과장하는가 | 2025 CRD의 세 조항을 pin하고 최종 CSSWG 명세가 아님을 명시한다. 통과. |
| 5 | fit-content track 개수가 다른 profile까지 admission되는가 | 제한 목표는 정확히 세 `fit-content()` member를 가진 six-track list다. 다른 fit count는 제외된다. 통과. |
| 6 | explicit/implicit row 개수가 혼동되는가 | 한 explicit control row와 정확히 여섯 implicit row를 각각 검증하도록 고정한다. 통과. |
| 7 | 6개 위치 중 일부 fit-position 조합이 누락되는가 | 세 position의 20개 조합 모두를 720-list coverage 및 RuntimeSession sample 선택 기준에 포함한다. 통과. |
| 8 | cap 값 순서가 canonicalization으로 사라지는가 | `0px`, `10px`, `20%` cap의 여섯 순서를 유지한다. 통과. |
| 9 | intrinsic tail의 순서별 동작이 합쳐지는가 | `min-content`, `max-content`, `auto`의 여섯 순서를 독립 key로 보존한다. 통과. |
| 10 | percentage cap이 border box를 basis로 쓰는가 | 42×222 border box 안에 40×220 content box를 고정하고 220px basis를 관측한다. 통과. |
| 11 | span-2 five-item topology의 마지막 mask가 빠지는가 | A–E semantic item 32개 mask와 여섯-row coverage를 모두 요구한다. 통과. |
| 12 | span-2/span-3 혼합 topology에서 span-3 item이 잘못 mask 처리되는가 | A–D 16개 mask와 각 item의 실제 covered rows를 검증한다. 통과. |
| 13 | DOM reverse가 item별 style을 다른 semantic item에 옮기는가 | forward/reverse를 함께 실행하며 mask bit는 semantic ID에 고정한다. 통과. |
| 14 | mask bit 순서가 우연히 DOM order에 종속되는가 | mask bit 순서를 A, B, C, D, E semantic ID 순으로 문서화한다. 통과. |
| 15 | topology별 mask count가 예상보다 작거나 큰가 | topology scene 분모와 32/16 distinct masks를 독립 verifier가 확인한다. 통과. |
| 16 | explicit-row empty control이 mask에 섞이는가 | 모든 positive scene에서 control `min-height:0px`를 고정한다. 통과. |
| 17 | alignment 차이가 mask 비교를 오염시키는가 | `start`와 `stretch`를 모든 track-list/mask 조합과 교차하고 pair key를 분리한다. 통과. |
| 18 | `min-height:auto`를 조건 없이 content-based minimum으로 처리하는가 | computed overflow·span track minimum·flexibility 조건을 item마다 평가한다. 통과. |
| 19 | span에 flexible track이 있어도 positive admission되는가 | positive multiset에 flexible track이 없고, profile 외 flexible 입력은 negative admission으로 둔다. 통과. |
| 20 | payload floor가 잘못된 text wrapping을 가려 주는가 | text, font, line-height, line rectangles와 payload frame을 관측값에 남긴다. 통과. |
| 21 | item frame과 covered-row 기하가 서로 모순돼도 통과하는가 | 각 span frame을 실제 covered row 합과 `0.01 CSS px` 안에서 대조한다. 통과. |
| 22 | 유효한 visible overflow를 0으로 clamp하는가 | overflow 1,344 scenes·mixed 1,236 scenes와 20px maximum을 기준 결과로 기록하고 0을 강제하지 않는다. 통과. |
| 23 | representative screenshot이 border 밖 paint를 증명하지 못하는가 | screenshot은 실제 1px red Grid border와 끝을 10px 넘는 `DDDD` sentinel을 보여 준다. Chrome reference임을 구분한다. 통과. |
| 24 | 기존 all-zero/all-auto profile 회귀가 새 mask matrix에 가려지는가 | 별도 기존 uniform oracle 11,520 endpoint를 row/item/payload geometry까지 비교한다. 통과. |
| 25 | reverse/alignment 비교가 topology나 semantic item을 잘못 짝짓는가 | pair key에 track list·alignment·mask·topology family를 넣고 semantic ID로 geometry를 비교한다. 통과. |
| 26 | 작은 RuntimeSession sample을 전체 oracle과 혼동하는가 | 36 list × 192 scenes = 6,912 sample이라고 명시하고 138,240 full runtime 실행으로 부르지 않는다. 통과. |
| 27 | 계획한 sample이 위치·cap·tail 차원을 충분히 포함하지 않는가 | rank 수식이 20 fit-position set과 여섯 cap/tail 순서를 각각 모두 포함한다. 통과. |
| 28 | fail-closed 이후 partial output이나 stale revision을 재사용하는가 | invalid profile의 atomic rejection, same-session valid revision recovery와 rounding 경계를 별도 gate로 둔다. 통과. |
| 29 | 모바일 검증을 기본 route smoke로 대체하거나 실행 자원을 남기는가 | 계획은 실제 mixed Root HTML scene을 API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 보도록 하며 앱·기기 종료와 disposable AVD 삭제를 요구한다. 이번 계획 PR은 simulator 실행을 완료로 표시하지 않는다. 통과. |
| 30 | evidence·로드맵·빌드 상태를 과장하는가 | SHA-256 manifest가 현재 precomparison 파일에서 검증된다. roadmap은 후보·20/20 계획 검토·구현 전을 표시한다. 문서 빌드는 전체 저장소의 1,567 dead-link 진단으로 실패했으나 신규 후보 경로는 출력에 없으며 green으로 기록하지 않는다. 통과. |

결론: 계획 PR diff는 후보 범위, 독립 Chrome oracle, 비완료 상태와 이후 실행·정리 관문을 일치시킨다. 이 30개 검토는 구현/runtime 검토 20개를 대신하지 않는다.
