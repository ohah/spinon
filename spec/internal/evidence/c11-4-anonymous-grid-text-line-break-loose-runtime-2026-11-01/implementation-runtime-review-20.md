# C11.4 computed `line-break: loose` 구현/runtime 실패 관점 검토 20개

## 범위와 판정

계획 PR의 20개 검토와 분리해 `LineBreakMode::Loose`, Parley 연결, anonymous Grid scene allowlist, FFI generator, Android/iOS adapter 및 실 simulator trace를 다시 확인했다. 결과는 고정 opt-in fixture slice에 한정한다. 일반 CSS 지원이나 glyph paint 검토가 아니다.

| # | 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | `Loose`가 기존 default를 바꾸는가 | `LineBreakMode`의 `Strict` default가 그대로다. 기존 Rust text-layout line-break regression 통과. |
| 2 | enum-to-Parley 변환에서 값이 누락되거나 바뀌는가 | `Loose`를 Parley `LineBreak::Loose`로 직접 전달하며 `Normal`·`Strict` mapping은 유지한다. |
| 3 | locale에 따른 prefix tailoring이 적용되는가 | 고정 font에서 `ja+loose` min-content 10px, `en+loose` 20px를 단위 테스트와 두 simulator trace에서 확인했다. |
| 4 | `normal`이 loose 전용 break 기회를 잘못 받는가 | `ja+normal`의 `文＄文` control은 20px이며 Android/iOS 모두 통과했다. |
| 5 | 기존 Japanese `normal`/`strict` 동작이 변했는가 | 기존 `文〜文` 다섯 장면을 신규 네 장면과 함께 양쪽 simulator에서 다시 실행해 5/5 회귀 통과했다. |
| 6 | max-content가 soft break에서 불필요하게 줄어드는가 | 일본어 loose max-content는 30px column·10px row로 Rust 및 양 simulator에서 일치했다. |
| 7 | min-content block size가 줄 수와 분리되는가 | 세 10px 줄은 30px, 두 줄은 20px다. item/frame/auto row 및 text layout height를 trace에서 대조했다. |
| 8 | source text의 U+FF04가 유사한 ASCII 기호로 바뀌는가 | FFI, runtime, app adapters와 expected JSON에서 정확히 `文＄文`을 전달하고 source trace도 같은 문자열이다. |
| 9 | 고정 font가 달라 기대 폭이 무효가 되는가 | Noto Sans KR fixture SHA-256 `9e1d729e…b86f76`과 1000/1000 CJK advance를 고정했다. |
| 10 | `lang`이 부모에서 잘못 상속되거나 `xml:lang`이 대체하는가 | exact Grid owner의 direct `lang`만 허용한다. 기존 owner-language negative checks와 새 locale mismatch test가 통과했다. |
| 11 | 허용되지 않은 line-break keyword가 새 mode로 흘러드는가 | 네 새 scene의 computed keyword만 `loose`; 잘못된 mode와 scene mismatch는 runtime rejection test로 검사한다. |
| 12 | 다른 text source나 추가 child가 allowlist를 우회하는가 | FFI source generator는 scene별 고정 text와 direct text node 하나만 생성한다. exact input generator test 통과. |
| 13 | unknown scene이 부분 layout/frame을 발행하는가 | 기존 fail-closed unknown scene 검사와 scene allowlist를 유지하고 관련 runtime test가 통과했다. |
| 14 | stale document/style revision이 새 text layout에 섞이는가 | 기존 line-break runtime의 stale revision rejection을 다시 확인했다. 양 simulator trace는 현재 HostDocument generation/revision에 연결됐다. |
| 15 | computed min/max intrinsic과 선택된 Grid track이 다르게 연결되는가 | 9개 scene 모두 source intrinsic, 최종 Columns/Rows track, item frame과 independently derived matrix가 일치했다. |
| 16 | Android adapter가 loose scene을 `Strict`로 전달하는가 | scene id에서 `Loose`를 명시적으로 판별하고, 로그의 `paragraphStyle.lineBreak`가 `Loose`임을 검사했다. |
| 17 | iOS adapter가 Android와 다른 keyword/geometry를 만드는가 | Swift adapter의 네 scene 기대값과 line-break mapping을 Java와 대조했다. Simulator 9/9 통과. |
| 18 | 줄 advance의 source order가 잘못 기록되는가 | 첫 Android 실행에서 English/control의 `[20,10]` 예상 오류를 잡았다. 실패 로그를 보존하고 `[10,20]`으로 고친 뒤 두 simulator 행렬을 다시 통과시켰다. |
| 19 | 시뮬레이터가 다른/stale app artifact를 실행했는가 | Android APK와 설치 APK, iOS executable과 설치 executable SHA-256이 각각 일치한다. AVD/UDID/runtime/model을 고정했다. |
| 20 | iOS startup supersede나 screenshot을 기능 성공으로 오해하는가 | 첫 장면의 `status=-12` 시도는 별도 log와 attempts 기록에 남기고 1회 재실행만 허용했다. Screenshot은 route/status만 보여주며 glyph/compositor 증거로 세지 않는다. |

## 결론

검토 중 발견한 앱 기대 line-advance 순서 결함은 수정 후 관련 Android/iOS build 및 전체 9-scene 실행을 재수행했다. 현재 남은 범위는 계획에 적힌 일반 CSS admission, arbitrary font/locale/text, WPT, glyph painting, hardware renderer, iOS compositor 이후 pixel과 전체 C11.4/C11이다. 공식 `72/213`은 올리지 않는다.
