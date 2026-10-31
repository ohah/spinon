# Negative `text-indent` post-merge status review 20개

구현 PR [#938](https://github.com/ohah/spinon/pull/938)의 병합 후 official status, limitation boundary, evidence link만 별도로 대조했다. 이 검토는 구현 PR의 30관점 diff 검토를 재사용하지 않고 C11 진행 상황을 과장할 수 있는 상태·문구 오류를 대상으로 한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR이 열린 상태인데 병합으로 잘못 기록되는가 | GitHub PR state는 `MERGED`다. 통과. |
| 2 | 문서의 merge SHA가 다른 커밋을 가리키는가 | PR merge commit `8c5ca0c62ed030642455f53122928b25bba80fda`를 문서와 대조했다. 통과. |
| 3 | 병합 시간이 local/UTC 변환에서 바뀌는가 | GitHub의 `2026-10-31T18:34:54Z`를 그대로 기록했다. 통과. |
| 4 | 빈 check rollup을 CI 성공으로 오인하는가 | check 결과가 보고되지 않았다고 적고 CI pass를 주장하지 않는다. 통과. |
| 5 | 구현 후보 entry가 `spec/STATUS.md`에서 빠졌는가 | PR #938과 실행 evidence가 C11.4 이력에 연결됐다. 통과. |
| 6 | 상위 C11 checkbox가 slice 때문에 완료 처리되는가 | C11 checkbox는 `[ ]` 상태다. 통과. |
| 7 | 하위 C11.3 전체가 완료로 바뀌는가 | C11.3 checkbox는 `[ ]` 상태다. 통과. |
| 8 | template areas candidate C11.3.3가 완료로 바뀌는가 | C11.3.3 checkbox는 `[ ]` 상태다. 통과. |
| 9 | C11.4 intrinsic track sizing 전체가 완료로 바뀌는가 | C11.4 checkbox는 `[ ]` 상태다. 통과. |
| 10 | 최근 simulator slice를 공식 체크 개수에 더하는가 | 전체 공식 집계는 `72/213` 그대로다. 통과. |
| 11 | 체크되지 않은 C11.2.2/C11.2.3 밖의 작업도 숨겨 완료로 표시하는가 | C11 안에서 `[x]`인 명시 체크는 C11.2.2와 C11.2.3 두 개며 parent C11은 미완료다. 통과. |
| 12 | 고정 fixture 결과를 일반 CSS admission으로 표현하는가 | 상태·계획·계약은 opt-in fixture slice라고 한정한다. 통과. |
| 13 | `-5px`/`-10px` 이외 임의 음수 길이를 허용했다고 쓰는가 | 값은 고정 scene allowlist 두 개에 한정한다. 통과. |
| 14 | `-30px`, percentage, `hanging`, `each-line` 결과가 지원으로 승격되는가 | 모두 fixture 범위 밖 또는 거부 경계로 명시했다. 통과. |
| 15 | Android AVD를 실기기 또는 hardware GPU라고 부르는가 | Android 근거는 API 37 ARM64 AVD라고 명시한다. 통과. |
| 16 | screenshot을 glyph paint나 pixel parity 근거라고 과장하는가 | renderer 캡처가 검으며 layout-ready status만 보여준다고 명시했다. 통과. |
| 17 | Android의 수정 전 capture를 최종 결과로 인용하는가 | 수정 후 APK hash와 최종 run `1793471314021-0e2d635a`를 가리킨다. 통과. |
| 18 | iOS 결과가 Android adapter 수정 후 실행된 것으로 잘못 표시되는가 | iOS evidence를 별도 Simulator 결과로 기록하고 Android 재실행과 구분한다. 통과. |
| 19 | archive scan을 APK/IPA 전체 제외 검사로 일반화하는가 | static archive 경계에만 한정하고 앱 bundle exclusion은 명시적으로 제외한다. 통과. |
| 20 | 내부 계약 버전 또는 후속 계획 링크가 잘못 바뀌는가 | 계약 0059는 0.1.0이며 negative follow-up 계획과 리뷰 evidence가 연결된다. 통과. |

병합 후 확인에서 C11/C11.4 checkbox와 공식 `72/213`은 바뀌지 않았다. 이 후속 PR의 결과는 제한 fixture 완료이며 C11 완료가 아니다.
