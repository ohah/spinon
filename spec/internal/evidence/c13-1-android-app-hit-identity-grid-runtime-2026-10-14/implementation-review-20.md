# C13.1 Android 앱 hit identity grid 구현 실패 관점 검토 20개 · 2026-10-14

계획 PR #163 검토와 분리해 Rust grid validator/comparator, JNI report 연결, Java 앱 route 및 API 37 AVD 동작을 확인했다. 이는 compact 300×100 CSS integer grid와 fixed PixelCopy 표본만 다루며 C13.1 완료 판정이 아니다.

| # | 독립 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | grid 파일이 바뀌어도 고정 oracle로 받아들인다 | parser가 파일 SHA-256 `35e88e…41f6`를 먼저 확인한다. 파일 변경 validator negative test가 거부한다. **통과.** |
| 2 | grid와 Chrome observation이 다른 브라우저 실행에서 섞인다 | Chrome version/revision/UA와 grid의 browser provenance를 함께 검사한다. metadata mutation test가 실패한다. **통과.** |
| 3 | 실행 대상 HTML/JS가 oracle capture 뒤 바뀐다 | feature 전용 `include_bytes!` 입력의 HTML·JS SHA를 고정 observation에 대조한다. source mutation validator가 거부한다. **통과.** |
| 4 | DPR·scroll·viewport 크기 차이로 점 좌표 의미가 달라진다 | 300×100, DPR 1, scroll 0 및 root frame을 각각 pin한다. viewport/convention mutation test가 거부한다. **통과.** |
| 5 | runtime NodeId 배열이 재정렬돼 frame index가 다른 Chrome ID로 매핑된다 | fixture script의 생성·append 순서, runtime DOM preorder를 고정하고 NodeId 증가 순서를 검사한다. 순서 mutation test가 거부한다. **통과.** |
| 6 | 빠진 행이나 빈 RLE 행이 전체 grid로 오인된다 | row count 100과 non-empty 각 row를 강제한다. empty-row mutation test가 거부한다. **통과.** |
| 7 | run 사이에 빈 점 또는 겹친 점이 생긴다 | 각 run의 시작점을 직전 끝점에 연결하고 `[start,end)`를 검사한다. gap·overlap·out-of-bounds tests가 거부한다. **통과.** |
| 8 | 30,000점보다 적거나 많은 좌표가 조용히 비교된다 | 각 행의 마지막 run까지 폭 300을 덮고 총점 30,000 및 run 수 870을 검증한다. 전체 query test가 30,000회 호출을 센다. **통과.** |
| 9 | node-index 변경 뒤 확장 identity가 맞지 않는다 | histogram과 expanded-grid digest를 비교한다. histogram·identity mutation tests가 거부한다. **통과.** |
| 10 | 기존 fixed 52 sample과 grid의 identity가 서로 모순된다 | 기존 52 좌표가 30,000점 중 같은 identity인지 validator가 확인한다. subset mutation test가 거부한다. **통과.** |
| 11 | 경계 `(299,99)`가 빠지거나 한 번 더 질의된다 | comparator가 `x=0..299`, `y=0..99`를 순회한다. query counter와 끝 좌표 one-point mismatch test가 확인한다. **통과.** |
| 12 | 한 점을 중복 질의하거나 row-major 순서를 바꿔 digest만 비교한다 | 각 validated run을 한 번 방문하며 test closure 호출 수가 30,000이다. preview 좌표는 첫 불일치의 고정 순서를 보존한다. **코드·focused test 통과.** |
| 13 | runtime hit의 NodeId가 fixture frame에 없는데 유효 index로 가장한다 | frame 매핑 실패는 `unknown-runtime-node-*` mismatch로 남는다. 이 경계는 코드에서 확인했고 AVD 정상 경로는 30,000/30,000이다. **코드 경로 통과.** |
| 14 | hit가 `None`인데 root fallback으로 바꿔 일치를 만든다 | 실제 `None`은 `node_index=None`으로 유지되어 예상 root와 mismatch가 된다. all-missing negative closure는 30,000 mismatch를 만든다. **통과.** |
| 15 | mismatch 개수와 match 개수의 합이 표본 수와 다르다 | Rust comparator가 query·match·mismatch 합을 확인한다. Java도 각 count의 상한과 합을 fail-closed로 검사한다. **통과.** |
| 16 | 거대한 mismatch 상세가 JNI report를 넘치게 한다 | Rust summary를 8,192 bytes로 제한하고 preview는 최대 16점이다. 전부 missing인 테스트도 16 preview에 머문다. **통과.** |
| 17 | JNI 문자열에서 JSON 문자열 내부 중괄호/escape가 object 종료로 오인된다 | Java balanced-object scanner가 quote·escape·nested brace를 추적한다. 정상 AVD JSON을 실제로 파싱했다. **통과.** |
| 18 | 잘못된 preview type 또는 count가 검증 로그 뒤에 발견된다 | Java는 count 범위, 좌표 영역, expected/actual ID type과 preview 길이를 요약 로그 전에 검사한다. 실제 parser의 malformed-summary fault injection은 실행하지 않았다. **코드 경계 통과, 직접 주입 미실행.** |
| 19 | runtime hit report와 비동기 PixelCopy 결과가 다른 실행으로 합쳐진다 | route 진입 때 만든 단일 run ID가 eval/grid/PixelCopy begin·summary에 나타난다. runtime generation 1과 SurfaceView generation 2는 다른 counter로 기록된다. **AVD 통과.** |
| 20 | feature fixture가 기본 library에 들어가거나 AVD 결과를 제품/실기기 결과로 넓힌다 | 기본 host archive에서 fixture 심볼·grid payload·`sha2`가 없고 opt-in positive control에는 있다. API 37 AVD 결과와 남은 플랫폼 범위를 분리했다. **통과.** |

확인 산출물: [실행 근거와 이미지](README.md), [feature 격리](feature-isolation.txt), [Rust comparator/validator tests](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1_compact_surface_matrix/hit_identity_grid/tests.rs), [AVD logcat](avd-logcat.txt). 보고된 preview corruption의 직접 fault-injection, iOS 앱 실행, physical device, WPT와 DOM event dispatch는 범위 밖이다.
