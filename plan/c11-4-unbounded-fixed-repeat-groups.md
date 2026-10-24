# C11.4 · 두 개 초과 fixed-repeat 그룹과 auto-repeat

**상태:** 계획-only. Chrome 156 사전 관찰과 계획 실패 검토 20개를 고정했다. Spinon 구현과 runtime 검증은 아직 시작하지 않았다. C11.4 공식 checkbox와 `72/213` 집계는 유지한다.

## 현재 동작과 결손

병합된 [PR #594](https://github.com/ohah/spinon/pull/594)는 한 track list 안에서 하나의 auto-fill/auto-fit과 최대 두 fixed-repeat 그룹을 제한 수용한다. `computed_auto_repeat_tracks`의 fixed-repeat 저장 공간이 두 칸이라 세 번째 그룹은 unsupported-value 오류로 끝난다.

CSS Grid 2025 CRD §7.2.3.1의 auto-track-list 문법은 하나의 auto-repeat과 그 앞뒤의 fixed-repeat 그룹들을 허용한다. 이번 계획은 임의의 그룹 개수 상한을 코드에 두지 않고, 기존 `MAX_COMPUTED_GRID_TRACKS` 10,000개 expanded descriptor와 checked arithmetic·fallible allocation으로 입력을 제한한다. 따라서 CSS 문법 전체 지원을 뜻하지 않는다.

## 목표와 제외

- 한 축의 한 track list에서 positive integer count fixed-repeat 그룹을 2개 초과하여 허용하고 auto-fill 또는 auto-fit 하나와 authored order대로 결합한다. columns와 rows를 별도 장면에서 확인한다.
- 각 fixed-repeat pattern은 1~2개의 computed finite nonnegative `LengthPx` track으로 제한한다. 패턴·count를 확장한 authored descriptor 합은 10,000 이하여야 한다.
- dynamic group metadata는 track-list component index로 바로 조회할 수 있어야 한다. group 수 증가로 반복 검색이 제곱 시간으로 바뀌지 않게 하고, reservation 실패는 node/property를 포함한 allocation error로 변환한다.
- 확장 전에 count 변환, `count × pattern length`, 누적 track 수, total authored descriptor 수, line metadata bytes를 checked arithmetic으로 검증한다. 지원 밖 입력은 부분 결과 없이 거부한다.
- fixed/outer/auto line names, 두 번째 auto-repeat, percentage 또는 unresolved `Calc`, flex/intrinsic/minmax fixed breadth, template areas, nested/other repeat shape, 일반 CSS admission, WPT, Android hardware GPU, iOS compositor 이후 pixel, 전체 C11.4 판정은 범위 밖이다.
- 내부 계약 0059의 `0.1.0` 및 공개 완료 수 `72/213`은 변경하지 않는다.

## 규범·독립 기대값·Chrome 사전 비교

- 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)이다. 이 규범은 auto-repeat이 track list에 한 번만 나오며 같은 list가 fixed-repeat 그룹도 포함할 수 있음을 명시한다.
- [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-unbounded-fixed-repeat-groups-precomparison-2026-10-24/README.md)는 네 fixed-repeat 그룹을 사용하는 column prefix와 row split 장면을 보존한다. Chrome은 grammar/used-value 교차 관찰이고 독립 산술이 기대값의 기준이다.
- 양 장면의 fixed-repeat breadth 합은 94px, 보통 neighbor는 5px, auto-repeat 최소 track은 20px, gap은 8px다. fixed/ordinary track은 9개이므로 최소 필요 공간은 `99 + 20n + 8(8+n) = 163 + 28n`이다. 320px 축에서 `n=5`는 303px, `n=6`은 331px라 최대 5회다. 남은 17px는 5개 `1fr`에 분배돼 독립값 23.4px가 되고 Chrome 156 CSSOM은 23.3906px로 직렬화한다. 비교 허용 오차는 1/64 CSS px다.
- row split 장면의 item은 `grid-row: 5 / 6`에 있고, 그 시작점은 `5 + 11 + 13 + 13 + 4×8 = 74px`다. Chrome 관측값은 root `80×320px`, item `(0,74,10,10)`이다.
- 원본 fixture, observation JSON, 독립 산술 JSON, screenshot은 같은 evidence 디렉터리에 있으며 SHA-256으로 고정한다.

## 구현 순서와 통과 기준

1. 이 계획과 별도 20관점 계획 검토를 먼저 완료한다. Chrome observation과 independent expectation JSON이 서로 일치하는지 대조한다.
2. 두 칸 fixed-repeat 저장소를 fallible, index-addressed dynamic metadata로 바꾼다. valid component 수는 각 그룹이 최소 한 descriptor를 만들고 전체 합이 10,000 이하여야 하는 제한에 맞춘다.
3. cascade tests에서 기존 한·두 그룹 입력, 3·4·8개 그룹 authored order, auto-repeat 앞/사이/뒤 배치, rows axis, 10,000 통과/10,001 거부, allocation failure와 다음 호출 복구를 검사한다. 두 번째 auto-repeat, named metadata, unsupported breadth와 malformed count는 계속 거부한다.
4. 기존 10개 C11.4 fixed-repeat runtime scene에 네 그룹 prefix/split 신규 scene을 추가해 총 12개를 만든다. AVD와 iOS Simulator에서 track/frame/trace/draw/present를 확인하고 PixelCopy 및 compositor 전 Metal 표본을 별도로 보고한다.
5. fixture module/artifact 경계를 확인한다. default product artifact와 opt-in positive control을 분리해 비교하며 static archive 결과를 APK/IPA exclusion으로 확대하지 않는다.
6. 코드·런타임 변경 후 새 실패 관점 20개를 기록하고, PR diff에는 별도의 20개 적대적 검증을 남긴다. 모든 관련 checks와 실제 merge 결과를 확인한 뒤 0059, STATUS, 공개 roadmap을 갱신한다. C11.4 전체 관문이 남아 있으므로 공식 checkbox와 `72/213`을 유지한다.

## 남은 범위

세 번째 fixed-repeat을 포함한 세션의 현재 구현, named-line merge/collision, fixed-size `minmax()`와 percentage breadth, 일반 CSS admission, WPT, 실기기/hardware GPU, iOS compositor 이후 pixels 및 C11.4 intrinsic track sizing 전체는 미완료다. 이 계획-only slice 자체는 제품 기능이나 공식 완료 항목이 아니다.

계획 실패 관점은 [20개 검토](./c11-4-unbounded-fixed-repeat-groups-plan-review-20.md), 변경 파일의 별도 검토는 [계획 PR diff 검토](./c11-4-unbounded-fixed-repeat-groups-pr-diff-review-20.md)에 기록한다.
