# C11.4 고정 max track span 제한 PR 변경 검토 30개

2026-10-19에 이번 PR diff 전체를 코드·fixture·실행 산출물 관점에서 별도 검토했다. 각 항목은 파일 경계, 데이터 전달, platform verifier 또는 문서 주장의 서로 다른 실패 양상을 확인한다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | `CompactTaffyStyle`의 replaced flag가 Taffy `CoreStyle` 구현에서 보존되는가 | `item_is_replaced`를 그대로 반환한다. **통과.** |
| 2 | 기존 `is_compressible_replaced`와 새 `is_replaced` 역할이 섞이는가 | 새 override는 replaced eligibility만 반환하고 compressible 분기는 기존 계산을 유지한다. **통과.** |
| 3 | cross-axis limit 인자가 contribution query에서 빠지는가 | `track_sizing.rs`가 `self.other_axis_tracks`를 전달한다. **통과.** |
| 4 | minimum contribution과 cached query signature가 불일치하는가 | direct 및 cached 경로가 같은 반대 축 track slice를 받는다. **통과.** |
| 5 | track sizing algorithm의 columns/rows 축 순서를 임의 변경하는가 | 변경은 argument 추가에 한정되며 기존 stage 순서는 그대로다. **통과.** |
| 6 | fixed track limit 계산이 item span을 무시하는가 | 기존 `track_range_excluding_lines` 경계를 재사용한다. **통과.** |
| 7 | 내부 grid line index를 실제 track index로 잘못 합산하는가 | 해당 range 문서는 bound line을 제외하고 item 내부 track/gutter만 가리킨다고 명시한다. **코드 확인.** |
| 8 | gutter가 고정 sizing function이 아니라는 이유로 sum에서 누락되는가 | gutter는 `GridTrack`으로 표현되고 고정 length sizing function을 갖는다. 새 matrix에서 gutter 합계는 직접 검사하지 않았다. **범위 한계.** |
| 9 | 여러 spanned fixed track의 합계가 단일 track 경로와 달라지는가 | helper는 slice 전체를 합산하지만 이 PR의 두 신규 scene은 단일 track이다. **추가 runtime case 필요.** |
| 10 | 전체 span이 fixed가 아닌데 부분 fixed 합계만 clamp하는가 | 전체가 definite일 때만 `Some(limit)`을 반환한다. **코드 확인.** |
| 11 | percentage max track을 definite basis 없이 fixed px로 오인하는가 | `definite_value(axis_parent_size, ...)`를 사용하며 percentage basis를 전달한다. percentage fixture는 없다. **코드 확인, 미검증.** |
| 12 | calc resolution callback이 전이 clamp에서 누락되는가 | 같은 Taffy tree의 `resolve_calc_value`를 전달한다. calc scene은 없다. **코드 확인, 미검증.** |
| 13 | 반대 축 track limit가 target axis에 적용되는가 | `opposite_axis_tracks` slice와 `opposite_axis`가 함께 전달된다. **통과.** |
| 14 | ratio transfer의 width/height 연산이 역전되는가 | 두 방향 scene에서 `8/1.5` row와 `8*1.5` column을 trace로 검사한다. **통과.** |
| 15 | content-box/border-box coordinate 변환이 clamp로 이중 적용되는가 | 반대 축 cap에서 장식 크기를 빼고 target 축에는 기존처럼 변환한다. 새 fixed-cap content-box 교차 scene은 없다. **범위 한계.** |
| 16 | zero-floor 처리 전에 NaN/음수 track limit가 통과하는가 | 유효 length input은 양수이고 결과를 0 이상으로 바닥 처리한다. malformed non-finite input은 adapter 경계 테스트가 담당한다. **통과.** |
| 17 | new pure Rust fixture의 owner/node ID가 다른 matrix와 충돌하는가 | 신규 ID `2600`, `2601`은 같은 test document에서 사용되고 기존 test별 document 소유권과 분리된다. **통과.** |
| 18 | test가 layout success만 보고 잘못된 geometry를 놓치는가 | contribution, target track base size와 item frame 네 값을 검사한다. **통과.** |
| 19 | test가 final trace가 아닌 첫 임시 stage만 조회하는가 | target-axis pass의 마지막 stage track base size를 비교한다. **통과.** |
| 20 | 앱 fixture scene 수 상수만 늘리고 실제 scene 목록을 빠뜨리는가 | FFI 및 runtime 목록이 모두 16→18로 늘고 고유 ID 검사가 있다. **통과.** |
| 21 | CSS fixture root/item ID가 runtime allowlist와 다르게 되는가 | HTML fixture와 runtime/FFI scene에서 동일한 ID를 사용한다. **통과.** |
| 22 | image item 스타일 validator가 새 width/height/aspect ratio를 잘못 허용하는가 | 두 새 enum만 expected size·ratio 1.5와 일치할 때 통과한다. **통과.** |
| 23 | `Track::FixedEight`가 임의 fixed length도 허용하는가 | validator는 `minmax(auto, 8px)` max 값만 `FixedEight`와 일치시킨다. **통과.** |
| 24 | Android expected frame/track trace가 iOS와 다른가 | 두 verifier의 scene IDs, root/item frame, target track과 contribution 값이 같다. **통과.** |
| 25 | floating point decimal 직렬화 차이로 정상 trace가 실패하는가 | Android verifier는 contribution과 track 값에 `0.0001` tolerance를 쓴다. **통과.** |
| 26 | aspect-ratio column pass 반복을 두 번으로 잘라 trace를 놓치는가 | 새 column transfer case를 3-pass 예외에 포함하고 `Columns → Rows → Columns`를 검사한다. **통과.** |
| 27 | overflow sample이 root 내부로 잘못 배치되는가 | verifier가 root 안, grid 오른쪽 밖, grid 아래 표본 좌표를 별도로 검사한다. **통과.** |
| 28 | PixelCopy와 Metal sample ID/color mapping이 다른가 | 각 platform verifier는 대응 신규 scene과 동일 RGBA expected 값을 사용한다. 최종 matrix는 양쪽 각 3/3 exact다. **통과.** |
| 29 | simulator script가 임의 기기나 사용자 app process를 종료하는가 | 지정 AVD와 UDID만 호출하고 자신이 띄운 앱 process만 종료하도록 되어 있다. 실기기는 연결하지 않았다. **통과.** |
| 30 | 문서가 Chrome 관찰을 W3C 규칙과 혼동하거나 C11.4 완료로 표기하는가 | CRD와 Chrome 155 차이를 명시하고 multi-track/gutter·WPT 등을 남은 범위로 유지한다. STATUS C11.4 체크는 해제 상태다. **통과.** |

이 검토의 실행 결과, oracle 차이, simulator 한계와 로그 링크는 [README](./README.md)에 기록한다. 이 검토는 모든 CSS Grid 조합이나 WPT 적합성 판정이 아니다.
