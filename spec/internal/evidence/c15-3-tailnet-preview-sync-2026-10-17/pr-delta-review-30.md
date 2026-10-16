# C15.3 Tailnet 동기화 PR 변경 적대 검토 30개 관점

대상은 C15.3 Chrome DOM 사전 비교 문서의 이미지·fixture 링크 수정, Tailnet 미리보기 동기화 기록, 내부 evidence 색인 및 JSON/화면 근거다. 제품 런타임 구현이나 C15 지원 완료 주장은 이 diff에 없다.

| # | 실패 관점 | 확인한 위험과 결과 |
|---:|---|---|
| 1 | 잘못된 기준 revision에서 로드맵을 복사 | PR #246의 merge commit `0c900ea8ba2f63ae95814b779fda4c9b595b0e12`을 branch base와 preflight가 가리킨다. 후보 입력별 hash는 manifest에 기록한다. |
| 2 | C15.3 계획을 구현 완료로 표시 | 로드맵과 STATUS는 C15.3을 계획 단계, C15/C16/C19를 미완료로 유지한다. Agent Browser에서 로드맵·STATUS 본문을 다시 확인했다. |
| 3 | 제품 완료 수와 칸반 소계 혼동 | 공개 상태 대장은 “현재 제품 지원 완료 없음”을 표시한다. 로드맵 칸반의 71/203은 작업 항목 카운터로 분리되어 있다. |
| 4 | scoped build를 전체 문서 빌드로 과장 | 검증은 고정 C15 입력을 모은 Rspress 2.0.22 scoped build다. 저장소 전체 build 성공은 주장하지 않는다. |
| 5 | Markdown 이미지가 번들러에서 모듈로 해석 | 이미지 주소를 `./chrome155.png`로 명시했다. Rspress가 `static/image/chrome155.bb1ba0e6c6.png`를 생성하고 브라우저에서 640×760으로 읽었다. |
| 6 | 파일명이 맞아도 캡처 bytes가 다름 | 원본 PNG SHA-256 `ba827180aac697b472b771b801102744c90b13f570816986c2d99c8d51810417`을 manifest에 대조했다. |
| 7 | Tailnet base path를 벗어나는 raw fixture link | 첫 화면 점검에서 `../../../../tests/...`가 `/tests`로 빠지는 결함을 발견했다. 저장소 원본은 GitHub 링크, 실제 미리보기는 `/spinon/tests/...`의 별도 링크로 바꾸고 그 링크를 클릭해 fixture를 열었다. |
| 8 | font README 링크도 같은 base-path 오류를 냄 | font README를 GitHub 원본과 `/spinon/tests/fixtures/resources/c15/c15-3/README.html` 제공 페이지로 분리했다. 최종 페이지에 `/tests/`로 시작하는 같은 출처 링크가 없다. |
| 9 | fixture 파일이 링크만 있고 미리보기에 없음 | HTML fixture의 Tailnet route를 열고 Hangul·CJK·Arabic·emoji·overflow 입력을 확인했다. |
| 10 | fixture가 시스템 fallback font에 조용히 의존 | Chromium의 `document.fonts.status`가 `loaded`이고 Noto KR, Arabic, Emoji, Roboto, Ahem 다섯 `document.fonts.check()`가 모두 참이다. |
| 11 | font URL 상대 경로가 C15.3 폴더 복사 뒤 깨짐 | fixture의 `../../resources/c15/...` 경로에 맞춰 세 Noto font와 기존 Roboto/Ahem을 같은 상대 경로로 복사하고 전부 로드했다. |
| 12 | font provenance/license가 빠짐 | Noto CJK·Arabic·Emoji OFL 원문과 hash 안내 README route를 포함했고, Tailnet 페이지에서 OFL 1.1 및 SHA-256 표가 열렸다. |
| 13 | 이미지가 빌드되었어도 제공 CDN 경로에서 404 | 최종 근거 페이지의 이미지 `naturalWidth=640`, `naturalHeight=760`, `complete=true`를 Agent Browser에서 확인했다. |
| 14 | 변경한 문서 외 stale route 제공 | STATUS, conformance, internal index, contract 0070과 필요한 C15.3 HTML route를 같은 scoped build에서 갱신했다. source mirror와 route bytes를 별도로 manifest한다. |
| 15 | 동기화 중 기존 페이지를 무백업 덮어씀 | 최종 사전 점검은 교체 파일의 이전 SHA-256과 backup path를 고정하고, 백업 bytes를 각 이전 hash와 다시 비교한다. 결과는 [backup verification](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/backup-verification.json)에 있다. |
| 16 | 복사 전 다른 프로세스가 source 또는 target을 바꿈 | preflight 이후 source hash와 기존 target hash를 재비교하고 최종 copy verification으로 destination bytes를 확인했다. 상세 기록은 [copy verification](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/copy-verification.json)이다. |
| 17 | manifest가 예정 목록과 실제 제공 파일을 다르게 기록 | 최종 manifest는 경로·byte 수·SHA-256·created/replaced/unchanged와 backup path를 담는다. manifest 자체 hash는 내부 목록에서 빼고 별도 응답 hash로 확인한다. |
| 18 | copy-only 주장 뒤에 예전 산출물을 삭제 | 최종 preflight와 manifest의 `deletions`는 빈 배열이다. 기존 경로는 제거하지 않았다. |
| 19 | 검증 편의를 위해 기존 Tailnet 서버 재시작 | 동기화 전후 listener는 동일한 `node` PID `1191`, `127.0.0.1:8768`이다. 프로세스에 stop/restart 신호를 보내지 않았다. |
| 20 | Agent Browser가 작업 뒤 남거나 Terminal Browser로 대체 | 화면·클릭·폰트·manifest 검증은 Agent Browser에서 수행하고 전용 session을 닫았다. 종료 뒤 Chrome/session 프로세스가 남지 않았고 Terminal Browser는 사용하지 않았다. |
| 21 | 20개 검토 URL이나 인덱스 링크가 이전 파일명에 남음 | 내부 인덱스와 동기화 README는 최종 30개 검토 route를 가리킨다. 기존 20개 경로는 최종 문서로 연결하는 안내 페이지로 바꿔 이전 링크도 유효하게 유지한다. |
| 22 | Markdown 검토 표와 렌더링된 HTML의 항목 수가 다름 | 저장소 원문은 1–30 순서의 고유 행 30개를 가진다. Rspress 출력과 Agent Browser DOM에서도 30행을 각각 센다. |
| 23 | 검토 문서는 열리지만 Rspress 검색에서 누락 | 새 30개 페이지의 route 제목이 fresh scoped build의 검색 인덱스에 들어가고 Tailnet 경로로 검색 결과 링크가 제공된다. |
| 24 | README가 Tailnet에 복사하지 않은 JSON을 같은 origin에 있는 것처럼 링크 | JSON 기록 링크는 명시적으로 GitHub `blob/main` 경로를 사용한다. Tailnet에는 정적 문서·fixture route만 제공한다고 설명한다. |
| 25 | PR 화면 이미지 URL이 병합 전 branch에서 접근 불가 | PNG는 PR의 고정 commit을 가리키는 raw GitHub URL로 본문에 넣어, main 병합 전에도 같은 화면을 읽을 수 있게 한다. |
| 26 | 문서 동기화 과정에서 공식 제품 상태를 바꿈 | `spec/STATUS.md`에는 변경이 없고 C15/C16/C19는 계속 미완료다. 칸반 71/203은 제품 지원 완료 수와 분리해 적는다. |
| 27 | Rspress가 이전 산출물을 재사용해 검토 제목/링크가 오래됨 | `site-scoped-publish2`라는 새 output directory에 Rspress 2.0.22를 빌드하고 새 route HTML·검색 인덱스가 생성됐는지 확인한다. |
| 28 | 재동기화 manifest가 초기·final·publish manifest를 덮어씀 | 이번 실행은 별도 `publish2` manifest 경로와 backup root를 사용한다. 기존 세 manifest와 백업을 보존하고 이번 preflight에서 새 경로가 비어 있는지 확인한다. |
| 29 | 저장소 밖 임시 sync script/build 산출물이 PR에 섞임 | script·Rspress scope/output은 `/private/tmp` 아래에 두고, PR은 source 문서·evidence JSON·화면 PNG만 포함하는지 staged path를 대조한다. |
| 30 | 기존 20개 Tailnet 페이지가 오래된 내용으로 계속 노출 | 기존 route를 삭제하지 않고 새 30개 review 문서로 안내하는 짧은 보존 페이지를 배포한다. 최종 인덱스의 canonical 링크는 새 30개 route를 사용한다. |

## 남은 경계

이 PR은 정적 문서와 고정 fixture 파일의 Tailnet 제공을 확인한다. C15.3 shaping, glyph ID/WGPU paint, overflow ink/clip/hit-test 통합 또는 Android/iOS 앱 실행은 이 결과에 포함하지 않는다. 전체 WPT 적합성과 실기기 검증도 주장하지 않는다.
