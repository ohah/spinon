# C11.3.3 WPT 하위 사례 inventory 재분류 · 2026-10-30

## 범위

이 변경은 2026-10-14 고정 Chrome 154 WPT 결과를 다시 실행하지 않고, 이미 병합된 C11.3.3 제한 fixture 결과와 WPT 하위 사례의 연결을 정정한다. 원본 Chrome report는 그대로 보존하며, 새 `classified-run06.json`은 pinned source/revision/hash를 다시 확인한 뒤 현재 inventory 규칙으로 재분류한 결과다. Chrome WPT oracle 실행은 4 test·73 subtest PASS, skip 0, failure 0이다. 이는 Spinon에서 실행한 WPT 결과가 아니다.

기존 precomparison, `classified-run01.json`, runtime matrix와 pixel evidence는 당시 inventory/reference hash를 보존한다. Regression test는 이 역사적 연결을 현재 inventory hash와 혼동하지 않고, `classified-run06.json`을 현재 inventory·classifier hash에 별도로 묶어 확인한다.

고정 WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`이며 `grid-placement-unknown-named-area-001.html` source SHA-256은 `7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4`다. 재분류한 보고서는 [classified-run06.json](./classified-run06.json)이다. Classifier는 네 scene ID가 실제 derived fixture HTML에 존재하는지 확인하고 그 source hash도 report에 기록한다.

## 네 하위 사례의 고정 연결

WPT 원문의 네 `.grid` subtest는 positive trailing implicit named-line/line-span fixture로 각각 연결한다. WPT source order와 `data-offset-*` 기대 geometry를 근거로 하며, classifier는 이름 집합을 고정해 새·누락된 subtest와 중복 mapping을 거부한다.

| WPT subtest | 원문 case | 기대 item rect `(x,y,w,h)` CSS px | 파생 fixture scene |
| --- | --- | ---: | --- |
| `.grid 1` | `bothNamedGridLineRow` | `(0,35,30,5)` | `unknown-both-named-row` |
| `.grid 2` | `bothNamedGridLineColumn` | `(85,0,45,10)` | `unknown-column-named-span` |
| `.grid 3` | `namedGridLineSpanRow` | `(0,35,30,25)` | `unknown-row-named-span` |
| `.grid 4` | `namedGridLineSpanColumn` | `(85,0,15,10)` | `unknown-column-span-one` |

파생 장면의 제한 구현·Android API 37 AVD·iOS Simulator 및 기존 단일 Android 실기기 근거는 [PR #393 follow-up evidence](../c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/README.md)에 있다. 이 재분류는 해당 고정 장면을 WPT 원문 실행으로 승격하지 않는다.

## 미지원 범위와 공식 상태

`grid-template-areas` 안에서 실제 area가 없는 `grid-area` bare ident를 처리하는 별도 inventory case `unknown-area-implicit-tracks`는 계속 기대 거부다. WPT 파일의 네 subtest는 `grid-row`/`grid-column`의 없는 named line을 양의 trailing 방향으로 생성하는 제한 profile과 구분한다.

이 mapping은 fixed-px auto tracks, 양의 trailing implicit line/span, 특정 opt-in fixture scene에 한정한다. 일반 CSS admission, negative/leading/general implicit placement, 임의 track sizing 또는 전체 WPT 적합성을 입증하지 않는다. C11.3.3은 candidate, C11은 미완료, 공식 완료 집계는 `72/213` 그대로다.

## 재현과 hash

Focused classifier test:

```sh
node --test tools/css-reference/c11-3-3-grid-template-areas-wpt-classification.test.mjs
```

재분류 명령은 고정 WPT checkout의 source hash와 원래 Chrome report를 확인한 뒤 실행했다. Classifier test 8/8은 매핑, report 순서 독립성, 새·누락된 subtest 거부, 중복 mapping 거부, malformed/duplicate 입력 거부와 기존 전체-test fallback 분류를 확인한다. 기존 C11.3.3 consumer suite 15/15는 역사적 산출물 연결과 최신 subtest 매핑을 각각 확인한다.

| Artifact | SHA-256 |
| --- | --- |
| 원본 WPT Chrome report | `368c2244afb1e95f1353ed2a3d311d405de7e03ce8b445fe39134ab4fde94c83` |
| derived fixture HTML | `222b2938c432b6f433a127dac21d0183bd98021f6b1c40b8410050656c11367d` |
| inventory | `0d44c89512e5b049970c0da806a99ecc9d0c6601c29d1e0c82c73c39975c77cc` |
| 재분류 report | `2d839344efbb2220bba571dc14362c9b521017aff87ebf76171e6aa5b1e50f13` |
| classifier | `dbd099e71c6c2fd8cce4e8ff4e001f9e492f006d8ebcef7bc4223ea072a8ad87` |
| classifier helper | `e27dc0d22d881bb32113e41f9e62babf285f039c5c846a4c0c03ffec64350f08` |
| focused classifier test | `8c55a5f6e872b0b9766de06d82c3d7df5bfbcf03c56e454d07bd70aa56f4f715` |
| C11.3.3 consumer test | `7585ea7512364fcbf0a5caaa87058806882617bbd261b21887b7da184b2c06c9` |

Agent Browser로 pinned WPT source를 확인했고 확인 후 browser session을 닫았다. 이 문서는 새로운 browser rendering·Spinon WPT 실행·Android/iOS 재실행 근거를 주장하지 않는다.
