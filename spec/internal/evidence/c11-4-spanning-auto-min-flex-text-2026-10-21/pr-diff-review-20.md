# PR diff 적대적 검토 20개

대상은 이 PR의 Rust fixture wiring/test, Android·iOS scene allowlist와 기대값, capture runner scene registry, 계획·공식 상태·public roadmap·실행 근거 변경이다. 구현/runtime의 20개 리뷰와 다른 변경 경계·통합 실패 관점으로 검사했다.

| # | 공격 관점 | 확인과 판정 |
|---:|---|---|
| 1 | fixture Rust 모듈이 일반 product module로 새어 나가는가 | 소스는 C11.4 fixture 하위 경로에만 추가되고 기존 fixture dispatcher에서만 연결된다. |
| 2 | Cargo 기본 build에 fixture entry point가 노출되는가 | runtime test와 diagnostics는 `c11-4-grid-track-sizing-fixture-test` gating을 사용한다. 이 PR은 product ABI를 추가하지 않는다. |
| 3 | FFI routing 순서가 기존 text/image scene을 가리는가 | 새 scene ID는 기존 adapters와 겹치지 않는다. 대표 unknown allowlist 및 scene tests가 통과했다. |
| 4 | 고정 HTML의 scene ID와 node IDs가 runner registry와 다른가 | FFI source, runtime validator, capture scene registry, Android/Swift 예상값의 ID·node order를 대조했다. |
| 5 | Chrome precomparison 입력과 실제 app source의 CSS가 다른가 | Grid 크기, 두 minmax flex track, 4px gap, Ahem 10px nowrap 20자와 span placement를 대조했다. |
| 6 | Rust test가 실제 runtime adapter 대신 수기 geometry를 쓰는가 | test는 HostDocument snapshot에서 `compute_request_for_block_flex_grid_track_sizing`을 호출하고 produced diagnostics·frame·stage를 읽는다. |
| 7 | Android scene allowlist만 열고 예상값 분기가 빠졌는가 | scene ID, root/item dimensions, intrinsic 200/200, node frames가 C0410 validator에 추가됐다. |
| 8 | Android text input expectation이 min-content만 검사하는가 | min/max content와 높이를 모두 200/10으로 검사한다. |
| 9 | Swift allowlist와 expected frames가 Android와 달라지는가 | 두 플랫폼의 root/item frames와 text intrinsic input을 서로 비교해 일치시켰다. |
| 10 | Simulator/device launcher가 새 scene을 미선택 상태로 두는가 | capture runner registry에 scene을 추가하고 `--scene-id`로 1 scene을 실행했다. |
| 11 | scene count 변경이 기존 150 scene을 조용히 삭제하는가 | 기존 150개 count·unique-ID 검증을 보존한 뒤 새 scene 추가 후 151 count와 unique-ID를 다시 검증한다. |
| 12 | 선택 scene 필터가 요청 오타를 빈 성공으로 취급하는가 | 실행 결과 `sceneCount=1`과 요청한 ID를 확인했다. 전체 suite count를 실행 결과로 가장하지 않는다. |
| 13 | 플랫폼 draw marker를 layout assertion과 혼동하는가 | 결과 JSON에서 frames/tracks/trace와 `drawPresented`를 분리해 보존한다. draw signal은 pixel equality로 해석하지 않는다. |
| 14 | PR 화면 링크가 잘못된 이미지나 생성물에 연결되는가 | Android의 성공 capture PNG를 직접 evidence로 연결했다. 첫 실패 run의 화면 대신 pass run 화면을 선택했다. |
| 15 | 계획 문서가 이 장면을 전체 C11.4 완료로 격상하는가 | 계획은 한 장면의 부분 slice로 서술하고 남은 범위·WPT·glyph paint 한계를 명시한다. |
| 16 | STATUS 완료 체크나 완료 집계가 부분 결과로 올라가는가 | C11.4 checkbox와 공식 완료 수는 그대로 두고 실기기 보충 기록만 추가한다. |
| 17 | public roadmap·Tailnet preview·official STATUS가 서로 다른 범위를 말하는가 | public roadmap에는 부분 검증 사실과 같은 미완료 표현만 추가한다. Tailnet 갱신은 merge된 원본으로 다시 빌드해 별도 검증한다. |
| 18 | 오래된 APK 재현 실패를 ADB 불안정으로 잘못 보고하는가 | 첫 log는 ADB transport 실패가 아닌 app allowlist argument rejection이며, build 전후 hash와 성공 run을 함께 기록한다. |
| 19 | 이전 roadmap 문서 commit이나 기존 evidence 작업이 PR에서 빠지거나 덮이는가 | 현재 branch의 roadmap commit을 유지하고, pre-existing untracked evidence는 stage 대상에서 제외한다. |
| 20 | check 실행 부재를 CI 통과라고 주장하는가 | PR 생성 뒤 `gh pr checks --required`의 실제 결과를 확인하고, 설정되지 않은 required gate는 통과로 보고하지 않는다. |

## 결론

diff 검토에서 공개 API 변화, 공식 완료 상태의 부당 승격, scene ID 충돌은 발견하지 않았다. 남는 핵심 제한은 이 fixture 한 장면만 확인했다는 점과 glyph/surface pixel을 검증하지 않았다는 점이다. 이 범위를 본문과 PR 설명에 유지한다.
