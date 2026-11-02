# 구현 PR diff 실패 관점 검토 30개

구현/runtime 20개 검토와 다른 관점에서, 계획 PR 이후 실제 implementation·tests·fixture·status/spec/evidence diff를 `1d838ba44bd635bb5c572dc950b9313f1863434a`와 비교했다.

| # | PR diff 검토 관점 | 판정과 근거 |
|---:|---|---|
| 1 | cascade option 이름과 소유자가 새 profile의 범위를 명확히 하는가 | Root HTML row-list option은 기존 template-list option과 분리돼 있다. `grid.rs`, `element.rs`. 통과. |
| 2 | option default가 다른 profile을 바꾸지 않는가 | option은 profile 조건부이고 다른 profile의 bool은 그대로다. cascade existing matrix와 scoped tests 통과. |
| 3 | cascade rows와 columns가 같은 확장 option을 공유하는가 | 신규 Root HTML option은 auto rows에만 전달되며 auto columns는 기존 flag를 쓴다. explicit negative test 통과. |
| 4 | track conversion이 percentage/fr/calc/intrinsic을 px로 강제 cast하는가 | 변환 결과 variant가 `LengthPx`인 경우만 채택하고 기타 값은 Unsupported로 남긴다. mixed list tests 통과. |
| 5 | CSS declaration이 cascade 우승 후 원 authored sequence와 달라지는가 | computed `ImplicitGridTracks` 순서를 그대로 복사한다. cascade positive list test 통과. |
| 6 | projection error property 또는 node identity를 잃는가 | validator는 node 및 `grid-auto-rows` property를 지정한다. invalid-list projection tests에서 확인. |
| 7 | shared helper가 scalar semantics를 잘못 바꾸는가 | `LengthPx` scalar는 helper에서 기존 admitted class로 분류되고 scalar runtime tests 통과. |
| 8 | helper가 empty TrackList를 fixed list로 인식하는가 | helper는 nonempty 조건을 포함하고 empty list test가 거부를 확인한다. |
| 9 | projection에서 length validation보다 목록 type validation이 앞서 잘못된 변형을 살리는가 | non-px list는 px-only gate에서, px negative/nonfinite와 10,001개 list는 projection에서 거부된다. projection 11/11. |
| 10 | capacity preflight가 newly accepted list를 누락해 overflow 또는 우회하는가 | fixed implicit rows classification helper가 preflight와 alignment gates 양쪽에서 사용된다. 10,000개 list/expansion ceiling tests 통과. |
| 11 | Grid area와 fixed implicit list를 함께 쓰도록 validator가 넓어지는가 | named-area profile negative test는 property-specific error를 유지한다. |
| 12 | fractional·percentage track 조합의 기존 허용 범위를 바꾸는가 | 변경은 auto-row list extraction/placement에 한정되고 기존 Root HTML track tests 통과. |
| 13 | layout contract가 Root HTML placement mode 밖에 list를 허용하는가 | list admission은 `RootHtmlImplicitRowsV1` match에만 추가했다. other placement branches unchanged. |
| 14 | unsupported `GridAutoRows::Auto`가 accidental success를 얻는가 | implicit row mode에서 여전히 거부되고 기존 Root HTML negative matrix 통과. |
| 15 | negative/nonfinite/oversized direct layout values가 implicit placement 분기 때만 검증되는가 | 발견 사항을 root contract admission에 옮겼다. explicit-row-only case에서도 negative/NaN/infinity를 거부하고 10,001 list를 `GridTrackLimitExceeded`로 반환한다. |
| 16 | allocation 호출 전에 checked implicit count와 track cap이 실행되는가 | `max_rows - explicit_rows` 이후 cap을 확인하고 `try_reserve_exact`를 호출한다. 10,001 boundary test 통과. |
| 17 | reservation 실패가 다른 error/property로 변환되거나 panic하는가 | expanded-row failpoint는 기존 `GridReservationFailure` 경로로 반환하고 recovery test 통과. |
| 18 | list `.cycle()`가 empty iterator에서 무한히 동작하거나 count를 넘어 소비하는가 | empty list는 사전 거부되고 `take(implicit_rows)`로 정확한 수만 확장한다. list/cap tests 통과. |
| 19 | fixed track 복사에서 source track 값이 손실되거나 재계산되는가 | `LengthPx` 값을 그대로 복사해 순환하며 explicit rows 뒤 pattern을 보존한다. exact frames 통과. |
| 20 | runtime test fixture가 item color 중복이나 DOM order 때문에 잘못된 item을 검사하는가 | same-color frames are collected and sorted by y/x, expected eight-frame matrix is independently enumerated. 통과. |
| 21 | runtime helper test item-count reduction이 잘못된 element를 제거하는가 | fixture id 1..8을 확인하고 suffix items만 한 개씩 제거한다. 1·2·3 implicit row cases 통과. |
| 22 | test oracle가 Chrome geometry 외에 제품 계약을 과장하는가 | plan은 독립 산술과 Chrome 관측을 분리하고 일반 CSS conformance를 주장하지 않는다. |
| 23 | CSS fixture universal reset/shorthand가 Root HTML parser 밖 속성을 만들어내는가 | fixture는 universal reset 대신 명시 `background-color`와 fixed declarations만 둔다. SHA는 precomparison source와 동일. |
| 24 | stale runtime rejection test가 새 지원 목록도 모두 거부하도록 남았는가 | 이전 multiple-list 거부를 mixed px/% negative로 바꾸고 positive list runtime을 별도 추가했다. |
| 25 | simulator shell probe가 미지원 body API를 써 제품 실패를 가장하는가 | 첫 probe failure artifacts 보존, corrected `firstChild` traversal을 최종 실행에서 사용. |
| 26 | app build에 opt-in C11 fixture flags가 켜져 기본 route라는 주장과 충돌하는가 | build logs는 C04 runtime GPU만 켜고 C04/C11/C15/C18/C19 fixture switches off를 기록한다. |
| 27 | Android/iOS UI capture가 GPU pixel equality로 과장되는가 | README/status는 screenshot과 draw/present를 visual evidence로 제한하고 PixelCopy/Metal readback을 명시적으로 제외한다. |
| 28 | build 중 bootstrap 파일을 덮어쓰고 원본을 잃는가 | `build-platform-apps-pass03.sh`가 source/generated bootstrap 다섯 파일을 backup 후 trap으로 복구하며 복구 hash가 원본과 같다. |
| 29 | evidence JSON·README의 screenshot/app hash가 최신 성공 run과 다른가 | pass03 APK, iOS executable, screenshot 및 동일 HTML input hash를 observation JSON과 대조한다. |
| 30 | status·contract·plan이 parent complete, CI 통과 또는 72/213 증가로 잘못 집계하는가 | C11.1/C11 checkbox와 공식 `72/213`을 유지하고 PR checks 미보고를 따로 기록한다. |

**판정:** 최종 implementation diff에서 30개 관점을 각각 재확인했다. 검토에서 발견한 direct no-implicit-row invalid-value validation 누락과 auto-row list count cap을 수정했고, alignment negative test의 기대 오류 단계를 실제 cascade boundary로 조정했다. Rust focused tests, scoped Clippy와 최신 simulator rerun 통과를 확인했다. GitHub check rollup/required check 상태는 PR 생성 후 별도로 확인한다.
