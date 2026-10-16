# C15.2b Tailnet 미리보기 동기화 PR 변경 검토 30개

검토 대상은 `README.md`, 사전 점검·복사·제공 검증 JSON, manifest, Agent Browser 캡처다. 각 항목은 별개의 실패 조건을 확인한다.

| # | 실패 관점 | 확인 기준과 결과 |
|---:|---|---|
| 1 | 원본이 병합 전 후보 commit을 가리킴 | source commit은 PR #243 병합 뒤 `origin/main` `17825ee576593c66c94307e4db4965c3f8e90f01`이다. 통과. |
| 2 | source tree가 다른 문서 산출물을 가리킴 | manifest의 source tree와 `origin/main` tree가 `48397c305b5411eb104b056d0498bbc2c367424c`로 일치한다. 통과. |
| 3 | 구현 PR과 로드맵 PR 병합 순서가 혼동됨 | #242 구현 merge `6a4d512224c1fe596bbbdefd3e9f31f3e093ccb5`, #243 로드맵 merge `17825ee...`를 분리해 적었다. 통과. |
| 4 | 미리보기 base path를 누락해 자산 route가 깨짐 | 사이트 base와 링크가 `/spinon/` prefix를 사용한다. 통과. |
| 5 | 빌드가 전체 문서에서 성공한 것처럼 읽힘 | scoped build만 성공으로 기록하고 repository-wide 실패 472개 진단을 명시했다. 통과. |
| 6 | scoped build가 어떤 자료를 담았는지 불명확함 | 상태, 적합성, 내부 계약 및 C15.2b 근거로 범위를 한정했다. 통과. |
| 7 | scoped link/image 검사 비활성화를 전체 설정에 적용한 것으로 오해 | 임시 scope에만 적용했으며 전체 build 완료를 주장하지 않는다. 통과. |
| 8 | C15.2b 문서 자체가 dead-link 원인이라고 잘못 설명 | 기존 build log는 새 README/review를 진단 목록에서 제외한다. 통과. |
| 9 | 계획 경로 수와 실제 manifest 항목 수가 다름 | 계획·manifest·브라우저 검증이 모두 73개다. 통과. |
| 10 | 전체 바이트 수가 복사 결과와 다름 | 사전 점검, 로컬 복사 검증, 브라우저 검증이 모두 4,276,838 bytes다. 통과. |
| 11 | 생성/교체/동일 분류 합계가 총계와 다름 | 45 + 17 + 11 = 73이다. 통과. |
| 12 | 사전 점검에서 삭제 대상을 숨김 | `deletions`가 빈 목록이고 README도 0개로 기록한다. 통과. |
| 13 | 교체된 기존 파일의 복구본이 없음 | 교체 17개는 사전 지정 backup root로 백업되었다. 통과. |
| 14 | unchanged 파일을 다시 쓴 것으로 표기 | manifest와 copy verification이 unchanged를 별도로 보존한다. 통과. |
| 15 | source commit과 복사 후 source가 바뀜 | copy verification은 원본 commit을 기록하고 파일별 hash를 보존한다. 통과. |
| 16 | 제공 URL에서 manifest 위치를 잘못 계산 | 브라우저가 `/spinon/sync-manifest-…json`에서 HTTP 200을 확인했다. 통과. |
| 17 | manifest가 자기 hash를 내부 항목에 넣어 순환함 | self hash 제외를 명시하고 별도 hash를 검증 결과에 둔다. 통과. |
| 18 | manifest의 별도 hash가 로컬 기준과 다름 | Agent Browser 응답 hash와 로컬 manifest SHA-256이 모두 `76753f6b…3258`이다. 통과. |
| 19 | 파일 fetch가 cache 결과를 재사용 | Agent Browser의 각 fetch에 `cache: no-store`를 지정했다. 통과. |
| 20 | HTTP 200이어도 내용이 달라짐 | 73개 response body 각각의 SHA-256 및 byte count를 manifest와 대조했다. 통과. |
| 21 | 원시 파일 검사와 UI route 검사를 같은 것으로 취급 | README는 manifest hash 검사와 Rspress route 화면 확인을 구분한다. 통과. |
| 22 | 평탄화된 상태 route 대신 `/spec/STATUS.html`로 안내 | UI route를 `/spinon/STATUS.html`로 명시하고 mirror는 raw 파일 검사로 제한했다. 통과. |
| 23 | C15가 완료로 표시됨 | Agent Browser DOM에서 C15 checkbox는 unchecked이며 README도 미완료로 기록한다. 통과. |
| 24 | roadmap 화면이 병합된 C15.2b 변경을 보여주지 않음 | Agent Browser에서 PR #242 병합 기록 문단을 확인하고 화면을 저장했다. 통과. |
| 25 | 근거 페이지의 Chrome oracle 이미지가 깨짐 | DOM에서 이미지 `complete=true`, 크기 1280×633을 확인했다. 통과. |
| 26 | 캡처를 앱 runtime 화면으로 과장 | README는 정적 문서 페이지 capture이며 앱 glyph/render 확인이 아니라고 설명한다. 통과. |
| 27 | Android/iOS target compile을 simulator app 검증으로 확대 | target compile 한계와 미검증 화면/ink 범위를 별도로 적었다. 통과. |
| 28 | preview server를 재시작하거나 잘못된 프로세스를 종료 | PID 1191과 `127.0.0.1:8768` listener가 유지되었다. 통과. |
| 29 | 브라우저 종료가 preview server 종료와 혼동됨 | Agent Browser만 종료 대상이며 Tailnet preview server는 유지한다고 분리했다. 통과. |
| 30 | 미리보기 검증으로 제품 지원이나 C15 완료를 선언 | 변경 범위를 정적 문서 복사·제공 검증으로 한정했다. 통과. |

## 결과

30개 관점을 확인했고 수정이 필요한 결함은 없었다. C15는 미완료이며 scoped docs build와 Tailnet 화면 검증은 전체 문서 build, 앱 실행, simulator 제품 화면을 대신하지 않는다.
