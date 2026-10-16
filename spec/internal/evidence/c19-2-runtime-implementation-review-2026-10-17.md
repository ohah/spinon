# C19.2 runtime 구현 적대적 검토 · 2026-10-17

## 검토 범위

- 대상: C19.1/C19.2 paint snapshot, stacking command 생성, WGPU opacity compositor, fixture FFI, Android/iOS 표면 표본 수집 경계.
- 기준 입력: 고정 C19.2 `360×240` CSS fixture와 기존 `runtime_opacity`/`runtime_stacking` 장면.
- 런타임 근거: Android API 37 AVD PixelCopy·offscreen 16/16, iPhone 17 Pro와 iPad Pro 13-inch iOS 26.2 Metal drawable 각각 16/16. 재실행별 원본은 [시뮬레이터 실행 기록](./c19-2-runtime-simulators-2026-10-17/README.md)에 연결했다.
- 이 기록은 아래 20개 구현 실패 경계를 독립적으로 점검한다. 구현 전체, C19 전체 또는 미지원 환경의 적합성 인증은 아니다.

## 실패 관점 20개

| # | 공격한 실패 경계 | 확인과 판정 |
|---:|---|---|
| 1 | `EndGroup` 선행, 닫히지 않은 group, 불균형 명령에서 depth underflow 또는 잘못된 draw | `RuntimeRenderSnapshot::validate_paint_commands`는 각 `EndGroup` 전 depth를 확인하고 끝에서 depth 0을 요구한다. **통과**. |
| 2 | NaN, 무한대, 0 미만 또는 1 초과 opacity가 shader uniform까지 유입 | snapshot 검증이 유한한 `0..=1` 범위만 허용한다. **통과**. |
| 3 | 중복·누락·역순·범위 밖 box index가 다른 geometry를 그리거나 panic | snapshot 검증은 box index가 순차적이고 box 개수 안에 있는지 검사한다. **통과**. |
| 4 | opacity가 다른 형제 context 사이에서 이전 group이 열린 채 다음 box에 섞임 | `paint_commands_for_boxes`는 context 경로의 공통 prefix만 유지하고 나머지를 닫은 뒤 연다. 마지막에 모든 group을 닫는다. 생성된 snapshot도 command 검증을 거친다. **통과**. |
| 5 | 루트 요소 opacity가 자손 전체 대신 루트 box에만 적용됨 | root stacking context의 opacity를 context 경로에 포함하고 같은 group 종료 때 합성한다. 구현 경로는 **통과**. 고정 C19.2 표면 fixture에는 루트 자체 opacity 사례가 없어 해당 변형은 런타임 표본 검증이 남았다. |
| 6 | 중첩 opacity를 자식에 반복 적용하거나 부모 group 경계를 건너 합성 | `precise_group_over.wgsl`은 자식 layer를 `EndGroup`에서 부모 layer로 한 번 합성하며 nested group은 순서대로 닫힌다. nested-opacity 고정 픽셀과 runtime opacity 테스트가 확인됐다. **통과**. |
| 7 | 색 alpha와 group opacity를 더하거나 premultiplied RGB에 alpha를 다시 적용 | 색 pass는 premultiplied source-over를 계산하고 group pass는 premultiplied RGB와 alpha에 같은 opacity를 곱한다. `runtime_opacity`는 source alpha와 nested opacity 픽셀을 고정값에 대조한다. 추가 회귀 점검에서 C08 profile이 partial-alpha 배경을 계속 거부하는 negative control도 통과했다. **통과**. |
| 8 | root가 투명해 검정 backdrop이 흔들리거나, 투명 group이 불투명하게 지워짐 | root layer는 불투명 검정으로 시작하고 각 group layer는 alpha 0으로 clear한다. 고정 fixture의 root backdrop 표본과 group overlap 표본이 예상값과 일치한다. **통과**. |
| 9 | source와 destination texture가 같은 ping-pong 대상으로 alias되어 픽셀이 손상 | box/group 합성 전 현재 layer를 반대 texture로 복사한 뒤 다음 texture에 기록하고 current 인덱스를 전환한다. **통과**. |
| 10 | 색 alpha 경계에서 8-bit 반올림이 플랫폼별로 달라지거나 sRGB surface에 검증 안 된 shader를 적용 | 정확도 후보는 `Rgba8Unorm`/`Bgra8Unorm`만 허용한다. sRGB 형식은 기존 경로로 우회하며 단위 테스트가 네 형식 경계를 고정한다. C19.2의 색 변환도 opt-in profile에서만 호출하며 이전 profile은 legacy alpha 정책을 유지한다. **지원 형식 안에서 통과**. |
| 11 | vertex 색이 linear/sRGB로 두 번 또는 한 번도 인코딩되지 않음 | format 선택에 따라 `ColorEncoding`을 고르고, 정밀 compositor는 허용한 비-sRGB unorm 표면에서 직접 sRGB 채널을 보존한다. 해당 형식·색 경계의 geometry 테스트와 시뮬레이터 픽셀 근거가 있다. **통과**. |
| 12 | ancestor clip이 opacity layer 합성 뒤 적용되어 잘린 자손이 밖으로 표시됨 | geometry 생성 시 각 box/border를 누적 clip과 교차한 뒤 같은 paint command 순서로 layer에 그린다. clip 내부·밖·겹침 표본 및 hit 결과가 `runtime_opacity`에 고정됐다. **통과**. |
| 13 | opacity context가 negative/zero/positive z-index 또는 flex-item paint phase를 바꿈 | stacking flatten 단계는 기존 phase별 순서를 만든 뒤 paint command가 이를 그대로 감싼다. `runtime_stacking`은 z-index 및 flex overlap을 검사한다. opacity + clip 통합 표본도 통과했다. **통과**. |
| 14 | opacity 0 또는 색 alpha 0인 요소가 hit-test에서 빠지거나 hit 순서가 바뀜 | hit-test는 alpha/opacity가 아닌 geometry, clip, 기존 paint/hit order를 사용한다. `runtime_opacity`가 opacity 0의 hit target과 clipped miss를 확인한다. **통과**. |
| 15 | 현재 요소의 background와 border가 서로 덮는 순서 또는 `currentColor` alpha가 잘못됨 | geometry는 box fill 뒤 border quads를 만들고 element의 resolved currentColor alpha를 전달한다. border 외곽/내부 고정 표본이 시뮬레이터에서 통과했다. **통과**. |
| 16 | opacity group에 image가 포함되면 정밀 compositor가 image를 누락하거나 무시 | 정밀 경로는 image scene을 거부하고 기존 opacity renderer로 넘긴다. image + group 조합의 정밀 일치는 검증되지 않았다. 이 **제한은 문서에 명시**했으며 정확성 근거를 확대하지 않는다. |
| 17 | 과도한 nesting이 group 배열을 벗어남 | 정밀 경로와 기존 경로 모두 최대 64 depth를 검사한다. 경계 단위 테스트가 64 허용·65 거부를 확인한다. **통과**. |
| 18 | 0 크기·최대 texture dimension 초과·곱셈 overflow·레이어 과다 할당 | dimension 검사, checked multiplication, root와 ping-pong group을 포함한 256 MiB 사전 예산이 draw 전에 적용된다. 정확한 256 MiB 경계·초과·overflow·texture extent 단위 테스트가 있다. **통과**. |
| 19 | WGPU layer allocation 오류가 uncaptured 상태로 남거나 부분 명령 뒤 fallback을 실행 | pipeline/layer 생성은 OutOfMemory·Validation error scope 안에서 이뤄진다. 정밀 layer 확보가 실패하면 encoder 명령을 추가하기 전 오류를 돌려 기존 RGBA8 경로를 시도한다. **확인한 texture/layer 실패 경로는 통과**. |
| 20 | FFI 표본 포인터·개수·좌표가 잘못되어 임의 readback이나 잘못 연결된 결과를 만듦 | null·고정 개수·곱셈 범위·중복·공통 1~8 배율 좌표를 검사하고 `# Safety`에 실제 메모리 범위·host 수명·동시 호출 계약을 적었다. Android JNI 및 iOS capture는 drawable 크기/generation/좌표와 결과를 재검사한다. FFI negative 단위 테스트와 시뮬레이터 artifact가 있다. fixture 심볼은 C19.2 opt-in feature에서만 노출한다. **통과**. |

