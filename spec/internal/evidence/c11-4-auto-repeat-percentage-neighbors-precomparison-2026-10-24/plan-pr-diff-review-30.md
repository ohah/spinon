# C11.4 percentage auto-repeat neighbor 계획 PR diff 적대 검토 30개

2026-10-24에 계획 PR의 실제 파일 diff를 대상으로 서로 다른 실패 관점 30개를 검토했다. 계획 자체의 구현 전 20개 검토와 분리했다. 발견은 수정한 뒤 관련 항목을 다시 확인했다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | Chrome patch version을 User-Agent string만으로 단정 | navigator.userAgentData의 fullVersionList에서 156.0.8078.48을 확인해 metadata에 기록했다. |
| 2 | Agent Browser의 platform/architecture가 실제 실행 호스트와 다름 | high-entropy metadata의 macOS 26.5.1, ARM64, DPR 1을 함께 보존했다. |
| 3 | fixed-px precedent 링크가 존재하지 않는 경로를 가리킴 | 기존 README가 있는 auto-repeat-neighbors 경로로 수정했다. |
| 4 | 2025 CRD가 목표 동작이 아닌 오래된 편집 초안 | 고정된 W3C CSS Grid 2025 CRD URL을 사용하고 이번 규범 규칙을 section으로 좁혔다. |
| 5 | percentage basis가 definite axis의 content size가 아님 | 136px border-box에 8px padding과 2px border를 둬 116px content basis를 별도 계산했다. |
| 6 | percentage size를 pixel 정수로 반올림해 count 기대값을 바꿈 | percentage 계산은 double 산술로 독립 도출하고 fractional CSSOM에 1/64px 허용치를 둔다. |
| 7 | repeat count 공식에서 neighbor track을 누락 | 양쪽 10%/5%, prefix-only, suffix-only, overflow cases를 고정했다. |
| 8 | repeat count 공식에서 gutters를 누락 | 4px gap을 fixture와 expectation에 고정해 inter-track gutter를 계산에 포함했다. |
| 9 | row percentage basis가 column width를 참조 | row-fill과 row-border-box scene에서 block-axis size를 따로 측정한다. |
| 10 | columns와 rows에 같은 projection 결과를 재사용 | CSSOM observation은 각 axis의 gridTemplateColumns/gridTemplateRows를 개별 기록한다. |
| 11 | percentage sum이 100%를 넘으면 zero repeats 반환 | 80%/30% controls에서 규범상 minimum one repeat와 overflow를 관측했다. |
| 12 | auto-fit 반복 수를 collapse 뒤 track 수에서 계산 | expected count와 post-placement used tracks를 분리해 기록했다. |
| 13 | auto-fit empty와 occupied scene을 동일하게 취급 | empty 및 first-track item 장면을 columns/rows에 각각 둔다. |
| 14 | collapsed track 주변 gutter가 살아남음 | auto-fit used track lists에 zero-width repeat tracks를 포함하고 frame을 비교한다. |
| 15 | 0% neighbor가 repeat-count UA floor 1px를 받음 | 0% prefix와 40px repeated minimum의 used value를 서로 다른 track으로 검사한다. |
| 16 | percentage neighbor 허용이 zero/negative/non-finite 경계를 약화 | plan은 finite nonnegative gate를 필수로 두고 negative/internal non-finite를 negative controls로 남겼다. |
| 17 | indefinite/cyclic percentage basis에서 computed value만 보고 허용 | plan은 해당 axis definite/positive basis gate를 required condition으로 명시했다. |
| 18 | calc neighbor 지원을 순수 percentage slice에 끼워 넣음 | calc, intrinsic, flex, fit-content neighbor를 명시적으로 제외했다. |
| 19 | Taffy가 definite percentage neighbor를 실제 count에 포함하지 않음 | source 추적과 13-scene runtime acceptance를 dependency-change 조건으로 계획했다. |
| 20 | border/padding 계산과 screenshot outer dimensions 불일치 | expected content basis와 Chrome track list 모두 116px 입력을 확인했다. |
| 21 | expected repeat count가 track list 길이에서 도출되지 않음 | JSON에 prefix/suffix track count를 추가했고 validator가 expanded count를 검사한다. |
| 22 | track list order가 맞지 않아도 숫자 합만 비교 | validator가 CSSOM ordered array element를 차례대로 비교한다. |
| 23 | fractional used-value rounding이 구현 오류로 오인 | spec expectation과 Chrome used values를 분리하고 1/64 CSS px 허용치를 명시했다. |
| 24 | item frame origin이 content-box origin으로 혼동 | 관측은 grid border rect를 기준으로 하고 기대 frame도 outer origin으로 표기한다. |
| 25 | fixture observation JSON이 oracle source HTML과 다른 scene을 담음 | oracle HTML과 browser observation을 함께 보존하고 validator가 13 IDs를 확인한다. |
| 26 | screenshot이 전체 scene을 보여주지 않음 | 1280×633 viewport와 1280×1532 full matrix capture를 모두 보존한다. |
| 27 | hash manifest가 screenshot 또는 browser metadata를 제외 | README, oracle, expected JSON, observation, metadata, 두 screenshot, validator를 sums에 넣는다. |
| 28 | SHA256SUMS 자기 자신을 포함해 검증 순환을 만듦 | manifest는 자신을 제외한 evidence artifact만 대상으로 생성한다. |
| 29 | Python validator가 저장소 runtime에서 지원하지 않는 문법 사용 | Python 호환성 검토에서 zip strict option을 제거하고 13/13 재실행했다. |
| 30 | 제한된 oracle과 상세 계획이 상위 계획 이력에서 제품 지원으로 승격 | README와 focused plan은 browser-only 관측과 runtime acceptance를 분리하고, 상위 계획의 이 항목은 계획 상태·미완료 범위로 연결하며 C11.4 checkbox와 `72/213`을 유지한다. |

결과: PR diff에서 발견한 4개 구체 항목(버전 출처, 선행 자료 경로, scene-count 검증, validator Python 호환성)을 반영하고 1·3·21·29번을 재검토했다. 구현 PR의 20개 runtime 검토와는 별도다.
