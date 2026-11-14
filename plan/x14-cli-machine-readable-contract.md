# X14 CLI 기계 판독 계약 계획

- **상태:** 계획 초안 · 구현·검증 미시작 · X14 미완료 유지
- **공식 항목:** [X14 CLI 기계 판독 계약](../spec/STATUS.md)
- **제안 계약:** [0005 개발 도구](../spec/0005-developer-tools.md) · [CLI·MCP 설계 초안](../docs/cli-mcp.md)
- **범위 경계:** 이 문서는 계약을 구현하거나 공개 지원을 선언하지 않는다. 결정이 필요한 값은 후보·중단 관문으로 남긴다.

## 목적

X14는 `devices`, `logs`, `inspect`, `profile`, `doctor` CLI 명령의 기계 판독 결과를 자동화가 안정적으로 소비할 수 있게 한다. X14 계획은 이미 제안된 공통 JSON 결과, 버전, 종료 코드, 오류 분류를 각 명령에 적용할 acceptance case로 구체화한다. 새 기능 구현이나 현재 CLI가 이미 존재한다는 주장은 포함하지 않는다.

사람을 위한 기본 출력과 `--json` 출력은 별도 표면으로 취급한다. JSON schema와 오류 분류를 버전 관리하고, 비밀이나 불필요한 개인 경로를 포함하지 않는다는 경계는 [CLI·MCP 설계](../docs/cli-mcp.md)에 이미 제안되어 있다. 이 계획은 그 원칙을 반복 서술하는 대신, 누락·오류·대상 선택 상태에서 검증할 결과를 명시한다.

## 선행 관계와 독립 범위

| 항목 | 계획에서 취급하는 관계 |
|---|---|
| S13 최소 CLI | `create`, `dev`, `build`, `doctor`를 제공하는 첫 CLI 수직 경로다. X14 구현과 실행은 S13의 실행기·명령 등록·기본 출력 경계가 정해진 뒤에 한다. 계획 작성은 기존 명세를 구체 acceptance case로 바꾸므로 S13 구현을 요구하지 않는다. |
| X13 UI 트리 조사 | `inspect tree`/`inspect node`의 데이터 의미와 revision 소유자에 필요한 데이터 원천이다. 공통 envelope·오류·대상 선택 계획은 진행할 수 있으나, inspect payload 필드와 성공 출력 판정은 X13 데이터 계약 전까지 unresolved로 둔다. |
| X15 로컬 MCP 서버 | CLI와 같은 진단 결과 모델·오류 코드를 재사용한다. 공통 envelope/version/error 호환성 결정은 CLI와 MCP의 안정된 구조화 결과에 함께 필요하다. X15의 transport, resource/tool mapping, 쓰기 작업은 별도 구현 범위이며 이 계획에 넣지 않는다. |
| X12 V8 Inspector 연결 | Inspector 개발 경로는 JavaScript debugging·profiling 데이터를 제공할 수 있다. `profile`의 JS 프로파일 payload 후보지만 X12의 데이터 shape·수명·clock source가 먼저 정해져야 한다. |
| R05 계측·profile 자료 | R05는 frame/input attribution 계측의 후보 source이며 X12 JavaScript CPU profile과 다른 결과다. `profile` command에서 어떤 profile kind를 지원할지, 각 시간 범위·clock domain을 별도로 정하기 전에는 payload를 미확정으로 둔다. |
| C11.4 현재 matrix | 이 문서만 추가하는 계획 작업은 C11 Cargo target, 런타임 코드, V8/bootstrap output을 사용하지 않는다. C11 terminal result, 최종 HEAD와 log hash를 기록하기 전 X14 CLI 구현·실행·빌드·테스트는 시작하지 않는다. |

## 계약 질문과 후보

| 주제 | 계획에서 고정할 acceptance 질문 | 미해결이면 |
|---|---|---|
| 공통 envelope | 성공·오류 모두 어떤 schema identifier/version을 내며, command identity와 결과/오류를 어떻게 구분하는가? 필드 생략과 `null`, unknown field, 추가 필드의 호환 규칙은 무엇인가? | envelope·버전 호환 정책이 결정될 때까지 `--json`을 안정 계약으로 구현하지 않는다. |
| stdout/stderr | `--json`에서 stdout에 기계 파싱 가능한 값만 쓰고 human progress·진단은 어디에 두는가? 한 번의 명령 결과와 streaming log를 같은 형식으로 오해하지 않게 하는가? | 초기 범위에서 streaming/follow를 제외하거나 별도 형식과 종료 규칙을 먼저 결정한다. |
| 오류와 종료 코드 | 잘못된 인자, 환경 의존성 누락, 대상 없음/모호함/offline, 세션 만료, 명령 실패, 내부 오류를 안정적으로 어떻게 분류하는가? 종료 코드와 JSON error code가 일치하는가? | 분류표와 exit-code mapping이 승인되기 전 외부 자동화 호환성을 주장하지 않는다. |
| 대상 선택 | `--device`/session 대상이 없거나 여러 후보가 있을 때 자동 선택·오류·대화형 prompt 중 어떤 동작인가? 명시한 대상이 offline/stale이면 다른 기기로 fallback하는가? | 하나의 비대화형 규칙이 정해질 때까지 대상 의존 명령의 성공 계약을 미정으로 둔다. |
| 정보 공개 | device id, path, app log, arguments, environment 값에서 민감한 값을 무엇으로 분류·제거하는가? 길이·레코드 수 제한과 truncation 표시는 어떻게 하는가? | redaction 및 bounded-output 실패 행렬이 없으면 해당 데이터 필드를 출력하지 않는다. |
| 명령 payload | `devices`, `logs`, `doctor`의 고정 관찰 필드가 무엇인가? `inspect`는 X13, JS `profile`은 X12, frame-attribution profile은 R05 중 어느 source와 clock 계약을 쓰는가? | X13/X12/R05 source contract가 없는 payload는 undocumented placeholder나 예시 값으로 만들지 않는다. |

