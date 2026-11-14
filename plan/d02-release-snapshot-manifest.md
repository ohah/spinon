# D02 릴리스 스냅샷 매니페스트 계획

- **상태:** 계획 초안 · D02 구현·적합성 검증 미시작 · 공식 완료 미표시
- **공식 항목:** [D02 릴리스 스냅샷 매니페스트](../spec/STATUS.md)
- **기준 문서:** [0004 실행·빌드·배포](../spec/0004-runtime-build.md) · [OTA 설계](../docs/ota-design.md) · [R15 그래프 호환 모델](../spec/internal/0013-r15-ota-chunk-compatibility.md)
- **범위:** 기존 제안의 결정 소유자, 미결 정책, 선행 입력과 acceptance 질문을 연결한다. wire schema, signature algorithm, key lifecycle 또는 OTA 허용 범위를 여기서 확정하지 않는다.

## 목적과 기준 경계

R15는 앱·플랫폼·ABI·`runtimeId`, graph schema, 기능 진입점과 청크·자원 edge, 객체 digest/size로 이루어진 목표 graph 본문을 제안한다. 로컬 loader와 원격 릴리스가 이 본문을 공유하되, 원격 릴리스는 별도 release ID·sequence·서명 envelope를 덧붙인다. R15는 내부 graph model 초안이며 공개 OTA schema나 실행 가능한 배포 기능이 아니다.

D02의 일은 이 graph를 참조하는 인증된 release envelope와 호환·anti-replay 결정을 정의하는 것이다. C02의 완전한 번들 의존성 추출이 원본의 누락 import를 막는지 증명해야 하며, graph 자체 검사만으로 소스 누락을 찾았다고 주장할 수 없다. D03은 channel/cohort 배정과 publish CAS, D04는 다운로드·저장·원자 활성화·health/rollback 소비를 소유한다.

## 이미 제안된 기준과 아직 정할 항목

