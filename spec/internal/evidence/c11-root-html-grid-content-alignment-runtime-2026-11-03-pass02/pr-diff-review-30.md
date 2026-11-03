# C11 Root HTML Grid 콘텐츠 정렬 PR 변경 검토 30개

최종 변경 전체를 서로 다른 회귀·계약 관점에서 검토했다. 검토 대상은 Rust cascade/projection/layout/runtime, fixture, 내부 계약·계획 및 실행 근거다.

1. **C11 parent 오승격:** `spec/STATUS.md`의 C11 체크를 바꾸지 않았고 공식 `72/213`을 유지한다.
2. **내부 버전 오승격:** 내부 계약 버전 `0.1.0`을 유지한다.
3. **공개 CSS 지원 오표시:** 계약과 계획이 후보를 public CSS admission과 구분한다.
4. **WPT 과장:** Chrome oracle 관찰을 Spinon WPT 적합성으로 표기하지 않는다.
5. **author cascade 누락:** Root HTML property registry와 longhand snapshot에 두 속성을 모두 연결했다.
6. **다른 cascade profile 오염:** 추가 property registry는 Root HTML text paint profile에 한정했다.
7. **align-content 값 경로 누락:** eligible Grid에서 computed value를 typed parser로 넘긴다.
8. **justify-content 값 경로 누락:** eligible Root HTML Grid 전용 match branch를 추가했다.
9. **Grid item alignment 오허용:** `align-items`, `justify-items`, self-alignment 허용 범위는 바꾸지 않았다.
10. **다른 Grid profile 회귀:** 기존 allow 예외를 명시 구조체로 옮기되 profile별 조건을 유지했다.
11. **허용 keyword 과다:** baseline 및 검토하지 않은 safe/unsafe modifier가 거부된다.
12. **키워드 누락:** normal/stretch, 위치·분배 값과 허용 safety 조합의 표를 양 property에 확인했다.
13. **그리드 자격 검사 순서:** eligibility는 typed snapshot과 host tree에서 computed nodes를 기준으로 판정한다.
14. **잘못된 tree identity:** container handle과 children, item owner 관계를 같은 snapshot에서 확인한다.
15. **빈 Grid 성공 오인:** direct item이 최소 하나 필요하다.
16. **whitespace text 배치 오염:** ASCII HTML whitespace text만 건너뛴다.
17. **unusual whitespace 허용:** 다른 text나 foreign node는 fail-closed다.
18. **oversized item count:** checked multiplication과 explicit cell capacity 비교를 사용한다.
19. **track token 불일치:** repeat count, auto-repeat, area와 line-name metadata를 검사한다.
20. **gap 중복 합산:** frame test가 fixed track·gap·distribution 조합의 정확한 좌표를 확인한다.
21. **음수 free space 누락:** negative container case가 center/end와 safe/unsafe를 양 축에서 검증한다.
22. **오차 허용 확대:** 허용치는 고정된 1/64 CSS px로 둔다.
23. **화면 캡처 오해:** README가 screenshot을 visual smoke로 한정하고 surface readback 부재를 명시한다.
24. **제품 앱 fixture 잔류:** bootstrap HTML/JS를 기존 SHA-256으로 되돌렸다.
25. **기본 runtime 미검증:** 실제 parsed Root HTML runtime test가 cascade부터 render snapshot까지 확인한다.
26. **플랫폼 build만으로 runtime 주장:** Android/iOS 앱 로그의 `layout=ready`, `boxes=7`과 screenshot을 각각 보관한다.
27. **Android 기기 유형 혼동:** API 37 AVD로 표기하고 실기기 근거와 구분한다.
28. **iOS 표면 단계 과장:** Metal draw log와 screenshot은 기록하되 compositor/pixel readback은 주장하지 않는다.
29. **evidence 손실:** 첫 iOS cascade 실패, Android 초기 hang, Chrome 비교 및 성공 캡처를 보존한다.
30. **문서-구현 동기화:** 계획, 내부 계약, runtime README가 현재 Android 성공 및 남은 범위를 일치시킨다.

이 검토는 checklist 수를 PR 본문에 성과 지표로 쓰기 위한 것이 아니다. 실제 PR을 만들 때 변경 요약, 테스트, simulator 근거와 한계를 본문에 싣는다.
