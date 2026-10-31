# C11.4 `white-space:pre` preserved-tab 구현/runtime 실패 관점 20개

계획 실패 관점 20개와 분리해 구현 및 simulator evidence를 확인했다. 범위는 고정 C11.4 anonymous Grid text fixture에 기본 `tab-size:8`을 더한 후보이며 전체 C11.4 완료 판정은 아니다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 새 tab 경로가 `PreWrap`의 기존 tab 동작을 바꾸는가 | **PASS** — Pre와 PreWrap gate를 분리했고, 같은 text test 실행에서 PreWrap regression 4개가 통과했다. |
| 2 | 기본 정지점 8이 다른 값으로 계산되는가 | **PASS** — Ahem `X`/space 10px에서 간격 80px이며 `XX\tXX` 100px 사례가 Rust와 두 simulator에서 통과했다. |
| 3 | 정지점과 정확히 만난 tab이 같은 위치에 머무는가 | **PASS** — `XXXXXXXX\tX`는 strict next stop x=160을 사용해 170px가 된다. |
| 4 | leading tab이 0폭이 되거나 첫 글자 뒤에 잘못 놓이는가 | **PASS** — `\tX`는 90px로 정규화되고 두 simulator trace가 예상 frame을 냈다. |
| 5 | 연속 tab이 모두 같은 정지점을 공유하는가 | **PASS** — `XX\t\tX`의 각 tab이 다음 정지점으로 이동해 170px가 된다. |
| 6 | trailing tab advance가 버려지는가 | **PASS** — `XX\t`는 80px를 유지한다. |
| 7 | forced break 뒤 cursor가 이전 줄 위치를 상속하는가 | **PASS** — `XX\tXX\nX\tX` 두 번째 줄은 새 cursor로 90px이고 전체 row는 100×20px다. |
| 8 | CRLF를 두 번 줄바꿈하거나 source mapping을 잃는가 | **PASS** — unit test가 CR/LF 각각의 source range와 하나의 normalized newline 범위를 고정한다. |
| 9 | 탭 하나의 source range가 확장 공백 전체를 가리키지 않는가 | **PASS** — `XX\tXX`의 U+0009 `[2,3)`은 paragraph UTF-8 `[2,8)`을 가리키며 capture validator가 모든 신규 positive scene을 비교한다. |
| 10 | 여러 TextSource가 합쳐질 때 tab의 원본 node를 잃는가 | **PASS** — split source 단위 테스트에서 node 3의 tab `[0,1)`이 `[2,8)`에 유지된다. |
| 11 | `min-content`에서 soft wrap이 생기는가 | **PASS** — `Pre`는 `min-content`와 `max-content` 모두 100px이고 10px 한 줄이다. |
| 12 | definite 25px track이 `pre` 줄을 wrap시키는가 | **PASS** — root track/frame은 25px이고 text line advance는 100px로 분리된다. |
| 13 | Ahem 이외 font나 문자가 조용히 허용되는가 | **PASS** — non-Ahem, 제한 외 ASCII 문자, inline ancestor를 intrinsic/final layout 양쪽에서 거부한다. |
| 14 | 다른 text style profile에서 tab이 부분 결과를 만드는가 | **PASS** — profile 불일치 시 `UnsupportedPreTabProfile`을 반환하며 unit test가 두 API 경로 모두 검사한다. |
| 15 | computed `tab-size` initial value가 snapshot에서 누락되는가 | **PASS** — Stylo test에서 omitted declaration은 `8`, 명시 `8`은 `8`, authored `2`는 `2`로 보인다. |
| 16 | authored `tab-size:2`가 지원 입력처럼 layout되는가 | **PASS** — 고정 negative scene은 property `tab-size`의 `unsupported_c114_anonymous_text_style`, node 1로 거부된다. |
| 17 | negative 입력이 frame/trace/present 성공을 남기는가 | **PASS** — Android·iOS 각 result에서 expected rejection 1/1이며 frame, trace, matched presentation/pixel 성공은 없다. |
| 18 | Rust/FFI/capture/app의 scene 행렬이 서로 어긋나는가 | **PASS** — 네 catalog와 boundary test가 40개 ID를 고정하고 Node test 24/24가 통과했다. |
| 19 | fixture marker 또는 ABI가 기본 FFI archive로 누출되는가 | **PASS** — 기본 archive는 새 ID 0/9·정의된 ABI 0개, fixture opt-in archive는 ID 9/9·정의된 ABI 2개다([archive 근거](./artifact-boundary.md)). |
| 20 | simulator가 stale binary, retry 또는 화면 제출 누락을 숨기는가 | **PASS** — 양 플랫폼 40/40, positive draw/present 39/39, retry 0이며 설치 binary hash가 build hash와 같다([실행 근거](./README.md)). |

## 남은 경계

두 simulator screenshot은 fixture surface 제출을 보여주지만 text glyph를 그리지 않는다. Android 실기기·hardware GPU, iOS compositor 이후 surface, 일반 CSS cascade, 다른 tab-size/폰트/방향, WPT 적합성은 이 검토 범위에 없다. 공식 C11.4/C11 checkbox와 `72/213`은 그대로 둔다.
