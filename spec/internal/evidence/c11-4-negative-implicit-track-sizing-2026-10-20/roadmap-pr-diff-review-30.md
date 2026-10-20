# C11.4 negative implicit track sizing 후속 roadmap 문서 diff 검토 30관점

범위는 PR #379 병합 뒤 `spec/STATUS.md`와 `spec/public/roadmap.html`에 반영한 상태·공개 roadmap 변경이다. 구현 동작은 재검증하지 않고 병합 식별자, 완료 상태, 결과 수치, 플랫폼 경계와 링크 표현을 각각 확인했다. PR #379 구현 변경의 별도 30관점 검토는 [구현 diff 검토](./pr-diff-review-30.md)에 있다.

| # | 실패 가능성 | 확인 결과 |
| ---: | --- | --- |
| 1 | feature commit을 merge SHA로 잘못 기록 | 두 문서는 PR #379의 merge SHA `99a2d408385a03591088a44005cb7b35aaf69cb3`를 사용한다. |
| 2 | PR이 아직 열렸는데 병합 완료라고 씀 | GitHub PR #379는 `MERGED`이며 merge commit이 `origin/main`에 있다. |
| 3 | 제한 slice 때문에 C11.4를 완료 처리 | `spec/STATUS.md`의 C11.4 checkbox는 `[ ]` 그대로다. |
| 4 | public roadmap의 공식 완료 수를 올림 | 추가 문단은 제한 범위와 기존 완료 수 유지만 기록한다. |
| 5 | 구현을 전체 negative implicit track 지원으로 과장 | numeric negative line과 leading track 등 구현된 slice만 열거하고 전체 Grid는 남겼다. |
| 6 | line grammar 전체가 지원된다고 읽힘 | 지원 밖 grammar가 남았다고 명시한다. |
| 7 | 역방향 `grid-auto-*` 순환이 빠짐 | `grid-auto-rows/columns` 목록의 reverse cycle을 직접 적었다. |
| 8 | sparse placement 결과를 dense로 오해 | signed sparse와 dense를 모두 명시해 구분을 보존한다. |
| 9 | auto-repeat과 negative line 조합이 검증됐다고 오해 | 두 문서 모두 결합 검증을 미완료로 남긴다. |
| 10 | Chrome case를 WPT 적합성으로 승격 | Chrome 155/W3C 기대값 33/33만 기록하고 WPT는 미완료로 둔다. |
| 11 | Android AVD를 실기기로 오해 | API 37 ARM64 AVD라고 장비 유형을 명시한다. |
| 12 | iOS Simulator를 실기기라고 오해 | iPhone 17 Pro/iOS 26.2 Simulator라고 한정한다. |
| 13 | draw/present를 OS 화면 표시 완료로 확대 | runtime 단계 draw/present만 기록하며 compositor 이후 검증은 미완료다. |
| 14 | PixelCopy/Metal 표본을 전체 화면 픽셀 검증으로 오해 | overflow 표본만 5/5 exact로 기록한다. |
| 15 | edge 색 표본을 CSS border stroke 결과로 오해 | root edge sample이 border stroke가 아님을 두 문서가 구분한다. |
| 16 | iOS Metal drawable 값을 compositor 이후 픽셀로 오해 | iOS 표면 readback이 compositor 전이라고 적었다. |
| 17 | Android software renderer를 hardware GPU 결과로 오해 | AVD 경계를 명시하며 hardware renderer를 주장하지 않는다. |
| 18 | 문서가 새 public API/CSS 지원 표를 발표 | `RuntimeBlockFlexGridTrackSizingV1` 내부 slice로 적고 public API 완료를 선언하지 않는다. |
| 19 | status 문구가 기존 C11.4 계획을 대체 | intrinsic contribution, cyclic percentage, repeat grammar와 WPT 등 미완료 항목이 계속 남아 있다. |
| 20 | 계획 route가 Rspress 형식과 다름 | roadmap의 계획 링크는 `/spinon/plan/c11-4-negative-implicit-track-sizing.html`이다. |
| 21 | 내부 계약 링크의 route가 빠짐 | roadmap은 `/spinon/internal/0059-c11-grid-layout.html` 경로를 쓴다. |
| 22 | contract fragment가 잘못됨 | roadmap의 `#c114-negative-implicit-track-sizing`은 0059 실제 heading id에 맞춘다. |
| 23 | evidence 링크가 폴더가 아닌 잘못된 파일을 가리킴 | roadmap은 실행 근거 `README.html`을, STATUS는 원본 `README.md`를 연결한다. |
| 24 | 실행 근거에서 결과 수치를 재확인할 수 없음 | STATUS와 roadmap 모두 PR #379 실행 evidence에 직접 연결한다. |
| 25 | 구현 review와 PR diff review를 같은 검토로 취급 | STATUS는 구현 검토 20개와 PR diff 검토 30개 링크를 분리한다. |
| 26 | 두 문서의 완료 경계가 서로 다름 | 둘 다 C11.4 미완료, WPT 미완료, auto-repeat 결합 미완료를 유지한다. |
| 27 | PR #375 기록과 새 PR #379 기록이 충돌 | PR #375는 positive implicit slice, PR #379는 negative signed-placement slice로 구분된다. |
| 28 | PR #379 merge 상태를 required CI 통과로 오해 | 문서 변경은 CI 녹색이나 필수 check 통과를 새로 주장하지 않는다. |
| 29 | source 문서 변경만으로 Tailnet 제공까지 됐다고 오해 | 이 PR은 source만 갱신하며 Tailnet serving과 browser 확인은 별도 후속 sync에서 기록한다. |
| 30 | 실제 미지원 범위를 가림 | 전체 CSS Grid, WPT, 실기기 및 compositor 이후 픽셀은 미완료로 남긴다. |

이 검토는 두 문서의 source diff에 한정한다. Tailnet 파일 백업·복사, served hash와 Agent Browser 확인은 별도 동기화 증거로 남긴다.
