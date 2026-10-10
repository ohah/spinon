# C10.3.4 범위 모순 수정 후 계획 재검토

## 변경 이유

계획의 앞 절은 `last baseline`을 C10.3.4 범위 밖으로 두면서 뒤 절과 단계표는 first/last baseline 모두를 같은 단계에서 요구했다. 구현 전에 이 충돌을 제거하고 성공 범위·실패 경계·Taffy adapter 책임을 다시 고정했다.

## 수정된 계획의 별도 실패 관점 20개

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | `baseline` computed alias와 authored `first baseline` 입력을 혼동하는가 | Authored 입력과 computed alias를 별도 관찰하고 둘 다 first-baseline self-alignment로 연결한다. |
| 2 | `last baseline`을 `baseline` 별칭으로 저장하는가 | 별도 computed 문자열과 typed 값으로 보존한다. |
| 3 | 예전 문장에 last baseline 미지원이 남아 있는가 | C10.3.4 성공 범위에 last self-alignment를 명시하고 text 외 미지원 경계만 남겼다. |
| 4 | first와 last를 같은 Taffy enum에 넣고 통과 처리하는가 | Taffy Flex output first-only 제약과 adapter 소유 last 계산 요구를 명시했다. |
| 5 | parent가 nested child의 last baseline을 받지 못하는가 | nested first/last propagation을 별도 성공 fixture와 완료 조건에 남겼다. |
| 6 | Flex line 순서를 DOM 원본 순서로만 판단하는가 | order-modified 순서와 시각적 line 순서를 fixture 축으로 요구한다. |
| 7 | wrap-reverse에서 first/last line 선택을 반대로 하는가 | visual cross-axis start/end 기준을 유지한다. |
| 8 | baseline-sharing group을 line 전체와 하나로 합치는가 | first·last 그룹을 같은 line 안에서도 분리한다. |
| 9 | 비참여 형제가 line cross size에 기여하지 않는다고 가정하는가 | 참여와 line-size 기여를 독립적으로 fixture화한다. |
| 10 | single participant에서 공유 정렬이 발생한다고 가정하는가 | single-participant fallback을 독립 case로 남긴다. |
| 11 | cross-axis auto margin과 baseline 정렬이 동시에 적용된다고 가정하는가 | auto margin precedence를 별도 case로 유지한다. |
| 12 | `align-content:first baseline`을 Flex 줄끼리 맞추는 값으로 해석하는가 | content-alignment와 line distribution을 분리한다. |
| 13 | `align-content:last baseline`을 pinned Chrome에서 지원한다고 선언하는가 | invalid declaration과 이전 cascade 값 보존만 성공 경계로 둔다. |
| 14 | `place-items`·`place-self`에서 baseline longhand가 누락되는가 | shorthand 결과도 self-alignment 입력 경로와 같은 계약에 넣었다. |
| 15 | `place-content` shorthand에서 content-alignment 범위를 과장하는가 | computed longhand 경계와 textless geometry만 적용한다. |
| 16 | column에서 row의 baseline 공유 알고리즘을 적용하는가 | computed keyword와 cross-start fallback만 비교하도록 명시했다. |
| 17 | 빈 element 결과를 텍스트 baseline 일반 지원으로 확장하는가 | text node·line box·font shaping을 C15 전까지 제외했다. |
| 18 | border·padding을 빼고 baseline을 상자 바깥에서 합성하는가 | 합성 기준은 border edge이며 C07.2 geometry를 입력으로 요구한다. |
| 19 | RTL·vertical writing mode 지원을 LTR 테스트로 주장하는가 | `horizontal-tb`·LTR만 성공 범위로 고정하고 C17을 선행으로 둔다. |
| 20 | 실패 입력을 평범한 flex-start frame으로 조용히 대체하는가 | baseline 산출 불가능한 입력은 node/property 문맥으로 명시 실패해야 한다. |

## 결과

기존 계획의 모순을 제거했다. 이 검토는 계획 문서만 대상으로 하며 Chromium capture, Rust 구현, WPT 실행 또는 모바일 runtime 통과를 의미하지 않는다.
