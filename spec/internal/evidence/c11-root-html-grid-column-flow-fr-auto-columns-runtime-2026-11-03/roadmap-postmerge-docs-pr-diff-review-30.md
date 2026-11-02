# C11.1 `fr` roadmap post-merge 문서 PR diff 검토 30개

검토 대상은 PR #1066 병합 뒤 `spec/STATUS.md`, C11 계획·내부 계약, runtime evidence와 공개 roadmap에 반영한 상태 동기화다. 변경 파일을 구현 지원 범위·상태 원본·체크 결과·링크 공개 경로 관점에서 따로 확인했다. 각 항목은 독립된 잘못된 상태·과장·링크 실패 경로를 다룬다.

1. **병합 여부와 상태** — GitHub PR #1066이 `MERGED`인지 확인했다. 기록은 현재 GitHub 상태와 일치한다: PASS.
2. **병합 시각** — API의 `mergedAt` `2026-11-02T15:41:43Z`와 문서 값을 대조했다: PASS.
3. **merge 방식** — PR 이벤트/제목의 Rebase and merge 기록과 문서 표현을 대조했다: PASS.
4. **merge SHA** — PR merge commit과 현재 `main` HEAD `2dde385c5f240c8b8837b9e55d9c5431f5f9294b`가 일치한다: PASS.
5. **선행 계획 PR** — 계획 PR #1065가 이미 병합된 사실을 구현 PR #1066과 혼동하지 않았다: PASS.
6. **공식 상태 원본** — 상태 항목은 `spec/STATUS.md`에 남고 계획·HTML은 그 상태를 복제해 표시한다: PASS.
7. **상위 C11 체크** — C11 checkbox가 미완료로 남아 있다: PASS.
8. **집계 보존** — 이 제한 구현만으로 공식 완료 수를 올리지 않고 `72/213`을 유지한다: PASS.
9. **계획 상태** — 전용 계획의 후보/merge 대기 문구를 병합 완료로 갱신했다: PASS.
10. **그리드 계획 인덱스** — `plan/c11-css-grid.md`에서 후보 표현을 병합 표현으로 바꾸고 전용 근거를 연결했다: PASS.
11. **내부 계약 버전** — 내부 계약 0059 숫자 버전 `0.1.0`을 유지한다: PASS.
12. **기능 경계** — 일반 CSS 지원 또는 전체 Grid 구현으로 서술하지 않는다: PASS.
13. **입력 admission** — 단일 기본 parsed Root HTML Grid 및 direct `fr` scalar/list라는 입력 범위를 유지한다: PASS.
14. **기존 px/% 동작** — 선행 px·percentage auto-column slice를 함께 표현해 기존 경로 단절을 암시하지 않는다: PASS.
15. **fraction 계산 기준** — fixed/percentage track과 gap을 제외한 남은 공간에서 factor를 배분한다는 계약과 일치한다: PASS.
16. **subunit factor** — 합이 1 미만인 factor 목록의 자동 최소값 경계가 계획·계약에서 삭제되지 않았다: PASS.
17. **zero factor** — `0fr` automatic minimum 조건을 명시한 기존 계약이 유지된다: PASS.
18. **invalid factor** — 음수·비유한 값 및 비어 있거나 과다한 목록 거부가 유지된다: PASS.
19. **지원 밖 track 문법** — auto/intrinsic/minmax/fit-content/calc/repeat 거부를 문서화한다: PASS.
20. **width 조건** — definite px width 및 min/max/aspect-ratio/math-backed 제한을 보존한다: PASS.
21. **item/flow 조건** — empty fixed-size direct div, sparse column flow, order 0, 단일 비중첩 Grid 범위를 보존한다: PASS.
22. **Rust 근거** — README의 focused suite 항목은 기록된 실행 범위만 주장하며 전체 C11 테스트로 확대하지 않는다: PASS.
23. **Android evidence** — API 37 ARM64 AVD 표기를 실기기 증거로 확대하지 않는다: PASS.
24. **iOS evidence** — iPhone 17 Pro/iOS 26.2 Simulator를 physical iOS 기기로 부르지 않는다: PASS.
25. **표면 검증 한계** — screenshot/draw/present를 PixelCopy·Metal readback이나 compositor 이후 pixel equality와 구분한다: PASS.
26. **기기/GPU 한계** — physical device와 hardware GPU 확인을 주장하지 않는다: PASS.
27. **PR check rollup** — GitHub `statusCheckRollup`이 빈 배열인 것을 기록하고 CI 성공으로 쓰지 않는다: PASS.
28. **필수 check 설정** — main branch required checks endpoint의 404(`Required status checks not enabled`)를 CI 결과와 구분한다: PASS.
29. **공개 카드 위치** — 새 카드가 C11.1 계획 카드 바로 뒤, 기존 PR #1000 카드 앞에 삽입됐다: PASS.
30. **배포 링크 경로** — 계획, Chrome/runtime README, merge record와 두 화면 파일의 선택 게시 source가 존재한다: PASS.

실패를 찾지 못했다. 이 검토는 문서 상태 및 선택 게시 링크의 일관성을 다루며 C11.1/C11 완성도나 새 runtime 동작을 재판정하지 않는다.
