# iOS Metal 표본 첫 실행 · test flag 누락

이 실행은 픽셀 결과로 채택하지 않았다. 앱은 V8 장면을 실행하기 전에 내부 전용 Metal 표본 기능이 비활성이라고 보고했다. 빌드 명령에 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`을 전달하지 않은 runner 결함이었다.

- 앱 runtime log: `SPINON_C111_METAL_DISABLED rebuild with the internal C11 pixel test flag`
- 표면 표본은 수집되지 않았고, 비교 결과를 만들지 않았다.
- 빌드 변수를 수정한 뒤 새 run ID와 앱 hash로 다시 실행한다.
- [빌드 로그](build.log) · [launch 로그](launch.log) · [unified log](unified.log)
