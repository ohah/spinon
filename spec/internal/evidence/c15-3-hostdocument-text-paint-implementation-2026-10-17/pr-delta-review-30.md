# C15.3 HostDocument text paint · PR 변경 적대 검토 30개

**검토 범위:** 이번 변경의 Cargo·Rust FFI 경계, HostDocument/layout/raster 연결, Android/iOS 호출·표면 확인, 공식 상태·명세·재현 근거를 각각 다른 실패 경로에서 대조했다. 앞선 구현 코드 20개 검토와 별도로 PR 통합 변경을 대상으로 작성했다. 2026-10-17 최종 소스와 final2 simulator 결과를 기준으로 확인했다.

| # | PR 통합 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | C15.3 fixture 모듈 선언이 일반 runtime 모듈로 이동해 제품 코드에서 컴파일된다 | `runtime_gpu.rs`와 `fixtures.rs` 선언이 전용 feature로 조건화되어 있다. fixture 경계 회귀 검사는 작업별 Rust 파일이 `fixtures/` 밖에서 선언되는 것을 거부하며 6/6 통과했다. |
| 2 | Cargo feature를 켤 때만 필요한 `spinon-layout`이 default dependency graph에 들어온다 | `spinon-layout`은 `c15-3-wgpu-glyph-paint-fixture-test`에서 `dep:` optional dependency로만 추가됐다. default 및 opt-in archive build를 모두 기록했다. |
| 3 | 헤더 선언은 기본 C 소비자에게 보이는데 실제 함수는 feature 비활성으로 빠진다 | 새 생성자 선언은 해당 opt-in C define 조건 아래 있으며 Android/iOS 빌드에서 동일 fixture-test define을 쓴다. 기본 archive scan에서 생성자 symbol이 없고 opt-in archive에서 발견된다. |
| 4 | 모듈을 fixture 폴더로 옮긴 것만 근거로 default artifact 격리를 주장한다 | 소스 경로 확인에 그치지 않고 기본·opt-in archive를 빌드해 symbol과 payload를 검사했다. 두 결과와 양성 대조 목록이 최종 scan JSON에 있다. |
| 5 | Rust 모듈 분할 뒤 `include_bytes!` 상대 경로가 이전 위치를 가리키거나 다른 font를 포함한다 | 분리된 `layout.rs`의 상대 경로는 C15 Ahem fixture를 resolve한다. 최종 opt-in scan에 Ahem/Noto Sans KR positive payload가 있고 Arabic/Emoji bytes는 없으며 default scan에는 C15.3 font payload가 없다. |
| 6 | C ABI 생성자가 null output 포인터를 역참조한다 | 새 constructor가 host 생성 전에 null pointer를 거부한다. 기존 FFI test 모듈에 null-pointer regression assertion이 추가됐고 49개 통과 결과에 포함됐다. |
| 7 | capacity 0인데 빈 버퍼에 report를 쓰려 한다 | constructor가 zero capacity를 거부한다. source review에서 report write 전에 분기하는 것을 확인했다. |
| 8 | capacity가 report보다 짧은데 유효 host pointer를 먼저 반환한다 | report 복사 성공 뒤에만 `Box::into_raw`가 호출된다. 기존 공용 short-buffer negative test가 opt-in FFI suite에서 통과했다. |
| 9 | HostDocument 생성 실패 또는 commit 오류 후 부분 상태가 paint host로 게시된다 | fixture 생성과 batch commit은 `Result` 전파를 사용하고, HostDocument layout 입력이 만들어진 뒤에야 scene 조립을 진행한다. 실패 시 FFI가 null을 반환한다. |
| 10 | document revision이 다른 batch를 commit해 우연한 tree를 snapshot으로 사용한다 | batch는 새 document의 현재 revision과 fixture owner로 생성한다. root, paragraph, text 생성과 삽입을 한 batch에서 commit하고 commit 실패를 그대로 반환한다. |
| 11 | layout source metadata가 임의 상수여서 실제 fixture snapshot 교체를 감지하지 못한다 | generation, document revision, render-tree revision을 snapshot에서 구성하고 `LayoutSourceRevision::HostDocument`와 전부 대조한다. |
| 12 | layout output은 현재 input과 같지만 paragraph text snapshot만 이전 revision이다 | input/output/text snapshot revision을 각각 검사한다. revision 불변식 negative test가 opt-in suite에 있어 stale result가 성공 처리되지 않는다. |
| 13 | root가 아닌 paragraph 또는 직접 Text가 아닌 descendant의 내용을 성공 입력으로 받는다 | paragraph node id, 단일 source 개수, 직접 text handle, `XX`, 빈 inline ancestry를 모두 검사한다. 이는 generic DOM admission을 주장하지 않는 fixture 경계와 일치한다. |
| 14 | UTF-16 source index와 UTF-8 paragraph byte offset 차이로 두 번째 glyph가 잘못 연결된다 | 두 단위의 `[0,1]`, `[1,2]` span을 별도로 고정 검사하고 glyph source mapping도 같은 UTF-16 span에 대조한다. |
| 15 | shaping 결과가 fixture 문구와 다르거나 glyph source가 누락돼도 layout geometry만 통과한다 | 두 glyph 수, advance 10px, x origin 0/10, 각 source node/span을 별도 확인한다. |
| 16 | 15px paragraph에서 `XX`가 wrap 또는 clamp되어 overflow sample이 사라진다 | 고정 oracle은 15px available width, 한 줄 advance/width 20px, 10px height, 5px visible overflow다. layout·scene 및 표면 검증에서 경계 바깥 ink 4개를 유지한다. |
| 17 | 320×240 viewport가 아닌 다른 root frame에 paint가 배치된다 | root 및 paragraph의 origin·width·height를 layout output과 대조한다. 내부 oracle 검사는 finite 좌표와 0.05px tolerance를 사용한다. |
| 18 | layout과 raster가 다른 font engine 또는 system fallback을 사용해 같은 glyph 수만 맞춘다 | system-font 없는 단일 `TextLayoutEngine`에 Ahem을 등록하고 layout/raster에 동일 engine을 전달한다. |
| 19 | 같은 글꼴 이름이라도 font registry가 바뀐 raster가 들어간다 | raster snapshot의 registry revision, paragraph id, glyph 수, glyph ID 및 source mapping을 layout snapshot과 대조한다. |
| 20 | bitmap이 빠진 glyph를 투명 glyph로 취급해 runtime에 누락 상태를 숨긴다 | alpha-mask content와 bitmap 치수·bearing·baseline을 확인하고 필요한 raster bitmap이 없으면 명시적 오류로 중단한다. |
| 21 | raster pixel buffer 길이 손상으로 sample 추출 중 panic 또는 잘못된 색이 발생한다 | sample index 접근을 bounds-check하고 버퍼 크기 불일치를 error로 반환하도록 고쳤다. 이 수정 후 전체 opt-in suite와 두 simulator를 final2 소스에서 재실행했다. |
| 22 | sample 좌표를 예상값으로 하드코딩해 실제 glyph ink 밖을 읽는다 | 여섯 좌표는 raster alpha coverage에서 파생한다. 좌표 중복, viewport 경계, owner 경계 안쪽 2개/바깥쪽 4개 분포를 검사한다. |
| 23 | text paint가 다른 owner node나 다른 frame의 origin을 사용한다 | runtime text owner는 HostDocument paragraph id이고 origin은 같은 `LayoutOutput`의 paragraph frame이다. scene 생성 key와 source/frame association 오류는 실패한다. |
| 24 | root/paragraph box 순서 또는 clip이 overflow ink를 숨긴다 | paint command 순서는 root box→paragraph box→text이고 fixture clip은 unconstrained다. PixelCopy와 Metal sample 모두 paragraph 오른쪽 밖 ink 4개를 읽었다. |
| 25 | 새 host flag가 기본 C15 glyph route의 렌더 동작을 바꾼다 | HostDocument route는 별도 constructor/flag로만 켜지고 default false다. 기존 constructor 분기를 유지하며 opt-in suite의 기존 회귀 포함 49 passed/2 ignored를 확인했다. |
| 26 | Android opt-in native method가 기본 Java source set에 직접 있어 기본 앱 compile을 깨뜨린다 | JNI adapter는 전용 fixture-test source set에만 있고 main Activity는 reflection 기반 route를 쓴다. 최종 Gradle AVD build/install/launch가 성공했다. |
| 27 | Android JNI symbol 이름, C declaration, Kotlin/Java 호출이 서로 달라 기존 화면만 실행된다 | JNI entry와 C ABI constructor 이름을 나란히 확인했다. 앱 report는 C15.3 HostDocument route와 runtime glyph batch를 기록하고 PixelCopy 검사까지 완료했다. |
| 28 | Android route extra가 일반 fixture key와 충돌해 이전 scene이 조용히 표시된다 | HostDocument route는 별도 intent extra로 분기하고 viewport/report 조건을 확인한다. final2 AVD log와 screen은 해당 route 실행 정보를 보존한다. |
| 29 | iOS C13 Swift allowlist가 새 CLI argument를 거부하거나 legacy route와 혼동한다 | 첫 실제 실행에서 validator rejection을 확인하고 허용 목록과 회귀 검사를 함께 수정했다. final2 iPhone 17 Pro Simulator가 새 route를 실행했다. |
| 30 | iOS selector/header/Objective-C++/Rust FFI 사이에서 새 constructor 연결이 끊기거나 surface DPR을 잘못 계산한다 | 새 selector·헤더·C ABI 호출 조건을 대조했다. final2 Metal drawable은 960×720에서 320×240 CSS 표본을 읽어 6/6 exact를 기록하며 측정 시점을 compositor 이전으로 표시한다. |

이 검토는 PR 코드·fixture 경계·기록된 Android/iOS simulator 결과 사이에서 실패 경로를 따로 대조했다. 실기기, Android hardware GPU, iOS compositor 이후 화면, 일반 DOM/CSS text admission은 실행 범위에서 제외했다. 공식 C15.3/C15는 미완료이며 완료 수는 `61/197`로 유지한다. [구현 실패 관점 20개](implementation-review-20.md) · [실행·화면·hash](README.md)
