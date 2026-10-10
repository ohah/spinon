# C09.3 병합 뒤 공식 문서 동기화 검토

- 대상 병합: [PR #111](https://github.com/ohah/spinon/pull/111), merge commit `21d147a3842399a653cfd75543c655494ff67168`
- 목적: 구현 PR이 병합된 뒤 상태 대장·계획·내부 계약·인덱스와 실행 근거의 상태를 맞추고 지원 범위를 과장하지 않는지 확인
- 원칙: C09.3 완료만 체크하며 C09 상위와 C09.4의 미완료 상태는 유지한다.

| # | 확인 관점 | 동기화 결과 |
| --- | --- | --- |
| 1 | 상위 상태 조기 완료 | C09 상위 체크는 미완료로 유지했다. C09.4가 남아 있다. |
| 2 | 구현 항목 체크 누락 | C09.3만 PR #111 병합 근거와 함께 완료로 표시했다. |
| 3 | C09.1 병합 정보 변형 | PR #107, 10개 case·30개 node 기록을 유지했다. |
| 4 | C09.2 병합 정보 변형 | PR #109, 16개 case·57개 node 기록을 유지했다. |
| 5 | C09.4 조기 완료 | shrink-to-fit은 미구현으로 유지했다. |
| 6 | 기준 artifact 규모 혼동 | reference v2 전체 30개 case·103개 node, C09.3은 4개·16개로 구분했다. |
| 7 | 비교 브라우저 drift | Chromium `154.0.8037.98` 고정을 유지했다. |
| 8 | DPR/device-pixel 혼동 | 비교 scale DPR 1·2, 결과는 CSS px임을 유지했다. |
| 9 | 허용 오차 확대·누락 | field별 `0.5 CSS px` 기준이 계약·계획·구현 근거에 유지됐다. |
| 10 | 내부 계약 버전 상승 | `0047`의 숫자 버전 `0.1.0`을 유지했다. |
| 11 | 내부 fixture를 공개 API로 오인 | 문서의 공개 API 아님 표기를 유지했다. |
| 12 | C09 profile 범위 확대 | `flow-root`는 `RuntimeBlockFormattingV1`에만 허용된다고 계약이 설명한다. |
| 13 | 다른 profile 누수 | C08 Block paint 등 다른 profile의 fail-closed 경계를 유지했다. |
| 14 | 모든 BFC 기능 지원으로 과장 | overflow·positioned·float·flex/grid·table의 다른 context 생성 경계는 미지원으로 남겼다. |
| 15 | 후속 dependency 누락 | C09.4의 C12/C14/C15/C26 및 intrinsic measurement 선행 관계를 보존했다. |
| 16 | 실제 render 경로 과장 | V8→Stylo→Taffy→WGPU 시뮬레이터 smoke로만 기록했다. |
| 17 | 양 플랫폼 수치 불일치 | Android/iOS의 root, FlowRoot, child, following sibling CSS frame 결과를 동일하게 기록했다. |
| 18 | Android 실기기·성능 일반화 | API 37 emulator 범위만 명시하며 실기기·성능 주장은 하지 않는다. |
| 19 | iOS 실기기 일반화 | iPhone 17 Pro / iOS 26.2 Simulator 범위만 명시한다. |
| 20 | 근거 링크 끊김·누락 | 계획, 계약, STATUS, 내부 index가 구현 검토 문서와 PNG/log 파일을 연결한다. |

검토 결과: 문서 동기화 PR에서 C09.3 병합 상태만 갱신하고 CSS 지원 범위·C09 상위 상태·`0.1.0` 버전은 확대하지 않는다.
