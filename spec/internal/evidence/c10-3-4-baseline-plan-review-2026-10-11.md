# C10.3.4 기준선 정렬 계획 검토

## 검토 범위

기존 계획의 `baseline-empty-boxes`와 `baseline-content-lines`를 구현 전에 검토했다. 계획의 문장만 되풀이하지 않고 CSS 기준, 고정 Chromium 관찰값, 잠긴 Taffy `0.14.0`의 실제 구현 경계를 대조했다. 이 문서는 계획 검토이며 기능 구현이나 WPT 통과를 뜻하지 않는다.

고정 기준은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport `320×240 CSS px`, DPR 1·2, `horizontal-tb`·LTR다. 같은 HTML에서 두 DPR의 CSS px 좌표가 같았다.

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 검토 결과와 계획 반영 |
|---:|---|---|
| 1 | `baseline`과 `first baseline`을 다른 값으로 저장하는가 | Chrome은 둘 다 computed value를 `baseline`으로 직렬화했다. 두 authored 값은 별도 입력으로 두되 computed 계약은 first-baseline alias로 기록한다. |
| 2 | `last baseline`을 `baseline`의 별칭으로 축약하는가 | Chrome은 `align-items:last baseline`을 지원하고 computed value를 유지한다. 아래 좌표도 first와 다르므로 별도 typed variant가 필요하다. |
| 3 | first·last baseline 참여자를 한 그룹으로 합치는가 | 첫 그룹의 빈 상자 세 높이 20·40·30px은 y=`20/0/10`, 마지막 그룹은 y=`80/60/70`이었다. 각각의 baseline 결과를 독립적으로 계산한다. |
| 4 | baseline 참여자가 한 개일 때도 공유 그룹이 있다고 가정하는가 | 높이 20px 항목 하나와 `flex-start` 형제를 둔 결과 first baseline은 y=0, last baseline은 y=80이었다. first/last fallback을 따로 보존한다. |
| 5 | baseline 비참여 항목을 공유 baseline 계산에 넣는가 | Flex 알고리즘상 비참여 항목은 공유 baseline을 정하지 않지만 line cross size에는 영향을 줄 수 있다. 참여 여부와 line 크기 기여를 분리해 검증한다. |
| 6 | baseline 그룹을 Flex line 전체가 아닌 컨테이너 전체로 묶는가 | 공유 그룹은 같은 Flex line 안에서 구성된다. wrap fixture마다 line 단위로 참여자와 fallback을 검사한다. |
| 7 | 컨테이너 first baseline이 첫 DOM 자식으로 고정되는가 | 기준 규칙은 order-modified 순서와 시각적 startmost line을 사용한다. 중첩 Flex fixture에서 child order, `order`, line 선택을 각각 비교한다. |
| 8 | 컨테이너 last baseline이 first line을 재사용하는가 | endmost line과 그 line의 endmost item을 선택해야 한다. Taffy `0.14.0`은 Flex `LayoutOutput`에 first만 내므로 이를 지원한다고 간주하지 않고 별도 출력 경로를 검증한다. |
| 9 | `order`가 baseline 선택에 반영되지 않는가 | 기준 문서는 baseline 선택 전에 order 재배치를 명시한다. HostDocument 순서를 바꾸지 않은 채 계산 순서만 바꾼 중첩 fixture를 추가한다. |
| 10 | `row-reverse`를 cross-axis baseline 방향으로 오인하는가 | `row-reverse`는 주축 item 순서만 바꾸며 baseline 측정축을 바꾸지 않는다. main-axis 순서와 cross-axis 좌표를 따로 관찰한다. |
| 11 | `wrap-reverse`에서 startmost/endmost line을 DOM 첫·끝 line으로 처리하는가 | wrap-reverse는 시각적 cross-axis line 순서를 뒤집는다. first/last 컨테이너 baseline 선택과 item fallback을 모두 fixture화한다. |
| 12 | `column`의 computed baseline을 row와 같은 기하로 적용하는가 | 고정 Chrome에서 column `align-items:baseline`은 computed keyword를 유지하지만 textless item은 cross-start에 놓였다. column의 이 fallback을 성공적인 row baseline 정렬로 오인하지 않는다. |
| 13 | `align-content:first baseline`을 줄끼리 baseline 정렬하는 값으로 해석하는가 | Box Alignment의 baseline content-alignment는 Flex item 내부 콘텐츠를 다루며 line 간 baseline-sharing group과 다르다. 두 동작을 계획과 fixture에서 분리한다. |
| 14 | empty fixed-size item의 line 위치만으로 baseline content-alignment를 검증하는가 | Chrome은 테스트한 두 줄에서 computed `baseline`을 반환했지만 좌표는 `flex-start`와 같았다. textless empty content에 한정된 관찰로 기록하고 실제 text 동작을 주장하지 않는다. |
| 15 | `align-content:last baseline`을 Chrome에서 유효한 값으로 받는가 | pinned Chrome의 `CSS.supports()`는 false였고 computed value는 `normal`이었다. invalid declaration 뒤의 이전 유효 선언 보존을 확인하며 이 값을 성공 경로로 내보내지 않는다. |
| 16 | `place-items`·`place-self`·`place-content` baseline shorthand를 놓치는가 | shorthand는 각각 align longhand를 설정하고, `place-content`의 생략된 `justify-content`는 baseline 값을 그대로 복사하지 않는다. authored shorthand와 computed longhand를 함께 비교한다. |
| 17 | auto cross-axis margin이 baseline 정렬을 덮어쓰는가 | Flex cross-axis auto margin 해결은 item alignment보다 먼저 수행된다. auto margin 항목은 baseline 참여 결과와 분리해 기존 C10.3.3 경로를 회귀 검사한다. |
| 18 | nonzero cross margin에서 `last baseline`을 단순 `flex-end`로 매핑하는가 | Chrome에서 last group의 border-bottom은 y=93으로 같았지만 `flex-end`는 각 항목의 bottom margin 차이를 그대로 노출한다. Taffy keyword 치환만으로 완료 처리하지 않고 실제 line-end baseline 계산을 요구한다. |
| 19 | 합성 baseline이 padding/border 외곽을 잘못 참조하는가 | CSS 기준은 Flex item의 border edge에서 baseline을 합성한다. C07.2의 used border geometry를 사용하며 border paint를 baseline 근거로 사용하지 않는다. |
| 20 | 텍스트 baseline이나 빈 컨테이너 baseline을 상자 높이로 추정하는가 | text baseline은 C15 글꼴·line layout 결과가 필요하고 빈 Flex container는 alignment context가 요구할 때만 baseline을 합성한다. 해당 경로를 성공 처리하지 않고 별도 경계·오류 fixture로 둔다. |

