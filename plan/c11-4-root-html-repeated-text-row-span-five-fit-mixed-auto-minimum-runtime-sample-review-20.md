# C11.4 five-fit mixed minimum RuntimeSession sample plan amendment review 20개

검토 대상은 [구현 계획](./c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum.md)의 RuntimeSession mixed-mask 범위 조정이다. Chrome 전체 oracle, sample 선택·독립 검증, 기존 uniform endpoint 회귀와 완료 판정이 서로 다른 증거 등급으로 기록됐는지 확인했다. 이 문서는 계획 검토이며 아직 통과하지 않은 RuntimeSession 구현 시험을 대신하지 않는다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 비용 절감을 위해 Chrome oracle도 줄임 | PASS — Chrome 414,720 scenes와 원본 압축 hash를 그대로 보존한다. |
| 2 | sample이 원본 oracle과 다른 임의 자료임 | PASS — verifier가 원본 scene key와 manifest hash로 subset 선택을 재구성한다. |
| 3 | sample scene 수가 선택 규칙 산식과 다름 | PASS — 대표 18 lists × 176 scenes와 회전 102 lists × 8 scenes가 총 3,984다. |
| 4 | 120 typed-cap order 중 일부만 sample에 들어감 | PASS — 120개 list 전부를 선택하며 sample metadata와 verifier가 수를 단언한다. |
| 5 | fit-position 또는 singleton-tail 한 축이 빠짐 | PASS — 대표 18 문맥은 여섯 fit-position × 세 tail의 전체 조합이다. |
| 6 | 대표 문맥에서 topology family가 누락됨 | PASS — span2Six와 span2Span3Six 각각의 중간 mask를 확인한다. |
| 7 | mixed mask가 대표 문맥에서도 일부만 확인됨 | PASS — 모든 대표 문맥에서 각각 30개와 14개 partial mask를 포함한다. |
| 8 | 대표 외 102개 cap order의 mask 선택이 고정 한 값으로 편향됨 | PASS — cap-order index에 따른 topology별 회전 mask를 사용하고 verifier가 규칙을 재구성한다. |
| 9 | DOM order reversal이 sample에서 사라짐 | PASS — 선택된 각 mask는 forward/reverse를 모두 보유한다. |
| 10 | `align-content:stretch` 또는 `start`가 빠짐 | PASS — 각 선택 mask는 두 alignment 값을 모두 포함한다. |
| 11 | mask bit와 semantic item ID가 다른 순서로 해석됨 | PASS — 기존 A–E/A–D bit 순서를 유지하고 Rust metadata test가 `min-height` 값을 대조한다. |
| 12 | baseline/all-auto endpoint를 mixed sample에 억지로 중복 | PASS — endpoint는 별도 34,560-scene uniform RuntimeSession matrix에 두며 2,160 lists를 모두 포함한다. |
| 13 | 기존 endpoint runtime 증거가 단순 Chrome observation으로 혼동됨 | PASS — 근거 README와 계획은 기존 RuntimeSession 실행 34,560/34,560 및 네 청크를 명시한다. |
| 14 | sample floor/overflow 예상값이 RuntimeSession 결과를 미리 결정 | PASS — Chrome에서 관측한 subset 기대값을 metadata test가 검증하고 실제 RuntimeSession 결과는 matrix가 별도로 비교한다. |
| 15 | 과거 중단된 전체 실행을 통과로 표시 | PASS — 3,840개 처리 후 약 5분 30초에 중단된 측정이며 완료 검증이 아니라고 기록한다. |
| 16 | 약 10시간 추정이 보장된 실행시간처럼 읽힘 | PASS — 처리율 기반 추정임을 명시하고 sample은 반복 개발 검증용으로 한정한다. |
| 17 | 기존 36개 2,160-list 청크 경계가 sample에 남음 | PASS — 테스트 청크를 120개 list의 0–40, 40–80, 80–120으로 다시 나눈다. |
| 18 | 기본 앱 Android/iOS smoke가 feature matrix를 대신함 | PASS — plan과 evidence README가 앱 smoke, Chrome oracle, RuntimeSession sample을 분리한다. |
| 19 | sample 통과만으로 atomic rejection/recovery를 생략 | PASS — rejection과 same-session recovery는 별도 필수 runtime gate다. |
| 20 | 축소된 RuntimeSession denominator로 C11.4 완료 또는 공식 수를 올림 | PASS — 상태는 구현 중/검증 미완료이며 C11.4/C11과 `78/214`는 그대로 둔다. |

계획 amendment 판정: sample 조정의 근거와 증거 범위 구분은 충분하다. 검토 당시 미완료였던 metadata 및 RuntimeSession sample gate는 2026-11-12 후속 실행에서 통과했다. 혼합 sample 3,984개 장면과 120개 list를 확인했으며 집계 순서는 `[min-height:0, min-height:auto]`다. 세 청크에서 예상/실제 floor shortfall은 `[1180, 0]`, `[120, 0]`, `[88, 0]`, Grid-end overflow는 `[2, 0]`, `[2, 0]`, `[0, 0]`으로 일치했다. positive/rejection recovery 전체, affected-crate 회귀와 플랫폼 smoke는 별도 gate로 남고, 이 계획 판정은 구현/runtime 20관점 검토나 실제 PR diff 검토를 대체하지 않는다.
