# C11.4 direct-text aspect-ratio min/max 구현·runtime 적대 검토 20개

**대상:** feat/c11-4-direct-text-aspect-ratio-min-max의 구현 source와 최종 simulator evidence
**기준:** CSS Grid §6.6 및 CSS Sizing §4.2·§4.4, Chrome 156 고정 oracle, 현재 변경된 Rust·Android·iOS 경로
**판정:** 20개 관점에서 새 blocker는 발견하지 않았다. 아래에서 기록한 구현 중 발견사항은 최종 확인 전에 수정했고, 각 수정 뒤 영향을 받은 test 또는 simulator 행렬을 다시 실행했다. PR diff 검토 30개는 별도이며 아직 수행하지 않았다.

| # | 실패 관점 | 검토와 판정 |
|---:|---|---|
| 1 | Replaced와 non-replaced Grid sizing 제안 혼용 | CSS Grid의 non-replaced content size suggestion은 relevant-axis min-content에서 출발하고 반대 축의 definite min/max를 ratio로 반영한다. Replaced 전용 transferred size suggestion과 나누어 보았다. 새 helper는 Grid의 min/max 제약 이전을 계산하며, final alignment의 ratio clamp는 replaced 항목에만 남겨 경로를 섞지 않는다. |
| 2 | 가로·세로 비율 변환 방향 뒤바뀜 | 2:1 fixture에서 min-width:60px은 높이 30px, min-height:30px은 폭 60px이어야 한다. row·column fixture와 Chrome 156 기대값 및 양 플랫폼 frame/track trace가 방향별로 일치한다. |
| 3 | 이전된 최소값이 반대 축의 직접 최대값을 침범 | min-width:60px; max-height:20px의 변환 높이는 30px이나 destination max가 20px으로 제한해야 한다. Oracle와 19개 min/max scene에서 높이 20px을 확인했다. |
| 4 | 이전된 최대값이 반대 축의 직접 최소값 아래로 내려감 | max-width:30px; min-height:20px에서 변환 최대 높이 15px은 직접 최소 높이 20px 아래로 적용될 수 없다. Oracle·runtime은 20px을 유지한다. |
| 5 | 같은 축에서 min > max일 때 max가 min을 우선 | source-axis 충돌 min-width:60px; max-width:40px 및 높이 대응 장면을 확인했다. CSS의 최소 크기 우선 규칙대로 min이 승리하고 Chrome 기대 geometry와 일치한다. |
| 6 | 기본 min-width:auto의 min-content floor를 잃음 | max-height:15px; min-height:0만 지정하면 AAAA의 min-content 폭 40px을 지켜야 한다. min-width:0을 추가한 별도 control만 transferred max 30px으로 내려간다. 두 입력은 별도 oracle/runtime scene으로 유지된다. |
| 7 | min-content와 max-content 기여 단계가 서로 다른 제약 적용 | Grid item의 두 intrinsic contribution call site에서 동일한 ratio/minmax projection을 적용하는지 보았다. 19개 scene의 각 축 contribution stage·used track·frame이 고정 기대값과 통과한 simulator matrix에 기록된다. |
| 8 | 정확한 ratio owner가 아닌 item에 min/max 게이트 적용 | projection 진입점은 지정된 NodeId와 유일한 typed ratio/minmax owner를 대조한다. 다른 direct item NodeId를 넘기는 runtime test가 unsupported_c114_text_style 계열 오류로 거부되는 것을 확인했다. |
| 9 | 두 개 이상 ratio owner 또는 별도 min/max owner를 허용 | Gate는 snapshot 안의 ratio owner가 정확히 하나이고, 제약이 있는 owner도 정확히 그 NodeId인지 확인한다. 복수 owner는 구현 입력으로 허용되지 않으며 fixture collector의 고정 tree shape 검증과 함께 fail closed한다. |
| 10 | computed CSS serialization과 typed ratio 값의 불일치 | "2 / 1"은 finite typed 2.0 및 BoxSizing basis와, "auto 2 / 1"은 finite typed 2.0 및 ContentBox basis와 함께 있어야 한다. serialization만 바꾸거나 typed basis를 바꾼 경우 gate가 열리지 않는다. |
| 11 | 숫자 ratio와 auto <ratio>의 box basis 혼동 | numeric 2:1은 CSS box-sizing이 고른 box를 사용하고 auto 2 / 1은 content-box 비율을 사용한다. border-box edge control과 content-box edge control에서 각각 30px과 31px row 등 독립 기대값을 확인했다. |
| 12 | padding/border를 content-box constraint 환산에서 누락 | 고정 padding 2px 4px, 1px border의 edge 합은 가로 10px·세로 6px이다. auto content-box ratio constraint를 border-box API의 길이 단위로 환산하고 edge를 복구하는 경로를 projection과 Chrome/fixture geometry에서 대조했다. |
| 13 | edge가 큰 경우 음수 content constraint가 layout에 유입 | Border-box min/max를 content-box로 바꿀 때 (value - edge_sum).max(0)을 적용한다. CSS 음수 constraint는 고정 fixture 입력으로 통과하지 않으며 0 미만 내부 값이 Taffy에 전달되지 않는다. |
| 14 | 구현이 일부 정수 oracle 값에만 고정 | 초기 표에 없는 min-width:45px와 max-height:17.75px를 Chrome 156에서 추가 관측했다. 연속 finite 0..=60px 허용 계약과 일치해 45 및 17.75가 통과하고, 각각 45×22.5px와 35.5×17.75px을 반환한다. 처음의 이산 값 전용 gate는 이 검토에서 발견해 연속 범위로 고쳤다. |
| 15 | percentage·calc·경계 밖·비유한 값을 고정 px 입력처럼 수용 | %와 60px 초과 입력의 negative tests는 거부한다. Gate는 fixed px의 유한 0..=60 값만 허용하며 calc(), 음수와 60px 초과는 범위 밖으로 둔다. |
| 16 | ratio 없는 min/max가 반대 축으로 전달 | no-ratio-min-width-60과 no-ratio-min-height-30은 독립 축 control이다. width 제약은 ratio가 없을 때 height로 변환되지 않고, row 10px/30px 기대값 및 두 플랫폼 matrix와 일치한다. |
| 17 | opt-in fixture 외의 CSS profile 또는 제품 ABI로 게이트 누출 | CSS admission은 RuntimeBlockFlexGridTrackSizingV1 profile, exact owner와 전용 fixture entrypoint에 묶여 있다. Taffy의 Grid sizing 알고리즘 수정 자체는 일반 알고리즘에 포함되며 default archive에도 컴파일되지만, default c04-runtime-gpu archive에서 신규 fixture scene marker 0/19와 fixture ABI symbol 부재, fixture positive control의 marker 19/19 및 symbol 존재를 확인했다. archive 결과를 APK/IPA 전체 경계로 확대하지 않는다. |
| 18 | 공통 Taffy 수정이 기존 replaced/nested image contribution을 깨뜨림 | 변경된 Grid known_dimensions와 alignment를 함께 검토했다. cargo test -p spinon-layout chrome155_image_transferred_size_suggestions_match_in_both_grid_axes --lib의 replaced-image 양축 min/max clamp 사례가 1/1 통과했다. 별도 nested text/replaced intrinsic regression도 fixture feature에서 1/1 통과했다. 두 검사는 전체 replaced CSS 적합성을 뜻하지 않는다. |
| 19 | Android/iOS validator가 서로 다른 scene 또는 trace를 승인 | 두 host validator는 동일한 기존 23개 direct-text ratio scene과 새 min/max 19개를 각각 확인한다. 최종 API 37 ARM64 QEMU AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 양쪽 모두 42/42 scene, 129/129 표면 sample이 일치했다. Android PixelCopy 좌표는 정수 CSS 좌표로 고정했다. |
| 20 | 표면·실행 증거를 OS 최종 화면 또는 전체 제품 지원으로 과장 | Android는 QEMU/software renderer AVD이며 iOS는 compositor 제출 전 Metal drawable readback이다. 표본은 지정된 fill/root 배경만 검사하고 glyph raster는 검사하지 않는다. 실행 receipt의 local/installed APK·executable hash는 각각 일치한다. 실기기, hardware GPU, WPT 및 전체 C11.4/C11은 이 증거의 결론이 아니다. |

