# C11.4 text-transform 병합 후 상태 문서 검토 30개

- 대상: PR [#952](https://github.com/ohah/spinon/pull/952) 병합 뒤 `plan/`, `spec/STATUS.md`, 내부 계약 0059 및 실행 README의 상태 동기화
- 검토 시작 기준: `be62a4dd3c1b67ada2db094f19a76231e9f3df35`
- 범위: 문서 diff와 연결된 실행 근거·PR 메타데이터. runtime 코드는 바꾸지 않았다.
- 판정: 체크 수, merge metadata 또는 evidence 경계를 잘못 표시하는 문서 결함은 찾지 못했다.

1. **C11 상위 상태가 잘못 완료될 위험** — `spec/STATUS.md`의 C11 상위 checkbox가 unchecked인지 확인했다.
2. **C11.4 전체 완료 오인** — anonymous text-transform slice의 병합을 C11.4 intrinsic track sizing 전체 완료로 확대하지 않고 unchecked로 유지한다.
3. **공식 집계 증가 오인** — `72/213`이 계획, 상태 대장, 내부 계약에서 동일하게 유지된다.
4. **부분 구현을 완료 항목으로 세는 위험** — PR #952를 제한 fixture slice로 기록하고, 공식 완료 집계가 바뀌지 않았음을 확인했다.
5. **PR 번호 오류** — 세 문서와 evidence README가 모두 PR #952를 가리킨다.
6. **병합 시각 오류** — GitHub `mergedAt` `2026-10-31T23:48:28Z`를 기록한 값과 대조했다.
7. **merge SHA 오류** — 최종 `be62a4dd3c1b67ada2db094f19a76231e9f3df35`를 GitHub PR metadata와 대조했다.
8. **merge 방식 오류** — Rebase and merge로 기록했으며 squash/merge commit이라고 쓰지 않았다.
9. **CI 상태 과장** — GitHub가 해당 branch에 checks를 보고하지 않은 사실을 적고 CI 성공으로 표현하지 않는다.
10. **로컬 검사와 CI 혼동** — Rust/Node/Clippy/rustfmt 통과 수는 local candidate 검증으로 남고 CI 결과로 옮기지 않았다.
11. **시뮬레이터 대상 혼합** — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각각 6/6을 분리해 유지한다.
12. **실기기 검증 과장** — AVD 실행을 실기기 결과로 쓰지 않는다.
13. **Android GPU 경로 과장** — SwiftShader AVD 결과를 hardware GPU 증거로 쓰지 않는다.
14. **iOS 표시 단계 과장** — Metal drawable readback을 compositor 이후 presentation이라고 쓰지 않는다.
15. **screenshot 의미 과장** — `layout=ready boxes=1` 앱 상태 화면만 나타내며 glyph pixel 검증이 아니라고 유지한다.
16. **원문 텍스트 변경 오인** — HostDocument source `IIII ii`는 보존되고 layout buffer만 바뀐다는 범위를 유지한다.
17. **Unicode casing 과장** — ASCII 전용 변환으로 표현하며 일반 Unicode/locale casing을 지원한다고 쓰지 않는다.
18. **언어 상속 범위 과장** — exact direct `lang="en"` fixture 전제를 유지하며 임의 언어 상속 지원을 주장하지 않는다.
19. **inline run 범위 과장** — inline styles/runs는 지원 범위 밖이라고 명시한다.
20. **CSS admission 범위 과장** — fixed opt-in fixture만 병합되었고 author CSS 일반 admission이 아니라고 유지한다.
21. **WPT 적합성 과장** — WPT/전체 CSS Grid 지원은 완료하지 않았다고 유지한다.
22. **공개 API 추가 오인** — 공개 API 없이 내부 Grid contract만 갱신된다고 명시한다.
23. **계약 버전 상승 오인** — 내부 contract numeric version `0.1.0`을 유지한다.
24. **failure artifact 유실** — 기존 거부·superseded capture 기록과 최신 passing matrix를 분리한 링크가 유지된다.
25. **archive 검사 범위 과장** — raw static archive marker scan을 APK/IPA 전체 bundle exclusion으로 확대하지 않는다.
26. **archive 도구 한계 누락** — Xcode `nm`의 LLVM object 호환성 한계를 유지한다.
27. **source manifest 낡은 hash** — 계획 문서 수정 뒤 36개 입력 hash를 다시 계산했고 현재 내용과 모두 일치한다.
28. **evidence link 단절** — plan, STATUS, contract에서 실행 README와 리뷰 기록을 상대 경로로 연결한다.
29. **PR diff 검토 누락** — 구현 PR의 별도 30-point diff 검토 기록 링크와 범위는 유지된다.
30. **문서-only 범위가 runtime 변경처럼 보일 위험** — 이번 상태 동기화 branch는 문서/evidence metadata만 변경하고 runtime source를 바꾸지 않는다.
