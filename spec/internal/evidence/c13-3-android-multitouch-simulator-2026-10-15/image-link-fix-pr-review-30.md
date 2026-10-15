# C13.3 Android AVD evidence 이미지 링크 수정 PR 적대 검토 30개

검토 범위는 Android multi-touch evidence README의 두 Markdown image 문법과 이 수정 PR의 증거 연결이다. 제품 runtime·시뮬레이터 실행 결과는 변경하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | cancel screenshot이 이미지 문법이 아닌 텍스트로 남는가 | `![alt](./android-after-second-pointer-cancel.png)` 문법을 확인했다. |
| 2 | recovery screenshot이 이미지 문법이 아닌 텍스트로 남는가 | `![alt](./android-after-recovery.png)` 문법을 확인했다. |
| 3 | alt-text와 URL 사이의 `](` 구분자가 빠지는가 | 두 행을 파서가 `image` token으로 읽고 literal `![` 문자열로 렌더하지 않음을 확인한다. |
| 4 | closing `)` 누락으로 이후 문서가 이미지 URL에 합쳐지는가 | 각 Markdown target이 단일 괄호 안에서 끝난다. |
| 5 | Rspress가 bare filename을 package module로 해석하는가 | 두 상대 URL 앞에 `./`를 명시했다. |
| 6 | directory 대소문자 또는 파일명이 실제 evidence와 다른가 | 두 target을 동일 evidence directory의 실제 파일과 대조한다. |
| 7 | 이미지 파일이 비거나 손상됐는가 | 두 PNG를 1080×2400 RGBA로 식별하고 SHA-256을 기록했다. |
| 8 | 캡션과 cancel 화면이 서로 다른 scenario를 가리키는가 | 캡션은 second-pointer cancel 뒤 보존된 위치 파일에 연결된다. |
| 9 | recovery 캡션이 cancel 화면을 중복 가리키는가 | recovery 캡션은 all-up 뒤 fresh pan PNG에 연결된다. |
| 10 | cancel과 recovery 이미지가 우연히 같은 파일 hash인가 | 두 캡처의 SHA-256이 서로 다르다. |
| 11 | README에 표시할 원본 화면을 변형하거나 재인코딩하는가 | 이 수정은 Markdown 경로만 바꾸며 PNG byte는 수정하지 않는다. |
| 12 | Rspress build가 통과하지만 이미지 import는 빠지는가 | 생성된 README HTML의 image URL 두 개와 각각의 hashed static image 파일(84,448/84,445 bytes)을 대조했다. |
| 13 | logo를 evidence screenshot으로 오인하는가 | Agent Browser image 검사에서 두 evidence alt-text와 1080×2400 natural size를 구분한다. |
| 14 | browser가 요청을 완료하기 전에 `naturalWidth=0`을 성공으로 세는가 | Agent Browser에서 두 image 모두 `complete=true`, `naturalWidth=1080`, `naturalHeight=2400`을 확인했다. |
| 15 | PR body의 직접 raw screenshot 링크가 영향을 받는가 | PR body raw URL은 README 상대 link와 별개이며 main 경로를 확인한다. |
| 16 | PR body 이미지가 다른 simulator run에서 오는가 | PR body URL은 같은 `c13-3-android-multitouch-simulator-2026-10-15` directory다. |
| 17 | 새 evidence page route가 public roadmap anchor와 다른가 | public roadmap은 evidence `README.html` route를 유지한다. |
| 18 | README 링크 수정을 AVD runtime 코드 변경으로 오해하는가 | diff에는 Markdown README와 image-link review만 포함된다. |
| 19 | `spec/STATUS.md`가 새 Android 동작을 다시 주장하는가 | 수정은 기존 AVD 결과를 변경하거나 확대하지 않는다. |
| 20 | plan이 상위 C13.3 완료 상태를 암시하는가 | 계획의 남은 public events·wheel·S04.10 및 전체 완료 gate는 그대로다. |
| 21 | ADB root 진단이 다른 device scope로 바뀌는가 | emulator-only `unroot` 안내는 그대로 유지한다. |
| 22 | gzip transcript/checksum이 link 수정 중 손상되는가 | 이미지 경로 수정 diff와 압축 transcript/checksum 파일을 분리해 확인한다. |
| 23 | screenshot 링크가 external network path에 의존하는가 | evidence README는 repository-relative image URL을 사용한다. |
| 24 | base path가 달라질 때 page source URL이 깨지는가 | Rspress가 relative image를 `/spinon/static/image/...`로 emit하는지 확인한다. |
| 25 | repository docs config가 임시로 변경되는가 | scoped Rspress config는 `build/` scratch에만 있고 tracked config diff가 없다. |
| 26 | scoped build를 full-site validation으로 말하는가 | omitted-page dead-link/image check를 끈 scoped result만 보고한다. |
| 27 | Rspress 버전이 프로젝트 pin과 다른가 | build output은 고정 `2.0.22`를 표시한다. |
| 28 | image parser/build의 negative control이 부족한가 | 기존 malformed `](` form에서는 literal text가 생성됐고, `./`를 포함한 수정 form에서는 두 image asset이 생성된다. |
| 29 | Tailnet에 잘못된 README HTML을 먼저 게시하는가 | 이 수정 PR 병합과 Agent Browser 확인 전에는 preview sync를 보류한다. |
| 30 | 문서 렌더 수정을 C13.3 기능 증거 완료로 과장하는가 | 수정은 표시 경로만 다루고 AVD/실기기·제품 동작 gate는 바꾸지 않는다. |

수정 뒤 Rspress 2.0.22 scoped output의 두 image asset과 로컬 base-path Agent Browser 표시를 확인했다. Tailnet 제공 revision/hash는 병합 후 sync에서 확인한다.
