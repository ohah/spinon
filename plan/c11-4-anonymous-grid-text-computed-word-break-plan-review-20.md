# C11.4 익명 Grid 텍스트 computed `word-break` 계획 적대 검토 20개

계획 PR과 구현 검토를 분리하기 위해, 계획 확정 전에 서로 다른 실패 경로 20개로 범위·oracle·수용 기준을 점검했다. 검토 중 문서의 `HostDocument root lang` 표현이 HTML document element와 익명 Text의 Grid style owner를 혼동할 수 있어 계획과 사전 비교 README를 수정했다. 이 기록은 계획 검토이며 Spinon runtime 통과를 뜻하지 않는다.

| # | 실패 관점 | 계획의 방어 또는 처리 | 판정 |
| ---: | --- | --- | --- |
| 1 | 기존 기능과 중복인 작업을 새로 계획 | 최신 `origin/main`에서 anonymous adapter가 `word-break:normal`만 허용하는 것을 확인했다. 열린 PR은 없고, 기존 anonymous text 후속 PR의 변경 파일에도 이 변환이 없다. | 통과 |
| 2 | Text 노드 자체의 computed style이 존재한다고 가정 | 직접 Text 노드의 상속 style owner인 정확한 Grid container에서 computed `word-break`를 가져온다. | 통과 |
| 3 | outer HTML language를 익명 Grid item의 locale로 잘못 사용 | fixture는 `<html lang="en">` 아래 Grid owner에 scene별 `lang="en"` 또는 `lang="ko"`를 둔다. Korean locale은 owner에서만 읽는다. | 통과 |
| 4 | `xml:lang`와 `lang` 충돌로 Stylo 언어가 달라짐 | Korean owner에 `xml:lang`이 있거나 값이 충돌하면 layout publication 전에 거부한다. | 통과 |
| 5 | 임의 BCP 47 태그 지원으로 범위가 암묵적으로 넓어짐 | 고정 Korean 장면은 정확한 owner `lang="ko"`만 허용하고 누락·변형 locale은 negative control로 거부한다. | 통과 |
| 6 | locale 속성 변경과 다른 style revision을 합성 | owner node, source/revision snapshot, computed style, anonymous text input의 revision을 같은 값으로 검증한다. | 통과 |
| 7 | computed CSS serialization을 약한 부분 문자열 검사로 수락 | `normal`, `break-all`, `keep-all` exact allowlist를 scene ID와 함께 검사하고 typed enum과 일치시킨다. | 통과 |
| 8 | deprecated `break-word`를 `normal`로 오인 | `break-word`를 별도 거부한다. CSS Text의 `overflow-wrap:anywhere` 결합 동작을 이 작업에서 구현했다고 주장하지 않는다. | 통과 |
| 9 | `overflow-wrap:anywhere`와 word-break 효과를 혼합 | 모든 장면에서 computed `overflow-wrap:normal`을 요구하고 다른 값은 거부한다. | 통과 |
| 10 | Parley Locale 입력 파싱 실패를 기본 locale로 조용히 대체 | pinned Parley 0.12.0의 `Locale(Option<Language>)` 경로를 사용하고 locale parse 실패는 publication 전에 거부한다. 기본 `None` 동작은 기존 paragraph regression으로 보존한다. | 통과 |
| 11 | enum에 `KeepAll`이 없어서 break-all과 같은 값으로 취급 | pinned Parley 0.12.0의 `WordBreak::KeepAll`을 별도 `WordBreakMode`에 연결하고 exhaustive match·unit test를 갱신한다. | 통과 |
| 12 | 공유 paragraph model 변경으로 일반 author CSS가 새로 허용 | `KeepAll` 및 locale을 text-layout API에 표현하되 CSS author admission 경로는 수정하지 않는다. generic direct-element parser의 거부 동작을 회귀 확인한다. | 통과 |
| 13 | Ahem fallback font가 환경마다 달라 Latin min-content가 바뀜 | Ahem SHA-256, face weight, `document.fonts.check`를 기록하고 고정 font bytes를 runtime fixture 입력으로 쓴다. | 통과 |
| 14 | 공백 collapse로 두 단어의 min-/max-content 기대값이 달라짐 | `XX XX`의 normal whitespace profile을 고정한다. 독립 산식은 min-content 20px, max-content 50px로 나누고 break-all min-content 10px을 별도 확인한다. | 통과 |
| 15 | 단어 내부 break 기회와 max-content를 혼동 | `XXXX` 및 `XX XX`에서 normal/break-all의 min-content 차이와 max-content 불변값을 각각 비교한다. | 통과 |
| 16 | Korean `normal`이 locale 없이 문자 사이에서 잘못 분리 | pinned Noto Sans KR `가나다라`와 owner `lang=ko`를 함께 요구하고 `normal` min-content 9.2px / 4줄 기대를 독립 산식으로 기록한다. | 통과 |
| 17 | `keep-all`을 nowrap 또는 max-content로 잘못 구현 | normal과 keep-all을 min-content track·row 높이에서 비교한다. 예상은 각각 9.2px/40px와 36.8px/10px이며 두 mode의 max-content는 36.8px다. | 통과 |
| 18 | variable font 축·font identity가 oracle과 달라짐 | C15.3에서 이미 고정된 Noto Sans KR 2.004 bytes/hash를 재사용하고 400 weight, face identity 및 U+AC00..U+B77C advance를 확인한다. | 통과 |
| 19 | Chrome의 1/64px quantization을 엔진 오차와 혼동 | 독립 hmtx 산식, Chrome 직렬화 차이(최대 0.0125px), runtime 허용치(0.05px)를 별도 열로 판정한다. Keyword·scene 수·Ahem geometry와 line count는 정확히 비교한다. | 통과 |
| 20 | 줄바꿈 외 입력 차이를 `word-break` 지원으로 확대 해석 | horizontal-tb/LTR, normal whitespace, `line-break:auto`, `overflow-wrap:normal`, 명시 line-height 및 고정 텍스트만 사용한다. Korean 입력에는 line-break strict/loose 판정을 바꾸는 문장부호를 넣지 않고, 일반 line-break·언어 조합은 지원 범위에서 제외한다. | 통과 |

계획은 fixture 전용 12 scene, exact owner locale provenance, default/fixture artifact 경계, Android/iOS Simulator 행렬, 구현 후 새 runtime 검토 20개와 PR diff 검토 30개를 요구한다. 이 사전 검토만으로 계획 PR 또는 구현을 승인된 것으로 간주하지 않는다.
