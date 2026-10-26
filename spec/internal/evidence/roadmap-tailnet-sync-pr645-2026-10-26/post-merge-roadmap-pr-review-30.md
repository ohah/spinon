# PR #645 병합 후 문서·로드맵 동기화 검토 · 30개 관점



검토일: 2026-10-26

대상 worktree: `/private/tmp/spinon-c11-4-pr645-postmerge-20261026`

기준 HEAD 및 `origin/main`: `28696fbfef552892f851113ed8e0d84c638edad5`

범위: 작업 트리의 `spec/STATUS.md`, generic direct-text 계획, Grid track-sizing 계획, runtime evidence README, `spec/public/roadmap.html` 변경. GitHub PR #645 상태 및 연결된 source/evidence도 읽기 전용으로 대조했다. 빌드·테스트·브라우저 실행은 하지 않았고, 코드나 기존 문서는 수정하지 않았다.



## 판정



**PASS 29 / PARTIAL 1 / FAIL 0. 병합 상태나 구현 상태를 잘못 나타내는 blocker는 없다.** PR은 실제 `Rebase and merge`로 병합되었고 merge SHA 및 시각이 다섯 문서에 기록한 값과 일치한다. C11.4 체크는 미완료이며 공식 집계 `72/213`도 그대로다.



유일한 PARTIAL은 artifact 재검산 범위다. pass03 실행 근거에는 screenshot과 hash가 있지만 APK/iOS executable 원본이 없어 이 worktree에서 두 binary hash를 재계산할 수 없다. roadmap 날짜 순서는 이번 focused recheck에서 해소됐다.



## 적대적 검토 30개



| # | 관점 | 결과와 근거 |

|---:|---|---|

| 1 | 검토 대상 source diff가 요청한 다섯 문서로 제한되는가 | **PASS** — 검토 대상 diff는 `STATUS.md`, 두 계획, runtime README, `roadmap.html`에 한정되고, 이 보고서는 별도 review artifact로 추가한다. |

| 2 | 검토 branch의 HEAD가 PR 병합 뒤 main 기준과 맞는가 | **PASS** — HEAD와 `origin/main` 모두 `28696fbfef552892f851113ed8e0d84c638edad5`다. |

| 3 | GitHub에서 PR #645가 실제 병합 상태인가 | **PASS** — `gh pr view`가 `state=MERGED`, base `main`을 반환한다. |

| 4 | 문서의 병합 시각이 GitHub 기록과 같은가 | **PASS** — `mergedAt=2026-10-26T03:57:10Z`; STATUS, 두 계획, runtime README, roadmap 표기와 일치한다. |

| 5 | 문서의 merge SHA가 GitHub API와 main tip에 맞는가 | **PASS** — PR `mergeCommit.oid`, main tip과 문서의 `28696fbfef552892f851113ed8e0d84c638edad5`가 일치한다. |

| 6 | PR 번호·제목·기준 branch가 올바른가 | **PASS** — API는 #645, 제목 `feat(c11.4): 일반 direct text intrinsic 입력 지원`, base `main`을 반환한다. |

| 7 | 문서가 source fix `f45596b8`를 PR에 없는 commit으로 잘못 지칭하는가 | **PASS** — GitHub PR commit 목록에 원본 fix commit `f45596b8b0b8a4d35807028bac30314a5dceaaab`가 포함된다. |

| 8 | Rebase and merge 뒤 실제 main source에 이미지 guard가 보존됐는가 | **PASS** — main의 재작성 source commit은 `acff4521a13ffdb13bed8fbda43e949272edf418`이며, f455와 이 commit 사이의 collector 및 negative-test 파일 diff가 없다. 현재 main source에도 `first_html_image` guard가 있다. 문서는 f455를 PR 원본/run source, 28696을 최종 merge SHA로 구별한다. |

| 9 | main 이력이 merge commit을 잘못 암시하는가 | **PASS** — PR 변경은 main에 선형 commit 5개로 다시 적용됐고, 최종 merge SHA는 단일 parent `8ce75f3d...`를 갖는다. 기록된 Rebase and merge 방식과 맞는다. |

