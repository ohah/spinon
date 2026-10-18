# C11.4 text intrinsic measure PR 변경 적대 검토 · 30개 관점

**검토 대상:** PR #335의 최종 diff, `origin/main` `8a0e79a1`부터 head `34c1d760`까지.
**범위:** Parley intrinsic measure, Taffy measure/cache 연결, 기존 definite-width 경로의 리팩터링, 명세·상태·fixture·실행 근거, PR 통합 상태. 구현 코드 검토 20개는 별도 문서에서 다뤘으며 여기서는 PR diff의 다른 실패 경계를 확인한다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | Parley line breaker가 같은 shaped paragraph를 두 번째 폭에서 재사용하지 못함 | pinned Parley `0.12.0`의 `Layout::break_all_lines`가 새 `BreakLines`를 만들고 남은 줄을 계산한다. 구현은 한 번 만든 layout에 `0px`, `None`을 순서대로 적용한다. definite 경로는 별도 호출이며 기존 snapshot 회귀가 통과했다. |
| 2 | min-content 폭에서 일반 단어가 공백 포함 문장 폭으로 합쳐짐 | 0px 재줄바꿈 후 line별 visible advance의 최댓값을 사용한다. 고정 Ahem `XX XXXXXXX` 결과 `70px`와 기존 Chrome inline-edge oracle `75px`(edge 포함)가 각각 확인됐다. |
| 3 | max-content가 explicit line break를 지우거나 문단 폭을 잘못 합침 | `None` 재줄바꿈의 widest layout width를 사용하고 높이는 같은 layout의 전체 height를 보존한다. `pre-line`의 `XX\nXXXX` fixture에서 `40px × 20px`를 검사한다. |
| 4 | trailing/hanging whitespace가 min-content 폭에 잘못 포함됨 | Parley line metrics의 `hanging_advance`를 visible contribution에서 빼며, Parley `0.12.0` source도 layout width가 hanging whitespace를 제외한다고 명시한다. 별도 trailing-space intrinsic oracle는 이번 diff에 없어 해당 입력 계열은 잔여 한계로 남긴다. |
| 5 | `white-space:nowrap`가 0px 요청에서 임의로 줄바꿈됨 | 같은 builder가 `TextWrapMode::NoWrap`을 설정한다. Ahem min/max 모두 `100px`, inline edge Chrome 결과 `110px`로 확인했다. |
| 6 | `word-break:break-all`이 min/max 중 한 방향에서만 적용됨 | 동일 shaped style을 두 재계산에 유지한다. Ahem `XXXXXXXX`는 `10/80px`; Chrome span-edge min-content는 `15px`로 확인했다. |
| 7 | `pre-line`의 명시 개행이나 빈 중간 줄이 block height에서 사라짐 | `XX\n\nXXXX` 회귀에서 min/max height `30px`, `XX\nXXXX`에서 `20px`를 검사한다. Chrome supplemental oracle도 decorated `pre-line`의 `45px × 20px`를 확인한다. |
| 8 | inline start padding/border가 모든 줄에 반복 가산됨 | start boundary는 `text_index` 뒤 첫 content line에만 배정한다. normal·break-all·pre-line supplemental Chrome 값과 Rust Ahem edge 회귀가 일치한다. |
| 9 | inline end padding/border가 첫 줄이나 모든 줄에 가산됨 | end boundary는 source offset 앞 마지막 content line에만 배정한다. nowrap에서는 양 edge가 한 줄에 각각 한 번 포함되는 `110px` oracle를 확인했다. |
| 10 | 같은 위치의 인접·중첩 inline boundary가 누락되거나 이중 가산됨 | inline box별 고유 ID와 반환 width를 대조하고, line advance에서 실제 inline width를 제거한 뒤 각 boundary를 한 번 재배치한다. C15 source-order/depth 검증은 재사용되지만 중첩 edge 전용 intrinsic oracle는 없어 이 조합은 지원 증거로 확대하지 않는다. |
| 11 | RTL 문단에서 logical start/end edge를 visual 좌우로 혼동함 | 폭 계산은 line별 bidi visual ordering에 좌우되지 않는 단순 합산이며 edge 소유는 logical source offset을 사용한다. 이번 Chrome fixture는 LTR만 다루므로 RTL edge의 브라우저 동등성은 미검증으로 남긴다. |
| 12 | UTF-8 multibyte 경계에서 inline edge가 잘못된 줄에 붙음 | InlineBox index는 normalized UTF-8 char boundary인지 검증한다. Parley line range와 같은 normalized offset을 사용한다. intrinsic 전용 emoji/combining-mark edge fixture는 없어 해당 조합을 주장하지 않는다. |
| 13 | inline run의 font family/size/weight/style/line-height가 intrinsic 경로에서 탈락함 | style validation과 `inline_run_style_ranges` builder를 `build_parley_layout`로 옮겨 definite와 intrinsic 양쪽이 공유한다. 기존 C15 suite의 고정 font/run 회귀가 기록된 테스트 로그에 포함된다. |
| 14 | 빈 normalized paragraph가 이전 line이나 registry를 재사용함 | 빈 텍스트는 shaping 전에 네 치수 0과 현재 revision/token을 반환한다. `empty_intrinsic_paragraph_returns_zero_sizes_with_current_font_revision`이 확인한다. |
| 15 | Parley가 낸 inline box ID·폭이 준비된 boundary와 다름 | 누락 ID는 `UnexpectedInlineBox`, 폭 bit mismatch 및 잘못된 index는 `InvalidGlyphLayout`로 실패한다. 오류를 size 0으로 바꾸는 경로가 없다. |
| 16 | 잘못된 style/font 또는 NaN·무한 결과가 성공 치수로 나감 | 같은 builder에서 style/family를 검증하고, 반환 전 네 치수의 finite/nonnegative 조건을 검사한다. registry font 실패도 기존 구체 오류를 보존한다. |
| 17 | 많은 inline edge 또는 line에서 f32 합이 overflow함 | inline 폭 합산 뒤 유한성을 검사하고, min/max layout 폭·height 및 min≤max 폭도 반환 전에 검사한다. 계산 불능은 `InvalidGlyphLayout`로 닫힌다. |
| 18 | min-content 폭에서 더 많은 줄이 생겼는데 block height가 max-content보다 작아짐 | 각 mode에서 별도로 line-height를 누적하고, explicit blank-line fixture와 min/max 크기 쌍을 검사한다. 일반적인 모든 paragraph에 대해 height monotonicity를 별도 invariant로 단정하지는 않는다. |
| 19 | 숫자 font revision이 같은 독립 registry를 같은 측정 결과로 오인함 | 후속 수정 `34c1d760`에서 token identity를 함께 저장한다. clone engine은 같은 token, 독립 engine은 같은 revision 숫자여도 다른 token임을 회귀가 확인한다. |
| 20 | 같은 registry라도 successful font 등록 후 이전 intrinsic result를 현재 결과로 취급함 | 등록은 성공 시 revision과 token을 함께 교체한다. failed registration은 candidate registry에서 검증하므로 live token/revision을 바꾸지 않는 기존 font 경계를 확인했다. |
| 21 | intrinsic 결과의 equality가 paragraph나 registry identity를 무시함 | custom `PartialEq`는 paragraph, token+revision, 4개 치수를 비교한다. 서로 다른 registry 결과가 같은 numeric revision이어도 같다고 나오지 않는 test가 있다. |
| 22 | definite-width text snapshot이 intrinsic 요청으로 덮이거나 재사용됨 | `text_intrinsic_layouts`를 별도 map으로 추가했다. 새 callback test는 intrinsic 요청 뒤 definite map이 비어 있음을, definite 30px 요청 뒤 별도 key가 생김을 확인한다. |
| 23 | 한 tree에서만 유효한 intrinsic 결과가 다른 layout tree의 node 재사용으로 stale됨 | 캐시는 `CalcLayoutTree` 소유이고 `TaffyNodeId`별로 한 번 저장된다. 각 tree는 immutable paragraph와 clone된 text engine으로 새로 만들어지며 전역·cross-tree cache가 없다. |
| 24 | intrinsic layout 오류가 Taffy callback에서 성공적인 0 size로 은폐됨 | `measure_text`는 `LayoutError::TextLayout`을 반환하고 기존 leaf callback은 첫 계산 오류를 `calc_error`에 기록한다. 새 결과 cache에는 성공한 값만 삽입된다. |
| 25 | paragraph가 없거나 text engine이 없는 노드의 기존 오류 계약이 바뀜 | paragraph가 없는 node의 `Size::ZERO`, paragraph는 있으나 engine이 없는 node의 `MissingTextLayoutEngine` 순서를 유지한다. definite/min/max match 앞에서 기존 paragraph/engine guard를 사용한다. |
| 26 | `AvailableSpace::Definite`가 새 intrinsic 분기 때문에 기존 width 정규화·snapshot 생성에서 벗어남 | definite 분기는 그대로 `normalize_zero`, `layout_paragraph`, definite map 삽입을 수행한다. 전체 관련 package test 로그와 fmt/Clippy 로그가 기록돼 있고, strict workspace 성공으로 확대하지 않았다. |
| 27 | 구현 slice가 Grid track sizing 완료로 공식 상태를 과장함 | `spec/STATUS.md`의 C11.4와 C11은 미완료로 유지되고 계획은 A 단계 일부만 구현으로 표시한다. typed track sizing, spanning contribution, auto-repeat은 명시적으로 잔여 범위다. |
| 28 | 순수 Rust 측정 변경을 Android/iOS 앱 화면 동작으로 잘못 주장함 | PR 본문과 evidence README는 앱 route·Grid 화면·surface overflow를 미검증으로 구분한다. 변경된 앱 동작이 없으므로 iOS/Android Simulator 캡처를 통과 증거로 제출하지 않았다. |
| 29 | 실행·Chrome·Tailnet evidence가 수정 뒤 stale하거나 누락된 파일을 가리킴 | intrinsic, inline oracle, Tailnet evidence 각 SHA256SUMS를 검사해 모든 선언 파일이 일치했다. README가 연결한 oracle fixture와 실행 로그 파일이 PR diff에 포함됐다. |
| 30 | PR을 체크 성공으로 오인하거나 정책과 다른 병합 방식을 사용함 | PR head `34c1d760`와 원격 branch가 일치하고 PR은 OPEN/MERGEABLE/CLEAN이다. GitHub check rollup은 비었고 branch protection `required_status_checks=null`, 승인 요구 0이다. 로컬 결과만 근거로 삼고, 병합 시 저장소 규칙의 Rebase and merge를 적용한다. |

## 발견 및 남은 경계

- 이 diff 검토에서는 추가 코드 결함을 찾지 못했다. registry identity 문제는 이미 별도 구현 검토에서 발견돼 `34c1d760`으로 수정했고, 위 19–21번에서 수정된 최종 diff를 재확인했다.
- trailing-space intrinsic, nested/RTL inline edge, 일반 CJK, 실제 Grid track algorithm, Android/iOS app route와 surface overflow는 이 PR로 검증되지 않는다. 이 항목들은 C11.4 전체 완료 주장에 포함하지 않는다.
- GitHub는 이 branch에 CI check를 보고하지 않는다. 로컬 테스트·lint 결과는 evidence log에 기록된 정확한 package와 lint 범위로만 해석한다.