| 경계 | 기존 문서에 적힌 기준 | D02 계획에서 남길 결정 |
|---|---|---|
| Graph 본문과 release envelope | 로컬·원격은 같은 graph 본문을 사용하고 원격 release 정보·서명은 별도 envelope에 둔다. | release schema/version, envelope byte representation, graph body와 서명 payload의 연결을 확정한다. 로컬 개발 manifest에 원격 배포 승인을 암시하지 않는다. |
| 호환 판정 | graph schema의 정확한 지원 버전과 바이너리가 제공하는 불투명한 `runtimeId`를 exact match한다. 숫자 버전 범위를 추측하지 않는다. | 앱·플랫폼·ABI·graph schema·`runtimeId` claim을 어떤 단계와 diagnostic으로 검증할지 D02/X01/D04 사이에서 나눈다. 현재 또는 후보 `runtimeId` 값은 고정하지 않는다. |
| OS 조건부 capability | 0004는 capability 요구·검사 계약이 정해지기 전 OTA chunk가 OS 조건부 API를 사용하지 못하게 한다. STATUS는 release manifest에 capability field가 필요한지 미결로 둔다. | D02는 field 필요성, capability 식별 원본, platform/OS 매핑, 검사 시점·owner와 미지원 거부 범위를 결정한다. schema field 이름을 미리 만들지 않으며, 계약 전에는 해당 API를 OTA chunk에서 사용하지 않는 gate를 유지한다. |
| 바이너리 대 OTA 범위 | native code, V8, GPU renderer, host API 또는 permission 변경에는 새 앱 바이너리가 필요하다. | release metadata가 이를 어떻게 나타내고 unsupported change를 어디서 거부할지 정한다. 서명된 OTA가 바이너리 경계를 우회한다고 쓰지 않는다. |
| Feature 영향 범위 | R15의 `affectedScope=features|app`, graph closure diff와 앱 설치 단위 cohort 제안. 안전한 diff가 증명되지 않으면 app 범위로 넓힌다. | `affectedScope`, `affectedFeatureIds`, `baseSnapshotId`, changed intent 가운데 어떤 값이 release 서명 대상이며 D03 publish 판정에 쓰이는지 결정한다. |
| Channel 및 동시 publish | channel head/CAS는 app·platform·ABI·runtime·channel 타깃별이며 stale base는 publish를 거부한다. STATUS/R15는 대상 channel과 `baseSnapshotId`를 서명 범위에 둔다. | D02는 `applicationId`·platform·architecture/ABI·`runtimeId`·channel의 전체 target tuple, `baseSnapshotId`, sequence, 해당 시 rollback target을 서명에 결합하고, D03은 같은 tuple별 CAS를 실행한다. channel 승격·이동은 별도 권한으로 분리한다. |
| 객체 무결성 | graph는 원본 객체의 SHA-256과 byte size를 참조한다. 후보 재사용도 적용 전 다시 검증하고 소비 전에 읽은 bytes를 검증한다. | envelope 인증과 object digest 검증의 관계, 입력 제한, 실패 순서·진단 owner, 검증된 동일 bytes가 parser/evaluator/resource consumer로 전달되는 보장, hash/size가 틀린 객체의 cache/staging 격리·정리 경계를 정한다. manifest 서명만으로 C02 graph completeness를 주장하지 않는다. |
| Strict parsing | 0004는 strict parsing과 오류 계약을 D02에서 정할 항목으로 남긴다. | 중복 key, 누락·중복 필드, 잘못된 타입, oversized input, 미지원 필드/schema의 거부 정책과 canonicalization 선택을 정한다. 구체적인 serialization이나 알고리즘은 승인 전 선택하지 않는다. |
| Key와 signature | R15/0004는 signature envelope와 key 운영을 제안하지만 공개 형식과 알고리즘은 미정이다. | key ID/version, 신뢰 anchor, rotation·revocation·compromise, 서명 생성 주체와 보관 경계를 누가 정하고 검증하는지 선택지·위협 모델로 정리한다. 안전한 key storage 방식을 추정하지 않는다. |
| Sequence와 replay | 타깃별 sequence/high-water exact order, 오래된 envelope replay 거부, built-in snapshot 별도 복구를 제안한다. | sequence scope를 channel을 포함한 전체 target tuple에 묶고 서명하는 방법, 발급자 권한, 상태 소유·영속화·복원 후 보장 및 D03 publish와 D04 install 간 race 책임을 정한다. |
| Rollback authorization | OTA rollback은 높은 sequence의 새 signed command이며 과거 envelope 재생은 허용하지 않는다. offline authorization은 미정이다. | 새 rollback command의 대상·만료·1회성 소비·channel/runtime binding, offline preauthorization 선택지와 D04의 소비 응답을 함께 정한다. authorization이 없을 때 high-water를 낮추지 않는 경계는 보존한다. |
| Restore/reinstall | app data restore·clear·reinstall 뒤 anti-replay state가 되돌아갈 수 있는 위협 경계는 D02/D04에 남아 있다. | 앱 상태 복원, 기기 백업, 재설치, keychain/secure-storage 생명주기별 보장·비보장과 재등록/복구 절차를 정책 owner와 결정한다. OS 저장소의 영속성을 가정하지 않는다. |
| 플랫폼 정책 | 기술적인 graph/signature 검증은 앱별 store policy 적격성·심사 승인이 아니다. R09/D05가 출시 시점 정책을 검토한다. | iOS·Android 정책 판단을 D02의 기술적 서명 성공으로 대체하지 않는다. R09/D05의 정책 gate와 release approval owner를 연결한다. |

## 소유자와 실행 선행 조건

