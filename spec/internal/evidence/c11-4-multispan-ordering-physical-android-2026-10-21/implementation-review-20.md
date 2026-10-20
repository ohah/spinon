# C11.4 multi-span ordering 구현 적대 검토 20개

검토 대상은 [고정 multi-span fixture](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/multispan_ordering.rs), [Chrome oracle 기반 Rust 검사](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs), [Android fixture assertion](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java), [Chrome 사전 관찰](chrome155-observation.json), [SM-S731N 실기기 관찰](physical-multispan-observation.json)이다. 각 항목은 서로 다른 실패 경로를 확인했다.

1. **Chrome 기대값이 구현 결과에서 역산되는가?** — fixture와 expected columns는 runtime 실행 전 §11.5.1 계획 증가 규칙 및 Chrome 155 사전 관찰에서 고정했다. `40,60,60,0`과 `53.328125,73.328125,73.34375,0`을 구현 trace와 별도로 비교했다. 통과.
2. **서로 겹치는 같은 span의 처리에서 source order가 결과를 바꾸는가?** — 80px 및 120px span-2 item을 forward/reversed로 각각 배치했다. Chrome, Rust 및 실기기에서 두 순서 모두 `40,60,60,0`이다. 통과.
3. **큰 span이 작은 span보다 먼저 처리되는가?** — DOM에서 span-3 item을 앞세운 case에서도 intrinsic batch stage가 `[2,3]`이다. source order와 단계 순서를 분리해 검사했다. 통과.
4. **겹친 planned increase가 합산되어 과대 성장하는가?** — span-2 증가를 track별로 대조해 1·2·3번 열이 `40,60,60`이 되는지 확인했다. 통과.
5. **span-3 기여가 앞선 span-2 결과를 반영하는가?** — 누적 `160px`에 남은 `40px`을 분배한 세 track 결과를 Chrome 양자화 허용치 내에서 비교했다. 통과.
6. **배치 범위 밖의 네 번째 열이 함께 성장하는가?** — 네 번째 used track이 두 종류 case 모두 `0px`인지 Rust·실기기 trace에서 확인했다. 통과.
7. **0px gutter가 track 개수나 used size에 섞이는가?** — Rust 검사에서 gutter를 제외한 track 수를 네 개로 확인하고, 실기기 final trace에서 Chrome 비교용 네 used track을 대조했다. 통과.
8. **남은 container 공간이 intrinsic 열을 늘리는가?** — `justify-content:start`, `max-content` 열, 220px root를 고정하고 Chrome 사전 관찰과 runtime used tracks를 비교했다. 통과.
9. **row placement 차이가 column ordering 결과를 가리는가?** — 각 DOM item을 별도 10px row에 놓고 x와 y를 따로 검사했다. 통과.
10. **한 intrinsic contribution 종류가 누락되거나 잘못 정렬되는가?** — `Minimum`, `MinContent`, `MaxContent` 각각에 대해 item handle, span, 기여값과 순서를 비교했다. 통과.
11. **item frame이 track 결과와 다른 line을 참조하는가?** — 시작 column 앞의 used tracks 합으로 각 `x`를 예측하고 width/height 및 row `y`까지 검사했다. 통과.
12. **trace 검사가 다른 Grid container를 읽는가?** — root handle/node ID로 trace를 선택하고 Columns pass 및 종료 stage가 존재하는지 확인했다. 통과.
13. **fixture ID 오타가 다른 scene으로 fallback되는가?** — 네 ID만 allowlist에 등록하고 unknown ID가 `None`인지 확인했다. 생성 source 크기도 4KiB 미만으로 제한했다. 통과.
14. **동적 source에 외부 문자열이 삽입되는가?** — 생성되는 ID는 고정 scene/item 목록만 사용하고 Rust debug-string quoting으로 JS 문자열을 만든다. 임의의 사용자 입력 경로는 없다. 통과.
15. **CSS precomparison과 앱 fixture의 track/placement가 달라지는가?** — 양쪽 모두 220×40 root, 네 개의 `minmax(0,max-content)` 열, gap 0, 같은 고정 item 크기와 column lines를 사용한다. Chrome frame은 grid root 기준 상대 좌표로 비교했다. 통과.
16. **Android validator가 잘못된 node 순서나 속성 문자열을 받아들이는가?** — frame count, preorder index/node identity, `x/y/width/height` prefix를 검사한다. 적대 검토에서 발견한 `NaN` 통과 허점을 finite 검사로 막았고, 수정 후 Android 빌드와 실기기 전체 장면을 다시 실행했다.
17. **부동소수점 표현 차이로 정상 결과를 거부하거나 큰 오차를 허용하는가?** — 모든 좌표에 Chrome used-value 양자화와 runtime f32 차이를 고려한 `1/64 CSS px` 상한을 적용했다. 최종 실기기 최대 track 차이 `0.010415`, frame 차이 `0.005207 CSS px`였다. 통과.
18. **앱 fixture assertion이 trace 없는 실행을 성공 처리하는가?** — 각 scene에서 `layout=ready`, 예상 box 수, node frame 및 draw/present 로그를 확인하고 원본 multi-chunk track trace도 보존했다. 통과.
19. **실기기 결과가 emulator 또는 다른 APK에서 나온 것인가?** — Android 16/API 36 SM-S731N, Vulkan/Samsung Xclipse 940을 확인했다. local APK와 설치 경로 APK SHA-256은 모두 `191f7457528f8691c3d0d03b1d8b28887a7172939a077b80e4d4b79fe4625224`다. 통과.
20. **overflow 표본이 root 바깥 item 또는 border stroke까지 입증한다고 과장되는가?** — 실제 SurfaceView PixelCopy에서 root 안/밖, 바깥 item, root 배경 표본 4/4를 확인했다. Android OS compositor 이후 검증도 border stroke 검증도 아니라고 범위를 제한했다. 통과.

## 검증 결과

- Rust layout oracle 1/1 및 FFI fixture source 검사 1/1 통과.
- workspace Clippy와 `c04-runtime-gpu-fixture-test` 포함 FFI Clippy 통과.
- Android fixture APK 빌드 및 연결된 SM-S731N 네 장면 모두 layout assertion과 draw/present 통과.
- 제품 `c04-runtime-gpu` 정적 라이브러리에서는 새 scene ID가 없고, opt-in fixture-test 정적 라이브러리에는 포함되는 것을 확인했다.
- 처음에는 Chrome decimal 문자열과 runtime f32 문자열을 strict string equality로 비교해 혼합 span 두 장면을 거부했다. 수치 허용치 비교로 바꾸고 전체 네 장면을 최종 APK로 재실행했다. 최초 assertion 거부 원본 로그도 evidence 디렉터리에 보존했다.

이 검토는 네 고정 scene의 제한 구현만 대상으로 하며 C11.4 전체 완료, WPT 적합성, Android OS compositor 이후 결과를 주장하지 않는다.
