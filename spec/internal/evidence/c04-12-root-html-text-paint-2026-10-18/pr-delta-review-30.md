# C04.12 direct text paint PR diff 검토 30개

구현 실패 관점 검토와 겹치지 않도록 최종 diff의 파일 경계, feature 전달, 문서·실행 근거 연결을 대상으로 확인했다. 각 항목의 현재 diff 상태를 확인했으며, 기능 동작은 연결된 test/build/readback 결과와 함께 대조했다.

| # | 변경 경계 | 확인 |
|---:|---|---|
| 1 | `spinon-runtime` Cargo feature가 parent Block fixture와 text layout dependency를 opt-in으로만 켬 | 통과; 기본 `spinon-runtime`에는 optional dependency가 활성화되지 않음. |
| 2 | `spinon-style-to-layout` text projection API가 dedicated feature 안에 있음 | 통과. |
| 3 | FFI Cargo feature가 runtime GPU, Metal readback, root HTML Block profile과 연결됨 | 통과; dependency chain 명시. |
| 4 | C header의 두 text fixture 함수 선언이 같은 feature define으로 보호됨 | 통과. |
| 5 | FFI 생성/eval 함수가 null host, null/zero output buffer를 처리함 | 통과; 안전 계약 주석 확인. |
| 6 | FFI 생성 실패가 report buffer에 원인을 쓰고 null handle을 반환함 | 통과; 기존 `write_report` 경로 사용. |
| 7 | FFI eval은 다른 host 종류를 임의로 허용하지 않고 입력 포인터 검증을 거침 | 통과; eval에서 text fixture host flag를 확인하며 helper의 true/false 판정을 테스트했다. 다른 종류의 live host를 전달하는 FFI 통합 호출은 test process V8 isolate 초기화 실패 때문에 실행하지 못했다. |
| 8 | Rust module index가 dedicated `fixtures/` 경로를 사용하고 orphan source가 생기지 않음 | 통과; 모듈 선언·include 경로 대조. |
| 9 | runtime fixture HTML이 기존 root parser 진입점에서만 HostDocument가 됨 | 통과; 생성 경로 확인. |
| 10 | session profile 분기가 Cargo feature에 가려져 기본 session selector에 노출되지 않음 | 통과. |
| 11 | C04.12 viewport metadata 입력이 text profile에서 새 의미로 바뀌지 않음 | 통과; 고정 viewport/DPR만 gate. |
| 12 | text property profile이 기존 root Block property list를 보존하며 필요한 값만 추가함 | 통과; 기존 Block 기능 회귀 workspace tests 통과. |
| 13 | unsupported author stylesheet feature/diagnostic이 부분 cascade로 이어지지 않음 | 통과; runtime cascade error path 재사용. |
| 14 | computed `color`가 root background paint carrier를 잘못 재사용하지 않음 | 통과; text profile은 computed text color만 전달. |
| 15 | layout text input이 일반 `RuntimeRootHtmlBlockPaint` 경로에 적용되지 않음 | 통과; dedicated profile과 feature branch 사용. |
| 16 | collapsible whitespace를 무시하는 HostDocument layout 생성자가 일반 C15 입력을 변경하지 않음 | 통과; 별도 profile-specific constructor. |
| 17 | renderer document order에서 기존 Block fixture text 거부 동작이 유지됨 | 통과; whitespace handling과 text profile 분기를 분리. |
| 18 | runtime text clip이 owner overflow clip과 일치함 | 통과; scene attach 뒤 equality 검사 및 simulator pixel 대조. |
| 19 | text command가 owner box 뒤에만 삽입되고 중복/missing owner가 실패함 | 통과. |
| 20 | glyph snapshot의 owner key, CSS origin, DPR, color, bitmap bytes가 새 scene에 보존됨 | 통과; `RuntimeRenderText` constructor로 검증. |
| 21 | FFI native surface sample이 reported surface dimensions에 기반함 | 통과; CSS viewport와 physical surface를 비율 투영. |
| 22 | fractional center rounding과 viewport 경계가 고정됨 | 통과; projection positive/negative unit tests 2개. |
| 23 | iOS readback이 renderer generation과 drawable 크기 변경 뒤 stale report를 받지 않음 | 통과; `isCurrentSurfaceGeneration` 및 size gate 확인. |
| 24 | iOS readback parser가 표본 개수·좌표·RGBA 타입과 exact 값을 확인함 | 통과; Swift compile 및 실제 5/5 run. |
| 25 | iOS 실패 뒤 pass status를 표시하지 않음 | 통과; readback failure는 operation error로 전파. |
| 26 | Android route가 fixture BuildConfig off 상태에서 host 호출을 성공 처리하지 않음 | 통과; feature check 및 native disabled error path. |
| 27 | Android source set의 PixelCopy adapter가 fixture flag에만 포함됨 | 통과; Gradle 조건과 output report 대조. |
| 28 | Android asynchronous capture가 generation/renderer/size/run identity를 비교함 | 통과; current-source predicate 및 PixelCopy 결과 로그 확인. |
| 29 | Android/iOS build script, native define, Cargo feature spelling이 동일함 | 통과; `rg`, `bash -n`, iOS `plutil`과 두 app builds. |
| 30 | 공식 대장은 하위 fixed-font slice만 기록하고 C04.12/C15 parent는 미완료로 둠 | 통과; contract `0080`, status row, evidence README가 같은 한계를 표시. |

최종 실행에서 발견된 결함은 해당 변경에 반영됐다. FFI fixture-kind guard는 helper 단위 테스트로 확인했으며, `RuntimeGpuHost`를 생성하는 해당 FFI test process는 V8 isolate 초기화에 실패해 wrong-kind live-host 호출까지 검증하지 못했다. 표준 workspace Clippy의 단일 `too_many_arguments` baseline은 `origin/main`과 동일한 함수 서명에서 나온다. 신규 feature의 runtime/FFI Clippy는 그 기존 lint만 제외하고 통과했다.
