# C12.4 Chrome 사전 비교 PR 변경 검토

검토 대상은 C12.4 Chrome 기준 자료를 추가하는 PR의 변경 경계와 구현 상태 동기화다. 기능 구현이나 배포를 승인하는 문서가 아니다.

| # | 실패 관점 | 대조 결과와 처리 |
| ---: | --- | --- |
| 1 | PR에 C12.3 이전 구현 커밋이 다시 들어가는가 | branch base와 `origin/main` 비교 전 PR 생성 시 merge-base 및 변경 파일 목록을 확인한다. |
| 2 | 작업 범위가 다음 계획 항목 C12.4인지 | C12.3은 병합됐고 C12.4가 미완료 다음 단계임을 상태 대장과 상위 계획에서 대조했다. |
| 3 | Chrome 자료 수집만으로 C12.4 기능이 완료 체크되는가 | `spec/STATUS.md` 체크를 미완료로 두고 기능 미구현을 명시했다. |
| 4 | Android/iOS C12.4 runtime을 이미 시험했다고 오해하는가 | 이번 산출물이 Chrome 사전 비교임을 쓰고, 모바일 runtime은 남은 단계로 둔다. |
| 5 | 기존 C12.3 iOS 실행을 현재 C12.4 결과처럼 섞는가 | C12.3 플랫폼 비교 기록과 C12.4 Chrome reference를 서로 다른 상태·근거 파일에 연결했다. |
| 6 | C12.3 캡처의 작은 우측 돌출이 잘림 또는 무검토로 남는가 | 근거 README에 두 고정 box의 `335`·`340` CSS px 오른쪽 좌표와 5px 겹침, viewport 내 위치 및 pixel-diff 한계를 추가했다. |
| 7 | C12.3 프레임 비교를 전체 screenshot pixel 동등성으로 과장하는가 | 캡처는 fixture 표시 자료이며 pixel-diff가 아님을 같은 근거 파일에 명시했다. |
| 8 | C12.4 frame 관측을 paint 순서 증명으로 과장하는가 | paint oracle은 screenshot interior pixel이며 frame·hit-test 결과와 분리했다. |
| 9 | WPT source를 실행하지 않았는데 통과로 표시하는가 | source SHA와 `not-run` 상태를 inventory, reference, evidence에 맞춰 기록했다. |
| 10 | custom fixture의 26 scene을 26 WPT case pass로 보이는가 | 문서에서 custom Chrome case 수와 WPT 실행 상태를 별도로 적었다. |
| 11 | Chrome empirical 값을 표준·다른 엔진의 보편 결과로 소개하는가 | Chrome 버전·revision을 연결하고 Flex 전체 규칙으로 일반화하지 않는다고 제한했다. |
| 12 | 캡처 PNG가 변경되었는데 JSON hash·회귀가 낡는가 | reference에 PNG hash/크기/표본을 저장하고 테스트가 파일 원문을 비교한다. |
| 13 | 새 명령이 package script에서 누락되는가 | `css:reference:c12-4-stacking-context`를 `package.json`에 추가했다. |
| 14 | 새 evidence가 내부 인덱스와 상태 대장에서 고립되는가 | `spec/internal/README.md`, `spec/STATUS.md`, C12 상위/세부 계획에 링크와 상태를 반영했다. |
| 15 | 계약 문서가 Chrome 측정으로 public API로 승격되는가 | 0058은 제안 내부 계약이며 공개 CSS/API가 아니고 숫자 버전 `0.1.0`을 유지한다. |
| 16 | 기존 계획 리뷰를 구현 리뷰처럼 재사용하는가 | C12.4 tooling 검토와 PR 통합 검토를 별도 파일로 두고 이전 계획 실패 관점 문서를 재사용하지 않는다. |
| 17 | 사용자 소유의 unrelated spike가 PR에 포함되는가 | `spikes/blitz-stylo-layout/`, `spikes/stylo-style/`는 이미 존재하는 untracked 자료로 확인했고 stage 대상에서 제외한다. |
| 18 | 정식 RSPress 문서까지 선행 업데이트하거나 게시하는가 | 내부 구현 전 사전 비교라 사용자 지시에 맞춰 RSPress와 공개 Pages 배포를 변경하지 않는다. |
| 19 | 임의 CI·버전 bump가 부수적으로 끼는가 | CI 파일·crate/package release version은 추가하지 않고 기존 node test 명령만 기록한다. |
| 20 | PR의 파일·요약·증거·라벨이 실제 변경과 어긋나는가 | PR 작성 전에 staged diff, 테스트 출력, screenshot 경로 및 기존 라벨/양식을 재대조하고 PR 본문은 한국어로 작성한다. |

이 문서는 PR을 만들기 전 검토 기록이다. GitHub PR 생성·업로드·병합 상태는 실제 결과가 난 뒤 별도로 기록한다.

## 미리보기 동기화 결과

- 첫 전체 RSPress candidate build는 기존 preview source의 `rust,ignore` fence 언어명과 `./` 없는 상대 이미지 참조 때문에 실패했다. 임시 `/tmp` preview source에만 fence label 3곳과 이미지 참조 48곳을 정규화한 뒤 RSPress `2.0.22` 전체 미리보기 빌드가 성공했다. 저장소 Markdown, RSPress 설정·버전은 이 문제 때문에 변경하지 않았다.
- 새 빌드를 기존 출력에 덮기 전에 후보 HTML route 집합을 비교했다. 기존 HTML route는 모두 후보에 남아 있었다. 이전 출력은 별도 `build/` 백업 디렉터리에 보존했다.
- Tailnet에서 로드맵, C12.4 계획·계약·상태·사전 비교 페이지와 DPR 1·2 PNG, reference JSON이 모두 HTTP 200으로 열렸다. `agent-browser` 화면에서 새 C12.4 진행 문구와 사전 비교 링크를 확인했다. GitHub Pages는 배포하지 않았다.
- C12.3 Android 실기기와 iOS Simulator comparator를 다시 실행해 각각 13 상태·364 frame 일치(최대 오차 `0.009375 CSS px`)를 확인했고 회귀 3개가 통과했다. 이는 이전 C12.3 근거 검토이며 C12.4 모바일 실행이 아니다.

PR 생성 뒤에는 GitHub 번호, 실제 첨부 자산, mergeability와 check 상태를 이 기록에 추가한다.
