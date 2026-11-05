# 실패한 실행과 수정 기록

성공한 run만 남겨 실패 원인을 지우지 않도록 실행 중 발견한 세 실패 경로를 기록한다.

## Android run `c11rmx-20261105`

- HTML에 viewport meta가 없어서 기본 Root HTML의 layout viewport는 `980×2004.4629`; visible viewport는 `411×842`였다.
- 첫 PixelCopy 구현은 sample CSS 좌표를 visible viewport에 매핑해 화면의 더 오른쪽·아래쪽 흰 곳을 읽었다. `min-width-overflow-inside-track`와 `min-width-overflow-outside-grid`는 흰색 대 파란색으로 불일치했고, item 바깥 흰색 표본은 일치했다.
- [실패 run logcat](android/failed-run-logcat.log), [실패 PixelCopy PNG](android/failed-pixelcopy.png)을 보존했다. PNG에 보이는 실제 콘텐츠와 세 표본의 위치도 대조했다.
- 수정: Android 표본 mapping에 980px layout viewport를 사용하고 높이는 surface 비율에서 계산했다. 이 결과 r3 표본의 PixelCopy 좌표는 `(72,237)`, `(83,237)`, `(94,237)`이고 3/3 exact다.

## Android Java 컴파일 실패

두 번째 빌드에서 `Math.round(double)`의 `long` 반환값을 `int`에 암시 대입해 `possible lossy conversion from long to int`가 발생했다. `(int) Math.round(...)`로 변환을 명시했다. 이후 feature-enabled Android APK build와 PixelCopy 실행이 통과했다.

## iOS run `c11rmx-20261105-r2`

- UIKit visible viewport는 `402×778`이었지만 Root HTML layout viewport는 `980×1896.617`, drawable은 `1206×2334`였다.
- 첫 Metal 변환은 visible viewport의 3×를 적용해 두 파란 표본을 흰색으로 읽었고, 바깥 흰색 대조는 통과했다. 실제 파란 픽셀은 `(80,265)`와 `(92,265)` 부근이었다.
- 원래 run log는 다음 실행이 경로를 덮어써 전체 파일을 보존하지 못했다. 관찰된 표본과 판정은 여기 고정한다.
- 수정: `C111RMX` 전용 sampling viewport를 layout 폭 980px 및 drawable 비율 기반 높이로 계산했다. r3에서 scale X/Y가 `1.2306123`으로 같고 3/3 exact다. 최신 전체 로그는 [iOS r3 run](ios/aspect-ratio-min-max/run.log)에 있다.

## 최종 확인

마지막 [Android](android/aspect-ratio-min-max/run.log)와 [iOS](ios/aspect-ratio-min-max/run.log)는 같은 fixture SHA `4ddb06ca0d0915215c7aa3ca9e6a49a39d558771325fed3603f3e7461ceaf08c`를 사용한다(각 [Android hash 기록](android/aspect-ratio-min-max/artifact-hashes.txt), [iOS hash 기록](ios/aspect-ratio-min-max/artifact-hashes.txt)). 양 플랫폼 앱을 stop/terminate했고, bootstrap 소스 원본 해시 복원을 검증했다.
