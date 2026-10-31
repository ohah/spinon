# C11.4 anonymous Grid text computed `line-height` 계획 PR diff 검토 30개

계획 PR에 포함할 문서·oracle·fixture source diff를 서로 다른 실패 관점으로 검토했다. 이 검토는 계획 PR diff만 대상으로 하며 구현 검토가 아니다.

| # | PR diff 실패 관점 | 확인·방어 | 판정 |
| ---: | --- | --- | --- |
| 1 | `spec/STATUS.md`에서 C11 parent를 완료 처리 | C11 checkbox와 C11.4 checkbox는 계속 `[ ]` | 통과 |
| 2 | 전체 완료 집계가 잘못 증가 | 공식 `72/213` 유지 문구 확인 | 통과 |
| 3 | 계획 노트를 C11.4 대신 다른 하위 항목에 배치 | 새 노트는 C11.4 기록 뒤, C12 시작 전에 위치 | 통과 |
| 4 | 계획 제목/링크가 이미 완료된 white-space PR을 중복 | `computed line-height`로 고유 ID와 별도 CSS 축을 표시 | 통과 |
| 5 | 비교 입력이 repo Ahem이 아닌 시스템 폰트에 의존 | fixture가 `tests/fixtures/resources/c15/Ahem.ttf`를 상대 참조 | 통과 |
| 6 | font file이 교체됐는데 증거가 그대로 유지 | README·fixture hash가 현재 SHA-256 결과와 일치 | 통과 |
| 7 | fixture text가 두 줄이라고 주장하지만 실제 newline 없음 | 원본 HTML `XX` LF `XX`를 확인하고 Chrome Range/raw 결과 보존 | 통과 |
| 8 | forced-break case에 explicit row가 결과를 고정 | `grid-template-rows:auto` 및 root auto height 확인 | 통과 |
| 9 | sample에서 margin/padding/border/gap이 추가되어 track을 바꿈 | zero edges와 zero gaps를 명시 | 통과 |
| 10 | 예상값 수와 관찰값 수가 다름 | expected-values와 raw observation ID 집합이 모두 16개 | 통과 |
| 11 | screenshot을 12-case 표로 설명하며 16개 입력을 누락 | screenshot은 12 value matrix와 4 line-shape control을 모두 표시 | 통과 |
| 12 | screenshot viewport가 source 값과 불일치 | observation JSON의 420×600/DPR 1과 README를 대조 | 통과 |
| 13 | Chrome version이 UA의 reduced version만 기록 | raw `fullVersionList`에 `156.0.8078.76` 기록, README에 정확 버전 사용 | 통과 |
| 14 | Ahem 미로드 상태에서 fallback metrics를 기대값으로 사용 | `fontLoaded:true` 및 `document.fonts.check` 기록 | 통과 |
| 15 | `normal`을 규범상 정확히 10px로 과장 | README에서 browser/font-metric dependent로 구분 | 통과 |
| 16 | unitless number를 absolute length로 계산 | 독립 기대값은 `n × computed font-size` 식을 사용 | 통과 |
| 17 | zero 값이 양수 검증 때문에 제거 | CSS grammar와 Chrome `0px`/0 track을 별도 기록 | 통과 |
| 18 | `<length-percentage>` 입력이 모두 raw source parser로 처리된다고 주장 | `%`, `em`, `calc()`는 Stylo resolved px 경로만 계획 | 통과 |
| 19 | unsupported numeric serialization을 임의 `f32` parse로 허용 | `normal`/finite nonnegative number/px만 strict allowlist | 통과 |
| 20 | NaN·Infinity·계산 overflow에서 partial layout 가능 | conversion 후 finite 검사와 pre-publication rejection을 계획 | 통과 |
| 21 | line-height 0 glyph overlap을 row height와 혼동 | Range, track, root height, glyph pixel을 분리 설명 | 통과 |
| 22 | CSS Grid track oracle를 root box나 Range rect로 대체 | `gridTemplateRows`/track trace를 primary row comparator로 지정 | 통과 |
| 23 | CSS Inline Working Draft를 확정 권고안으로 표시 | source는 W3C TR 링크로 표시하고 결과를 해당 관찰 범위에 제한 | 통과 |
| 24 | CSS Grid §11.5 source 링크가 다른 버전/알고리즘을 가리킴 | 2025 CRD `algo-track-sizing` URL을 직접 연결 | 통과 |
| 25 | computed-style source와 fixed `ParagraphStyle` 사이 mapping 누락 | 계획에서 `ComputedElementStyle.properties`를 정확 root/revision에 묶음 | 통과 |
| 26 | scene name이 line-height 값을 다시 하드코딩 | 계획에 scene-selected scalar 제거와 exact computed value 사용 명시 | 통과 |
| 27 | C15 inline span 범위로 확장되는 validation 변경 | `ParagraphStyle`만 수정하고 `InlineRunStyle`/C15.2b 제외 | 통과 |
| 28 | plan review 20을 PR diff review와 중복 집계 | 계획 검토 파일과 이 PR diff 검토 파일을 분리 | 통과 |
| 29 | simulator screenshot을 glyph/compositor proof로 확대 | AVD, iOS drawable submission boundary, glyph pixel limit 기록 | 통과 |
| 30 | evidence/docs 경로·해시 또는 문서 간 참조가 깨짐 | JSON parse, 16/16 oracle equality, SHA-256 및 staged whitespace check 통과 | 통과 |

결과: 계획 문서, 상태 기록, 고정 fixture, Chrome observation과 30개 diff 관점이 서로 연결된다. Spinon 구현·Android/iOS 실행·public CSS/WPT 적합성 판정은 여기에 포함하지 않는다.
