# C10.3.1 Flex 역방향 구현 · 실패 관점 검토와 실행 근거

검토 범위는 C10.3.1 구현 브랜치의 Stylo typed projection, Taffy 방향·줄 배치, Runtime V8/FFI 경로, Android·iOS WGPU fixture, 비교 기준·로그와 상태 문서다. 검토 과정에서 확인된 문제는 수정하고 관련 관점을 다시 확인했다.

| # | 실패 관점 | 확인한 반례 또는 위험 | 결과와 수정 |
|---:|---|---|---|
| 1 | 주축 반전 | `row-reverse`가 source child 배열을 역순으로 바꾸면 DOM 순서와 NodeId가 손실될 수 있다. | 원본 preorder를 유지하고 모든 reference node의 keyed frame을 검사한다. |
| 2 | row main-start/end | `justify-content:flex-start`와 `flex-end`가 reverse 축에서 잘못된 모서리를 사용할 수 있다. | `row-reverse-start/end`를 Chromium 좌표와 개별 비교했다. |
| 3 | column main axis | `column-reverse`를 x 축에 적용하거나 높이 기준 배치를 놓칠 수 있다. | column 기준 40×20 item의 y 좌표와 single item을 포함했다. |
| 4 | row wrap-reverse | line의 교차축만 역전하지 않고 줄 안 item 수집 순서까지 반전할 수 있다. | 세 줄의 y 좌표와 각 line item 위치를 따로 비교했다. |
| 5 | column wrap-reverse | 세로 main axis를 가진 container의 line 교차축 x 위치를 잘못 적용할 수 있다. | 두 column line의 x와 line 안 y 좌표를 별도 검증했다. |
| 6 | reverse 조합 | `row-reverse + wrap-reverse`가 한 축만 반전하거나 line order를 두 번 반전할 수 있다. | combined row/column fixture를 고정 Chromium과 비교했다. |
| 7 | shorthand token 순서 | `flex-flow:row-reverse wrap`과 토큰 순서가 바뀐 선언의 계산 결과가 달라질 수 있다. | 두 토큰 순서를 모두 reference에 고정하고 Rust cascade 검사에 포함했다. |
| 8 | shorthand/longhand cascade | shorthand 뒤의 `flex-direction` 선언이 이기지 못할 수 있다. | longhand override 결과를 reference computed style 및 frame으로 검사했다. |
| 9 | 정확한 줄 적합 | gap까지 포함한 item 폭이 정확히 맞는 경계에서 불필요하게 wrap할 수 있다. | exact-fit fixture를 손계산 좌표와 Chromium frame으로 확인했다. |
| 10 | 경계 초과 wrap | 1 CSS px 초과를 같은 line에 넣거나 source 두 번째 item의 줄 순서를 바꿀 수 있다. | 1px-over fixture에서 두 번째 item의 줄 위치를 확인했다. |
| 11 | 중첩 Flex | 부모 reverse 상태를 자식 container에 유출하거나 nested direction을 덮어쓸 수 있다. | 서로 다른 parent/child direction의 node frame을 별도로 비교했다. |
| 12 | `display:none` | 숨긴 item이 line 공간을 차지하거나 다른 sibling frame을 밀 수 있다. | 숨긴 node의 0 frame과 남은 source item 배치를 reference에서 고정했다. |
| 13 | 빈·단일 container | 빈 line 또는 item 하나에서 reverse 보정이 잘못 적용될 수 있다. | empty row와 single column reverse를 포함해 모든 node를 비교했다. |
| 14 | profile 누수 | legacy `FlexAlignmentV1`에 reverse 값이 성공 투영될 수 있다. | 해당 profile의 `row-reverse`는 NodeId·property를 가진 오류로 거부한다. |
| 15 | runtime profile 불일치 | 여섯 runtime profile 중 한 profile만 reverse 값을 누락할 수 있다. | 여섯 profile 모두 `flex-flow:row-reverse wrap-reverse`의 computed longhand와 Taffy frame을 검사했다. |
| 16 | RTL 미검증 누수 | reverse mapping이 LTR fixture에서만 검사됐는데 `direction:rtl`을 조용히 처리할 수 있다. | 공격 검토에서 누수를 찾아 reverse+RTL을 fail-closed로 변경하고 회귀 테스트를 추가했다. |
| 17 | percentage basis·gap 축 | `row-reverse` 또는 `column-reverse`에서 주축 gap의 definite axis 검사가 틀릴 수 있다. | reverse variant의 주축 width/height 선택을 기존 definite-basis 분기와 동일하게 처리했다. RTL은 별도 거부한다. |
| 18 | runtime fixture 입력 누락 | runtime app의 margin이 Chromium baseline에 빠지면 화면과 기준이 달라질 수 있다. | 초기 비교에서 8px margin 누락을 확인해 runtime computed properties와 reference를 갱신한 뒤 17 case 비교를 재실행했다. |
| 19 | 플랫폼 viewport 경쟁 | Android surface가 초기 기본 크기 `301×100`으로 cascade 뒤 바뀌면 실행 순서에 따라 frame이 달라질 수 있다. | C10.3.1 runtime fixture가 처음부터 `320×240 CSS px`를 사용하도록 고정하고 재빌드·재실행했다. |
| 20 | stale draw 및 로그 잘림 | 새 scene이 오래된 draw를 supersede하면 `-12`가 queue failure로 표시될 수 있고 iOS 장문 report는 frame을 잘라낸다. | 두 플랫폼에서 stale draw 뒤 최신 draw를 재요청한다. iOS는 node별 frame 로그와 개수 요약을 추가했고, Node 회귀 검사가 실제 로그의 7개 frame·WGPU 성공을 기준과 비교한다. |

