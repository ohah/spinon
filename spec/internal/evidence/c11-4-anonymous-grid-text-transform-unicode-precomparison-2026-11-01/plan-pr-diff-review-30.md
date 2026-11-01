# C11.4 Unicode `text-transform:capitalize` 계획 PR diff 적대 검토 30개

검토 대상은 base `0ac8911c303cfacb1ed562c7409f300b12b8767f` 대비 이번 계획 PR의 계획·oracle·roadmap 문서다. 아래 runtime 23개 검증은 계획된 후속 작업이며 이번 PR에서 실행하거나 완료하지 않았다. 각 관점은 다른 파일 경계, 규범 조건 또는 증거 과장 위험을 확인했다.

| # | 실패 관점 | diff 검토 결과 |
|---:|---|---|
| 1 | 문서 PR이 기능 구현이나 C11.4 완료를 주장한다 | 계획과 사전 oracle로 한정했고 구현·runtime·simulator는 아직 시작하지 않았다고 썼다. |
| 2 | 기존 ASCII `capitalize` slice가 기준점에서 빠진다 | PR #986과 원래 제한을 후속 계획 첫머리에 연결했다. |
| 3 | 상위 C11 계획이 이미 병합된 ASCII slice를 현재 후보로 계속 가리킨다 | roadmap candidate link를 Unicode 후속 계획과 Chrome precomparison으로 변경했다. |
| 4 | 문서 수정만으로 공식 완료 수가 바뀐다 | `spec/STATUS.md` 및 완료 checkbox는 이 diff에 없고 `72/213`을 유지한다고 명시했다. |
| 5 | 기존 runtime regression 수가 코드와 다르다 | 기존 fixture table의 12 scene(기본 6 + ASCII capitalize 6)을 근거로 계획 수를 바로잡았다. |
| 6 | 신규 runtime scene 수가 계획 표와 다르다 | Unicode runtime table의 신규 11 scene을 세고 총 23으로 기록했다. |
| 7 | 12-row Chrome oracle와 23-scene runtime matrix를 같은 목록으로 오해한다 | 사전 HTML의 observation row와 기존 runtime scene 수를 별도 범위로 설명했다. |
| 8 | `none`/`uppercase`/`lowercase` regression이 빠진다 | 기존 12개 전체를 새 revision에서 재실행하도록 명시했다. |
| 9 | Chrome 156이 표준의 정답으로 승격된다 | Chrome target/candidate widths와 normative mapping string을 JSON의 서로 다른 항목으로 분리했다. |
| 10 | screenshot이 Spinon 앱의 동작 화면인 것처럼 보이거나 파일이 바뀐다 | README/PR에서 Chrome precomparison 화면으로만 표시하며 simulator 결과가 아니다. PNG SHA-256도 README 표와 일치했다. |
| 11 | 관측이 font fallback으로 바뀐다 | `fontLoaded`와 family/weight/size를 JSON에 기록하고 screenshot을 포함했다. |
| 12 | font asset이 바뀌어도 같은 oracle인 것처럼 쓴다 | README의 고정 font SHA와 expected metadata의 같은 SHA를 대조했다. |
| 13 | HTML fixture 변경과 observation이 서로 다른 입력을 가리킨다 | README/JSON은 동일 fixture 경로를 가리키고 해시를 별도 기록한다. |
| 14 | machine-readable observations가 JSON 문법 오류를 가진다 | `python3 -m json.tool`로 observation JSON을 파싱했다. |
| 15 | expected values가 JSON 문법 오류를 가진다 | `python3 -m json.tool`로 expected JSON을 파싱했다. |
| 16 | floating point 허용치가 CSSOM 양자화보다 작거나 숨겨진다 | 1/64 CSS px quantization과 0.03125 CSS px 허용치를 함께 명시했다. |
| 17 | CSS Text 최신 문서가 바뀌어 규범 기준이 이동한다 | CSS Text Level 3 CRD 2026-08-14의 날짜 고정 URL을 썼다. |
| 18 | UAX #29와 case mapping 데이터 버전이 다르다 | UAX #29 revision 47/Unicode 17.0.0 및 ICU4X 후보 버전 2.3.0을 계획에 고정했다. |
| 19 | Unicode U+00DF 제목 변환이 Chrome 기대값으로 흐려진다 | Unicode titlecase `Ss`와 Chrome 156의 unchanged result를 차이로 명시했다. |
| 20 | 첫 숫자 뒤 영문을 첫 lowercase letter로 찾아 변환한다 | CSS Text typographic letter unit은 Number도 포함한다고 기록하고 `49ers`/`none` control을 넣었다. |
| 21 | apostrophe·hyphen 규칙이 한 사례로 뭉뚱그려진다 | `o'neill`과 `well-known`을 각각 독립 scene과 기대 문자열로 분리했다. |
| 22 | opening punctuation 뒤 첫 letter 처리가 비검증 상태로 남는다 | `«élan»` scene과 `«Élan»` candidate를 추가했다. |
| 23 | combining sequence가 NFC로 정규화된다 | `e + U+0301` 입력과 `E + U+0301` output, normalization exclusion을 기록했다. |
| 24 | transform이 whitespace collapse 앞에서 실행된다 | 고정 CRD §2.1.2 순서인 Phase I → transform → Phase II를 계획과 README에서 동일하게 썼다. |
| 25 | split Text node마다 capitalization 상태가 초기화된다 | adjacent direct Text node 두 개의 positive와 no-transform control을 둔다. |
| 26 | `ß` 확장으로 UTF-16 provenance가 유실된다 | output `S`와 `s`가 원본 `[0,1)`을 공유하는 expected source map을 추가했다. |
| 27 | glyph advances를 CSS track width와 동일시한다 | nominal `hmtx` 25.58, HarfBuzz diagnostics, Chrome CSSOM을 분리하고 HarfBuzz ASCII 반례를 적었다. |
| 28 | fixture 폰트가 없는 glyph를 실제 raster support로 과장한다 | `ǆ`/Deseret은 unit mapping test만 계획하고 glyph paint claim에서 제외한다. |
| 29 | 새 plan/evidence link가 깨져 계획이 고립된다 | 변경 파일의 계획·README·fixture·screenshot 및 이 review의 자체 링크를 추가 뒤 경로 검사해 모두 확인했다. |
| 30 | 계획 PR이 구현 review나 공개 CSS support를 대신한다 | 별도 구현/runtime 20관점, PR diff 30관점, artifact/platform gates와 명시적 제외 범위를 남겼다. |

## 판정

계획 PR은 병합 가능한 문서/oracle 범위로 제한된다. scene 수와 폭 근거 오류를 수정했으며 `git diff --cached --check`, JSON 파싱, fixture/resource 경로 확인을 통과했다. simulator matrix, Android/iOS app builds, 전체 C11.4 및 C11 완료는 이번 diff의 판정 범위가 아니다.

## 검토 자료

[Unicode titlecase 계획](../../../../plan/c11-4-anonymous-grid-text-transform-unicode-titlecase.md) · [계획 실패 관점 검토 20개](../../../../plan/c11-4-anonymous-grid-text-transform-unicode-titlecase-plan-review-20.md) · [Chrome 관측](./chrome156-observations.json) · [규범 기대값](./expected.json) · [Chrome precomparison screenshot](./chrome156-capitalize-unicode.png)
