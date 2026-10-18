# C13.3 iOS negative-X 재현 후속 PR 변경 실패 관점 30개

검토 대상은 이번 diff의 계획, 공식 상태 대장, XCTest 설명, Run 04–06 요약·원본 로그·첨부와 링크다. 앱 구현 코드는 변경하지 않았다. 각 항목은 새 증거가 과장되거나 잘못 집계될 수 있는 서로 다른 경계를 확인한다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | Run 04 XCTest 성공을 callback 성공으로 바꿈 | XCTest 3/3과 앱 callback/process 3/3을 별도 자료에서 대조했다. **통과.** |
| 2 | Run 04의 raw delta와 normalized 부호를 혼동 | 원본 raw X는 양수, normalized CSS X는 음수로 JSON에 각각 기록됐다. **통과.** |
| 3 | negative-only 입력을 음수 방향 소비 성공으로 셈 | 세 실행 모두 offset 0에서 시작해 remainder로 남았고 `published=false`다. **통과.** |
| 4 | Run 04 sample이 의도한 inner target 밖으로 간 경우를 성공에 포함 | 세 callback 모두 target 7이다. **통과.** |
| 5 | queue process 누락을 callback 성공으로 보정 | 세 callback count와 세 process count가 독립적으로 남아 있고 일치한다. **통과.** |
| 6 | Run 04 입력이 이전 process 상태에 의존 | summary에 반복마다 앱 재실행이 기록돼 있다. **통과.** |
| 7 | XCTest 요청 `−38`을 UIKit 실제 delta로 인용 | 계획과 README는 요청값과 관찰 delta를 다르게 설명하며 세 실제 delta를 명시한다. **통과.** |
| 8 | Run 05 양성 callback 누락을 숨김 | 다섯 번의 positive-X callback/process를 5/5로 분리 기록했다. **통과.** |
| 9 | Run 05 negative callback 누락을 숨김 | 요청 5회, 관찰 3회, 누락 반복 1·5를 app log와 summary에서 대조했다. **통과.** |
| 10 | 누락 callback을 adapter reject로 단정 | 해당 반복에 callback/process/reject가 없고 원인 단계를 미확정으로 썼다. **통과.** |
| 11 | Run 05의 전달 sample이 다른 target으로 감 | negative sample 세 개와 positive sample 다섯 개 모두 target 7이다. **통과.** |
| 12 | Run 05 요청 액션 수와 callback 분모 불일치 | actions JSON의 수평 왕복 10요청과 8개 처리 sample을 대조했다. **통과.** |
| 13 | final offset 합산이 기록된 delta와 다름 | 각 repetition에서 관찰된 양·음 delta와 누락 방향을 사용해 다섯 최종 offset을 다시 계산했다. **통과.** |
| 14 | delivered negative가 앞선 positive를 정확히 상쇄한다고 오해 | 세 final offset이 모두 0보다 크고 zero-return은 0/3이다. **통과.** |
| 15 | Run 05 결과를 네 방향 또는 전체 wheel matrix로 확대 | 문서는 수평 inner-target 시나리오만 판정한다. **통과.** |
| 16 | Run 06 실패를 0/5 또는 5/5로 집계 | result `unknown`, total test count 0, 완료 test 0을 게이트에서 제외했다. **통과.** |
| 17 | Run 06 저장 공간 실패 원인을 누락 입력의 원인으로 전이 | 저장 공간 오류는 xcresult write 단계로만 기록했고 callback 누락과 인과관계를 주장하지 않는다. **통과.** |
| 18 | Run 06 partial log를 완전한 repetition matrix로 취급 | 부분 관측만 기록하고 callback 비율·XCTest 성공률 분모로 사용하지 않는다. **통과.** |
| 19 | center coordinate sample을 inner target 7 reverse 증거로 사용 | partial negative sample의 target 5 변경을 드러내고 이를 inner-target 증거에서 제외했다. **통과.** |
| 20 | Run 06에서 실제로 저장된 app process/callback 수 왜곡 | partial summary와 app log에서 process 4개와 callback 6개를 대조했다. **통과.** |
| 21 | 세 run이 서로 다른 바이너리 결과를 섞음 | 각 summary의 app executable SHA-256이 동일한 opt-in 앱 해시다. **통과.** |
| 22 | 시뮬레이터 결과를 실기기 또는 전체 iOS 지원으로 확대 | 계획·evidence는 iPad Pro 13-inch M5, iOS 26.2 Simulator에 범위를 고정한다. **통과.** |
| 23 | 화면 첨부를 callback/queue 증거로 오인 | Run 05 캡처는 scene 확인용으로만 설명하고 callback 근거는 app log에 둔다. **통과.** |
| 24 | 검은 negative 캡처를 PR 이미지로 노출 | PR에 사용할 캡처는 기존 positive-X fixture scene으로 제한하며 negative 검은 화면은 참조하지 않는다. **통과.** |
| 25 | XCTest test method 자체를 사용자 동작 지원 계약으로 표현 | harness/evidence는 opt-in XCTest synthesis로 범위를 제한한다. **통과.** |
| 26 | 변경 문서가 앱 adapter를 수정했다고 암시 | diff의 실행 산출물과 문서만 바뀌며 adapter source는 변경되지 않는다. **통과.** |
| 27 | 공식 완료 체크 또는 count를 올림 | `spec/STATUS.md`의 C13.3 완료 표시는 유지하고 새 결과를 미완료 근거로 추가했다. **통과.** |
| 28 | 계획과 상태 대장의 Run 04–06 수치·한계 불일치 | plan 요약·상세, evidence README와 STATUS 문구를 서로 대조했다. **통과.** |
| 29 | 새 summary·log·action·screenshot이 해시 검증에서 빠짐 | `inner-target-reruns/SHA256SUMS`에 새 run의 보존 파일을 열거하고 자체 검증한다. **통과.** |
| 30 | 수정한 계획의 상대 링크가 계속 잘못되거나 후속 검토·manifest 링크가 빠짐 | API 29 근거 링크 네 곳을 `spec/internal/evidence`로 바로잡고 plan·evidence README가 이번 검토와 manifest를 가리키는지 확인했다. **통과.** |

## 판정

이번 문서 diff에서 callback 누락, boundary 미소비, 요청량과 UIKit 관찰량의 차이, partial run을 완성 결과로 오해할 위험을 숨기는 문제는 발견되지 않았다. C13.3은 미완료로 유지한다. Run 06은 저장 공간 오류로 완료되지 않았으며 재실행하지 않았다.
