# C11.4 generic mixed text/image · implementation/runtime review 20

범위: generic HostNode mixed adapter, trace revision validation, native scene routing, and the accepted Android/iOS simulator runs. Each row checks a distinct failure path. This review was completed after the final 8-scene matrix.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 기존 고정 mixed scene이 generic adapter에 먼저 잡힘 | `prepare`는 기존 `SCENES` ID를 먼저 해석하고 lookup miss만 새 adapter에 전달한다. Rust static-route test와 두 simulator의 fixed-ID 4개 control이 통과했다. PASS |
| 2 | 새 HostNode ID가 scene 선택이나 기대값에 사용됨 | FFI route는 `novel-generic-mixed-*` root/item ID를 사용한다. 네 track/order 조합 모두 두 simulator에서 통과했다. PASS |
| 3 | 한 text item만 있는 기존 legacy fallback이 mixed adapter에서 claim됨 | text-only root는 `Ok(None)`이다. C11.4 legacy fallback의 기존 13-scene evidence와 이번 route test가 유지된다. PASS |
| 4 | image-only root가 text/image 혼합으로 처리됨 | image-only root는 `Ok(None)`이고 partial input을 만들지 않는다. Runtime test 통과. PASS |
| 5 | 두 text item과 HTML image가 generic text collector에 허용됨 | 새 adapter는 정확히 두 direct child와 text/image 각 한 개를 요구해 거부한다. 기존 generic collector negative fixture도 root image + 두 text를 `unsupported_c114_text_source`로 거부한다. PASS |
| 6 | direct text node가 root 아래 익명 child로 들어와 무단 수용됨 | mixed signature는 활성화되지만 root topology 조건에서 unsupported error로 끝난다. Partial input은 생성되지 않는다. PASS |
| 7 | direct child 누락·추가가 허용됨 | 정확히 두 child를 요구한다. 추가 child negative가 Rust test에서 거부됐다. PASS |
| 8 | child tag/namespace 중복으로 text/image 역할이 모호해짐 | HTML namespace의 `div` 하나와 `img` 하나만 각 역할에 들어갈 수 있고 다른 조합은 오류다. 역할 수 검증을 코드에서 확인했다. PASS |
| 9 | nested text 또는 non-leaf image가 direct source로 오인됨 | text `div`의 자식은 text node 하나, image descendant는 없어야 한다. nested text와 malformed input negative가 통과했다. PASS |
| 10 | 바뀐 text content가 기대 intrinsic 폭으로 사용됨 | 정확한 `XX XXXXX`만 허용한다. 변경 text negative가 통과했고 Ahem 입력은 Android/iOS runtime matcher에서 50px/80px로 확인됐다. PASS |
| 11 | 다른 이미지 URL 또는 누락된 `src`가 fixture PNG로 대체됨 | 기존 `IMAGE_URL`과 일치하지 않으면 실패한다. wrong/missing source negative가 통과했다. PASS |
| 12 | decode 실패나 PNG 크기 불일치가 intrinsic input에 숨겨짐 | decode 오류는 별도 runtime failure로 반환하고 natural dimensions 24×16을 확인한다. 두 플랫폼 trace가 replaced input 24×16을 보고했다. PASS |
| 13 | track type이나 row/column 개수가 다른 입력이 장면으로 분류됨 | column은 단일 `MinContent` 또는 `MaxContent`만 허용하고 전체 Grid style validator가 두 fixed row와 layout profile을 검사한다. bad-track negative 및 네 장면 trace가 통과했다. PASS |
| 14 | root edge, alignment, direction 또는 writing-mode 차이가 결과를 바꿈 | root style와 writing-style snapshot 모두 고정 조건을 검사한다. padding edge negative가 실패하고 실제 root frame/track은 두 플랫폼에서 기대값과 일치했다. PASS |
| 15 | item placement, dimensions 또는 self-alignment 차이가 허용됨 | item validator는 지정 CSS subset을 검사하고 두 item의 row/column placement를 고정한다. bad placement negative와 네 장면 frames가 통과했다. PASS |
| 16 | stale document/style snapshot이 새 HostNode IDs에 재사용됨 | profile·generation·document revision·render-tree revision을 검사한다. stale snapshot/profile negatives가 실패했다. PASS |
| 17 | mixed image intrinsic revision과 text sub-layout revision의 정당한 차이가 거부됨 | 앱 실행에서 발견한 검사 결함을 수정했다. text layout은 source/style/environment가 일치해야 하고 replaced intrinsic revision만 달라도 허용한다. FFI trace test 6/6 및 simulator 8/8씩 통과했다. PASS |
| 18 | 실제 text/image intrinsic inputs는 잘못된 revision을 공유함 | trace reporter는 intrinsic measurement와 item input에 전체 revision equality를 계속 요구한다. Android/iOS matcher가 input revision, source kind, node ID, source order를 각각 확인했다. PASS |
| 19 | reversed source order에서 app matcher가 node ID나 kind를 뒤바꿈 | Android Java와 iOS Swift가 forward/reversed ID별 node/order 기대값을 고정한다. 각 플랫폼에서 min/max reversed 두 장면이 모두 통과했다. PASS |
| 20 | fixture route가 기본 runtime artifact에 노출되거나 실기기로 잘못 실행됨 | 기본 archive에는 route 문자열이 없고 opt-in archive에는 있다. 캡처는 `emulator-5554`와 지정 iPhone 17 Pro Simulator만 사용했으며 앱 종료 후 두 simulator는 켜진 상태로 남았다. PASS |

검증 근거: [runtime tests](validation/runtime-tests.log), [FFI route tests](validation/ffi-tests.log), [trace-report tests](validation/ffi-trace-report-tests.log), [simulator matrices](README.md), [archive boundary](validation/archive-boundary/result.txt). 화면은 layout boxes/host surface이며 glyph와 image texel painting 검증이 아니다.
