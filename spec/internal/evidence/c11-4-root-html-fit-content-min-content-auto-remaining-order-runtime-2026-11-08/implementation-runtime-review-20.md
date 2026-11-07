# C11.4 fit-content/min-content/auto 순서 구현·runtime 실패 관점 검토 20개

기능 코드와 실행 근거를 대상으로 계획 검토와 별도로 확인했다.

1. **허용 목록 길이가 늘어나며 임의 track 조합을 열지 않는가?** 세 항목 exact pattern 여섯 개만 통과하고 layout list 길이 제한을 유지한다.
2. **기존 두 순서가 퇴행하지 않는가?** `fit-content min-content auto`, `fit-content auto min-content` projection/layout/runtime 회귀가 기존 테스트와 함께 남아 있다.
3. **누락·중복 intrinsic member가 다른 위치에서 잘못 허용되지 않는가?** 모든 허용 pattern은 `fit`, `min-content`, `auto`를 하나씩 포함하고 길이가 정확히 3이다.
4. **`fit-content()`가 다른 항목 위치일 때도 해당 cap을 검사하는가?** pattern의 `fit` binding 위치와 상관없이 단일 bound cap에 finite·non-negative 검사를 적용한다.
5. **음수·NaN·무한 cap이 projection을 통과하지 않는가?** 신규 order 말단 및 중간 위치의 px/% 검사에서 거부된다.
6. **모든 CSS 길이 표현을 accidental로 허용하지 않는가?** 현재 exact gate는 direct px 및 direct percentage만 받으며 calc·fraction·minmax는 기존 실패 경계를 유지한다.
7. **percentage cap이 indefinite 축에서 무의미하게 확장되지 않는가?** runtime fixture는 definite 100px content-box height를 사용하고, layout 계약의 definite-root-size 검증을 유지한다.
8. **Cascade와 Style projection이 order allowlist에서 갈라지지 않는가?** computed-value gate가 여섯 순서를 보존하고 기존 single projection path로 같은 typed list를 넘긴다.
9. **Layout gate와 auto-placement gate 중 한쪽만 새 order를 허용하지 않는가?** `supports_min_content_auto_track_list`와 sparse normalizer의 fit/min/auto branches를 대조했다.
10. **Auto-placement gate에서 cap validation이 누락되지 않는가?** 네 추가 pattern 모두 direct px/% cap의 finite 및 non-negative 검사를 거친다.
11. **implicit row 목록이 explicit row부터 잘못 소비되지 않는가?** explicit 10px row 다음 첫 implicit row부터 pattern을 적용하는 frame 기대값을 검사한다.
12. **목록 순환이 4번째 implicit row에서 깨지지 않는가?** explicit row와 implicit row 네 개의 used-height 배열이 네 신규 order마다 두 번째 pattern cycle을 포함한다.
13. **fit-content cap이 max-content contribution보다 커질 때 임의 상한으로 clamp되지 않는가?** cap 20px, payload 34px의 item/payload frame을 각각 비교해 track와 visible overflow를 구별한다.
14. **fit-content track 자체가 auto처럼 stretch를 받지 않는가?** 네 order의 기대값은 cap 20px을 보존하고 free space는 `auto` track만 받는다.
15. **min-content track이 free space를 받지 않는가?** expected rows에서 34px 또는 10px의 contribution 크기가 그대로 유지된다.
16. **auto 한 개와 두 개의 free-space 분배가 혼동되지 않는가?** 네 산술 profile은 각 auto track에 나뉘는 20, 6, 10/10, 3/3px를 구분한다.
17. **기대 geometry가 Chrome 좌표를 그대로 베끼지 않는가?** 비교 계획의 base-size/free-space 산식에서 rows를 계산했고 브라우저 관측은 cross-check로만 기록했다.
18. **unsupported revision에서 부분 render/layout이 노출되지 않는가?** RuntimeSession 검증이 invalid dark-scheme revision의 `completed=None`을 확인한다.
19. **실패 후 같은 session이 회복 가능한가?** light-scheme으로 돌아온 후 valid new order에서 Ready 및 네 번째 implicit row frame을 확인한다.
20. **시뮬레이터 성공이 surface pixel parity로 과장되지 않는가?** 같은 source hash, 12-box ready/present 로그와 보이는 화면만 기록하고 readback 검증은 범위 밖으로 명시했다.

별도 runtime harness 관찰: Android의 최초 screenshot이 draw-present 전에 찍히는 capture race를 확인했다. 빈 이미지를 실패 artifact로 보존하고 현재 PID의 present marker를 기다리는 capture script로 재촬영했다. 이는 geometry 코드 실패가 아니라 증거 수집 순서 오류였다.
