# Tailnet 로드맵 동기화 pass02 · PR diff 실패 관점 30개

검토 대상은 최신 `489dabc158b1ed55c59d023027070e3662829a7f` 기반의 선택 사이트, Tailnet preview 복사/검증 기록, Agent Browser 화면과 재현 스크립트다. 공개 문서 자동 배포는 실행하지 않았다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 선택 source head가 최신 main인지 | selected-root와 build manifest가 `489dabc1`을 기록한다. |
| 2 | 실행 스크립트가 다른 branch에서 실행될 수 있는지 | branch assertion은 `docs/roadmap-tailnet-sync-2026-10-25`를 요구한다. |
| 3 | 최신 STATUS가 선택 root에 반영됐는지 | main 문서를 overlay하고 route SHA를 기록했다. |
| 4 | 최신 roadmap의 #615 merge SHA가 들어갔는지 | build marker와 Agent Browser가 `0ccec669...`를 확인했다. |
| 5 | 새 PR #617 review 문서가 build에 포함됐는지 | HTML route 및 raw evidence route를 선택했다. |
| 6 | 생성 페이지 수가 잘못 계산됐는지 | RSPress build 19 pages를 기록한다. |
| 7 | 선택 route 중복이 있는지 | stage script가 route map 중복을 거부하고 620개를 해시했다. |
| 8 | 선택 source에 symlink가 섞이는지 | tree walker가 evidence symlink와 non-file entry를 거부한다. |
| 9 | 출력 경로가 pass01을 덮는지 | pass02 전용 root, config, output, report를 사용한다. |
| 10 | 기존 pass01 backup을 덮는지 | copy target backup root는 별도 `pass02`다. |
| 11 | preview route를 삭제하는지 | preflight/copy/verify report 모두 `no_deletes: true`다. |
| 12 | pre-existing bytes 백업이 정확한지 | 교체 전 크기와 SHA를 저장하고 복사 후 27/27을 대조했다. |
| 13 | mismatched 기존 backup을 허용하는지 | copy script가 기존 backup의 크기와 SHA 불일치를 거부한다. |
| 14 | 교체 대상 파일을 프로세스가 열고 있는지 | 27개 모두 lsof 검사 결과 open handle이 없었다. |
| 15 | 교체 직전 대상이 달라지는지 | copy가 사전 존재 여부·크기·SHA를 다시 확인한다. |
| 16 | 소스가 preflight 후 바뀌는지 | copy가 각 source 크기와 SHA를 다시 확인한다. |
| 17 | 부분 파일이 노출되는지 | 목적지별 임시 파일 작성 후 rename하고 결과 SHA를 확인한다. |
| 18 | same route를 불필요하게 다시 쓰는지 | 590개 same 경로는 그대로 둔다. |
| 19 | 신규 경로 수가 누락되는지 | preflight에서 3개 create를 기록했다. |
| 20 | PR #393 evidence의 PNG import가 깨지는지 | 3개 sibling PNG를 source hash와 대조해 selected root에 연결했다. |
| 21 | RSPress 버전이 바뀌는지 | frozen lockfile install은 211/229 installs를 확인했고 lockfile 변경은 없었다. |
| 22 | 실패한 빌드를 성공처럼 보고하는지 | missing root bin 첫 시도와 package-local bin 성공 retry 로그를 분리해 보존한다. |
| 23 | 실패 시도가 제품 코드 변경을 유발했는지 | 문서 build와 의존성 링크 확인만 수행했다. |
| 24 | Tailnet 주소가 다른 서비스로 바뀌었는지 | `tailscale status`와 127.0.0.1:8768 listener를 확인했다. |
| 25 | remote가 HTTP 오류를 성공으로 처리하는지 | 620 route 각각 HTTP 200을 요구한다. |
| 26 | remote content가 다른 revision인지 | 620/620 remote body 길이와 SHA가 selected source와 일치한다. |
| 27 | Agent Browser가 다른 roadmap을 봤는지 | Tailnet roadmap title, URL, PR #615와 merge SHA를 확인했다. |
| 28 | 공식 상태를 잘못 렌더하는지 | Agent Browser에서 `72/213` 및 C11.4 미완료 표기를 확인했다. |
| 29 | 계획/evidence HTML이 깨졌는지 | 계획 페이지와 새 review 페이지를 열어 merge SHA와 30개 표 행을 확인했다. |
| 30 | 브라우저가 작업 뒤 남는지 | 세션을 닫고 `pgrep -l agent-browser`에 남은 프로세스가 없는지 확인했다. |

## 결과

차단 결함은 발견하지 않았다. 로컬 preview 620/620, 백업 27/27, Tailnet HTTP 200 및 SHA 620/620이 확인됐다. Agent Browser 시각 검증을 마쳤고 해당 세션을 종료했다. 제품 테스트는 실행하지 않았다.

## PR 본문 이미지 후속 확인

PR #618 본문의 첫 raw image URL은 커밋 SHA 오기로 404였다. Git의 실제 `HEAD`를 대조해 URL을 수정하고, 새 Agent Browser 세션에서 PR 본문 이미지를 다시 확인했다. 이미지 `complete`는 true이며 natural size는 1280×633이다. 최종 URL은 `d87b6eda7b00f8a6c771fa32b9686c7c1dbf4dfd`에 고정되어 있다. 수정 전후 화면 검증 세션을 모두 종료했다.
