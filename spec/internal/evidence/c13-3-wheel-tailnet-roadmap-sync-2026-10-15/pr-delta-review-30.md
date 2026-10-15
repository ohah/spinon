# Tailnet 로드맵 동기화 PR 변경 검토 30개 · 2026-10-15

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | Tailnet의 실제 정적 root가 달라져 다른 디렉터리에 게시 | Agent Browser가 지정된 `/spinon/roadmap.html`을 열었고 대상은 SpinonPreview였다. 통과. |
| 2 | 새 로드맵 화면에서 wheel 설명이 빠지거나 옛 페이지가 남음 | 갱신된 C13.3 설명과 실행 근거 링크가 표시됐다. 통과. |
| 3 | 기본 `index.html`을 덮어써 기존 미리보기가 바뀜 | index는 후보 목록과 복사 대상에서 제외했다. 통과. |
| 4 | #194가 아닌 다른 코드 revision의 증거를 연결 | 문서 기준 main SHA를 `28b3ba214fc681b6327da607288a990e55e96f06`로 고정했다. 통과. |
| 5 | unpinned Rspress가 다른 HTML을 생성 | 설치된 저장소 pinned Rspress `2.0.22`로 빌드했다. 통과. |
| 6 | 임시 config가 저장소 Rspress 설정을 바꿈 | 별도 `/tmp` config를 사용했고 저장소 config 변경은 없다. 통과. |
| 7 | 원본 빌드가 전체 문서 사이트를 덮어씀 | 제한 staging root와 지정된 산출물 경로만 사용했다. 통과. |
| 8 | Tailnet 배포 전에 생성기 오류가 남음 | 제한 Rspress build가 성공했다. 통과. |
| 9 | Markdown의 상대 이미지가 생성 단계에서 누락 | 여섯 이미지에 `./`를 추가한 뒤 실행 페이지 HTML에 모두 생성됐다. 통과. |
| 10 | Android 이미지 중 일부가 빠짐 | 세 장 모두 Agent Browser에서 `complete=true`, 원본 1080×2400이었다. 통과. |
| 11 | iOS 이미지 중 일부가 빠짐 | 세 장 모두 Agent Browser에서 `complete=true`, 원본 1206×2622이었다. 통과. |
| 12 | 화면이 빈 placeholder 또는 0px로 렌더링 | 여섯 이미지 모두 양수 너비와 natural dimensions가 있었다. 통과. |
| 13 | 이미지 alt가 없어 어떤 장면인지 식별 불가 | 여섯 시나리오별 alt 텍스트를 유지했다. 통과. |
| 14 | 실행 근거 HTML 경로가 로드되지 않음 | Tailnet 로드맵 링크를 클릭해 `README.html`이 열렸다. 통과. |
| 15 | 계획 링크의 대상이 없거나 잘못 연결 | wheel delta routing HTML을 해당 경로에 배치했고 빌드 산출물이 존재한다. 통과. |
| 16 | 관련 status/계약/scroll plan HTML이 오래된 채 남음 | 필요한 세 페이지를 새 산출물로 교체했다. 통과. |
| 17 | 실행 페이지 링크는 있지만 근거 원본 파일이 빠짐 | 해당 evidence 디렉터리 전체를 복사했고 링크 자료 12개가 HTTP 200이었다. 통과. |
| 18 | 정적 JS/CSS chunk 누락으로 화면이 깨짐 | 생성된 `static/**` 전부 반영하고 Agent Browser에서 페이지를 열었다. 통과. |
| 19 | fingerprint가 같은 이름의 기존 정적 파일을 무심코 교체 | 기존 동일 hash 파일은 그대로 두고 5개 동일 후보는 skip했다. 통과. |
| 20 | 서로 다른 기존 정적 리소스를 백업 없이 교체 | 바뀐 기존 파일 여섯 개를 복사 전에 백업하고 이전 hash를 기록했다. 통과. |
| 21 | 새 정적 경로가 대소문자 차이로 404 | 최종 manifest에 기록한 exact path들이 모두 HTTP 200이었다. 통과. |
| 22 | 서버가 경로 대신 fallback HTML을 200으로 응답 | 각 served body SHA를 로컬 예상 hash와 대조해 manifest 전체 항목이 일치했다. 통과. |
| 23 | 로드맵 파일은 갱신됐지만 앱 근거 링크가 예전 내용을 가리킴 | roadmap→README 경로와 화면 링크를 Agent Browser에서 열었다. 통과. |
| 24 | C13.3 조각을 전체 구현 완료로 표시 | 로드맵과 실행 페이지에 synthetic fixture 및 미검증 범위를 남겼다. 통과. |
| 25 | 시뮬레이터 화면을 실기기 근거로 오인 | Android AVD와 iOS Simulator라는 실행 환경을 명시했다. 통과. |
| 26 | screenshot 차이를 compositor/scanout 검증으로 확대 해석 | 문서에 compositor 이후 출력 미검증을 유지했다. 통과. |
| 27 | #171 이미지 문제가 Tailnet 이미지 문제와 혼동 | #171 세 raw 이미지가 Agent Browser에서 정상 로드됨을 따로 기록했다. 통과. |
| 28 | 기존 페이지가 복구 불가능하게 덮임 | 여섯 교체 파일의 사전 백업이 있고 manifest에 각 이전 hash가 있다. 통과. |
| 29 | 무관한 정적 페이지를 일괄 덮거나 삭제 | 대상은 네 기존 HTML, 새 wheel 문서/근거, 새 static 파일로 한정했다. 삭제는 없다. 통과. |
| 30 | 수동 미리보기를 Pages 배포 또는 재시작으로 오인 | 정적 디렉터리 동기화만 했고 Pages workflow·gh-pages·서버 재시작은 사용하지 않았다. 통과. |

## 판정

갱신한 Tailnet 화면, 여섯 이미지, 114개 served SHA-256을 확인했다. PR #171 원격 이미지 URL에는 수정할 오류가 재현되지 않았다. Tailnet wheel evidence Markdown의 상대 이미지 경로 오류를 수정했다. 이 운영 동기화는 C13.3 제품 지원 범위를 넓히지 않는다.
