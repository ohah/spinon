# C13.1 전체 표면 pixel-center PR 변경 검토 · 2026-10-15

PR #170의 코드, 실행기, oracle 비교, 플랫폼 빌드 설정, 실제 실행 근거와 GitHub 본문을 실패 관점으로 각각 확인했다. 작업 단위 구현 검토와 겹치지 않도록 PR 경계·재현성도 별도로 살폈다. 발견한 실행기 오류와 수정은 ADB 근거에 남겼고 Android capture 도구의 해시를 실행 산출물에도 고정했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 내부 fixture 검증이 C13.1 전체 제품 지원으로 게시된다 | `spec/STATUS.md`는 C13.1을 미완료로 유지한다. PR 본문·통합 README도 compositor, 실기기, WPT와 제품 지원을 미확인으로 제한한다. **통과.** |
| 2 | Metal readback FFI가 기본 Rust archive에 들어간다 | 기본 archive에서 두 readback symbol이 없고 fixture feature positive control에서만 생기는 LLVM 22 `llvm-nm` 로그를 확인했다. **통과.** |
| 3 | C 헤더가 fixture flag 없이 실험 ABI를 선언한다 | C13.1 host/readback 선언이 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST` 조건 안에 있다. **통과.** |
| 4 | iOS와 Android build flag가 Rust feature와 어긋난다 | 저장된 양쪽 build log에서 C04 runtime GPU와 C13.1 fixture-test flag가 함께 켜진 실행을 확인했다. **통과.** |
| 5 | Metal 전용 인수가 기본 화면으로 빠진다 | AppDelegate route 목록에 pixel-center 인수가 포함되고, 단독 route negative control은 host 생성 전에 명시적으로 거부된다. **통과.** |
| 6 | pixel-center route가 필요한 hit-identity scene 없이 실행된다 | controller의 사전 조건 검사와 `requires-compact-hit-identity-grid` 로그로 host 생성 전 차단된다. **통과.** |
| 7 | 두 fixture route가 겹쳐 서로 다른 oracle을 적용한다 | matrix route와 pixel route 충돌 negative control이 `multiple-fixture-routes`로 host 생성 전에 거부된다. **통과.** |
| 8 | 첫 layout callback이 fixture 초기화 중 UIKit presentation update를 연다 | 실제 iOS 초기 `status=-12`를 근거로 초기화 완료 guard를 넣었고, 이후 fresh process 3회가 eval `status=0`이다. **수정 후 통과.** |
| 9 | artifact 초기화 오류가 만들어 둔 runtime host를 누수시킨다 | Swift capture 생성 실패 경로에서 `freeRuntimeGpuHost` 뒤 오류를 게시하는 것을 확인했다. **코드 경로 확인.** |
| 10 | FFI 표본 수 곱셈 overflow 또는 과대 batch가 포인터 읽기를 앞선다 | `checked_mul`, 1–64 범위, null pointer와 출력 buffer 검사가 slice 생성보다 먼저 있으며 focused FFI tests 2개가 이를 거부했다. **통과.** |
| 11 | 호출자 좌표 배열 길이·수명·할당 실패가 잘못 처리된다 | C Safety 계약에 `2 × sample_count`와 호출 중 수명을 적고, `try_reserve_exact` 실패는 상태 코드로 반환한다. **통과.** |
| 12 | 이전 batch가 끝나기 전에 다른 좌표를 예약한다 | capture state는 outstanding batch가 있으면 다음 예약을 거부하며 draw report를 수락한 뒤 다음 batch를 요청한다. 469 batch 로그가 순차다. **통과.** |
| 13 | 화면 x/y scale이 다를 때 한 축의 배율을 다른 축에 재사용한다 | iOS manifest와 Android PNG comparator가 x/y scale을 분리한다. AVD PixelCopy scale은 `2.6266… × 2.63`이고 30,000점 비교가 exact다. **통과.** |
| 14 | viewport 가장자리나 마지막 행이 빠져 일부 표면만 비교한다 | CSS `(0,0)`부터 `(299,99)`까지 row-major 30,000행과 마지막 48개 batch를 확인했고 결과는 120,000 channel이다. **통과.** |
| 15 | batch 경계에서 표본을 누락·중복·재정렬한다 | 468×64 + 48, 합계 469 batch이며 comparator는 누락·중복·순서 변경 mutation을 거부한다. **통과.** |
| 16 | drawable이 resize되거나 format이 바뀐 뒤 표본을 한 세대로 합친다 | iOS capture는 generation·크기·8-bit format을 고정하고 draw report와 매번 대조한다. 세 실행 모두 `900×300`, 동일 generation, `Bgra8UnormSrgb`다. **통과.** |
| 17 | report 좌표나 RGBA 값이 예약과 다르거나 JSON 타입이 잘못돼도 저장된다 | Swift 수락 경로가 정확한 키·좌표·정수 channel·0–255 범위를 확인하고 comparator가 같은 제약을 다시 적용한다. **통과.** |
| 18 | 중간 실패로 일부 JSONL이 완성 표본처럼 남는다 | capture는 모든 표본 완료 뒤 manifest를 쓰며 오류·deinit 경로가 `.part`와 최종 경로를 제거한다. 비교기는 `status=complete`와 30,000행을 요구한다. **통과.** |
| 19 | 실행 사이 표본 파일을 섞거나 내용이 바뀌어도 성공으로 처리한다 | 각 행·manifest의 run ID와 generation, 표본 SHA-256, process ID를 비교하고 변경 hash·다른 run·generation mutation을 거부한다. **통과.** |
| 20 | Simulator 재시작이 없어 반복 결과가 같은 앱 process에서 나온다 | iOS capture는 각 실행 전에 앱을 종료하고 PID가 새 값인지 확인한다. 보존된 세 PID는 `72208`, `74726`, `77216`이다. **통과.** |
| 21 | 다른 iOS 기기·runtime을 고정 Simulator 근거로 기록한다 | 실행기는 iPhone 17 Pro UDID와 iOS 26.2 runtime을 강제 확인하며 저장된 run README가 이를 기록한다. **통과.** |
| 22 | ADB 실행이 실기기나 다른 Android 이미지에서 우연히 통과한다 | runner는 emulator serial, `qemu=1`, API 37, ARM64, SwiftShader를 확인한다. 대상은 `emulator-5554` AVD다. **통과.** |
| 23 | 비동기 `am start` 직후 아직 없는 PID 때문에 앱 실행을 실패 처리한다 | 재현한 ADB 원인을 반영해 기존 process 종료를 기다리고 launch 뒤 새 PID를 poll하도록 runner를 수정했다. 최종 AVD run은 완료됐다. **수정 후 통과.** |
| 24 | JNI와 Java의 동일 frame log 중복이 누락·과한 frame으로 판정된다 | runner는 node index로 41개 고유 payload를 모으고 중복 payload 불일치를 거부한다. 최종 증거는 41개 frame이다. **수정 후 통과.** |
| 25 | eval 필드 순서 또는 앱 process 외 log가 실행 성공을 만든다 | ADB parser는 PID로 logcat을 제한하고 `status=0`과 별도 `layout=ready`를 함께 확인하며 run ID가 hit grid·PixelCopy와 맞아야 한다. **통과.** |
| 26 | 앱이 기록한 PixelCopy와 실제 추출 PNG가 다르다 | `run-as`로 추출한 PNG SHA-256을 앱 로그값과 대조한다. 저장된 PixelCopy digest는 `09ab4348…678ab3ab`다. **통과.** |
| 27 | AVD capture 도구를 실행 근거에서 찾을 수 없다 | APK app source revision과 별도로 capture 도구 경로·SHA-256을 요약 JSON과 `artifacts.sha256`에 추가했다. **검토 중 보강.** |
| 28 | Chrome 기준 이미지나 browser revision이 바뀌었는데 기존 결과를 재사용한다 | comparator가 observation의 Chrome 155 버전·PNG SHA를 검증하고 결과에 고정한다. Android·iOS 모두 같은 pinned oracle을 사용한다. **통과.** |
| 29 | 부모 border 밖 overflow가 정상 색으로 보이는지 검사하지 않는다 | `(270,72)`는 parent right edge x=264 밖이며 두 플랫폼의 30,000점 scan에 포함된다. Chrome RGBA `[227,84,78,255]`에서 R/G/B/A 1-byte mutation 모두 실패한다. **통과.** |
| 30 | PR 화면 링크가 GitHub 본문에서 실제 이미지로 표시되지 않는다 | agent-browser로 PR #170을 열었다. iPhone 17 Pro, Android AVD, PixelCopy 이미지 3장이 모두 로드되고 표시되는 것을 확인했다. **통과.** |

## 검증 경계

- iOS 근거는 iPhone 17 Pro / iOS 26.2 Simulator의 compositor 제출 전 Metal drawable 표본 3회다.
- Android 근거는 API 37 ARM64 AVD의 ANGLE/Vulkan/SwiftShader SurfaceView PixelCopy 1회다.
- 전체 physical drawable byte, compositor/scanout, 실기기·hardware GPU, 다른 크기·orientation, DOM event·CSSOM, WPT와 C13.1 전체 지원은 검증하지 않았다.
- GitHub PR에는 현재 check run이 없으며 main branch protection에도 필수 status context가 설정되어 있지 않다. PR은 mergeable로 표시된다.
