# `align-content:safe|unsafe flex-start` 계획 실패 관점 검토 20개

**대상:** [C11.4 parsed Root HTML `align-content:safe|unsafe flex-start` 계획](./c11-root-html-auto-row-safe-flex-start.md)와 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/README.md). 구현/runtime 검증은 아직 수행하지 않았다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | Grid에서 `flex-start` alias를 잘못 해석하는가 | 통과: CSS Box Alignment §4.1의 non-flex equivalence와 Chrome start control을 각각 기록한다. |
| 2 | computed `safe flex-start`가 `safe start`로 normalize돼 identity를 잃는가 | 통과: computed string을 정확히 보존하는 테스트를 요구한다. |
| 3 | computed `unsafe flex-start`가 plain `flex-start`로 normalize되는가 | 통과: safety modifier 포함 exact serialization을 확인한다. |
| 4 | positive free space에서 세 값이 다른 frame을 내는가 | 통과: expected y `[0,10,20]`을 positive scene에 고정했다. |
| 5 | overflow에서 `unsafe flex-start`가 end/center처럼 배치되는가 | 통과: start-side expected y `[0,56,112]`와 `start` control 비교를 둔다. |
| 6 | safe overflow fallback이 `flex-start`와 다른 위치를 만드는가 | 통과: §4.4 fallback과 Chrome overflow observation이 일치한다. |
| 7 | alignment admission이 auto-row track sizing을 바꾸는가 | 통과: 모든 scene의 rows `[10,10,10]`과 두 gutter를 검사한다. |
| 8 | `calc()` percentage basis가 모호한가 | 통과: definite 40px block basis와 사용 gap 46px를 고정했다. |
| 9 | positive control과 overflow가 서로 다른 item shape를 쓰는가 | 통과: 하나의 fixed root와 동일 direct-item order에서 row-gap만 바꾼다. |
| 10 | computed projection과 layout allowlist가 비대칭인가 | 통과: 두 경계의 exact same allowlist를 수용 조건으로 둔다. |
| 11 | 미지원 `safe flex-end`가 우발 admission되는가 | 통과: paired negative control로 계속 거부한다. |
| 12 | baseline 또는 distributed safety modifier가 열리는가 | 통과: baseline 및 `safe space-between`류 negative control을 남긴다. |
| 13 | error가 잘못된 CSS property에 귀속되는가 | 통과: unsupported computed value의 `align-content` attribution을 확인한다. |
| 14 | 실패에서 partial frames/snapshot이 노출되는가 | 통과: RuntimeSession의 atomic rejection을 gate에 명시했다. |
| 15 | 거부 뒤 같은 session이 회복하지 못하는가 | 통과: valid flex-start 요청을 다음에 보내 same-session recovery를 검증한다. |
| 16 | 최근 safe/unsafe start·center·end가 회귀하는가 | 통과: 해당 focused regressions와 row-gap cases를 필수로 둔다. |
| 17 | 테스트가 oracle 장면과 입력을 다르게 해 비교가 무효인가 | 통과: 같은 40×40 root, tracks, children, two row-gap cases를 명시했다. |
| 18 | screenshot을 pixel readback 또는 glyph equality로 과장하는가 | 통과: 화면은 visual bounds에만 사용하고 정확한 좌표는 assertions로 판정한다. |
| 19 | Simulator 시험 후 앱/build/bootstrap이 변경 상태로 남는가 | 통과: 입력 복원 hash 기록을 gate에 포함한다. |
| 20 | bounded slice가 전체 C11.4/C11 완료로 오해되는가 | 통과: 상위 checkbox, `78/214`, contract `0.1.0`은 그대로 둔다. |

계획 검토는 구현 검토나 PR diff 검토를 대체하지 않는다.
