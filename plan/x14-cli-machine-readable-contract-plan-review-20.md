# X14 CLI 기계 판독 계약 계획 · 적대 검토 20개

- **대상:** [X14 계획 초안](./x14-cli-machine-readable-contract.md)
- **검토 기준:** `main` `1325ae5bbbd8eceb650129cb98829beed8659952`에서 분리한 문서 전용 worktree
- **검토 범위:** 계획의 의존성·계약 경계·입력/오류·정보 공개·자원 격리. 구현·CLI 실행·빌드·기기 검증이 아니다.
- **수행 방식:** dependency/contract lens가 1–7, file/scope lens가 8–14, resource lens가 15–20의 서로 다른 실패 관점을 읽기 전용으로 확인했다. 첫 검토 PARTIAL 3건은 X15 공통 envelope, X12/R05 profile source, offline/stale target의 승인되지 않은 fallback 표현이었다. 수정 후 관련 관점을 재검토해 모두 PASS로 판정했다.

| # | 적대 관점 | 발견 사항과 보정 | 최종 판정 |
|---:|---|---|---|
| 1 | S13 CLI dispatcher가 없는데 X14 plan을 구현 완료처럼 전제 | S13은 X14 구현·실행의 선행 경계로 두고, 계획 작성만 기존 문서를 acceptance case로 구체화하도록 분리했다. | PASS |
| 2 | X13 데이터가 없는데 `inspect` 성공 payload를 발명 | 공통 envelope 계획과 inspect payload를 분리하고 X13 revision·ownership 계약 전에는 성공 필드와 판정을 미해결로 남겼다. | PASS |
| 3 | X15가 같은 result/error model을 공유하는 조건을 무시 | 공통 envelope/version/error 결정은 CLI와 MCP의 안정된 구조화 결과에 함께 필요하다고 명시했다. X15 transport/resource/tool 구현은 별도 범위다. 첫 판정의 “X14가 차단하지 않는다” 모호성을 수정 후 재검토했다. | PASS · 재검토 |
| 4 | `profile`을 하나의 시간 척도로 뭉쳐 출처를 혼동 | X12 JavaScript CPU profile과 R05 frame/input attribution을 다른 profile kind로 구분하고 각각 source·clock·sampling window를 보류했다. 첫 판정에서 발견한 R05 단일 출처 서술을 수정 후 재검토했다. | PASS · 재검토 |
| 5 | CLI `inspect`가 Rust runtime 내부 자료를 소유한다고 가정 | command별 성공 데이터는 device provider, X13, X12, R05 등 실제 source owner에 연결하고 없는 source를 가정하지 않는다. | PASS |
| 6 | `docs/cli-mcp.md`의 제안 원칙을 이미 승인된 exact schema로 승격 | 공통 envelope·필드 이름·버전·null/omission 및 종료 코드 숫자를 후보와 미결 질문으로 유지한다. | PASS |
| 7 | X14를 versioned public API 또는 제품 지원 완료로 표시 | `STATUS.md`의 API 미작성과 미완료 상태를 보존하고 안정 호환성 주장은 차단 관문 뒤로 둔다. | PASS |
| 8 | 계획 범위에 CLI source, package manifest 또는 실제 command 변경이 섞임 | 현재 plan worktree 변경은 계획 문서와 그 20관점 검토 기록 두 Markdown뿐이다. 구현은 별도 후속 PR로 둔다. | PASS |
| 9 | 기존 `docs/cli-mcp.md`와 설명이 중복되어 불명확한 추가가 됨 | 기존 원칙을 반복하는 대신 명령별 acceptance 질문·실패 조건을 구체화하도록 범위를 제한했다. | PASS |
| 10 | 계획 및 계약 상대 링크가 없는 파일을 가리킴 | STATUS, `spec/0005-developer-tools.md`, `docs/cli-mcp.md` 링크가 모두 존재함을 확인했다. | PASS |
| 11 | X14가 MCP protocol/transport/write 기능까지 가져감 | MCP resource/tool·transport·쓰기 작업은 제외하고 shared result model만 X15와 조정한다. | PASS |
| 12 | 계획 파일이 공식 체크·완료 집계·계약 버전을 바꿈 | X14 상태, 공식 완료 수, 공개 명세 버전을 변경하지 않는다고 명시했다. | PASS |
| 13 | 숫자나 결과 key가 확정되지 않았는데 versioned contract를 기정 사실로 기록 | schema 이름·exact key·버전 호환·exit code mapping은 결정 관문으로 남겼다. | PASS |
| 14 | dirty C11 root나 lock/build output이 계획 작업에 결합 | 별도 `origin/main` worktree에서 계획 문서만 작성하고 source·manifest·lockfile·Cargo/V8/cache 변경을 금지한다. | PASS |
| 15 | 활성 C11.4 matrix와 CPU/build/cache 경합을 일으킴 | 계획은 Cargo target·runtime·V8/bootstrap output을 사용하지 않으며 C11 terminal HEAD/log 기록 전 구현·실행·build/test를 금지한다. | PASS |
| 16 | 실제 CLI 또는 자동화 검증을 실행한 것처럼 보고 | 문서는 구현·검증 미시작임을 밝히고 계획 단계의 CLI 실행·build/test를 제외한다. | PASS |
| 17 | 기기·Simulator·agent-browser를 조사 없이 실행 | 계획 단계에서는 device/provider·ADB·Simulator·browser 실행을 수행하지 않는다고 한정한다. | PASS |
| 18 | logs, paths, device IDs, arguments, env에서 secrets나 개인 정보를 노출 | redaction 분류, 출력 상한, truncation 표기 기준을 acceptance gate로 두고 미결 field를 출력하지 않게 한다. | PASS |
| 19 | streaming `logs`를 단일 JSON document로 가장하거나 반대로 처리 | finite result와 streaming/follow를 분리하고 별도 framing/종료 규칙 전까지 follow를 제외한다. | PASS |
| 20 | 명시한 offline/stale 대상에서 허가 없는 자동 fallback을 허용하거나 반대로 정책을 단정 | 대상 fallback 정책은 미결로 남기고, negative case는 승인된 contract 없이 다른 대상으로 자동 fallback하는 동작으로 한정했다. 첫 판정 문구를 수정한 뒤 재검토했다. | PASS · 재검토 |

## 최종 판정

위 20개 계획 실패 관점을 검토했다. 첫 검토에서 X15 shared envelope 경계, profile source 분리, offline/stale target fallback 문구가 모호하다는 부분 판정을 받아 계획을 보정하고 해당 관점을 재검토했다. 계획 문서만으로 실행 가능한 X14 CLI contract가 이미 결정됐다고 주장하지 않는다. S13 구현, X13 inspect payload, X12/R05 profile source, 공통 JSON/error compatibility 및 대상 선택 결정은 후속 관문이다.

검토자는 CLI command, build/test, package install, device/Simulator, agent-browser 또는 benchmark를 실행하지 않았다. 이 기록은 계획 검토이며 X14 구현·실행 근거가 아니다.
