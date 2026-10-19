# C11.4 image ratio transfer PR #353 diff 실패 관점 검토 30개

검토 대상은 PR #353의 최종 `origin/main...feat/c11-4-image-ratio-transfer-20261019` diff와 PR 본문이다. 30개 관점은 계산 규칙, 스타일·fixture 경계, Android/iOS 실행, 문서·증거 무결성에서 서로 다른 실패 경로를 확인한다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | Chrome oracle 표·JSON·렌더 이미지의 case 수가 맞지 않는가 | `README.md` 표와 `chrome155-observation.json` 모두 27개(26 replaced image + non-replaced control)이고 PNG는 해당 fixture capture다. **통과.** |
| 2 | 비교 PNG가 예상하지 않은 natural size이거나 network request에 의존하는가 | fixture는 repository의 `c14-1-png-basic.png`를 사용하고 `naturalImageSizeCssPx`가 24×16이다. `image.decode()` 완료 전 수집하지 않으며 외부 URL을 요청하지 않는다. **통과.** |
| 3 | oracle 비교에서 track list 대신 frame만 비교하거나 허용 오차가 불명확한가 | fixture와 README가 computed track list, grid/item rect, overflow 방향을 저장하며 모든 CSS px dimension은 0.05px 허용치다. **통과.** |
| 4 | natural `aspect-ratio:auto`와 authored `3 / 2` 입력이 하나의 결과로 뭉개지는가 | 열·행 natural cases와 authored cases가 별도 ID이고 observation JSON에 computed `aspectRatio`가 각각 `auto`와 `3 / 2`로 남는다. **통과.** |
| 5 | 0-minimum 또는 min-content track control이 auto-minimum 기대값을 우연히 같게 만드는가 | `column-zero-natural-ratio`·`row-zero-natural-ratio`는 0-min control이며 min-content control도 별도 case다. Chrome과 pure Rust matrix에서 기대 track이 각각 유지된다. **통과.** |
| 6 | 열 방향 ratio 변환에서 축을 뒤집거나 나눗셈을 사용하는가 | Taffy Inline 경로는 definite block preferred size에 ratio를 곱한다. `column-auto-natural-ratio`와 authored case는 8→12를 확인한다. **통과.** |
| 7 | 행 방향 변환에서 ratio 곱셈을 사용하거나 열 결과를 복사하는가 | Taffy Block 경로는 definite inline preferred size를 ratio로 나눈다. `row-auto-natural-ratio`·authored case는 12→8을 확인한다. **통과.** |
| 8 | 반대 축 preferred size가 definite하지 않은데 transferred suggestion을 임의 생성하는가 | `transferred_size_suggestion`은 반대 축 preferred size 해석 실패 시 `None`이고 auto/auto ratio-only intrinsic input은 layout validator가 거부한다. 그 negative branch의 독립 Chrome case는 없다. **코드 경계 통과, 별도 browser negative는 미실행.** |
| 9 | specified-size suggestion이 transferred-size suggestion보다 뒤에 적용되는가 | 열 definite width 10px 및 행 definite height 6px case가 지정 크기를 유지한다. 예상값과 oracle 관측이 고정되어 있다. **통과.** |
| 10 | opposite-axis max가 ratio 변환 전에 빠지는가 | natural/authored column max-height 4px 및 row max-width 6px 결과를 Chrome·Rust에서 대조했다. **통과.** |
| 11 | opposite-axis min이 ratio 변환 전에 빠지는가 | column min-height 12px 및 row min-width 18px 결과가 각각 18×12를 만든다. **통과.** |
| 12 | 반대 축에서 `min > max`일 때 max를 먼저 적용하는가 | 열 `min-height:12px/max-height:4px`와 행 `min-width:18px/max-width:6px`를 각각 Chrome oracle·layout matrix·Simulator에서 확인했다. 최소값 우선이다. **통과.** |
| 13 | target-axis max width/height cap을 누락하는가 | column target max-width 9px와 row target max-height 6px case가 둘 다 Chrome 155, Rust 및 두 Simulator에 연결된다. **통과.** |
| 14 | target-axis min-width를 적용하며 지정한 반대 축 높이 8px를 ratio로 늘리는가 | target min-width 18px scene은 18×8을 보존한다. 세 번째 Columns pass를 Android·iOS verifier가 따로 검사한다. **통과.** |
| 15 | border-box padding/border를 transferred result에 두 번 더하는가 | `border_box_ratio_transfer_keeps_preferred_size_in_border_box_coordinates`와 Chrome·앱 scene 결과가 1px padding/border 각 면에서도 12×8이다. **통과.** |
| 16 | content-box decoration을 transfer에 더하지 않거나 잘못된 축에서 더하는가 | `content_box_ratio_transfer_adds_padding_and_border_after_conversion` 및 Chrome·앱 scene에서 12×8 content result에 양쪽 장식이 더해져 16×12가 된다. **통과.** |
| 17 | scrollable overflow의 automatic minimum을 transferred image size로 키우는가 | `column-authored-ratio-scrollable-overflow-min-zero`는 8px track과 12×8 overflow item을 유지한다. **통과.** |
| 18 | 비-replaced `div`에도 replaced image transfer가 적용되는가 | Taffy의 `is_replaced` gate, `non_replaced_items_keep_their_min_content_suggestion_instead_of_transfer`, Chrome non-replaced control을 함께 확인했다. **통과.** |
| 19 | transferred value가 Grid automatic-minimum eligibility 바깥에서 쓰이거나 fixed-track cap을 건너뛰는가 | transfer 호출은 `content_based` 분기 안이며 기존 `spanned_fixed_track_limit` clamp가 이어진다. 변경분은 기존 span 로직을 건드리지 않는다. ratio×span/fixed-limit 교차 oracle은 없다. **코드 경계 통과, 교차 matrix는 남음.** |
| 20 | 스타일 projection이 모든 Grid profile에 ratio+min/max를 허용하는가 | 허용은 `RuntimeBlockFlexGridTrackSizingV1` profile에서 direct HTML namespace `img` item ID에 한정된다. 다른 profile의 negative test가 `aspect-ratio` 거부를 확인한다. **통과.** |
| 21 | nested image나 비-image Grid item이 direct replaced image allowlist를 우회하는가 | `ratio_constraints_remain_rejected_for_non_image_grid_items` 및 `ratio_constraints_remain_rejected_for_nested_html_images`가 각각 전체 계산 거부를 검사한다. **통과.** |
| 22 | intrinsic-size snapshot의 source revision 불일치를 이번 허용 예외가 우회하는가 | `validate_intrinsic_sizes`의 source-revision 일치 검사는 예외 전에 수행되고, runtime fixture는 source/style/layout revision을 비교한다. **통과.** |
| 23 | 기존 아홉 image intrinsic scene의 스타일·ID·natural source가 신규 scene에서 덮이는가 | FFI와 runtime allowlist는 기존 9개를 보존하고 image scene 길이를 각각 16으로 확장한다. 생성된 scene ID 7개가 FFI/runtime/Java/Swift에 모두 있다. **통과.** |
| 24 | app fixture가 허용 scene 이외 입력을 받아 arbitrary HTML 실행기로 확장되는가 | runtime fixture는 각 root/item ID·tag·direct child 수·computed track/style을 case allowlist와 대조하고 FFI는 알 수 없는 scene ID를 거부한다. **통과.** |
| 25 | frame이 맞아도 stage trace 순서나 contribution 검증 없이 scene이 성공 처리되는가 | Android·iOS verifier는 Columns/Rows trace의 ID, pass 수, 각 axis와 track값을 검사한다. target-min-width는 `Columns → Rows → Columns` 세 pass를 검사한다. **통과.** |
| 26 | Android AVD 실행이 Chrome item frame이나 fixture 성공 assertion과 다른 값을 허용하는가 | API 37 ARM64 AVD의 7개 `runtime-*.log` 및 `comparison.json`에서 모든 `frame_match`가 true이고 app summary는 `status=0`, `layout=ready`다. **통과.** |
| 27 | iOS summary가 Swift verifier 성공 전에 기록되거나 Chrome frame과 어긋나는가 | iOS scene summary는 frame·trace assertion 후 기록된다. 비교 JSON의 7개 iOS item/root frame이 Chrome oracle 값과 일치한다. **통과.** |
| 28 | PR 화면 또는 overflow claim이 측정하지 않은 장면까지 pixel exact라고 확장하는가 | PR은 natural-ratio overflow 장면 screenshot 두 장만 표시한다. PixelCopy와 Metal readback은 그 overflow 장면 안/밖/background 3개 표본씩이며 나머지 여섯 장면은 frame/trace/screenshot만 검사했다고 명시한다. **통과.** |
| 29 | Android/iOS screenshot이 실기기·hardware GPU·compositor 이후의 증거로 잘못 제시되는가 | README와 PR은 AVD SwiftShader/OpenGL ES, iOS compositor 전 Metal drawable 및 실기기 미검증을 각각 표시한다. 실행 뒤 앱 PID는 종료했고 기존 iPad Simulator를 건드리지 않았다. **통과.** |
| 30 | roadmap·공식 상태·hash가 후보 범위를 완료로 과장하거나 원본과 어긋나는가 | C11.4 `[ ]`는 유지되며 roadmap/STATUS/plan/internal index가 PR #353 후보로 연결된다. Chrome 4개 파일과 simulator 186개 파일의 SHA-256 manifest가 검증된다. GitHub에는 required check가 보고되지 않았으며 branch protection의 required approval count도 0으로 조회했다. **통과; 외부 CI 체크 성공이라고 기록하지 않음.** |

영향을 받은 8·19번 검토는 코드 기준으로 통과하지만 각각 독립 negative browser case와 ratio×span/fixed-limit 교차 matrix가 남는다. PR의 local Rust, Android AVD, iOS Simulator와 checksum 근거는 [Chrome oracle README](./README.md) 및 [Simulator README](../c11-4-image-ratio-transfer-simulator-2026-10-19/README.md)에 연결돼 있다. 이 검토는 C11.4/C11 완료 판정이 아니다.
