# C11.1 aspect-ratio 자동 최소 크기 수정 후 구현·런타임 검토 20개

**대상:** 후보 구현 최종 변경과 2026-11-05 실행 근거. 초기 구현 검토 및 두 계획 검토와 분리했다.

| # | 독립 실패 경로·불변 조건 | 확인 및 판정 |
|---:|---|---|
| 1 | 다른 Grid의 같은 스타일 텍스트까지 새 최소 크기를 받음 | eligible target은 `RootHtmlDirectTextAspectRatioV1` owner가 `is_supported` 전체 검사를 통과한 target `NodeId`만 담는다. 통과. |
| 2 | layout node마다 모든 input node를 다시 훑어 quadratic 경로가 됨 | target `BTreeSet`을 tree 구성 전에 한 번 만들고 membership 조회한다. 검토 중 발견한 반복 전체 탐색을 제거했다. 통과. |
| 3 | 비슷한 ID나 잘못된 owner-child 관계가 target으로 연결됨 | `NodeId` exact membership과 기존 owner/topology admission이 함께 필요하다. 통과. |
| 4 | width-locked text의 transfer가 line box보다 작아 한 줄 높이가 붕괴 | `width:60px;height:auto` exact target에만 finite positive line-height를 min-height로 설정한다. Ahem 10px 기대 검사가 통과했다. |
| 5 | line-height가 0 또는 무한대일 때 잘못된 min-height 생성 | finite positive 검사를 거친 뒤 `set_min_height`를 호출한다. 통과. |
| 6 | 일반 wrapped paragraph의 automatic minimum을 이 제한 slice로 추정 | target eligibility는 고정 `AAAA` paragraph와 기존 nowrap/layout 조건을 포함한다. 통과. |
| 7 | height-locked `1/100`에서 transferred 0.3px가 텍스트 min-content보다 우선 | 동일 paragraph의 text engine `intrinsic_inline_sizes`에서 min-content를 구해 40px 최소 폭을 적용한다. Ahem frame 검사가 통과했다. |
| 8 | 폰트별 min-content를 플랫폼 공통 상수로 고정 | 구현은 등록된 text engine 측정값을 사용하고, Chrome monospace 24.09375px 관찰은 플랫폼 간 고정값으로 단언하지 않는다. 통과. |
| 9 | intrinsic measurement가 NaN, 무한대 또는 음수 | finite·nonnegative guard에서 `UnsupportedTextParagraph`로 실패시킨다. 통과. |
| 10 | text layout engine 부재를 panic 또는 부분 layout으로 처리 | `MissingTextLayoutEngine`을 반환하며 tree 완성 전에 실패한다. 통과. |
| 11 | min-content를 한 번 재고 실제 measure callback이 다른 결과를 재사용 | 측정한 `LayoutIntrinsicSize`를 같은 Taffy text node ID에 보관해 measure callback의 cache와 공유한다. 통과. |
| 12 | row target의 intrinsic cache key가 owner와 합성 paragraph ID를 혼동 | target은 실제 postorder node이며 `engine_ids[&external_id]`에 저장한다. `AnonymousGridText` 합성 노드 경로는 eligibility에서 제외된다. 통과. |
| 13 | `100/1` transfer 0.6px가 one-line automatic minimum을 낮춤 | line-height 10px로 row와 target을 유지하는 focused Rust 검사 및 simulator fixture가 통과했다. |
| 14 | finite ratio라도 두 번째 절대 frame의 `x+width`/`y+height`가 무한대 | 공통 frame 수집이 origin·size와 오른쪽/아래 끝점을 모두 검사한다. 두 scene 합성 overflow 회귀가 `NonFiniteFrame`으로 거부됐다. |
| 15 | ratio 방향 또는 나눗셈/곱셈을 반대로 적용 | 14 scene matrix가 width→height와 height→width 숫자를 각각 검사한다. Focused 6/6 통과. |
| 16 | height 80px 예외가 다른 ratio나 no-ratio target에 열림 | projection과 layout 모두 typed ratio 0.5 조합만 admission하며 3/2·40px 조합 negative가 거부된다. 통과. |
| 17 | `auto`, zero, typed box 차이를 numeric transfer로 취급 | typed numeric BoxSizing 값만 전이한다. zero, `auto`, `auto <ratio>`와 누락 state 경계 테스트를 유지했다. 통과. |
| 18 | 75px target이 70px Grid 경계에서 잘려 visible overflow가 사라짐 | 5/2 height-locked frame은 75px이며 15px overflow를 Rust와 runtime geometry에서 단언한다. 통과. |
| 19 | 정적 layout green을 앱 runtime이나 두 플랫폼 동작으로 확대 해석 | Android API 37 AVD와 iOS 26.2 Simulator에서 주/보충 fixture 4회 모두 layout-ready 및 draw/present에 도달했다. NodeId frame 및 surface/compositor readback 주장은 제외했다. 통과. |
| 20 | fixture 교체·복구 실패, broad regression 실패 은폐 | APK/app bundle fixture hash 일치와 bootstrap 원본 두 SHA 복구를 재확인했다. Focused 6+1+1+1 통과; broad suite 441/442 및 391/401 실패는 각각 로그와 README에 분리 기록했다. 통과. |

**판정:** 검토 중 target eligibility를 layout node마다 다시 검색하던 비용을 발견해 한 번 구성한 `BTreeSet<NodeId>`로 바꿨다. 바뀐 코드 이후 focused Rust 검증, Clippy/rustfmt 및 네 simulator 실행이 통과했다. Broad package test 전체는 기존 실패 때문에 green이 아니며 이 문서는 그 실패를 성공으로 간주하지 않는다.