| 10 | GitHub PR check rollup이 비어 있다는 문구가 맞는가 | **PASS** — `statusCheckRollup=[]`; 문서에 “check rollup은 비었다”고 정확히 적었다. |

| 11 | 병합 시점/commit에 check 결과가 없는 상태를 오독하는가 | **PASS** — PR head와 merge commit 각각 check-run 수 0이다. PR head combined status에는 status context가 0개다. |

| 12 | main required status check 설정 주장이 과장됐는가 | **PASS** — `main` branch protection 응답에는 `required_status_checks`가 없고 repo ruleset 목록도 비었다. “required status checks는 설정되지 않았다”는 문서와 일치한다. |

| 13 | 공식 C11.4 체크가 실수로 완료로 바뀌었는가 | **PASS** — `spec/STATUS.md:273`의 C11.4 항목은 계속 `- [ ]`다. 변경 diff도 체크 표시를 바꾸지 않는다. |

| 14 | 공식 `72/213` 완료 수가 새 PR로 증가했는가 | **PASS** — STATUS와 다섯 문서의 변경은 서술 기록만 수정하고, 새 완료 checkbox를 추가하지 않는다. 현재 C11.4 항목 및 roadmap entry 모두 `72/213`을 명시한다. |

| 15 | 구현 slice를 전체 C11.4 또는 제품 완료로 부풀리는가 | **PASS** — STATUS, 계획, README, roadmap 모두 전체 C11.4 미완료를 유지하며 일반 CSS admission·legacy fallback 전수·Release artifact exclusion 등 제외 범위를 남긴다. |

| 16 | generic plan의 image-mix 설명이 현재 계약과 맞는가 | **PASS** — 두 직접 text 후보를 모은 뒤 root subtree의 HTML `<img>`를 `unsupported_c114_text_source`로 닫는다고 기록한다. main source `generic_direct_text.rs`의 guard와 맞는다. |

| 17 | 빈 block sibling 허용 문구가 코드/test와 일치하는가 | **PASS** — plan·STATUS는 빈 HTML block sibling 허용을 유지한다고 적는다. `negative.rs`의 `two_candidates_allow_a_non_text_block_sibling_without_adding_text_input` test가 존재하고 pass03 Rust 로그도 generic suite 9/9를 기록한다. |

| 18 | 원래 PR review 결과와 수정 재검토 결과를 혼동하는가 | **PASS** — 원본 report는 `PASS 21 / PARTIAL 8 / FAIL 1`, fix recheck는 `PASS 4 / PARTIAL 3 / FAIL 0`으로 분리돼 있고 각 문서가 역사적 원본 및 후속 재확인을 구별한다. |

| 19 | generic/C11.4/runtime/FFI test 수가 evidence 로그와 맞는가 | **PASS** — 저장된 pass03 로그는 각각 9/9, 34/34, 19/19, 1/1을 기록하고 STATUS·계획·README가 같은 수를 인용한다. 리뷰어는 재실행하지 않았다. |

| 20 | fmt/targeted Clippy를 실행한 것으로 과장하는가 | **PASS** — 기존 pass03 artifact에 fmt 및 targeted Clippy 로그가 있고, 문서는 이 저장된 실행 결과를 요약한다. 이 리뷰 자체는 실행하지 않았다. |

| 21 | runtime README가 병합 정보를 simulator 재검증 증거로 쓰는가 | **PASS** — README는 병합 정보가 아래 simulator 실행에 추가 검증을 보태지 않는다고 분리해 적는다. |

| 22 | run source-head가 runtime README의 build source 설명과 맞는가 | **PASS** — pass03 `source-head.txt`는 원 PR commit `f45596b8...`를 기록하고, GitHub PR commit 목록에 같은 SHA가 있다. 재작성된 main source도 같은 파일 내용을 보존한다. |