## 후속 발견 및 profile 경계 수정

전체 workspace 회귀 검증에서 기존 C08 profile도 C19 Color 4 sRGB conversion을 사용해 부분 alpha 배경을 허용하는 호환성 회귀를 발견했다. 이 회귀는 paint 결과 자체보다 새 계산 API를 기존 profile까지 공유한 경계 오류였다.

- 기존 `runtime_paint_from_absolute_color`는 legacy sRGB 변환 및 alpha `0/1` 허용 정책으로 복구했다. partial alpha는 scene을 publish하지 않는다.
- 별도 `C19BoxPaintOpacityV1` profile과 C19 전용 conversion/cascade API를 추가해 C19.2 fixture만 새 계산을 사용하도록 했다. incremental style, layout, render와 Android/iOS fixture 진입점에도 profile을 전달한다.
- C08 거부 negative control, legacy color helper alpha 거부, `runtime_opacity` 고정 픽셀, C19.2 FFI feature 경로를 재검증했다. 새 profile 기준 Android offscreen/PixelCopy와 iPad Metal 표본은 각각 제품 `16/16`, Chrome `15/16`으로 일치한다. 원본 실행은 [시뮬레이터 실행 기록](./c19-2-runtime-simulators-2026-10-17/README.md#c19-전용-계산-profile-재실행)에 연결했다.
- 이 수정으로 초기 리뷰의 “새 결함 없음” 판정은 대체한다. 영향받은 #7(alpha policy), #10(color conversion boundary), #20(opt-in FFI route)을 수정 후 다시 확인했다. 나머지 구현 관점은 이전 표와 동일한 구현 경로를 다룬다.

## 결과와 남은 검증

- 후속 회귀 검증에서 발견한 profile 오염 결함은 수정하고 위 영향 관점을 다시 확인했다. 표면 형식·image scene·root-opacity fixture 제한은 통과로 승격하지 않고 각각 별도 상태로 남긴다.
- `cargo fmt --all -- --check`와 `git diff --check`를 2026-10-17 현재 후보에서 다시 실행해 통과했다.
- 미검증 범위: Android hardware GPU·실기기, iOS compositor 이후 화면, image가 포함된 opacity group의 pixel parity, root element opacity 고정 런타임 표본, text glyph·contrast 통합, 다른 GPU backend와 sRGB-encoded surface 형식.
- 따라서 C19.2 및 상위 C19 완료 체크는 보류한다. PR 검토·전체 CI가 끝나기 전에는 병합 준비 완료로 판정하지 않는다.
