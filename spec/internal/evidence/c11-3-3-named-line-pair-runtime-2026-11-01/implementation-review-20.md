# C11.3.3 양 끝 positive implicit named-line pair 구현·runtime 실패 관점 검토 20개

검토 대상은 pair admission/projection, Grid layout validation, implicit track expansion, C11.3.3 tests, opt-in FFI/app route와 고정 Chrome/Android/iOS 실행이다. 계획 검토표와 다른 구현·runtime 실패 경로를 확인했다.

1. **원본 grammar:** pair eligibility는 resolver 진입 전의 양 endpoint가 모두 `NamedLine`일 때만 설정한다. 숫자 line·`auto`·span을 pair로 승격하지 않는다. PASS.
2. **양 endpoint provenance:** 두 endpoint가 각각 resolver에서 implicit named grid line을 사용했는지 확인한다. explicit named line과 implicit line의 혼합은 제외된다. PASS.
3. **positive trailing 경계:** 양 endpoint가 양수이고 explicit 마지막 line을 넘어야 한다. negative leading/trailing 동작은 기존 signed mode로 남는다. PASS.
4. **occurrence 해석:** bare name과 `2 alpha / 3 omega` occurrence는 기존 named-line resolver를 사용한다. CSSOM reference와 Rust scenes가 서로 다른 track sequence를 고정한다. PASS.
5. **equal endpoint:** `alpha / alpha`에서 CSS Grid의 end 제거 동작을 별도 Chrome scene 및 layout test로 고정한다. PASS.
6. **역순 endpoint:** `2 omega / 1 alpha`가 resolver/layout 경계에서 line order를 뒤집어도 root-relative item frame은 Chrome과 일치한다. PASS.
7. **다중 track sizing:** item의 fixed 30px contribution이 두 auto implicit column에 15px씩 분배되는 사례를 CSSOM track list와 Taffy trace test에 고정한다. PASS.
8. **축 분리:** Columns/Rows pair 모두 실행한다. opposite axis의 explicit line은 별도로 확인한다. PASS.
9. **두 축 동시 확장:** mode validator는 `columns_pair == rows_pair`일 때 거부하므로 둘 다 false 또는 둘 다 true인 모호한 mode를 허용하지 않는다. style/runtime negative case가 both-implicit shape를 확인한다. PASS.
10. **fixed item 크기:** projection은 width와 height가 모두 fixed일 때만 mode를 선택하며, auto width 또는 height는 기존 fail-closed path에 남는다. PASS.
11. **참여 child 수:** layout 경계가 모든 direct layout-participating child를 다시 세고 정확히 하나가 아니면 거부한다. 추가 visible item 테스트가 있다. PASS.
12. **숨김과 visibility:** layout의 `hidden` 판정에서 `display:none` subtree는 참여 수에서 빠지고 `visibility:hidden`은 layout 참여를 유지한다. 그 결과로 second visible child를 숨김 속성만으로 우회할 수 없다. PASS(code path).
13. **placement shape:** layout validator가 다시 두 positive pair line, 단일 implicit 축, opposite definite axis를 확인한다. CSS adapter metadata 오류만 믿고 mode를 허용하지 않는다. PASS.
14. **template-area 우회:** style admission은 areas 존재 시 pair flag를 만들지 않고, layout 입력을 직접 구성해도 별도 mode guard가 areas를 거부한다. PASS.
15. **auto-track grammar:** pair mode에서 `auto`와 fixed-px list만 처리하며 빈/지원 외 track value는 일반 validation 오류로 fail closed 한다. PASS.
16. **track ceiling:** 10,000 tracks는 성공, 10,001개 요구는 error를 반환한다. 한도 시험은 선택적인 전체 trace 수집과 분리했다. PASS.
17. **기존 profile 상호작용:** 기존 positive named-start + auto-end, fixed-px control, signed extent 테스트가 pair eligibility가 없을 때 각각 원래 mode를 유지한다. PASS.
18. **fixture 격리:** 새 scene source는 opt-in fixture feature에서만 정적 archive에 나타나며 default `spinon-ffi` artifact에는 scene marker/fixture ABI가 없다. PASS.
19. **Android 표면 경로:** Activity route, demo allowlist, sample item ID/좌표/RGBA를 모두 추가했고 root 바깥 sample 두 장면이 PixelCopy 1/1 exact다. 잘못된 좌표와 미등록 route는 readback/draw 전에 거부된다. PASS(AVD only).
20. **iOS 표면 경로:** Swift route와 pixel allowlist를 추가했다. 두 축 모두 Metal drawable sample 1/1 exact이며 잘못된 표본 좌표는 readback 전에 거부된다. OS compositor 이후와 실기기 경로는 미검증으로 남긴다. PASS(Simulator boundary).

검토 중 발견해 해결한 사항:

- 기존 style test가 `grid-row:missing / missing` fixed item을 무조건 거부하던 것은 새 pair contract와 충돌했다. 동일 line의 positive pair로 포함되는 frame `(0, 30, 10, 10)`을 확인하도록 assertion을 갱신했다.
- 10,000-track 테스트에서 전체 track trace를 켜자 trace allocation error가 cap 판정을 가렸다. 이 cap 전용 입력에서 trace를 끄고 정확한 경계 성공 및 초과 입력 거부를 확인한다.
- 전체 두 스타일/layout 패키지 실행은 C07 aspect-ratio computed-style precision assertion 한 건에서 실패했다. 해당 테스트 파일과 관련 projection 코드는 이 diff에서 변경하지 않았다. 나머지 C11.3.3 subset, layout package, workspace Clippy는 별도로 실행해 기록한다.
