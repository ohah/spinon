# C11.4 anonymous Grid text `overflow-wrap` 사전 비교

이 사전 비교는 기존 C11.4 opt-in anonymous direct-text Grid 경로의 computed `overflow-wrap`를 다룬다. [고정 HTML 입력](../../../../tests/fixtures/css/c11/grid-c11-4-anonymous-text-overflow-wrap-precomparison.html), [기계 판독 Chrome 관찰값](./chrome156-observations.json), [독립 기대값](./expected.json), [Agent Browser 화면](./chrome156-overflow-wrap-grid.png)은 하나의 9-scene 입력에 대응한다.

## 기준과 기대값

[CSS Text Level 3, 2026-08-14 Candidate Recommendation Draft §5.4](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#overflow-wrap-property)는 `overflow-wrap:anywhere`가 추가하는 soft-wrap opportunity를 min-content 계산에 포함하고, `break-word`의 emergency wrapping opportunity는 min-content 계산에서 제외한다. CSS Sizing 3 §5.1과 CSS Grid 1 2025-03-26 CRD §11의 intrinsic contribution/track sizing과 함께 적용한다.

고정 Ahem font에서 `X` advance는 1000/1000 em이므로 10px font의 각 glyph는 10px이다. `XXXX`는 40px max-content다. `normal`과 `break-word`의 min-content는 40px이고, `anywhere`는 한 glyph 폭인 10px이다. min-content Grid의 auto row는 normal/break-word에서 10px, anywhere에서 네 줄 40px이다. `break-word`는 고정 10px track의 actual wrap control에서도 네 줄이 되지만 그 emergency break를 min-content 기여에 되돌려 쓰지 않는다.

Chrome 156 Headless Agent Browser는 macOS ARM64, DPR 1, 1280×633 viewport에서 Ahem font loaded를 확인했다. CSSOM tracks/rows와 text-range line tops는 [관찰 JSON](./chrome156-observations.json)에 기록했다. line tops는 actual line layout control이며 track intrinsic size의 대용값으로 쓰지 않는다. 독립 계산과 아홉 장면의 기대값은 [expected JSON](./expected.json)에 있다.

## SHA-256

| 입력 | SHA-256 |
| --- | --- |
| Ahem font | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| HTML fixture | `013297e8d5e4952b2bc0f7bb02eba53c12009dc638dac204e6d01e79037e00a9` |
| Chrome observation JSON | `8faf4a7906e3232c8bfccf15967467531654b4cea4a09bb16bbe62d01ef77ef5` |
| Screenshot | `75c14c1c375e688092bb327e9262a813a08dbed5768583542a22011f484c945e` |

Chrome is an oracle cross-observation, not Spinon runtime, WPT conformance, mobile rendering, glyph paint, or public CSS support evidence.