| 23 | simulator 결과가 두 플랫폼과 같은 범위로 기록됐는가 | **PASS** — Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 양쪽 log에 `status=0`, `layout=ready`, `boxes=3`, frame 및 draw/present가 있다. 이 run은 canonical positive scene 한 개다. |

| 24 | screenshot을 글리프 페인팅 증거로 과장하는가 | **PASS** — 양쪽 draw log의 `runtime_text_count=0`, `glyph_batch_draw_count=0`을 README와 roadmap이 명시하고 glyph paint/pixel equality를 주장하지 않는다. |

| 25 | Android/iOS screenshot hash 및 dimensions가 기록 파일과 맞는가 | **PASS** — 저장 PNG를 다시 hash했을 때 Android `9b74719a…42e5a73`, iOS `a7b38f7a…4ea721`로 README와 일치한다. 이미지 크기도 각각 1080×2400, 1206×2622다. |

| 26 | README가 APK hash를 실제 첨부 binary와 혼동하게 만드는가 | **PASS** — README는 Debug APK SHA-256을 실행 근거의 hash로 기록하고 APK 첨부나 본 검토의 재계산을 주장하지 않는다. |

| 27 | iOS executable과 APK hash의 독립 재계산 가능 범위가 분명한가 | **PARTIAL, P3** — 두 binary hash는 README에 기록돼 있지만 pass03 run과 현재 worktree에는 APK 또는 `.app/SpinonBootstrap` 원본이 없다. 이 검토는 저장된 두 hash를 재계산하지 않았다. 이는 증거 재검산 한계이며 문서의 run/build 통과 주장과 충돌하지 않는다. |

| 28 | 새 roadmap 문단의 external PR과 internal plan/evidence/report 링크가 유효한가 | **PASS** — PR #645 URL은 GitHub API 대상과 일치한다. 계획·내부 계약·README·fix-recheck target의 `.md` source 및 새 `/spinon/internal/evidence/roadmap-tailnet-sync-pr645-2026-10-26/post-merge-roadmap-pr-review-30.html` report source가 모두 존재한다. |

| 29 | Android/iOS screenshot과 STATUS anchor 링크가 유효한가 | **PASS** — 두 pass03 PNG 파일이 존재하고, `/spinon/STATUS.html#css-구현-체크리스트` 대상 heading은 `spec/STATUS.md:177`에 있다. |

| 30 | 분리된 roadmap paragraph의 markup과 날짜 순서가 올바른가 | **PASS** — orthogonal candidate(10/25)와 #632(10/25)가 #635(10/26)보다 앞에 있고, 새 #645(10/26) 항목은 별도 `<p>`로 그 뒤에 놓였다. 문단·링크 태그가 닫히며 C11.4 완료 집계는 바뀌지 않았다. 새 report route는 #28에서 source target까지 확인했다. |



## 결과와 한계



- 병합 상태, 시각, merge SHA, check rollup 및 main required-check 상태를 GitHub API/CLI의 읽기 전용 조회로 확인했다. `origin/main`은 merge SHA와 같았다.

- source fix의 원 PR commit `f45596b8`은 Rebase and merge 후 main의 source commit `acff4521`로 재작성됐다. 두 source/test 파일은 byte-for-byte 내용이 같고 최종 merge SHA는 `28696fbf`다. 이번 docs 변경은 원 PR SHA, pass03 source-head 및 최종 merge SHA를 각각 기록한다.

- screenshots와 그 hash는 현재 저장소 파일에서 확인했다. APK/iOS executable의 원본 바이너리가 없어 해당 두 binary hash는 재계산하지 않았다.

- Tailnet 배포나 live webpage 전달은 이 문서만으로 주장하지 않는다. 이 리뷰는 로컬 Markdown/HTML, 저장 evidence, GitHub merge metadata를 대조했다.

- blocker는 없으며 C11.4 공식 체크 `72/213`은 미완료로 유지된다. 본 보고서는 PR 코멘트가 아니다.
