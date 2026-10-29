# C11.4 zoomed `fit-content` PR diff 실패 관점 검토 30개

2026-10-29 · 구현/runtime 20개 검토와 별개로 변경 diff의 제품 경계, 통합, 재현성과 문서 주장을 검토했다. 계획 검토 표를 복사하지 않았다.

| # | diff 공격 관점 | 확인 및 결과 | 판정 |
|---:|---|---|---|
| 1 | CSS `zoom`이 일반 C11.4 allowlist에 추가돼 다른 문서까지 받아들이는가 | 일반 함수는 유지되고 zoom 포함 함수는 별도 opt-in feature 아래 분리돼 있다. | 통과 |
| 2 | fixture-test 전용 `StyloDocumentView` 생성자가 기본 제품 API에 노출되거나 호출자가 arbitrary base URL을 선택하는가 | 고정 C11.4 constructor만 feature-gated이며 임의 URL helper는 private다. public opt-in constructor는 Chrome-only route를 고정한다. 기본 `c04-runtime-gpu` build가 확인됐다. | 통과 |
| 3 | `ComputedCssZoom` 필드가 기본 cascade snapshot ABI를 바꾸는가 | type과 snapshot field 모두 C11.4 fixture feature 아래 조건부 컴파일된다. default 제품 archive에서도 fixture symbol/payload가 없다. | 통과 |
| 4 | incremental style recompute가 zoom을 빼거나 이전 값을 재사용하는가 | incremental snapshot 생성/재사용 경로에 같은 feature-gated `c11_4_zoom` 복사가 들어 있다. | 통과 |
| 5 | unrelated Flex/Grid allowlist와 기존 visibility/text-orientation 검사가 약화됐는가 | 기존 allowlist와 property 검증을 공유하고 zoom 전용 선언 한 개만 추가한다. `float` 거부 regression이 있다. | 통과 |
| 6 | cascade helper가 product path에서 fixture zoom을 활성화하는가 | `compute_runtime_block_flex_grid_track_sizing_zoom_fixture_test_cascade_with_stylesheets`는 전용 feature로만 export한다. 일반 cascade test는 `zoom`을 거부한다. | 통과 |
| 7 | `RuntimeBlockFlexGridTrackSizingV1` 전체에 root-wide zoom exception이 추가되는가 | 예외 선택은 exact scene root의 ID predicate로 분기되고 그 밖에는 원래 unsupported-property 검사로 돌아간다. | 통과 |
| 8 | Stylo 값 대신 source 문자열만 읽어서 `40px→80px`로 바꾸는가 | Stylo typed snapshot에서 effective zoom과 dimensions를 보존하고 layout projection에 typed lengths를 넘긴다. inline source는 fixture builder 입력으로만 쓰인다. | 통과 |
| 9 | stylesheet cascade나 authored zoom inheritance가 route 이름만으로 다른 stylesheet에 적용되는가 | zoom property snapshot은 지정된 opt-in cascade helper에서만 추출되고 runtime validator가 root/item/child 값과 전체 style set을 재확인한다. | 통과 |
| 10 | root zoom predicate helper가 기본 경로의 other unsupported properties를 통과시킬 수 있는가 | true branch에서 zoom route 검사를 수행하고 false branch는 기존 일반 경로를 보존한다. style regression에서 float는 fixture route도 실패한다. | 통과 |
| 11 | runtime fixture dispatch가 다른 scene 또는 prefix 변형을 받는가 | scene ID 문자열 완전 일치만 source를 반환한다. 변형 scene FFI unit test는 실패 응답을 확인한다. | 통과 |
| 12 | root에 맞지 않는 generic node selector/style를 ID로만 강제 연결하는가 | host document parent, namespace/tag, exact ID, node ownership, style node set을 준비와 출력 시 확인한다. | 통과 |
| 13 | `RuntimeC11_4FitContentZoomedGridObservation`이 trace report를 통해 빠질 수 있는가 | diagnostics snapshot clone, fixture report serializer와 C11.4 report JSON의 해당 field가 이어진다. 최종 platform trace chunk에 값이 있다. | 통과 |
| 14 | report JSON에 WPT 기대값과 engine trace가 한 필드로 섞이는가 | CSSOM `40px`, typed Taffy limit `80px`, contribution `800px`, used row `80px`가 distinct keys다. README도 역할을 구분한다. | 통과 |
| 15 | instrumentation 변경이 기존 C11.4 fixture 출력 JSON contract를 깨는가 | 새 optional field만 추가하고 기존 fields는 그대로 둔다. 캡처/fixture boundary suite 20/20. | 통과 |
| 16 | FFI module registry가 fixture를 제품 runtime source 루트에 추가하는가 | 새 scene/runtime report는 `runtime_gpu/fixtures/c11_4/` module tree에 연결한다. 이전 경로 고아 파일은 없고 boundary test가 확인한다. | 통과 |
| 17 | fixture module 위치만 분리하고 Cargo feature gate를 빠뜨리는가 | Cargo feature `c11-4-grid-track-sizing-fixture-test`가 style/runtime feature에 전달되고 FFI fixture 모듈도 opt-in `c04-runtime-gpu-fixture-test` 하위다. | 통과 |
| 18 | C header/FFI exported function을 불필요하게 늘리는가 | 새 외부 C entrypoint는 추가하지 않고 기존 fixture harness route/report 구조에 붙인다. | 통과 |
| 19 | Android host process 내부 경로에 fixture scene selection을 제품 입력으로 연다 | compile-time fixture flag와 scene allowlist 경로에서만 source와 pixel matrix를 선택한다. | 통과 |
| 20 | Android viewport 높이 예외가 모든 scene에 적용되는가 | zoomed fixture scene ID 전용 높이 `800` 분기가 기본 `240` 환경을 바꾸지 않는다. capture manifest와 strict presentation identity가 정확히 `320×800`을 확인한다. | 통과 |
| 21 | Android PixelCopy sample을 오래된 surface generation에서 읽어 우연히 맞추는가 | expected viewport/revisions, surface generation, presentation identity를 먼저 맞춘 뒤 4개 표본을 읽고 로그에 generation을 남긴다. | 통과 |
| 22 | Android AVD RGBA만으로 hardware GPU나 실기기 결과라고 확대하는가 | 결과를 API 37 ARM64 software AVD, ANGLE/SwiftShader로 제한해 기록한다. | 통과 |
| 23 | iOS `pixelInput` size check만 바꾸고 Metal request path를 240px로 남기는가 | 두 경로에서 expected viewport height가 800이며 final request와 drawable은 `320×800` / `960×2400`이다. | 통과 |
| 24 | iOS Metal readback의 format/scale/bounds가 sample별로 검증되는가 | output summary는 scale 3.0 drawable과 표본별 expected/observed RGBA·exact bit를 기록한다. 4/4 matched. | 통과 |
| 25 | iOS drawable 표본을 compositor 최종 화면 proof라고 표현하는가 | 코드와 README 모두 compositor 제출 전 Metal readback이라고 제한한다. | 통과 |
| 26 | sample 한 점이 child painting과 overflow 경계를 대표한다고 과장하는가 | row 안, item 밖, root 밖 child yellow 세 지점과 root 밖 black control을 따로 사용한다. | 통과 |
| 27 | screenshot만으로 frame/overflow를 성공 판정하는가 | frame/track/revision summary와 per-pixel PixelCopy/Metal assertions가 성공 기준이다. screenshot은 보조 확인으로만 보존한다. | 통과 |
| 28 | capture catalog의 scene count·allowlist가 새 항목과 어긋나는가 | 전체 fixture boundary suite의 stale count를 542로 고친 뒤 20/20이다. scene selection은 한 ID만 선택한다. | 통과 |
| 29 | 변경이 공식 C11.4 또는 완료 수를 조기에 올리는가 | `spec/STATUS.md`는 C11.4 checkbox 미완료, 공식 `72/213`을 유지한다. 내부 계약 0059도 exact fixture와 남은 범위를 제한해 적는다. | 통과 |
| 30 | PR 설명의 화면/빌드 주장이 재생 불가능하거나 증거 파일이 빠지는가 | README와 계약 0059가 최신 run IDs, 결과 JSON, raw log, 화면 SHA-256, 앱 빌드 및 검증 한계를 연결한다. focused checks와 Clippy는 최신 코드에서 재실행됐다. | 통과 |

## 검토 결과

30개 diff 관점에서 병합을 막는 변경 결함은 남지 않았다. 검토 중 발견한 arbitrary base URL 파라미터는 public fixture constructor에서 제거해 Chrome-only URL을 고정했다. 기본 제품과 opt-in artifact 분리, 두 simulator surface 판정, 문서의 제한 범위가 같은 exact fixture slice를 가리킨다. 별도 일반 API 확장이나 전체 C11.4 완료 주장은 하지 않는다.
