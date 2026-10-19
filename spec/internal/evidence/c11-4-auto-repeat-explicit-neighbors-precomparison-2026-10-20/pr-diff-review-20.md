# C11.4 auto-repeat explicit neighbor PR 변경 검토 · 20개 실패 관점

**검토 대상:** `plan/c11-4-grid-track-sizing.md`의 2026-10-20 explicit-neighbor 계획 추가, `spec/0001-conformance.md`, `spec/STATUS.md`, Chrome fixture 및 사전 비교 evidence. 계획 전용 변경이며 제품 코드는 포함하지 않는다. 계획 검토 20개와 구별되는 PR 변경 검토다. 사용자가 요청한 이 PR의 적대 검토 수는 20개로 적용한다.

최종 diff 대조 중 repeat count 산정에 쓸 track sizing function 선택이 요약되어 있던 점을 발견했다. CSS Grid §7.2.3.2에 맞춰 definite max function 우선, 없으면 definite min function, 양쪽 definite일 때 max의 min 하한을 계획과 README에 구체화했다. 이 수정 후 관련 기준 관점을 다시 확인했다.

| # | 실패 관점 | 확인한 조건 | 결과 |
| ---: | --- | --- | --- |
| 1 | 계획 전용 PR을 구현 완료처럼 표시 | 변경은 계획, oracle, status 기록이며 status와 conformance에서 구현·Simulator·WPT 미실행을 명시한다. | PASS |
| 2 | 공식 C11.4 체크나 완료 수를 조기 증가 | `spec/STATUS.md`에는 미완료 문구만 추가됐고 C11.4 체크를 바꾸지 않았다. | PASS |
| 3 | 기존 제한 auto-repeat 지원을 전부 없다고 설명 | conformance는 기존 단일 auto-repeat subset과 이번 explicit-neighbor gap을 각각 설명한다. | PASS |
| 4 | 계획 범위를 일반 CSS Grid 지원으로 넓혀 읽을 수 있음 | direct fixed `px` neighbor, 기존 fixed-minimum repeat subset, 축당 단일 auto-repeat으로 제한했다. | PASS |
| 5 | neighbor 유효성 조건이 느슨해짐 | explicit neighbor는 direct, finite, non-negative `px`만 후보에 포함하고 기타 입력은 fail-closed로 남긴다. | PASS |
| 6 | auto-repeat 앞·뒤·중간 순서 중 하나가 빠짐 | fixture의 both, prefix-only, suffix-only case가 해당 위치와 component 순서를 각각 고정한다. | PASS |
| 7 | 반복 fragment 내부 여러 track 수를 하나로 취급 | 두-track `minmax(30px,1fr) 20px` case가 component 경계·확장 순서를 기록한다. | PASS |
| 8 | definite track sizing function 선택 오류 | plan과 README 모두 definite max, 아니면 definite min, 둘 다 definite면 max를 min 아래로 낮추지 않는 규칙을 기록한다. | PASS |
| 9 | preferred/max/min/indefinite repeat count branch 혼동 | preferred/max는 content box에 맞는 최대 양수 count, minimum-only는 최소 충족 count, 전부 indefinite면 1회로 분리했다. | PASS |
| 10 | neighbor나 gap을 count 산정에서 제외 | plan은 non-repeated track 및 해당 축의 모든 gap을 포함한다. oracle의 20/10px neighbor와 8px gap 수치가 함께 고정돼 있다. | PASS |
| 11 | fit할 반복이 없어 zero repeat 또는 정상 fit으로 처리 | overflow control은 한 반복을 유지하며 40px container 밖 content overflow가 생긴다고 기록한다. | PASS |
| 12 | auto-fit을 placement 전에 collapse | 계획과 spec summary는 item placement 후 empty 여부를 판정하도록 명시한다. | PASS |
| 13 | span item이 반복 track을 비워 둔 것으로 판정 | 두-track span oracle은 점유 track 두 개와 collapse되는 세 번째 track을 분리한다. | PASS |
| 14 | collapse 때 gutter나 suffix line 위치가 틀림 | auto-fit case는 computed track sequence와 suffix item 좌표를 보존하고 인접 gutter collapse를 명시한다. | PASS |
| 15 | row/column 축 basis 혼용 | columns는 width/column-gap, rows는 height/row-gap case로 분리하고 각각 rect와 resolved track list를 기록한다. | PASS |
| 16 | 13-case 설명과 관찰 JSON이 어긋남 | JSON의 grids 목록은 표의 11 columns case와 2 rows case를 포함하며 rect와 computed rows/columns가 확인된다. | PASS |
| 17 | fraction tolerance가 모든 수치에 퍼짐 | 1/64 CSS px tolerance는 track serialization에만 적용하고 root/item rect는 정수 fixture 값과 대조한다고 README에 한정했다. | PASS |
| 18 | fixture의 line index 또는 suffix placement가 잘못됨 | before/after, prefix-only, suffix-only, two-track pattern 및 row suffix line index가 CSS와 oracle 설명에서 일치한다. | PASS |
| 19 | WPT 후보가 통과한 것으로 오인 | 고정 revision·source hash만 남기고 파일은 실행하지 않았으며 subset을 WPT pass로 세지 않는다고 명시한다. | PASS |
| 20 | screenshot이나 evidence가 Spinon/mobile 결과로 오인 | README와 status는 Chrome DOM screenshot만 제공하고 Spinon, Android/iOS surface, artifact boundary는 미검증으로 분리한다. | PASS |

검토 후 남은 제한: 이 PR에는 Spinon 계산 코드, WPT 실행, Android API 37 AVD, iOS Simulator 및 artifact boundary가 없다. 해당 근거가 없으므로 C11.4 상태와 완료 수는 미완료로 유지한다.
