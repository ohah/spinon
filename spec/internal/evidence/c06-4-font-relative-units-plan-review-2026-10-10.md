# C06.4 글꼴 상대 단위 계획 검토

**대상:** [구현 계획](../../../plan/c06-4-font-relative-units.md) · [내부 계약 0040](../0040-c06-font-relative-units.md)

계획에 합성 `body`의 UA baseline, html/body box·paint fail-closed 범위, flex computed/used-value 비교 규칙을 추가한 뒤 실패 경로를 다시 점검했다. 이 기록은 계획의 예방 계약을 검토하며 구현 완료 근거를 대신하지 않는다.

| # | 공격 경로 | 계획의 방어와 확인 근거 |
|---:|---|---|
| 1 | 기존 HostRoot를 CSS `:root`로 계산 | 합성 HTML root 아래 body와 mount를 둔다. Chromium/Rust에서 root 20px와 mount 10px를 분리한다. |
| 2 | root `font-size:1.25rem`이 새 root 값을 자기 참조 | html cascade 때 initial medium 16 CSS px를 유지해 자기 rem을 20px로 계산한다. |
| 3 | root computed size가 자식 cascade 뒤에 적용 | html 계산 직후 Device root size를 갱신한 뒤 body/mount를 cascade한다. rem target과 root-zero 테스트를 둔다. |
| 4 | body `font-size` 상속 경로 누락 | 합성 body computed style을 mount의 parent style로 넘기고 body 2rem → mount 40px 케이스를 확인한다. |
| 5 | 합성 body 기본 display가 browser block과 달라짐 | runtime UA baseline에서 body를 block으로 둔다. 사용자 display 변경은 block 이외 값이면 실패한다. |
| 6 | body nonzero margin/padding이 layout에서 조용히 소실 | body는 layout box가 아니므로 margin 0/padding 0만 허용하며 나머지는 오류 처리한다. |
| 7 | html width/height/flex-basis 지정이 viewport 효과 없이 무시 | 합성 html dimensions는 Auto만 허용한다. CSS width 지정 fixture는 오류가 나야 한다. |
| 8 | html margin/padding으로 root box 이동·축소가 소실 | 계산된 html margin/padding이 0이 아니면 오류 처리한다. |
| 9 | `display:none` 또는 flex body가 mount visibility/배치를 바꿈 | 합성 html/body의 computed display는 block만 허용해 숨김·flex wrapper 효과를 버리지 않는다. |
| 10 | html/body background-color가 viewport canvas에 그려지지 않음 | 완전 투명 외 synthetic background paint를 오류로 처리한다. 배경은 실제 mount node에 적용한다. |
| 11 | synthetic nodes가 HostDocument/Taffy/GPU에 추가 | 합성 요소는 HostNodeId가 없고 snapshot/frame 수는 12개 HostDocument 요소와 일치해야 한다. |
| 12 | `html`, `:root`, `body` selector가 내부 root와 불일치 | 합성 노드는 HTML namespace와 실제 local name을 갖고 Chromium fixture selector를 적용한다. |
| 13 | `font-size:em`이 같은 요소 크기 또는 잘못된 조상을 참조 | Stylo가 parent computed font-size로 계산하며 1.2em→12px, 자식 1.5em→18px를 비교한다. |
| 14 | 일반 em이 parent 대신 own computed font size를 쓰지 않음 | width/spacing/flex-basis는 Stylo typed computed px를 Taffy에 넘기고 own/parent가 다른 case를 포함한다. |
| 15 | font-size percentage 뒤의 em이 이전 값 사용 | 10px 부모의 150%가 15px이 되고 자손 1em/frame도 15px인지 비교한다. |
| 16 | zero root/element font size를 기본값으로 대체하거나 0 나눗셈 | 0px root에서 rem/em 길이가 0px인 Rust cascade 경로를 직접 확인한다. |
| 17 | `var()` 안 em/rem이 문자열 재계산으로 Stylo 결과와 달라짐 | Stylo custom-property substitution을 유지하고 2em/2rem typed width를 Chromium과 대조한다. |
| 18 | metric 단위가 inline/stylesheet/비활성 fallback/escaped 이름으로 통과 | token scanner가 입력 전체를 검사하고 각 source 경계·nested fallback을 negative fixture로 다룬다. |
| 19 | DPR 배율이 CSS px 값 또는 frame에 중복 적용 | DPR 1·2에서 computed values와 Taffy CSS px frame을 각각 비교하고 동일성을 확인한다. |
| 20 | flex item의 cascade computed height와 browser post-flex used height를 혼동 | Stylo typed pre-layout value와 Taffy 최종 rect를 검증하고 Chromium resolved height는 rect와 대조한다. Android/iOS 실행은 별도 관문이다. |

## 판정

합성 html/body는 cascade parent일 뿐 일반 문서 box가 아니며, 이 경계에서 layout·paint 결과가 빠지는 값을 오류로 차단하도록 계획을 구체화했다. Chromium precomparison, Rust/Taffy oracle, Android API 37 emulator, iPhone 17 Pro / iOS 26.2 Simulator, workspace test·Clippy·format·FFI check가 구현 완료 조건이다. 공개 `document.documentElement`, 일반 body box, 외부 CSS와 metric unit 제공은 C06.4에서 완료로 간주하지 않는다.
