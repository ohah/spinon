# C11.4 `visibility:hidden` mixed-track PR diff 적대 검토 30개

계획 검토와 구현/runtime 20관점과 별개로 제출 변경의 파일 경계, 플랫폼 동등성, provenance와 범위를 검토했다. 각 행은 서로 다른 회귀 경로 또는 계약 경계를 다룬다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 새 Rust 파일이 fixture 모듈 경계 밖에서 컴파일된다 | FFI·runtime adapter는 기존 `fixtures/c11_4/.../generic_intrinsic_flex_fixed/` 하위 모듈로만 연결했다. 기본/fixture release archive를 별도 빌드해 symbol 경계를 확인했다. |
| 2 | helper가 이름이 비슷한 scene ID를 접두사 매칭한다 | FFI route는 정확한 문자열 네 개만 받고 unknown suffix·collapse는 거부한다. |
| 3 | legacy 8개 scene의 source 분기가 새 helper에 흡수된다 | 기존 scene table을 먼저 검사하고 후속 helper에서만 새 ID를 매칭한다. 기존 8개와 display:none 2개가 두 앱 행렬에서 회귀하지 않았다. |
| 4 | runtime helper 우선순위가 display:none hidden scene을 잘못 가져간다 | visibility root ID가 exact scene map에 없으면 `None`을 반환한 뒤 기존 hidden helper가 그대로 처리한다. 전체 14 scene이 통과했다. |
| 5 | generated JavaScript가 scene ID나 root ID를 문자열로 안전하게 고정하지 않는다 | IDs는 Rust formatting의 quoted value로 만들고 data-node·root/child identity를 각각 기록한다. app runtime summary가 exact scene/root frame으로 확인됐다. |
| 6 | Columns/Rows fixture가 서로의 root/gap/track CSS를 사용한다 | FFI source와 expected JSON은 300×120 / 120×300 root, 해당 축 gap 10px, opposite 축 고정 track을 분리한다. 네 장면이 각 matcher 표와 일치했다. |
| 7 | overlap DOM order가 pixel control의 색을 반대로 만든다 | direct child order는 A, span, image, C로 고정한다. hidden은 A 색, visible control은 span 색을 PixelCopy/Metal 표본에서 냈다. |
| 8 | test stylesheet의 positive control selector가 모든 `.span`에 잘못 적용된다 | `visibility-hidden`과 `visibility-visible-control` class만 별도 selector로 설정하며 C11.4 runtime fixture feature 아래 시험한다. |
| 9 | hidden state에서 revision 검사를 건너뛴다 | snapshot/style revision validator는 scene route 성공 뒤 첫 검사 중 하나로 호출한다. stale revision은 runtime failure다. |
| 10 | tag/namespace/parent 검사가 다른 DOM element를 허용한다 | HTML root `div`, direct span, img, div와 각 exact parent/ID를 검사한다. fixture child list가 exact four여야 한다. |
| 11 | root 또는 item style 변경이 track expectations에 숨는다 | root/visible item/span의 computed visibility, track placement, root/track style을 별도 확인하며 root track mutation·span visibility mutation test가 거부한다. |
| 12 | nested child 또는 잘못된 text node가 text contribution을 만들지 않아도 통과한다 | A/span/C direct text를 각각 고정 문자열로 검사하고 image는 leaf인지 확인한다. 신규 runtime Rust test와 app direct input matcher가 모두 exact text input을 확인한다. |
| 13 | Row control에서 writing mode나 line break가 horizontal로 바뀐다 | Rows에서만 Ahem paragraph의 `VerticalRl`과 `LineBreakMode::Normal`을 설정한다. 양 row scene의 min-content와 item frame이 기대값과 일치했다. |
| 14 | hidden span paragraph를 제거해 보이지 않지만 sizing도 빠지는 구현이 된다 | 세 text handle(A/span/C)을 intrinsic paragraph map에 두며 span의 150px input과 frame을 검사한다. display:none 경로는 기존 helper로 분리한다. |
| 15 | image network URL/decode/natural dimensions가 새 adapter에서 느슨해진다 | fixture URL, decode 완료, image snapshot 및 24×16px natural size를 다시 검사한다. app trace는 replaced source와 natural size를 확인했다. |
| 16 | renderer가 frame을 확인하기 전에 hidden node를 조용히 생략한다 | `validate_runtime_frame` 다음에만 hidden/display:none/zero-size box를 거른다. test는 hidden layout frame 유효성과 paint box 생략을 동시에 확인한다. |
| 17 | visible child override의 renderer box도 hidden parent 조건만으로 삭제된다 | renderer filtering은 각 element computed visibility에 적용한다. 이 matrix의 visible sibling/control은 expected draw/pixel proof가 있다. 일반 descendant inheritance behavior는 범위 밖으로 남겼다. |
| 18 | renderer change가 `visibility:collapse`나 임의 CSS 지원까지 주장한다 | painter는 hidden 값만 생략하며 fixture route는 네 exact computed value만 허용한다. collapse/general visibility 지원을 문서에서 주장하지 않는다. |
| 19 | Android item/draw count가 frame record count와 섞인다 | Java matcher는 hidden 3 items/visible 4 items, frame map 5개, root 포함 box count와 draw count를 각각 비교한다. Android 14/14 및 PixelCopy 표본이 통과했다. |
| 20 | Android contribution record 순서 고정 때문에 정상 runtime 정렬 변경에서 깨진다 | Java matcher는 item/kind key로 네 exact contribution을 unordered comparison하고 duplicate/missing/wrong value를 거부한다. trace 출력 순서와 결합하지 않는다. |
| 21 | Android PixelCopy 표본의 target node 또는 CSS/device coordinate가 틀린다 | Java expected samples가 node ID, root-local integer CSS 좌표, RGBA를 고정하고 PixelCopy 로그가 exact=true로 각각 일치했다. |
| 22 | iOS route list에 새 ID만 추가되고 actual Metal sample guard는 갱신되지 않는다 | Swift stage와 Metal sample matcher 양쪽에 네 ID, owner node, CSS coordinate, exact RGBA를 추가했고 iOS 14/14 통과했다. |
| 23 | Swift contribution JSON parser가 `NSNull` minimum source를 허용하지 않거나 잘못 허용한다 | `Minimum`은 exact AutomaticMinimum 문자열, MinContent는 explicit `NSNull`을 요구하며 중복·누락·다른 값도 거부한다. 실제 Metal runtime log가 통과했다. |
| 24 | iOS `C0410` Metal fixture argument gate가 새 sample scene을 무시한다 | generic surface sample validator가 C114FlexStage에 위임하고 route arguments의 node/coordinate/RGBA가 expected table에 맞아야 한다. 2/2 표본 per scene이 matched였다. |
| 25 | iOS test build에서 Metal readback opt-in flag가 빠진다 | capture build environment에 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`을 추가했다. 최초 off-flag probe 실패를 기록하고 재빌드한 full matrix를 성공시켰다. |
| 26 | scene whitelist, capture scene mapping, platform matcher table 중 한곳에서 ID가 빠진다 | C0410 Android/iOS scene set, C114FlexStage, precomparison JSON mapping, Python expected inventory를 교차 대조했다. 14 IDs와 플랫폼 tables가 일치한다. |
| 27 | fail-closed route test가 유효한 새 control scene 자체를 reject해 오진한다 | 네 visibility scenes는 positive matrix에서 각각 실행한다. legacy unknown visible-control name, unknown ID, conflicting C11.2.1 profile은 별도 negative route로 거부됐다. |
| 28 | capture harness가 screenshot이나 trace를 다른 process/run ID와 섞는다 | manifest는 run ID별 launch/runtime log, source file set hash, V8 revision, app hash, screenshot path/hash/dimensions를 남기며 독립 Python verifier가 이를 재확인했다. |
| 29 | 독립 verifier가 contribution·sample·coordinate mutation을 놓친다 | Python self-test는 15개 mutation/negative cases를 거부하고 full 14-scene Android+iOS manifest와 두 platform matcher tables를 통과했다. |
| 30 | 제한 slice를 C11.4 완료나 full app/GPU parity로 승격한다 | README와 plan은 simulator/API/device·pixel boundary를 제한하고 `spec/STATUS.md` C11.4 checkbox 및 `72/213`을 미완료로 유지한다. Merge 후 roadmap/status sync는 별도 단계다. |

**판정:** 현재 변경에서 blocker는 발견하지 못했다. 남은 한계는 plan과 [런타임 증거 README](README.md)에 적었다. `spinon-runtime`/`spinon-style-to-render` all-targets Clippy 결과는 변경하지 않은 `generic_direct_text_image/fixture.rs:311`의 baseline `map_identity` 경고를 포함하며, 영향받은 library-only Clippy는 통과했다.
