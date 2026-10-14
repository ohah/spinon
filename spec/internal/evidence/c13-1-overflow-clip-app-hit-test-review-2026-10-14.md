# C13.1 Android 앱 hit-test delta 실패 관점 20개 검토 · 2026-10-14

검토 대상은 C13.1 opt-in Android app fixture가 실제 게시할 `RuntimeRenderSnapshot`에서 Chrome observation의 고정 hit-test 표본을 평가하고 JNI logcat에 기록하는 경로다. 실행 증거는 [Android API 37 ARM64 AVD](c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md), 기준 oracle은 [Chrome observation JSON](c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json)이다. 이 검토는 DOM event dispatch, 일반 입력 연결 또는 C13.1 완료 판정이 아니다.

| # | 실패 관점 | 적대적 확인과 결과 |
| --- | --- | --- |
| 1 | 초기 환경 갱신 시점에 빈 scene hit를 실패 처리 | 실제 AVD에서 JS 평가 전 `layout=empty`, `boxes=0` 환경 갱신이 발생함을 재현했다. query를 fixture eval 이후로 제한해 정상 통과시켰다. |
| 2 | C13.1 외 runtime eval이 fixture oracle에 묶임 | hit report enable flag는 C13.1 fixture host에서만 켠다. environment update와 다른 host는 query를 호출하지 않는다. |
| 3 | feature-off Rust build에 Chrome observation payload 포함 | oracle 상수·함수·host flag는 `c13-1-overflow-clip-fixture-test` cfg 안에 둔다. |
| 4 | feature-off Android library에 C13 logger 유입 | JNI per-sample helper definition과 호출을 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST` compile guard 안에 둔다. |
| 5 | host app이 eval 전에 runtime environment를 정상 설정하지 못함 | 빈 scene 초기 환경과 fixture 13-node layout을 별도 app log state로 확인한다. 최종 run의 environment status는 성공이다. |
| 6 | browser oracle 버전이 바뀌어 기대 node가 stale | fixture query는 Chrome `155.0.8059.40` JSON version만 허용하고 다른 버전이면 fail-closed한다. |
| 7 | observation JSON에서 frame array가 빠져 index를 임의로 계산 | frame inventory가 없으면 fixture eval이 오류를 반환한다. |
| 8 | hit-test sample inventory가 비거나 일부 누락 | 고정 oracle가 11 sample인지 검사하며 AVD에서 11개 로그를 각각 확인했다. |
| 9 | sample ID 누락으로 잘못된 점을 건너뜀 | ID를 읽지 못하면 명시 오류로 중단한다. |
| 10 | 좌표가 누락되어 기본점이나 0을 사용 | x/y를 필수 JSON number로 읽고 누락 시 오류 반환한다. |
| 11 | Chrome target ID가 frame inventory 밖에 있음 | target ID를 frame에서 못 찾으면 sample별 오류를 반환한다. |
| 12 | hit query가 `None`인데 성공으로 기록 | 각 고정점에서 node를 못 찾으면 fixture eval이 실패한다. |
| 13 | 반환 NodeId가 layout frame과 연결되지 않음 | frame inventory에 없는 runtime NodeId면 실패한다. |
| 14 | 실제 반환 node가 Chrome target과 다름 | actual frame index와 oracle frame index를 비교한다. 하나라도 다르면 전체 eval을 실패시킨다. |
| 15 | point·target 표본 순서가 서로 달라 잘못 대조 | Chrome JSON 배열 순서대로 평가하고 Android 표본 id·CSS 좌표를 같은 순서로 대조해 11/11 일치했다. |
| 16 | JSON 직렬화 실패를 빈 report로 성공 처리 | serialization 오류는 `RuntimeGpuHost::wait_and_publish`에서 `ERR_LAYOUT`으로 전달한다. |
| 17 | 초기 C131 eval log가 긴 report 제한으로 표본을 잃음 | 첫 앱 log에서 `SPINON_C131_EVAL` 한 줄이 잘리는 것을 확인했다. JNI가 각 JSON object를 독립 `SPINON_C131_HIT_TEST` row로 출력하도록 바꿨다. |
| 18 | Android log row가 여러 표본을 합쳐 다시 잘림 | final AVD log의 11개 JSON row는 각각 완전하며 개별 `JSON.parse` 후 oracle 비교에 성공했다. |
| 19 | 이전 앱 PID의 잔여 log가 현재 실행의 근거로 섞임 | 설치 후 app PID `6671`에 한정해 logcat을 추출하고 run ID `b96d0f0b00f`의 PixelCopy 결과와 연결했다. |
| 20 | query 증거를 DOM event 또는 paint 성공으로 확대 해석 | JSON은 `RuntimeRenderSnapshot::hit_test_css_point()` 반환 node만 기록한다. DOM listener dispatch는 호출하지 않으며 PixelCopy는 별도 paint 표본이다. |

최종 검증은 Android API 37 ARM64 AVD run `b96d0f0b00f`: Chrome point hit 11/11, PixelCopy RGBA 11/11 exact, `layout=ready`, `boxes=13`이다. `cargo test --locked --workspace`, feature-on `spinon-ffi` tests, workspace 및 feature-on FFI Clippy, rustfmt, `git diff --check`, fixture module boundary test 5/5가 통과했다. 전체 C13.1 oracle matrix, iOS Simulator 통합과 DOM events는 미완료다.
