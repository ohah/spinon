# C04.12 초기 page-scale WGPU scene mapping PR 변경 검토 30개

병합 전 전체 변경을 서로 다른 30개 실패 관점으로 대조했다. 각 결과는 아래 코드·문서·test·simulator evidence에 연결한다. 특정 renderer scene slice의 통과를 일반 HTML 지원이나 전체 CI로 확대 해석하지 않는다.

| # | 독립 관점 | 대조 결과 |
|---:|---|---|
| 1 | snapshot 생성자 호환성 | `new`/`new_with_scroll_metadata*`는 mapping을 `None`으로 초기화해 legacy 입력 계약을 유지한다. 기존 renderer test 통과. |
| 2 | 배율 값의 finite/positive 경계 | DPR/page scale의 0·음수·NaN·infinity 거부를 runtime test가 확인. |
| 3 | 곱셈 overflow와 underflow | `DPR × page_scale`이 non-finite 또는 0인 경우 snapshot 부착 전에 실패. |
| 4 | 구체적인 실패 ABI | 새 내부 오류 variant `InvalidPageScaleTransform`로 기존 error shape를 재사용하지 않는다. |
| 5 | layout/input 좌표의 보존 | viewport CSS 크기와 hit-test point는 CSS px로 그대로 유지하는 assertion 통과. |
| 6 | 문서 revision rebinding | `with_document_snapshot_revision`에서 optional mapping을 보존하는 test 통과. |
| 7 | text와 paint command 재구성 | `with_texts_and_paint_commands` 이후 mapping과 images 보존 test 통과. |
| 8 | scroll revision 재발행 | `with_scroll_state_revision`은 transform 값을 복사하며 scene key도 기존 방식대로 갱신한다. |
| 9 | transform의 이중 적용 | `CssToSurfaceMapping`이 저장된 `physical_scale`을 한 번만 적용하고 Chrome 0.5/2 oracle geometry와 WGPU 경계가 일치. |
| 10 | 축간 surface dimension 혼용 | x 변환은 width, y 변환은 height를 참조하고 세로 1170×2532 readback 통과. |
| 11 | CSS origin 변화 | transform에는 translation/safe-area 값이 없고 고정 oracle bbox와 일치. safe-area는 명시적으로 제외. |
| 12 | 기존 mapping 없는 box/border 경로 회귀 | legacy 분기 계산을 유지하며 render/WGPU 전체 unit 및 integration test 통과. |
| 13 | CSS clip 적용 순서 | box geometry clip은 CSS 좌표에서 교차 후 mapping되며 page-scale readback에서 안/밖 표본 통과. |
| 14 | 물리 right/bottom edge | scale 2 WGPU 표본에서 포함 edge와 surface 밖 표본이 기대값과 일치. |
| 15 | visual viewport outside control | 세 FFI case 각각 우/하단 밖의 노란 제어 box가 보이지 않는 30표본 검사 통과. |
| 16 | border 폭·edge·z순 | 각 CSS side 폭을 기존 합성 순서로 만들고 scaled clip/border readback에서 border와 본체 색상 통과. |
| 17 | image object-fit/UV | 변환은 image position만 변경하고 source UV는 그대로 둔다. crop UV unit test와 3-case magenta 표본 통과. |
| 18 | image resource key 및 빠진 owner | 기존 source revision/node 검증을 그대로 호출하고 fixture image key가 scene revision을 맞춘다. |
| 19 | glyph source raster scale | glyph bitmap의 기존 DPR은 raster source에 남고 page scale은 geometry 투영에만 적용된다. glyph vertex test 통과. |
| 20 | glyph visual viewport CPU crop | CPU visibility bounds가 physical target / physical scale로 CSS visible 범위를 계산하며 edge/crop test 통과. |
| 21 | paint command 순서 | scene의 box/text draw 순서를 재작성하지 않는다. scale readback에서 border, body, opacity sample 결과 통과. |
| 22 | opacity group과 transform 조합 | page-scale scene의 GPU readback에서 0.5 opacity green `[0,127,0,255]` 확인. |
| 23 | readback adapter 요청 실패 | adapter/device/readback 오류는 `Result` 실패로 전파되어 FFI 보고 token 전에 중단된다. Android/iOS Simulator 양쪽 실행 통과. |
| 24 | 표면·sample point 범위 | readback helper는 0 크기와 범위 밖 표본을 사전 거부하는 기존 경계를 사용한다. |
| 25 | FFI C 문자열·길이 처리 | null pointer, 0 length, oversized source, invalid UTF-8 부정 입력 test 통과. |
| 26 | 실패 중 세션 수명 | viewport matrix의 매 반복에서 C ABI session free 후 결과를 판정하고, WGPU failure는 outer FFI에 error report로 전달된다. |
| 27 | report premature PASS | 30개 RGBA가 모두 같아야만 `page_scale_wgpu=PASS`를 붙인다. Android/iOS UI도 이 token을 확인한다. |
| 28 | Android opt-in flag 연결 | Gradle env → BuildConfig/JNI define → C header declaration → Cargo feature 경로를 대조하고 API 37 AVD 실행. |
| 29 | iOS opt-in flag 연결 | build script env → Cargo feature → ObjC/C define → Swift condition 경로를 대조하고 iOS 26.2 Simulator 실행. |
| 30 | 기본 artifact와 완료 문서 상태 | default staticlib에서 symbol/payload 부재, opt-in positive control 존재. C04.12 parent unchecked이며 scene-child만 checked. |

## 병합 전 남은 제한

- `RuntimeSession`의 기본 HTML render pipeline은 이 mapping을 자동 받지 않는다. fixture의 scene은 고정 데이터다.
- Simulator 화면은 fixture PASS 안내 화면이다. 일반 HTML paint, actual compositor surface pixels, Android 실기기와 iPhone 실기기 동작은 검증하지 않았다.
- staticlib artifact 검사와 focused test/clippy는 전체 workspace CI·플랫폼 release artifact 검증을 대신하지 않는다.
