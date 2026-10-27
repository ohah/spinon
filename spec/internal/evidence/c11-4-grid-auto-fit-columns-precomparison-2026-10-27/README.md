# C11.4 · WPT `grid-auto-fit-columns-001.html` 사전 비교

## 입력과 출처

- WPT checkout: `build/wpt-c124-source`, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- 대상: `css/css-grid/grid-definition/grid-auto-fit-columns-001.html` 전체. `expected.json`은 WPT의 `data-offset-*` 및 `data-expected-*` 항목을 그대로 뽑아 35개 root·37개 item의 oracle로 만든다. [재현기](./derive_expected.py).
- auto-repeat count 기대값은 별도 [반복 횟수 표](./repeat-count-expected.json)와 [재현기](./derive_repeat_expectations.py)에 고정했다. 재현기는 pinned HTML·`grid.css`의 hash와 각 track-list CSS rule을 확인하고, authored sizing function과 200px definite content box에서 CSS Grid §7.2.3.2의 count 크기를 분수 산술로 계산한다. Chrome의 used track 크기나 count에서 역산하지 않는다.
- WPT stylesheet·harness와 원본 파일의 SHA-256은 [source-manifest.json](./source-manifest.json)에 있다.
- 독립 normative source: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1–§7.2.4, §8.3–§8.5, §11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).

## Chrome 관측

Agent Browser `0.31.1`의 HeadlessChrome `156.0.0.0`, viewport `1280×633` CSS px, DPR 1에서 원본 WPT와 harness를 실행했다. Harness는 35개 test를 찾았고 35/35 Pass, 0 Fail, 0 Skip이었다. root 35개와 child 37개의 computed track list, root/item frame, authored placement와 WPT expectation은 [chrome-observation.json](./chrome-observation.json)에 보존했다. 전체 실행 화면은 [chrome-wpt-full-35-results.png](./chrome-wpt-full-35-results.png)다.

이 자료는 Chrome에서 고정 WPT를 실행한 결과다. Chrome 통과는 Spinon 적합성이나 앱 동작을 뜻하지 않는다. `gridTemplateColumns`는 Chrome 관측이고 CSS 규범 기대값으로 역산하지 않는다. Item frame의 독립 기대값은 Chrome 결과가 아니라 WPT source의 data attributes다. WPT helper는 각 offset/size 차이가 1 CSS px 이상일 때 실패시키므로 구현 acceptance에서는 정수 expected frame을 별도로 비교해 더 엄격한 경계를 사용한다.

## Runtime adapter 입력 투영

scene table은 WPT의 `grid.css`, 문서 class rule, direct item의 inline placement를 원문 hash 확인 뒤 고정 입력으로 투영한다. Android API 37 ARM64에서 scene 01을 실행하자 V8/DOM 경로 뒤 CSS cascade가 실패했다. WPT의 `background: cyan`이 지원되지 않는 `background-position-x`를 만들기 때문이다. 대상 문서는 단색 `cyan`/ `green`만 지정하므로 adapter가 이 두 shorthand를 동일 색의 `background-color`로 제한 정규화한다. 일반 CSS admission은 변경하지 않는다. 오류 원본은 [첫 Android 실행 로그](../c11-4-grid-auto-fit-columns-runtime-2026-10-27/android-grid-auto-fit-01-pass1.log)에 보존한다. 이 투영을 전체 WPT runner 실행으로 확대 해석하지 않는다.

## 고정된 알고리즘 경계

- Auto-repeat은 definite repeat size와 gap을 포함해 content box 안에 들어가는 양의 최대 반복 수를 고른다. 양쪽 sizing function이 definite이면 max를 min보다 낮지 않게 본다.
- 2번 case는 CSS `width:200px`인 content box에 좌우 13px padding이 있다. Padding은 grid content box 원점을 이동시키고 외곽 border box를 넓히지만 선언된 200px content-box 폭을 174px로 줄이지 않는다. 따라서 authored `30%` track은 60px이고 source-authored full-span item은 180px다. Chrome root frame은 230px, item offset은 13px, item 폭은 180px다. 32번의 authored `2em`은 고정 Chrome used track 관측에서 32px로 resolve된다.
- `auto-fit`은 auto-fill과 같은 반복 수를 먼저 고른 뒤 item placement 후 빈 repeated track 및 인접 gutter를 collapse한다. Item이 span으로 가로지른 repeated track은 빈 track이 아니다.
- Expanded line-name list는 repeat 내부 경계와 인접 fixed-track line names를 합친다. Numeric·named placement와 missing-name fallback은 해당 확장 결과 및 implicit track 영향을 보존한다.
- Flexible suffix는 반복 count/placement 뒤 leftover space를 분배한다. Auto-repeat의 repeat-count sizing과 최종 flex sizing은 서로 다른 기대값으로 기록한다.

위 불변식은 이 WPT 파일의 정해진 입력에 대한 계획 기준이다. 이 결과는 전체 CSS Grid, 일반 CSS admission, Spinon WPT runner 통과, 다른 browser의 결과 또는 C11.4 완료를 주장하지 않는다. Android/iOS Simulator와 surface 검증은 아직 이 사전 비교에 포함되지 않았다.

반복 횟수는 WPT index 1–35 순서로 `[6, 3, 6, 6, 6, 6, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 1, 1, 1, 5, 5, 5, 1, 1, 3, 3, 3, 1, 1, 1, 2, 1, 3, 3]`이다. Chrome CSSOM은 반복 횟수 oracle이 아닌 별도 cross-check다. 최종 used list에는 overflow placement가 만든 implicit columns가 포함된다(case 7, 8, 11, 14, 19); 이를 explicit repeat count로 세지 않는다.

`repeat-count-expected.json` SHA-256: `b21913bda7c84d3ea804533b1eb58da2eb36dac0cb1f1d4791686871ee0a2cd0`.
