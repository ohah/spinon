# C11.4 two-fit row-span mixed minimum · 계획 PR diff 검토 30개

검토 대상은 계획, 독립 Chrome 사전 비교, 작업 fixture, 공식 상태 대장, C11.4 순차 계획, 공개 로드맵 변경이다. 계획 자체의 실패 관점 20개와는 별개로, 이번 PR diff에서 생길 수 있는 계약·근거·표시 오류를 한 항목씩 대조했다.

| # | 실패 경로 | 확인·조치 | 판정 |
|---:|---|---|---|
| 1 | 계획 문구가 이미 완료된 RuntimeSession 기능처럼 보임 | 제목과 상태에 구현 전 후보, RuntimeSession 미실행을 명시 | 통과 |
| 2 | C11.4 또는 C11을 완료로 오표시 | 체크박스는 미완료 그대로 두고 별도 후보 기록만 추가 | 통과 |
| 3 | 공식 완료 수가 사전 비교 때문에 증가 | `78/214`를 STATUS·계획·로드맵에서 유지 | 통과 |
| 4 | 내부 숫자 계약이 조사만으로 변경 | `0.1.0` 유지, 새 API·버전 계약을 주장하지 않음 | 통과 |
| 5 | 기존 projection admission을 신규 기능 범위로 오해 | 이미 허용된 입력과 전용 RuntimeSession 근거 부재를 구분 | 통과 |
| 6 | fit count 2가 전용 mixed runner를 이미 가진다고 서술 | 현재 RuntimeSession two-fit uniform 3,840 scene과 mixed matrix 부재를 분리 | 통과 |
| 7 | Chrome oracle 결과를 제품 비교 결과로 확대 | 모든 산출물에서 Chrome CSS Grid 사전 비교로 한정 | 통과 |
| 8 | scene 산식에서 topology mask 수 누락 | `480 × 2 × 2 × (32 + 16) = 92,160`을 기록과 대조 | 통과 |
| 9 | 고유 track list 수 계산 오류 | 15 fit 위치 × 8 ordered cap assignment × 4 tail order = 480 확인 | 통과 |
| 10 | percentage cap basis가 모호함 | definite `220px` Grid block basis를 고정 | 통과 |
| 11 | 표본 cap을 허용 값 전체로 과장 | 직접 px/% 표본이며 allowlist가 아니라고 명시 | 통과 |
| 12 | track multiset 변형이 matrix에 섞임 | 두 fit, 두 min-content, 한 max-content, 한 auto member를 고정 | 통과 |
| 13 | topology별 item/mask 수 혼합 | 다섯-item span2Six는 32 mask, 네-item span2Span3Six는 16 mask | 통과 |
| 14 | reverse DOM order가 minimum mask owner를 바꿈 | mask bit를 semantic ID에 고정하고 양 source order 비교 | 통과 |
| 15 | explicit-row control이 mixed mask에 포함 | 10px empty control은 계속 `min-height:0` | 통과 |
| 16 | endpoint 수가 전체 scene 수와 혼동 | all-zero·all-auto 각각 3,840 scene으로 구분 | 통과 |
| 17 | 보관 scene과 검증 결과의 분모가 다름 | compressed observation hash와 verifier의 92,160 identity 수 확인 | 통과 |
| 18 | source-order 또는 alignment pair를 일부만 비교 | 각 46,080 pair의 비교량과 관계값을 verifier JSON에서 확인 | 통과 |
| 19 | text payload overflow를 item frame overflow로 취급 | payload, line rect, item frame, track sum, Grid edge를 분리 | 통과 |
| 20 | 실제 경계 밖 장면을 필터링 | zero-cap visible overflow 320 scene과 최대 30px를 보존 | 통과 |
| 21 | overflow control을 모든 cap/order의 일반 규칙으로 확대 | 320 scene의 zero-cap, stretch, span2Six 조건을 고정 | 통과 |
| 22 | unsupported CSS가 새 계획에서 조용히 허용 | 고정 profile 밖 grammar 및 hidden/scrollable overflow는 범위 밖으로 둠 | 통과 |
| 23 | RuntimeSession 구현 검증이 representative sample로 대체 | 계획의 gate는 full 92,160 scene과 동일 observation hash 비교 | 통과 |
| 24 | automatic minimum이 선언값만으로 판정 | eligibility·covered track·scrollable overflow·contribution trace를 요구 | 통과 |
| 25 | rounding tolerance가 CSS 차이를 감춤 | row/frame `0.01px` 기준과 1/64px·trace 예외를 분리 | 통과 |
| 26 | rejection 뒤 이전 상태가 부분 publish | stale/missing/duplicate/wrong-owner trace와 same-session recovery를 gate에 포함 | 통과 |
| 27 | 플랫폼 smoke를 feature parity로 과장 | Android/iOS는 후속 기본 Root HTML route smoke로 한정하고 pixel parity를 배제 | 통과 |
| 28 | 작업용 simulator/browser가 끝난 뒤 남음 | 이번 확인의 부팅 iOS simulator 0, Agent Browser session 0; 임시 서버·브라우저 설정·빌드 산출물 제거 | 통과 |
| 29 | 로드맵 후보 표시가 STATUS와 다르거나 공식 카운트를 변경 | two-fit은 NEXT 카드, C11.4는 미완료·`78/214`; 코드 없는 후보 카드는 공식 task count와 구분 | 통과 |
| 30 | 진행 중 C11.4 카드가 할 일 목록에도 중복 계산 | Agent Browser에서 중복을 발견해 `inProgressCodes` 제외를 적용하고 desktop/mobile 재확인; 문서 전체 build의 기존 dead link 실패는 별도로 기록 | 수정 후 통과 |

**결과:** 서로 다른 30개 diff 실패 경로를 검토했다. 30번에서 발견한 칸반 중복 집계는 수정했고, Agent Browser에서 할 일 133, 진행 중 1, 외부 대기 3, 완료 78을 재확인했다. 공식 원장은 여전히 `78/214`다. 전체 RSPress 문서 build는 기존 내부 근거 파일들의 dead link 때문에 실패했으며, 변경 문서의 32개 상대 링크는 개별 검사에서 누락 0개였다. 이 실패를 빌드 통과로 간주하지 않는다.
