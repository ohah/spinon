# E08 시작 시간 최적화 · PR diff 적대 검토 30개

- **PR:** [#1385](https://github.com/ohah/spinon/pull/1385)
- **검토한 commit:** `dbbc1a8166e7870b25ef90ae757cae546d3e458b`
- **base:** `04947b4a4055ce315a221194feb7f6c2460c9f16`
- **검토한 functional diff:** E08 계획 및 계획 검토 기록 Markdown 2개 추가. 코드·fixture·`STATUS.md` 변경 없음. 이 PR diff review artifact는 그 뒤 별도 추가되어 최종 PR tree에서 다시 확인한다.
- **구분:** 이 검토는 별도의 계획 실패 경로 20개 검토를 재사용하지 않는다. 계획/PR 범위, source provenance, 플랫폼·자원 경계, PR metadata의 새 diff 관점이다.

| # | 적대 관점 | 판정과 근거 |
|---:|---|---|
| 1 | STATUS나 공식 완료 수를 바꿔 E08을 완료로 표시 | **PASS.** 검토한 기능 diff는 두 계획 문서만 추가하며 E08 상태·완료 수 변경이 없다. [STATUS](../spec/STATUS.md) |
| 2 | 계획 초안을 측정 완료처럼 보이게 함 | **PASS.** 계획은 측정 미실행·E08 미완료를 명시한다. [계획:3](./e08-startup-optimization.md) |
| 3 | E/X 병렬 그룹을 모든 구현·측정이 독립이라는 보장으로 해석 | **PASS.** 현재 bundle path로 한정하고 X01/OTA 경계 및 입력 identity stop gate를 둔다. [계획:91](./e08-startup-optimization.md) |
| 4 | 측정 방법을 API 지원·호환성 계약으로 승격 | **PASS.** 문서는 실험 설계이며 공개 API/V8 호환성 약속이 아니라고 제한한다. [계획:10,12](./e08-startup-optimization.md) |
| 5 | 실제 화면 시작을 첫 frame 성능으로 과장 | **PASS.** GPU submit과 callback을 optical presentation 증거로 취급하지 않는다. [계획:40](./e08-startup-optimization.md) |
| 6 | 모호한 bundle pre-transform을 임의로 구현 | **PASS.** 후보 의미를 실행 전에 정의하거나 근거를 갖춰 미적용 처리하도록 한다. [계획:18,67](./e08-startup-optimization.md) |
| 7 | A 후보를 생략한 채 A/B/C 전체 완료 주장 | **PASS.** A/B/C를 각각 분류하고 unresolved 후보가 있으면 완료 후보를 막는다. [계획:108](./e08-startup-optimization.md) |
| 8 | 근거 없이 plan-review-20을 전부 재검토했다고 주장 | **PASS.** 검토 기록은 20개 범위와 보정 후 재검토를 설명하고 실행 검증과 분리한다. [review:5,33](./e08-startup-optimization-plan-review-20.md) |
| 9 | 계획 검토를 E08 구현 승인으로 오인 | **PASS.** 후보 측정 전 관문과 구현 PR 검토를 별도로 둔다. [계획:112](./e08-startup-optimization.md) |
| 10 | PR이 코드·실행 결과를 추가했다고 가장 | **PASS.** PR body와 변경 경로가 문서 전용임을 표시한다. |
| 11 | 상대 링크의 target path가 없거나 잘못됨 | **PASS.** 계획·review의 repository links를 확인했다. [계획:3–6,26,50,112](./e08-startup-optimization.md) · [review:3](./e08-startup-optimization-plan-review-20.md) |
| 12 | 제안한 bootstrap 입력이 Android/iOS 실제 경로와 다름 | **PASS.** `index.html`/`root-app.js` 경로를 확인하고 packaged bytes identity 확인 전 run을 막는다. [계획:44,50](./e08-startup-optimization.md) |
| 13 | V8 15.6.0을 안정 API 호환 약속으로 기록 | **PASS.** V8 version 문서의 candidate 경계와 일치한다. [계획:12](./e08-startup-optimization.md) |
| 14 | untracked C11 log를 공개 E08 result로 링크 | **PASS.** C11 log는 당시 동시성 설명으로만 적고 untracked·E08 근거 아님을 명시한다. [계획:89](./e08-startup-optimization.md) |
| 15 | RuntimeSession timing을 순수 V8 initialization으로 오해 | **PASS.** session coordinator, V8 create, facade/bundle compile/evaluate 단계가 구분된다. [계획:26–40](./e08-startup-optimization.md) |
| 16 | cache/snapshot identity가 runtime variant를 구분하지 못함 | **PASS.** V8·library·adapter·build·source·origin·compile mode·artifact format과 snapshot seed를 다룬다. [계획:81–83](./e08-startup-optimization.md) |
| 17 | 후보들이 shared bootstrap/Cargo/V8 outputs를 덮어씀 | **PASS.** baseline bytes copy와 variant별 output/Cargo target isolation을 요구한다. [계획:50–61](./e08-startup-optimization.md) |
| 18 | Simulator 결과를 iOS device/JITless 결과로 일반화 | **PASS.** platform, ABI, OS profile, JIT mode cohorts와 주장 범위를 분리한다. [계획:56,81,85](./e08-startup-optimization.md) |
| 19 | cleanup wording이 전체 process/filesystem 정리를 증명한다고 함 | **PASS.** run-owned path와 task-owned process만, active handles 확인 뒤 정리한다. [계획:95](./e08-startup-optimization.md) |
| 20 | diff에 계획 외 source/evidence changes가 섞임 | **PASS.** 검토한 commit diff에는 두 계획 Markdown 파일만 있었다. 이 적대 검토 기록은 뒤에 별도로 더해 최종 PR tree 범위를 재확인한다. |
| 21 | cold/warm 이름이 cache state를 숨김 | **PASS.** fresh-process/no-artifact, valid artifact, new Isolate, same Isolate cohorts를 분리한다. [계획:73](./e08-startup-optimization.md) |
| 22 | 실행 순서·표본·CI·제외 기준을 결과 후 선택 | **PASS · 실행 전 관문.** randomized blocks와 paired analysis를 정하고 threshold/예산은 candidate 결과 전 고정하도록 한다. [계획:75–77](./e08-startup-optimization.md) |
| 23 | 계측 오버헤드나 결과 의미 변화가 이득처럼 보임 | **PASS.** field-level oracle, tracing on/off control, noise 기준을 둔다. [계획:46,77](./e08-startup-optimization.md) |
| 24 | CI/test/benchmark/browser run을 통과라고 잘못 보고 | **PASS.** PR body와 계획 검토 기록은 해당 실행이 없다고 명시한다. |
| 25 | platform variants가 Cargo/V8/build outputs를 공유 | **PASS.** variant target/output roots와 V8 identity를 고정한다. [계획:61,81–85](./e08-startup-optimization.md) |
| 26 | E08 run과 C11 matrix가 동시에 resource/measurement를 오염 | **PASS.** 당시 C11 process/HEAD/log를 기록하고 terminal evidence 전 E08 runtime/build/measurement를 금지한다. [계획:89](./e08-startup-optimization.md) |
| 27 | 손상 cache/raw failures를 cleanup 중 버림 | **PASS.** 실패 artifact와 원본 logs를 보존하고 hash manifest 전 삭제하지 않는다. [계획:95](./e08-startup-optimization.md) |
| 28 | cleanup이 사용자·다른 session process를 종료 | **PASS.** run-owned path/task-owned process 제한과 `ps`/`lsof` 확인이 있다. [계획:95](./e08-startup-optimization.md) |
| 29 | 문서 PR인데 browser screenshot 또는 visual proof를 있다고 주장 | **PASS.** UI 동작 변경이 없어 캡처가 없음을 body에 적고, presentation 주장을 제한한다. |
| 30 | PR template·labels·remaining scope를 빠뜨림 | **PASS, CI 상태 한정.** PR body에는 E08 ID·변경·명세 경계·근거·남은 범위가 있고 `spike`, `documentation`, `area: runtime` labels가 붙었다. commit status/workflow runs는 비어 있어 CI-green으로 해석하지 않는다. PR metadata 조회 시 mergeable은 true였다. 이 review file link는 artifact 추가 뒤 body에 보탠다. |

## 판정 경계

30개 관점은 commit diff와 PR metadata를 대상으로 했다. #22·#30은 측정 threshold와 CI가 아직 없는 상태를 발견한 것으로, 실험 전 관문과 “통과하지 않은 status를 green이라고 하지 않음”을 확인했다. PR diff review 자체는 계획의 20개 실패 관점 검토와 별개다. 이 파일은 review artifact이며 E08 measurement/implementation 결과가 아니다.
