# PR #652 병합 후 상태·로드맵 동기화 변경 적대 검토 30개

- 기준: `origin/main` `36394223bfdf0d4791178ccc248e96c825722354`
- 문서화 대상 구현: PR [#652](https://github.com/ohah/spinon/pull/652), merge SHA `baa55101a5557edbc547ccd4208db7f1906f5760`
- 검토 범위: 상세 계획, 상위 계획, 공식 상태 대장, 공개 로드맵 HTML, 이 검토 기록
- 결론: 아래 서로 다른 실패 경로 30개를 확인했다. 차단 결함 없음.

| # | 적대 관점 / 확인한 실패 경로 | 결과 |
|---:|---|---|
| 1 | GitHub에서 PR #652가 병합되지 않은 상태인데 병합으로 기록했는지 확인 | PASS — PR API가 `MERGED`와 병합 시각을 반환 |
| 2 | 기록한 merge SHA가 다른 PR 또는 다른 커밋인지 확인 | PASS — PR API와 `origin/main` Git 객체가 `baa55101a5557edbc547ccd4208db7f1906f5760`으로 일치 |
| 3 | Rebase and merge 표기가 merge commit 생성과 혼동되는지 확인 | PASS — 병합 객체는 단일 부모를 가지며 merge commit이 아님 |
| 4 | `spec/STATUS.md`에 새 C11.4 완료 체크를 추가했는지 확인 | PASS — C11.4 체크박스는 계속 미완료 |
| 5 | 제한 fixture 병합으로 공식 완료 집계가 바뀌었는지 확인 | PASS — 상태 대장과 두 계획 문서, 공개 로드맵 모두 `72/213` 유지 |
| 6 | 기존 #643 PNG sizing slice와 #652 overflow slice를 같은 범위로 합쳤는지 확인 | PASS — #643은 overflow 제외로 남고 #652는 별도 후속 scene으로 기록 |
| 7 | 상세 계획의 “PR diff 검토/병합 대기”가 남아 최신 상태와 충돌하는지 확인 | PASS — 상세 계획은 #652 병합 시각과 SHA를 명시 |
| 8 | 상위 계획의 최근 날짜 순서에서 #652가 빠졌거나 오래된 섹션에 들어갔는지 확인 | PASS — 2026-10-26 기록으로 추가되고 기존 10월 23일 항목보다 앞에 배치 |
| 9 | 공식 대장에서 #652가 #643 또는 #656 기록을 덮거나 중복하는지 확인 | PASS — #643 뒤, #656 앞에 독립된 한 줄로 기록 |
| 10 | 공개 로드맵에서 #652가 앞뒤 C11.4 기록과 시간 순서를 어기는지 확인 | PASS — #645 뒤, #656 앞에 한 항목으로 삽입 |
| 11 | border-box 12px에서 content width 10px 산술이 잘못됐는지 확인 | PASS — 양쪽 1px border를 제외한 값과 기록이 일치 |
| 12 | 고정 PNG의 자연 크기와 scene의 image size가 뒤바뀌었는지 확인 | PASS — 입력과 기대값 모두 24×16px |
| 13 | 첫 track과 이웃 고정 track의 순서가 바뀌었는지 확인 | PASS — 기대 columns는 `[24,17]px` |
| 14 | row track 결과를 columns 값으로 잘못 재사용했는지 확인 | PASS — 기대 row는 `[16]px` |
| 15 | image frame 원점이 border 안쪽의 content origin을 반영하는지 확인 | PASS — root border 1px을 반영한 `(1,1,24,16)` |
| 16 | border 바깥 overflow 길이를 border edge가 아닌 content edge에서 계산했는지 확인 | PASS — image 오른쪽 `x=25`, root border 오른쪽 `x=12`, 차이 13px |
| 17 | 두 simulator에서 실행한 scene 수와 control 포함 여부를 과장했는지 확인 | PASS — Android와 iOS 각각 control·overflow 2/2로 기록 |
| 18 | Android PixelCopy 표본과 iOS Metal 표본 수를 서로 혼동했는지 확인 | PASS — 각 4/4이며 원 실행 근거가 두 경로를 분리 |
| 19 | iOS drawable readback을 compositor 이후 화면 검증으로 확대했는지 확인 | PASS — 문서가 제출 전 Metal 표본이라고 한정 |
| 20 | API 37 AVD를 실기기나 hardware GPU로 표현했는지 확인 | PASS — Android software-rendered AVD라고 명시 |
| 21 | `image_draw_count=0`을 PNG texel paint 통과로 해석했는지 확인 | PASS — box marker만 보이고 PNG texel은 확인하지 않았다고 명시 |
| 22 | 첫 iOS 3/4 mismatch를 최종 통과로 덮었는지 확인 | PASS — 최초 실패와 surface-clear 기대값 수정이 runtime evidence에 남아 있음 |
| 23 | 첫 Android scene allowlist 실패를 통과 횟수에 합산했는지 확인 | PASS — 실패 시도는 보존되고 수정 후 실행만 통과로 집계 |
| 24 | 일반 CSS overflow·clip 지원이나 C11.4 전체 지원을 암시하는지 확인 | PASS — 일반 CSS·WPT·전체 C11.4 미완료를 반복해 명시 |
| 25 | 새 API/계약 또는 계약 버전 변경을 근거 없이 주장했는지 확인 | PASS — 새 API를 주장하지 않고 기존 계약 0059 범위에 연결 |
| 26 | 계획·사전 비교·runtime evidence 상대 링크가 잘못된 기준 디렉터리를 쓰는지 확인 | PASS — 세 Markdown 링크가 현재 저장소 파일로 해석됨 |
| 27 | 공개 로드맵 screenshot 링크가 실행 산출물 파일과 맞지 않는지 확인 | PASS — Android/iOS 각각의 최종 rerun PNG 경로가 존재 |
| 28 | 공개 HTML 문단 삽입이 기존 paragraph 경계나 앵커를 깨뜨렸는지 확인 | PASS — HTMLParser가 전체 파일을 오류 없이 처리하고 새 `p`/`a` 경계를 인식 |
| 29 | 한국어 프로젝트 문서 규칙 또는 변경 범위를 벗어나 불필요한 API 설명을 추가했는지 확인 | PASS — 한글 기록이며 제품 소스·계약은 바꾸지 않고 상태표·계획·공개 HTML과 검토 근거만 갱신 |
| 30 | 30개 검토 결과를 테스트·CI·실기기 증거로 과장했는지 확인 | PASS — 문서 구조 확인만 했고 새 테스트·CI·실기기 실행을 주장하지 않음 |

이 변경은 이미 병합된 제한 fixture의 상태를 동기화한다. 상위 C11.4 완료 판정이나 공개 CSS 지원 범위를 바꾸지 않는다.