| 작업 | D02 계획에서 필요한 연결 | 계획 작성과 구현 구분 |
|---|---|---|
| R15 graph model | 기존 typed graph와 정확한 `runtimeId` 호환 기준을 입력으로 삼는다. R15 초안을 완성된 loader/API로 취급하지 않는다. | 이 model을 읽고 미결 항목을 매핑하는 계획은 지금 가능하다. |
| C02 / X02 / X03 | 번들러에서 해석된 JS/CSS/asset graph의 완전성·원본 위치와 fixture를 제공한다. | D02 plan은 빠진 import를 판정했다고 주장하지 않는다. 구현·end-to-end publish 판정은 graph completeness evidence를 기다린다. |
| X01 | 로컬 graph manifest와 referrer/specifier에 따른 ESM 해결, 모르는 graph schema 거부를 소유한다. | D02 release envelope를 X01의 로컬 manifest 포맷에 합치지 않는다. |
| D03 | publish, 타깃별 channel/cohort, base/head CAS 및 배포 객체 수명을 소유한다. | 서명된 claim의 소유·검증과 CAS 실행·경합을 분리한다. |
| D04 | 다운로드·크기 한도·기존 객체 재검증·staging·원자 활성화·health·rollback 소비를 소유한다. | envelope claims의 parse/authentication과 설치 상태 기계의 동작을 각각 검증한다. |
| R09 / D05 | App Store/플랫폼 허용 범위와 출시 시점 정책을 재검토한다. | 기술적으로 signed/compatible인 후보를 정책상 허용으로 표시하지 않는다. |
| X08/J15 및 CSS/runtime 변경 | host API·permission/native module 변화의 new-binary 경계를 유지한다. CSS/runtime의 의미 있는 변화는 이후 `runtimeId` 산정에 영향을 줄 수 있다. | C11 완료를 D02 계획의 선행조건으로 만들지 않고, 미정/가변 `runtimeId`를 이번 계획에서 숫자로 고정하지 않는다. |

## 계획 acceptance matrix

아래는 결정해야 할 fail-closed 관찰값과 소유자를 지정하기 위한 질문이다. 실제 envelope, parser, release pipeline의 실행 통과 결과가 아니다.

