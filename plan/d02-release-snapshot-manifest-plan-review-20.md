# D02 릴리스 스냅샷 매니페스트 계획 · 적대 검토 20개

- **대상:** [D02 계획 초안](./d02-release-snapshot-manifest.md)
- **기준:** `origin/main` `280c87507967331cc4775ec0824237dd614f2397`에서 분리한 문서 전용 worktree
- **검토 범위:** 계획의 의존성, 계약·상태 경계, 파일 범위, 입력·오류, 자원 격리. D02 구현이나 OTA 실행 검증은 아니다.
- **수행 방식:** dependency lens가 1–7, file/scope lens가 8–14, runtime/resource lens가 15–20을 독립적으로 검토했다. 발견한 부분 판정은 계획을 수정한 뒤 해당 관점을 다시 확인했다.
- **검토 기록 한계:** 부분 판정과 수정 후 재검토는 세 검토자의 이번 세션 검토 결과를 기록했다. Git commit에는 최종 문서만 담겨 있어 중간 초안 이력 자체를 증명하지는 않는다.
- **실행 제한:** build/test, agent-browser, 터미널 브라우저, ADB, iOS/Android Simulator를 실행하지 않았다. 변경은 D02 계획서와 이 검토 기록 두 Markdown뿐이다.

| # | 적대 관점 | 발견 사항과 보정 | 최종 판정 |
|---:|---|---|---|
| 1 | 내부 R15 graph 초안이 공개 D02 envelope 또는 구현된 loader로 오인됨 | graph body와 remote release envelope를 분리하고 R15를 내부 초안으로 한정했다. | PASS |
| 2 | 유효한 graph 구조만으로 원본 import/resource 누락까지 검출했다고 주장 | C02의 번들러 graph completeness fixture를 별도 입력으로 두며, D02가 이를 대신 검증한다고 하지 않는다. | PASS |
| 3 | D02 서명, D03 publish/CAS, D04 설치·high-water 책임이 섞이거나 거부 후 상태가 바뀜 | envelope 인증, publish head/CAS, client 활성 snapshot/high-water를 나누고 단계별 실패 불변 조건을 명시했다. | PASS |
| 4 | X01 로컬 graph manifest가 D02 remote release envelope와 같은 계약으로 합쳐짐 | 로컬 manifest와 release authorization의 포맷·책임을 구분했다. | PASS |
| 5 | `runtimeId`를 추정해 고정하거나 CSS/C11 완료를 D02 계획 선행조건으로 요구 | `runtimeId`를 현재 미정·불투명하게 두고 C11을 D02 계획의 선행조건으로 만들지 않았다. native/host/permission 변경은 새 바이너리 경계로 남겼다. | PASS |
| 6 | signed claim에서 channel, sequence 또는 rollback target이 빠져 replay/mix-up 가능 | 첫 검토에서 sequence/rollback target 서명 binding이 모호하다는 지적을 받았다. 전체 target tuple·`baseSnapshotId`·sequence·해당 시 rollback target을 서명 결정에 추가하고 D03 CAS를 같은 tuple로 분리해 재검토했다. | PASS · 재검토 |
| 7 | 기술적 호환·서명 성공을 App Store나 플랫폼 정책 승인으로 간주 | R09/D05의 출시 정책 gate와 기술 검증을 분리했다. | PASS |
| 8 | dirty root의 C11/C15/C19 source·lock·evidence 변경과 D02 문서가 섞임 | root WIP를 그대로 두고 `origin/main` 기준 별도 worktree에 D02 계획서와 이 검토 기록 두 파일만 추가했다. | PASS |
| 9 | 계획의 상대 링크가 없는 파일이나 잘못된 경로를 가리킴 | 공식 상태·0004·OTA design·R15 상대 링크가 존재함을 확인했다. | PASS |
| 10 | 계획이 STATUS 완료 수, API 또는 내부 contract version을 몰래 변경 | STATUS와 API/version 파일을 바꾸지 않았고 공식 완료 수·공개 API·numeric contract version을 유지한다고 적었다. | PASS |
| 11 | OS-conditional capability field 미결 상태가 빠져 unsupported API를 OTA 허용 | 첫 검토에서 capability 결정 질문 누락을 지적받았다. field 필요성, 원본·platform/OS mapping, 검사 시점·owner, unsupported rejection 범위를 추가했고 계약 전 0004 gate도 유지해 재검토했다. | PASS · 재검토 |
| 12 | 설계 입력을 승인된 wire schema, field name, serialization, signature algorithm으로 성급히 확정 | unresolved 결정으로 남겼고 R15 graph model을 별도 신규 schema로 복제하지 않았다. | PASS |
| 13 | C11 진행 중인 worktree의 로그·실행 결과를 D02 증거로 재사용하거나 C11 완료를 주장 | D02 계획 의존성을 점검하되 C11 구현·실행 근거는 가져오지 않았고 C11 완료 주장을 명시적으로 금했다. | PASS |
| 14 | 문서 초안을 구현, OTA 지원, 공개 호환성 완료로 표현 | 상태·목적·수행 범위를 계획 초안으로 제한하고 D02 구현/적합성은 미시작으로 명시했다. | PASS |
| 15 | 변조·digest/size 실패가 object fetch, cache, 활성 snapshot 또는 high-water에 부작용을 남김 | 검토 중 client fetch 경계, D03 CAS/head 무변경, post-publish 객체 검증이 순차로 보강됐다. 수신·압축 해제 후 원본 bytes를 검증하고 검증한 동일 bytes만 CSS/JS parser·evaluator·image/font consumer에 전달하며 mismatch면 후보 전체를 활성화 전에 거부하고 active snapshot/high-water를 보존하고 bad bytes를 재사용 가능한 staging/cache에서 격리·정리하도록 추가해 재검토했다. | PASS · 재검토 |
| 16 | duplicate key, oversized input 또는 ambiguous canonicalization이 signature 검증을 우회 | strict parsing, 크기 제한, 누락/형식 오류와 canonical byte 선택을 미결 결정 및 acceptance 질문에 두었다. | PASS |
| 17 | key rotation/revocation 중 stale cached trust로 된 envelope를 허용하거나 offline 정책을 추측 | key ID/trust/rotation/revocation/compromise와 stale cache·offline 가용성·복구 owner를 결정 항목으로 남겼다. | PASS |
| 18 | 다른 channel/platform/runtime sequence가 소비되거나 publish와 install high-water의 scope가 다름 | signed sequence를 channel 포함 전체 target tuple 및 base에 바인딩하고 D03 CAS/head와 D04 high-water의 per-target·per-stage 불변을 구분했다. | PASS |
| 19 | 오래된 envelope를 rollback으로 재생하거나 restore/reinstall이 anti-replay 상태를 낮춤 | 새 higher-sequence rollback authorization, 만료/1회 소비/target binding, restore/reinstall 및 high-water 복구 경계를 D02/D04 결정으로 남겼다. | PASS |
| 20 | `affectedScope`/feature closure claim이 바뀌거나 기술 검증을 rollout 허가로 승격 | 영향 범위의 서명·D03 검증·불확실 시 app 범위 처리 질문을 두고, source completeness와 R09/D05 policy gate를 각각 분리했다. | PASS |

## 최종 판정

20개의 서로 다른 계획 실패 관점을 검토했다. sequence/rollback signing, OS capability, 서명 거부 시 단계별 상태 불변, 수신·cache 객체 bytes digest/size 및 격리 경계에서 발견된 부분 판정은 계획에 반영하고 각 영향을 받는 관점에서 재검토해 통과했다. 이 기록은 D02 계획 검토일 뿐 구현, wire contract, OTA 기능 또는 플랫폼 승인 근거가 아니다.
