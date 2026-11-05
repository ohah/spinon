# C11.4 PR #1167 병합 후 로드맵 동기화 PR diff 적대 검토 20개

검토 대상은 PR #1167의 실제 merge 결과를 `spec/STATUS.md`와 실행 근거 README에 반영하는 문서 변경이다. 구현 PR diff 검토 20개는 재사용하지 않고, 병합 상태·공식 상태·근거의 범위를 서로 다른 실패 관점으로 대조했다.

1. **잘못된 PR을 연결하는 오기** — 상태표와 README의 링크가 모두 #1167을 가리키는지 확인했다.
2. **원본 head SHA를 merge SHA로 기록하는 혼동** — `gh pr view`의 merge commit `18782b58c5edf2e2cc00aeb71928c523c752a7c3`와 기록을 대조했다.
3. **병합 시각의 시간대 오류** — GitHub 응답의 `2026-11-05T01:31:53Z`를 그대로 기록했다.
4. **merge 방식 오기** — PR 동작과 프로젝트 규칙에 맞춰 Rebase and merge로 표기했다.
5. **열린 PR이 완료된 것처럼 보이는 표현** — 병합 사실만 기록하고 새로운 제품 완료 선언은 넣지 않았다.
6. **C11 상위 체크가 조기 완료되는 위험** — 상위 C11 체크박스를 그대로 미완료로 유지한다.
7. **C11.4 전체 완료로 오해될 위험** — 체크박스를 유지하고 해당 PR을 bounded runtime slice라고 한정했다.
8. **공식 집계가 자동 증가하는 오류** — 공식 완료 수를 `72/213`으로 유지했다.
9. **Chrome oracle 장면 수와 앱 실행 수 혼합** — Chrome 19 scene과 runtime 17 scene을 구분했다.
10. **zero-axis oracle-only 장면을 앱이 실행했다고 과장** — positive definite-axis 17 scene만 runtime에 연결했다고 적었다.
11. **Chrome 버전 손실** — precomparison 기준을 Chrome 156으로 특정했다.
12. **Android simulator를 실기기로 과장** — API 37 ARM64 AVD라고 명시했다.
13. **iOS simulator를 실기기로 과장** — iPhone 17 Pro / iOS 26.2 Simulator라고 명시했다.
14. **draw/present를 표면 픽셀 검증으로 확대** — 표면 readback 또는 pixel exact 주장을 추가하지 않았다.
15. **현재 결과에 없는 테스트를 새 기록에서 주장** — README의 기존 실행 근거를 연결하고 검증 항목을 추가하지 않았다.
16. **focused 통과를 전체 suite green으로 확대** — broad crate suite의 알려진 실패를 같은 문장에 유지했다.
17. **빈 check rollup을 CI 성공으로 표현** — check rollup이 비어 있다고 두 위치에서 분명히 기록했다.
18. **이전 실패 목록을 누락해 회귀가 숨겨지는 위험** — README의 `394/404`, `443/444`와 실패 분류를 그대로 보존한다.
19. **기존 구현 PR 리뷰를 새 문서 PR 리뷰로 재사용** — 이 파일은 문서 변경 전용 20관점 검토이며 구현 PR 검토와 별도다.
20. **문서 링크가 깨지는 위험** — 상태표의 evidence 경로 두 개와 README의 PR 링크가 저장소 내 대상 및 GitHub PR을 가리키는지 확인한다.

## 판정

문서 변경은 PR #1167의 merge 상태와 한계를 기록하고, 공식 C11 완료 판정은 변경하지 않는다. check rollup이 비어 있고 전체 broad suite가 green이 아니라는 한계도 남긴다.
