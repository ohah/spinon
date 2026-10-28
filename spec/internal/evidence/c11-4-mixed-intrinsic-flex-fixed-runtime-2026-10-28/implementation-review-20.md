# C11.4 mixed intrinsic/flexible/fixed 구현·런타임 실패 관점 20개

검토 대상은 generic adapter, Grid projection, intrinsic text path, 앱 장면 matcher, 현재 소스의 Android/iOS 실행 근거다. 검토 뒤 발견한 차이는 없다. `코드`는 분기·불변식 확인이고 `실행`은 실제 test 또는 manifest 결과다. 정적 검토를 별도 부정 실행으로 포장하지 않는다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 오래된 document/style revision을 새 fixture로 처리한다 | `validate_snapshot_revision` 선행 호출이 거부한다. 인접 row adapter의 revision mismatch test와 양 앱의 revision-stamped manifest 경로도 확인했다. |
| 2 | 다른 root namespace, tag, parent를 지원 fixture로 오인한다 | HTML namespace, `div`, `HostParent::Root`를 강제하고 다르면 오류를 반환한다. |
| 3 | 알 수 없는 scene ID가 이 renderer fixture로 라우팅된다 | FFI는 정확한 8 ID만 생성한다. Android/iOS에서 `column-freeze-unlisted`가 거부됐다. |
| 4 | Columns/Rows가 반대로 선택되거나 root 축 크기가 흐려진다 | 두 computed track list의 방향과 root `300×120`/`120×300` dimensions를 함께 확인한다. 실제 8개씩 결과가 matcher 표와 일치한다. |
| 5 | flex factor, min/max breadth 또는 fixed neighbor가 바뀌어도 허용한다 | mixed track vector는 `min-content/1fr`, fixed `40px`, `0px/2fr`의 구조적 동등성으로 검사한다. |
| 6 | gap이 두 축에서 섞이거나 root 크기에 포함된 padding이 무시된다 | 축별 정확한 10px/0px gap과 0 edge를 강제한다. 두 축의 최종 base 합과 gutter 합이 300px다. |
| 7 | auto repeat, named line, template area 또는 auto-flow가 추가된다 | repeat·areas·line names·auto rows/columns·flow를 각각 제한하고, 불일치는 fail closed다. |
| 8 | root box sizing, alignment, writing mode 또는 direction 차이가 결과에 섞인다 | 고정된 `border-box`, `start/normal`, `ltr`, `horizontal-tb` CSS property와 typed style을 함께 검증한다. |
| 9 | item이 직접 자식이 아니거나 HTML 이외 요소여도 처리된다 | namespace와 parent를 각 child에서 검사한다. direct child 수는 정확히 세 개여야 한다. |
| 10 | 누락·추가·중복 track 위치 또는 지원하지 않는 source 순서를 받아들인다 | source track sequence는 `[1,2,3]` 또는 `[3,2,1]`만 허용한다. |
| 11 | item이 명시 track 하나를 넘거나 cross-axis cell 밖으로 놓인다 | start/end line과 반대 축의 1/2 line을 모두 검사한다. 잘못된 placement runtime test가 거부된다. |
| 12 | text `div` 안의 nested element 또는 여러 text node를 합쳐 사용한다 | direct text child 하나만 받는다. child tag 및 HostParent 관계도 확인한다. |
| 13 | freeze/no-freeze/equality 입력 길이가 서로 바뀌어도 기대값을 유지한다 | A text는 10/4/8개의 `X`, C text는 정확히 4개의 `X`여야 한다. 길이 7 negative test가 거부됐다. |
| 14 | 이미지가 중간 fixed track 밖에 놓이거나 복수 이미지가 들어온다 | image 단 하나를 track 2로 제한하고 나머지 tag 조합은 거부한다. |
| 15 | 잘못된 URL, child가 있는 image, decode 실패, natural size 변조를 정상 처리한다 | exact URL, leaf topology, PNG decode, `24×16` natural metadata 확인을 거친다. URL/decode 변조 단위 control은 이번 신규 test에서 개별 실행하지 않았다. |
| 16 | Ahem이 아닌 font나 text-style 변화가 intrinsic 결과에 반영되지 않는다 | font/size/weight/style/line-height/white-space/break/direction 등 고정 property와 paragraph writing mode를 확인한다. |
| 17 | vertical nowrap text가 soft break를 허용해 min-content가 작아진다 | 현재 제한 ASCII/Ahem vertical profile에서 `NoWrap`을 받아들이고 `A BC`의 min/max inline size가 모두 40px인 focused test가 통과했다. |
| 18 | 여러 orthogonal paragraph가 projection에서 차단되거나 일반 프로파일까지 무제한 완화된다 | C11.4 validator의 단일 노드 개수 제한만 제거했다. paragraph set/style/profile 검증은 유지한다. Rows scene의 두 text item이 모두 layout된다. |
| 19 | freeze, no-freeze, strict equality 또는 source reversal에서 최종 track base가 틀린다 | Rust runtime test와 양 앱 matcher가 8 scene의 trace/frame을 확인했고 simulator verifier가 전부 통과했다. |
| 20 | capture가 제출 코드와 다른 source/artifact를 기록하거나 화면 대신 준비된 report만 통과한다 | 재캡처 manifest의 source hash·앱 hash·trace·frame·PNG hash를 verifier가 확인했다. screenshots를 직접 열어 Android/iOS row-freeze presentation을 확인했다. 이는 실기기 및 픽셀 동일성 검증은 아니다. |

남은 테스트 경계: 이미지 source/decode/natural-size 변조 각각의 신규 adapter negative test, arbitrary DOM/style 입력, 물리 기기는 이 slice에서 실행하지 않았다. 지원은 해당 고정 profile에 한정한다.
