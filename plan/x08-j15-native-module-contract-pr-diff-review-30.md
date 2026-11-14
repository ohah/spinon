# X08/J15 네이티브 모듈 계약 계획 PR diff 검토

- **PR:** [#1387](https://github.com/ohah/spinon/pull/1387)
- **검토 기준:** base `4f5c490cbaef7313dc7ff9f188b2de1ced5f5160`, 후보 head `5408a0af0a717cd67dc3489049fa0e1700ea6290`
- **검토 대상:** [계획](x08-j15-native-module-contract.md), [계획 관점 검토](x08-j15-native-module-contract-plan-review-20.md), PR 본문과 변경 파일 경계.
- **결과:** 아래 30개의 서로 다른 계약·의존성·파일·런타임 실패 경로는 모두 최종 후보 기준 PASS다. C11 실행 맥락 표현을 줄인 뒤 관련 의존성, 파일, 자원 검토를 최신 후보에서 다시 확인했다.
- **검토 한계:** 계획 문서 PR이므로 구현·플랫폼 동작 검증은 하지 않았다. 빌드, 테스트, CLI, 브라우저, device 또는 simulator를 실행하지 않았다.

## 의존성과 계약 경계

| # | 적대 검토 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | S03 내부 binding을 앱 공개 API로 오인 | **PASS.** S03 internal host-binding과 X08/J15 public module API owner를 분리한다. (계획 §선행 의존과 소유 경계) |
| 2 | J15의 상위 의존성 목록 또는 X08→J15 계층을 잘못 연결 | **PASS.** S03·X08·D02·X16의 공식 의존성과 X08/J15 역할이 STATUS와 일치한다. (계획 §선행 의존과 소유 경계; STATUS J15) |
| 3 | 공개 callback 계약과 V8/DOM callback 자원 소유자를 혼동 | **PASS.** J15는 공개 등록·취소·session, S03은 V8 handle/Isolate, S03.3은 DOM root 보유 closure의 추가 규칙을 소유한다. (계획 §선행 의존과 소유 경계, §계약 질문과 후보) |
| 4 | J14 기본 모듈의 권한 prompt/거부와 J15 사용자 모듈 선언을 합침 | **PASS.** 기능별 동작과 공통 declaration 경계를 나눈다. (계획 §선행 의존과 소유 경계, §계획 acceptance matrix) |
| 5 | 0004의 새 바이너리 제안을 미결로 되돌리거나 D02 결정을 임의 고정 | **PASS.** 기존 바이너리 경계는 기록하고 manifest 표현·reject/error 세부는 D02에 남긴다. (계획 §선행 의존과 소유 경계, §계약 질문과 후보) |
| 6 | 원격 JS의 native module 호출과 native code OTA 배포를 혼동 | **PASS.** 기존 바이너리의 JS 호출과 native implementation 배포를 명시적으로 구분한다. (계획 §계약 질문과 후보) |
| 7 | X16 적합성 없이 web/Android/iOS 지원 완료로 해석 | **PASS.** 고정 입력·플랫폼 결과 전까지 지원 주장을 보류한다. (계획 §선행 의존과 소유 경계) |
| 8 | registry key, module identifier, version conflict 동작을 계획에서 임의 확정 | **PASS.** 판단 위치와 실패 시나리오를 질문으로 남기고 exact schema를 결정하지 않는다. (계획 §계약 질문과 후보) |
| 9 | 구현 등록은 있지만 platform module이 없을 때 silent no-op/fallback을 가정 | **PASS.** diagnostic owner와 unsupported 표시를 결정하게 한다. (계획 §계획 acceptance matrix) |
| 10 | host API/ABI/OTA 조합 미지원이 성공 fallback으로 통과 | **PASS.** 지원표 밖 조합은 fail-closed이며 reject point와 오류 계약은 gate로 남는다. (계획 §계약 질문과 후보, §계획 acceptance matrix) |

## 파일·상태·근거 무결성

| # | 적대 검토 관점 | 판정 및 근거 |
|---:|---|---|
| 11 | 계획 범위를 벗어나 root의 C11/C15/C19 dirty WIP를 포함 또는 덮음 | **PASS.** 변경은 계획 및 검토 문서에 한정하고 공유 root WIP는 diff 밖에 둔다. |
| 12 | 계획만 추가하면서 STATUS 완료 수나 X08/J15 상태를 바꿈 | **PASS.** STATUS diff는 없고 X08/J15 미완료 경계를 유지한다. (계획 머리말, §중단 조건과 완료 경계) |
| 13 | 공개 API 또는 내부 interface의 숫자 version을 새로 선언 | **PASS.** 등록·API/version 값은 미정이며 숫자 version 변경이 없다. (계획 §계약 질문과 후보, §중단 조건과 완료 경계) |
| 14 | 상대 Markdown 링크 또는 0004 OTA 앵커가 끊김 | **PASS.** 계획·검토 문서 링크, 원문 링크 및 `#ota`가 해석된다. |
| 15 | PR 본문 계획 링크가 현재 후보 이외의 commit을 가리킴 | **PASS.** 검토 시점 본문 URL이 후보 head와 일치했다. 마지막 계획 정리 commit 뒤 링크를 갱신하고 최신 head에서 재확인했다. |
| 16 | 0003/0004/0021 또는 기존 DOM lifecycle 계획을 복제·모순 | **PASS.** 기존 제안과 내부 수명 경계를 출처로 사용하고 세부 owner를 나눈다. (계획 §목적과 계획 범위, §선행 의존과 소유 경계) |
| 17 | 20개 계획 검토 기록이 현재 문서와 불일치하거나 구현 통과로 과장 | **PASS.** 양방향 링크, 최종 판정, callback/resource 보정 및 검토 한계가 맞다. ([계획 검토 기록](x08-j15-native-module-contract-plan-review-20.md)) |
| 18 | 계획 경로·파일명·언어·표가 저장소 문서 규칙에서 벗어남 | **PASS.** `plan/`의 한국어 계획·검토 기록 구조에 맞는다. |
| 19 | manifest, lockfile, build/config, 코드 또는 binary artifact가 함께 변경 | **PASS.** 변경 범위는 Markdown 문서이며 제품 산출물은 포함하지 않는다. |
| 20 | 로컬 C11 PID, log, checkout 경로 또는 특정 실행 결과가 계약 근거처럼 노출 | **PASS.** 계획과 검토 기록은 로컬 식별자/실행 결과를 공개하지 않는다. 활성 작업 상태는 격리·자원 경계 검토에만 반영했다. |

## 런타임·실패·자원 경계

| # | 적대 검토 관점 | 판정 및 근거 |
|---:|---|---|
| 21 | 계획 acceptance 질문이 실제 구현·플랫폼 증거라고 오인 | **PASS.** 계획 초안과 미시작 상태를 명시하고 동작 근거를 주장하지 않는다. (계획 머리말, §계획 acceptance matrix) |
| 22 | C11 일부 작업 진행을 C11 전체 완료 또는 X08/J15 준비 완료로 승격 | **PASS.** 이 변경은 C11 상태를 수정하지 않으며 X08/J15를 미완료로 둔다. (계획 §중단 조건과 완료 경계) |
| 23 | C11을 제품 기능 선행조건으로 취급하거나 CPU/I/O 경합을 무시 | **PASS.** C11은 제품 의존성이 아니고, 겹치는 Cargo/V8 검증은 별도 자원 확인을 요구한다. (계획 §계획 단계와 자원 경계, §중단 조건과 완료 경계) |
| 24 | 잘못된 입력이 native side effect 이후에야 실패 | **PASS.** JS boundary validation 시점과 error owner를 acceptance 질문으로 둔다. (계획 §계획 acceptance matrix) |
| 25 | sync/async 오류, 취소 경합, 늦은 완료, session 종료 뒤 callback에서 결과 누락 | **PASS.** 오류 전달과 cancellation/lifecycle 소유자를 각 시나리오에서 결정한다. (계획 §계약 질문과 후보, §계획 acceptance matrix) |
| 26 | Isolate 호출과 UI/main-thread 작업 사이 thread affinity 위반 | **PASS.** 허용 호출·반환 순서와 UI thread 일반화 금지를 결정 질문으로 둔다. (계획 §계약 질문과 후보) |
| 27 | session/Isolate 종료 뒤 pending V8 callback strong handle이 남거나 호출 가능 | **PASS.** public lifetime과 V8 cleanup owner를 분리하고 종료 후 호출 처리를 acceptance 경로에 포함한다. (계획 §선행 의존과 소유 경계, §계획 acceptance matrix) |
| 28 | DOM wrapper/HostNodeHandle/external-root를 잡은 closure가 너무 일찍 회수되거나 root 누수 | **PASS.** 이 closure에만 S03.3의 추가 root/release 규칙을 둔다. (계획 §선행 의존과 소유 경계, §계약 질문과 후보) |
| 29 | 고용량 image/frame 데이터를 일반 호출에 섞어 ownership·release·backpressure를 잃음 | **PASS.** 대용량 경로를 별도 계약으로 분리하고 zero-copy를 기본 가정하지 않는다. (계획 §계약 질문과 후보, §중단 조건과 완료 경계) |
| 30 | OS capability·플랫폼·ABI·host version이 빠져도 성공 fallback 또는 지원 주장을 함 | **PASS.** 미지원 진단, X16 적합성 gate, fail-closed 결과를 유지한다. (계획 §선행 의존과 소유 경계, §계획 acceptance matrix) |

## 최종 후보 재확인

검토 중 C11 실행 맥락을 계획·검토 기록에서 걷어내고, 제품 의존성과 자원 경합 구분만 남겼다. 이에 영향을 받는 dependency·file·runtime/resource 관점을 새 head에서 다시 확인했다. 이 파일은 검토 결과 기록이며 제품 계약·상태를 변경하지 않는다.
