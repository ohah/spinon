# C11.4 auto-repeat `calc()` neighbor 현재 소스 캡처 중단 · 2026-10-24

## 실행 범위와 결과

현재 작업 트리의 typed projection 경계 변경을 포함한 iOS 26.2 Simulator 실행이다. 대상은 iPhone 17 Pro Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`이며, 첫 positive scene `calc-neighbor-col-fill-both`에서 runner가 중단됐다.

- 앱 build와 install은 완료됐다.
- 앱은 첫 scene을 실행해 `status=0`, `layout=ready`, `boxes=2`를 보고했다. 이는 해당 scene의 layout 계산 결과이며 화면 표시나 surface 검증을 뜻하지 않는다.
- 후속 Metal 표면 캡처 요청은 `status=-90 C11.4 Metal surface pixel fixture is not enabled`로 끝났다.
- capture runner가 성공 scene의 presented marker와 surface sample을 요구하므로, 첫 scene validator가 실패하고 matrix가 중단됐다.

원본은 [install log](./ios-simulator-c114-calc-neighbor-projection-boundary-ios-simulator-20261024T035213164Z-43d74ffc/install.log), [launch log](./ios-simulator-c114-calc-neighbor-projection-boundary-ios-simulator-20261024T035213164Z-43d74ffc/launch-calc-neighbor-col-fill-both.log), [runtime log](./ios-simulator-c114-calc-neighbor-projection-boundary-ios-simulator-20261024T035213164Z-43d74ffc/runtime-calc-neighbor-col-fill-both.log)에 보존했다.

## 판정

이 실행은 현재 소스의 양 플랫폼 positive matrix, `draw/present`, Metal 표본 또는 앱 negative-control 계약을 검증하지 않는다. 특히 뒤의 세 negative scene은 실행하지 않았다. 런타임 log의 layout-ready 결과만으로 scene 통과를 주장하지 않는다.

오류 메시지는 이 빌드에서 C11 Metal surface pixel fixture가 활성화되지 않았음을 나타낸다. 따라서 실패 지점은 CSS layout 계산 다음의 capture gate다. 이 결과만으로 typed projection 변경의 앱 runtime 성공·실패를 판정하지 않는다.

이전 [11-scene runtime 기록](../c11-4-auto-repeat-calc-neighbors-runtime-2026-10-24/README.md)은 projection-boundary 수정 전 작업 트리의 결과다. 현재 소스 검증으로 재사용하지 않는다. fixture-test build flag를 켠 후속 실행은 Android와 iOS에서 각각 14/14를 통과했다. [최신 실행 기록](../c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md). C11.4는 미완료이며 공식 `72/213` 집계도 바뀌지 않는다.
