# 구현/runtime 적대 검토 20개

대상: `auto_repeat_two_track_pattern_preserves_named_line_at_the_ceiling_on_both_axes` 및 입력 helper. 계획 검토와 구분해 실제 Rust layout trace·frame assertion과 현재 Grid 입력 검증 코드를 대조했다. 발견했던 outer metadata 길이 결함은 아래 5번에서 기록하고 수정 뒤 전체 검증을 다시 실행했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | exact-fit의 auto-fill 반복 횟수가 definite basis 계산과 어긋난다 | **통과.** 15,002px에서 `(15,002 - 5) / 3 = 4,999`; prefix·9,998 repeated tracks·suffix가 합계 10,000이다. |
| 2 | overflow 입력이 실제 cap 초과를 요청하지 않는다 | **통과.** 15,005px은 5,000회, 즉 prefix·10,000 repeated tracks·suffix의 10,002 source tracks를 요청한다. |
| 3 | clamp가 repeated 개수만 세어 prefix를 밀어내거나 총 explicit count를 넘긴다 | **통과.** `used_*.explicit_track_count`와 gutter 제외 최종 `Initialize` track 수를 각각 10,000으로 assert한다. |
| 4 | cap이 pattern 경계에서만 잘려 partial pattern을 숨긴다 | **통과.** overflow 기대 sequence 끝은 4,999개의 `[1px,2px]` pair 뒤의 `1px` 하나다. |
| 5 | authored repeat names가 있어도 outer metadata 길이 검증이 빠진다 | **초기 실패 후 수정.** 첫 실제 실행이 `InvalidGridTrackList`를 냈다. validator는 `tracks.len() - repeated_track_count + 2 = 4` outer group을 요구한다. helper에 해당 축의 빈 group 네 개를 두고 focused test를 재실행했다. |
| 6 | 같은 이름을 outer line과 repeat line에 중복해 occurrence를 바꾼다 | **통과.** 이름 세 group은 `GridAutoRepeat.line_names`에만 있고 outer 네 group은 빈 값이다. |
| 7 | repeat metadata의 line group 수가 pattern track 수와 맞지 않는다 | **통과.** 두 반복 track 경계에 필요한 start·middle·end 세 group을 제공한다. |
| 8 | negative occurrence `-1`이 마지막 반복 대신 첫 반복을 가리킨다 | **통과.** named start/middle을 함께 배치하고 마지막 `1px` track의 frame을 검사한다. |
| 9 | exact-fit에서 named-line item이 마지막 완전 pair와 맞지 않는다 | **통과.** target-axis offset `14,996`, size `1px`를 고정한다. |
| 10 | overflow 뒤 named-line item이 남은 partial pair에 오지 않는다 | **통과.** target-axis offset `14,999`, size `1px`를 고정한다. |
| 11 | Columns와 Rows에서 배치 입력이 서로 뒤집힌다 | **통과.** 반복 axis는 named line, 직교 axis는 numeric line 1→2로 별도 구성하고 해당 frame 좌표를 각각 검사한다. |
| 12 | 직교 축이 반복 축의 trace 선택을 오염시킨다 | **통과.** 반복 축 바깥은 고정 `1px` 한 track이고, helper는 요청한 axis의 trace만 반환한다. frame assertion은 반복 축 좌표·크기에 한정한다. |
| 13 | viewport와 root definite size가 달라 반복 basis가 불명확하다 | **통과.** helper가 두 값을 반복 축 basis로 동일하게 설정하고 orthogonal 축은 `1px`로 고정한다. |
| 14 | gap, border 또는 padding이 독립 계산에 숨어 들어온다 | **통과.** `LayoutGap::default()`와 `LayoutStyle::default()`의 zero edge를 사용한다. root helper는 해당 축의 크기 외 추가 inset을 설정하지 않는다. |
| 15 | f32 반올림이 10,000-track sequence 기대값을 불안정하게 만든다 | **통과.** basis와 모든 track/item 크기는 16,777,216보다 작은 정수 px여서 f32 정수 표현 범위 안이다. |
| 16 | 전체 track sequence 대신 끝 값만 비교해 내부 순서 오류를 놓친다 | **통과.** 10,000개 모두의 `(collapsed, base_size)`에서 base size 전체 벡터를 독립 sequence와 비교한다. |
| 17 | `auto-fill` 빈 반복 track이 실수로 collapse된다 | **통과.** 네 실행 모두 모든 track의 collapsed flag가 false인지 검사한다. |
| 18 | gutter가 track count나 expected sequence에 섞인다 | **통과.** 공통 trace helper가 `is_gutter`를 제외한 뒤 count와 sequence를 반환한다. |
| 19 | 중간 pass 또는 다른 axis의 trace를 검사한다 | **통과.** 단일 node trace를 확인하고 각 반복 axis의 마지막 pass에서 유일한 `Initialize` stage를 선택한다. |
| 20 | trace 폭증 또는 잘못된 item/frame lookup이 무시된다 | **통과.** captured stage track 합계가 100,000 미만인지 확인하고 `frames[&id(2)]`의 axis 위치·크기를 검사한다. |

**판정:** 20개 관점을 확인했다. 발견된 입력 metadata 누락은 수정했고, 이후 focused ceiling 3/3, Grid 135/135, `spinon-layout` 314/314, Clippy와 rustfmt를 다시 통과했다. 검토는 순수 Rust fixture trace에 한정하며 앱 runtime, simulator, WPT 또는 일반 author CSS 동작을 증명하지 않는다.
