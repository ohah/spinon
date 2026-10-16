# C18.3 구현 PR 변경분 적대 검토 30개

검토 대상은 PR [#224](https://github.com/ohah/spinon/pull/224)의 실제 diff다. 2026-10-16에 code, fixture, platform adapters, build guards, contract/status와 실행 artifact를 서로 다른 실패 조건으로 대조했다. 계획 검토와 구현 20개 검토를 재사용하지 않았다.

| # | 별도 실패 조건 | 확인한 변경과 근거 | 판정 |
| ---: | --- | --- | --- |
| 1 | 제한 fixture가 공개 HTML image 지원으로 오인되는가 | 계약 0066, 상태 C18 parent unchecked, PR 본문 scope | 통과: C18.3 opt-in 내부 slice만 표시 |
| 2 | 기본 build에 fixture ABI가 실수로 들어가는가 | default/opt-in staticlib·APK·iOS executable hash 및 symbol scan | 통과: 기본에는 FFI marker 없음 |
| 3 | fixture-test feature가 Android/iOS/Cargo 간 불일치하는가 | Cargo feature, Gradle source set, Xcode define, build script flag 검사 | 통과: dedicated C18.3 opt-in이 함께 연결됨 |
| 4 | `object-fit`의 미지원 keyword가 기본값으로 통과하는가 | `RuntimeImageObjectStyle::parse` 및 unsupported fit test | 통과: five keywords 외 오류 |
| 5 | 잘못된 position 항 수·keyword·길이·`calc()`가 수용되는가 | parser와 Stylo profile negative cases | 통과: 두 물리축 percentage만 허용 |
| 6 | NaN/무한 position이 GPU geometry에 들어가는가 | f32 finite guard 및 `RuntimeImageObjectPosition` test | 통과: 생성 중 거부 |
| 7 | `image-rendering` 지원 범위가 속성 이름만으로 넓어지는가 | computed value `auto` 검사와 `crisp-edges` negative case | 통과: 설정값이 다르면 실패 |
| 8 | inline 값만 읽어 stylesheet cascade 결과를 놓치는가 | Stylo computed style snapshot을 읽고 12개 고정 fixture value와 비교 | 통과: cascade 결과를 기준으로 삼음 |
| 9 | stylesheet/inline의 미지원 position이 profile 사전 검사에서 누락되는가 | `css_profile.rs`, `runtime_paint.rs` 검사와 style tests | 통과: 미지원 값이 attach/render 전에 드러남 |
| 10 | 기본 `object-fit`/`object-position`이 다르게 직렬화되는가 | initial-values fixture 및 cascade test | 통과: `fill`, `50% 50%` 유지 |
| 11 | `fill`이 종횡비를 잘못 보존하는가 | object geometry unit cases와 `fill` surface anchors | 통과: frame 크기로 늘어남 |
| 12 | `contain`/`cover`가 min/max 축 비율을 바꾸는가 | concrete rect unit cases와 center/start/end samples | 통과: min/max scale 분리 |
| 13 | `none`/`scale-down`이 natural size 또는 축소 분기를 잃는가 | two `scale-down` branches와 `none` clip fixture | 통과: 확대하지 않고 두 분기 확인 |
| 14 | 음수 free space의 percentage offset이 반대로 적용되는가 | `cover` 0/50/100% 및 150% position case | 통과: `(frame-object) × percentage` 적용 |
| 15 | 0·overflow·표현 불가 geometry가 유한값처럼 제출되는가 | zero frame, huge frame, finite/end-boundary guards | 통과: 명시 오류로 no-draw |
| 16 | object rect가 Taffy frame, hit-test box 또는 paint order를 바꾸는가 | `RuntimeRenderSnapshot::with_images`와 image geometry 분리 | 통과: layout box 유지 |
| 17 | style 변경 뒤 이전 object style이 새 render key에 붙는가 | `object_style_revision` snapshot gate 및 stale revision test | 통과: `InvalidImageStyleRevision` |
| 18 | 다른 document/tree/intrinsic source의 PNG가 재사용되는가 | `RuntimeImageResourceKey`, request-bound decode 및 snapshot check | 통과: generation/revision tuple 일치 필요 |
| 19 | byte 길이 또는 RGBA natural dimensions 검사가 우회되는가 | checked `width × height × 4`, fixture PNG SHA/dimensions | 통과: 불일치면 attachment 실패 |
| 20 | `object-fit:none` 큰 이미지가 image content box 밖에 보이는가 | frame/object/ancestor clip intersection 및 밖 sample | 통과: 밖 표본은 root 배경색 |
| 21 | cover crop가 clip 후 UV 원점을 틀리게 계산하는가 | clipped vertex UV 산식, WGPU geometry tests, center/edge anchors | 통과: 19개 surface 표본 exact |
| 22 | 빈 clip/역전 UV가 degenerate triangle을 그리는가 | empty intersection은 정점 생략, UV/coordinate validation | 통과: 잘못된 geometry 거부 |
| 23 | image cache가 같은 pixels라도 source·node·sampling 차이를 합치는가 | `ImageCacheKey` 및 `images_tests.rs` identity controls | 통과: cache identity가 각각 구별됨 |
| 24 | sampler의 sRGB 변환이 encoded interpolation 값을 바꾸는가 | `LinearEncoded` 전용 `Rgba8Unorm`, 기존 sampling mode 분리 | 통과: C18.1/2 경로 유지 |
| 25 | byte midpoint와 ties-to-even이 Chrome oracle에서 한 채널씩 달라지는가 | WGSL texel byte 복원, bilinear interpolation, ties-to-even quantization | 통과: Chrome DPR별 표면 sample exact |
| 26 | Android WGPU downlevel에서 image texture view feature를 요구하는가 | sampling별 texture format 사용 및 Android opt-in build/readback | 통과: 미지원 view-format feature 요구 제거 |
| 27 | renderer/readback 실패가 직전 실행의 성공 sample을 재사용하는가 | FFI/report status와 fixture run validation 경로 | 통과: 실패 상태를 보고하며 이전 run을 성공 처리하지 않음 |
| 28 | C ABI null pointer, output capacity, sample count 또는 중복 좌표가 잘못된 memory read를 만드는가 | C18.3 FFI guard와 null/wrong-count Rust test | 통과: readback 예약 전에 거부 |
| 29 | Android/iOS route가 다른 flag·좌표 배율·표면 형식을 사용해도 성공으로 보이는가 | Android AVD PixelCopy 19/19, iOS Metal `Bgra8Unorm` 19/19 및 scene revision 로그 | 통과: DPR 2.625/3 각각 독립 비교 |
| 30 | 문서가 CI·지원 범위·화면 증거를 과장하는가 | 계약, status, PR 본문, 15개 C11/C13 suite failures, Agent Browser PR screenshot load | 통과: 제한과 실패를 공개하고 PNG 두 장을 1080×2400/1206×2622로 실제 로드 확인 |

변경 중 발견한 경계 문제는 수정했다. 공용 C18 route guard를 쓰도록 낡은 C18.2 iOS build-flag assertion을 고쳤고, Android에서 지원되지 않는 WGPU view-format 요구와 Chrome midpoint rounding 차이도 제거했다. 최종 code review에서 추가 결함은 발견되지 않았다. 전체 CSS reference suite의 남은 15개 실패는 C11/C13 기존 source-boundary assertion이며 이 PR의 C18 검사는 관련 집중 로그에서 통과한다.
