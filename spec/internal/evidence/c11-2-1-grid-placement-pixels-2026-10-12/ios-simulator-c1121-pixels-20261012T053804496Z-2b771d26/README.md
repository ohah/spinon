# iOS Metal 표본 실행 · 호스트 선택 순서 결함

이 실행은 픽셀 결과로 채택하지 않았다. `--spinon-c1121-grid-placement`와 C11 Metal 표본 인수를 함께 전달했을 때 Swift host selector가 C11.1 표면 표본 host를 먼저 선택했다. 그 host에 C11.2.1 placement fixture를 평가해 CSS runtime 결과가 `status=-11`로 실패했다.

수정은 두 부분이다.

1. iOS host 선택에서 C11.2.1 profile을 C11.1 pixel host보다 먼저 선택한다.
2. 내부 C11 Metal 표본 test feature가 켜진 빌드에서 C11.2.1 host에도 해당 fixture의 surface pixel readback을 허용한다. 기본 빌드에는 이 capability가 들어가지 않는다.

이 run의 표본은 수집되지 않았고 비교 결과를 만들지 않았다. [원본 unified log](ios-log.json) · [빌드 로그](build.log) · [launch 로그](launch.log)
