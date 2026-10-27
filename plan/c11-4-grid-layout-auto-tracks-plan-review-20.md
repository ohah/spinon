# C11.4 `grid-layout-auto-tracks` 계획 실패 관점 검토 20개

계획 파일 [c11-4-grid-layout-auto-tracks-wpt.md](c11-4-grid-layout-auto-tracks-wpt.md)와 사전 비교 자료를 대상으로 구현 전에 서로 다른 실패 경계를 확인했다.

| # | 실패 관점 | 계획의 예방 기준 | 판정 |
|---:|---|---|---|
| 1 | WPT 원본이 다른 revision으로 바뀐다 | revision과 원본 SHA-256 고정 | PASS |
| 2 | reftest 화면 캡처를 testharness PASS로 보고한다 | 테스트 종류를 reftest로 명시하고 브라우저 관찰과 분리 | PASS |
| 3 | reference 파일의 hash가 바뀐다 | reference 경로와 SHA-256 별도 고정 | PASS |
| 4 | Chrome 기본 글꼴의 19px을 앱의 규범값으로 사용한다 | 원본 font 관찰과 Ahem normalized oracle 분리 | PASS |
| 5 | Ahem font 미로드 상태를 테스트 입력으로 착각한다 | font-ready 및 `document.fonts.check()` 증거 요구 | PASS |
| 6 | 첫 row의 두 item contribution을 합산한다 | 같은 track에는 maximum 10px을 기대 | PASS |
| 7 | 명시 row만 세어 positive implicit row를 누락한다 | `c`의 row 2와 최종 2-row trace 요구 | PASS |
| 8 | positive implicit column의 placement가 잘못된다 | `b`의 row 1 / column 2 위치와 frame 검사 | PASS |
| 9 | 50px nested child가 column sizing에 반영되지 않는다 | implicit column 50px 및 child frame 50px 요구 | PASS |
| 10 | implicit column을 root width 안에 축소한다 | root width 100px와 total columns 150px을 따로 검사 | PASS |
| 11 | root auto height가 row height와 불일치한다 | root 20px와 `[10,10]` rows를 교차 검사 | PASS |
| 12 | root/item frame만 맞고 내부 stage가 다르다 | columns/rows track trace 및 contribution도 검사 | PASS |
| 13 | gutter/sentinel이 CSS track으로 계산된다 | 0 gap과 gutter/collapsed-entry 분리 검사 | PASS |
| 14 | B item 대신 nested child의 frame을 item으로 본다 | root, 세 grid item, B child identity/frame을 별도 확인 | PASS |
| 15 | direct text, child text와 item ownership이 섞인다 | exact DOM topology·source ownership·text 검사 | PASS |
| 16 | auto item이 stretch되어 intrinsic contribution을 가린다 | `align-items:start` 고정 및 track trace 검사 | PASS |
| 17 | root `height:auto`를 definite height로 잘못 처리한다 | auto-sized root sizing 및 20px content-driven 결과 고정 | PASS |
| 18 | scene ID allowlist 바깥의 CSS가 route를 통과한다 | root/style/placement를 pre-layout fail-closed로 검증 | PASS |
| 19 | fixture 경로가 제품 artifact에 포함된다 | 기본 archive negative와 opt-in positive control 대조 | PASS |
| 20 | 부분 layout·draw 결과가 성공처럼 남는다 | revision, error atomicity, draw/present와 surface sample을 확인 | PASS |

## 판정

계획 검토 PASS 20/20, PARTIAL 0, FAIL 0. 구현 전 검토이며 구현/runtime 검증이나 PR diff review를 대신하지 않는다.
