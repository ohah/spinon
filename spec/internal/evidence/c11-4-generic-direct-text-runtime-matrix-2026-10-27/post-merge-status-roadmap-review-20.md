# C11.4 병합 후 상태·로드맵 검토 · 20개 관점

PR #679의 병합 정보와 저장된 시뮬레이터 근거를 기준으로 상태 대장, 계획, 실행 근거, 공개 로드맵 갱신분을 대조했다. 구현 범위·공식 완료 수·미검증 경계 사이에 모순이 없다. Tailnet 배포는 문서 병합 뒤 별도로 확인하며 이 변경에서는 완료했다고 주장하지 않는다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 제한된 fixture 결과로 상위 C11 체크가 완료 처리된다. | 상위 C11 체크는 미완료로 남아 있다. |
| 2 | 시뮬레이터 결과만으로 공식 완료 수가 늘어난다. | 공식 완료 수는 72/213이며 체크리스트 ID와 집계 원본은 바뀌지 않았다. |
| 3 | 대조군을 포함한 여섯 장면을 다섯 장면으로 잘못 센다. | 상태와 로드맵 모두 장면 6개와 플랫폼별 6/6 통과를 기록한다. |
| 4 | Android AVD 결과가 실기기나 hardware GPU 검증으로 확대된다. | API 37 ARM64 AVD로 적고 hardware GPU는 범위에서 제외했다. |
| 5 | iOS Simulator 결과가 실제 iPhone 검증으로 확대된다. | iPhone 17 Pro / iOS 26.2 Simulator로 적었다. |
| 6 | 전체 결과만 기록해 실패 또는 누락 장면을 감춘다. | 장면별 결과가 있는 플랫폼 matrix와 실행 README로 연결한다. |
| 7 | layout box를 글리프 페인팅으로 설명한다. | `runtime_text_count=0`, `glyph_batch_draw_count=0`을 밝혀 둔다. |
| 8 | draw/present를 픽셀 동등성으로 확대한다. | box/surface present만 확인했으며 픽셀 일치는 주장하지 않는다. |
| 9 | opt-in 경로 결과가 일반 CSS 지원으로 표현된다. | 일반 CSS admission과 임의 HTML/text 동작은 미검증으로 남긴다. |
| 10 | archive marker 확인을 APK/IPA 전체 fixture 제외로 확대한다. | Rust static archive 검사만 기록하고 전체 APK/IPA 주장은 제외한다. |
| 11 | 테스트한 source와 병합된 revision을 혼동한다. | PR head와 병합 SHA를 각각 기록하고 tree 일치를 따로 확인했다. |
| 12 | 시간이나 SHA를 앞선 계획 PR에서 복사한다. | PR #679의 병합 시각과 SHA `093a2adfa846f0ad0e7218074d34821f1825d771`을 기록한다. |
| 13 | 빈 check rollup을 CI 통과로 적는다. | rollup이 비었고 main required status checks가 설정되지 않았다고 적었다. |
| 14 | 계획 단계라는 오래된 상태 문구가 남는다. | 상태 대장과 계획은 구현 및 시뮬레이터 matrix 완료로 갱신했다. |
| 15 | README나 로드맵이 병합되지 않은 산출물을 가리킨다. | 링크 대상은 PR #679 tree에 있으며 PR 본문 이미지 URL은 Agent Browser에서 로드됐다. |
| 16 | 화면 URL이 잘못되었거나 다른 실행의 이미지를 가리킨다. | 두 링크가 6/6을 완료한 pass02의 화면을 가리킨다. |
| 17 | 계획이 Stage A 전체 또는 C11.4 전체 완료를 뜻한다. | 범위를 여섯 고정 direct-text positive 장면으로 제한하고 남은 범위를 유지했다. |
| 18 | 공개 로드맵과 공식 STATUS의 완료 상태가 다르다. | 둘 다 C11.4/C11 미완료와 72/213을 유지한다. |
| 19 | Tailnet 게시 전에 미리보기 배포를 완료로 주장한다. | 병합 후 Agent Browser 동기화 검증에서만 게시 여부를 확인한다고 적었다. |
| 20 | 무관한 row-axis 실행 파일이나 상태 항목을 함께 변경한다. | 변경은 generic direct-text 상태, 계획 상태, 실행 README, 관련 로드맵 문단과 이 검토 근거에 한정된다. |

이 검토는 병합된 PR tree와 GitHub 메타데이터를 기준으로 한다. Tailnet 미리보기 갱신은 확인하지 않았다. 게시 완료를 주장하려면 별도 사전 점검, 선택 빌드, 안전한 복사·백업, 제공 hash 확인, Agent Browser 화면 확인과 브라우저 프로세스 종료 기록이 필요하다.
