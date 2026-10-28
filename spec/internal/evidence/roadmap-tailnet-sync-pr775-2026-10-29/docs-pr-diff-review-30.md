# PR #775 병합 후 상태·공개 로드맵 문서 변경 검토

대상은 PR #775 병합 사실을 기록하는 `spec/STATUS.md`와 C11.4 최근 작업을 보여주는 `spec/public/roadmap.html`이다. 구현 동작을 다시 판정하지 않고, 병합 메타데이터·공식 상태·공개 문구가 서로 같은 범위를 말하는지와 기존 로드맵 기록이 보존되는지를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 PR 번호를 병합 기록에 연결 | 두 페이지 모두 C11.4 implicit-column slice의 PR #775를 사용한다. |
| 2 | 잘못된 병합 결과를 기록 | GitHub PR API의 `merged` 상태와 merge SHA `2a8cb22307ca21fd6a9ed1c55aa6181d4e3eb610`을 대조했다. |
| 3 | 병합 시각을 로컬 시간대와 혼동 | `2026-10-28T22:34:46Z` API 값을 그대로 기록했다. |
| 4 | 저장소 정책과 다른 병합 전략을 표시 | PR #775는 저장소에서 허용된 Rebase and merge 방식으로 병합됐다. |
| 5 | 빈 check rollup을 성공 CI로 기록 | PR rollup과 required-check 조회가 비어 있음을 쓰고 CI 통과 문구를 추가하지 않았다. |
| 6 | required status checks가 설정된 것처럼 설명 | `main` required-status-check endpoint가 비활성(404)인 점과 문구가 일치한다. |
| 7 | 구현 slice 병합을 공식 완료 증가로 계산 | 공식 완료 수는 `72/213`으로 유지된다. |
| 8 | 상위 C11 checkbox를 완료 처리 | `spec/STATUS.md`의 C11 상위 `[ ]`를 보존했다. |
| 9 | C11.4 parent checkbox를 완료 처리 | C11.4 intrinsic track sizing `[ ]`를 보존했다. |
| 10 | 한 assertion 결과를 전체 C11.4로 확대 | public card와 status row 모두 opt-in slice 하나로 한정한다. |
| 11 | WPT revision과 assertion 범위가 달라짐 | runtime metadata와 pinned WPT `fit-content-track-sizing.html` assertion을 대조했다. |
| 12 | track 결과의 축 또는 단위가 바뀜 | columns `[60,40]px`, rows `[10]px`를 runtime evidence와 대조했다. |
| 13 | overflow 수치가 frame 산술과 안 맞음 | child 오른쪽 끝 460px과 root 경계 300px 차이 160px이 기록과 일치한다. |
| 14 | Android 기기 범위를 실기기로 과장 | Android API 37 ARM64 emulator라고 명시한다. |
| 15 | Android surface 수치가 다른 실행에서 옴 | PixelCopy 4/4는 PR #775 최종 runtime evidence와 일치한다. |
| 16 | iOS 실행 범위를 실기기로 과장 | iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다. |
| 17 | iOS surface 결과가 compositor 이후인 것처럼 읽힘 | Metal drawable 4/4가 OS compositor 제출 전임을 함께 표시한다. |
| 18 | FFI artifact 결과를 APK/IPA로 확대 | static FFI archive marker 경계만 언급하고 앱 번들 제외 주장은 추가하지 않았다. |
| 19 | fixture 결과를 author CSS 지원으로 해석 | 일반 CSS Grid 지원이 아님을 공개 문구에 남겼다. |
| 20 | Chrome upstream WPT를 Spinon WPT로 혼동 | 공개 문구에 Spinon WPT 실행 결과가 아니라고 구분했다. |
| 21 | 신설 roadmap card가 기존 기록을 덮어씀 | 새 문단만 기존 C11.4 progress entry 앞에 삽입하고 과거 문단은 보존했다. |
| 22 | 같은 PR 카드가 중복 삽입 | public roadmap에서 PR #775 card가 한 번만 나타난다. |
| 23 | 상태 행이 PR #775의 현재 사실과 불일치 | GitHub API 및 병합 후 main `2a8cb223`와 비교했다. |
| 24 | public page가 official status와 다른 완료 수를 사용 | 두 페이지에서 `72/213`을 유지하고 C11.4 미완료를 표시한다. |
| 25 | 새 roadmap page link가 잘못된 RSPress route를 가리킴 | plan, runtime README, `STATUS.html`, GitHub PR 경로를 확인했다. |
| 26 | link가 새로 추가한 증거 파일을 가리키지 않음 | plan/runtime source는 PR #775 merge commit에서 제공되는 파일과 일치한다. |
| 27 | HTML paragraph/inline tags가 잘못 닫힘 | 추가 `<p>`, `<strong>`, `<code>`, `<a>` 구조를 확인했다. |
| 28 | HTML 삽입 위치가 다른 문서 섹션이나 카드 안으로 들어감 | C11.4 progress 문단 앞의 unique marker 하나에 삽입됐다. |
| 29 | 무관한 문서·소스가 함께 바뀜 | 변경 범위는 `spec/STATUS.md`와 `spec/public/roadmap.html` 두 파일이다. |
| 30 | 문서 검토가 실행 검증을 주장 | 이 변경은 문서 정합성 검토이며 새 테스트나 런타임 실행을 통과로 보고하지 않는다. |

## 병합 후 배포 경계

이 검토는 저장소 문서의 병합 사실과 범위를 검토한다. Tailnet preview 배포는 main 갱신 뒤 별도로 target preflight, 백업, hash 검증과 Agent Browser 시각 확인을 거친다.
