# C11.4 Unicode `text-transform:capitalize` 계획 실패 관점 검토 20개

검토 대상은 계획과 사전 oracle이며 제품 구현 결과가 아니다. 각 항목은 서로 다른 계약 경계나 실패 경로를 점검한다. 검토 중 scene 수와 font metric의 해석 문제를 찾아 계획·oracle을 고쳤다.

| # | 실패 관점 | 검토 결과와 반영 |
|---:|---|---|
| 1 | 기존 런타임 회귀 scene이 새 행렬에서 빠진다 | 초안의 `6 + 11 = 17`은 잘못된 수였다. 기존 none/uppercase/lowercase 6개와 ASCII capitalize 6개, Unicode 11개를 합한 23개로 고쳤다. |
| 2 | 현재 이미 구현된 ASCII 제한 slice를 미완료 후보처럼 설명한다 | 부모 계획의 현재 후보를 Unicode 후속으로 바꾸고 ASCII PR #986을 기준점으로 명시했다. |
| 3 | 바뀔 수 있는 최신 CSS Text 페이지를 근거로 삼는다 | 2026-08-14 CRD 고정 URL을 연결해 해당 계획의 규범 revision을 고정했다. |
| 4 | 타이포그래피상 단위와 Unicode scalar를 혼동한다 | CSS Text §1.4의 typographic letter unit(`Letter` 또는 `Number`)을 입력 판정에 사용한다. |
| 5 | 단어 시작 숫자 뒤의 글자를 대문자로 바꾼다 | `49ers`는 첫 단위가 Number라 그대로 둔다. `text-transform:none` control과 비교한다. |
| 6 | 하이픈 뒤 단어의 첫 글자를 놓친다 | UAX #29 분절의 `well-known` 기대값 `Well-Known`과 고정 Chrome candidate를 둔다. |
| 7 | apostrophe 안쪽 `n`을 새 단어 첫 글자로 잘못 처리한다 | UAX #29에서 `o'neill`을 처리하고 기대 문자열 `O'neill`을 고정한다. |
| 8 | 여는 문장부호 다음 첫 글자를 놓친다 | `«élan»`을 두고 기대 문자열 `«Élan»`을 기록했다. |
| 9 | `<html lang>` 상속이나 locale 자동 추정을 사용한다 | fixture root의 직접 `lang="en"`만 허용하고 다른 입력은 fail-closed한다. |
| 10 | Chrome 관측을 Unicode 규범으로 오인한다 | `ßeta` Chrome target과 CSS Text/Unicode 17 기대값을 별도 필드·차이 분류로 보존한다. |
| 11 | 영어 profile 밖의 Turkish, Dutch 등의 규칙을 일반화한다 | 직접 `lang="en"` 이외의 locale과 locale 상속을 명시적으로 제외했다. |
| 12 | 결합문자 입력을 NFC/NFD로 몰래 정규화한다 | `e + U+0301`은 `E + U+0301`로 보존하며 정규화하지 않는다고 고정했다. |
| 13 | text transform과 whitespace collapse를 반대 순서로 적용한다 | 고정 CSS Text §2.1.2와 Appendix A를 확인해 Phase I 뒤 transform, Phase II 앞 순서로 계획했다. |
| 14 | U+00DF 확장이 output/source offset을 밀어 뒤 문자 mapping을 손상한다 | 두 output scalar가 같은 원본 UTF-16 `[0,1)`을 가리키고 나머지 원본 범위가 이어지는 기대값을 썼다. |
| 15 | adjacent direct Text node마다 단어 상태를 초기화한다 | split fixture를 단일 anonymous item으로 처리하고 `ß`와 `eta` 사이에서 재시작하지 않는다. |
| 16 | UTF-16 code unit과 UTF-8 paragraph byte range를 같은 단위로 취급한다 | 계획은 output UTF-8 byte ranges와 원문 UTF-16 provenance를 별도로 유지하도록 했다. supplementary scalar를 별도 unit test로 포함한다. |
| 17 | Chrome의 target width를 Unicode 변환 문자열의 기대 폭으로 사용한다 | 규범 문자열은 CSS Text/Unicode에서, CSSOM 폭은 Chrome shaping 관찰에서만 가져온다고 분리했다. |
| 18 | `Sseta`의 nominal font advances를 shaped width로 보고하거나 HarfBuzz를 CSS 폭 oracle로 쓴다 | unshaped `hmtx` 합 25.58px, HarfBuzz `Sseta` 25.17px, Chrome candidate 25.1875px를 구분했다. HarfBuzz가 `Iiii Ii` 공백을 280 units로 치환해 19.66px를 내고 Chrome은 19.09375px인 반례도 기록해 둘 다 CSS 폭 oracle에서 제외했다. |
| 19 | 폰트가 지원하지 않는 scalar로 simulator glyph paint를 주장한다 | 고정 font hash와 CSSOM/layout 증거만 사용하고 `ǆ`·Deseret 같은 unit-test 값은 glyph raster 증거에서 제외했다. |
| 20 | 계획 PR이나 opt-in slice가 C11.4 완료·공식 완료 수를 올린다 | 계획·runtime·PR gates는 C11 checkbox와 `72/213`을 유지하도록 명시한다. 구현/CI/플랫폼 결과는 아직 별도 후속 근거가 필요하다. |

## 판정

계획은 23개 기존+추가 scene, 규범과 Chrome 차이의 분리, source mapping, shaping 경계 및 artifact 분리를 명시한다. 발견한 scene count와 `hmtx` 해석 오류는 반영했다. 이 검토는 구현·시뮬레이터 결과나 구현 전 20개 runtime 리뷰를 대체하지 않는다.
