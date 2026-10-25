# 직교 Ahem text intrinsic 일반화 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-orthogonal-text-intrinsic-generalization.md), [고정 입력](../tests/fixtures/css/c11/grid-orthogonal-text-generalization-c114-precomparison.html), [Chrome observation과 독립 계산](../spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/README.md)이다. 구현 전에 계획·oracle·입력 경계를 서로 다른 방향에서 반박했다. 이 검토는 코드/runtime나 PR diff 검토를 대신하지 않는다.

| # | 공격 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | Chrome 관측을 CSS 규범 기대값으로 바꿔 썼는가 | 독립 Ahem advance 계산과 Chrome JSON이 별도로 저장된다. 검증기는 독립 계산을 입력으로 기대 row를 만들고 Chrome 값과만 비교한다. **PASS** |
| 2 | Ahem의 각 문자 advance를 임의로 10px이라 가정했는가 | 저장소 Ahem.ttf의 unitsPerEm=1000 및 A–F, 9, Z, U+0020 advance 1000을 직접 읽었다. 10px em에서 10px이다. **PASS** |
| 3 | min-content를 전체 문자열 폭으로 계산했는가 | soft-wrap opportunity는 U+0020뿐인 영숫자 단어 사이에 있다. 다섯 입력의 longest token을 독립 계산하고 10개 image-backed target row를 Chrome과 대조한다. **PASS** |
| 4 | max-content에 collapsed space advance를 빠뜨렸는가 | A BC와 AB CDE F에서 collapse 뒤 공백 한 칸을 포함해 40px·80px로 계산한다. **PASS** |
| 5 | 여러 공백 또는 양끝 공백을 일반 단어 길이에 포함했는가 | 양끝과 반복 공백이 있는 입력의 collapsed string은 A 9Z이며 기대 min/max는 20/40px이다. 공백만 있는 비어 있지 않은 입력은 text contribution 0을 검증한다. **PASS** |
| 6 | min/max scene이 실제로 알고리즘을 구별하는가 | A BC, AB CDE F, collapsed whitespace는 min/max row 값이 다르다. ABC와 공백만 있는 입력은 의도된 equality control이다. **PASS** |
| 7 | Writing Modes의 논리축과 물리 row를 혼동했는가 | 부모 horizontal-tb block axis와 자식 vertical-rl inline axis 대응을 CSS Writing Modes §§6.1, 7.3으로 고정했다. **PASS** |
| 8 | 이미지 intrinsic size를 text contribution에 더했는가 | text/image는 서로 다른 column의 row-span-1 item이므로 row의 max contribution만 사용한다. 여덟 일반 target은 text가 16px보다 크고, 두 spaces-only target은 image floor 16px을 보이며, image 없는 두 Chrome control은 0px을 확인한다. **PASS** |
| 9 | item frame을 track contribution과 같은 것으로 무조건 취급했는가 | Chrome 관측에서 row min/max에 따라 text frame이 50×row로 stretch되고 PNG는 natural 24×16px다. 계획에 둘을 따로 비교한다. **PASS** |
| 10 | auto row sizing의 parent width 또는 columns가 변해 결과를 오염시키는가 | 모든 scene은 74px root width, [50,24]px columns, zero edges/gaps로 고정했다. **PASS** |
| 11 | scene별 CSS 입력이 서로 다를 수 있는가 | 열 image-backed target은 같은 HTML/CSS template와 root geometry를 공유하고 text와 row sizing keyword만 바뀐다. 두 no-image control은 image 부재만 다르며 Chrome oracle 전용이다. **PASS** |
| 12 | 글꼴 download/decode race가 관측에 섞였는가 | observation은 `fontStatus=loaded`, `fontLoaded=true`, 열 target image의 `complete` 및 natural size를 기록한다. 두 no-image control은 image decode 대상이 아니다. **PASS** |
| 13 | profile을 ASCII 이외 모든 vertical text로 넓히는가 | implementation gate는 ASCII alphanumeric과 U+0020만 허용하고 Unicode scripts, emoji, punctuation/hyphen을 명시적으로 제외한다. **PASS** |
| 14 | 실제 vertical shaping 또는 glyph orientation을 입증한다고 과장하는가 | Ahem mixed-orientation layout 측정 subset으로만 판정하며 glyph raster/pixel painting을 명시적으로 제외한다. **PASS** |
| 15 | text-orientation이나 writing-mode 경계가 모호한가 | vertical-rl/mixed child와 horizontal-tb parent만 허용한다. vertical-lr와 upright는 거부 대상으로 둔다. **PASS** |
| 16 | font, line-height, direction, wrapping style을 임의로 바꿔도 허용하는가 | Ahem 10px/400/normal, explicit 10px line-height, normal whitespace/word-break/line-break, LTR/normal ligatures를 gate 및 negative controls로 고정한다. **PASS** |
| 17 | inline span 또는 여러 source node를 지원한다고 읽힐 수 있는가 | 단일 paragraph source와 empty inline style/ancestor 조건만 수용하며 그 밖의 topology는 제외한다. **PASS** |
| 18 | 입력 revision이나 font registry provenance가 사라질 수 있는가 | 구현 acceptance에 source/style revision, font registry revision/token 및 contribution provenance 보존을 명시했다. **PASS** |
| 19 | opt-in fixture 결과를 일반 CSS 제품 지원으로 승격하는가 | 공개 API/admission, WPT 및 전체 C11.4 완료를 범위 밖으로 두고 checkbox와 72/213을 유지한다. **PASS** |
| 20 | simulator, surface 및 artifact 결과를 과장하는가 | Android AVD/iOS Simulator만 계획하고 실기기·hardware GPU·compositor 뒤 pixel 및 release app 전체 fixture exclusion을 주장하지 않는다. **PASS** |

## 판정

20개 관점에서 계획의 oracle, 논리축, 입력 gate, negative boundary, fixture scope와 완료 주장을 확인했다. 독립 계산 검증기는 Chrome 156 observation의 10개 image-backed target과 2개 no-image control, 총 12/12를 통과했다. 두 control은 모바일 runtime 입력에 포함되지 않는다. 이 판정은 계획 단계만 통과한 것이며 구현 또는 PR 승인 결과가 아니다.
