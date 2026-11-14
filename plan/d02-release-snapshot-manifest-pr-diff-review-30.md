# D02 릴리스 스냅샷 매니페스트 PR · 적대 diff 검토 30개

- **PR:** [#1388](https://github.com/ohah/spinon/pull/1388)
- **검토 대상 head:** `e4713c0ad089ebb5e9140c50c613b91ef9208e5a`
- **base:** `main` `280c87507967331cc4775ec0824237dd614f2397`
- **검토 시점 diff:** D02 계획서와 계획 검토 20개 기록 두 Markdown 추가. 최종 head의 PR body 링크, base/head, labels도 함께 확인했다.
- **수행 방식:** dependency/contract lens 1–10, file/scope/metadata lens 11–20, runtime/resource lens 21–30을 검토했다. 발견된 부분 판정은 영향을 받는 관점에 따라 보정 후 재검토했다.
- **실행 한계:** build/test·ADB·Simulator·agent-browser·터미널 브라우저는 실행하지 않았다. GitHub commit status/workflow 결과는 비어 있어 CI 통과로 표현하지 않았다.

| # | 적대 관점 | 최종 판정 |
|---:|---|---|
| 1 | R15 graph 초안을 D02 공개 envelope나 구현 완료로 오인 | PASS |
| 2 | C02 원본 graph completeness 없이 내부 graph 검사만으로 import 누락을 승인 | PASS |
| 3 | D02 인증, D03 publish/CAS, D04 설치·high-water·rollback 소유권을 섞거나 실패 후 상태를 변경 | PASS |
| 4 | X01 로컬 graph manifest와 D02 원격 release envelope를 같은 계약으로 취급 | PASS |
| 5 | `runtimeId`를 추정 고정하거나 C11 완료를 D02 계획 선행조건으로 요구 | PASS |
| 6 | target tuple·channel·base·sequence·rollback target 중 하나를 서명 밖에 둠 | PASS |
| 7 | channel을 포함한 tuple 간 sequence/high-water/replay scope가 섞임 | PASS |
| 8 | OS conditional capability가 미지원인데 미결 field 때문에 OTA API 사용이 허용됨 | PASS |
| 9 | D02 계획을 STATUS 완료, 공개 API 또는 numeric contract version으로 과장 | PASS |
| 10 | 기술적 signature/compatibility 통과를 R09/D05 store-policy 승인으로 승격 | PASS |
| 11 | root의 dirty C11/C15/C19 source·lock·evidence가 PR diff에 섞임 | PASS |
| 12 | PR diff가 계획과 계획 검토 기록을 넘어 runtime, STATUS, build 설정까지 변경 | PASS |
| 13 | 문서·PR body의 상대 링크 또는 SHA-pinned GitHub 링크가 깨짐 | PASS |
| 14 | STATUS/API/version 파일을 바꾸지 않고 상태·완료 수를 유지한다는 주장이 실제 diff와 다름 | PASS |
| 15 | 미승인 wire schema·field name·serialization·signature algorithm을 확정 | PASS |
| 16 | 계획 초안을 구현·적합성·OTA 동작의 증거처럼 설명 | PASS |
| 17 | C11 결과를 D02 근거로 재포장하거나 C11을 downstream 공통 gate로 확대 | PASS |
| 18 | 20개 계획 검토 기록의 파일 수, 관점 배정, 수정·재검토 한계가 실제 기록과 불일치 | PASS |
| 19 | PR template의 상태/API/동작 근거 항목이나 남은 범위를 오해하게 작성 | PASS |
| 20 | 작업 영역에 맞지 않는 labels, base, head, title 또는 본문 link metadata | PASS |
| 21 | envelope 서명·target 검증보다 먼저 content object를 fetch | PASS |
| 22 | 압축 해제된 원본 bytes를 검사한 뒤 CSS/JS parser·evaluator·image/font decoder 등 consumer에 다른 bytes 전달 (TOCTOU) | PASS · 재검토 |
| 23 | 손상 object, oversized stream, 부족한 staging이 cache 재사용 또는 저장 자원 고갈을 유발 | PASS |
| 24 | 거부된 publish/install이 D03 CAS/head 또는 active snapshot/high-water를 변경 | PASS |
| 25 | 다른 platform/runtime/channel의 base·sequence·rollback target을 mix-and-match | PASS |
| 26 | revoked/compromised key 또는 stale cached trust가 offline 상태에서 허용됨 | PASS |
| 27 | duplicate key·잘못된 형식·큰 입력·canonicalization 차이로 서명 검증을 우회 | PASS |
| 28 | 오래된 envelope replay, rollback 재사용, restore/reinstall 뒤 high-water rollback | PASS |
| 29 | `affectedScope`가 graph closure와 불일치하거나 원본 graph completeness를 대신한다고 주장 | PASS |
| 30 | 계획 검토나 빈 CI 결과를 build/test·기기·화면·브라우저 동작 증거로 표시 | PASS |

## 수정 및 최종 확인

- **#22 최초 PARTIAL:** digest/size 검증이 JS 평가 전인지 설명했지만 CSS·이미지·폰트처럼 다른 자원 소비 경계가 빠져 있었다. 계획 24·51행에 압축 해제된 원본 bytes를 검증하고 동일 bytes를 모든 parser/evaluator/resource consumer에 전달하는 조건을 추가했다. dependency #2/#3와 runtime #22–24 및 계획 검토 #15에서 재확인했다.
- **#18 최초 PARTIAL:** 계획 검토 기록이 두 파일 중 계획서만 추가한 것처럼 표현했다. 파일 수를 계획서와 20점 검토 기록 두 개로 바로잡고, 중간 초안 chronology는 commit history가 아니라 검토자 기록에 근거한다고 명시했다. file lens에서 재확인했다.
- **#20 최초 PARTIAL:** `area: devtools`가 OTA 계획에 맞지 않았다. 기존 `area: ota` label로 바꾸고 최종 head 및 PR 본문 링크를 다시 확인했다.

**최종 판정:** 지정한 PR head `e4713c0ad089ebb5e9140c50c613b91ef9208e5a`에서 30개 서로 다른 PR diff 관점 모두 PASS다. 이 기록은 계획 문서의 설계·범위 검토 결과이며 D02 구현 완료나 런타임 동작 근거가 아니다.
