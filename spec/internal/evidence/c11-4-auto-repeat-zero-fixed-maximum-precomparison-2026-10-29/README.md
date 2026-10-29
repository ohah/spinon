# C11.4 자동 반복의 0px 고정 최대값 · Chrome 사전 비교

- **캡처 날짜:** 2026-10-29
- **저장소 기준 revision:** `a85713e81a33bacad9569951a2b414e46e0bc8d8`
- **브라우저:** Agent Browser, HeadlessChrome 156.0.0.0, macOS platform `MacIntel`, device scale factor 1
- **fixture:** [`grid-auto-repeat-zero-fixed-maximum-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-auto-repeat-zero-fixed-maximum-c114-precomparison.html)
- **원본 관찰값:** [`chrome-156-observations.json`](chrome-156-observations.json)
- **화면:** [`chrome-156-precomparison.png`](chrome-156-precomparison.png)
- **SHA-256 manifest:** [`sha256sums.txt`](sha256sums.txt)

## 입력과 독립 기대값

24개 장면은 열·행, `auto`·`min-content`·`max-content`, `auto-fill`·`auto-fit`, 0px·4px 간격의 조합이다. 반복 패턴은 definite한 16px content axis에 `minmax(<minimum>, 0px)` track 하나를 둔다. 12×12px item 하나는 세 번째 반복 track을 차지한다. 직교축과 root 크기도 고정해 JSON에 기록한다.

CSS Grid Level 1 §7.2.3.2는 0으로 나누는 일을 피하도록 자동 반복 횟수 계산에 쓰는 track 크기를 UA가 지정하는 최소값으로 바닥 처리하며, 1px을 제안한다. 이 값을 1px로 두면 `n × 1px + (n − 1) × gap ≤ 16px`을 만족하는 가장 큰 양의 정수가 기대 반복 수다. 따라서 0px 간격은 16개, 4px 간격은 4개다.

## 관찰값

24개 모두 CSSOM에 예상 반복 수를 만들었다. 사용 track 목록에서 item이 놓인 세 번째 track은 item의 12px 최소 크기를 보존해 12px이며 나머지 반복 track은 0px이다. 간격 4px에서 `auto-fill` item은 root 시작점에서 8px 떨어지고, `auto-fit`은 앞의 두 빈 track과 연결 gutter를 접어 offset 0px에 놓는다. 간격 0px에서는 양 mode의 offset이 모두 0px이다. 행 장면도 세로축에서 같은 결과를 냈다.

저장소 루트에서 `python3 spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/verify_precomparison.py`를 실행하면 scene 구성, 반복 수 산술, CSSOM track 크기, item offset을 원본 JSON과 대조한다.

## 증거 범위

이 결과는 고정된 layout/runtime slice를 위한 Chrome oracle이다. Spinon 동작, 일반 CSS admission, 픽셀 출력, 플랫폼 일치, C11.4 전체 완료를 증명하지 않는다. 세 minimum token은 여기서 문법 조합을 넓히기 위한 입력 축이다. item의 12px 최소 크기를 고정했으므로 이번 fixture는 세 token의 일반적인 intrinsic contribution 차이를 구분하지 않는다.

규범 출처: [CSS Grid Layout Module Level 1, §7.2.3.2](https://drafts.csswg.org/css-grid-1/#auto-repeat).
