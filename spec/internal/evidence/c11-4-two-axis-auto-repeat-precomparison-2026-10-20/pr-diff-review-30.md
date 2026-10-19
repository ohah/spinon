# C11.4 양쪽 축 auto-repeat 계획 PR 변경 적대 검토 30개

**대상:** PR #367 최종 계획·fixture·Chrome oracle 변경.
**검토 기준:** 계획 문서, 12-case HTML, observation JSON/PNG, 계획 실패 관점 20개 및 현재 저장소의 C11.4 제한과 Taffy 0.15.0 pin을 대조했다. 이 PR에는 제품 구현 변경이 없다.

| # | 독립 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | repeat 제한을 grid 전체에 적용해 양 축 사용을 계속 금지할 수 있다. | W3C §7.2.3.1의 제한 단위는 track list다. 계획은 columns/rows 별도 list에 하나씩 허용한다고 정확히 썼다. |
| 2 | 한 list 안에 여러 auto-repeat을 허용하는 과도한 scope 확장. | 계획은 각 list당 하나를 유지하고 list 내부 복수 repeat을 기존 unsupported grammar로 남겼다. |
| 3 | 규범과 다른 `grid-template-columns`/`rows`의 list 관계 해석. | W3C §7.2.1은 두 property를 서로 다른 columns/rows track list로 정의한다. 계획의 근거가 두 절을 함께 링크한다. |
| 4 | `auto-fill`과 `auto-fit` 선택이 두 axis 사이에서 한 값으로 합쳐지는 문제. | Fill/fill·fit/fit 및 양방향 mixed mode가 계획과 oracle case에 있다. |
| 5 | axis 반복 수를 공유하여 다른 크기 축의 결과가 무시되는 문제. | `both-fill-asymmetric`의 columns 2회·rows 3회 resolved list가 JSON에 고정돼 있다. |
| 6 | 2-track fragment의 두 번째 `20px`가 repeat basis에서 빠지는 문제. | `38 + 76n` 식은 반복당 40px minimum과 20px fixed track을 모두 포함한다. |
| 7 | 양쪽 explicit neighbor가 반복 수 계산에 빠지는 문제. | 식은 prefix 20px과 suffix 10px을 포함하고 fixture의 repeat axis에 같은 값을 둔다. |
| 8 | first/last gutter 개수 off-by-one. | 전체 track 수가 `2 + 2n`이므로 gutter `2n + 1`개라는 식을 evidence가 기록한다. |
| 9 | 200px 경계의 ceil/floor 오류가 화면만으로 숨는 문제. | 독립 계산은 n=2에서 190px, n=3에서 266px으로 구분되고 Chrome resolved list는 200px case에서 각 flex track 45px을 관찰한다. |
| 10 | 266px row count를 200px column count에 맞춰 clamp하는 문제. | definite asymmetric case의 Chrome JSON은 rows에 3회, columns에 2회를 저장한다. |
| 11 | auto-repeat 수 계산에서 `1fr` max를 definite로 잘못 취급하는 문제. | 계획/evidence는 count basis로 definite 40px minimum을 쓰고 최종 used `1fr` track size와 구분한다. |
| 12 | definite size를 min-size-only 알고리즘과 섞는 문제. | 이번 fixture는 definite 200/266px preferred axis size를 사용한다. 별도 definite-minimum-size `auto-fit` 차이는 제외 문구와 후속 링크에 남겼다. |
| 13 | empty auto-fit track collapse 전 반복 개수 자체를 잃는 문제. | `both-fit-empty` observation에는 4개의 zero repeated tracks가 남아 collapse 이후 serialization을 보존한다. |
| 14 | 빈 fit 축의 collapse가 mixed fill 축으로 번지는 문제. | 양방향 mixed empty case가 fit list는 zero, fill list는 사용 크기로 유지하는 관찰을 담는다. |
| 15 | collapsed track과 collapsed gutter를 동일 수치로 간주하는 문제. | 계획은 used track과 active/collapsed gutter를 독립 검사하도록 하고, evidence가 PNG 대신 JSON/runtime geometry를 acceptance 기준으로 둔다. |
| 16 | 한 repeated track의 점유로 나머지 반복 track이 잘못 남는 문제. | `both-fit-single`은 154px occupied track 하나와 나머지 zero tracks, 12×12 item rect를 기록한다. |
| 17 | item span이 repeated track 점유 판정에서 누락되는 문제. | `both-fit-span-two`는 line 2/span 2 배치와 남은 zero repeated tracks를 양 축에서 관찰한다. |
| 18 | 두 번째 반복 line index가 prefix와 pattern 길이를 포함하지 않아 잘못되는 문제. | `both-fit-second-repeat`은 line 4를 사용하고 first repeat zeros와 second repeat 154px track을 보존한다. |
| 19 | mixed mode에서 second-repeat item이 반대 axis 좌표/track list까지 바꾸는 문제. | 두 mixed second-repeat observation은 서로 반대 축만 collapse하며 item 위치 `(28,109)`와 `(109,28)`을 저장한다. |
| 20 | one-axis controls에서 item이 implicit row/column을 만들고 새로운 semantics를 섞는 문제. | controls의 반대 축은 명시 `20px 12px`이며 fixture/계획은 implicit track 생성을 범위에서 제외한다. |
| 21 | row case를 column 결과 복사본으로 검증해 block-axis sizing을 누락하는 문제. | `rows-fill-control`과 one-axis row observation은 별도 `gridTemplateRows`·frame을 수집한다. |
| 22 | writing mode/direction 차이가 auto-repeat 비교를 오염시키는 문제. | grid fixture는 `horizontal-tb`와 LTR을 명시하고 계획도 이 범위를 고정한다. |
| 23 | mixed/empty cases에서 item이 없는 사실을 0×0 item frame으로 오해하는 문제. | JSON은 absent item을 `null`로 보존하며 table은 “no item”이라고 따로 표시한다. |
| 24 | UA/viewport/DPR 정보가 빠져 결과를 재현할 수 없는 문제. | 원 JSON은 UA, 1280×633 viewport, DPR 1을 보존하고 README는 Agent Browser 0.31.1/arm64 host를 기록한다. |
| 25 | CSSOM resolved list에서 개별 track 값 파싱이 repeat count와 혼동되는 문제. | HTML은 DOM resolved string을 원 observation에도 저장하고 track strip은 보조 시각화다. acceptance는 JSON track list를 사용한다. |
| 26 | screenshot만으로 browser와 Spinon/native 결과를 같은 것으로 주장하는 문제. | README와 PR 본문은 이미지가 Chrome oracle 화면이고 앱/simulator를 증명하지 않는다고 명시한다. |
| 27 | fixture, observation, screenshot의 서로 다른 revision이 한 세트처럼 섞이는 문제. | 세 파일의 SHA-256을 문서에 고정했고 `shasum -c`로 모두 일치함을 확인했다. |
| 28 | 기존 pinned Taffy가 양 축을 지원하지 않는다고 가정하거나 확인 없이 보장하는 문제. | dependency는 0.15.0 pin이다. 해당 source test는 columns 2회·rows 3회 independent count를 검증하지만 계획은 project adapter/runtime 별도 검증을 요구한다. |
| 29 | 시뮬레이터 scene 수만 맞고 기존 regression 누락 또는 ID 중복이 발생하는 문제. | 계획은 저장된 기존 21 ID와 새 12 ID의 33개 exact unique set을 Android/iOS 양쪽에서 요구한다. |
| 30 | 문서 계획을 구현 완료/공개 지원으로 오인하거나 이후 roadmap이 고정되는 문제. | C11.4 check/count는 유지하고 구현 후 계약 갱신, 별도 Tailnet sync, 각 PR의 독립 20/30 review와 Rebase and merge를 gate로 둔다. |

사전 검토에서 발견한 계획 checklist의 미표시된 완료 상태와 one-axis control axis 설명을 수정한 뒤 이 최종 diff를 검토했다. 이 검토 파일 추가 후에도 전체 PR diff를 다시 확인했으며 변경은 이 검토 기록뿐이다. 30개 항목은 계획 자체의 20개 표를 복사하지 않고 실제 최종 PR 파일·관찰값을 서로 다른 실패 경로로 확인한다.
