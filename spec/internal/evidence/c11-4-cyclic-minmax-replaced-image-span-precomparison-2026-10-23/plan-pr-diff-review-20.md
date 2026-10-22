# C11.4 replaced image cyclic minmax 계획 PR diff 적대 검토 20개

**범위:** 집중 계획, 계획 자체의 20개 검토, 계획 입력 fixture, spec-derived expectation, Chrome Agent Browser 관측, evidence README 및 상위/public roadmap 연결이다. Runtime 구현은 포함하지 않는다. 계획 검토 20개와 구별되는 PR 변경 실패 경로를 확인했다.

| # | 실패 경로·불변 조건 | 검토 결과 |
|---:|---|---|
| 1 | 계획 PR이 새 CSS 동작을 제품 runtime에 연결하는가? | 변경 Rust/Kotlin/Swift/C runtime 파일은 없고 fixture HTML, 계획, 사전 evidence 및 roadmap만 다룬다. |
| 2 | 브라우저 fixture의 PNG 상대 경로가 잘못되어 빈 이미지로 sizing하는가? | fixture 위치에서 `../../resources/c14-1-png-basic.png`가 실제 `tests/fixtures/resources` 자산을 가리키며 Agent Browser의 모든 image decode와 natural size가 통과했다. |
| 3 | PNG 파일이 예상한 것과 다른 크기나 bytes인가? | `24×16px` natural size를 브라우저로 확인했고 source SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`를 파일에 대조했다. |
| 4 | replaced width가 intrinsic ratio 대신 attr 또는 stretch에서 오는가? | 고정 CSS `width:auto`, A/B/C `height:8/12/16px`, start alignment를 source와 plan에 일치시켰다. |
| 5 | 두 축의 cyclic `calc()` input이 계획·fixture·expectation에서 어긋나는가? | fixture와 JSON 모두 3-column/2-row `minmax(calc(10% + 2px), max-content)`, 양축 2px gap, 2px border, 0 padding을 쓴다. |
| 6 | 세 image가 같은 span topology 또는 잘못된 line을 사용해 span-group 자극을 잃는가? | A `1/3`, B `2/4`, C `1/4` column placement 및 전체 `1/3` row placement를 세 파일에서 대조했다. |
| 7 | 관측된 root 크기에 border와 content-box basis가 혼합되는가? | intrinsic track/gap content `25×16`과 border `29×20`, definite content `80×80`과 border `84×84`를 별도 필드로 적었다. |
| 8 | intrinsic planned increase를 DOM 순서대로 누적해 Chrome 결과를 맞추는가? | 독립 JSON은 A/B span-2 increases `[3,3,0]` / `[0,6,6]`의 per-track maximum `[3,6,6]`을 사용한다. |
| 9 | span-3 C의 24px width가 기존 span-2 result 뒤 추가 grow하는가? | 세 track과 gutters의 25px available sum과 24px contribution을 비교해 additional growth 0을 예상했다. |
| 10 | Rows 계산이 Columns와 무관하거나 height를 inline contribution으로 쓰는가? | A/B/C width와 height를 분리하고 rows의 span-2 contribution 증가 1/3/5 및 maximum `[5,5]`에서 `[7,7]`을 유도했다. |
| 11 | 80px definite control의 cyclic basis에 border 4px이 포함되는가? | 80px content basis에서 `10% + 2px = 10px`, 각 track minimum 10px을 계산했다. |
| 12 | 80px controls에서 image가 track maximum을 키우는데 이를 숨기는가? | A/B/C span available sizes가 각 image contribution보다 크므로 `[10,10,10]` 및 `[10,10]`에 머무는 산술을 명시했다. |
| 13 | Root frames가 absolute viewport positions로 저장되어 DOM order 차이를 오판하는가? | Observation JSON과 independent expectation은 root-relative x/y 및 item-keyed frames를 기록한다. |
| 14 | Reverse variants가 item identity 외의 style·placement까지 바꾸는가? | Fixture 정의 배열 비교에서 order arrays만 forward/reverse로 다르며 observation의 item-keyed geometry가 같다. |
| 15 | Chrome 156 output을 normative expected 값으로 역산하는가? | README에서 W3C 2025 CRD 산술과 Chrome observation 경로를 분리하고 JSON에 independent derivation을 명시했다. |
| 16 | PNG screenshot이 다른 브라우저·fixture 또는 로컬 stale page에서 나온다? | Agent Browser UA/URL/scene geometry를 재확인한 뒤 screenshot을 캡처하고 PNG hash `088d89db83da7a4690e6e4163e797731cbcb5d460da8a27e1e1b5d0e76b10963`를 산출했다. |
| 17 | 문서 이미지·JSON·계획 링크가 selected site에서 dead route로 변하는가? | 로컬 산출물 링크를 안정적인 GitHub `main` blob URL로 바꾸고 screenshot 표시에는 raw content URL을 사용했다. 전체 저장소 빌드를 다시 실행한 결과 이번 slice는 dead-link 오류 목록에 없다. |
| 18 | 공개 roadmap이 새 항목을 runtime 완료처럼 표시하거나 review link를 놓치는가? | Agent Browser가 로컬 roadmap source에서 새 카드, runtime 미시작 문구와 계획·사전 비교·계획 검토·PR diff 검토·status 5개 href를 확인했다. Tailnet 최종 URL은 계획 PR merge 후 sync evidence에서 별도로 검사한다. |
| 19 | 공식 C11.4 check or completion counter changes accidentally? | `spec/STATUS.md` is unchanged; only parent plan and roadmap narrative are added; C11.4 remains unchecked and text stays `77/213`. |
| 20 | Browser/simulator/oracle evidence is expanded to unrun app proof? | Plan and README distinguish browser oracle from Android/iOS runtime, draw/present, pixels and artifact gates; none are claimed complete by this plan PR. |

**판정:** 20개의 서로 다른 diff 실패 경로를 확인했다. selected RSPress build와 browser/roadmap route checks는 연결된 evidence에서 확인한다. Repository-wide RSPress build는 기존 전역 dead-link 진단 때문에 실패할 수 있으며 해당 결과를 full build pass로 표시하지 않는다. 이 검토는 계획 자체의 20개 검토 및 이후 implementation/runtime 20개 검토를 대체하지 않는다.
