# C11.4 양축 span text 계획 PR 변경분 적대 검토

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 PR을 구현 완료나 CSS 지원으로 읽는가? | 상태가 runtime 미구현이며 C11.4 미완료와 `77/213`을 유지한다고 명시한다. |
| 2 | 새 범위가 이전 column/row span 검증의 중복인가? | 새 invariant는 동일 item의 Columns 결정과 wrapped block contribution 의존성이다. |
| 3 | 규범 근거가 기준 구현의 수치를 복사했는가? | CSS Grid 2025 CRD §7.2.1, §11.1, §11.5.1을 단계별로 계산에 적용하고 Chrome 관측은 별도 JSON이다. |
| 4 | 2×2 explicit grid가 실제 양축 span을 강제하는가? | 입력과 plan이 `grid-column:1/3`, `grid-row:1/3` 및 두 tracks per axis를 고정한다. |
| 5 | min-content width 10px의 계산에서 gap을 누락하는가? | 한 2px gutter를 빼서 각 column `[4,4]`, content width 10px을 재구성한다. |
| 6 | max-content width 60px의 계산이 font fallback에 기대는가? | Ahem font SHA와 6×10px advance를 명시하고 `document.fonts.check`를 관측했다. |
| 7 | block contribution이 columns 결과와 무관하게 고정되는가? | 10px width→6 lines/60px, 60px width→1 line/10px을 독립 acceptance로 쓴다. |
| 8 | Rows 기대값에서 row gap을 잘못 분배하는가? | 60−2와 10−2를 각각 둘로 나눠 `[29,29]`와 `[4,4]`를 계산한다. |
| 9 | cyclic percentage를 intrinsic size 계산에 넣어 순환하는가? | intrinsic pass에서 auto처럼 두고 resulting content basis로 3px/8px minimum을 resolve한다. |
| 10 | definite control의 width를 border-box로 오해하는가? | 10px/60px은 content-box이며 border-box는 각 방향 4px 추가로 따로 기록한다. |
| 11 | `min-width:0` counterexample를 최종 기대값으로 혼합하는가? | 거부 표식·별도 JSON·screenshot으로 분리하고 README에서 acceptance에서 제외한다. |
| 12 | line Range rect를 glyph paint의 증거로 부풀리는가? | Chrome 결과와 plan이 Range geometry와 실제 simulator glyph paint를 구분한다. |
| 13 | definite controls가 최소/최대 endpoint를 검증하는가? | 10px·60px definite scene은 각각 min-content·max-content scene의 tracks와 lines를 재현한다. |
| 14 | runtime adapter가 arbitrary DOM이나 style을 받는가? | 계획은 exact four IDs, one item, exact typed styles와 fail-closed rejection을 요구한다. |
| 15 | Simulator 결과가 전체 Grid/웹 호환성을 뜻하는가? | 제한 fixture, AVD·Simulator 범위와 WPT/CSS admission 미포함을 문서화한다. |
| 16 | surface sample이 border stroke를 검증했다고 주장하는가? | pixel gate는 item fill sample로 제한하고 border stroke를 명시적으로 제외한다. |
| 17 | Android 결과에 physical device 또는 hardware GPU를 섞는가? | 계획은 API 37 ARM64 AVD만 대상으로 하고 실기기 연결을 제외한다. |
| 18 | iOS readback이 compositor 이후 pixel로 오인되는가? | drawable sample을 compositor 제출 전으로 제한한다. |
| 19 | plan, plan-review, implementation review 관점이 서로 재사용되는가? | 계획 자체 검토와 향후 runtime/PR diff 검토를 별도 산출물로 요구한다. |
| 20 | 계획 등록이 공식 집계나 직전 PR 기록을 왜곡하는가? | STATUS에는 C11.4 plan link만 추가한다. Parent plan은 PR #503의 실제 simulator 결과와 merge SHA를 기록하며 checkbox/API/`77/213`은 바꾸지 않는다. |

## 판정

계획 PR 변경분은 비교 모델, 고정 input, 독립 수치, 반례, scope boundary와 별도 구현 관문을 연결한다. 시뮬레이터 결과와 전체 C11.4 완료를 선취하지 않는다.
