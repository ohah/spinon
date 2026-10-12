# C11.2.1 Android 환경 갱신 경합 후속 근거

## 발견한 동작

Android 실기기에서 `positive-spans`를 처음 실행했을 때 초기 평가가 `status=-12`로 끝났다. 로그의 내부 작업은 `op=eval status=0`이며 응답은 더 새 화면 요청이 도착해 이전 장면을 거부했다고 설명한다. 그 뒤 최신 환경 갱신은 layout과 WGPU 장면을 준비했지만, 초기 `-12` 응답을 최종 실패 요약으로 기록한 탓에 수집기는 성공 장면의 최종 summary를 찾지 못했다.

- 실패 재현: [초기 실기기 로그](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T171745152Z-5125eb50/positive-spans-runtime.log)
- 재현 run ID: `c1121-android-20261012T171745152Z-5125eb50-positive-spans`

## 수정 및 회귀 방지

Android 런타임은 `status=-12`이면서 내부 `op=eval status=0`인 경우에만 이를 대체된 초기 장면으로 분류하고 `SPINON_C1121_GRID_SUPERSEDED`를 남긴다. 이 응답은 최종 실패 summary나 실패 진단으로 처리하지 않는다. 최신 환경 갱신은 현재 surface generation·presentation sequence를 다시 확인한 뒤 `status=0` 및 `layout=ready`인 결과만 단일 성공 summary와 frame·paint-order 기록으로 내보낸다.

비교기는 초기 eval과 최종 summary의 상태를 대조한다. 초기 eval이 `-12`라면 동일 run·scene의 superseded marker와 내부 eval 성공 상태, summary보다 앞선 기록 순서를 요구한다. 다른 초기 오류가 성공 summary와 함께 들어와도 통과하지 않는다. ready summary 누락, 예상 거부 장면의 상태 불일치도 실패한다.

- [비교기 회귀 검사](../../../../tools/css-reference/c11-2-1-grid-placement-runtime-compare.test.mjs)
- [비교기 구현](../../../../tools/css-reference/c11-2-1-grid-placement-runtime-compare.mjs)
- [Android 성공-summary 경계](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java)
- 검증: 집중 비교기 검사 5/5, CSS reference 전체 342/342 통과.

## 수정 후 모바일 실행

| 대상 | run ID | 장면·노드 | geometry | surface 표본 |
|---|---|---:|---:|---:|
| Android 16/API 36 SM-S731N 실기기 · Xclipse 940/Vulkan 경로 | `c1121-android-20261012T172006650Z-11559104` | 10 scene · 21 node · 성공 6/6 · 기대 거부 4/4 | 최대 오차 0 CSS px | 20/20 exact · 최대 채널 오차 0 |
| iPhone 17 Pro · iOS 26.2 Simulator · Metal drawable | `c1121-iossim-20261012T172100424Z-9872aaa2` | 10 scene · 21 node · 성공 6/6 · 기대 거부 4/4 | 최대 오차 0 CSS px | 20/20 exact · 최대 채널 오차 0 |

- [Android 실기기 결과](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T172006650Z-11559104/README.md) · [세부 비교](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T172006650Z-11559104/comparison.json)
- [iOS Simulator 결과](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T172100424Z-9872aaa2/README.md) · [세부 비교](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T172100424Z-9872aaa2/comparison.json)

수정 후 10-scene 재실행에서는 `-12` 분기가 다시 발생하지 않았다. 따라서 수정 후 실기기 로그가 해당 분기를 직접 밟았다고 주장하지 않는다. 이전 로그가 경합의 원인을 재현하고, 비교기 회귀 검사는 superseded·정상 ready·누락 marker·무관한 실패 상태를 구분한다. Android의 희귀한 시작 경합을 재실행에서 재현하지 못한 점은 남은 증거 경계다.

이 결과는 제한 C11.2.1 profile에 대한 실행 근거다. Spinon WPT 적합성, iOS 실기기, 전체 CSS Grid, C11 통합·완료를 증명하지 않는다. C11.1 및 C11.2.1은 아직 통합되지 않았다.
