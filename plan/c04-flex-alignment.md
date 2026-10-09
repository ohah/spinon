# C04.3 · Flex 정렬 전달

- **문서 유형:** 구현 계획과 비교 기준 · 구현 상태는 [상태 대장](../spec/STATUS.md)에서 확인
- **기준일:** 2026-10-09
- **상위 항목:** [구현 상태 대장 C04](../spec/STATUS.md#css-구현-체크리스트)
- **관련 내부 계약:** [0017 C04.2 computed style→Taffy adapter](../spec/internal/0017-c04-style-layout-bridge.md)

## 목적과 범위

C04.2에서 Stylo가 계산한 `align-items`와 `justify-content`가 Taffy 입력에서 고정값으로 바뀌는 구간을 제거한다. 새 내부 computed-style profile `FlexAlignmentV1`을 추가해 기존 `FlexLayoutV1`의 입력·출력을 유지하면서 아래 값만 Stylo→`LayoutStyle`→Taffy로 전달한다.

| 속성 | 지원 값 | 정규화 |
|---|---|---|
| `align-items` | `normal`, `stretch`, `flex-start`, `flex-end`, `center` | Flex 컨테이너에서 `normal`은 `stretch` |
| `justify-content` | `normal`, `flex-start`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly` | `normal`은 `flex-start` |

`baseline`, `start`, `end`, `left`, `right`, `safe`/`unsafe` 조합은 이 profile에서 실패시킨다. `align-self`, `align-content`, wrapping, reverse 방향, Grid는 이 변경의 지원 범위가 아니다. 선언을 조용히 버리지 않고 전체 계산을 실패시킨다. 이 내부 fixture slice는 공개 CSS 지원이나 Android·iOS runtime 지원을 의미하지 않는다.

## 비교 모델

- 기준은 설치된 Chrome `154.0.8037.98` / revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`이다. 실행 환경·실행 파일·fixture·CSS·캡처 도구 hash는 [고정 reference JSON](../tests/fixtures/css/references/c04-flex-alignment-v1.json)에 기록한다. C01.2의 `.95` 자료를 새 기준인 것처럼 재사용하지 않는다.
- 고정 viewport와 Flex 컨테이너는 모두 `301×40` CSS px, 배율 `1`, `screen`, `en-US`, `UTC`다. 컨테이너는 `column-gap:5px`, 세 자식으로 구성한다.
- 자식은 grow/shrink를 끄고 main-axis basis와 cross-axis 크기를 지정해 alignment만 관찰한다. `normal`/`stretch`의 자동 cross-size, 행·열 축, LTR·RTL, 모든 허용 분배 값을 각각 관찰한다.
- `align-items`·`justify-content`의 computed value는 Chromium 문자열과 정확히 비교한다. 각 자식과 부모의 상대 `x`, `y`, `width`, `height` 오차는 개별 좌표마다 `0.5 CSS px` 이하여야 하며 평균으로 실패를 상쇄하지 않는다.
- 유효 여유 공간이 있는 fixture로 분배 동작을 비교한다. 음수 여유 공간의 안전 정렬, 줄바꿈, baseline, reverse, min-size 자동값은 이번 판정에 포함하지 않는다.

## 구현 순서와 완료 조건

1. 계획을 별도 20개 실패 관점으로 점검하고 고친 뒤 고정 Chromium reference를 만든다.
2. `spinon-layout`에 엔진 중립 정렬 enum을 추가한다. 기본값은 현재 관찰 동작인 `stretch`/`flex-start`로 유지한다.
3. `spinon-style`에 새 `FlexAlignmentV1` cascade profile을 추가한다. 기존 `FlexLayoutV1` whitelist와 출력은 바꾸지 않는다.
4. `spinon-style-to-layout`에 새 entrypoint와 엄격한 값 parser를 추가해 reference fixture를 통과시킨다. 미지원·진단·inline style은 전체 실패한다.
5. 새 버전의 내부 계약과 실행 근거를 추가하고 `spec/STATUS.md`의 C04.3 하위 항목만 완료한다. C04·S02 전체나 공개 CSS 지원은 완료 처리하지 않는다.
6. 기존 C04.2 회귀, 새 Chromium geometry, 미지원 값, stale revision, 이전 profile 분리를 실행하고 구현에 대해 별도의 20개 실패 관점을 점검한다.

## 계획 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 계획에 반영한 방지 기준 |
|---:|---|---|
| 1 | `align-items: normal`을 CSSOM 값 `stretch`로 잘못 직렬화 | computed 문자열과 layout 정규화를 분리하고 각각 Chromium·Taffy에서 확인한다. |
| 2 | `justify-content: normal`을 stretch로 해석 | 속성마다 다른 Flex 초기 동작을 명시하고 `flex-start`로 매핑한다. |
| 3 | 논리 `start`/`end`를 `flex-start`/`flex-end`와 혼동 | 이번 profile에서 논리 값은 거부한다. |
| 4 | baseline alignment를 단순 중앙/끝으로 대체 | baseline은 미지원 오류로 남긴다. |
| 5 | `safe`·`unsafe` modifier를 무시해 overflow 의미 변경 | modifier가 있는 값 전체를 거부한다. |
| 6 | 정렬 속성을 자식 자신에 적용 | fixture에서 부모 computed value와 자식 frame을 별도로 검사한다. |
| 7 | `align-self`/`align-content` 지원을 암묵적으로 주장 | 새 profile whitelist에 포함하지 않고 명시적 거부 사례를 둔다. |
| 8 | `place-items` 같은 shorthand가 일부만 적용 | 허용 선언을 확장 결과까지 검사하고 누락 longhand를 실패 처리한다. |
| 9 | gap을 남는 여유 공간에서 빼지 않아 분배가 달라짐 | 고정 gap이 있는 Chromium geometry로 각 분배 값을 비교한다. |
| 10 | flex-grow가 여유 공간을 소비해 justify 차이를 숨김 | 자식 grow/shrink를 0으로 고정하고 computed 값을 검사한다. |
| 11 | flex-basis와 width의 우선순위가 fixture를 바꿈 | 고정 basis를 두고 computed `flex-basis`를 확인한다. |
| 12 | auto cross-size의 stretch 동작이 빠짐 | auto 높이·auto 너비 fixture를 각각 둔다. |
| 13 | 행 축만 시험하고 열 축의 주·교차축을 뒤바꿈 | `column`에서 justify와 align을 각각 관찰한다. |
| 14 | RTL `flex-start`가 물리적 좌측으로 고정 | RTL 행 fixture를 포함하고 row-reverse와 구분한다. |
| 15 | 자식 순서 또는 gap이 분배 중 바뀜 | 세 자식 ID 순서와 모든 개별 frame을 비교한다. |
| 16 | CSS computed 값이 Taffy enum과 다르게 정규화됨 | 허용 값 문자열 정확 비교와 frame oracle을 함께 요구한다. |
| 17 | 알 수 없는 유효 값이 기본값으로 조용히 대체됨 | 값 parser는 node·property·원본 값을 포함해 실패한다. |
| 18 | 스타일 진단·inline style에서 일부 결과를 반환 | 기존 adapter의 전체 실패 경계를 새 entrypoint에서도 유지한다. |
| 19 | 새 profile 추가로 C04.2 결과가 변함 | 기존 public entrypoint·profile·oracle을 그대로 두고 회귀 시험한다. |
| 20 | fixture 통과를 제품·모바일 전체 지원으로 과장 | C04.3 내부 완료만 기록하고 C04·S02·공개 API 상태는 미완료로 둔다. |

이 표는 계획 단계 검토다. 구현 코드 검토에는 재사용하지 않는다.
