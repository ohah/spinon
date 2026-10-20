# C11.4 positive implicit Tailnet sync 작업 실패 관점 검토 20개

검토 대상은 pass16의 fragment 오류 발견, pass17 수정 sync, pass18의 최종 review 문서 재sync다. 각 항목은 staging·복사·served hash·Agent Browser 관찰 결과를 따로 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 미병합 브랜치나 잘못된 main에서 선택 build | pass17 source는 문서 수정 commit `fefcc7a0cf850213b14974c37076dcd3a7628588`, pass18은 검토 갱신 commit `f6c36038c104760487e5c34fa9911a89b98efd80`이다. 양쪽 모두 PR #376 merge `98d4910539f0cc69ee8065f266038f1e996c03ce` 이후 브랜치다. |
| 2 | 원본 dirty checkout의 C11.3 작업을 변경 | 빌드·수정은 격리된 C11.4 docs worktree에서 진행했다. 원래 checkout에는 접근해 수정하지 않았다. |
| 3 | 임시 Rspress 버전 사용 | 선택 build는 이미 설치된 Rspress `2.0.22`를 실행하고 성공했다. |
| 4 | config가 workspace package를 evidence 경로에서 잘못 해석 | 첫 pass16 시도 오류를 보존했다. 설정에서 중복 `@rspress/core` import를 제거해 workspace config를 통해 로드하도록 수정했고 선택 build가 성공했다. |
| 5 | build가 전체 docs root를 의도치 않게 덮어씀 | Rspress root/outDir는 pass별 새 `build/spinon/c11-4-positive-implicit-tailnet-{root,site}-passN` 경로다. |
| 6 | stale seed나 일부 C11.4 route 누락 | pass18 seed는 최신 선택 파일 목록으로 준비했고 plan, public roadmap, STATUS, contract, precomparison 및 구현 근거 route가 생성됐다. |
| 7 | Markdown relative link가 Tailnet route 밖으로 나감 | 선택 파일끼리는 `/spinon/` 경로로 재작성하고 선택하지 않은 source는 GitHub main 링크로 바꾸는 기존 helper를 사용했다. |
| 8 | 구현 근거 evidence 이미지가 preview asset에서 빠짐 | 증거 폴더의 이미지가 static asset 경로에 포함됐으며 pass17 browser가 Android/iOS 원본 이미지를 모두 로드했다. |
| 9 | source hash manifest가 선택 파일을 빠뜨림 | pass18 manifest는 74개 고유 source의 1,437개 파일을 SHA-256으로 기록했다. |
| 10 | home/worktree 절대 경로가 static output에 남음 | selected-root redaction 347개와 generated asset redaction 1개 후 remaining host path는 0이다. |
| 11 | roadmap 경로가 선택 산출물과 다름 | `roadmap.html`과 plan·contract·evidence 산출물을 각각 존재 확인하고 Tailnet HTTP 200·hash 검증에 포함했다. |
| 12 | 오타난 contract fragment가 성공으로 오인 | pass16 Agent Browser가 anchor failure를 찾았다. DOM에서 실제 double-hyphen ID를 읽고 href를 수정해 pass17 `anchor_found=true`를 얻었다. |
| 13 | C11.4 제한 slice를 전체 지원 완료로 표시 | roadmap은 negative implicit, auto-repeat 조합, WPT와 전체 C11.4 미완료를 유지하고 checkbox를 끈 상태로 표시한다. |
| 14 | 표시 집계가 개인 상태 또는 CSS API 수와 혼동 | Tailnet은 `212개 표시 · 공식 완료 72/212`를 렌더링한다. STATUS의 C11.4 checkbox는 별도로 unchecked다. |
| 15 | 대상 preview root가 다른 앱 데이터와 겹침 | helper는 정해진 `/Users/yoonhb/Library/Application Support/SpinonPreview`와 pass별 백업 root만 허용한다. |
| 16 | preflight가 기존 파일을 삭제하거나 덮기 전 값을 잃음 | pass18은 2,815개 file을 create/replace/same으로 분류하고 deletes=false다. replacement 102개는 각각 고유 backup을 남겼다. |
| 17 | 교체 중 열린 static file을 발견하지 못함 | 복사 직전 `lsof`가 PID 1191의 기존 HTML 두 개를 포착했다. 두 이전 hash가 pass18 backup에서 검증됐다. |
| 18 | 원자 교체 후 bytes가 source와 다름 | pass18 copy 결과 2,815개 전부 verified이며 205개 변경 path에 temporary-file rename을 사용했다. |
| 19 | 로컬 복사만 성공하고 Tailnet에서 stale bytes가 제공됨 | 외부 HTTPS origin에서 2,815/2,815 HTTP 200·byte·SHA-256이 일치했다. |
| 20 | Agent Browser 확인 후 전용 프로세스가 남음 | pass16·17·18 각 namespace/session을 닫고 active=false·pid=null을 확인했다. 공유 PID 67317과 preview server PID 1191은 유지했다. |

이 검토는 선택 정적 preview와 위에 열거한 route·화면 범위에 한정된다. 전체 저장소 링크 검사, GitHub Pages, 앱 실행 및 물리 기기 검증을 주장하지 않는다.
