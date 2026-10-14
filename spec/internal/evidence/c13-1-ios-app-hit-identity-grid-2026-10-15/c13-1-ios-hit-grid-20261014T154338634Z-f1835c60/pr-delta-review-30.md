# C13.1 iOS hit identity grid PR 변경분 적대 검토 30개

검토일: 2026-10-15
대상: 구현 PR에 포함되는 코드·문서·fixture 실행 증거의 경계
판정: PR #166의 변경 범위와 게시 상태를 확인했다. agent-browser에서 PR 본문 이미지 로드와 natural dimensions를 확인했다.

1. **구현 범위 부풀림** — 새 profile·제품 API·Rust FFI symbol이 추가되는지 확인했다. 기존 compact fixture host/report 위의 iOS app integration만 변경한다.
2. **상태 대장과 PR 내용의 불일치** — C13.1 완료로 표시되는지 확인했다. app hit identity 후속의 Simulator 결과만 추가하고 전체 C13.1 및 공개 CSS 지원은 미완료로 유지한다.
3. **내부 인터페이스 버전 상승** — 실행 증거를 새 계약 버전으로 오해할지 확인했다. 내부 계약 0060의 숫자 버전을 올리지 않는다.
4. **규범 계약과 실험 기록 혼합** — Simulator 관찰을 제품 계약으로 발표하는지 확인했다. 내부 0060은 검증 범위와 한계를 분리해 기록한다.
5. **계획 review 재사용** — 구현 20개 검토가 계획 검토 항목을 재사용하는지 확인했다. 구현 파일은 route/parser/runtime 경로를 새 관점으로 다룬다.
6. **PR review와 구현 review 중복** — 30개 PR 관점이 구현 실패 항목을 표현만 바꾸어 반복하는지 확인했다. 이 문서는 문서 연결, artifact, 범위, CI, merge 및 제출 검증을 별도로 다룬다.
7. **PR 본문 이미지 누락·잘못된 링크** — screenshot은 PR head artifact의 raw URL을 가리킨다. agent-browser에서 PR #166 본문 image element가 complete=true로 렌더되고 해당 raw URL을 사용하는 것을 확인했다.
8. **이미지 해시와 본문 그림 불일치** — 본문 screenshot이 임의 파일로 바뀔 가능성을 확인했다. final evidence summary에 PNG SHA-256과 실행별 screenshot 이름을 보존하고 첫 실행 그림을 쓴다.
9. **이미지 자연 크기와 화면 crop** — agent-browser DOM에서 naturalWidth=1206, naturalHeight=2622를 확인했다. 이 값은 저장된 원본 PNG 크기와 같으며 PR 본문에서 이미지가 로드됐다.
10. **화면이 기능 통과를 과장** — screenshot의 완료 문구가 runtime 로그와 다를 수 있는지 확인했다. 화면과 해당 PID의 30,000/30,000·52/52 summary를 함께 보존했다.
11. **실기기 검증 주장** — iPhone 17 Pro 이름만으로 physical iPhone으로 오해할 수 있는지 확인했다. 문서와 summary는 iOS 26.2 Simulator 및 고정 UDID를 명시한다.
12. **GPU 픽셀 동등성 주장** — draw 제출을 drawable readback/RGBA exact로 표현하는지 확인했다. evidence는 readback 미수행을 분명히 적는다.
13. **DOM event/CSSOM 주장** — hit test 결과를 DOM event dispatch나 scrolling 계약으로 확대하는지 확인했다. 둘 다 미검증 경계로 반복 명시한다.
14. **WPT·공개 CSS 지원 주장** — 고정 Chrome oracle 비교를 WPT 적합성으로 오해하는지 확인했다. Chrome 155 fixture oracle 대조와 WPT를 구분하고 C13.1 미완료 상태를 유지한다.
15. **플랫폼 간 결과 합산** — 기존 Android AVD와 iOS Simulator를 동일 실행으로 합치는지 확인했다. 별도 작업 결과로 기록하며 iOS 증거는 iOS Simulator에 한정한다.
16. **oracle의 구현 후 편집** — expected identity를 앱 결과에 맞춰 바꾸는지 확인했다. 계획과 실행 evidence가 기존 Chrome oracle SHA들을 고정한다.
17. **digest의 목적 혼동** — identity·grid file·Chrome observation digest를 하나로 취급하는지 확인했다. 모두 별도 이름과 pinned value로 보존한다.
18. **run ID 재사용** — 여러 process log가 서로 섞일 수 있는지 확인했다. 실행마다 UUID, runtime generation, PID를 기록한다.
19. **fresh-process 주장** — 한 앱 process의 재시작 없는 반복을 세 실행으로 세는지 확인했다. capture summary에는 세 PID가 각각 다르며 앱 terminate 뒤 새로 launch했다.
20. **negative control의 성공 오인** — conflict route가 성공 screenshot에 섞이거나 host를 생성하는지 확인했다. 별도 PID 로그에서 host/eval 부재를 기록하고 상태는 rejection으로 분리한다.
21. **V8 source 재현성** — worktree에 없는 V8 경로가 암묵적으로 보장된다고 쓰는지 확인했다. revision·clean status·archive를 capture 단계에서 확인하고 선택 경로를 summary에 남긴다.
22. **빌드 flag 증거** — simulator build가 fixture flag 없이 성공한 것으로 혼동되는지 확인했다. build log에서 C04 runtime 및 C13.1 fixture-test flag를 확인한다.
23. **프로젝트 파일 누락** — parser가 Xcode target에서 제외되는지 확인했다. PBX file reference와 Sources build phase entry를 포함한다.
24. **테스트 기록 과장** — 증거 파일과 실행 결과를 섞거나 실행하지 않은 gate를 PASS로 세는지 확인했다. parser mutation, FFI 8/8, workspace tests/Clippy, rustfmt와 iOS build/runtime의 개별 logs를 보존한다.
25. **parser negative controls 수 과장** — mutation count가 FFI case까지 포함하는지 확인했다. README는 parser 5 positive·23 negative와 FFI 8/8을 분리한다.
26. **stale evidence 연결** — 최신 근거 대신 중간 run folder를 문서가 가리킬 수 있는지 점검했다. 발견한 153802… 참조를 authoritative 154338… 실행으로 바꾼다.
27. **실패 run 파일의 authoritative 오해** — 불완전한 캡처가 성공 증거에 섞이는지 확인했다. 재현 디버깅에만 쓰인 폴더는 목록과 크기를 확인해 final evidence tree에서 정리한다.
28. **공유 worktree 변경 유입** — root checkout의 다른 사용자 작업이 PR에 들어가는지 확인했다. 격리 worktree에서 의도한 파일만 stage하고 root의 dirty 변경은 보존한다.
29. **라벨·템플릿 조건** — PR에 작업 유형·주요 영역 라벨이 있고 template의 status/contract/evidence/remaining scope가 채워지는지 확인한다. 기존 label을 재사용하고 신규 label은 만들지 않는다.
30. **merge 이후 상태 참조** — merge 전 SHA를 최종 상태로 기록하거나 squash/merge commit 방식을 잘못 쓰는지 확인한다. Rebase and merge 후 실제 merge SHA, checks, branch protection 결과를 확인해 STATUS와 이 문서를 동기화한다.

## PR 게시 시점 상태

- PR: https://github.com/ohah/spinon/pull/166
- screenshot raw URL: https://raw.githubusercontent.com/ohah/spinon/feat/c13-1-ios-app-hit-identity-grid/spec/internal/evidence/c13-1-ios-app-hit-identity-grid-2026-10-15/c13-1-ios-hit-grid-20261014T154338634Z-f1835c60/run-1-ios-simulator.png
- agent-browser 결과: PR 본문 image complete=true, natural dimensions 1206×2622
- 검토 당시 GitHub 상태: PR 열림, MERGEABLE, merge state CLEAN. branch main 보호 응답에 required status checks 항목이 없고 gh pr checks는 “no checks reported”를 반환했다.
- 프로젝트 규칙에 맞춰 Rebase and merge를 사용한다. 최종 병합 commit은 GitHub PR metadata와 상태 대장에서 확인한다.
