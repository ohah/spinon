# iOS C11.4 fixture startup comparison

현재 source `9b124aeab080711028f1198686b8818f26aed122`, iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`에서 fixture process를 매회 fresh launch로 시작했다. 고정 scene에 대해 아래 세 cohort를 각각 5회 실행했다.

| Cohort | Scene | Summary |
| --- | --- | ---: |
| 기존 대조 | `text-transform-none-min-content` | 5/5 |
| 추가 원문 대조 | `text-transform-capitalize-none-min-content` | 5/5 |
| 신규 변환 | `text-transform-capitalize-min-content` | 5/5 |

각 실행은 `SPINON_C114_GRID_SUMMARY`의 `status=0 layout=ready`를 남겼고 이 15회에서는 superseded evaluation이 없었다. machine-readable run IDs는 `summary.json`, raw logs는 `existing-none.log`, `added-none.log`, `capitalize.log`에 보존했다.

이 결과는 시작 안정성 비교다. 이전 `ios-simulator-pass01`과 commit-bound `ios-simulator-pass03`의 `status=-12` 실패는 여전히 유효한 관찰이며 이 15회나 `ios-simulator-pass04` 성공이 실패를 고친 것은 아니다. 원인은 아직 특정하지 않았고, 전체 six-scene 독립 capture 결과는 별도 [runtime evidence](../README.md)에 있다.
