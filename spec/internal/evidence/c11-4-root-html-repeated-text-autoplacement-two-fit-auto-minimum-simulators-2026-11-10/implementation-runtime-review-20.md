# C11.4 두 fit-content auto-placement automatic minimum 구현/runtime 검토 20개

최종 style projection, layout contract, RuntimeSession 결과와 iOS·Android 기본 Root HTML 실행을 서로 다른 실패 조건으로 점검했다. Chrome 157 관측 2,880개 auto-minimum 장면과 기존 2,880개 min-height:0 장면을 다시 대조했다.

1. **지원 track profile을 임의로 넓힘:** 두-fit auto-placement 여섯 member의 기존 인식 함수와 fit 개수 2를 모두 요구한다. one-fit·three-fit·tail multiplicity mutation이 거부되어 통과.
2. **fit 위치 중복/순서 혼동:** 각 장면에서 authored list와 계산 track 순서를 비교하고 위치·순열 전체를 돌린다. 2,880/2,880 일치로 통과.
3. **px·% cap을 같은 입력으로 취급:** 관측 입력의 typed cap을 그대로 생성하고 zero/percentage 조합을 따로 대조한다. 0px와 0% 장면까지 전체 비교가 통과.
4. **text item 하나만 auto minimum 적용:** 계산 style의 여덟 text item 각각에서 computed min-height를 Chrome 기대값과 비교한다. auto profile에서 2,880/2,880 통과.
5. **빈 explicit-row control의 minimum까지 auto로 허용:** 새 profile은 첫 item min-height:0도 요구한다. control만 auto로 바꾼 negative scene이 거부되어 통과.
6. **0과 auto minimum 혼합:** 여덟 text item 중 하나만 0으로 바꾼 scene을 거부한다. mixed-minimum negative test 통과.
7. **one-fit auto-placement의 auto minimum을 우회 허용:** one-fit list mutation에서 min-height:auto가 거부된다. existing min-height:0 baseline 2,880 scenes는 별도 비교에서 통과.
8. **row-span과 auto-placement의 조합을 허용:** text item을 span topology로 바꾼 장면을 거부한다. mixed topology negative test 통과.
9. **텍스트 외 payload가 intrinsic minimum을 오염:** nested span child를 추가한 scene을 거부한다. direct payload 제한 test 통과.
10. **implicit column 또는 column/dense flow로 확장:** 두 번째 column, column auto-flow, dense flow mutations를 거부한다. negative matrix 통과.
11. **percentage cap에 definite basis가 없음:** Grid block size를 auto로 바꾼 scene이 거부된다. 유효한 definite 220px basis는 전체 matrix에서 통과.
12. **zero fit cap이 content-based automatic minimum을 깎음:** feature trace에서 두 fit item 모두 `AutomaticMinimum { content_based: true }`와 양수 10px line floor를 확인한다. 최종 row base가 minimum 이상으로 통과.
13. **track trace가 다른 owner나 row를 가리킴:** trace node를 계산된 단일 Grid owner와 비교하고 explicit 1 + implicit 8 rows를 확인한다. zero-cap trace test 통과.
14. **text wrapping·line-height가 플랫폼별로 달라짐:** Chrome 장면별 payload line count, text frame과 item frame을 비교한다. 자동 minimum 2,880 scenes에서 전부 통과.
15. **start/stretch가 서로 다른 track 크기로 변형됨:** 1,440개 고유 list의 start/stretch 결과와 auto row 발생 횟수를 검사한다. baseline 및 auto-minimum matrix 통과.
16. **auto minimum 때문에 item frame이 cap 아래로 내려감:** 각 장면의 fit row content floor와 frame을 대조한다. 자동 minimum matrix에서 floor 위반 0건.
17. **숨겨진 미디어 Grid의 alignment가 현재 scheme을 실패시킴:** inactive `display:none` subtree를 projection alignment 검증에서 제외하고, hidden `justify-content`는 layout default로 둔다. 라이트·다크 전환 회귀가 current environment revision과 함께 통과.
18. **숨겨진 장면을 표시할 때도 검증을 건너뜀:** 같은 RuntimeSession에서 light와 dark를 각각 활성화하고 보이는 Grid의 start/stretch 및 Ready layout을 확인한다. 두 scheme 모두 통과.
19. **거부 후 이전 frames가 유효한 것처럼 재사용됨:** invalid environment에서 completion 부재를 확인한 뒤 같은 session의 유효 scheme으로 복귀한다. atomic rejection/recovery test 통과.
20. **시뮬레이터의 실제 Root HTML route에서 draw가 실패:** iOS 26.2 Simulator와 Android API 37.2 ARM64 AVD의 라이트·다크 로그 모두 app boot pass, layout-ready, `draw presented`를 기록하고 네 화면을 캡처했다. Android는 기존 공유 AVD를 read-only로 실행했다.

## 실행 결과

`runtime-tests.log`의 auto-placement 관련 5개 테스트는 5/5 통과했다. feature-gated zero-cap trace도 통과했고, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`가 통과했다. Simulator/AVD는 검증 후 종료했고 작업용 iOS profile과 실패한 작업용 Android AVD를 삭제했다.
