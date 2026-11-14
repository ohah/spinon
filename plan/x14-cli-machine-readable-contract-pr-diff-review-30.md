# X14 CLI 기계 판독 계약 · PR diff 적대 검토 30개

- **PR:** [#1386](https://github.com/ohah/spinon/pull/1386)
- **최초 검토 head:** `9d368f29a61b3cd5e12f14ca1ef3cbd62f0d7028`
- **base:** `1325ae5bbbd8eceb650129cb98829beed8659952`
- **최초 functional diff:** X14 계획과 계획 검토 기록 Markdown 2개 추가.
- **검토 구분:** 세 렌즈가 PR 범위·계약 주장(1–10), 파일·링크·provenance(11–20), metadata·검증 주장·자원 gate(21–30)를 확인했다. 이는 PR artifact의 무결성 점검이며 계획 검토와 합쳐 50개의 고유 제품 실패 경로라고 주장하지 않는다. 실행하지 않은 CLI 동작과 자동 검증은 GitHub metadata로 입증할 수 없으므로 작업자 보고와 CI evidence를 구분한다.

## PR 범위와 계약 주장

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 1 | 제목·본문의 roadmap ID가 PR 변경과 다름 | **PASS.** 제목은 `docs(x14)`이고 본문은 X14 CLI 계획으로 범위를 특정한다. |
| 2 | X14 체크 또는 공식 완료 집계를 갱신했다고 잘못 표시 | **PASS.** 실제 diff는 `STATUS.md`를 바꾸지 않으며 본문도 X14 미완료·집계 미변경을 말한다. |
| 3 | 계획 초안을 현재 CLI 구현이나 기존 동작으로 오인 | **PASS.** 계획과 본문은 구현·지원 선언을 분리하고 CLI가 이미 존재한다고 말하지 않는다. |
| 4 | S13 구현 선행 조건을 X14 계획 작성까지 앞당김 | **PASS.** 최신 본문은 계획을 지금 진행할 수 있고 구현은 S13 이후라고 구분한다. |
| 5 | X13 소유 데이터 없이 `inspect` 성공 payload를 주장 | **PASS.** inspect 성공 필드와 판정은 X13 계약 전까지 미해결이다. |
| 6 | X12 JS profile과 R05 frame attribution의 출처를 합침 | **PASS.** 계획과 본문이 서로 다른 profile kind/source와 clock 경계를 둔다. |
| 7 | X15가 CLI 공통 결과 envelope를 사용한다는 의존을 누락 | **PASS.** 계획은 stable CLI/MCP 결과에 공통 envelope/version/error 결정이 필요하다고 하고 transport/tool 구현은 분리한다. |
| 8 | 미정 JSON key/version/exit code를 이미 정한 규격으로 표현 | **PASS.** 정확한 schema와 호환성 정책은 승인 전 미결이다. |
| 9 | 공개 API, CLI/MCP 호환성 또는 X14 완료를 주장 | **PASS.** 본문은 versioned contract나 machine-readable compatibility를 게시하지 않는다고 한다. |
| 10 | 남은 선행 결정과 C11 gate를 PR에서 감춤 | **PASS.** 본문에는 X13/X12/R05 payload 소유와 C11 terminal HEAD/log gate가 기록돼 있다. |

## 파일·링크·provenance

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 11 | PR base/head 또는 검토 대상 diff가 다른 commit을 가리킴 | **PASS.** GitHub base `1325ae5…`와 head `9d368f2…`가 계획 작업 경계와 일치한다. |
| 12 | 선언한 두 파일 외에 다른 경로가 diff에 섞임 | **PASS.** 실제 diff는 계획서와 20관점 검토 기록 두 Markdown 추가뿐이다. |
| 13 | source, fixture, 실행 결과 또는 generated binary가 문서 PR에 포함 | **PASS.** 해당 파일은 없다. |
| 14 | manifest, lockfile, build output 또는 Cargo/V8 artifact가 포함 | **PASS.** diff에 없다. |
| 15 | root C11 WIP나 untracked log가 PR tree로 복사됨 | **PASS.** C11은 동시성 조건으로만 언급되며 root source/log/evidence는 변경되지 않았다. |
| 16 | 최초 PR 본문의 plan/review 링크가 검토 당시 SHA·경로를 벗어남 | **PASS.** 초기 본문 링크는 최초 검토 head `9d368f2…`에 고정돼 resolve됐다. artifact 추가 뒤 본문은 최종 head로 갱신했고 final check에서 링크를 다시 열어 확인했다. |
| 17 | 문서의 상대 링크가 저장소 내 없는 파일을 가리킴 | **PASS.** STATUS, `spec/0005`, CLI·MCP 설계 및 plan 상대 링크가 존재한다. |
| 18 | 계획 검토 artifact의 표 행 수와 “20” 제목이 불일치 | **PASS.** 검토 표에 1–20 번호가 각 한 번 있다. |
| 19 | 계획 검토의 세 렌즈가 같은 행을 중복하거나 분배 수가 불일치 | **PASS.** 기록은 #1–7, #8–14, #15–20으로 7/7/6을 지정한다. |
| 20 | 첫 PARTIAL 수정과 재검토 결과가 과장되거나 모순 | **PASS · provenance 한정.** 기록은 X15, profile source, fallback 문구 세 항목을 수정·재검토해 PASS로 남긴다. 중간 초안 commit은 보존하지 않아 chronology는 검토자/작업자 기록에 의존한다. |

## PR metadata·검증 주장·자원 경계

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 21 | PR type/area labels가 없거나 본문 선언과 다름 | **PASS.** 실제 label은 `documentation`, `planning`, `area: devtools`이며 본문과 일치한다. |
| 22 | PR template의 범위·명세·근거·남은 작업 항목을 누락 | **PASS.** 각 섹션을 작성하고 API 해당 없음의 이유를 설명한다. |
| 23 | versioned API spec이 추가됐다고 잘못 연결 | **PASS.** 본문은 이번 PR이 제안 계획뿐이라고 명시한다. |
| 24 | 제품 화면 변경이 없는데 screenshot 또는 GUI 동작을 증거로 주장 | **PASS.** UI 변경과 캡처가 없다고 본문에 적혀 있다. |
| 25 | CLI/build/test/device/browser 실행을 CI 또는 독립 근거로 표시 | **PASS · 작업자 보고.** 본문은 미실행이라고 밝힌다. GitHub metadata는 로컬 실행 부재를 독립 증명하지 않으므로 이 주장은 작업 기록 범위다. |
| 26 | 빈 combined status를 성공 CI로 해석 | **PASS.** status 목록은 비어 있어 CI green 주장을 하지 않는다. |
| 27 | 빈 workflow 목록을 자동 검증 완료로 표현 | **PASS.** PR workflow runs가 없으며 실행 통과 주장은 없다. |
| 28 | `mergeable=true`를 approval 또는 required checks 통과로 오해 | **PASS · 한정.** mergeability는 GitHub metadata일 뿐 승인·CI 성공을 뜻하지 않는다. PR은 검토 시점에 open 상태였다. |
| 29 | 수정된 PR 본문이 plan의 planning-now/implementation-later 경계와 다시 어긋남 | **PASS.** 최종 본문은 계획을 지금 진행하고, 구현을 S13 이후, payload를 X13/X12/R05 source 이후로 둔다. 공통 CLI/MCP envelope 결정도 기록한다. |
| 30 | 활성 C11 matrix 중 CLI execution/build/test를 병렬 실행하게 만듦 | **PASS.** 검토 시 C11 PID 98424가 활성이고 log가 terminal 상태가 아니었다. plan/body 모두 terminal result·HEAD·log hash 전 실행을 막는다. |

## 판정 경계

최초 두 파일 diff를 세 렌즈에서 검토한 뒤 이 artifact를 별도 추가했다. artifact 추가 뒤 세 렌즈가 최종 3파일 tree, 본문 permalink, labels, status/workflow와 mergeability를 재확인했다. 이 점검은 X14 CLI 구현, JSON output runtime, device/browser 동작 또는 성능을 검증하지 않는다. status/workflow가 비어 있다는 사실은 통과한 CI가 있다는 뜻이 아니다.
