# C11.4 anonymous Grid text computed `word-break` PR diff 30 관점 검토

## 검토 범위

PR [#942](https://github.com/ohah/spinon/pull/942)의 구현 commit `977dbee2aaeb81d23d48dcd23c64cb8484a58f34`, base `991d96f3e3ae71fa1d848f4d2c0a989e26039a06`에서 시작한 69-file 변경을 검토했다. 이 기록은 구현/runtime 실패 관점 20개와 구별해 API 변경, fixture 연결, 플랫폼별 adapter, image·merge 경계를 대조한다. 본 검토 문서 추가는 documentation-only다.

## 독립 검토 항목

1. **변경 범위 귀속** — 변경 69개 경로가 C11.4 anonymous Grid text word-break, 기존 `ParagraphStyle` initializer 호환, 내부 계약과 해당 실행 증거로 제한된다. 별도 untracked 작업 자료는 PR에 포함하지 않았다.
2. **기존 기본 스타일 보존** — `ParagraphStyle::default()`에서 `locale=None`, `word_break=Normal`을 유지하고 기존 직접 initializer를 갱신했다. default assertion과 Rust 빌드가 통과했다.
3. **enum 확장 호환** — `Normal`·`BreakAll` 분기를 보존하고 `KeepAll`을 명시적으로 추가했다. text-layout 및 runtime 테스트에서 세 variant를 각각 확인했다.
4. **locale 태그 문법 거부** — trim 전후 공백, 빈 문자열, 파싱 불가 태그를 `ParagraphStyle::validate()`가 거부한다. `missing_locale_keeps_default_and_invalid_locale_is_rejected`를 통과했다.
5. **유효 locale 전달** — `ko`, `ko-KR`, `en-US`를 받아 pinned Parley `Language`로 변환하고 `StyleProperty::Locale`에 전달한다. Rust 테스트와 platform trace가 값을 확인한다.
6. **locale 원인 오해** — pinned Parley 0.12.0 strict segmenter가 한국어 locale을 분할 기준으로 쓰지 않는 것을 소스에서 대조했다. locale 있는/없는 Noto `normal` geometry control을 추가하고 계획·사전비교·계약 문구를 고쳤다.
7. **Parley word-break mapping** — `Normal`, `BreakAll`, `KeepAll`이 각각 pinned Parley variant에 직접 연결되며 별도 glyph/line-width 보정은 없다. 3개 text-layout test가 통과했다.
8. **fixture 외 기본값 누출** — locale은 opt-in scene에만 설정되고 나머지 anonymous-text 경로는 `None`이다. 기존 paragraph default assertion과 focused runtime 회귀가 통과했다.
9. **정확한 HTML language owner** — Korean `lang=ko`는 direct Grid style owner attribute에서 읽어 paragraph로 전달한다. 부모 document의 `lang`을 새 동작 근거로 추론하지 않는다.
10. **XML language precedence** — owner의 `xml:lang`은 profile 밖 입력으로 거부된다. runtime negative test에서 확인했다.
11. **누락·충돌 language 차단** — owner `lang` 누락 또는 scene과 다른 값은 computed style/layout publication 전에 실패한다. `anonymous_grid_word_break_rejects_missing_or_xml_language_and_mismatched_style`를 통과했다.
12. **computed property strictness** — `word-break`의 scene별 computed serialization을 exact 비교하고 missing/unsupported value를 허용하지 않는다. runtime scene test가 12개 전부 computed value를 확인했다.
13. **scene catalog 중복** — FFI generator, runtime catalog, Android/iOS adapter, expected JSON의 12개 ID가 고정 allowlist로 유일하다. FFI와 runtime uniqueness 검사 및 fixture-module boundary 26/26이 통과했다.
14. **deprecated `break-word`** — allowlist에 없고 FFI generator가 source를 만들지 않으며 unknown anonymous word-break scene은 runtime에서 거부한다. FFI negative assertion과 runtime preparation guard를 대조했다.
15. **직계 text topology** — 각 scene은 Grid root 하나와 direct Text node 하나만 생성한다. FFI generator test가 element·text node 수를 확인한다.
16. **고정 글꼴 identity** — Ahem 및 Noto Sans KR 2.004 고정 bytes를 사용하며 system font fallback 없이 Noto scene에서만 font를 등록한다. runtime paragraph trace가 family와 revision을 확인한다.
17. **Ahem `normal` min-content** — `XXXX`는 40px, `XX XX`는 20px와 두 줄이다. 독립 Ahem 계산, Rust test, Android/iOS 행렬이 일치한다.
18. **Ahem `break-all` min-content** — 두 입력 모두 10px 및 네 줄이며, space 포함 `XX XX`에서도 max-content는 50px로 유지된다. 세 계층의 line/track 값을 대조했다.
19. **Korean `normal` intrinsic** — Noto advance 9.2px × 4 glyph의 min-content 9.2px, 4행/40px을 고정한다. locale 없는 geometry 대조와 Android/iOS 12-scene trace가 통과했다.
20. **Korean `keep-all` intrinsic** — min-content가 36.8px·1행으로 늘고 `normal`과 max-content 36.8px은 같다. `KeepAll` mapping, independent formula, Rust 및 양 simulator를 대조했다.
21. **max-content invariant** — Ahem 두 mode에서 40px/50px, Korean 두 mode에서 36.8px이 예상과 일치한다. 각 max-content scene은 한 줄을 확인한다.
22. **automatic row sizing** — min-content 줄 수에 따른 row height가 auto Rows track과 root/item frame으로 전파된다. capture가 line count, row track, item frame을 검사했다.
23. **정확한 Grid track 입력** — runtime validator는 scene의 `min-content` 또는 `max-content` column과 `auto` row를 요구한다. fixed-length column control은 runtime test에서 거부된다.
24. **문서 revision 연결** — layout trace의 source revision은 같은 HostDocument generation/document revision과 일치하고 finite revision fields를 가진다. 각 simulator log에 run ID 및 revision을 보존했다.
25. **trace field 계약** — trace report는 `fontFamily`, `locale`, `wordBreak`, line metrics를 기록하고 native adapter가 전부 expected value와 비교한다. 기존 제품 API/FFI ABI를 새로 추가하지 않았다.
26. **Android fixture gate** — Java scene adapter는 fixture-test BuildConfig 아래에서만 값을 제공한다. Android API 37 ARM64 AVD에서 고정 12 scenes가 실행됐다.
27. **iOS fixture gate** — Swift adapter는 대응 simulator compile define 아래에서만 값을 제공한다. iPhone 17 Pro/iOS 26.2 Simulator에서 같은 12 scenes가 실행됐다.
28. **capture 완결성·artifact identity** — capture runner는 summary, unique trace chunks, JSON, intrinsic values, line advances, 최종 tracks와 설치 APK/executable SHA-256을 검증하고 종료 시 앱을 force-stop/terminate한다. 양 플랫폼 12/12 및 build/install hash equality를 기록했다.
29. **PR inline image load** — Agent Browser로 실제 PR #942 화면을 열어 markdown 본문 이미지를 확인했다. Chrome 1280×633, Android 1080×2400, iOS 1206×2622 이미지 모두 `complete=true` 및 양수 `naturalWidth`였다. Simulator 이미지는 검은 renderer 때문에 glyph paint로 주장하지 않았다. 검증 뒤 이 세션을 닫고 `agent-browser` process와 dashboard listener가 없음을 확인했다.
30. **공식 상태·merge 정책** — C11.4/C11 미완료와 공식 `72/213`을 유지하며 이 제한 slice로 체크를 승격하지 않는다. main linear-history 정책은 Rebase and merge만 허용하고 required status checks는 설정되지 않았으며 PR check rollup도 비어 있다. 이는 CI 성공이 아니며 병합 뒤 status/roadmap에 제한 결과만 추가한다.

## 판정

구현 diff 30개 관점에서 미해결 동작 결함을 찾지 못했다. 발견한 문서 인과성 과장은 pinned Parley source, no-locale geometry control, 내부 계약과 사전 비교 문서로 바로잡았다. 남는 한계는 opt-in 12-scene 범위, Simulator 전용 실행, glyph surface 미검증, 일반 CSS admission/WPT 및 전체 C11.4 미완료이며 PR 본문과 runtime evidence에 명시했다.
