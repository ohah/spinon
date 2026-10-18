# C11.4 text intrinsic measure slice 구현 적대 검토 · 20개 관점

**대상:** `TextIntrinsicInlineSizes`, Parley intrinsic measure, Taffy measure callback/cache, 해당 Rust 회귀 및 명세 변경.
**검토 기준:** 각 항목은 서로 다른 입력·소유권·line-break·오류 경계를 확인한다. 계획 PR의 20/30 관점 검토와 별개이며, 이 구현 slice의 코드 검토다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | normalized paragraph가 비었는데 Parley가 빈 line이나 이전 결과를 남김 | 빈 입력을 반환 전에 처리해 네 intrinsic 치수가 모두 0이고 paragraph/font revision을 보존한다. `empty_intrinsic_paragraph_returns_zero_sizes_with_current_font_revision`으로 검사한다. |
| 2 | 정상 줄바꿈의 min-content가 공백 포함 전체 문장 폭으로 계산됨 | zero-width 재줄바꿈에서 unbreakable word의 최대 advance를 고른다. Ahem `XX XXXXXXX`의 min/max는 `70/100px`이다. |
| 3 | `word-break:break-all`이 min-content에서 무시되거나 max-content에도 적용됨 | Ahem 8글자의 min/max는 `10/80px`; Chrome 155 span-edge 보조 oracle은 min-content `15px`, row height `80px`를 관찰했다. 해당 oracle은 max-content 값을 직접 측정하지 않았다. |
| 4 | `white-space:nowrap`가 min-content에서 임의 줄바꿈됨 | nowrap은 Ahem min/max 모두 `100px`; Chrome span-edge oracle은 min-content column `110px`, row height `10px`를 관찰했다. |
| 5 | `pre-line`의 명시 줄바꿈 또는 빈 줄이 block height에서 사라짐 | 두 줄과 빈 줄 입력을 각 줄 높이 10px로 검사한다. `XX\n\nXXXX`의 min/max height는 `30px`이다. |
| 6 | max-content가 hard line break를 지우거나 각 문단 전체 폭을 합침 | 무제한 재줄바꿈 결과의 widest line과 total line height를 반환한다. Ahem Rust 입력은 `XX\nXXXX`에서 폭 `40px`, 높이 `20px`를 검사한다. Chrome span-edge oracle은 이 입력의 min-content `45px`와 row height `20px`를 관찰했다. |
| 7 | inline start padding/border가 줄바꿈 후 잘못된 line contribution에 붙음 | `text_index` 이후 첫 content line에만 배정한다. Chrome normal/break-all/pre-line min-content는 각각 `75/15/45px`이다. |
| 8 | inline end padding/border가 첫 줄 또는 모든 줄에 더해짐 | boundary 전 마지막 content line에만 배정한다. nowrap은 양 edge가 같은 줄에서 총 한 번씩 `110px`에 포함된다. |
| 9 | Parley line advance에 포함된 inline box와 경계 폭을 중복 가산 | 각 line advance에서 positioned inline box를 뺀 뒤 각 boundary를 대상 줄에 다시 더한다. Chrome 4개 edge case의 inline 폭과 Rust Ahem 검사를 대조한다. |
| 10 | inline boundary ID·폭이 Parley 반환 값과 바뀜 | 준비 시 ID별 owner/index/edge를 저장하고 returned width bit pattern을 검사한다. 불일치는 `InvalidGlyphLayout`이다. |
| 11 | 직접 text source의 inline hierarchy가 뒤섞여 잘못된 edge를 붙임 | 기존 inline source order·depth·owner validation을 동일 builder에서 실행한다. 기존 C15.2 nested/wrap 회귀가 계속 통과한다. |
| 12 | intrinsic 전용 경로가 font/run style 검증을 우회함 | definite layout과 같은 `build_parley_layout`을 사용한다. parent/run font 부재·metric 오류는 기존 구체 `TextLayoutError`로 전달된다. |
| 13 | 독립 registry 두 개의 같은 revision 숫자가 같은 font 입력으로 오인됨 | 결과는 `FontRegistryRevision`과 private `FontRegistryToken` identity를 함께 보존한다. clone engine 결과는 같은 registry revision이고, 별도 engine의 같은 숫자 revision은 다르다는 회귀를 추가했다. |
| 14 | NaN, infinity, 음수 또는 min > max가 성공 값으로 나감 | 모든 폭·높이의 finite/nonnegative 조건과 min ≤ max를 반환 전에 검사하고 위반 시 `InvalidGlyphLayout`을 반환한다. |
| 15 | `MinContent`/`MaxContent` callback이 definite snapshot map을 오염 | callback은 별도 `TextIntrinsicInlineSizes` map에 저장한다. direct callback test에서 definite map이 비어 있음을 확인한다. |
| 16 | intrinsic callback이 definite measure의 결과를 재사용함 | 테스트가 min/max 요청 후 definite `30px` 요청을 별도 layout으로 수행하고 definite key가 따로 기록됨을 확인한다. |
| 17 | callback이 min/max width는 맞추고 block height를 0으로 버림 | callback은 min/max 각각의 line-break height를 함께 돌려준다. Ahem·inline edge·pre-line Rust/Chrome 수치로 검사한다. |
| 18 | 같은 layout tree에서 반복 intrinsic 요청이 서로 다른 값을 계산하거나 stale entry 사용 | node당 결과 하나를 `CalcLayoutTree` 수명 안에서 재사용한다. tree는 immutable `LayoutInput`과 한 번 복제한 text engine으로 구성되며 cross-tree cache는 없다. |
| 19 | intrinsic 계산 오류가 `Ok(Size::ZERO)`로 바뀌어 track가 조용히 축소됨 | callback은 `TextLayoutError`를 `LayoutError::TextLayout`으로 전파한다. width callback에서 missing engine/paragraph와 계산 실패를 zero fallback하지 않는다. |
| 20 | library slice를 Grid 지원·앱 화면·플랫폼 결과로 과장 | 현재 변경은 text measure input만 추가한다. typed Grid track sizing, item contribution integration, Android/iOS app path, surface overflow는 미구현이며 상태·계약에서 완료 주장하지 않는다. |

## 발견 후 수정·재확인

- 최초 Clippy 실행이 `build_parley_layout`의 반환 tuple을 `type_complexity`로 지적했다. tuple을 `BuiltParleyLayout`로 바꾸고 builder/intrinsic 책임을 `engine/intrinsic.rs`로 이동했다. 수정 후 Clippy를 다시 실행했다.
- line-height 검토에서 빈 `pre-line` 줄의 누락 가능성을 확인해 `XX\n\nXXXX` 회귀를 추가했다. 실제 Parley 출력에서 세 line box가 유지되고 30px height임을 확인했다.
- 전체 Grid track algorithm과 native 앱 route는 이 slice에서 검토·구현 완료를 주장하지 않는다. 상세 실행은 이 디렉터리의 `README.md`와 로그가 정의하는 범위만 증명한다.
