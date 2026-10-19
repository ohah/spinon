# C11.4 PR #368 Tailnet roadmap sync PR diff 실패 관점 검토 30개

**범위:** roadmap·STATUS·계획 갱신, 선택된 Tailnet site 동기화 helper, pass12/pass13 실행 결과 및 공개 PR 본문 증거를 서로 다른 실패 경로로 확인했다.

| # | 실패 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | 문서 동기화 PR에 Rust 제품 코드를 끼워 넣었나? | 변경은 roadmap·상태·계획·내부 색인과 Tailnet build/sync helper 및 문서 근거에 한정됐다. Rust 구현 변경은 없다. |
| 2 | 원래 checkout의 C11.3 작업을 섞었나? | 독립 branch는 병합된 `origin/main` `c8aa0939` 기준이며 원래 checkout의 staged/unstaged 변경은 건드리지 않았다. |
| 3 | PR #368 merge SHA가 모든 문서에서 일치하나? | plan, STATUS, roadmap note가 `c8aa093981a1b0d435347d9757b1fd0146a1f7db`를 사용한다. |
| 4 | merge method를 잘못 적었나? | GitHub의 PR #368 메타데이터와 plan은 Rebase and merge로 일치한다. |
| 5 | required checks 미설정을 CI 통과라고 표현했나? | required status check가 없다고 적었고 CI 통과 주장은 넣지 않았다. |
| 6 | 계획의 구현 gate를 전체 C11.4 완료로 해석하나? | 완료한 체크는 구현 PR #368의 merge gate뿐이고 상위 C11.4 작업 gate는 미완료다. |
| 7 | 공식 상태 체크를 실수로 완료했나? | `spec/STATUS.md`의 C11.4 checkbox는 unchecked로 유지했다. |
| 8 | 공식 완료 수를 올렸나? | roadmap은 `212개 표시 · 공식 완료 72/212`이고 공식 집계 변경은 없다. |
| 9 | roadmap note가 구현 범위를 넓혀 주장하나? | 양쪽 축별 단일 auto-repeat 제한, 12-case oracle, 33-scene simulator 근거와 남은 범위를 함께 적었다. |
| 10 | status index에서 끊어진 계획·근거 링크를 추가했나? | plan, precomparison, simulator evidence, 구현 검토, PR #368과 STATUS 대상 route를 Browser와 served hash로 확인했다. |
| 11 | 입력 manifest가 실제 pass13 선택을 반영하나? | pass13 source manifest의 240개 경로를 재해시해 mismatch 0개였다. |
| 12 | pass12 역사 manifest를 pass13 값으로 덮었나? | pass12 snapshot을 별도 파일로 보존하고 pass13 manifest는 다른 이름으로 기록했다. |
| 13 | run manifest가 생성되지 않고 선언만 됐나? | helper가 공용 manifest와 pass별 manifest를 각각 쓴다. |
| 14 | manifest가 자기 자신을 hash해 비결정 상태가 되나? | 공용·최종·현재 pass manifest를 입력 목록에서 제외한다. |
| 15 | 과거 pass6–11 백업 경로를 바꿨나? | 기존 pass 분기는 유지했다. pass13 재실행은 이전 backup의 다른 target SHA를 재사용하지 않도록 `pass13-report-refresh`, `pass13-final-review`, `pass13-final-links`, `pass13-final-report-wording` 전용 backup suffix를 썼다. |
| 16 | 빌드 산출물의 host 경로 또는 symlink가 preview에 새나? | 선택 staging의 199개 경로와 빌드 1개 파일을 정리했고 최종 sanitizer 검사에서 남은 절대 경로가 0개다. |
| 17 | 문서 도구 의존성을 올렸나? | 잠금된 Rspress 2.0.22 실행 파일을 썼고 package manifest 및 lockfile 변경이 없다. |
| 18 | 실수로 받은 Rspress 1.x로 preview를 빌드했나? | `bunx` 도움말 실행은 빌드에 사용하지 않았고 고정 로컬 2.0.22 binary로 build log를 만들었다. |
| 19 | 전체 저장소를 Tailnet에 공개했나? | 88개 명시 선택 항목으로 만든 sanitized site 1,522개만 preflight 및 copy 대상으로 삼았다. |
| 20 | 새 route가 `/spinon/` 밖을 가리키거나 새 링크가 빠졌나? | 최종 71 HTML route audit에서 same-origin out-of-base는 0개다. 선택하지 않은 과거 route 10개 결손은 기존 범위로 따로 기록했고, 신규 관련 route 8/8이 HTTP 200과 hash 일치다. |
| 21 | 초기 링크 검사 오판을 감췄나? | output 폴더만 검사한 첫 305개 누락 오판 JSON을 보존하고 실제 preview root 기준 10개 과거 대상과 분리했다. |
| 22 | pass13 manifest의 누락 경로 실패를 숨겼나? | 최초 ENOENT, 원인과 수정 내용을 별도 JSON으로 보존하고 per-pass manifest 경로를 고쳤다. |
| 23 | Tailnet 서버가 열어 둔 문서를 강제 교체했나? | PID 1191이 두 pass13 조사에서 열어 둔 HTML 2개와 추가 3개의 기존 bytes/SHA를 staging에 보존해 target을 쓰지 않았다. |
| 24 | 교체 파일 중 열린 handle이 남은 상태로 copy했나? | 최종 65개 교체 대상은 `lsof` 재검사에서 open 0개인 상태로 복사했다. |
| 25 | 기존 preview 파일을 backup 없이 바꾸거나 삭제했나? | 마지막 고유 backup root의 교체 파일 65/65가 bytes/SHA 일치하고 delete는 없었다. |
| 26 | atomic copy 일부만 성공했는데 전체 성공으로 적었나? | 생성 4 + 교체 65, 총 69개 변경을 source/target bytes와 SHA로 확인했고 모두 검증됐다. |
| 27 | 외부에서 제공된 Tailnet 응답과 local tree가 다르나? | HTTP 200 응답 1,522/1,522의 byte 수와 SHA-256이 모두 일치했다. |
| 28 | Agent Browser가 다른 주소나 오래된 문서를 확인했나? | 최종 Tailnet report·roadmap·sync-review를 직접 열었다. roadmap DOM은 72/212, C11.4 unchecked를 보였고 report의 그림은 6/6 로드됐다([최종 확인](./browser-final-check-pass13.json)). pass12 precomparison·simulator evidence 확인은 별도 browser JSON에 남겼다. |
| 29 | simulator 캡처를 실기기 증거로 부풀렸나? | Android API 37 ARM64 AVD 1080×2400, iPhone 17 Pro/iOS 26.2 Simulator 1206×2622 캡처를 구분한다. 실기기·hardware GPU·iOS compositor-after 결과는 미완료다. |
| 30 | Browser나 서버·tmux 프로세스를 잘못 정리했나? | 최종 Agent Browser session을 닫은 뒤 session inactive와 Agent Browser/Terminal Browser process 0개를 확인했다. Tailnet PID 1191과 13개 tmux session은 유지했다. |

## 변경 후 보류한 범위

이 PR은 PR #368 구현 자체를 다시 검증하거나 재실행하지 않는다. C11.4 전체 Grid·WPT 적합성, 실기기, hardware GPU 및 compositor-after pixel parity도 완료로 표시하지 않는다.
