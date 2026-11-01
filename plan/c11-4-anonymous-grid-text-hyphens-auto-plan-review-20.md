# C11.4 `hyphens:auto` 계획 적대 검토 20개

기준: `plan/c11-4-anonymous-grid-text-hyphens-auto.md`, Chrome 156 사전 비교와 CSS Text 3 §5.3. 각 항목은 서로 다른 실패 원인을 검토했다. 이는 아직 구현 검토나 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | CSS 규범이 분할 알고리즘을 정하지 않는데 Chrome의 exact breakpoints를 normative oracle로 오인 | 차단 해소: 계획은 UA별 차이를 명시하고 고정 Hypher pattern을 구현 기대값으로 분리한다. |
| 2 | 콘텐츠 `lang`이 없는데 English dictionary를 임의 적용 | 차단 해소: 정확한 `en`만 허용하고 language가 없으면 거부한다. |
| 3 | `lang=fr` 또는 미지원 BCP 47 tag가 English fallback으로 통과 | 차단 해소: 현재 slice는 정확한 `en`만 허용하며 나머지는 거부한다. |
| 4 | Stylo cascade 값 대신 scene id나 author 원문을 검색해 `auto`를 결정 | 차단 해소: fixture alias를 거친 실제 computed value와 revision을 사용하도록 적었다. |
| 5 | `none`·`manual`의 기존 soft-hyphen 동작이 `auto` 추가로 바뀜 | 차단 해소: 기존 modes를 유지하고 세 mode regression을 같은 행렬에 포함한다. |
| 6 | `auto` min-content에서 U+2010 advance가 빠져 track이 작아짐 | 차단 해소: dictionary break별 visible advance를 min-content에 포함하고 독립 Ahem 폭을 검사한다. |
| 7 | `auto` max-content에 synthetic marker 또는 hyphen advance가 포함됨 | 차단 해소: max-content는 원문폭을 보존하고 no-break control을 둔다. |
| 8 | line이 dictionary point에서 끝나지 않았는데 discretionary hyphen을 더함 | 차단 해소: line-ending synthetic marker 확인 후에만 10px를 더한다. |
| 9 | dictionary marker가 DOM text나 selection/search source offset을 바꿈 | 차단 해소: marker를 normalized layout text에만 넣고 원문·UTF-16 길이를 유지한다. |
| 10 | UTF-8 byte와 UTF-16 code unit 경계가 불일치 | 차단 해소: 이 profile을 ASCII로 제한하고 source mapping gap을 테스트한다. |
| 11 | 뒤 문자의 paragraph range가 synthetic marker 삽입 뒤 밀림 | 차단 해소: marker 전후 source range와 line range의 offset·owner revision을 비교한다. |
| 12 | U+00AD가 이미 있는 word에서 CSS의 auto/manual 우선순위를 잘못 처리 | 차단 해소: 혼합 동작은 이 limited profile에서 입력 단계에 거부한다. |
| 13 | `hyphenate-limit-*`나 spelling-changing hyphenation이 무시된 채 일반 지원으로 표현 | 차단 해소: 해당 속성과 spelling change를 명시적으로 범위 밖에 둔다. |
| 14 | 잘못된/빈 Hypher 결과가 단어를 없애거나 유효하지 않은 break를 만듦 | 보완 요구: 구현은 허용 단어 allowlist, non-empty segment와 재결합 원문 동일성을 검증해야 한다. 계획의 고정 pattern 및 fail-closed 조건에 이 검증을 포함한다. |
| 15 | Hypher no-alloc API의 >45-byte panic 경계 도달 | 차단 해소: 허용 단어는 45바이트 미만이며 API 호출 전 길이를 재검사한다. |
| 16 | default feature가 다국어 dictionary bytes를 제품 artifact에 포함 | 차단 해소: optional dependency, English feature 단독, 기본/product와 opt-in archive positive/negative checks를 요구한다. |
| 17 | Cargo feature가 Android/iOS 앱 fixture-test 경로에 전파되지 않음 | 차단 해소: runtime의 기존 opt-in C11.4 fixture feature에 text-layout feature를 연결하고 두 simulator에서 빌드·실행한다. |
| 18 | 같은 50px에서 pattern segments를 잘못 묶어 예상 줄 수가 달라짐 | 차단 해소: 각 fixed word의 syllable 폭·line count·per-line advance를 독립 계산값과 비교한다. |
| 19 | Chrome/Hypher breakpoint 차이가 버그 또는 호환 완료로 과장됨 | 차단 해소: Chrome은 browser observation, Hypher는 pinned fixture algorithm으로 별도 표기하고 차이를 보존한다. |
| 20 | 제한 fixture를 전체 C11.4/C11, WPT, public CSS 지원 또는 실제 기기 완료로 승격 | 차단 해소: 일반 author CSS·WPT·glyph paint·hardware GPU·실기기 범위와 공식 `72/213` 미완료를 명시한다. |

검토 중 #14에서 segment integrity assertion을 구체화했다. 구현은 각 word의 분절 조각이 모두 비어 있지 않고 원문을 순서대로 정확히 재구성하는지 검사해야 한다. 이 계획은 구현 진입 조건이며 계획 PR diff 검토와 구현 후 20개 검토를 별도로 남긴다.