| 입력·실패 상황 | 계획에서 정할 검증·거부 경계 |
|---|---|
| 서명 payload byte 또는 signed `baseSnapshotId`/target tuple/channel/sequence/rollback-target claim 변조 | client 전달 후 변조 후보는 envelope 서명·target 호환 검증을 content object fetch 전에 하고, content object fetch·활성 snapshot·D04 client high-water 변경 없이 거부한다. D03이 받는 publish 후보의 signature/base/target 검증 실패는 channel head·CAS 변경 전에 거부한다. post-publish 객체 변조 검사는 이 client 경계와 분리해 owner를 정한다. |
| 수신·압축 해제·cache 재사용 중 객체 bytes가 signed digest/size와 다름 | D04가 압축 해제 후 원본 전체 bytes를 검사하고, 검증한 동일 bytes만 CSS/JS parser·evaluator·image/font decode 등 어떤 resource consumer에도 전달하도록 보장한다. mismatch면 후보 전체를 활성화 전에 거부하고 활성 snapshot·high-water를 유지하며 잘못된 객체를 재사용 가능한 cache/staging에서 격리·정리할 owner를 정한다. |
| app·platform·ABI·`runtimeId`가 설치 대상과 다름 | signature-valid라도 호환 검사에서 거부하고 다른 타깃의 sequence/channel을 소비하지 않는지 정한다. |
| release가 OS 조건부 capability를 요구하지만 field·검사 계약이 없거나 target에서 지원되지 않음 | manifest field 필요성과 declaration owner, X01/D04 중 검사 owner, unsupported 결과 범위를 정한다. 계약 전에는 해당 API를 OTA chunk에서 사용하지 않는 0004 gate를 유지한다. |
| 알 수 없는 graph/release schema 또는 unsupported required field | 정확한 지원 버전 확인 위치, 진단 분류, 무시와 fail-closed 사이의 필드 규칙을 정한다. |
| duplicate key·필수 필드 누락·형식 오류·큰 입력 | parsing 한계, canonicalization 적용 전후, 서명 계산 대상 byte를 정하고 모호한 해석이 서명 검증을 우회하지 않게 한다. |
| 서명 불량·알 수 없는 키·회전 중인 key version | content object fetch 전 서명 검증, trust update 경계, active key rollback 금지와 오류 소유자를 정한다. |
| 캐시된 trust state가 stale한 동안 key revocation/compromise가 발생 | trust freshness 기준, 오프라인에서의 거부·가용성 경계와 compromise 복구 절차 owner를 정한다. |
| sequence replay·stale base·다른 channel 또는 target에 대한 명령 | 서명된 sequence가 전체 target tuple/channel 및 base에 묶이는지 확인하고, D03 publish CAS/head와 D04 client high-water의 per-target scope, 각 단계의 거부 시 상태 불변을 구분한다. |
| `affectedScope`·`affectedFeatureIds`가 graph diff와 다르거나 바뀜 | 어느 필드가 서명되고 D03이 재계산·검증하며 불확실 diff를 app 범위로 넓히는지 정한다. |
| 과거 signed release를 rollback으로 재사용 | 과거 envelope를 거부하고 새 higher-sequence command를 검증하는 owner와 정책을 정한다. |
| offline rollback authorization 없음·만료·재사용·다른 target binding | OTA high-water를 낮추지 않고 active snapshot 또는 내장 snapshot을 보존하는 D02/D04 경계를 정한다. |
| app data restore·초기화·재설치 뒤 high-water 손실 | 보장 가능한 저장소 scope, 보장 불가 시 재등록/서버 복구/실패 폐쇄 결과를 제품 owner가 정한다. |
| graph 내부 edge는 유효하나 원본 import/resource가 누락 | R15 구조 검증과 C02 원본 graph completeness fixture의 차이를 유지하며 D02가 이를 대신 검증한다고 하지 않는다. |
| 서명·호환이 유효하나 앱 스토어 정책이 불명확 | 기술 승인과 R09/D05 release policy 승인을 분리하고 허가되지 않은 앱을 rollout하지 않는다. |

## 산출 단계와 중단 조건

1. 0004·OTA design·R15·D02/D03/D04/X01/C02·R09에서 확정 제안과 미결 사항을 decision register로 분리한다.
2. 각 미결 항목에 책임 owner, 필요한 product decision, acceptance evidence, 실패 시 owner와 현재 임시 경계를 기록한다. 이 문서에서는 algorithm, wire field naming, key service, sequence lifetime을 선택하지 않는다.
3. 서명·호환·anti-replay·rollback·graph completeness 사례를 별도 20개 계획 실패 관점으로 검토하고 수정 후 영향 관점을 재검토한다.
4. 구현은 사용자 정책 결정, 버전 있는 D02 계약, C02 graph completeness fixture, D03 publish interface 및 D04 consumer boundary가 준비된 별도 작업으로 남긴다. 그때 실제 code/runtime review와 별도 PR diff review를 새로 한다.

- 서명 algorithm/key custody, canonical byte rules, offline rollback, reinstall protection 중 하나라도 owner·위협 경계가 정해지지 않으면 release format을 확정하지 않는다.
- D02 envelope와 D03 CAS/D04 install state가 동일한 sequence, channel, target, base ID의 소유를 두고 충돌하면 구현을 보류한다.
- R15 graph completeness가 C02 fixture로 증명되지 않으면 manifest 내부 검사로 source completeness를 주장하지 않는다.
- `runtimeId`, host API, permission 또는 new-binary gate를 확정할 근거가 없으면 OTA 호환 조합을 허용하거나 배포 가능으로 표시하지 않는다.
- C11, D02 implementation, OTA server/client, store-policy approval, public support/API는 이 계획으로 완료 처리하지 않는다. 이 계획은 [공식 상태 대장](../spec/STATUS.md)의 D02 체크·완료 수, 공개 API 또는 numeric contract version을 변경하지 않는다.