공통 envelope 후보는 최소한 schema 이름·버전, command identity, 성공/실패 상태, data 또는 structured error와 제한된 diagnostics가 필요할지 검토한다. 이 목록은 승인된 JSON schema가 아니다. 정확한 key 이름, versioning policy, `null`/omission 의미, 추가 필드 호환성, exit code 숫자는 구현 전에 별도 검토해 결정한다.

## 명령별 검증 범위

| 명령군 | 계획이 다룰 acceptance case | 외부 의존 또는 보류 경계 |
|---|---|---|
| `devices` | 기기 0개·1개·여러 개, offline/unavailable, 명시 id 불일치의 JSON envelope, 정렬·대상 선택·오류 코드. | 실제 device provider의 stable id, platform 및 연결 상태 출처를 먼저 고정한다. 실기기 연결이나 ADB/Simulator 실행은 계획 단계에 포함하지 않는다. |
| `logs` | 빈 로그, 여러 source, 잘못된/역전된 time range, 결과 상한 초과의 schema·truncation·진단. | 민감정보 제거, 시간대/clock 의미, 세션 식별과 로그 크기 한도를 정한다. Follow/streaming은 별도 결정 전 제외한다. |
| `doctor` | 모든 전제 정상, 각 필수 도구 누락, 버전 범위 불일치, 조회 자체 실패의 개별 진단과 최종 결과. | [0005](../spec/0005-developer-tools.md)의 read-only 진단을 유지한다. 설치·환경 자동 수정은 포함하지 않는다. |
| `inspect tree` / `inspect node` | argument 오류와 stale/unknown id 결과의 공통 error envelope. 성공 payload의 field/assertion은 X13 revision·ownership 계약이 마련된 뒤 정한다. | X13 계약 전에는 node, computed style, layout, accessibility 정보의 성공 schema를 정의하지 않는다. |
| `profile` | 잘못된 시간 범위·대상·지원되지 않는 profile kind에 대한 안정된 오류 envelope. | X12의 JS CPU profile과 R05의 frame/input attribution을 구분한다. 각 source·clock domain·sampling window·platform coverage가 정해지기 전 성공 payload 및 성능 주장을 보류한다. |

명령마다 human mode, JSON mode, stderr 진단, exit code, redaction, unknown target 동작을 함께 기록한다. 오류를 성공 JSON처럼 출력하거나 stdout human text를 JSON parser에 섞는 경우, 명시한 대상이 offline/stale일 때 승인된 contract 없이 다른 대상으로 자동 fallback하는 경우, 잘린 로그를 완전한 결과로 표시하는 경우는 negative case다. 실제 출력 예제는 contract 결정 전 고정하지 않는다.

## 계획 단계 및 증거

1. `spec/0005-developer-tools.md`, `docs/cli-mcp.md`, S13, X13, X14, X15 및 관련 R05 명세에서 이미 정해진 주장과 미결 결정을 표로 분리한다.
2. 각 command의 입력·대상 선택·성공/오류·stdout/stderr·exit code·redaction·bound를 기록하는 acceptance matrix 초안을 만든다. 아직 존재하지 않는 CLI 구현, 기기, session 또는 profile 데이터를 가진 것처럼 예시를 쓰지 않는다.
3. 공통 envelope/version/error compatibility 정책을 구현 전에 결정해야 하는 차단 관문으로 둔다. `inspect` payload는 X13, JS profile payload는 X12, frame/input attribution payload는 R05가 소유하는 데이터 경계에 연결한다.
4. 서로 다른 20개 실패 관점으로 이 계획 초안을 검토했다. [계획 검토 기록](./x14-cli-machine-readable-contract-plan-review-20.md)에 초기 판정, 수정 및 영향 관점 재검토를 남겼다. 이 검토는 구현·CLI 출력·기기 검증이 아니다.
5. 구현 PR은 X14 계약 명세·예제·실제 CLI 결과를 함께 연결하고, 새 구현의 별도 코드/runtime 실패 관점 검토와 PR 변경분 검토를 수행한다.

계획 단계에서 실행하지 않을 항목은 CLI build/test, package install, `spinon` 명령 실행, Android/iOS device 또는 simulator launch, agent-browser, runtime benchmark다. 지금 상태는 문서 준비이며 X14 완료 증거로 사용할 수 없다.

## 중단 조건과 완료 경계

- 기존 `docs/cli-mcp.md`나 `spec/0005`와 충돌하는 envelope/version 결정이 발견되면 구현을 진행하지 않고 변경 제안을 먼저 명세에 기록한다.
- S13 command dispatcher 또는 X14 command input/output ownership이 없으면 contract를 존재하는 동작인 것처럼 기술하지 않는다.
- `inspect` payload는 X13, JS profile payload는 X12, frame/input attribution payload는 R05의 source와 clock boundary가 고정될 때까지 각각 미해결이다.
- log/device 결과의 redaction, output bounds, stale target handling과 process exit semantics를 고정할 수 없으면 해당 field/command의 지원 주장을 중단한다.
- C11.4가 활성인 동안 계획 파일 밖의 source, package manifest, lockfile, build output, Cargo/V8/cache 및 기기 자원은 수정·사용하지 않는다.

이 계획은 [상태 대장](../spec/STATUS.md)의 X14 체크, 공식 완료 수, 공개 명세 버전을 변경하지 않는다. 구현 시작 전 후보 contract decision과 계획 검토 결과가 준비되어도 X14는 미완료이며, CLI·MCP 기능이 동작한다고 주장하지 않는다.
