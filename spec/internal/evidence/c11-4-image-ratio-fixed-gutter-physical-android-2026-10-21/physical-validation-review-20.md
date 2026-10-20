# Android fixed-gutter 실기기 검증 실패 관점 20개

이 검토는 capture script와 실제 Samsung SM-S731N 실행 증거를 대상으로 한다. 기존 구현 검토 20개와 PR diff 검토 30개를 대체하지 않는다.

1. **다른 serial 연결** — CLI serial이 없거나 패턴이 잘못되면 시작 전에 거부한다. 실행 serial `R5KYB06SMPE`는 `adb devices -l`의 단일 `device` 행과 일치했다.
2. **AVD를 실기기로 오인** — `ro.kernel.qemu=1`이면 거부한다. 이번 기기는 `qemu=0`이었다.
3. **다른 모델 연결** — 모델을 `SM-S731N`으로 고정 검사한다. 실행 기기는 일치했다.
4. **다른 API에서만 통과** — Android API 36인지 검사한다. 기기는 Android 16/API 36이었다.
5. **32-bit 또는 잘못된 ABI** — `arm64-v8a`를 요구한다. 연결 기기는 ARM64였다.
6. **다른 GPU 경로로 바뀜** — SurfaceFlinger `GLES:` renderer에 Xclipse 940과 Vulkan이 있는지 검사한다. 관찰값은 Xclipse 940/Vulkan 1.3.279/ANGLE이었다.
7. **debug APK가 없음** — 로컬 APK를 읽기 전에 파일 존재 여부가 OS 오류로 드러난다. 실행에 사용한 APK 경로는 `platforms/android/app/build/outputs/apk/debug/app-debug.apk`였다.
8. **split install에서 base APK를 잘못 선택** — 설치 경로 중 `/base.apk`를 찾고 그 바이트를 검증한다. 이번 패키지에서 base APK가 확인됐다.
9. **기기 설치본이 오래됨** — local/install SHA가 다르면 재설치하고 다시 해시를 비교한다. 이번 local/install 값은 모두 `700db402…f8e2439c`로 일치해 재설치가 필요 없었다.
10. **요청 외 scene 실행** — scene ID는 두 fixed-gutter case로 코드에 고정돼 있고, Android host도 C11.4 allowlist를 검사한다. column과 row 각 1회만 실행했다.
11. **다른 장면의 run ID 로그가 섞임** — scene마다 별도 run ID로 필터링한다. 저장된 로그의 모든 C11.4 summary·PixelCopy marker에 해당 scene run ID가 포함됐다.
12. **column frame만 맞고 row가 틀림** — 두 방향의 root/item frame을 별도로 판정한다. column 12×6/36×24px, row 6×12/54×36px가 각각 status 0으로 기록됐다.
13. **trace가 없는데 frame만 통과** — Android host는 trace chunk 확인 실패 시 scene 성공 summary를 기록하지 않는다. 두 scene 모두 trace chunk와 status 0 summary가 있다.
14. **span 내부 gutter를 빼거나 중복 합산** — trace의 fixed track, gutter, limit을 구분한다. 두 장면 모두 7px + 3px = 10px를 기록했다.
15. **다른 contribution source 또는 span** — 원본 trace의 contribution은 `PreferredSize`, span 2이며 limit 10px이다. 두 scene 로그에서 이 값을 확인했다.
16. **화면 표본 좌표/대상이 바뀜** — 앱 validator가 각 scene의 border/item/right/below 표본 node ID와 CSS 좌표를 고정한다. PixelCopy 로그의 대상 ID와 좌표가 기대값에 맞았다.
17. **표본은 수집됐지만 색이 다름** — 각 표본의 RGBA가 기대값과 일치할 때만 `status=matched`가 된다. column 및 row 모두 4/4 exact였다.
18. **스크린샷 저장 실패** — 기기 screenshot이 비어 있으면 capture script가 중단된다. 두 1080×2340 PNG가 저장되고 로컬에서 화면을 확인했다.
19. **PixelCopy 표면 파일이 빠짐** — 앱 cache에서 scene/run ID에 맞는 SurfaceView PNG를 가져오며 파일이 비면 실패한다. 두 900×675 PNG가 저장됐다.
20. **앱 종료 뒤 결과 화면을 놓침** — 중간 scene만 force-stop하며 마지막 row scene은 기기에서 열어 둔다. 실행 뒤 row 결과 화면이 연결된 기기에 남았다.

### 한계 확인

이번 capture는 기존 해시 고정 debug APK를 사용했고 새 APK build를 수행하지 않았다. surface 표본은 지정 위치의 SurfaceView 색을 확인하며 PNG texel과 compositor 이후 화면은 검증하지 않는다. 한 기기의 보충 검증이므로 다른 Android 기기나 일반 CSS Grid 동작으로 확장하지 않는다.
