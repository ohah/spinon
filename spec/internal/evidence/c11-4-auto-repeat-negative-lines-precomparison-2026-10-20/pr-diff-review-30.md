# 계획 PR 최종 diff 적대 검토 30개

검토 대상은 C11.4 auto-repeat/negative line 집중 계획, 상위 계획 링크, 18-case Chrome oracle fixture와 관찰·validator·증거 문서다. 각 관점은 실패 경로 또는 계약 경계를 달리해 최종 staged diff에서 확인했다.

| # | 적대적 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 PR이 구현 완료나 공식 지원 범위를 암묵적으로 선언하는가 | 계획·Chrome 사전 비교만 완료로 표시하고 runtime·simulator는 미수행이라고 명시한다. C11.4 checkbox와 완료 수도 유지한다. 통과. |
| 2 | 규범 근거가 최신 문서의 움직이는 링크에만 의존하는가 | 판정 기준은 날짜가 고정된 CSS Grid Level 1 2025 CRD이며 Level 2는 교차 확인으로만 둔다. 통과. |
| 3 | auto-repeat 횟수에서 `1fr` 최대값을 definite로 취급하는가 | 고정 최소 track을 기준으로 count를 계산한다. `minmax(40px,1fr)`의 최소 40px을 명시한다. 통과. |
| 4 | columns count 산식이 prefix, suffix, 모든 gutter를 빠뜨리는가 | 한 repeat track과 앞뒤 고정 track을 포함한 `38 + 48n` 산식 및 134/200/230px 기대값이 일치한다. 통과. |
| 5 | rows count 산식이 columns gap이나 track 크기를 잘못 재사용하는가 | 독립 `26 + 36n` 산식과 134/180/206px 입력에서 3/4/5회 기대값을 둔다. 통과. |
| 6 | 반복 수가 최소 1회인 조건 또는 repeat-only 축에서 틀리는가 | repeat-only column 200px 입력을 별도 포함하며 독립 validator가 축별 count를 계산한다. 경계 직전/직후 입력은 runtime 계획에 남긴다. 통과. |
| 7 | 음수 CSS line이 authored repeat vector 길이에서 계산되는가 | 기준은 확장된 explicit track count E이며 정적 normalizer/Taffy 순서 차이를 구현 후보로 명시한다. runtime 재현은 미완료라고 제한한다. 통과. |
| 8 | 음수 line의 1-based/0-based 변환이 off-by-one인가 | `E + 1 + k` 식과 `-1 → E` 예를 함께 기록하고 case별 기대 boundary를 validator가 재계산한다. 통과. |
| 9 | 양수 line도 repeat 확장 전 vector에서 계산되는가 | 양수 `6 / 7` control을 두고 `k - 1` 식과 trailing implicit track을 독립 검사한다. 통과. |
| 10 | line 0이 일반 boundary로 수용되는가 | line 0은 규범상 invalid로 계획에 기록하고 성공 oracle에는 섞지 않으며 이후 오류 회귀를 요구한다. 통과. |
| 11 | reversed line pair를 정렬해 저자 입력 의미를 잃는가 | `-5 / -7` 입력과 관찰을 보존한다. 기대 boundary는 각각 원래 순서를 유지한다. 통과. |
| 12 | negative line과 span이 leading prefix를 가로지를 때 extent 계산이 빠지는가 | 두 leading track과 `-8 / -5` span을 포함하고 used track/frame을 확인한다. 통과. |
| 13 | leading `grid-auto-*` 목록이 잘못된 방향으로 순환하는가 | `[31,47]`에서 왼쪽→오른쪽 `[31,47]`, 가장 가까운 값 47을 columns와 rows에서 별도로 고정한다. 통과. |
| 14 | trailing implicit track이 leading 규칙과 혼동되는가 | 양수 line control이 trailing column 31px을 만들고 forward pattern을 검증한다. 통과. |
| 15 | `auto-fit`이 `auto-fill`보다 적은 repetition count를 산출하는가 | 비어 있는 반복 구간을 접더라도 count는 같다고 분리하고 기대값에 원래 반복 슬롯을 보존한다. 통과. |
| 16 | 점유된 `auto-fit` repeat도 collapse되는가 | 한 case에서 leading item과 마지막 repeat item을 함께 둬 두 빈 슬롯만 collapse되는 관찰을 확인한다. 통과. |
| 17 | 빈 반복 슬롯의 gutter가 Chrome 동작을 따라 규범값으로 들어가는가 | 알려진 Chrome prefix gutter 편차를 별도 기재하고 validator는 해당 auto-fit frame을 규범 기대값으로 쓰지 않는다. 통과. |
| 18 | border/padding/box sizing이 definite content-box 기준을 바꾸는가 | fixture grid는 border/padding 없이 px width/height를 고정하고 validator가 실제 grid rect와 대조한다. 통과. |
| 19 | writing mode, direction 또는 단위 변환이 암묵적으로 scope에 포함되는가 | `horizontal-tb`, LTR, fixed non-negative px로 scope를 제한하고 다른 프로필은 제외한다. 통과. |
| 20 | 두 축에서 한 축의 count나 auto-repeat metadata가 다른 축을 덮는가 | columns/rows 각각 다른 min/gap/count를 쓰는 fill/fit 결합 case와 축별 독립 계산이 있다. 통과. |
| 21 | fixture ID 중복이나 누락 때문에 18개를 더 적게 검증하는가 | validator가 기대/관찰 길이와 관찰 ID 유일성·기대 ID 존재 여부를 단언한다. 18 IDs가 일치한다. 통과. |
| 22 | computed track 문자열에서 px 값을 잘못 토큰화하는가 | px track 전부를 추출하고 track 개수, leading/trailing 크기, 반복 슬롯을 검사한다. 현재 고정 fixture의 px 출력과 맞는다. 통과. |
| 23 | fractional geometry 차이를 exact 값으로 오인하거나 과도하게 허용하는가 | 정수 track/count는 exact이고 frame만 CSS px 1/64 허용 오차를 둔다. 환경 viewport/DPR도 고정한다. 통과. |
| 24 | item이 지정 line에 놓였는지 computed placement만 보고 오판하는가 | `auto-fill`은 line 식과 실제 item 시작 좌표를 대조한다. `auto-fit` frame은 Blink 편차 때문에 별도 제한한다. 통과. |
| 25 | Chrome oracle 화면을 Spinon runtime 결과로 게시하는가 | README와 계획에서 Chrome을 reference/oracle로 명시하며 simulator 실행은 후속 단계로 남긴다. 통과. |
| 26 | static source-order 관찰을 재현된 runtime 버그로 과장하는가 | normalizer/Taffy source 경계를 기록하되 실제 Spinon runtime failure는 주장하지 않는다. 통과. |
| 27 | 큰 extent, signed overflow, 오류 또는 cap 초과에서 partial layout이 나오는가 | checked arithmetic·기존 cap·fail-closed와 no-partial-output 회귀를 구현 계획에 두고 전역 allocator 보장은 주장하지 않는다. 통과. |
| 28 | 기존 matrix case와 새 case를 이중 집계하거나 플랫폼 증거를 합치는가 | 기존 106과 auto-repeat 33을 exact subset으로 유지하고 새 18 unique IDs를 추가한다. Android AVD와 iOS Simulator 결과를 별도 기록하도록 한다. 통과. |
| 29 | 브라우저·시뮬레이터 증거가 장치 및 캡처 시점보다 넓은 주장을 하는가 | Agent Browser Chrome 환경, AVD PixelCopy, iOS Metal readback을 구분하고 physical device/GPU/compositor 주장을 제외한다. 통과. |
| 30 | 이 PR의 라벨·본문·필수 checks·merge 절차가 저장소 계약과 어긋나는가 | PR 본문은 템플릿에 따라 계획/근거/미완료 범위를 기록하고 영역·유형 라벨, 최신 required checks 확인, Rebase and merge를 요구한다. 통과. |

## 판정

최종 diff에서 이 30개 관점의 미해결 결함은 발견되지 않았다. 독립 validator와 artifact hash를 재검증했다. 이 판정은 계획 PR에만 해당하며 구현 및 simulator 동작 검토를 대체하지 않는다.
