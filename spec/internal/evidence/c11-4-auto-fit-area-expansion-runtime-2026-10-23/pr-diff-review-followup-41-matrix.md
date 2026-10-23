# PR diff follow-up · 정확한 41-scene 실행 집합

**추가 검토 관점:** 7개. 기존 30개 diff 관점 검토 뒤 41-scene manifest·route guard, 고정 simulator target, Android host Java typecheck를 별도로 확인했다.

| 실패 관점 | 검증과 판정 |
| --- | --- |
| no-area oracle 12개 또는 area fixture 29개 중 하나가 capture route에서 누락된다. | Node test가 사전 oracle과 current JSON을 41-ID manifest에 대조하고 `--list-scenes`에 전부 존재하는지 검사한다. 7/7 통과. |
| matrix manifest가 41개보다 적거나 중복 ID를 가진다. | test가 total과 unique count 모두 41인지 검사한다. 통과. |
| precomparison oracle ID가 바뀌거나 current area fixture ID와 manifest가 엇갈린다. | manifest의 두 ID 배열을 각각 authoritative source JSON과 deep-equal 비교한다. 통과. |
| catalog 검사가 실제 AVD를 실행하거나 evidence 디렉터리를 만든다. | `--list-scenes` 분기가 `mkdir`, ADB, simulator command보다 앞에 있는지 source에서 확인했다. Node test가 `--scene-id`와 목록 모드로 선택 41개를 exact 비교했고, 기기 실행 없이 종료했다. 통과. |
| aggregate count가 area fixture count와 혼동돼 336 guard가 빠진다. | 29-area fixture count와 336 full-route count를 별도 검사한다. Node test 7/7 통과. |
| 고정 simulator target이 다른 AVD·iOS 기기를 가리킨다. | `emulator-5558`에서 API 37/arm64-v8a와 AVD 이름, iPhone 17 Pro UDID와 booted 상태를 read-only 조회했다. 테스트 실행이나 기기 상태 변경은 하지 않았다. |
| Android host의 새 scene switch가 Java 타입·문법 오류를 포함한다. | 변경된 Java source를 JDK 17, API 37.2 `android.jar`, 기존 app class output과 함께 `javac -proc:none -source 17 -target 17`로 컴파일했다. 성공했다. 이는 Gradle app build나 최신 Rust/native 연결 검증이 아니다. |

이 검사는 route와 ID membership만 검증한다. Android/iOS 앱 build, 41-scene simulator 실행, frame/trace/draw/present 또는 pixel 표본을 검증하지 않는다.
