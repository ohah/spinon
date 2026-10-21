# C11.4 anonymous Grid text 구현 실패 관점 검토 20개

2026-10-21 최종 collector, layout/runtime adapter, renderer frame validation, Android/iOS app fixture와 다섯 scene capture를 서로 다른 실패 경계 20개로 검토했다. 검토 시 기준은 현재 checkout source와 최신 AVD/iOS Simulator 결과다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 제품 feature에서 fixture ABI가 기본 포함됨 | C11.4 runtime harness는 `c11-4-grid-track-sizing-fixture-test` opt-in 하위 모듈에 있다. 기본/positive artifact 경계 검사는 최종 검증에서 별도 실행한다. |
| 2 | 임의 DOM이 고정 scene 허용 목록에 진입 | runtime adapter는 다섯 scene ID, `div` root와 고정 Grid style만 허용하고 anonymous prefix의 미등록 scene은 거부한다. |
| 3 | direct child 수/소유권 불일치가 부분 layout을 만듦 | validator는 child 개수와 각 child의 실제 parent를 확인한 뒤 입력을 만든다. |
| 4 | text UTF-16 오류가 조용히 치환됨 | collector는 `String::from_utf16` 실패를 `InvalidTextEncoding`으로 반환한다. |
| 5 | text node 경계 또는 comment가 item 경계가 됨 | 연속 직접 text source는 같은 `sources`에 모이고 comment는 HostDocument child node가 아니어서 run을 나누지 않는다. 두 comment scene 변형과 intrinsic provenance test를 확인했다. |
| 6 | `display:none` direct child가 run을 flush함 | collector는 style 확인 후 hidden subtree를 제외하고 현재 run을 유지한다. `direct_display_none_child_does_not_split_anonymous_text_run`과 다섯 번째 app scene을 확인했다. |
| 7 | hidden descendant text가 measure/provenance에 유입 | hidden traversal은 element descendants를 frame/layout 제외 집합에 넣고 text는 collector에 push하지 않는다. layout test는 visible text source 두 개만 검사한다. |
| 8 | hidden element frame이 runtime output에서 누락 오류 | `runtime_frames`는 computed `display:none` subtree를 순회·게시하지 않는다. Android/iOS hidden scene은 root frame만 내고 5/5 통과했다. |
| 9 | hidden subtree traversal 중 style/node 누락을 성공으로 삼음 | `collect_hidden_layout_nodes`는 모든 element의 style과 snapshot node/children을 확인하고 누락 시 오류를 반환한다. |
| 10 | visible block child가 이전/다음 text run과 병합 | `LayoutDisplay::Block`에서만 flush하며 before/after source order를 보존한다. `anonymous-runs-around-block`의 각 플랫폼 frame·track·trace가 통과했다. |
| 11 | 보이는 inline/flex/grid/flow-root를 임의 변환 | collector는 명시적으로 `UnsupportedTextParagraph`로 거부한다. |
| 12 | 빈/collapsible-only run이 anchor 없이 남음 | HTML collapsible set을 고정하고 anchor가 없으면 source list를 비운다. whitespace scene은 item/cell을 만들지 않고 예상 tracks로 통과했다. |
| 13 | anchor가 앞선 공백 text node가 됨 | 첫 non-collapsible 문자 포함 text node만 anchor로 채택한다. hidden child case는 첫 visible node anchor와 source IDs를 단위 검증한다. |
| 14 | duplicate text node가 두 anonymous item으로 처리됨 | `validate_unique_source_nodes`는 각 item의 node 집합 중복을 거부한다. |
| 15 | paragraph와 provenance anchor가 서로 다름 | collector가 `ParagraphInput.node`와 item anchor를 맞추고 일치 여부를 다시 검사한다. |
| 16 | anonymous item에 DOM의 arbitrary placement가 적용됨 | runtime fixture는 네 placement end 모두 `Auto`인지 확인하며 line mutation regression도 추가했다. |
| 17 | anonymous box style이 DOM child style을 상속/누출 | item은 blockified default style에 Grid direction만 전달하고 paragraph style은 Grid owner profile을 쓴다. |
| 18 | style-to-render가 hidden subtree 생략과 frame 무결성을 혼동 | visible paint node에는 frame을 요구하고 추가 frame은 document node만 허용한다. `display_none` renderer regression 2개가 통과했다. |
| 19 | scene adapter 목록/track 기대값이 플랫폼별로 갈림 | Rust, Android, iOS, capture script가 모두 다섯 scene을 열거한다. Android AVD와 iOS 26.2 Simulator에서 각 track/frame/trace/draw-present 5/5가 통과했다. |
| 20 | 결과가 설치 APK 또는 최신 source와 다른 앱에 귀속 | Android local/install APK SHA-256은 `937d31b8869ed3d96270aaabaddea6b593124aafc8877cbb9f0fa6a0f9a8f792`로 일치하고 iOS app hash는 `4d93da63b62aa1eb701fbe78506eb4dfe6d2c459931f8c37db33ebc58152dbb4`다. 이 evidence는 simulator를 증명할 뿐 Samsung 실기기는 ADB에 나타나지 않았다. |

검토 중 기존 collector의 hidden-child flush 결함은 수정 후 7개 layout test와 두 simulator 5/5로 재검증했다. 구현은 고정 fixture subset이다. 글리프 픽셀, Android hardware GPU, iOS compositor 이후 픽셀, WPT, 실기기 Android, 일반 Grid direct text 지원은 검증 범위 밖이다.
