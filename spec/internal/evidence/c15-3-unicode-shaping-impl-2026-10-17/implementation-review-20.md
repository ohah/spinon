# C15.3 pure Rust shaping 구현 실패 경로 검토 20개

검토 대상은 이 PR의 현재 Rust 구현과 고정 테스트 입력이다. 각 항목은 서로 다른 실패 축이다. 이 기록은 runtime painter·플랫폼 동작을 대신하지 않는다.

| # | 공격 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | available width가 음수, NaN, 무한대이거나 음의 0인 경우 | 기존 boundary test가 invalid width를 거부하고 `-0.0`을 정규화한다. 통과. |
| 2 | 공백만 있는 paragraph가 empty fast path에서 shaping metadata를 버리는 경우 | RTL/Normal/None 설정을 빈 snapshot에서 각각 확인하도록 regression을 추가했다. 통과. |
| 3 | 공백 collapse와 newline 처리 뒤 source UTF-16이 잃어버린 문자를 잘못 가리키는 경우 | 기존 whitespace/CRLF 및 supplementary-scalar source mapping 회귀가 통과한다. |
| 4 | supplementary scalar를 UTF-8 byte와 UTF-16 code unit으로 같은 길이라고 처리하는 경우 | 기존 supplementary scalar test와 emoji UTF-16 `0..5` assertion을 대조했다. 통과. |
| 5 | grapheme의 base와 combining mark가 다른 DOM node에서 오면 owner가 한쪽으로 합쳐지는 경우 | `x`와 U+0323을 두 source로 나눈 시험에서 두 node/range가 모두 남는다. 통과. |
| 6 | 한 grapheme를 여러 glyph가 만들 때 cluster 범위가 glyph마다 잘리는 경우 | 각 glyph에 동일한 유효 cluster range와 그 range에 기여한 source를 붙인다. combining fixture 통과. |
| 7 | ligature start/continuation가 source 문자 범위를 줄이는 경우 | `office`의 `ffi` glyph가 UTF-8·UTF-16 `1..4`를 소유하는지 확인한다. 통과. |
| 8 | ligature를 끄더라도 HarfBuzz optional feature가 남는 경우 | Parley CSS feature syntax로 `liga/clig/dlig/hlig/calt`를 끄며 width가 `49.5625`에서 `51.25`로 바뀐다. 통과. |
| 9 | Bidi run range 순서와 visual glyph 순서를 같은 것으로 가정하는 경우 | Arabic glyph clusters의 descending source order를 실제 positioned glyph와 맞춰 확인한다. 통과. |
| 10 | RTL 문단에서 European digit glyph도 RTL로 뒤집히는 경우 | `السَّلَامُ 123`의 digit run은 LTR이고 x 좌표가 증가한다. 통과. |
| 11 | Auto/LTR/RTL 방향 branch 중 일부가 실제 배치에 적용되지 않는 경우 | Arabic first-strong 입력에서 Auto와 RTL 시작 x가 일치하고 LTR은 x=0이다. 통과. |
| 12 | `line-break:strict`가 closing punctuation을 다음 줄 첫 위치에 놓는 경우 | 폭 40의 고정 CJK fixture에서 줄 묶음과 `、` 소유 줄을 확인한다. 통과. |
| 13 | `line-break:normal` branch가 빈 입력 metadata에만 기록되고 shaping에는 전달되지 않는 경우 | nonempty Arabic paragraph에 Normal을 지정해 실제 layout을 실행하고 snapshot mode도 확인한다. 통과. |
| 14 | 글리프가 없을 때 다른 system font나 `.notdef`가 화면 결과인 것처럼 반환되는 경우 | system-font discovery를 끈 Ahem registry에서 otter 입력이 `MissingGlyphCoverage`로 끝난다. 통과. |
| 15 | glyph가 cluster source map 외부를 가리키거나 UTF-8 경계가 아닌 byte range를 갖는 경우 | `valid_text_range`, cluster lookup 및 source 비어 있음 오류 분기를 확인했다. malformed Parley stream 주입 hook은 없어 해당 오류 branch는 직접 생성하지 않았다. |
| 16 | glyph x/y/advance가 NaN/무한대인 경우 | 모든 positioned glyph에서 유한값을 검사한 뒤 snapshot을 구성한다. upstream 값 변조용 test seam이 없어 branch 자체의 주입 시험은 남아 있다. |
| 17 | Parley visual cluster와 positioned glyph 배열의 수 또는 ID가 다를 때 조용히 짝이 어긋나는 경우 | 길이와 각 순서별 ID를 대조해 mismatch를 거부한다. 별도 synthetic mismatch seam은 없다. |
| 18 | RTL ligature continuation가 line/run 경계에서 시작해 잘못 묶이는 경우 | continuation-only 시작과 비연속 atom 범위를 거부하는 guards를 검토했다. 고정 Arabic/ligature fixture는 정상 경로를 통과한다. |
| 19 | fixture font가 기본 product artifact로 들어가 설치 크기나 runtime font registry를 바꾸는 경우 | 네 pinned font 전체 byte sequence가 `.rlib`에는 없고 positive-control test binary에는 있다. 통과. |
| 20 | 내부 shaping 통과를 runtime paint 또는 C15 완료로 과장하는 경우 | spec/STATUS는 C15 parent 미완료로 유지하고 runtime text admission, glyph paint, consumer sizing, overflow/clip/hit 및 Android/iOS ink를 제외한다고 명시했다. |

## 발견 후 수정

최초 전체 workspace Clippy는 Chrome oracle f32 literal 두 개의 과도한 정밀도를 지적했다. 각각 `96.470_17`, `88.139_92`로 표현을 정리하고 최종 workspace Clippy를 다시 실행해 통과시켰다. 리뷰 중 Auto/LTR/Normal nonempty 경로의 테스트 공백도 발견해 회귀를 추가했다. 최종 테스트와 lint 로그는 같은 디렉터리의 README에 연결했다.
