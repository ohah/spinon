# C13.1 hit-test viewport negative controls 변경 검토 20개 · 2026-10-14

검토 대상은 pinned Chrome elementFromPoint()의 viewport 밖 null 네 점을 Rust host 및 opt-in Android app runtime query의 None과 비교하는 변경이다. 관찰값은 [Chrome observation](c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json)과 [agent-browser direct output](c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/agent-browser-negative-controls.json), 실제 AVD 실행은 [negative-control evidence](c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/README.md)에 기록했다. DOM event dispatch나 C13.1 완료 판정은 포함하지 않는다.

| # | 실패 관점 | 적대적 확인과 결과 |
| --- | --- | --- |
| 1 | x 음수를 0으로 clamp해 root가 target이 됨 | agent-browser (-1,20) 결과는 null; host와 Android query도 None이어야 통과한다. |
| 2 | y 음수를 0으로 clamp해 상단 요소가 target이 됨 | (100,-1)의 Chrome 결과와 양 runtime query가 모두 null이다. |
| 3 | 오른쪽 viewport edge를 inclusive로 처리 | x=320은 Chrome viewport 폭과 같고 elementFromPoint()가 null이다. Android viewport도 더 좁아 null이다. |
| 4 | 아래쪽 viewport edge를 inclusive로 처리 | y=120은 Chrome viewport 높이와 같으며 양 runtime 결과도 null이다. |
| 5 | 한 축만 viewport 밖일 때 다른 축 hit가 새어 나옴 | 네 표본은 각각 left/top/right/bottom 한 경계를 벗어난다. 각 반환값을 개별 비교한다. |
| 6 | null expected target을 누락된 oracle 데이터로 혼동 | FFI는 JSON null만 no-target으로 허용하고 문자열·null 외 타입은 오류로 거부한다. |
| 7 | 예상 target이 있는 표본에서 runtime None을 성공 처리 | 예상 frame index와 actual Option<usize>를 비교해 Some/None이 다르면 fixture eval이 실패한다. |
| 8 | 예상 null에서 root 또는 child node를 성공 처리 | 예상값이 None일 때 actual frame이 Some이면 동일 비교에서 실패한다. |
| 9 | 새 표본 누락을 전체 성공으로 처리 | oracle inventory는 정확히 15개인지 확인한다. 기존 11점과 네 null 점이 빠지면 fail-closed한다. |
| 10 | 브라우저 표본 순서와 Android log 순서가 달라짐 | Chrome JSON 순서대로 실행하고 15개별 log row를 같은 index·id·좌표와 비교했다. |
| 11 | 기존 positive hit coverage가 null 처리로 우회됨 | 기존 11 clip/margin/nested/escape sample은 여전히 frame target을 요구하며 11/11 통과했다. |
| 12 | negative sample이 fixture frame inventory에 없는 임의 node를 가리킴 | elementId=null은 frame lookup 없이 None만 허용한다. 문자열 target은 pinned frame에서 찾아야 한다. |
| 13 | Android log truncation이 null row를 제거 | JNI logger가 sample object를 짧은 JSON row로 분리한다. 새 네 row가 완전한 JSON null로 기록됐다. |
| 14 | app report JSON이 null 대신 문자열 "null"을 기록 | log row를 JSON으로 파싱하고 elementId와 nodeIndex가 실제 null인지 비교했다. |
| 15 | Android viewport가 Chrome viewport와 같다고 잘못 가정 | Chrome은 320×120; AVD는 surface/density 환산 300.9524×100.1905다. 고정 네 점은 두 viewport 모두 밖에 있다. |
| 16 | document root 배경이 viewport 밖의 좌표에서 hit target이 됨 | RuntimeRenderSnapshot::hit_test_css_point()가 box 순회 전에 viewport 경계를 검사하며 Android app query 4/4 null이었다. |
| 17 | hit query 표본 수 증가가 paint sample 수를 바꿈 | PixelCopy inventory는 11점으로 유지됐고 exact 11/11이다. hit query 15개와 별개로 보고했다. |
| 18 | viewport null 결과를 DOM input/event 성공으로 과장 | 검토와 evidence는 box query만 주장한다. listener, pointer dispatch, 접근성 입력은 남은 범위로 쓴다. |
| 19 | isolated build 실패를 제품 회귀로 판정 | 첫 빌드는 worktree에 없는 기본 V8 경로 때문에 중단됐다. pinned V8 checkout 7b50b62가 clean임을 읽기 전용 확인한 뒤 SPINON_V8_DIR를 명시해 APK 빌드와 AVD 실행을 통과했다. |
| 20 | fixture oracle가 기본 FFI 제품 archive에 들어감 | C13.1 feature-on archive에 FFI symbols와 viewport-left-outside 문자열이 있고 기본 archive에는 없다. 판정은 host static archive 범위로 제한했다. |

검증 결과: agent-browser Chrome 155.0.8059.40 네 경계 밖 표본은 4/4 null이었다. cargo test --locked --workspace, feature-on spinon-ffi tests, workspace Clippy, feature-on FFI Clippy, rustfmt와 diff check가 통과했다. Android API 37 ARM64 AVD run c2952496a76의 app runtime query는 15/15, viewport null controls는 4/4, PixelCopy paint sample은 11/11 exact였다. AVD는 ANGLE/Vulkan/SwiftShader이며 실기기를 사용하지 않았다.
