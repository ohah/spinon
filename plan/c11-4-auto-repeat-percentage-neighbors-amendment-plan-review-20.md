# C11.4 percentage auto-repeat neighbor 계획 보정 검토 20개

2026-10-24 계획 보정본을 구현 재개 전에 기존 계획 검토와 분리해 20개의 실패 관점으로 다시 확인했다. 이 검토는 원래 계획의 20개 표를 재사용하지 않으며, implementation/runtime 검토와 PR diff 검토도 대체하지 않는다.

| # | 독립 실패 관점 | 보정본에서 확인한 기준 |
|---:|---|---|
| 1 | Chrome 호환성 관측을 CSS 규범값으로 승격 | 필수 기대값은 CSS Grid 2025 CRD 계산으로 두고 Chrome CSSOM은 별도 compatibility observation으로 둔다. |
| 2 | auto-fit 반복 개수를 collapse 뒤에 계산 | 네 반복 count를 placement 전 확정하고 빈 반복의 collapse는 post-placement 단계로 분리한다. |
| 3 | collapsed track 옆 gutter를 flex 공간에서 빼먹음 | §7.2.3.2에 따라 빈 반복의 양쪽 gutter를 제거한다. 해당 장면에는 prefix와 점유된 첫 repeat 사이 gutter 하나만 남는다. |
| 4 | prefix와 suffix percentage track을 계산에서 빠뜨림 | 24px prefix, 12px suffix, 240px content basis를 반복 count와 flex 분배에 모두 포함한다. |
| 5 | 남은 repeat 수만으로 196px을 정당화 | `240 - 24 - 12 - 4 = 200px`을 independently 계산하며 browser가 낸 196px은 별도 값이다. |
| 6 | 처음 배치된 2px item frame만으로 track edge를 추정 | 시작 정렬의 x/y=38px은 196px과 200px 모두에서 같다고 기록하고 track trace를 수용값으로 둔다. |
| 7 | suffix item x/y=238px을 남은 gutter의 증거로 오해 | 규범의 200px+한 gutter와 Chrome의 196px+두 gutter가 모두 238px을 만들므로 이 probe는 판별력이 없다고 명시한다. |
| 8 | end-alignment probe 결과가 없어 track 끝 차이를 놓침 | Chrome 156 DOM-only 진단의 x/y=232px과 규범 계산 x/y=236px을 별도 기록한다. |
| 9 | browser-only 진단이 일반 CSS 지원을 넓힘 | end-alignment 진단은 reference 측정에만 쓰며 `justify-self`/`align-self`를 runtime 지원 범위에 추가하지 않는다. |
| 10 | Chrome CSSOM 자료를 규범 계산으로 재사용 | validator는 W3C 입력 산술과 Chrome raw observations를 서로 다른 비교로 검사한다. |
| 11 | 반복 횟수와 expanded track 수를 혼동 | 원래 prefix/repeat/suffix track order와 0px collapsed repeat entries를 보존해 비교한다. |
| 12 | 비정수 percentage 결과의 반올림 차이를 숨김 | fractional scene은 규범 계산과 Chrome CSSOM을 각각 1/64 CSS px 허용치로 확인한다. |
| 13 | overflow 장면에서 반복 횟수가 0이 됨 | 모든 후보가 넘칠 때 repeat-to-fill 규칙의 최소 1회를 유지한다. |
| 14 | 0% explicit neighbor가 auto-fit track처럼 collapse | 0% prefix는 explicit non-repeating track으로 남고 repeat-floor와 별도 유지한다. |
| 15 | 빈 auto-fit 장면에서 flex 분모가 0이 됨 | 모든 반복 track이 비면 track과 gutter는 0이 되며 반복 count는 4로 보존한다. |
| 16 | column 계산을 row에 재사용 | rows와 columns의 13-scene 입력·track list·item frame을 따로 검사한다. |
| 17 | 136px border-box를 percentage basis로 사용 | 2px border와 양쪽 8px padding을 빼 116px content axis를 사용한다. |
| 18 | Chromium main 소스를 Chrome 156 바이너리와 동일시 | source 페이지는 current `main` 관측으로 한정하며 exact binary revision 또는 원인 분석이라고 주장하지 않는다. |
| 19 | 특정 CRD 장면 차이를 모든 브라우저 동작으로 일반화 | 단일 Chrome 156 고정 입력의 관측 차이로만 분류하고 다른 engine/WPT 통과를 주장하지 않는다. |
| 20 | 제한 runtime fixture를 C11.4 완료로 승격 | Android/iOS 각 13/13은 해당 slice만 판정하며 C11.4 checkbox와 `72/213`은 유지한다. |

재검토에서 발견한 핵심 결함은 기존 표가 Chrome 196px을 규범 기대값으로 복사했고, suffix item x/y=238px probe의 판별력을 과대해석한 점이다. 두 항목을 계획·validator·증거 자료에서 분리해 고쳤다. End-alignment 진단도 browser-only로 제한했다. 이 표는 PR 변경 diff 적대 검토 30개와 구현 후 새로운 runtime 검토 20개를 대신하지 않는다.
