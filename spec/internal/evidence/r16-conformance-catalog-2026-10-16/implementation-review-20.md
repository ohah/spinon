# R16 적합성 카탈로그 구현 적대 검토 · 2026-10-16

검토 대상은 [0009 적합성 시나리오 카탈로그](../../../0009-conformance-scenarios.md), `0001`/`README` 연결과 R16 상태 변경이다. 이 문서 검토는 시나리오 실행이나 제품 적합성 결과가 아니다.

| # | 실패 관점 | 확인 근거·검사 | 결과 |
|---:|---|---|---|
| 1 | R16 체크가 runtime 기능을 뜻하는 것처럼 읽힘 | 상태 문구에 명세 산출물만 완료, 시나리오 7개 미실행, API 0개를 함께 기록 | 통과 |
| 2 | 0009 계약 숫자가 출시/호환성 버전으로 올라감 | 0001–0009 전부 `0.1.0-draft`, 출시 전 숫자 `0.1.0` 고정 | 통과 |
| 3 | 0001–0008 중 문서 ID·소유 영역이 빠짐 | 인덱스 표를 파일 첫 메타데이터·각 문서 목적과 대조 | 통과 |
| 4 | 0009가 0002/0007 DOM 규범을 복제 | 표는 owner mapping만 두고 DOM mutation 규칙은 소유 문서로 연결 | 통과 |
| 5 | 내부 HostDocument/FFI evidence가 공개 DOM 지원으로 승격 | 공개 문서와 internal ABI를 분리하고 내부 근거는 후보 입력으로만 사용 | 통과 |
| 6 | 첫 적합성 대상 프레임워크/OS를 정식 정책으로 선언 | 모든 framework/OS/첫 release 선택을 미정 등록부에 둠 | 통과 |
| 7 | 시나리오 결과가 API index의 지원 API 수를 바꿈 | `spec/api/index.md` 변경 없음, 0001과 0009에서 미실행을 분리 | 통과 |
| 8 | SC ID가 기존 fixture/task ID와 충돌 | `SC-` prefix와 일곱 ID의 중복 여부 확인 | 통과 |
| 9 | CSS pixel 기준이 플랫폼 표시 전체에 과장 | C08 고정 Chrome oracle·pre-compositor capture로 한정, color policy 미정으로 표시 | 통과 |
| 10 | SC-UI 혼합 자식 순서가 입력 HTML과 불일치 | 고정 입력 `text, span, text, em`의 실제 child node 순서와 expected text 비교 | 통과 |
| 11 | accepted tap과 canceled pan의 입력 계약을 동일시 | SC-INPUT의 결과 조건을 분리하고 미정 event order는 실행 차단으로 기록 | 통과 |
| 12 | JS handler 완료와 GPU 표시 callback을 같은 시점으로 간주 | SC-ASYNC에서 handler/microtask/frame marker와 revision을 분리 | 통과 |
| 13 | 미지원 CSS negative control이 조용히 기본값으로 대체 | SC-DIAG는 `position: sticky`의 원본 위치/원인 진단과 block positive control을 요구 | 통과 |
| 14 | surface 재생성에서 document revision을 임의 갱신 | SC-LIFE는 document/environment/surface generation·revision을 각각 보존 | 통과 |
| 15 | accessibility action을 시각 tap/click과 동일시 | SC-A11Y role/name/action을 분리하고 TalkBack/VoiceOver 결과는 미실행·미정 | 통과 |
| 16 | 기존 simulator/Chrome component evidence를 새 SC 통과로 계산 | 각 시나리오에서 기존 evidence는 oracle 출처일 뿐이라고 명시 | 통과 |
| 17 | negative control 없이 happy path만 통과 | 모든 SC에 잘못된 입력·stale/cancel/diagnostic control을 별도 정의 | 통과 |
| 18 | 버전·범위 결정을 R16 문서가 대신 확정 | 미정 등록부가 소유 문서·영향·결정 전 금지 주장을 기록 | 통과 |
| 19 | API index/HTML roadmap 상태가 수동 중복 변경 | API index는 그대로 두고 roadmap은 공식 STATUS의 R16 상태를 source로 사용 | 통과 |
| 20 | 링크/실행 한계가 모호해 문서 완성을 runtime 검증으로 오독 | 계획·근거·미실행 결과를 연결하고 RSPress와 Tailnet은 별도 검증 | 통과 |

검토 결과: 계획의 일곱 시나리오와 미정 계약 등록부가 모두 연결됐다. Chrome oracle 및 기존 앱 근거는 scenario PASS로 표시하지 않았다. 구현·문서 변경만 검토했으며 Android/iOS 앱·simulator는 실행하지 않았다.
