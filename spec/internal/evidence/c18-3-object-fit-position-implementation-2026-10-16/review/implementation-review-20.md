# C18.3 구현 적대 검토 20개

2026-10-16에 C18.3 구현 diff, fixed fixture, Rust/WGPU 검증, Android AVD·iOS Simulator 표면 로그를 서로 다른 실패 관점에서 확인했다. 계획 문서의 사전 검토와 PR diff 검토는 별도 기록이다.

| # | 독립 실패 관점 | 확인한 경계와 근거 | 결과 |
| ---: | --- | --- | --- |
| 1 | 모르는 `object-fit` keyword가 조용히 `fill`로 바뀌는가 | `RuntimeImageObjectStyle::parse`; `scale` 기대 거부 테스트 | 통과: 명시 오류 |
| 2 | position keyword·길이·`calc()`·잘못된 항 수가 성공 경로에 들어오는가 | parser와 Stylo inline/style profile 사전 검사; negative cases | 통과: 제한 profile 밖 값 거부 |
| 3 | `NaN`·무한 percentage가 geometry에 전파되는가 | finite 검사 및 `RuntimeImageObjectPosition::new` | 통과: 생성 단계에서 거부 |
| 4 | `image-rendering` 미지원 값이 `auto`로 처리되는가 | computed value 검증 및 `crisp-edges` 거부 테스트 | 통과: 조용한 fallback 없음 |
| 5 | CSS cascade 대신 fixture 상수로 style을 그리는가 | live V8 fixture의 computed `object-fit`, `object-position`, `image-rendering` 검증 | 통과: 12개 image의 computed value를 대조 |
| 6 | `fill`, `contain`, `cover`, `none`, `scale-down`의 size 식이 섞이는가 | `c18_3_object_fit.rs` geometry tests와 12 scene report | 통과: 각 분기와 scale-down 양 분기 확인 |
| 7 | 음수 여유 공간과 0–100 밖 percentage를 clamp하는가 | `-25% 150%`, `150% -25%` geometry 및 surface scenes | 통과: CSS free-space 식으로 위치 계산 |
| 8 | 이미지 object rect가 Taffy element frame 또는 paint order를 변경하는가 | snapshot은 기존 frame을 보유하고 image geometry에서 별도 rect 사용 | 통과: frame·object rect를 분리 |
| 9 | style revision이 바뀐 scene이 이전 object style을 수용하는가 | `RuntimeRenderSnapshot::with_images` revision 검사와 stale-style test | 통과: `InvalidImageStyleRevision` |
| 10 | 다른 문서·tree·intrinsic revision의 PNG가 재사용되는가 | `RuntimeImageResourceKey`, request completion, snapshot source 검사 | 통과: 전체 source key에 결합 |
| 11 | PNG byte length 또는 natural dimensions가 틀려도 upload되는가 | checked RGBA8 byte length, fixture의 24×12 dimensions·source 검증 | 통과: 잘못된 입력 거부 |
| 12 | 이미지가 content box보다 클 때 content 밖이 그려지는가 | frame/object/ancestor clip 교집합과 `none-outside-content-box` 표본 | 통과: 19개 중 밖 표본이 배경색 |
| 13 | cover crop 뒤 UV가 원본 이미지 좌표와 어긋나는가 | clip/object 교집합에서 UV 재계산; geometry tests 및 center/start/end 표본 | 통과: 표면 표본 exact |
| 14 | 빈 교집합이나 역전된 UV가 정점으로 제출되는가 | empty quad는 정점 생략, 유효하지 않은 UV/좌표 검증 | 통과: 실패 또는 no-draw 경로 |
| 15 | image cache가 같은 bytes의 서로 다른 source·node·sampling을 혼동하는가 | `ImageCacheKey`와 `images_tests.rs` identity cases | 통과: revision/content/dimensions/sampling을 구별 |
| 16 | auto sampling의 encoded midpoint가 Chrome 결과와 다른가 | WGPU `LinearEncoded` shader, 8-bit texel 복원 및 ties-to-even; DPR별 oracle | 통과: 기존 `pow` 경로의 중간값 차이를 수정하고 exact 확인 |
| 17 | WGPU dual texture view가 Android downlevel 기능에 의존하는가 | sampling별 `Rgba8Unorm`/`Rgba8UnormSrgb` texture format 사용, opt-in Android build | 통과: 미지원 `VIEW_FORMATS` 요구 제거 |
| 18 | C ABI가 null host/좌표/output, 표본 수 오입력 또는 좌표 중복을 읽는가 | C18.3 FFI argument guards 및 null/잘못된 수 테스트 | 통과: 표본을 예약하기 전에 거부 |
| 19 | DPR 좌표 또는 iOS drawable format을 혼동하는가 | Android PixelCopy 588×189, iOS Metal `Bgra8Unorm` 672×216 raw log | 통과: 각 platform surface mapping에서 19/19 |
| 20 | default app에 fixture ABI·JS·PNG가 포함되거나 화면상 성공을 오인하는가 | [artifact 경계](../artifacts/README.md), 실제 app hash 검사, full-screen screenshot과 surface log 분리 | 통과: default 제외·opt-in 포함, compositor 이후 주장 없음 |

검토 중 확인된 동작 문제는 수정을 끝냈다. C18.3 iOS route guard를 C18.1 전용 guard에서 공용 fixture guard로 옮겼고, 이 변경으로 낡아진 C18.2 iOS build-flag assertion도 새 공용 guard 계약에 맞췄다. Android에 지원되지 않는 dual-view texture feature 요구는 sampling별 texture format으로 바꿨으며, Chrome midpoint에 맞게 encoded texel을 8-bit로 복원한 뒤 ties-to-even 양자화를 적용했다. 최종 simulator 표본은 각 19/19 exact다.

남은 경계는 실기기·hardware GPU, iOS compositor 이후 표면, 전체 `<position>` 문법, 일반 이미지 로더와 C18 전체 지원이다.
