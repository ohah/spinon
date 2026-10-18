# C13.3 standalone UIKit 대조 harness·실행 실패 관점 검토 20개

1. **Run 02가 잘못된 Simulator에서 실행되는가?** — xcresult device ID·model·OS build를 함께 보존했다.
2. **test host의 bundle identifier가 제품 앱과 충돌하는가?** — 전용 `dev.spinon.probe.oracle.*` 식별자를 쓴다.
3. **test host가 Spinon production bundle을 수정·재설치하는가?** — 별도 UIKit host bundle만 빌드·실행했다.
4. **Run 02에 scroll view가 없어 boundary 측정이 무의미한가?** — Run 02는 UIScrollView `contentSize.width=20000`, 시작 x=10000으로 사용한다.
5. **Run 02 콘텐츠 프레임과 contentSize가 달라 예상하지 못한 가장자리가 생기는가?** — 둘 다 폭 20,000pt로 고정했다.
6. **비활성 bounce가 경계 offset을 왜곡하는가?** — UIScrollView bounce를 끄고 관찰된 실제 offsets도 원본으로 남긴다.
7. **Run 02 값이 가장자리에 근접했는데도 경계 외라 주장하는가?** — 8,919~10,791pt 범위와 시작점·콘텐츠 폭을 함께 기록한다.
8. **Run 03에 숨은 UIScrollView가 남아 있는가?** — plain mode는 scroll view를 view hierarchy에 추가하지 않는다.
9. **Run 03 recognizer가 여전히 root view에 붙어 제품 계층과 비교가 틀리는가?** — plain mode는 측정 target UIView 자체에 recognizer를 부착한다.
10. **Run 03 offset이 시스템 scroll 소비값처럼 해석되는가?** — callback translation delta를 수동 누산한 표식이며 `contentOffset`이 아니다.
11. **Run 03 XCTest가 target을 ScrollView로 찾아야만 동작하는가?** — accessibility Button target에 coordinate scroll을 요청하고 XCTest activity를 보존했다.
12. **callback label의 접근성 값이 측정에 사용되지 않는가?** — 매 요청 전후 `value`를 읽어 문자열 첨부물로 저장한다.
13. **유실된 OS log가 일부 반복 데이터를 없애는가?** — Run 02·03의 10행 첨부물이 xcresult에서 추출됐는지 확인한다.
14. **첨부물은 생성됐지만 xcresult에 보존되지 않았는가?** — export manifest의 attachment name·test identifier를 함께 저장했다.
15. **Cumulative translation을 action delta로 혼동하는가?** — adapter 소스처럼 began에서 baseline을 초기화하지만 README는 요청량과 관찰량을 동일시하지 않는다.
16. **recognizer state 종료 시 baseline이 다음 측정에 새지 않는가?** — ended/cancelled/failed 뒤 baseline을 초기화한다.
17. **UIScrollView simultaneous recognition 설정이 Run 03에도 영향을 주는가?** — plain mode는 recognizer delegate를 지정하지 않고 다른 recognizer를 설치하지 않는다.
18. **UI 테스트가 negative control 없이 실행되는가?** — direction별 callback이 없는 행을 명시하고 Run 01/02/03의 서로 다른 호스트 조건을 비교한다.
19. **테스트 1/1 pass를 기능 통과로 잘못 계산하는가?** — README·STATUS·roadmap에서 C13.3 미완료를 유지한다.
20. **실험이 증거 없이 제품 수정으로 이어지는가?** — 어느 단계의 누락인지 미확정이므로 harness/문서만 수정하고 Spinon production code는 건드리지 않는다.
