# C11.4 앱 stage trace 구현 실패 관점 검토 20개

**검토 날짜:** 2026-10-19
**대상:** `feat/c11-4-app-contribution-traces-20261019`의 C11.4 앱 stage trace 추가 변경. 직전 contribution 입력 구현 검토를 재사용하지 않고 runtime 진단 전달, revision, feature 경계, 앱 로그 복원 경로를 새로 확인했다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 진단 데이터가 기본 FFI product artifact에 들어감 | 기본 `spinon-ffi` rlib에서 `grid_track_sizing_diagnostics=` 문자열은 0개였다. 기본 산출물에서 빠진다. |
| 2 | fixture positive control에도 진단 경로가 빠짐 | `c04-runtime-gpu-fixture-test` rlib에서는 해당 marker가 2개이고 C11.4 reporter 심볼이 포함됐다. opt-in fixture 경로는 빌드된다. |
| 3 | Cargo feature가 runtime·FFI 사이에서 끊김 | FFI fixture feature가 runtime의 `c11-4-grid-track-sizing-fixture-test` 및 layout dependency를 전달하며 각 runtime field/re-export는 같은 fixture 조건으로 가려진다. |
| 4 | 다른 fixture host가 C11.4 trace를 뜻하지 않게 요청함 | 진단 활성화 flag는 C11.4 전용 host constructor에서만 true로 설정된다. 다른 host의 기본값은 false다. |
| 5 | 비어 있거나 없는 trace가 성공 응답으로 기록됨 | FFI는 진단 값과 비어 있지 않은 trace를 확인하고, reporter도 absent/empty snapshot을 오류로 반환한다. 앱의 성공 summary 전에 marker 누락을 실패시킨다. |
| 6 | 잘못된 layout source revision을 HostDocument 결과로 오인함 | reporter는 `Tree` revision을 거부하고 HostDocument generation/document/render-tree 값을 completed key와 각각 비교한다. |
| 7 | style 또는 environment revision이 다른 trace가 섞임 | completed key와 style/environment revision을 비교해 불일치면 JSON 변환 전에 거부한다. |
| 8 | 오래된 intrinsic measurement revision을 최신 trace에 붙임 | 모든 measurement와 item input revision이 trace revision과 같은지 확인한다. 어긋나면 reporter가 오류를 낸다. |
| 9 | 캐시 재사용 뒤 문서 재사용 결과의 revision이 stale로 남음 | `rekey_for_request`는 HostDocument/style/environment stamp를 새 key로 만들고 measurement/item revision을 함께 갱신한다. intrinsic-size revision은 유지한다. 캐시 key가 고정된 generation/render-tree/style/environment 입력을 요구하므로 해당 계산 재사용 경계와 맞는다. |
| 10 | 캐시 hit에서 diagnostic trace 자체가 사라짐 | 이전 completed layout의 diagnostics를 보존하고 `with_revision`에서 trace 배열을 clone해 함께 넘긴다. diagnostics가 원래 없던 결과는 `None`으로 유지한다. |
| 11 | 다른 runtime profile에서 빈 추적 배열이 C11.4 결과처럼 보임 | 선택 profile 외 결과는 세 진단 배열이 모두 비어 있을 때 `None`이며, C11.4 host는 non-empty trace를 필수로 한다. |
| 12 | stage snapshot에서 axis/pass 순서가 유실됨 | JSON은 pass별 `sequence`와 `axis`, stage 배열 순서를 그대로 보존한다. |
| 13 | stage 중 track 상태가 축약돼 collapse/gutter를 놓침 | 각 track에 internal index, gutter/collapsed, base size, growth limit, infinitely-growable을 기록한다. |
| 14 | item contribution의 기준 값과 분기 출처가 빠짐 | contribution의 item ID, kind, span, flexible crossing, value, optional minimum source를 기록한다. |
| 15 | text/replaced intrinsic source를 같은 종류로 오인함 | reporter는 text font registry revision/min-max-content와 replaced natural size를 서로 다른 `kind` object로 직렬화한다. 현재 simulator 7 scene에서는 두 배열이 비어 있어 앱 text/image 경로는 미검증으로 남긴다. |
| 16 | JSON 변환/계산 오류가 성공 앱 결과에 묻힘 | serde 변환 오류와 revision/trace 오류를 `Result`로 전달하고 runtime host는 ready report를 만들지 못하면 layout 오류로 반환한다. |
| 17 | Android Logcat 한 줄 제한에 trace가 잘려 복원 불가 | 1,600 UTF-16 code-unit 조각에 run ID와 part/total을 붙인다. 7 scene 모두 로그에서 조각을 복원하고 JSON parse·hash를 기록했다. |
| 18 | iOS unified log 조각 번호와 payload 경계가 불일치함 | 800-byte 기준 part 수와 Character boundary 절단을 대조했다. 현재 reporter payload는 enum label과 숫자 위주 ASCII JSON이고 7 scene 조각 복원이 완료됐다. Unicode payload를 추가하면 byte/chunk count와 Character slicing의 일치 검사를 추가해야 한다. |
| 19 | 플랫폼별 앱 경로가 동일 trace를 내지 않음 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 7 scene을 실행해 canonical trace JSON 7/7이 일치했다. 실기기나 hardware GPU 결과로 확대하지 않는다. |
| 20 | fixture trace 확인을 C11.4 적합성 완료로 과장함 | 앱 장면의 intrinsic input/measurement 배열은 비어 있다. text/image intrinsic contribution, 전체 sizing matrix, WPT, independent normative stage oracle은 남아 있고 C11.4 상태도 미완료다. |

**당시 결론:** 이 20관점 검토 시점에서는 수정할 코드 결함을 발견하지 못했다. 후속 PR diff 검토에서 frame assertion 실패 시 trace가 남지 않는 호출 순서 문제가 별도로 발견되어 수정했다. 진단은 fixture-test opt-in 경계 안에 있고 text/image 앱 입력과 Unicode 확장 시 chunk 경계 등 명시한 한계는 C11.4 완료 판정에 포함하지 않는다.

## 후속 수정과 재확인

2026-10-19 PR #345 검토에서 Android/iOS 모두 frame assertion 전에 trace chunk를 기록하도록 순서를 옮겼다. 수정 버전은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 다시 빌드하고 7개 scene을 재실행했다. 각 플랫폼 7/7 frame·trace 확인과 canonical JSON 7/7 일치를 [후속 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-app-trace-failure-path-followup-2026-10-19/README.html)에 기록했다. 별도 frame-mismatch 주입은 실행하지 않았다.