## 계획에 반영한 수정 사항

1. item self-alignment(`align-items`·`align-self`)와 content-alignment(`align-content`)를 구분한다. `align-content:first baseline`을 줄끼리 정렬하는 기능으로 설명하지 않는다.
2. first와 last baseline을 서로 다른 typed 값과 별도 geometry fixture로 둔다. Taffy의 first-only Flex baseline 출력을 이용해 last baseline 지원을 추정하지 않는다.
3. `align-content:first baseline`은 고정 Chrome의 computed value와 textless empty-box 기하만 기록한다. 실제 baseline content-alignment는 텍스트 측정이 준비되기 전까지 완료 범위가 아니다.
4. `align-content:last baseline`은 고정 Chrome에서 unsupported인 입력으로 처리하고 cascade fallback을 비교한다.
5. row·column, wrap·wrap-reverse, order, 단일 참여자, 비참여 형제, margins, nested Flex container를 최소 교차 행렬로 명시한다. `horizontal-tb`·LTR 밖의 방향성은 C17 선행 범위로 남긴다.
6. C10.3.3 PR #122는 `2026-10-10` 리베이스 병합됐다. 이 계획과 상태 대장의 오래된 “PR 검토 대기” 문구를 같은 변경에서 바로잡는다.

## 확인한 기준

- CSS Flexbox Level 1 §8.3·§8.5 및 §9.4·§9.6: [2025-10-14 Candidate Recommendation Draft](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/).
- CSS Box Alignment Level 3 §4.2·§5.1.3·§5.4·§6.2.4·§9.1: [2026-10-08 Working Draft](https://www.w3.org/TR/2026/WD-css-align-3-20261008/). 초안 문서의 명시적 구현 경고를 최종 표준 주장으로 바꾸지 않는다.
- 잠긴 Taffy `0.14.0`: `AlignItemsKeyword::Baseline`은 있으나 Flex `AlignContentKeyword` baseline 값은 없고, Flex 계산은 row만 baseline-align하며 Flex output은 first baseline만 반환한다.
- 대응 WPT 경로는 `flex-align-baseline-001.html`–`004.html`, `align-items-baseline-row-horz.html`, `align-self-baseline-with-flex-wrap.html` 등이다. 이 검토에서 WPT harness는 실행하지 않았다.
