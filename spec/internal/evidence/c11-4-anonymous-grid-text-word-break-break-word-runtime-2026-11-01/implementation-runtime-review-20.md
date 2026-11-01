# C11.4 익명 Grid 텍스트 `word-break: break-word` 구현/runtime 실패 관점 20개

대상은 Stylo parser 호환 경계와 computed-style cascade source 복원, text-layout의 effective overflow-wrap, opt-in 18-scene runtime/FFI 경로, Android·iOS fixture 검증기 및 실제 simulator trace다. 계획 리뷰와 계획 PR 검토에 재사용하지 않고 구현과 실행 결과를 따로 확인했다. cascade 검토에서 `revert`와 `revert-layer`를 부모 상속으로 뭉뚱그리면 lower-origin/lower-layer 값이 사라지는 문제를 찾아 수정했고, 영향을 받는 style tests와 양 플랫폼 행렬을 다시 실행했다.

| # | 독립 실패 관점 | 확인한 경로와 판정 |
| ---: | --- | --- |
| 1 | `break-word`와 비슷한 문자열·custom property·다른 속성이 잘못 변경됨 | CSS 토큰 parser는 `word-break`의 단일 literal identifier만 rewrite한다. `content`, custom property, `var()`와 다른 `word-break` 값은 보존하며 정규화 단위 검사 3개가 통과했다. **통과** |
| 2 | 대소문자·`!important`·토큰 간격이 parser 변환에서 깨짐 | case-insensitive keyword와 `!important`를 처리하고 뒤 선언 위치가 움직이지 않는지 같은 길이 assertion으로 확인했다. **통과** |
| 3 | UTF-8 원문 offset과 Stylo의 선언 source location이 어긋남 | rewrite는 토큰 byte range 내부만 공백과 `normal`로 바꾸고 원문 길이를 유지한다. raw stylesheet map은 동일 offset을 가진 원본 CSS에서 선언 블록을 회수한다. inline offset·후속 선언 단위 검사와 simulator runtime trace가 통과했다. **통과** |
| 4 | 문자열·주석·중첩 rule 안의 가짜 선언을 실제 CSS로 해석 | `cssparser` 기반 rule/declaration parser가 quoted string과 component value를 건너뛰고 nested media rule의 선언만 찾는다. 중첩 규칙 단위 검사가 통과했다. **통과** |
| 5 | 비정상적으로 깊은 중첩에서 일부 선언만 바뀌어 parser tree가 달라짐 | 깊이 64 제한을 넘으면 원문 전체를 그대로 반환한다. 해당 입력을 부분 수정하지 않고 기존 Stylo parse 경로에 넘기는 fail-closed 동작이다. **통과** |
| 6 | inline style 원문과 stylesheet 선언 블록을 서로 혼동 | inline은 exact `Arc::ptr_eq`로 분기해 원래 attribute text를 읽고, stylesheet는 raw block pointer에서 선언 원문을 찾는다. inline `revert-layer` 통합 회귀와 18개 inline-scene simulator 행렬이 통과했다. **통과** |
| 7 | selector specificity와 stylesheet 순서가 source 복원 결과를 뒤집음 | 실제 Stylo rule tree 순서에서 첫 winning declaration을 사용한다. 같은 specificity의 뒤 normal 규칙과 낮은 specificity의 뒤 `div` 규칙을 모두 대조하는 cascade 단위 검사가 통과했다. **통과** |
| 8 | 동일 declaration block에서 마지막 값 또는 importance별 winner를 놓침 | block declaration iterator를 실제 cascade importance와 맞춰 역순으로 읽고 원문에서도 해당 importance의 마지막 `word-break`를 복원한다. source-order와 양쪽 `!important` 대조 검사가 통과했다. **통과** |
| 9 | `inherit`·`unset`·미지정 상속이 부모 alias를 잃거나 만든 값이 자식으로 누출 | parent alias flag를 element traversal과 synthetic html/body 경로로 전달한다. 자식 미지정·`inherit`·`unset` 회귀가 모두 `break-word`를 보존한다. **통과** |
| 10 | `revert`가 lower cascade origin의 지정값을 무시하고 parent alias를 유지 | Stylo `CascadePriority::allows_when_reverted(Origin)`으로 author 선언을 건너뛰도록 수정했다. author parent alias, child `revert`, lower UA `normal` 통합 검사가 `normal`을 확인했다. **통과** |
| 11 | `revert-layer`가 lower layer/style-attribute 이전 선언을 놓침 | Stylo layer priority rollback을 사용한다. child inline style의 `revert-layer`가 lower stylesheet의 `break-word`를 복원하는 통합 검사가 통과했다. 일반 author `@layer` admission은 이 fixture profile 범위 밖이다. **통과** |
| 12 | rollback을 적용할 때 같은 origin/layer의 더 약한 규칙을 잘못 살림 | 실제 pinned Stylo `CascadePriority` 비교를 재사용해 cascade origin과 layer priority를 따른다. lower-origin·lower-layer priority unit cases가 통과했다. **통과** |
| 13 | 원본 declaration source를 찾지 못했는데 부모의 alias를 성공값으로 사용 | winning parsed declaration이 있지만 원문 복원이 안 되면 `Other`로 내려 alias를 fail closed한다. 선언이 실제로 없는 경우에만 `None`으로 부모 값을 사용한다. **통과** |
| 14 | rollback 뒤 lower declaration이 없을 때 상속 property의 fallback이 달라짐 | `word-break`가 inherited임을 반영해 rollback 후보를 소진한 경우 parent value를 반환한다. no-lower-declaration unit case가 통과했다. **통과** |
| 15 | computed CSSOM 출력은 alias인데 layout은 다른 스타일 snapshot을 소비 | computed property map의 `word-break` override는 Stylo cascade 결과와 같은 `ComputedStyleSnapshot` 구성 시점에만 반영된다. runtime 검사는 두 computed property와 `document_revision`을 같은 snapshot에서 확인한다. **통과** |
| 16 | `word-break: break-word`를 `break-all`로 바꾸거나 실제 overflow-wrap 값을 덮어씀 | Parley에는 `WordBreak::Normal`과 effective `OverflowWrap::Anywhere`를 전달하고 입력/출력의 computed overflow-wrap mode는 별도 provenance로 보존한다. Rust text-layout 회귀 8개가 통과했다. **통과** |
| 17 | emergency break 기회가 min-content 외 max-content 또는 unrelated mode에 새어 나감 | `break-word`의 anywhere 효과와 `overflow-wrap:break-word`의 min-content 제외를 분리한다. `normal` control, 3 overflow-wrap 값, min/max/fixed track의 독립 18개 기대값을 양 플랫폼에서 확인했다. **통과** |
| 18 | track 크기는 맞지만 line count·advance·row height·문서 revision이 잘못됨 | runtime fixture test는 intrinsic sizes, anonymous text owner, 각 line advance와 최종 column track을 함께 비교한다. simulator capture는 frame, text layout, paragraph values, revision tuple, intrinsic inputs, final Grid track을 장면마다 검증했다. Android 18/18, iOS 18/18이다. **통과** |
| 19 | wrong owner/language/track, 중복 direct text 또는 미등록 scene이 성공으로 처리 | runtime negative tests는 computed mismatch, language/`xml:lang`, track, duplicate node, unknown scene을 거부한다. combined runtime 테스트 5개와 FFI scene dispatcher test 1개가 통과했다. **통과** |
| 20 | fixture API가 기본 artifact에 새거나 Android/iOS 기대값·oracle이 서로 다름 | opt-in boundary 검사 31개가 feature guard, exact 18 IDs, 양 플랫폼 route와 independent precomparison 연결을 확인했다. 설치 APK/executable hash는 빌드 파일과 일치하고, capture의 Ahem 계산값으로 양 플랫폼 trace 18개씩 통과했다. **통과** |

확인 범위는 Android API 37.1 ARM64 16 KB SwiftShader AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 고정 fixture다. 화면 캡처는 현재 build에서 glyph paint가 비활성화되어 있어 글자 픽셀 근거로 쓰지 않는다. 일반 CSS admission, author `@layer` 문법, 임의 custom-property alias, WPT, 실기기/hardware GPU, glyph/compositor pixel 및 전체 C11.4/C11은 입증하지 않는다. [실행 matrix·trace·hash](./README.md) · [독립 사전 비교](../c11-4-anonymous-grid-text-word-break-break-word-precomparison-2026-11-01/README.md).
