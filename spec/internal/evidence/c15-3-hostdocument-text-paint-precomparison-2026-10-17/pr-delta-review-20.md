# C15.3 HostDocument→runtime paint 계획 PR 변경 검토 20개

대상은 C15 plan amendment, C15 상태 요약과 다음 slice 계획 기록이다. 아래는 계획 자체의 실패 관점 검토와 겹치지 않게 변경 diff·인용·상태 일치만 검사한 결과다. 구현 코드나 실행 gate pass를 판정하지 않는다.

1. **기능 제목의 범위** — “HostDocument text layout→runtime glyph paint”가 CSS 지원으로 읽힐 수 있는지 확인했다. 계획·상태 모두 fixed fixture 제한과 C15 미완료를 함께 적어 범위를 닫았다.
2. **HostDocument 입력의 provenance** — live V8 문서로 오해할 수 있는지 확인했다. `fixture-owned HostDocumentSnapshot`이며 live V8 DOM은 제외한다고 plan, status, precomparison에 적었다.
3. **기준 fixture의 동일성** — `long-word`를 임의로 수정했는지 확인했다. 원 HTML과 원 observation은 그대로 두고 absolute label 제거 변형을 별도 JSON으로 기록했다.
4. **absolute label 제거 판정** — label 전후 box, text range, 두 character rect가 동등한지 확인했다. Agent Browser 결과가 모두 동일하고 `geometryEqual=true`다.
5. **fixture hash** — C15.1 HTML hash를 현재 checkout에서 재계산했다. 기록된 `efba6981…daf64`와 일치한다.
6. **원 Chrome JSON hash** — 사전 비교 JSON hash를 재계산했다. 기록된 `8c4673df…226e86`와 일치한다.
7. **Ahem hash** — 현재 pinned font bytes를 재계산했다. 기록된 `b719ecb3…b8448`과 일치한다.
8. **새 observation browser** — requested browser type을 점검했다. Agent Browser 0.31.1이 Chrome 155.0.8059.85를 열었으며 terminal-browser는 사용하지 않았다.
9. **browser process cleanup** — browser close를 요청하고 exact process name을 확인했다. `agent-browser close` 뒤 `pgrep -x agent-browser`는 비어 있었다.
10. **절대 y 좌표 의존성** — 기존 oracle과 새 캡처의 viewport 높이가 다른데 절대 page y를 acceptance로 쓰는지 확인했다. parent-relative geometry만 판정하고 screenshot y는 제한 자료로 남긴다.
11. **Block/text 크기 분리** — parent 15px과 text 20px이 혼동되는지 대조했다. Chrome Range는 20px, parent box는 15px로 분리해 기록했다.
12. **5px overflow 의미** — line을 잘라 맞춘 성공을 허용하는지 점검했다. owner 밖 5px ink를 유지하고 viewport 안 sample을 요구한다.
13. **Text source identity** — actual direct text node 입력이 string-only helper로 축약될 여지가 있는지 확인했다. HostDocument source node와 UTF-16 range 연결이 gate다.
14. **layout snapshot 재사용** — Taffy output 이후 별도 shape를 허용하는지 점검했다. 동일 `LayoutOutput.text_layouts`의 revision과 glyph 결과를 paint까지 전달하도록 명시했다.
15. **C16 계약 누수** — CSS font parsing/cascade나 fallback을 계획에 섞었는지 확인했다. 고정 `LayoutStyle`/`ParagraphStyle`과 명시 Ahem만 포함한다.
16. **기본 artifact 오염** — fixture font/FFI symbol이 product build에 포함될 수 있는지 점검했다. default/opt-in artifact 대조를 required gate로 기록했다.
17. **pixel oracle 과장** — Chrome screenshot을 byte-exact 기준으로 쓰는지 확인했다. Chrome은 DOM geometry oracle이며 app 표면의 opaque sample은 각 simulator에서 별도 readback한다.
18. **플랫폼 범위** — simulator 결과를 실기기·hardware GPU·iOS compositor 결과로 일반화하는지 확인했다. AVD PixelCopy와 iOS pre-compositor Metal drawable의 경계를 적었다.
19. **공식 체크박스** — 계획 amendment만으로 C15를 완료 체크하는지 검토했다. 상위 checkbox는 unchecked 그대로이며 status에 구현 결과가 없다고 표시했다.
20. **완료 수와 Tailnet 주장** — 근거 없는 count/Tailnet update를 주장하는지 확인했다. `61/197`을 유지하고 plan PR 단계에서 Tailnet sync 완료를 주장하지 않는다.

## 판정

20개 변경 관점에서 계획·상태·사전 비교 자료 사이의 범위 모순은 발견하지 못했다. 남은 작업은 계획 PR의 required checks와 병합 후 구현이며, 이 검토는 그 실행 결과를 대신하지 않는다.
