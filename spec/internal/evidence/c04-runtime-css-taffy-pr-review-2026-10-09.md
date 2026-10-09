# C04.9 · PR 변경 적대 검토

**검토일:** 2026-10-09 · **검토 대상:** 구현·내부 계약·상태 대장·시뮬레이터 근거의 최종 변경 묶음 · **판정:** 아래 항목 수정/재확인 뒤 merge 가능

이 표는 구현 경로 검토와 구분해 PR 전체의 변경 일관성·유지보수 경계·완료 표현을 확인한다.

| # | PR 경계 | 판정 |
|---:|---|---|
| 1 | worktree에 C04.9와 직접 관계없는 제품 변경이 섞였는가 | feature branch의 Rust cascade/layout, JNI/iOS fixture, 해당 내부 문서·근거에 한정했다. build output은 gitignore 대상이다. |
| 2 | Cargo package numeric version 또는 앱 출시 버전이 올라갔는가 | 오르지 않았다. 변경 Cargo.lock은 spinon-runtime의 local dependency 항목만 반영하고 모든 Spinon crate는 0.1.0이다. |
| 3 | 계약의 성숙도 변경만으로 숫자 버전을 올렸는가 | 새 내부 계약 0030과 구현 계획은 출시 전 0.1.0 고정을 명시한다. docs/project-rules.md의 고정 정책과 일치한다. |
| 4 | JSON schema 이름에 매 구현마다 새 숫자를 붙였는가 | 새 layout schema는 spinon.runtime.layout으로 고정하고 숫자 suffix를 넣지 않았다. |
| 5 | 기능 상태 대장·내부 명세 인덱스와 실제 구현 범위가 일치하는가 | C04.9만 완료로 제안하고 GPU 전달·전체 CSS·제품 화면은 열어 둔다. 내부 인덱스의 0030을 번호 순서 표에 넣고 빠져 있던 0027 링크를 복원했다. |
| 6 | C04 parent가 완료된 것처럼 보이는가 | C04 전체와 C01 전체는 미완료로 유지했다. 공개 지원 API 완료를 선언하지 않았다. |
| 7 | 미구현 author stylesheet, CSSOM, 외부 URL loader가 구현됐다고 오해할 수 있는가 | 0030과 상태 대장에 별도 미지원 범위로 표시했다. 외부 CSS URL 로더 기존 미구현 항목은 보존했다. |
| 8 | 계획·명세·코드가 flex shorthand 지원 경계에서 서로 다른가 | Stylo expanded longhand 검사, flex: 0 1 auto 허용, flex: 1 percentage basis 거부를 계획·0030·tests가 같은 방향으로 설명한다. |
| 9 | 새 API의 소유자와 스레드 설명이 런타임 코드와 다른가 | 기존 세션별 CSS worker가 순차 수행하고 FFI는 현재 JSON을 복사한다. 새 worker를 추가하지 않았다. |
| 10 | 화면 캡처를 GPU CSS paint 성공으로 오인하게 하는가 | 증거 문서가 캡처는 diagnostic native UI와 JSON readback이며 GPU 표시 근거가 아니라고 적는다. |
| 11 | Android 실행 환경을 실기기로 과장하는가 | emulator model/API 37 기록만 사용하고 실기기는 미실행으로 표시했다. |
| 12 | iOS 실행 환경을 실기기·광학 presentation으로 과장하는가 | iPhone 17 Pro / iOS 26.2 Simulator로만 기술하고 실기기/presentation은 미검증이다. |
| 13 | Chromium 비교가 고정 기준 대신 구현 후 기준에 맞춰졌는가 | 사전 고정 fixture/reference를 사용했고 입력·reference 파일은 변경하지 않았다. |
| 14 | geometry 허용오차가 문서와 probe/test에서 달라지는가 | precomparison, 구현 계획, 0030, Rust test와 실제 probe가 좌표별 0.5 CSS px 기준을 쓴다. |
| 15 | 시간 표본으로 성능 우위를 주장하는가 | 단일 실행 값을 실행 흔적으로만 보존하고 warm-up·실기기 측정 부재 및 성능 비교 금지를 명시했다. |
| 16 | CSS worker startup/snapshot 값이 layout projection duration과 혼동되는가 | C04.8 cascade timing과 C04.9 projectionDurationUs의 포함 구간을 별도로 설명했다. |
| 17 | 새 API 함수 선언·JSON 필드·오류·buffer 규칙이 서로 일치하는가 | Rust export, C header, 0030 계약, FFI helper test와 one-byte retry probe를 대조했다. |
| 18 | 로그와 screenshot이 같은 최신 build에서 나왔고 확인 가능한가 | Android·iOS를 각각 다시 빌드·설치·실행한 뒤 로그와 캡처를 저장하고 SHA-256을 기록했다. |
| 19 | 문서 작성 과정에서 사용자가 금지한 배포나 링크를 실행하는가 | GitHub Pages 배포, Tailnet 링크 삽입, Tailnet 업로드는 이 PR에서 하지 않는다. |
| 20 | 필수 검증·한글 문서 규칙·rebase merge 흐름과 맞는가 | Bun 통합 suite, Rust workspace test·Clippy, Android/iOS Simulator 실행을 통과했고 신규 설명은 한글로 작성했다. GitHub PR의 라벨·필수 check·mergeability는 PR 생성 후 병합 전에 별도로 확인한다. PR 제목 type prefix만 영어로 둔다. |

## 변경 후 재검증

최종 변경 묶음은 Bun 통합 suite에서 JavaScript 2개, CSS reference 19개, Android touch analyzer/capture 29개가 모두 통과했다. Rust workspace 252 tests와 doc-test, Clippy warnings-as-errors, Android/iOS Simulator의 실제 V8 runtime probe도 완료했다. 별도 성능 주장이나 전체 제품 CSS 완료 선언은 하지 않는다.

PR을 올린 뒤에는 GitHub가 계산한 mergeability와 필수 check를 다시 확인하고, 병합 직전 diff·버전·증거 파일을 재대조한다.
