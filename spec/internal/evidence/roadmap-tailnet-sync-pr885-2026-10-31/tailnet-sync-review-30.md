# Tailnet PR #885 roadmap sync diff 실패 관점 30개

문서 변경 PR 뒤 Tailnet preview build·copy·검증 근거의 저장 범위를 점검했다. 검토 subject patch SHA-256은 base `01dc8487cf56652c1bf733fb393ce35d153005a1` 대비 `37521dbd79f27e88de4d7fd0299b9a82f12d67d1e794a5318c74024526225023`이고 이 보고서 파일 자체는 제외한다.

| # | 공격 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | publication source가 PR #886 병합 전 tree를 쓰는가 | **PASS** — source HEAD는 merge 이후 `01dc8487cf56652c1bf733fb393ce35d153005a1`이다. |
| 2 | source tree hash가 다른 revision을 가리키는가 | **PASS** — root manifest의 tree hash는 `42a3fcfbdd610495156cdcaf0789b3e3959ec2eb`이다. |
| 3 | untracked WIP가 publication에 섞이는가 | **PASS** — root는 해당 commit의 Git tracked blob만 `git show`로 복사한다. |
| 4 | 작업 당시 dirty tracked file을 main 결과로 오인하는가 | **PASS** — 준비 과정이 HEAD와 main tree 및 tracked diff를 확인한다. |
| 5 | 선택 route에 `..`, absolute path 또는 symlink가 들어가는가 | **PASS** — route와 source tree를 각각 검증한다. |
| 6 | 전체 docs tree를 무심코 게시하는가 | **PASS** — 734개 파일의 selected root만 사용한다. |
| 7 | raw public roadmap가 정확한 main source인가 | **PASS** — publication staging은 `roadmap.html`과 source mapping byte equality를 검사한다. |
| 8 | site icon·logo·roadmap asset이 빠지는가 | **PASS** — 세 asset을 raw route와 docs route에 선택했다. |
| 9 | status page canonical route가 빠지는가 | **PASS** — `STATUS.md`, `spec/STATUS.md`와 generated `.html` routes를 보존했다. |
| 10 | C11 plan·0059/0070 contract와 referenced evidence가 빠지는가 | **PASS** — root manifest 및 required route 목록에 연결 경로가 있다. |
| 11 | 이번 tab implementation screenshots/evidence가 누락되는가 | **PASS** — 결과 README, review page, Android/iOS 화면 route를 요구한다. |
| 12 | 이번 Tailnet sync evidence 자체를 같은 publication에 복사해 순환하는가 | **PASS** — selected root prefix에 sync evidence directory가 없다. |
| 13 | 문서 renderer version이 바뀌는가 | **PASS** — pinned RSPress `2.0.22`와 저장소 lockfile을 사용했다. |
| 14 | build output이 다른 run을 덮어쓰는가 | **PASS** — 전용 `pass01` output이 존재하지 않는지 검사한다. |
| 15 | build가 실패했는데 성공으로 표시되는가 | **PASS** — 첫 CLI usage 실패와 수정 뒤 성공을 별도 gzip log에 보존했다. |
| 16 | full-doc link validation을 수행했다고 과장하는가 | **PASS** — root/build metadata는 전체 링크 검사를 하지 않았다고 표시한다. |
| 17 | duplicate publication route에서 서로 다른 bytes가 덮어쓰이는가 | **PASS** — duplicate route는 SHA-256 동일성이 필요하다. |
| 18 | build가 실제 PR #885 merge SHA·공식 완료 수를 빼먹는가 | **PASS** — PR #885, merge SHA, `72/213`, 미완료 marker를 요구한다. |
| 19 | STATUS HTML이 merge 사실이나 C11.4 상태를 잃는가 | **PASS** — 상태 페이지 marker와 C11 evidence route를 확인한다. |
| 20 | 새 plan에 기대 범위와 official count가 없는가 | **PASS** — generated plan에서 `tab-size:8`, `40/40`, `72/213`, PR marker를 확인한다. |
| 21 | stale target과 비교 없이 변경을 복사하는가 | **PASS** — preflight가 source·target bytes/hash와 file set을 고정한다. |
| 22 | existing target bytes를 백업 없이 교체하는가 | **PASS** — 교체 전 bytes와 SHA를 전용 backup root에 보존한다. |
| 23 | 불필요한 파일 삭제가 포함되는가 | **PASS** — preflight와 copy 결과 모두 `no_deletes: true`다. |
| 24 | 잘못된 preview server나 공개 listener에 쓰는가 | **PASS** — PID command와 단일 `127.0.0.1:8768` listener를 검사한다. |
| 25 | preflight 이후 source 또는 target이 바뀌는데 계속 복사하는가 | **PASS** — copy 직전 expected byte count와 SHA를 다시 대조한다. |
| 26 | 다른 process가 열어 둔 target 파일을 교체하는가 | **PASS** — 기존 경로 open handle 소유자가 preview server 외이면 copy를 중지한다. |
| 27 | 중간 write가 target file로 보이는가 | **PASS** — 임시 경로에 fsync한 뒤 atomic rename한다. |
| 28 | target 자체 검사에서 누락·변형 파일을 숨기는가 | **PASS** — 861/861 target 및 53/53 backup SHA가 일치한다. |
| 29 | HTTPS가 stale/다른 bytes를 제공하는데 성공으로 세는가 | **PASS** — 861/861 HTTP 200, 861/861 SHA match, 실패 0이다. |
| 30 | browser 확인을 다른 browser 종류나 남은 browser process로 대체하는가 | **PASS** — Agent Browser live page에서 PR/SHA/count/incomplete marker를 확인하고 screenshot을 남긴 뒤 session과 executable process를 닫았다. Terminal Browser는 사용하지 않았다. |

이 검토는 Tailnet selected-publication sync의 diff boundary와 저장 근거만 다룬다. 전체 site link validation, GitHub Pages, 일반 CSS 적합성 또는 C11 완료를 뜻하지 않는다.
