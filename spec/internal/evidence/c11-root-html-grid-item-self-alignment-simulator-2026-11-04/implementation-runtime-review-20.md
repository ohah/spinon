# C11.5 구현/runtime 실패 관점 검토 20개

이 검토는 C11.5 구현 candidate의 실제 코드, Chrome 기준, Rust runtime, Android/iOS simulator readback과 artifact 경계를 검사한다. 계획 PR의 20개 검토 및 계획 PR diff 검토 30개와 별개다. Candidate는 아직 구현 PR로 병합되지 않았고 broad Cargo package tests는 green이 아니다.

| # | 구현/runtime 실패 관점 | 확인 결과 |
|---|---|---|
| 1 | self-alignment가 Root HTML 외 computed-style profile로 새어 나가는가 | **통과** — owner eligibility 집합은 `RuntimeRootHtmlTextPaintV1`에서만 만든다. 다른 profile은 `LayoutStyle::default().justify_self == Auto`를 사용한다. |
| 2 | 한 item이 자기 Grid owner가 아닌 다른 적격 Grid의 허용을 빌리는가 | **통과** — `grid_item_owners`와 적격 container set의 교집합으로 item set을 만들며 exact-owner 단위 테스트가 통과했다. |
| 3 | nested, spanning, text 또는 non-empty item이 단일 direct-child cell로 취급되는가 | **통과** — eligibility는 empty direct `div`, exact owner, auto placement와 capacity를 검사한다. 이 조건 밖에서는 non-initial computed self value가 거부된다. |
| 4 | dense/column flow, repeat/area, fractional 또는 implicit track이 후보 profile에 들어오는가 | **통과** — 기존 fixed-track owner eligibility를 재사용하고 C11.5 runtime의 fractional-track negative control은 exact `align-self` error로 실패한다. |
| 5 | 위치 정렬 mapping이 item의 명시 fixed width/height를 바꾸는가 | **통과** — 19-scene fixture에서 item size는 고정하고 x/y 위치만 기대값과 비교한다. Chrome precomparison과 Rust matrix가 exact다. |
| 6 | `align-self` block 축과 `justify-self` inline 축 또는 physical keyword가 뒤바뀌는가 | **통과** — 비대칭 expected coordinates와 `left/right` LTR control이 일치한다. `align-self:left/right`는 allowlist 밖이다. |
| 7 | computed `auto`가 부모의 다른 axis 또는 초기 `normal`로 잘못 대체되는가 | **통과** — `align-self:auto`/`justify-self:auto`를 Taffy `Auto`로 전달하며 parent `align-items`/`justify-items` 기본값 관찰과 일치한다. |
| 8 | `place-self` shorthand의 첫째/둘째 값 또는 single-value 복제가 뒤집히는가 | **통과** — authored shorthand를 파싱하지 않고 Stylo computed longhand 둘을 사용한다. 비대칭 two-axis scenes의 위치가 expected JSON과 일치한다. |
| 9 | `left/right`가 `align-self`에 허용되거나 다른 writing mode에 일반화되는가 | **통과** — physical alignment는 `justify-self`의 LTR `horizontal-tb`에서만 start/end로 mapping한다. |
| 10 | safe overflow가 음수 위치로 넘치거나 unsafe overflow가 start로 clamp되는가 | **통과** — 50×40px item/40×30px cell에서 safe `(0,0)`, unsafe center `(-5,-5)`, unsafe end `(-10,-10)`을 Rust와 표면 표본으로 확인했다. |
| 11 | CSS-invalid declaration이 이전 cascade winner 대신 layout 오류가 되는가 | **통과** — Chrome 156의 `align-self:left` 및 `justify-self:top` computed `auto`와 좁은 parser diagnostic fallback을 대조했다. cascade unit test 1/1이다. |
| 12 | 좁은 CSS-invalid recovery가 유효한 미지원 값이나 다른 property diagnostic까지 숨기는가 | **통과** — property와 invalid keyword를 구체적으로 제한하고 `center`, `safe start`, `justify-items`, 다른 diagnostic category positive-negative unit controls가 있다. |
| 13 | `normal`, `stretch`, baseline, `safe start` 등 sizing/contribution 의미가 위치-only slice로 허용되는가 | **통과** — projection allowlist에서 거부하며 runtime test가 exact property/value와 partial scene 부재를 확인한다. |
| 14 | 기존 C11.1 container alignment 또는 교체 요소 `justify-self:start` fallback이 깨지는가 | **통과** — C11.1 alignment regression 7/7과 Taffy replaced-item fallback mapping test가 통과한다. |
| 15 | invalid self value 하나가 형제 frame 또는 completed scene을 부분 공개하는가 | **통과** — runtime rejection matrix는 `layout=Failed`, exact node/property, `completed=None`을 검사한다. |
| 16 | item owner, root route 또는 draw viewport가 사전 표본과 다르지만 readback만 우연히 일치하는가 | **통과** — Android draw report의 viewport/surface identity와 iOS draw report의 `980×735` viewport를 readback 전에 검증한다. |
| 17 | sample point가 cell 경계, safe/unsafe overflow, viewport clipping을 충분히 건드리는가 | **통과** — 각 플랫폼 7개 sample이 내부/외부 경계와 clipping inside/outside를 포함하며 직접 surface 표본은 Android PixelCopy 7/7, iOS Metal 7/7이다. |
| 18 | OS screenshot이 직접 surface readback으로 오인되거나 simulator가 hardware/physical device 근거로 과장되는가 | **통과** — OS-composited PNG, PixelCopy, pre-compositor Metal readback을 metadata에서 분리했다. Android는 SwiftShader AVD로 기록한다. |
| 19 | fixture adapter와 fixed scene/sample payload가 기본 앱 artifact에 포함되는가 | **통과** — Android default DEX에서 opt-in adapter 클래스명과 fixture scene ID가 없고 iOS default executable에서는 C11.5 argument와 scene ID가 없다. Opt-in positive builds에는 각각 fixture adapter/argument와 표본 ID가 있다. Android의 runtime request/diagnostic guard 문자열은 기본 DEX에 남을 수 있어 payload exclusion과 구분한다. |
| 20 | candidate를 전체 Grid/C11 완료로 부풀리거나 전체 package test 실패를 숨기는가 | **한계 기록** — Chrome/Rust focused와 simulator 검증은 bounded evidence다. broad Cargo는 spinon-layout 423 passed/1 failed, style-to-layout 372/1, style 283/1, runtime 261 passed/2 failed/3 ignored로 green이 아니다. 전체 C11·WPT·실기기·hardware GPU를 주장하지 않는다. 공식 완료는 `72/213`이다. |

## 실패 수정 후 재확인

- C11.1 projection 테스트가 새로 지원하는 `align-self:end`/`justify-self:start`를 계속 reject하도록 남아 있던 것을 발견했다. 그 검사를 `baseline`/`safe start`의 valid-but-unsupported negative controls로 고쳤고 C11.1 alignment regression 7/7을 통과했다.
- Android default APK에 opt-in capture adapter reflection class name이 남아 있었다. 호출을 BuildConfig compile-time gate 안으로 옮긴 뒤 default DEX에서 adapter name/scene IDs 부재, opt-in DEX에서 adapter/sample ID 포함을 다시 확인했다.
- iOS default executable에 사용되지 않는 C11.5 argument exception 문자열이 남아 있었다. 이 인수는 conflict filter의 `--spinon-c*` 조건과 일치하지 않아 해당 예외를 지운 뒤 default executable 부재와 opt-in executable 포함을 다시 검사했다.
- broad package failure 중 이번 candidate 코드가 건드리지 않은 C07 aspect-ratio string comparison, C15 HostDocument error-kind, 다른 Grid profile track-list, PR #1117 dense-flow rejection expectation은 관련 README와 내부 계약에 별도 보존했다. 이를 green baseline으로 취급하지 않는다.
