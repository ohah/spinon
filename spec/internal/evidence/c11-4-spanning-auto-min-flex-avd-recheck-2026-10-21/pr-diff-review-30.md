# PR diff 적대적 검토 30개

**범위:** PR #415의 Android API 37 AVD 보충 검증 자료, C11.4 계획·공식 상태·정적 roadmap의 변경분. 기능 코드는 수정하지 않았다. 아래 검토는 이번 PR diff에만 적용했고 이 검토 문서 자체는 대상으로 세지 않았다.

| # | 공격 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | oracle 값이 AVD 결과에 맞춰 사후 변경될 수 있음 | 48/48 columns와 100×30, 100×10 값은 기존 Chrome 155 계획에 고정돼 있다. 실행 결과는 이 값을 그대로 비교한다. 통과. |
| 2 | fixture topology가 다른 장면으로 바뀔 수 있음 | runner는 `spanning-auto-min-flex-tracks` ID만 받으며 기록된 scene도 같다. 통과. |
| 3 | 1-scene 결과를 전체 행렬 통과로 부풀릴 수 있음 | `sceneCount=1`, 결과 JSON과 문서 모두 한 장면 보충 실행으로 한정한다. 통과. |
| 4 | 물리 기기를 AVD로 잘못 표기할 수 있음 | target은 `android-avd`이고 ADB serial은 `emulator-5558`이다. 통과. |
| 5 | Android OS/API 버전을 잘못 기록할 수 있음 | 기기 메타데이터는 Android 17/API 37로 기록되고 fingerprint와 model이 함께 남는다. 통과. |
| 6 | 16 KB 호환 AVD와 일반 AVD를 혼동할 수 있음 | 선택한 대상은 `sdk_gphone64_arm64` ARM64 API 37 AVD이며 16 KB AVD라는 주장은 없다. 통과. |
| 7 | 이전 작업의 AVD를 현재 실행으로 오인할 수 있음 | evidence directory, result `runPrefix`, simulator metadata가 모두 이번 `20261021T091026603Z` 실행에 대응한다. 통과. |
| 8 | stale APK가 scene allowlist를 거부할 수 있음 | fixture feature를 켜고 APK build가 성공한 뒤 runner가 새 APK를 설치했다. 통과. |
| 9 | 다른 앱 package를 검사할 수 있음 | 설치 검증은 `dev.spinon.bootstrap`의 `base.apk` 경로를 기록한다. 통과. |
| 10 | 로컬 APK와 설치 APK가 달라질 수 있음 | SHA-256 두 값이 모두 `b8f02254392a35766ff83bc50ac54479a0b54de7a857582ea7eb177b2ca6f355`이고 `matched=true`다. 통과. |
| 11 | flex track 수치가 독립 oracle과 다를 수 있음 | 관찰된 base sizes는 `[48,48]`로 고정 기대와 같다. 통과. |
| 12 | root 크기를 item 크기로 혼동할 수 있음 | root frame은 `100×30 CSS px`로 별도 기록한다. 통과. |
| 13 | spanning item frame을 root와 같은 값으로 잘못 주장할 수 있음 | item frame은 `100×10 CSS px`로 별도 기록한다. 통과. |
| 14 | gap을 무시해 총 inline size가 어긋날 수 있음 | 두 48px track과 4px gap의 합은 100px이며 계획 입력과 일치한다. 통과. |
| 15 | track sizing trace가 없는데 계산을 단정할 수 있음 | runtime log와 result JSON이 6 trace chunks를 기록한다. 통과. |
| 16 | 계산은 됐지만 frame이 표시되지 않을 수 있음 | `drawPresented=true`를 실행 결과로 확인했다. 통과. |
| 17 | 재시도된 superseded 결과를 최종 결과로 쓸 수 있음 | `supersededRetries=0`이며 superseded log가 없다. 통과. |
| 18 | result JSON이 다른 run의 log를 요약할 수 있음 | summary의 run ID와 `runPrefix`가 같은 실행 timestamp/nonce를 가리킨다. 통과. |
| 19 | screenshot이 결과 JSON과 다른 실행에서 나올 수 있음 | PNG, runtime log, launch log, simulator metadata가 같은 run directory에 있다. 통과. |
| 20 | screenshot이 실제 화면을 보여주지 않을 수 있음 | 로컬 PNG를 열어 Grid box와 runtime 상태 표시를 직접 확인했다. 통과. |
| 21 | 화면을 보고 Ahem glyph paint 성공을 주장할 수 있음 | 보고서와 roadmap에 glyph paint는 미검증이라고 명시했다. 통과. |
| 22 | Chrome의 `scrollWidth=200`을 Spinon 화면 관찰로 오인할 수 있음 | 해당 값은 Chrome oracle로 한정하고 앱의 overflow pixel 증거로 사용하지 않는다. 통과. |
| 23 | screenshot만으로 surface pixel 정확도를 주장할 수 있음 | PixelCopy 표본은 수집하지 않았다고 명시했다. 통과. |
| 24 | 기존 iOS 결과를 이번 PR에서 다시 실행한 것처럼 보일 수 있음 | iOS Simulator는 PR #415의 별도 기존 근거로 링크하고 이번 대상은 Android AVD로 분리했다. 통과. |
| 25 | 기존 Samsung 실기기 결과를 AVD 결과와 합칠 수 있음 | 이전 실기기 근거를 별도 문서로 유지하며 이번 실행에는 물리 기기를 사용하지 않았다. 통과. |
| 26 | 보충 장면을 CSS Grid 전체 지원으로 확대 해석할 수 있음 | 범위 밖 항목에 WPT와 전체 C11.4를 남겼다. 통과. |
| 27 | roadmap note가 공식 항목을 완료로 올릴 수 있음 | C11.4 checkbox는 unchecked로 유지한다. 통과. |
| 28 | 공식 완료 개수를 증가시킬 수 있음 | STATUS와 roadmap 모두 `72/213`을 유지한다고 적었다. 통과. |
| 29 | API/인터페이스 의미가 바뀌었는데 명세가 빠질 수 있음 | Rust·FFI·공개 API를 수정하지 않고 관찰 자료와 링크만 추가했다. 통과. |
| 30 | 문서 링크와 full build 상태를 숨길 수 있음 | 실행 보고서·계획·STATUS·정적 roadmap 간 링크를 추가했다. full `docs:build`는 저장소의 dead-link 오류 695건으로 실패했고 새 evidence README 경로는 오류 목록에 남지 않았다. 빌드를 성공으로 표시하지 않는다. 통과. |

**판정:** 30개 관점을 각각 확인했다. 기능 구현 완료나 전체 문서 적합성은 주장하지 않는다.
