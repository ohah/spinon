# E08 시작 시간 최적화 · PR diff 적대 검토 30개

- **PR:** [#1385](https://github.com/ohah/spinon/pull/1385)
- **최초 검토 commit:** `dbbc1a8166e7870b25ef90ae757cae546d3e458b`
- **검토 base:** `04947b4a4055ce315a221194feb7f6c2460c9f16`
- **최초 functional diff:** 계획과 계획 검토 기록 Markdown 2개 추가. 이 review artifact는 뒤에 별도 추가했다.
- **검토 구분:** 아래 30개는 PR 변경 범위, 링크·provenance, GitHub metadata와 검증 주장의 무결성을 각각 확인한다. 계획의 기술 내용을 다시 검증하는 30개 구현 실패 경로라는 뜻은 아니며, 계획 검토 20개와 합쳐 50개의 고유 제품 실패 경로라고 주장하지 않는다. 일부 주제는 같은 계획을 대상으로 하므로 겹칠 수 있지만, 검토 질문은 PR 경계에 한정한다.

## 변경 범위와 파일 provenance

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 1 | PR base SHA가 검토 기준과 달라 비교 대상이 틀림 | **PASS.** 최초 검토 base `04947b4…`는 당시 `main` 및 PR base SHA와 일치했다. |
| 2 | head branch에 다른 작업 commit이 섞여 계획 diff에 잘못 귀속 | **PASS.** 최초 PR commit `dbbc1a8…`의 base 비교는 이 계획 작업의 두 Markdown 추가를 보였다. |
| 3 | 설명한 두 파일 외에 다른 경로가 diff에 포함 | **PASS.** 최초 functional diff 경로는 E08 계획과 계획 검토 기록 두 개였다. |
| 4 | Markdown 문서라고 표시했지만 source, fixture 또는 binary가 실수로 포함 | **PASS.** 두 경로 모두 일반 Markdown이며 code·fixture·binary 변경은 없었다. |
| 5 | root checkout의 C11 WIP가 PR에 섞여 공유 | **PASS.** C11 source·실행 로그는 root checkout에만 있고 PR diff에는 없었다. |
| 6 | C11 실행 log가 E08 결과 증거처럼 PR에 추가 | **PASS.** log는 PR 파일·첨부 증거에 없고 계획 안에서도 당시 untracked 동시성 맥락이라고 구분한다. |
| 7 | `spec/STATUS.md` 또는 공식 완료 집계 변경이 숨겨짐 | **PASS.** STATUS 경로는 최초 diff에 없고 PR 본문도 미변경을 정확히 말한다. |
| 8 | 초안 계획을 E08 완료로 표시해 roadmap 상태와 충돌 | **PASS.** 계획과 본문 모두 E08을 미완료·미측정으로 명시한다. |
| 9 | 문서 전용이라는 본문 설명과 실제 파일 종류가 충돌 | **PASS.** 최초 diff에는 문서 파일만 있다. |
| 10 | 동작 화면 캡처를 했거나 첨부했다고 잘못 표시 | **PASS.** UI 변경과 캡처가 없다고 본문에 적고 PR에 screenshot artifact가 없다. |

## 링크와 근거 연결

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 11 | PR 본문의 계획 링크가 다른 저장소나 잘못된 경로를 가리킴 | **PASS.** 링크는 `ohah/spinon`의 `plan/e08-startup-optimization.md`를 가리킨다. |
| 12 | PR 본문의 20개 검토 기록 링크가 누락되거나 잘못된 경로를 가리킴 | **PASS.** 링크 target `plan/e08-startup-optimization-plan-review-20.md`가 최초 diff에 있다. |
| 13 | PR 본문 링크 SHA와 해당 시점에 검토한 commit이 다름 | **PASS.** 최초 본문에 있던 계획·20개 검토 링크는 검토 당시 `dbbc1a8…` commit에 고정됐다. 이후 review artifact 링크는 추가 commit에서 별도 확인했다. |
| 14 | 계획 문서의 공식 상태 링크가 존재하지 않는 STATUS 경로 | **PASS.** `../spec/STATUS.md` 경로가 repository에 있다. |
| 15 | 계획 문서의 UI·runtime·V8 기준 링크 중 하나가 잘못된 파일명 | **PASS.** `spec/0002-ui-tree-events.md`, `spec/0004-runtime-build.md`, `native/v8/VERSION.md` 경로가 존재한다. |
| 16 | 계획 문서의 코드 참조가 없는 RuntimeSession 또는 V8 adapter 파일을 지목 | **PASS.** `crates/spinon-runtime/src/session.rs`와 `native/v8/src/spinon_v8.cc`가 존재한다. |
| 17 | 계획 문서의 bootstrap 입력 링크가 누락된 package/example 경로를 지목 | **PASS.** `package.json`, `examples/bootstrap/index.html`, `examples/bootstrap/root-app.js`가 존재한다. |
| 18 | review-20 문서의 상대 링크가 plan 디렉터리 기준에서 잘못 해석 | **PASS.** 문서 내 plan 링크 및 계획의 review-20 링크가 repository 내 실제 파일로 해석된다. |
| 19 | 로컬 절대 경로 또는 `file://` URL이 public evidence인 것처럼 노출 | **PASS.** PR 본문 링크는 GitHub permalink이며 추적 불가능한 local URL은 없다. 로컬 C11 log는 공개 결과 링크로 쓰지 않는다. |
| 20 | “20개” 검토 artifact의 표 행 수나 검토 범위가 제목과 불일치 | **PASS.** review-20에는 1–20 항목과 계획 검토의 범위·비실행 경계가 기록돼 있다. |

## PR metadata와 검증 주장

| # | 적대 검사 | 판정과 근거 |
|---:|---|---|
| 21 | 제목이 E08 계획 문서 변경임을 식별하지 못함 | **PASS.** 제목은 `docs(e08): 시작 시간 실험 계획`이다. |
| 22 | PR에 실제 필수 type/area label이 없거나 본문 선언과 불일치 | **PASS.** 실제 labels는 `spike`, `documentation`, `area: runtime`이며 본문과 일치한다. |
| 23 | PR 본문이 변경 항목·API 명세·동작 근거·남은 범위 양식을 빠뜨림 | **PASS.** PR template 해당 항목이 작성돼 있고 API 변경 없음도 명시한다. |
| 24 | 공개 API 또는 인터페이스 명세 링크가 없는 이유를 감춰 검토자가 누락으로 오해 | **PASS.** “해당 없음”과 그 이유를 본문에 적었다. |
| 25 | 문서만 바뀌었는데 제품 동작의 시각 검증 결과를 주장 | **PASS.** 캡처가 없고 UI 동작 변경도 없다고 본문에 명시했다. |
| 26 | 실행하지 않은 test/build/benchmark/browser/simulator를 통과했다고 표시 | **PASS.** 본문은 이 작업에서 모두 실행하지 않았다고 명시한다. |
| 27 | commit status가 비었는데 CI green이라고 주장 | **PASS.** 최초 검토 시 status 결과는 비어 있었고 PR 본문·검토 기록은 green을 주장하지 않았다. |
| 28 | workflow run이 비었는데 필수 자동 검증이 통과했다고 주장 | **PASS.** 최초 검토 시 PR workflow run은 비어 있었다. 실행 통과 주장은 없다. |
| 29 | GitHub `mergeable=true`를 CI 또는 코드 검토 통과로 혼동 | **PASS.** 문서는 mergeability와 빈 CI 상태를 분리한다. Mergeability는 required checks 통과로 표시하지 않는다. |
| 30 | 남은 범위가 C11 terminal gate나 E08 미완료 상태를 누락 | **PASS.** 본문은 C11.4 terminal evidence 전 E08 build·앱 실행·benchmark·tracing 금지와 A/V8/threshold의 미결 항목을 기록한다. |

## 판정 경계

이 검토는 PR artifact의 diff·링크·metadata·주장 무결성을 확인한다. 계획에 적힌 V8 API 동작, 성능, UI 또는 구현을 새로 검증하지 않으며, 테스트·빌드·브라우저·시뮬레이터를 실행하지 않았다. 원래 두 파일 diff를 검토한 뒤 이 보고서를 추가했으므로, 추가된 파일까지 포함한 최종 PR tree와 본문 permalink는 후속 final check에서 별도 확인한다. 이 검토 기록은 E08 측정·구현 근거가 아니다.
