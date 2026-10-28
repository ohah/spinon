# PR #743 후속 문서 동기화 PR diff 검토 · 20개 관점

검토 범위는 `spec/STATUS.md`, #738 집중 계획, `spec/public/roadmap.html`의 source-tracking correction 기록이다. runtime 동작을 새로 검증했다고 주장하지 않으며, 앞선 #743 병합의 실제 GitHub metadata 및 main의 focused test 결과와 대조했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11.4 parent checkbox가 source 수리만으로 완료 처리된다 | checkbox는 계속 `[ ]`다. |
| 2 | 공식 완료 수가 `72/213`에서 변한다 | 상태 대장·계획·공개 HTML 모두 `72/213`을 유지한다. |
| 3 | PR #743 merge 시각을 다른 timezone/시점으로 적는다 | GitHub merge metadata `2026-10-28T06:50:44Z`와 일치한다. |
| 4 | PR #743 merge SHA가 틀리다 | `925113bde86055362c89fa32d505690044a26313`으로 GitHub merge commit과 맞는다. |
| 5 | #738에서 빠졌던 경로를 잘못 기록한다 | 실제 부모의 `#[path]` 대상 `generic_intrinsic_flex_fixed/visibility_hidden.rs`를 가리킨다. |
| 6 | 원인을 코드 동작 오류라고 잘못 분류한다 | 수정은 이미 참조된 source의 Git tracking 복구이며 동작 변화가 없다고 썼다. |
| 7 | 당시 로컬 테스트 결과를 깨끗한 merge tree 결과로 포장한다 | 계획은 #738 당시 미추적 local source를 썼다고 분리하고, #743 뒤 main 재실행 1/1을 별도로 적는다. |
| 8 | fixture test 개수가 부풀려진다 | main에서 실행한 focused test는 1 passed로 제한해 기록한다. |
| 9 | package library Clippy를 workspace 통과로 확대한다 | package library 결과와 workspace all-target 실패를 다른 문장으로 둔다. |
| 10 | workspace Clippy의 FFI failure가 감춰진다 | `spinon-ffi/src/runtime_layout.rs:139`의 누락 field 문제를 적었다. |
| 11 | 기존 `map_identity` warning이 새 파일 결함으로 돌아간다 | 경로 `generic_direct_text_image/fixture.rs:311`를 명시하고 기존 code라고 분리한다. |
| 12 | PR checks가 없는데 성공 CI로 보인다 | PR checks 미보고 상태를 기록하며 통과라고 하지 않는다. |
| 13 | status와 상세 plan의 #743 기록이 서로 다르다 | 두 문서의 PR, merge SHA, focused test 결과 및 count 유지가 일치한다. |
| 14 | public roadmap이 이전 #738 설명만 보여 source correction을 숨긴다 | 기존 #738 card 바로 뒤에 별도 correction card를 추가했다. |
| 15 | roadmap card가 구현 기능 추가로 오독된다 | card 제목과 설명은 source tracking correction으로 한정한다. |
| 16 | 공개 API나 내부 계약 version을 올린 것으로 읽힌다 | 문서 diff는 버전 명세나 public contract를 추가하지 않는다. |
| 17 | 이번 수정에서 Android/iOS를 다시 실행했다고 오인한다 | 앱 재실행이나 새 simulator 결과를 주장하지 않는다. |
| 18 | #738 screenshot을 #743 새 캡처로 재사용한다 | 이 docs-only PR에는 새 실행 화면을 넣지 않고 기존 캡처를 새 결과처럼 복제하지 않는다. |
| 19 | 링크가 없는 파일이나 PR을 가리킨다 | PR #743, recovery README, 집중 계획, 상태 대장은 현재 main의 tracked 경로다. |
| 20 | Tailnet에 이미 배포했다고 주장한다 | 이 diff는 source HTML만 수정한다. Tailnet 동기화와 Agent Browser 검증은 별도 후속 작업으로 남긴다. |

**검토 결과:** 문서 세 곳이 #743의 추적 복구를 같은 범위와 상태로 반영하고 C11.4 공식 완료 판정을 그대로 유지한다. Tailnet 서버 반영은 이 PR diff에 포함되지 않는다.