## 구현 중 찾아 수정한 사항

1. 첫 구현 gate가 사전 표의 정수값만 허용해 continuous px 계약과 충돌했다. Chrome 45px 및 17.75px 추가 oracle을 관측하고 finite 0..=60px으로 고친 뒤 최종 42-scene 행렬을 다시 실행했다.
2. Android PixelCopy가 fractional CSS sample 좌표를 허용해 native pixel lookup 좌표가 불명확했다. sample 좌표를 floor해 두 플랫폼 최종 행렬을 다시 확인했다.
3. content-box ratio owner gate를 helper signature 단순화 중 놓쳤다. focused runtime test가 실패해 min/max owner에서 typed content-box basis를 다시 도출하도록 고쳤고, 이후 최종 build와 simulator matrix를 실행했다.
4. Clippy의 8-argument helper 경고를 boolean flags 대신 DirectTextParagraphProfile enum으로 정리했다. 최종 runtime feature Clippy 및 matrix는 이 변경 뒤 통과했다.
5. Apple LLVM 17 llvm-nm은 Rust 1.96 LLVM 22 object의 attribute를 읽지 못했다. Rust toolchain의 LLVM 22 llvm-nm으로 바꿔 동일 archive 검사를 다시 완료했다. 이는 scanner 도구 호환 문제였다.

## 연결된 최종 근거

- [최종 Android/iOS 실행·화면·해시와 범위](./README.md#2026-10-30-direct-text-aspect-ratio-minmax-구현-후보)
- [Chrome 사전 비교와 fractional px oracle](../c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.md)
- [CSS Sizing Level 4 §4.4 Min/Max Size Transfers](https://drafts.csswg.org/css-sizing-4/)
- [CSS Grid §6.6 Automatic Minimum Size](https://drafts.csswg.org/css-grid/#min-size-auto)

이 검토의 기준 checkout HEAD는 d3fdd5660614c7f877908e95c2bffca39bd4496a이고, 최종 Android/iOS receipt에는 그 당시 dirty source의 파일별 SHA-256이 보존되어 있다. PR이 열리면 별도의 30개 diff 실패 관점 검토를 새로 수행한다.
