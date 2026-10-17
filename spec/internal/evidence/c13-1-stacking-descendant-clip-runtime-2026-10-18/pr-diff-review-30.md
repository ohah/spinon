# C13.1 복합 stacking descendant PR diff 검토 30개

구현 계획의 20개 리뷰와 별도로, 최종 PR diff가 주변 계약·증거·병합 경계에 미치는 영향을 30개 관점으로 대조했다.

1. C13.1 전체 작업과 이 복합 stacking fixture slice의 경계가 분명한가 — 제한 fixture로 표기했다.
2. C13 상위 항목이 완료 처리되지 않았는가 — 체크 미완료 유지.
3. 이 PR이 공식 상태 체크를 변경했는가 — 변경하지 않았다. 현재 로드맵 합계는 상태 대장에서 동적으로 읽으며 C13.1/C13은 미완료다.
4. 날짜 기준만 최신 실행일로 갱신했는가 — STATUS 기준 2026-10-18.
5. 기존 plan/oracle 단계 문구가 과거 상태로 남지 않았는가 — runtime 결과 링크로 교체했다.
6. 계획 문서가 implementation 완료를 제품 지원 완료처럼 표현하지 않는가 — 제한 fixture 수준으로 구분했다.
7. 내부 계약 숫자 버전을 올리지 않았는가 — `0.1.0-draft` 유지.
8. 내부 계약에 새 사례의 색·hit 관계가 추가됐는가 — 18점과 stacking 경계를 기록했다.
9. 상태 대장에서 근거 파일까지 경로가 유효한가 — 저장소 내 상대 링크를 확인한다.
10. 내부 README 인덱스가 신규 근거를 찾을 수 있는가 — plan·runtime·feature·reviews 연결.
11. 기존 Chrome oracle을 재생성 결과와 혼동하지 않는가 — source hash와 fixed oracle 링크 유지.
12. screenshot과 hit oracle의 역할이 분리됐는가 — paint color와 hit identity를 따로 명시했다.
13. Rust fixture adapter가 `fixtures/` 경계에 있는가 — 작업 ID 파일과 모듈 import를 확인했다.
14. FFI declaration과 implementation symbol 이름이 일치하는가 — header/native/FFI를 대조한다.
15. unsafe ABI safety 문구가 호출자 buffer 계약을 설명하는가 — header와 Rust 함수 주석을 확인했다.
16. null/짧은 output buffer가 fail-closed인가 — focused FFI tests와 code path를 확인했다.
17. Cargo opt-in feature가 Android/iOS와 맞는가 — 동일 C13.1 feature flag로 app build한다.
18. default archive에서 fixture symbol/payload가 빠지는가 — SHA·strings·symbol evidence를 연결했다.
19. LLVM nm 경고를 숨기거나 성공으로 포장하지 않았는가 — tool incompatibility와 보조 확인을 기록했다.
20. Android JNI report가 expected sample 수와 연결되는가 — 18 sample logging/summary를 확인했다.
21. Android surface capture가 draw revision과 같은 scene을 읽는가 — generation/document/tree/style equality guard가 있다.
22. Android surface generation 또는 presentation sequence가 바뀐 callback이 배제되는가 — callback validity guard를 확인했다.
23. Android ADB 대상이 승인된 AVD뿐인가 — evidence command는 `emulator-5554`만 지정한다.
24. iOS fixture route가 다른 C13 경로와 동시에 실행될 수 있는가 — route conflict negative log는 host/eval 전 거부한다.
25. iOS stale probe extra가 fixture route 검사에 안전하게 포함됐는가 — stacking route modifier로만 허용하고 테스트 실행에서 readback 전에 차단했다.
26. iOS Metal manifest가 run/PID/generation/drawable/format/hash를 보존하는가 — sampleCount 18·complete를 기록했다.
27. 프로세스 간 run 혼동이 생길 수 있는가 — 각 플랫폼 3개의 PID/run ID를 별도 표기했다.
28. 초기 테스트 실패를 성공 근거에 합쳤는가 — 잘못된 `-e` string extra와 수정된 `--ez`, 수정 전후를 분리했다.
29. 시뮬레이터 결과가 실기기·compositor·WPT 완료로 표현되는가 — 각각 미검증 한계를 명시했다.
30. PR 본문·라벨·screenshots·rebase merge가 프로젝트 규칙에 맞는가 — PR 생성 직전 양 플랫폼 대표 screenshot과 template/labels를 대조하고 병합은 보호 체크 통과 후 수행한다.

**결과:** 확인된 문서·증거·코드 결함은 반영했다. PR 제출 시점에 remote diff, label, image attachment와 required checks를 다시 확인한다. 리뷰 완료는 remote CI 또는 C13.1/C13 전체 완료를 뜻하지 않는다.
