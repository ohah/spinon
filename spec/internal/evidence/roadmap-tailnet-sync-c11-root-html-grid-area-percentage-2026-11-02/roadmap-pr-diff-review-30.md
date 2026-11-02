# C11.1 Tailnet 로드맵 동기화 PR diff 검토 30개

검토 대상은 이미 병합한 C11.1 percentage named-area 결과를 기존 Tailnet 미리보기에 선택 게시하고 그 route·hash·browser 결과를 추가하는 변경이다. 검토 근거는 selected root/publication manifest, preflight/copy/served-verification JSON, Rspress build log와 Agent Browser live capture다.

| # | 독립 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | selected root가 main merge commit과 다른 작업 트리에서 만들어지는가 | Manifest source HEAD `ac1437908f9b064a28670c4f6affe6650bc508e7`와 tree가 일치한다. |
| 2 | repository의 untracked 작업물을 게시 root에 섞는가 | Source inventory는 `git ls-files` 기반이고 지정한 plan·status·contract·evidence만 복사한다. |
| 3 | source file symlink가 selected root를 벗어나는가 | source 파일은 일반 파일로 검사하고 route에는 `.`/`..`, 절대 경로, 역슬래시를 허용하지 않는다. |
| 4 | 동일한 route가 서로 다른 source bytes로 덮이는가 | publication map은 중복 route hash가 같은 경우만 합치며 충돌은 assertion으로 막는다. |
| 5 | generated Rspress file이 symlink 또는 비정규 파일인가 | Site tree walk는 symlink와 지원하지 않는 항목을 거부한다. |
| 6 | 원본 public roadmap raw HTML이 Rspress 산출물 누락으로 사라지는가 | `roadmap.html` raw source를 별도 selected-root route로 보존한다. |
| 7 | 잘못된 구현 PR이나 merge SHA가 게시되는가 | Card marker #1035와 merge SHA `4b921f3e8ccd86e7e156562bb7e5b4a464f082f2`를 선택 publication 전에 assertion한다. |
| 8 | official aggregate가 `72/213` 외 값으로 보이는가 | Roadmap card와 STATUS route의 `72/213` marker를 검사한다. |
| 9 | 제한 slice가 C11 완료로 게시되는가 | selected manifest와 status assertion은 `c11Complete=false`다. |
| 10 | 지원되지 않는 일반 CSS Grid/WPT 범위가 카드에서 빠지는가 | 공개 card가 제한 map/tracks 및 미지원 범위를 직접 명시한다. |
| 11 | local card link가 누락된 route를 가리키는가 | 카드의 일곱 local route를 manifest에서 각각 찾는다. |
| 12 | Android/iOS PNG 중 하나가 publication에서 빠지는가 | 두 PNG route가 required-route 검사와 HTTPS hash 검사에 들어간다. |
| 13 | plan/runtime/status page route가 다른 경로에 게시되는가 | 선택 root aliases와 generated `.html` route를 함께 검증한다. |
| 14 | 사용하지 않는 C11 plan alias를 덮어쓰거나 삭제하는가 | `spec/plan/c11-css-grid.html`과 `.md`는 범위 밖이며 삭제 대상은 없다. |
| 15 | 전체 docs link validation을 통과한 것처럼 오인시키는가 | Manifest는 `fullDocsLinkValidation=false`로 기록한다. |
| 16 | Rspress 버전이 정책과 다르게 바뀌는가 | 저장소 config에서 고정된 `2.0.22`로 선택 문서 build가 성공했다. |
| 17 | 전체 root build 실패와 선택 build가 혼합되는가 | 이 sync는 selected root와 해당 성공 build log만 근거로 삼는다. |
| 18 | generated site 정리 중 기존 build 결과를 덮어쓰는가 | 고유 `pass02`가 비어 있는지 확인하고 staging/publication도 exclusive create로 만든다. |
| 19 | preview root가 symlink 또는 예상하지 않은 경로인가 | Sync preflight는 preview root의 일반 directory와 canonical path를 확인한다. |
| 20 | 다른 process가 8768 preview port를 소유하는가 | Preflight는 단일 listener·예상 server command·주소를 확인했고 PID `28040`을 기록했다. |
| 21 | server listener나 Tailscale backend가 오프라인인가 | Preflight에서 backend `Running`, local roadmap HTTP 200을 확인했다. |
| 22 | 교체 대상 파일이 열린 상태에서 교체되는가 | 교체 대상 open-handle count는 0이다. |
| 23 | overwrite 전에 기존 content를 보존하지 않는가 | 교체 21개는 고유 backup directory에 저장하고 hash를 다시 확인했다. |
| 24 | 부분 복사가 canonical file을 손상시키는가 | 각 대상은 same-directory temp 파일에서 hash 검증 후 atomic rename한다. |
| 25 | 복사 과정에서 사용하지 않는 route를 삭제하는가 | Copy와 verification report 모두 `noDeletes=true`다. |
| 26 | 배포 source가 preflight 이후 바뀌어 다른 bytes가 복사되는가 | Copy 직전 source·target existence와 hash를 preflight report와 비교한다. |
| 27 | local preview가 manifest bytes와 다른가 | 기존 local 302 route 전부 bytes/hash exact다. |
| 28 | HTTPS preview가 stale cache 또는 다른 bytes를 반환하는가 | HTTPS 302 route 전부 HTTP 200이며 manifest bytes/hash exact다. |
| 29 | 좁은 viewport에서 card가 잘리거나 가로 넘침이 생기는가 | Tailnet Agent Browser 1440px/390px에서 document width가 각각 viewport와 같다. |
| 30 | browser 확인 후 세션이나 server가 불필요하게 남거나 재시작되는가 | Agent Browser `No active sessions`, matching daemon 0, preview PID `28040` 유지가 기록됐다. |

**검토 결과:** 302개 선택 route 모두 live local/HTTPS hash가 일치했고 replacement backup 21개를 확인했다. Agent Browser는 PR card, merge SHA, 공식 `72/213`, simulator PNG route를 live Tailnet에서 읽었다. Live preview 작업은 C11 완료나 runtime pixel evidence를 추가하지 않는다.
