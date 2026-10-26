# C11.4 direct text intrinsic plan · pass04 targeted follow-up

**범위:** 수정된 text plan에서 helper result path, candidate/fallback 경계, pass04 기록, runtime claim, 완료 수만 읽기 전용으로 재확인했다. 전체 계획 관점 재검토는 아니다. Branch와 `origin/main`은 `040a5360be92f73e141797aa7dcd4ef00ab24942`다.

- Helper 경로가 구체화됐다. 계획은 helper 반환 `StyleLayoutOutput`을 `output.layout`로 접근하고 intrinsic inputs, measurements, text snapshots, frame을 그 안에서 대조한다. 이전에 지적된 wrapper 접근 불일치는 해소됐다.
- Generic collector positive candidate는 direct text-item 두 개로 정확히 제한한다. 비후보는 `Ok(None)`으로 기존 legacy one-item/one-text `TEXT_CASES` fallback 선택을 요청하고, fallback 자체도 기존 root/subtree/ID/text/style 조건이 맞을 때만 성공한다. Candidate source/style/grid 오류와 두 개 초과는 fallback하지 않고 typed error로 닫는다. 이전 candidate count PARTIAL은 문서상 해소됐다.
- [pass04 보고서](./c11-4-generic-direct-text-intrinsic-input-plan-review-pass04-20-2026-10-26.md)는 child index에 연결돼 있다. 보고서의 historical result는 정확히 **PASS 16 / PARTIAL 4 / FAIL 0**, 20개 관점이다. 네 PARTIAL은 당시의 candidate count 모호성, 세 property-specific inline rejection controls 미실행, `StyleLayoutOutput.layout` 접근 누락, pass04 index 링크 누락이었다. 수정 후에도 세 control의 실행 증거만 acceptance gate로 남는다. 현재 plan은 이 부분을 미실행이라고 밝히며 통과로 주장하지 않는다.
- 새 runtime 완료 주장은 찾지 못했다. Plan은 구현과 platform evidence가 아직 없다고 밝히고, runtime/helper assertion 및 Android/iOS simulator를 미래 acceptance gate로 표현한다.
- C11.4 checkbox와 공식 `72/213`은 변경되지 않았다고 plan 상단과 완료 판정이 명시한다.

**판정:** 이 targeted check에서 요청한 문서 경계는 정합적이다. 세 property controls는 구현 acceptance 전 실행해야 할 PARTIAL이다. 이 후속 검토는 전체 fresh plan review를 대신하지 않으므로 plan의 full fresh review 필요 문구는 아직 유효하다. 제품 문서/evidence, tests, browser, simulator는 수정·실행하지 않았고 commit하지 않았다.
