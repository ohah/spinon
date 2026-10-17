# C15.3 hit-test 로드맵 PR 변경 실패 관점 30개

검토 대상은 사용자용 정적 로드맵에 추가하는 PR #258의 제한 probe 기록이다. 구현 자체의 20개 및 #258의 PR diff 검토와 구분해 시간 순서, 상태 의미, 사용자에게 보이는 범위와 링크의 실패 가능성을 확인했다.

| # | PR 변경 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 새 기록이 C15.3 owner overflow clip slice 앞에 삽입된다 | 새 문단은 기존 owner clip 기록 바로 다음, 더 오래된 C14.1 기록 전에 놓인다. 통과. |
| 2 | 날짜나 병합 PR 번호가 실제 변경과 다르다 | 기록은 `2026-10-17` 및 병합 PR #258을 가리킨다. 통과. |
| 3 | PR #258의 변경 범위를 전체 C15 구현으로 넓힌다 | 고정 Ahem owner-box hit-test probe만 설명한다. 통과. |
| 4 | 상위 C15.3 상태를 완료로 표현한다 | 문단에서 C15.3과 C15가 미완료라고 명시한다. 통과. |
| 5 | 공식 STATUS 체크박스가 함께 바뀌어 버린다 | 변경은 사용자용 roadmap HTML 한 파일에만 있고 STATUS를 수정하지 않는다. 통과. |
| 6 | 공식 완료 체크 수를 늘린다 | 체크박스나 체크 항목 수를 수정하지 않는다. 통과. |
| 7 | 지원 API 카탈로그에 내부 probe가 노출된다 | 공개 API 지원 목록을 변경하지 않는다. 통과. |
| 8 | visible sample이 실제 owner box 영역 밖이라고 오해된다 | 문구는 parent 경계 밖이라는 조건만 설명하고 owner-box hit 결과로 한정한다. 통과. |
| 9 | visible hit target을 DOM `Element` identity로 과장한다 | runtime snapshot owner 결과를 적으며 DOM API 결과라고 쓰지 않는다. 통과. |
| 10 | hidden clip 안쪽 표본이 hidden owner를 반환하는 기대값이 누락된다 | hidden clip 내부 점에서 hidden owner를 반환한다고 명시한다. 통과. |
| 11 | hidden clip 바깥 결과를 반드시 `None`으로 고정한다 | 문구는 hidden owner 제외만 뜻하며 underlying fallback의 세부 값은 주장하지 않는다. 통과. |
| 12 | hidden clip 바깥에서도 hidden owner를 허용한다 | 해당 지점에서 hidden owner가 제외된다고 기록한다. 통과. |
| 13 | visible owner에 잘못 적용한 clip negative control을 놓친다 | 잘못된 clip 거부 사례를 별도 확인 결과로 포함한다. 통과. |
| 14 | overlay hit order가 text owner를 덮어쓰는 오류를 놓친다 | 더 높은 hit-order overlay가 우선한 control을 기록한다. 통과. |
| 15 | 임의 font나 glyph 전체를 확인한 것으로 보인다 | 고정 Ahem `XX` 샘플이라고 명시한다. 통과. |
| 16 | sample 위치와 기존 surface sample을 연결했다고 과장한다 | 고정 fixture hit-test와 표면 paint sample을 별도 근거로 설명한다. 통과. |
| 17 | PixelCopy/Metal 표본을 hit-test 결과로 잘못 읽는다 | 3/3 값은 paint 표본이고 owner hit report와 분리한다고 명시한다. 통과. |
| 18 | iOS 표본을 compositor 이후 화면 결과로 확대한다 | iOS Simulator Metal drawable 한계는 실행 근거에서 확인하고 실기기/compositor-after를 미검증으로 적는다. 통과. |
| 19 | Android 결과를 실기기 결과로 오해한다 | Android API 37 ARM64 AVD로 한정하고 실기기를 미검증으로 적는다. 통과. |
| 20 | SwiftShader/Metal Simulator를 hardware GPU 근거로 부른다 | hardware GPU가 미검증이라고 명시한다. 통과. |
| 21 | screenshot을 실제 DOM 이벤트 전달 증거로 쓴다 | 별도 hit-test report와 surface paint evidence를 구분하고 DOM event를 제외한다. 통과. |
| 22 | glyph alpha mask 기반 문자 단위 hit geometry를 지원한다고 오해된다 | glyph별 hit geometry 미검증을 남은 범위에 명시한다. 통과. |
| 23 | whitespace·grapheme/source range targeting을 확인했다고 오해된다 | CSSOM/DOM event 및 glyph geometry 미검증 경계가 유지된다. 통과. |
| 24 | non-zero scroll offset도 지원한다고 읽힌다 | scroll transform은 미검증이라고 명시한다. 통과. |
| 25 | 이 probe가 CSS overflow 전체 값을 보증한다 | 제한된 hit-test slice이며 전체 C15 완료가 아니라고 적는다. 통과. |
| 26 | 내부 계약 링크가 다른 문서 ID나 제품 공개 API를 가리킨다 | `/spinon/internal/0073-c15-3-overflow-text-owner-hit-test.html`은 현재 C15.3 probe 계약을 가리킨다. 통과. |
| 27 | 실행 근거 링크가 PR 브랜치 파일명에 의존하거나 끊긴다 | canonical Tailnet evidence route는 `/spinon/internal/evidence/c15-3-overflow-text-hit-test-2026-10-17/README.html`이다. 통과. |
| 28 | PR 링크가 병합 전 임시 branch나 잘못된 번호를 사용한다 | GitHub pull/258 URL로 직접 연결한다. 통과. |
| 29 | 기존 C15.3·C14.1 기록이 삭제되거나 순서가 깨진다 | 기존 문단은 그대로 두고 새 C15.3 기록을 한 문단 추가한다. 통과. |
| 30 | 문구가 아직 결정되지 않은 app event 계약을 확정한다 | owner-box hit-test 한계와 DOM event 미검증을 유지하며 출시·호환성 정책을 변경하지 않는다. 통과. |

이 검토는 정적 roadmap 문단과 연결 링크의 범위만 다룬다. simulator 실행, 제품 CSS 지원, 전체 DOM hit-test 또는 C15 완료를 대신하지 않는다.
