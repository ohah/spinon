# C11.4 cyclic minmax mixed text/image 계획 실패 관점 검토 20개

계획의 입력, CSS Grid 2025 CRD 독립 기대값, Chrome 156 관측, 앱 실행 경계와 제외 범위를 각각 다른 실패 경로에서 검토했다. Chrome 관측은 규범 기대값을 만드는 데 사용하지 않았다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | cyclic `%`가 intrinsic axis basis에 먼저 적용되어 10%가 순환 입력이 되는가 | §7.2.1의 intrinsic auto 취급과 최종 resolve pass를 분리했다. |
| 2 | `calc(10% + 2px)`에서 절대 2px까지 사라지는가 | 초기 track bases `[2,2,2]`로 보존한다. |
| 3 | span-2 text와 image 증가를 순서대로 누적해 same-span maximum을 훼손하는가 | A/B 각각 `[7,7,0]`, `[0,17,17]`을 낸 뒤 per-track max `[7,17,17]`을 적용한다. |
| 4 | span-3 C를 span-2보다 먼저 처리해 contribution group 규칙이 달라지는가 | Span length 순서와 네 scene의 trace expectation을 명시했다. |
| 5 | text/image 종류가 같은 span group에서 분리되어 서로 다른 algorithm pass를 타는가 | 한 layout revision의 intrinsic-input list에서 source kind와 함께 모으도록 요구한다. |
| 6 | image 자연 크기와 CSS 지정 width 40px이 혼동되는가 | 24×16 source bytes, 40px CSS width, auto block size 및 3:2 ratio를 각각 고정했다. |
| 7 | image decode 전 natural size 0을 valid fixture로 처리하는가 | decode 완료, natural dimensions, image revision을 gate로 둔다. |
| 8 | Ahem 대체 글꼴이 20/60/110px 기여를 바꾸는가 | font load·revision/hash를 fixture allowlist와 사전 비교에 고정했다. |
| 9 | A/C text의 intrinsic width와 final wrapped line width를 같은 값으로 재사용하는가 | intrinsic inputs와 Columns 이후 line widths를 별도 assert한다. |
| 10 | Columns보다 Rows를 먼저 계산해 text wrapping 변화가 누락되는가 | Columns 이후 Parley line measurement가 Rows 기여를 만든다는 순서를 요구한다. |
| 11 | image의 80/3px block ratio가 intrinsic row contribution에 반영되지 않는가 | rational row calculation과 rendered frame을 각각 검증한다. |
| 12 | fractional CSSOM quantization을 0 tolerance로 비교하는가 | 각 used value에 1/64 CSS px tolerance를 적용한다. |
| 13 | binary fraction의 row 값과 rational 37/3, 80/3을 같은 정확도라고 주장하는가 | JSON은 rational expectation과 관측 CSSOM 값을 분리한다. |
| 14 | DOM order 역전이 node identity 또는 placement도 바꿔 원인 분리를 깨는가 | A/B/C key와 placement는 유지하고 order만 A/B/C↔C/B/A로 바꾼다. |
| 15 | image가 Grid area로 stretch되어 40px overflow를 숨기는가 | image frame과 definite root border 밖 12px overflow를 성공 조건으로 둔다. |
| 16 | root border stroke를 renderer에서 실제 그린다고 잘못 보고하는가 | border stroke sample을 요구하지 않고 CSS border paint 한계를 적었다. |
| 17 | border 밖 image sample이 screenshot이나 geometry 값으로 대체되는가 | Android PixelCopy와 pre-compositor Metal 표면에서 image 및 뒤 clear를 따로 읽는다. |
| 18 | 기본 앱 artifact에 fixture FFI/payload가 포함되는가 | default product와 opt-in positive-control artifact를 따로 검사한다. |
| 19 | 미등록 scene에서 일부 item만 LayoutOutput으로 반환하는가 | exact allowlist와 partial-input/output fail-closed 기준을 둔다. |
| 20 | 이 제한 fixture를 전체 C11.4/WPT/실기기 지원 완료로 확대 해석하는가 | status count, C11.4 체크, 물리 GPU/WPT/compositor 경계를 고정했다. |

## 결론

기준 입력, independent expectations, CSSOM tolerance, simulator pixel 경계와 fail-closed 조건이 구현 계획으로 검토 가능하다. 계획을 수정할 차단 누락은 발견하지 않았다. 구현 뒤에는 이 표를 재사용하지 않고 새 runtime failure review를 작성한다.
