# X08/J15 네이티브 모듈 계약 계획 검토

- **대상:** [X08/J15 계획](x08-j15-native-module-contract.md)
- **검토일:** 2026-11-15
- **검토 범위:** 계획·소유권·실패 경로·작업 경계. API 구현, 제품 동작, 플랫폼 지원 검증은 아니다.
- **관점:** 의존성, 파일 결합, 런타임·자원 경계의 독립 적대 검토.
- **결과:** 최초 검토에서 발견된 소유권 및 OTA 표현 결함을 계획에 반영하고 해당 관점을 재확인했다. 아래 20개 항목은 최종 초안 기준 PASS다.

| # | 실패 관점 | 최종 판정 및 확인 |
|---:|---|---|
| 1 | 내부 V8 binding을 앱 공개 API로 노출하거나 X08/J15의 API 소유권을 S03에 넘김 | **PASS.** S03 internal host-binding과 X08/J15 public module API owner를 구분한다. (계획 §선행 의존과 소유 경계) |
| 2 | S03·D02·X16 미완료를 이유로 계획 단계까지 막거나, 반대로 구현 게이트를 생략함 | **PASS.** 계획 작성과 구현 선행조건을 나누고 미결 결정을 남긴다. (계획 §선행 의존과 소유 경계) |
| 3 | 일반 모듈 callback, V8 handle 정리, DOM wrapper root의 owner가 뒤섞여 해제 누락 또는 이중 소유가 됨 | **PASS.** 공개 register/cancel/session 계약은 J15, V8 handle/Isolate mechanics는 S03, DOM wrapper·HostNodeHandle·external-root closure의 추가 root 규칙은 S03.3에 둔다. (계획 §선행 의존과 소유 경계, §계약 질문과 후보) |
| 4 | 기본 모듈의 권한 prompt·거부 규칙을 사용자 모듈 계약에 복제함 | **PASS.** J14의 기능별 동작과 J15 공통 permission declaration 경계를 분리한다. (계획 §선행 의존과 소유 경계) |
| 5 | 이미 제안된 새 바이너리 경계를 미결로 잘못 낮추거나 D02 미결 OTA 세부를 임의 확정함 | **PASS.** 0004의 native code/V8/GPU/host API/permission 변경 시 새 앱 바이너리 기준을 기록하고 manifest 표현·reject point·오류 계약은 D02에 남긴다. (계획 §선행 의존과 소유 경계, §계약 질문과 후보) |
| 6 | X16 적합성 증거 없이 웹·Android·iOS 전체 지원을 선언함 | **PASS.** 고정 입력과 플랫폼별 결과가 없으면 계획만 두고 지원 완료를 주장하지 않는다. (계획 §선행 의존과 소유 경계) |
| 7 | 0003의 제안과 아직 결정되지 않은 exact API·registry·permission 형식을 혼동함 | **PASS.** 제안된 원칙과 미확정 값을 구별한다. (계획 §목적과 계획 범위, §계약 질문과 후보) |
| 8 | 계획 작성이 공유 root checkout의 C11 더티 파일을 덮거나 상태를 오염시킴 | **PASS.** 계획 및 관련 검토 산출물은 격리 worktree에 한정하며 root C11 WIP를 수정하지 않는다. |
| 9 | 기존 DOM lifecycle 계획과 GC·node graph 계약을 중복 정의해 owner가 갈라짐 | **PASS.** node/wrapper/lease 세부는 S03.3에 위임하고 X08/J15 module boundary만 남긴다. (계획 §선행 의존과 소유 경계, §계획 단계와 자원 경계) |
| 10 | 내부 계약 0021의 callback/Isolate 종료 및 external-root 규칙과 충돌함 | **PASS.** V8 정리와 DOM root 유지 조건을 서로 다른 owner에 배정하고 0021을 참조한다. (계획 §선행 의존과 소유 경계) |
| 11 | 상대 Markdown 링크가 깨지거나 원문 근거에서 벗어남 | **PASS.** STATUS, 0003, 0004, 0021 및 기존 lifecycle 계획의 링크 대상을 확인했다. |
| 12 | 계획 문서만으로 STATUS 완료 수, API 지원, 내부 interface version이 바뀐 것처럼 보임 | **PASS.** X08/J15 미완료를 유지하며 공식 상태·완료 수·version을 변경하지 않는다고 명시한다. (계획 머리말, §중단 조건과 완료 경계) |
| 13 | 문서 계획에 manifest, lockfile, generated binding, build artifact 또는 제품 코드를 끌어들임 | **PASS.** 변경 범위를 계획 문서에 한정하고 구현 산출물과 분리한다. |
| 14 | 0003/0004 제안이 이미 확정·구현된 런타임 동작이라고 잘못 읽힘 | **PASS.** acceptance 질문과 소유자만 정하며 실행 결과나 OTA 허용을 주장하지 않는다. (계획 §목적과 계획 범위, §계약 질문과 후보) |
| 15 | C11 실행 결과를 X08/J15의 선행조건·제품 근거로 재사용하거나 자원 경합을 무시함 | **PASS.** C11 결과는 C11 범위로 한정하며 X08/J15 의존성이나 근거가 아니다. 겹치는 Cargo/V8 검증은 별도 CPU/I/O 자원 확인 대상으로 둔다. (계획 §계획 단계와 자원 경계, §중단 조건과 완료 경계) |
| 16 | C11의 특정 matrix 통과를 C11 전체 완료 또는 X08 준비 완료로 확대 해석함 | **PASS.** 해당 run을 X08/J15와 분리하고 STATUS 완료 상태를 유지한다. (계획 머리말, §계획 단계와 자원 경계, §중단 조건과 완료 경계) |
| 17 | 권한 없음·거부·철회·OS 미지원 시 prompt 중복 또는 웹 fallback 오작동 | **PASS.** J14, 앱, 사용자 모듈 간 owner와 웹 대체의 유무를 결정 질문으로 남긴다. (계획 §계약 질문과 후보, §계획 acceptance matrix) |
| 18 | 잘못된 입력·비동기 취소·늦은 callback·session 종료·thread affinity에서 side effect나 stale 호출 발생 | **PASS.** validation, error mapping, cancellation race, late completion 및 호출 thread의 owner를 acceptance 항목으로 둔다. (계획 §계약 질문과 후보, §계획 acceptance matrix) |
| 19 | 대용량 image/frame payload를 일반 호출에 넣어 ownership·release·backpressure를 잃음 | **PASS.** 고용량 경로를 별도 계약 후보로 분리하고 zero-copy를 기본 가정하지 않는다. (계획 §계약 질문과 후보, §중단 조건과 완료 경계) |
| 20 | 브라우저·시뮬레이터 한 플랫폼 결과를 세 플랫폼 지원 근거로 확대하거나 미지원 조합이 성공 fallback함 | **PASS.** capability 차이를 분리하고 X16 결과 전에는 미지원 조합을 fail-closed로 둔다. (계획 §계약 질문과 후보, §계획 acceptance matrix) |

## 검토 한계

이 검토는 계획 문서의 의존성·계약 경계·파일 범위만 확인했다. 빌드, 테스트, CLI, 브라우저, device 또는 simulator를 실행하지 않았다. C11의 별도 RuntimeSession matrix 결과는 X08/J15 검토 근거로 사용하지 않았다.