## 다시 확인한 결과

- 고정 Chrome 154 reference는 17 case·64 node, DPR 1·2다. Taffy Rust 비교에서 각 case의 모든 node와 네 frame field가 `0.5 CSS px` 허용치 안에 들었다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8 평가 `status=0`, DOM node 7개, source preorder NodeId 1–7, WGPU 제출 7 box를 확인했다. 각 CSS frame은 runtime Chromium reference와 일치한다.
- iOS의 OS 로그는 한 report가 길 때 뒤쪽 frame marker를 잘라낼 수 있다. 별도 로그 7개를 기록하도록 바꾼 뒤 수집한 로그에 NodeId 1–7과 `frames=7`이 있다.
- 최종 변경을 포함해 `cargo test --locked --workspace --all-targets` 전체 453개 통과·2개 기존 무시, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`, `node --test tools/css-reference/c10-3-1-flex-reverse.test.mjs` 5/5를 확인했다.
- 최종 변경으로 Android와 iOS Simulator 앱을 다시 빌드·실행하고 화면과 로그를 갱신했다. Android 빌드는 성공했으며 AGP가 compile SDK 37.2를 시험한 버전은 36.1까지라는 경고가 남는다. iOS 빌드도 성공했고 정적 라이브러리 중복 심볼 경고가 남지만 실행에서 7개 상자 제출을 확인했다.
- WPT 링크는 규칙별 상류 대응 경로다. 이 제한 fixture에서는 WPT reftest 자체를 실행했다고 주장하지 않는다. RTL, writing mode 전환, 텍스트, `order`, 정렬·baseline, 실기기와 hardware GPU 성능도 이번 결과에 포함되지 않는다.

## 명세 정합성

[구현 계약 0050](../0050-c10-3-1-flex-reverse.md), [공식 상태 대장](../../STATUS.md), [C10.3 계획](../../../plan/c10-3-flex-order-alignment.md), [Chromium 사전 기준](./c10-3-1-flex-reverse-precomparison-2026-10-10.md)을 서로 대조한다. 하위 C10.3.1만 구현 검증 완료로 표시하고 C10.3 parent 및 다른 하위 단계는 미완료로 둔다. 패키지·내부 계약 숫자 버전은 `0.1.0`으로 유지한다.
