# C10.3.4 계획 범위 수정 PR 검토

## 검토 범위

`plan/c10-3-flex-order-alignment.md`의 C10.3.4 범위 문구와 계획 재검토 근거 파일 링크를 검토한다. 이 PR은 계획만 수정하며 fixture, Chromium reference, Rust 구현, 상태 완료 표시를 포함하지 않는다.

## 실패 관점 검토

| # | 확인 관점 | 결과 |
|---:|---|---|
| 1 | 문서 제목과 C10.3.4 범위가 일치하는가 | baseline 계획 범위 수정으로 한정되어 있다. |
| 2 | 기존의 `last baseline` 제외 문장이 남아 새 범위와 충돌하는가 | 해당 제외 문장을 제거했다. |
| 3 | `baseline`과 `first baseline`의 의미가 섞이는가 | computed alias와 authored keyword를 구분했다. |
| 4 | `last baseline`이 first 값으로 축약될 수 있는가 | 별도 값 보존을 명시했다. |
| 5 | `place-items`·`place-self`가 입력 경로에서 빠지는가 | 해당 shorthand가 만드는 longhand 경계를 명시했다. |
| 6 | 성공 범위가 실제 텍스트 렌더링까지 과장되는가 | textless fixed-size element box로 제한했다. |
| 7 | 텍스트·글꼴·line box가 C10.3.4 완료로 오인되는가 | C15 선행 계약과 미지원 범위를 명시했다. |
| 8 | vertical writing mode나 fragmentation이 LTR 결과로 간주되는가 | 완료 범위에서 제외했다. |
| 9 | column Flex에 row baseline-sharing 알고리즘을 적용하는가 | computed keyword와 cross-start fallback 경계만 두었다. |
| 10 | `align-content:first baseline`을 줄 간 baseline 정렬로 오해하는가 | 해당 설명을 금지하고 textless geometry만 비교하도록 했다. |
| 11 | `align-content:last baseline`을 지원 문법으로 취급하는가 | invalid declaration과 cascade fallback 비교로 한정했다. |
| 12 | Taffy의 first-only 결과를 last 결과로 재사용하는가 | Taffy 0.14.0의 제약과 adapter 책임을 분리했다. |
| 13 | 중첩 Flex의 last baseline 전파가 빠지는가 | parent 전달을 성공 조건으로 명시했다. |
| 14 | order·wrap·wrap-reverse의 시각 순서가 누락되는가 | 기준 축과 fixture 입력으로 포함했다. |
| 15 | cross-axis margin이 baseline frame 계산에서 누락되는가 | cross-axis margin 관찰을 성공 범위에 포함했다. |
| 16 | 산출할 수 없는 입력을 조용히 flex-start로 처리하는가 | 명시적인 실패 처리로 고정했다. |
| 17 | 계획 수정만으로 상태 대장이 완료 처리되는가 | `spec/STATUS.md`를 변경하지 않았고 구현 미완료를 유지한다. |
| 18 | 내부 계약 버전이 문서 수정 때문에 상승하는가 | 계약 버전 파일을 수정하지 않았다. |
| 19 | 재검토 근거 링크가 존재하지 않거나 계획과 연결되지 않는가 | 계획 재검토 기록을 추가하고 링크했다. |
| 20 | 이전 계획 검토를 구현 검토로 재사용하거나 무관한 파일을 섞는가 | 구현 검토와 분리했고 C07.2·Stylo 미추적 파일은 범위에서 제외했다. |

## 결과와 한계

계획 문서의 서로 모순되던 first/last baseline 범위를 일치시켰다. 이 문서 검토는 Chromium capture, 구현 정확성, WPT, Android/iOS runtime 동작을 증명하지 않는다. 해당 근거는 구현 PR에서 새 fixture와 별도 검토로 추가해야 한다.
