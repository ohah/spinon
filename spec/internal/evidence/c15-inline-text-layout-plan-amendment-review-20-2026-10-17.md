# C15 계획 보완 적대 검토 20개

**검토 대상:** [C15 계획](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md)의 C15.1 span 범위와 `plain-inline-fragment` 판정값<br>
**검토 시점:** 첫 implementation slice의 Agent Browser 관찰 뒤, 다음 구현 단계 전<br>
**판정:** 해결이 필요한 범위 충돌 없음. C15/C16/C19 상태는 미완료로 유지한다.

| # | 독립 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | border가 있는 기존 Chrome fixture를 바꾸어 원본 oracle이 흐려진다 | 원본 fixture와 `21×12px` 두 조각 관찰·screenshot은 그대로 두고, span border를 끈 별도 C15.1 변형을 추가했다. 통과. |
| 2 | 기존 `inline-fragment` 결과를 C15.1 성공으로 재사용한다 | 계획은 기존 border fixture를 C15.2에 남기고 C15.1은 `plain-inline-fragment` ID로 분리한다. 통과. |
| 3 | 첫 C15.1 fixture 수가 표와 불일치한다 | 기존 fixture ID 하나를 대체하며 표의 첫 slice 수는 10개로 유지된다. 통과. |
| 4 | undecorated span 결과를 문자 advance로만 추정한다 | Agent Browser가 Ahem 로드 후 두 span rect 각각 `20×10 CSS px`를 반환했다. 통과. |
| 5 | trailing collapsed space가 첫 줄 fragment에 남는다 | Browser `getClientRects()`와 글자별 Range는 줄 끝 space advance 0을 보였고, layout 회귀 fixture도 이를 요구한다. 통과. |
| 6 | border·padding을 C15.1에서 일부만 조용히 버린다 | 계획은 모든 non-default inline box style을 식별 가능한 오류로 닫고 decoration 계산은 C15.2로 이동한다. 통과. |
| 7 | 중첩 span이 C15.1 profile에 섞인다 | 계획은 한 단계 span만 허용하고 nested inline은 C15.2에서 처리하도록 명시한다. 통과. |
| 8 | inline span 자체의 font/style cascade를 약속한다 | 모든 span은 paragraph의 고정 fixture style을 상속하며 font cascade는 C16으로 남는다. 통과. |
| 9 | span display나 box style을 layout child로 오인한다 | C15.1 source collector는 span을 Taffy box child에서 제거하고 paragraph inline owner로만 보존한다. 통과. |
| 10 | 줄 끝 공백을 glyph advance와 fragment geometry에서 같은 것으로 취급한다 | shaped glyph/run은 보존하지만 visible fragment 폭은 Taffy line의 hanging advance 밖으로 늘리지 않도록 나눠 판정한다. 통과. |
| 11 | fragment 둘을 union rect 하나로 합친다 | 기대값은 `line_index`별 두 rect이며 x 좌표는 container-relative로 비교한다. 통과. |
| 12 | browser viewport 장식 offset을 CSS fragment 좌표로 고정한다 | acceptance 값은 fragment 크기와 line별 개수이고 fixture page의 page offset은 판정값에서 제외한다. 통과. |
| 13 | 폰트 미로드 fallback 값을 oracle로 기록한다 | Agent Browser 관찰에서 Ahem resource 요청은 성공했고 `document.fonts.check("10px Ahem")`은 true였다. 통과. |
| 14 | 새 geometry 결과를 Chrome full-version 전체 동작으로 확대한다 | 결과는 단일 local span 변형이며 C15.1 고정 fixture 범위에만 쓴다. 통과. |
| 15 | 새 변형 때문에 사전 비교 screenshot/hash가 낡는다 | 기준 screenshot은 원본 border fixture 상태로 보존하고 변형은 별도 evidence로 적는다. 통과. |
| 16 | C15.2의 장식 구현 조건이 사라진다 | C15.2 표와 단계 설명에 line fragmentation 및 각 fragment의 border/padding contribution이 유지된다. 통과. |
| 17 | nested inline 보류가 오류 없이 fallback된다 | 입력이 nested span 또는 non-default box style이면 paragraph owner를 지목하는 fail-closed 오류가 필요하다. 통과. |
| 18 | 다른 display/overflow/direction 값을 기본 style로 간주한다 | C15.1은 baseline 외 box style을 허용하지 않고 별도 fallback을 정의하지 않는다. 통과. |
| 19 | geometry tolerance와 raster coverage가 합쳐진다 | `20×10 CSS px` geometry는 `0.05px` tolerance로, glyph coverage는 Ahem 고정 fixture의 별도 pixel gate로 둔다. 통과. |
| 20 | 계획 수정으로 C15 전체 상태가 올라간다 | C15.1는 shape/layout 내부 slice이고 painter·runtime·platform gate가 남아 상위 C15/C16/C19는 미완료다. 통과. |

원본 precomparison과 새 단순-span 관찰은 별도 입력·판정값으로 유지한다. Browser-only 관찰은 Spinon runtime, glyph paint 또는 Android/iOS Simulator 검증을 대신하지 않는다.
