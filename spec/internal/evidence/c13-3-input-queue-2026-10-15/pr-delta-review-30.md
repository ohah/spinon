# C13.3 bounded input queue PR 변경 관점 30개

## 기준

- 비교 대상은 `origin/main`과 이 branch의 전체 staged PR delta다.
- 내부 queue model, Rust crate exports, C13.3 plan/contract/status/evidence 간 일치와 scope 경계를 확인한다.
- 실행 근거: `spinon-render` 72/72, 전체 workspace 861 passed·0 failed·4 ignored, workspace Clippy, rustfmt와 diff check 통과.

| # | 적대적 관점 | 판정 |
|---:|---|---|
| 1 | 새 타입이 `spinon-render` crate root에 export되지 않아 platform caller가 접근하지 못한다 | `runtime`와 crate root export를 함께 확인했다. 통과. |
| 2 | `scroll_input_queue` module이 test-only로 선언되어 release runtime에서 사라진다 | Production module은 일반 runtime tree에 등록되며 tests만 `cfg(test)`다. 통과. |
| 3 | 새 queue가 fixture feature로 잘못 묶여 기본 runtime에 다른 ABI가 섞인다 | FFI/C header/Android/iOS build flag를 건드리지 않았다. Pure Rust runtime type이며 fixture ABI가 아니다. 통과. |
| 4 | 출시 전 internal API인데 버전을 올리거나 public web API로 소개한다 | `0.1.0-draft` 유지, plan/contract에서 public Pointer/Wheel/scroll API가 아니라고 명시한다. 통과. |
| 5 | gesture identity에서 pointer 또는 surface generation을 빼 stale terminal이 새 contact를 해제한다 | Identity는 gesture ID, pointer ID, surface generation 전체를 비교한다. 통과. |
| 6 | pointer ID 0을 invalid로 오인해 Android 첫 pointer 입력을 거부한다 | gesture ID만 non-zero 조건으로 두고 pointer ID 0은 허용한다. 통과. |
| 7 | queue capacity가 regular entries만 제한해 terminal 쪽에 무한 event를 쌓는다 | reservation vector와 regular queue 모두 생성 시 고정 크기다. 각 gesture는 slot 하나만 가진다. 통과. |
| 8 | capacity 생성 시 allocator 실패가 panic으로 process를 종료한다 | `try_reserve_exact` 오류를 queue construction error로 반환한다. 통과. |
| 9 | Begin queue insertion은 성공했지만 terminal slot 확보는 실패해 dangling active stream을 만든다 | regular/terminal capacity를 둘 다 확인한 뒤에만 reservation과 event를 추가한다. 통과. |
| 10 | Begin `Rejected`와 second-pointer `Suppressed`를 같은 bool 값으로 처리해 terminal slot을 조기 반환한다 | `Tracking`/`Suppressed`/`Rejected` disposition enum을 추가하고 rejected만 해제한다. 통과. |
| 11 | Begin 결과를 받기 전에 worker가 그 뒤의 Move를 session에 전달한다 | 미확정 Begin이 있으면 `pop_next`가 멈추며 completion 뒤에 순서가 이어진다. 통과. |
| 12 | Begin 거부 뒤 이미 queue된 identity event가 worker에 남는다 | rejection은 해당 identity의 regular events와 reserved terminal을 함께 제거한다. 통과. |
| 13 | Begin 중복이나 gesture ID 0이 reservation을 중복 점유한다 | zero/duplicate identity controls와 reservation count 검사가 있다. 통과. |
| 14 | native slop/recognizer 전에 Move를 넣거나 PanBegan을 여러 번 넣어 session state를 건너뛴다 | Move는 PanBegan 뒤에만 허용하고 PanBegan은 한 번만 허용한다. 통과. |
| 15 | terminal side slot 이벤트가 regular queue보다 먼저 또는 배열 순서대로 전달된다 | regular head와 가장 이른 terminal sequence를 비교한다. 통과. |
| 16 | terminal sequence가 같은 gesture의 earlier accepted Move를 건너뛴다 | reserved slot End 테스트에서 full regular queue 뒤에도 Move→Wheel→End 순서를 확인한다. 통과. |
| 17 | 한 Move가 regular capacity를 채울 때 마지막 event가 조용히 빠져 gesture가 active로 남는다 | 비병합 motion saturation은 reserved `InputQueueSaturated` Cancel로 바뀐다. 통과. |
| 18 | terminal pending 뒤 늦은 Move/End가 두 번째 terminal slot을 만들거나 state를 바꾼다 | same identity 후속 event는 `TerminalAlreadyQueued`로 거부한다. 통과. |
| 19 | 대기 중인 두 Move를 병합하며 cumulative displacement/마지막 sequence를 잃는다 | 마지막 cumulative sample과 sequence를 대체 기록하고 sample count를 증가시킨다. 통과. |
| 20 | 다른 gesture terminal이 reserved slot에 끼어도 regular tail만 보고 Move를 합쳐 sequence를 재정렬한다 | 초기 review에서 찾은 결함을 `last_accepted_sequence`와 회귀 fixture로 수정했다. 통과. |
| 21 | capacity에서 거부된 wheel input도 accepted barrier로 취급해 Move를 불필요하게 cancel한다 | coalescing adjacency는 마지막 accepted sequence만 사용한다. rejected wheel control이 이를 확인한다. 통과. |
| 22 | wheel queue key가 point만 보고 서로 다른 target/owner/revision/policy를 합친다 | key에는 CSS point, target, full render key/basis revision, policy epoch이 들어간다. 통과. |
| 23 | 서로 다른 wheel source, momentum phase, unit, scale 또는 modifiers를 합쳐 trace 의미를 잃는다 | raw metadata도 coalescing key와 sample payload에 남는다. 통과. |
| 24 | 비유한 raw/CSS delta, 0 scale 또는 snapshot key와 다른 basis가 enqueue된다 | `valid_wheel_sample`가 sample을 먼저 거부한다. 통과. |
| 25 | wheel 합산 overflow가 이전 queued sample을 먼저 변경한 뒤 실패한다 | 새 값을 range 검사한 다음에만 대입한다. queue room이 있으면 두 sample로 보존하고 없으면 전체 reject한다. 통과. |
| 26 | wheel queue 포화가 X/Y 일부만 적용하거나 touch gesture를 cancel한다 | wheel sample 전체를 거부하고 별도 saturating counter만 올린다. 통과. |
| 27 | 한 terminal reservation이 dequeue 뒤 두 번 반환되거나 duplicate terminal을 소비한다 | terminal dequeue가 slot을 비우며 두 번째 terminal은 unknown/pending error로 끝난다. 통과. |
| 28 | 새 Rust tests가 기존 session tests와 책임을 섞거나 500-line 권고를 넘긴다 | Queue tests는 gesture/pressure/terminal/wheel 하위 파일로 나누고 production files는 각각 500줄 미만이다. 통과. |
| 29 | 계획·내부 계약·STATUS가 queue model을 native runtime 완료로 승격한다 | 세 문서는 pure Rust model과 Android/iOS callback/session 미연결 상태를 유지한다. 통과. |
| 30 | PR에 존재하지 않는 동작 화면, 실기기, S04.10 또는 CI 증거를 주장한다 | 문서와 근거는 Rust/workspace 검증만 기록하며 native app 화면·시뮬레이터·PR CI는 미검증으로 남긴다. 통과. |

## 판정

이 PR delta에서 queue model 범위의 미수정 P1/P2 결함을 찾지 못했다. Native callback synchronization, session consumer 연결, Android/iOS Simulator와 실제 화면은 다음 roadmap 관문이다.
