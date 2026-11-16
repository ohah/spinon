# C15.3 Flex intrinsic text layout · implementation review 20

## 판정

**GO for the bounded Rust layout regression only.** 이 테스트가 입증하는 것은 고정 Noto 입력을 사용한 Rust `LayoutInput`의 Flex max-content 결과와 Chrome rect의 근접성이다. 정확한 intrinsic callback 종류, CSS/HTML 통합 또는 C15.3/C15 완료 근거는 아니다.

## 20 관점

1. **Oracle 값 — PASS.** C15.3 계획과 Chrome observation 모두 `한글字`, Noto Sans KR 20px에서 Flex rect `56.8125×24px`를 기록한다.
2. **Oracle 출처 — PASS.** Chrome 155 환경, fixture hash와 observation hash가 기록돼 있다.
3. **DOM rect/Canvas advance 혼용 — PASS.** 테스트는 Flex box rect를 비교하고 glyph advance와 구분한다.
4. **입력 문자열 — PASS.** 정확한 `한글字`를 사용한다.
5. **고정 폰트 asset — PASS, 작은 공백.** repository Noto Sans KR bytes를 사용하며 README hash와 일치하지만 test 자체는 hash를 assert하지 않는다.
6. **폰트 등록 — PASS.** 엔진에 고정 Noto asset을 등록하고 실패 시 중단한다.
7. **시스템 fallback — PASS.** `new_without_system_fonts()`를 사용한다.
8. **weight/style — PASS.** 기본 400/normal 값은 fixture와 맞는다.
9. **glyph identity — OUT OF SCOPE.** 개별 glyph ID를 검사하지 않는다.
10. **Flex direction — PASS.** row 방향이다.
11. **max-content 부모 — PASS.** 부모 크기와 sizing mode를 max-content로 둔다.
12. **자동 Flex item — PASS.** width/height/flex-basis가 Auto이고 사전 계산 크기를 주입하지 않는다.
13. **min/max 강제 — PASS WITH LIMIT.** 고정 constraint는 없지만 기본값을 별도 assert하지 않는다.
14. **Flex default 차이 — DOCUMENTED.** 내부 flex_shrink 기본값은 CSS 기본값과 다르지만 이 max-content 입력의 결과에는 영향이 없다.
15. **nowrap — PASS.** 고정 입력과 Chrome fixture 모두 no-wrap이다.
16. **frame 결과 — PASS.** 부모와 item 폭/높이를 기대값 0.05px 이내로 비교한다.
17. **최종 text snapshot — PASS.** owner, source/style, revision, 한 줄 높이와 available width를 확인한다.
18. **callback 종류 — GAP.** trace는 Grid 측정만 공개하므로 Flex의 정확한 intrinsic callback 입력은 관찰하지 않는다.
19. **false-positive 경로 — PASS WITH LIMIT.** auto item/사전 주입 없는 layout consumer를 확인하지만 nowrap은 definite/max-content 폭의 동등성 때문에 callback 종류 변화를 잡지 않는다.
20. **cfg/build/evidence boundary — PASS.** test-only 모듈이고 focused log hash가 README와 일치한다. log는 source hash를 담지 않으며 결과를 Rust layout consumer 범위로 한정한다.

세 번의 독립성 적대 검토도 이 test-only 범위가 C11.4와 파일 수준에서 겹치지 않는다고 판정했다. Cargo `target`은 공유하므로 장시간 C11.4 matrix와 Rust test build는 순서대로 실행했다.
