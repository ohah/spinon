# C13.1 compact surface matrix PR 변경 관점 적대 검토 · 2026-10-14

이 문서는 계획 검토와 구현 실패 검토를 반복하지 않고, 변경 묶음이 저장소 계약·상태·증거·플랫폼 통합·PR 경계와 충돌하는지 별도로 살폈다.

| # | PR 변경 실패 관점 | 변경 대조와 판정 |
|---:|---|---|
| 1 | PR base가 stale해 이미 main에 들어간 C13.1 후속을 되돌리거나 충돌을 숨긴다 | `origin/main` fetch 뒤 현재 branch의 공통점은 최신 `c29b0b8`이며 branch는 plan/oracle 두 commit만 앞서 있다. **통과.** |
| 2 | 동일 branch의 기존 PR을 새 PR로 중복 개설한다 | 열린 PR 목록에서 `feat/c13-1-surface-matrix` PR은 없었다. **통과.** |
| 3 | 과거 #157/#158/#160의 작업을 다시 구현으로 주장한다 | 이 변경은 이미 병합된 full-viewport hit identity 뒤의 compact screen/PixelCopy slice로 계획·상태에서 분리했다. **통과.** |
| 4 | compact follow-up 한 조각으로 C13.1 전체 상태를 완료로 표시한다 | `STATUS.md`의 상위 C13.1 및 full-surface/WPT/event gate는 미완료다. follow-up은 13 case의 41 frame/52 sample 결과만 설명한다. **통과.** |
| 5 | 공개 CSS 지원 또는 앱 API를 추가한 것으로 읽힌다 | 변경 ABI는 C13.1 fixture-test feature 안에 있고 문서·화면 label도 fixture임을 표시한다. 공개 CSS/API 지원을 추가했다고 쓰지 않았다. **통과.** |
| 6 | 내부 인터페이스 버전을 임의로 올리거나 API 계약 없이 완료 체크한다 | 숫자 계약 버전은 변경하지 않았다. 내부 0060 명세에 관찰된 paint 경계와 한계를 반영했고 상태 ID는 미완료로 남겼다. **통과.** |
| 7 | Rust fixture가 제품 runtime 모듈 안에 섞여 default build 책임을 넓힌다 | 새 oracle adapter는 `runtime_gpu/fixtures/c13_1_compact_surface_matrix.rs` 아래 전용 feature로 연결했다. **통과.** |
| 8 | fixture 경로 이동 후 중복 source나 stale include가 생긴다 | 새 파일은 기존 fixture 계층을 사용하고 공유 JS source를 단일 include한다. C13.1 module 참조를 검색해 중복 파일이 없다. **통과.** |
| 9 | source fallback 수정이 Chrome baseline과 다른 fixture를 만든다 | HTML wrapper와 runtime은 같은 JS source를 쓴다. browser의 `document.body` 경로는 유지되고 body-less host에서만 document fallback을 쓴다. source SHA와 observation hash는 precomparison README에 갱신됐다. **통과.** |
| 10 | Chrome oracle metadata/샘플 파일이 바뀌었지만 PR evidence가 이를 드러내지 않는다 | README에 Chrome revision·fixture/source SHA·정규화 observation/raw JSON/screenshot hash를 각각 적고 runtime validator도 revision을 검사한다. **통과.** |
| 11 | 기존 Chrome precomparison README가 실행 결과로 덮여 의미를 잃는다 | README diff는 precomparison 내용과 agent-browser Chrome screenshot을 유지하고 runtime 실행 근거는 별도 `runtime-execution.md`로 분리한다. **통과.** |
| 12 | runtime evidence가 ADB transport 실패를 제품 결함으로 오진한다 | 기록은 emulator가 `device`로 연결된 뒤 최초 앱 route가 `SPINON_C131_DISABLED`로 실패했다고 구분한다. 수정한 원인은 canonical/legacy build flag 전달 mismatch다. **통과.** |
| 13 | emulator 실행 도구 PATH 이슈가 ADB 실패 원인으로 잘못 남는다 | SDK `emulator` binary 존재와 이미 실행 중인 AVD를 분리해 설명하며 이를 ADB/app install 실패로 취급하지 않았다. **통과.** |
| 14 | 실패 로그를 제거해 수정 전 실제 원인을 추적할 수 없다 | failure diagnosis와 두 negative-control logs를 별도 보존하고 최종 runtime log를 구분했다. **통과.** |
| 15 | negative control이 실제 구현과 무관한 문자열 주장으로만 남는다 | 서로 다른 build flag 값의 exit-2 로그와 AVD fixture route collision 거부 로그를 PR evidence에 연결한다. **통과.** |
| 16 | feature isolation이 static archive 결과를 최종 앱/product binary로 확대 해석한다 | 문서는 default/opt-in host static archive에서 symbol/payload만 판정하고 app size·release bundle 보증을 하지 않는다고 명시한다. **통과.** |
| 17 | feature isolation tool의 조기 pipe 종료가 false negative로 남는다 | 최종 evidence는 전체 `llvm-nm`/`strings` 출력 파일 생성 후 matching한 결과만 사용하며 잘못된 초기 SIGPIPE 판정은 제거했다. **통과.** |
| 18 | Android 전체 화면과 surface crop이 다른 실행에서 왔다 | 최종 runtime log의 `run_id=1eaf84a37315`, PixelCopy PNG hash 및 788×263 크기를 대조했다. 전체 화면은 같은 설치된 positive fixture 실행 직후 보존했다. **통과.** |
| 19 | Android image가 AVD display screenshot만 있고 검증한 surface가 PR에서 보이지 않는다 | PR 본문에서 788×263 PixelCopy PNG와 1080×2400 전체 AVD 앱 화면을 각각 inline embed할 수 있는 repository artifact로 추가한다. **통과.** |
| 20 | iOS Simulator 화면을 Chrome 또는 Android compositor oracle로 오인한다 | iOS 화면은 별도 Simulator artifact로 분류하고 Metal drawable readback/scanout이 아니라고 runtime 문서에서 제한했다. **통과.** |
| 21 | AVD가 실기기나 hardware Vulkan 성능 결과처럼 보인다 | evidence는 API 37 ARM64 AVD, `ro.kernel.qemu=1`, ANGLE/Vulkan/SwiftShader software backend라고 적었다. **통과.** |
| 22 | app route는 추가했지만 build feature가 꺼져 실패하는 상태를 성공 화면처럼 남긴다 | `MainActivity` gating, native host creation report, eval status와 ready label을 확인했고 캡처는 positive route successful run에서 만들었다. **통과.** |
| 23 | C13.1 다른 route와 compact route가 동시 활성화돼 결과가 어느 fixture인지 모호하다 | Android와 iOS 초기화에서 한 번에 하나만 허용하고 Android route collision control이 host 생성 전 거부를 확인한다. **통과.** |
| 24 | Android/iOS가 서로 다른 viewport 또는 case/sample 순서를 사용한다 | Rust oracle, Android/Swift report parser, shared JS fixture의 13 ID와 role 순서를 비교했다. 두 runtime은 300×100, 41 frame, 52 sample을 보고했다. **통과.** |
| 25 | 플랫폼 parser는 개수만 세고 누락/중복 ID나 잘못된 RGBA를 success 처리한다 | Android/Swift는 ID·role·좌표·RGBA domain·hit status를 확인하며 Rust oracle은 NodeId와 frame mapping을 먼저 fail-closed 한다. **통과.** |
| 26 | 보고된 PixelCopy 점 개수가 맞아도 한 표본이 다른 ID에 연결된다 | Android capture는 고정 sample ID를 각 PixelCopy pixel에 유지하고 개수, 각 ID의 oracle RGBA, 52 exact match를 확인한다. **통과.** |
| 27 | 새 paint ordering이 별도 hit-test order나 overflow clip 계약을 암묵적으로 바꾼다 | renderer test는 parent-before-child paint와 child-before-parent hit order를 별도로 검사한다. 명세는 clip ancestry가 독립적으로 적용됨을 설명한다. **통과.** |
| 28 | 문서의 실제 범위보다 검증 명령 또는 CI 통과 범위를 넓혀 말한다 | workspace 775/0/4, focused fixture 2/2, Node 3/3, Clippy/fmt, Android/iOS build와 실제 simulator 결과를 각각 구분한다. PR 생성 후 required CI는 별도로 조회한다. **통과.** |
| 29 | GitHub 라벨이 없거나 존재하지 않는 라벨을 만들어 프로젝트 규칙을 우회한다 | 현재 저장소에 기존 `enhancement`, `area: renderer`, `area: android`, `area: ios`, `area: testing` 라벨이 있다. PR에 기존 라벨만 적용한다. **통과.** |
| 30 | 공개 evidence log가 사용자 홈·외부 checkout 경로를 노출한다 | 공개 저장소를 확인한 뒤 log 속 사용자명·홈·임시 worktree·V8 source 절대경로만 placeholder로 익명화했고 성공/실패 출력과 flag는 보존했다. 로그에서 경로 외 자격 증명 문자열은 발견하지 않았다. **통과.** |

머지 전에는 PR의 최신 required checks, mergeability, conflict 상태를 다시 확인한다. 본 변경은 sample된 runtime paint/hit slice로 제한되며 broad surface/event/WPT gate는 남겨 둔다.
