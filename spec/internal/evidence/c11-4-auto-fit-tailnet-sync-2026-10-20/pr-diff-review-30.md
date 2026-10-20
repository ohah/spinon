# C11.4 auto-fit Tailnet 갱신 PR diff 적대 검토 30개

**대상:** Tailnet 선택 빌드 범위, 동기화 helper의 pass14 확장, preflight/backup/served evidence, Agent Browser 기록과 README. 별도의 20개 sync review와 구분한다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | `pass14`가 선택 root helper에서 유효한 run으로 등록되지 않는가? | **통과.** `prepare-selected-root.mjs`가 pass14를 허용하고 전용 output root를 만든다. |
| 2 | 기존 pass6–pass13 동작을 pass14 추가 때문에 삭제하는가? | **통과.** 기존 run 목록과 분기는 유지하고 pass14만 새 조건으로 추가했다. |
| 3 | pass14가 PR #370 plan 문서를 출력물에서 빼는가? | **통과.** helper mapping에 plan source와 `plan/` destination이 있고 Tailnet route가 열렸다. |
| 4 | fixture 및 Chrome evidence가 public output에 복사되지 않는가? | **통과.** fixture는 `public/tests/fixtures/...`로, evidence asset은 `public/` 자산 route로 배치했다. |
| 5 | evidence README가 두 기존 route namespace 중 하나에서 누락되는가? | **통과.** `internal/evidence/`와 `spec/internal/evidence/` destinations를 모두 생성한다. |
| 6 | Rspress custom config가 실행 환경에서 불필요한 `@rspress/core` 재해석을 하는가? | **검토에서 발견·수정.** `defineConfig`를 직접 재import하지 않고 pinned package의 base config를 spread하며 빌드 성공을 확인했다. |
| 7 | custom config가 기본 docs config의 theme/navigation을 잃는가? | **통과.** `...baseConfig`를 유지하고 root/outDir와 dead-link 옵션만 Tailnet 선택 build에 맞게 덮어쓴다. |
| 8 | output root 변경으로 저장소 문서의 기본 build 설정을 바꾸는가? | **통과.** Tailnet 전용 config만 수정하고 `packages/docs/rspress.config.ts`는 수정하지 않았다. |
| 9 | build command가 Bunx의 최신 Rspress로 drift하는가? | **통과.** 최종 명령은 workspace의 Rspress 2.0.22 CLI를 사용했다. |
| 10 | staging redaction 이후 build output에 절대 host path가 재유입되는가? | **통과.** sanitizer가 1개 output asset을 정리했고 남은 경로 0개를 기록했다. |
| 11 | sanitizer가 CSS/HTML/JS/JSON map 내 경로까지 검사하지 않는가? | **통과.** pinned script의 text-extension set을 그대로 사용하고 `remaining_host_path_files=[]`를 확인했다. |
| 12 | sync pass가 다른 pass의 backup path를 재사용하는가? | **통과.** `pass14`는 `c11-4-pr370-tailnet-roadmap-sync-2026-10-20/pass14` 전용 backup namespace를 사용한다. |
| 13 | target root를 임의 입력으로 지정해 preview 바깥을 쓰는가? | **통과.** helper는 실제 `SpinonPreview`와 `SpinonPreviewBackups` 경로 외 입력을 거부한다. |
| 14 | backup directory symlink를 통해 다른 폴더를 덮는가? | **통과.** helper가 target/source/backup path 구성 요소의 symlink 및 non-directory를 거부한다. |
| 15 | source build tree의 symlink나 special file을 복사하는가? | **통과.** `listFiles`가 symlink와 일반 file/directory 외 항목을 거부한다. |
| 16 | copy가 preflight 뒤 수정된 source file set을 받아들이는가? | **통과.** copy 단계에서 정확한 경로 집합과 각 byte/hash를 preflight와 대조한다. |
| 17 | target 바이트가 preflight 후 바뀌었는데 오래된 backup으로 덮어쓰는가? | **통과.** 이전 target 존재 여부·byte·SHA를 재검사하고 다르면 abort한다. |
| 18 | 기존 backup이 다른 snapshot인데 그대로 재사용되는가? | **통과.** 존재하는 backup도 기대 SHA-256과 대조하며 최종 backup 71/71 검증됐다. |
| 19 | temporary file을 다른 파일시스템에 써서 rename atomicity를 깨는가? | **통과.** temp file은 target과 같은 directory에 기록한 뒤 rename한다. |
| 20 | 열린 target 파일을 직접 truncate하는가? | **통과.** replacement는 same-directory atomic rename이며 열린 descriptor 3개를 별도 보존 기록했다. |
| 21 | sync copy가 미리보기의 나머지 파일을 정리한다는 위험이 있는가? | **통과.** copy helper는 선택 파일만 생성·교체하고 삭제 경로가 없다. |
| 22 | backup 결과의 모양을 검증하지 않고 count를 오기하는가? | **통과.** nested `copy_verification`와 `backup_verification`에서 1,543/1,543 copy와 71/71 backup을 확인했다. |
| 23 | HTTP status 200만 확인하고 content freshness를 놓치는가? | **통과.** served check는 status, byte 수, SHA-256을 local target과 모두 비교했다. |
| 24 | origin/path injection이 served verifier에서 path traversal로 이어지는가? | **통과.** HTTPS origin만 허용하고 각 relative path의 절대·`.`·`..` 구성요소를 거부한다. |
| 25 | 1,543개 전부가 같은 선택 결과인지 검증하지 않는가? | **통과.** preflight file list 1,543개가 served result와 1,543/1,543 정확히 대응한다. |
| 26 | public roadmap link가 계획 없는 route나 repo path 외부를 가리키는가? | **통과.** Tailnet roadmap의 plan/evidence anchors가 실제 열린 `/spinon/` routes로 향한다. |
| 27 | STATUS 링크 재작성 뒤 C11.4 상태가 체크된 것으로 보이는가? | **통과.** Browser에서 C11.4 checkbox `false`와 새 plan 링크를 확인했다. |
| 28 | README Browser screenshot을 앱의 Android/iOS 캡처라고 말하는가? | **통과.** README와 PR 본문은 Tailnet 문서 화면이라고 구분한다. |
| 29 | Agent Browser 검사에서 보지 않은 전체 내부 route·link audit을 통과로 주장하는가? | **통과.** 같은-origin base 검사는 네 개 실제 확인 페이지로 한정했고 full-link audit이라고 쓰지 않았다. |
| 30 | 계획 동기화를 CSS 구현·WPT·simulator 완료로 과장하는가? | **통과.** C11.4 미완료를 유지하고 앱 runtime, simulator, physical device, WPT 또는 GPU 검증을 주장하지 않는다. |

새 문서 변경은 선택 build와 실제 Tailnet 제공 route로 검토했다. repository 전체 `docs:build`의 dead-link 오류를 이 선택 build의 성공으로 바꿔 말하지 않는다.
