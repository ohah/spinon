# C11.4 고정 max track 반대 축 전이 구현 검토 20개

검토 대상은 replaced item 판정, 반대 축 fixed max track 제한, Grid contribution 경로와 direct-image fixture다. 2026-10-19에 Rust 구현·단위 회귀·Android/iOS simulator trace 및 PixelCopy/Metal 표본을 대조했다. 기존 PR #353의 검토 표를 재사용하지 않고 이번 span 제한의 별도 실패 경계를 확인했다.

| # | 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | compact layout style이 Taffy `CoreStyle::is_replaced()`에서 direct image 상태를 잃는가 | `CompactTaffyStyle`이 `item_is_replaced`를 반환하고, 두 방향 ratio fixture에서 transferred suggestion이 기록된다. **통과.** |
| 2 | column minimum contribution이 반대 축이 아닌 columns를 조회하는가 | `AbstractAxis::Inline`의 반대 축을 `Block`으로 선택하며 8px row clamp case가 `12px` column contribution을 낸다. **통과.** |
| 3 | row minimum contribution이 ratio를 곱해야 할 때 나누거나 그 반대인가 | `AbstractAxis::Block`은 width를 ratio로 나누며 8px column clamp에서 `5.333333px` row contribution을 낸다. **통과.** |
| 4 | definite preferred size가 없는 반대 축에서 transfer가 허위 생성되는가 | helper는 preferred size 해석 실패 시 `None`을 반환한다. 새 fixture는 definite size만 다루며 이 회귀는 기존 oracle에 둔다. **코드 경계 확인, 새 oracle 없음.** |
| 5 | aspect ratio가 없는 replaced item에도 ratio transfer를 만드는가 | `self.aspect_ratio?`에서 중단하고 기존 natural/authored ratio fixture를 유지한다. **통과.** |
| 6 | 반대 축 min/max가 고정 track limit보다 나중에 적용되는가 | 순서는 preferred → authored min/max clamp → fixed track clamp → ratio다. 기존 min-over-max oracle는 max보다 min을 적용하는 동작을 고정한다. **통과.** |
| 7 | target axis의 definite preferred size가 transferred suggestion보다 우선하지 않는가 | 최종 suggestion은 target preferred size로 제한되고 기존 `column-target-min-width-clamp` scene은 frame·세 pass를 검사한다. **통과.** |
| 8 | target axis의 max size가 전이 결과에 적용되지 않는가 | 최종 target clamp에서 max size를 적용하며 기존 row/column max clamp case를 유지한다. **통과.** |
| 9 | content-based automatic minimum 조건 밖에서도 새 helper가 호출되는가 | helper 호출은 `content_based` 분기 안에 있다. scrollable overflow와 non-replaced control이 기존 행렬에 남아 있다. **통과.** |
| 10 | target 축 자체의 fixed track maximum 제한이 사라지는가 | 기존 `spanned_fixed_track_limit(axis, axis_tracks, ...)` 제한이 transferred value 뒤에도 유지된다. **통과.** |
| 11 | 반대 축 span 안에 비-fixed max가 하나 있어도 고정 합계를 반환하는가 | `spanned_fixed_track_limit`은 모든 spanned max sizing function이 definite일 때만 합계를 반환한다. 혼합 max sizing runtime case는 아직 없다. **코드 경계 통과, 조합 oracle 남음.** |
| 12 | item 밖의 fixed track이 반대 축 제한에 영향을 주는가 | helper는 `track_range_excluding_lines(opposite_axis)`만 더한다. unspanned track 경계의 새 image case는 없다. **코드 경계 확인, 추가 oracle 남음.** |
| 13 | span 내부 gutter가 fixed track 합계에서 빠지는가 | Taffy는 gutter를 같은 track slice에 담고 sizing function을 합산한다. 새 span-limit fixture는 한 track·gap 0만 사용해 gutter 합계는 직접 검증하지 않았다. **범위 한계.** |
| 14 | content-box item의 반대 축 padding/border를 track border-box limit에서 빼지 않는가 | 구현은 `box_sizing_adjustment`를 빼고 0 아래를 막는다. 기존 content-/border-box transfer test는 있지만 새 fixed-span clamp와 content-box를 결합하지 않았다. **코드 경계 확인, 교차 oracle 남음.** |
| 15 | 장식 크기가 limit보다 커서 음수 content-box cap을 만드는가 | `f32_max(limit - adjustment, 0.0)`가 0으로 바닥 처리한다. 이 값만을 단독 검증하는 scene은 없다. **코드 경계 확인, 직접 oracle 남음.** |
| 16 | row/column이 아닌 writing mode에서 `Size`와 `AbstractAxis`가 뒤섞이는가 | helper는 논리 축 API를 사용하지만 신규 fixture는 기본 horizontal writing mode다. **미검증.** |
| 17 | percentage 또는 calc fixed max track이 반대 축 definite basis로 resolve되는가 | 부모 inner size basis와 `resolve_calc_value`를 전달한다. 새 scene은 8px absolute length만 사용한다. **코드 경계 확인, 입력 matrix 남음.** |
| 18 | 반복 sizing pass의 minimum-contribution cache가 새 반대 축 입력을 stale하게 유지하는가 | 기존 Grid 재실행은 axis별 cache를 지우며 새 column-transfer case에서 `Columns → Rows → Columns` trace가 예상 contribution을 유지한다. percentage basis 변경은 미검증이다. **고정 길이 경로 통과.** |
| 19 | target track 크기만 같고 item frame은 틀린 결과가 통과하는가 | Rust test는 minimum contribution, final target track과 item width/height를 모두 검사한다. 앱 verifier는 root/item frame과 trace를 별도 비교한다. **통과.** |
| 20 | 앱 결과가 grid 밖의 색 표본 없이 통과하는가 | 양 simulator에서 새 두 장면의 root 바깥 오른쪽·아래 표본을 PixelCopy/Metal로 각각 3/3 검사했다. 표본은 CSS background box이며 이미지 texel paint가 아니다. **통과, paint 범위 한정.** |

최종 실행과 고정 입력은 [실행 근거](./README.md)에 연결한다. multi-track/gutter, writing mode, percentage/calc, cross-cap content-box, WPT, 실기기와 전체 C11.4는 이번 검토로 닫히지 않았다.
