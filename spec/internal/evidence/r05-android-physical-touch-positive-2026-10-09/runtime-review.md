# 런타임 증거 적대 검토

이번 실행에서 증거를 오인할 수 있는 서로 다른 실패 경계를 대조했다. 직접 입력이 없는 상태를 pass나 latency로 승격하지 않았다.

| # | 공격 관점 | 대조 결과 |
|---:|---|---|
| 1 | 에뮬레이터 로그를 실기기 결과에 섞는다 | ADB 목록에는 USB 실기기 1대만 있었다. 기록에서 일련번호는 제거했다. |
| 2 | Android 버전 표기를 잘못 옮긴다 | Android 16 / API 36으로 표기하고 raw `SDK_INT_FULL`은 36.1로 분리했다. |
| 3 | 재연결 뒤 다른 input node를 읽는다 | `/dev/input/eventN`을 고정하지 않고 실행 때 `sec_touchscreen` 이름과 direct property를 다시 확인했다. |
| 4 | 터치패드를 touchscreen으로 오인한다 | `sec_touchscreen`의 `INPUT_PROP_DIRECT`를 확인했다. `sec_touchpad`는 입력 장치 목록에서 구분했다. |
| 5 | ADB 주입을 physical sample로 분류한다 | synthetic control은 app action 1건, raw touchscreen 0건으로 기록되어 direct contact와 분리됐다. |
| 6 | 빈 raw 파일을 성공으로 계산한다 | 32분 30초 수집 파일은 0바이트다. 이를 입력 수 0으로 기록했고 positive sample은 만들지 않았다. |
| 7 | touch 없는 app launch를 입력 경로 검증으로 부른다 | PID 29740 로그에는 surface 초기화만 있고 R05 `ACTION_UP`은 없다. 입력 연결 성공을 주장하지 않는다. |
| 8 | unrelated logcat line을 앱 입력으로 착각한다 | app log는 PID 29740으로 수집했고 raw 장치 로그를 별도 파일로 보존했다. |
| 9 | 대조군의 signal을 physical input에 귀속한다 | control PID 29454 기록은 control 폴더에만 두고 physical 표본과 합산하지 않았다. |
| 10 | `TransactionStats` callback을 화면 scanout으로 확대 해석한다 | callback/fence는 OS signal 경계까지만 표시했다. 패널 scanout이나 photon 시각은 주장하지 않는다. |
| 11 | `target_vsync_id=-1`을 유효 VSync로 취급한다 | `-1`을 그대로 보존했고 VSync 기반 latency나 frame 상관을 계산하지 않았다. |
| 12 | 미측정 latency를 인접 timestamp로 보간한다 | physical touch가 없으므로 latency 계산 파일과 수치가 없다. |
| 13 | 다른 V8 소스 revision으로 Release를 빌드한다 | checkout HEAD와 `tools/v8/v8-revision.txt`가 `7b50b62cb18f28617959e8452e2cd18195b38bcf`로 일치했다. |
| 14 | NDK가 바뀐 빌드를 고정 toolchain 결과로 설명한다 | 빌드 로그의 사용 NDK `27.1.12297006`을 기록했다. 환경변수의 다른 NDK 경로는 helper가 무시했다. |
| 15 | 임시 GN 변경이 checkout에 남는다 | build script 종료 뒤 `BUILDCONFIG.gn` diff가 없고 임시 NDK 링크가 제거됐다. |
| 16 | V8 archive 생성만으로 Android Release 성공을 주장한다 | Gradle `:app:assembleRelease`가 실행되어 `BUILD SUCCESSFUL`을 확인했다. |
| 17 | unsigned APK를 설치 가능한 Release 산출물로 부른다 | 원본은 unsigned라고 명시하고 SHA-256을 기록했다. 설치에는 임시 서명 copy만 사용했다. |
| 18 | 다른 signing key로 현재 앱을 덮어쓴다 | 설치 전 release test copy와 현재 APK의 signer SHA-256 지문이 같은지 확인한 뒤 `adb install -r`를 사용했다. |
| 19 | debug-only flag가 Release에서 실행된다고 오인한다 | Release에서 동일 extra를 보냈고 `UNAVAILABLE reason=debug_only`를 관측했다. |
| 20 | Release 기본 빈 화면을 GPU 렌더 성공으로 표시한다 | bootstrap 결과와 빈 화면 screenshot을 같이 보존하고 GPU visual pass가 아니라고 구분했다. |
| 21 | smoke 후 다른 APK가 남는다 | 기존 debug APK를 재설치했고 설치본 SHA-256이 시작 전 고정 artifact와 다시 일치했다. |
| 22 | 테스트가 화면 설정을 바꾸고 되돌리지 않는다 | 밝기·timeout·충전 중 화면 켜짐·활성 mode/주사율이 전후 동일했다. |
| 23 | 테스트 종료 뒤 앱이 foreground에 남는다 | Chrome을 foreground로 복귀시킨 것을 window focus로 확인했다. |
| 24 | 작은 표본으로 성능 일반화를 한다 | physical sample이 0이고 synthetic control도 한 건뿐이다. p95·속도 우위·제품 지연 주장을 하지 않는다. |
| 25 | debug keystore smoke를 production signing으로 과장한다 | production signing과 production 배포 경로는 미검증으로 남겼다. |

주요 확인 근거는 [README.md](README.md), [release-build.log](release-build.log), [raw touchscreen](physical/raw-touchscreen.txt), [physical app log](physical/app-logcat.txt), [Release default log](physical/release-default.log), [Release debug-only rejection](physical/release-debug-only-rejection.log), [input device capabilities](input-devices.txt)다.
