# C15.1 PR diff 적대 검토 30개 관점

**대상:** [PR #238](https://github.com/ohah/spinon/pull/238)의 GitHub diff, base `92f08e285f28d0ffdccb4a0235b9b83f4962c8ea`, 검토 head `d81797d302fc4017631634cbd35f8dba7fc949b6`<br>
**원격 diff 근거:** `gh pr diff --color never 238`, 68 files · 5,682 lines · SHA-256 `3e813e488b4fdff1ff2624a895fcdb16369f86008d055eb4a97ecb8aa829ec90`<br>
**검토 시점:** PR 생성 뒤 실제 원격 diff를 읽고, 코드·고정 Taffy 0.14.0 소스·fixture·실행 로그를 대조한 시점<br>
**범위:** 코드와 테스트, 내부 계약·상태 문서, 재현 자료·실행 근거. 마지막 항목에서 이 검토 자료와 PR 표기 경계도 확인한다.

| # | 독립 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 잠금 없는 Parley 버전 변경이 line breaking 결과를 바꾼다 | `spinon-text-layout/Cargo.toml`은 Parley `=0.12.0`으로 고정하고 `Cargo.lock`에 transitive dependency를 잠근다. `cargo test --locked`가 성공했다. 통과. |
| 2 | system font가 fixture 결과에 끼어들어 다른 OS에서 결과가 바뀐다 | `CollectionOptions.system_fonts`가 `false`이고 테스트는 Ahem/Roboto를 명시 등록한다. 없는 family는 `MissingFontFamily`로 닫힌다. 통과. |
| 3 | 등록 요청 family가 없는데 이미 존재하는 family ID만 보고 성공 처리한다 | `register_font`는 이번 `register_fonts` 호출의 반환 face 목록에 기대 family ID가 있어야 성공한다. 같은 family ID가 있던 registry에 다른 family의 bytes를 등록하는 회귀 입력도 실패한다. 통과. |
| 4 | malformed font 실패가 live registry나 revision을 부분 변경한다 | candidate collection에서 등록을 시험한 뒤 기대 family를 확인할 때만 교체한다. malformed bytes, 다른 family bytes, 누락 family 후 상태가 그대로임을 전용 test가 확인한다. 통과. |
| 5 | revision overflow가 collection 교체 뒤 보고되어 일관성이 깨진다 | 다음 revision을 collection clone/등록보다 먼저 `checked_add`한다. `u64::MAX` negative control이 실패하고 font family가 추가되지 않는다. 통과. |
| 6 | 공백 family, 0·음수·비유한 font size/line-height가 upstream에 전달된다 | `ParagraphStyle::validate`가 family 공백과 각 유효 범위를 거부하며 별도 style test 5개 입력을 확인한다. 통과. |
| 7 | NaN, 무한대, 음수 available width가 Parley/Taffy 캐시로 넘어간다 | text engine은 finite·non-negative를 사전 검증한다. 음수·NaN·양의 무한대 negative control이 `InvalidWidth`를 반환한다. 통과. |
| 8 | `-0.0`이 `+0.0`과 다른 Taffy cache key를 만든다 | text engine과 Taffy adapter 모두 zero를 양수 0으로 정규화하며 snapshot width bit를 확인한다. Taffy Block의 0px 폭에서도 실제 advance가 box 밖에 남는다. 통과. |
| 9 | supplementary code point가 UTF-16 두 code unit이 아닌 한 단위로 매핑된다 | normalizer는 각 Rust scalar의 `len_utf16()`로 source 범위를 만들고 source offsets와 normalized UTF-8 offsets를 분리한다. 별도 test가 `😀`를 UTF-16 `0..2`, UTF-8 `0..4`로 확인한다. Unicode glyph shaping 자체는 C15.3 범위다. 통과. |
| 10 | HostDocument의 unpaired surrogate가 손실 변환이나 replacement character로 성공한다 | HostDocument adapter는 `String::from_utf16` 실패를 해당 text node의 `InvalidTextEncoding`으로 반환한다. unpaired high surrogate negative control이 확인됐다. 통과. |
| 11 | 여러 연속 공백을 모두 advance에 포함하거나 source owner를 버린다 | normalizer는 collapsed space 한 byte range에 원본 code point 범위들을 보존한다. Ahem `X   X` 결과와 text source range, 글자 advance를 함께 확인한다. 통과. |
| 12 | 선행·후행 collapse 공백만 있는 paragraph에 가짜 line box를 만든다 | normalized text가 비면 빈 immutable snapshot과 zero-length source ranges를 반환하고 line/height는 0이다. 공백-only test가 이를 확인한다. 통과. |
| 13 | `pre-line`에서 CRLF를 두 줄바꿈으로 처리하거나 source의 한쪽을 잃는다 | CRLF를 normalized newline 하나로 만들고 두 원본 code unit을 모두 같은 결과 범위에 연결한다. Ahem line count/height와 mapping test가 통과했다. |
| 14 | 일반 word breaking이 unbreakable 단어를 부모 폭에 맞춰 clamp한다 | width 15px `XX` 및 0px `X`에서 paragraph box 폭과 별개로 text advance·line fragment를 보존한다. Chrome DOM의 동일 경계 수치와 Rust test를 대조했다. 통과. |
| 15 | `word-break: break-all`을 무시하고 normal처럼 단어 전체를 overflow한다 | Ahem `XXXXX`, width 20px의 `20/20/10px` line advance와 세 line을 fixture test로 확인한다. 통과. |
| 16 | `nowrap` 입력을 일반 wrapping으로 처리한다 | width 30px에서도 `XX XX`를 한 줄 50px로 유지하며 auto-height 10px를 확인한다. 통과. |
| 17 | font size × 문자 수를 실제 shaping 대신 사용한다 | Roboto variable `iiii`/`WWWW`의 서로 다른 advance와 glyph ID·origin·advance를 검사한다. fixture의 font hash를 기록해 임의 문자 폭 대조군을 배제한다. 통과. |
| 18 | line/font/glyph/cluster/fragment의 NaN 또는 overflow가 성공 snapshot에 들어간다 | line metrics, run font metrics, glyph 좌표·advance, cluster offset·advance와 fragment/union 경계에 finite 검증이 있다. 잘못된 결과는 `InvalidGlyphLayout`로 닫힌다. 통과. |
| 19 | hanging whitespace를 눈에 보이는 line fragment 폭으로 잘못 센다 | line advance에서 `hanging_advance`를 제외한 visible advance를 fragment geometry 상한으로 쓴다. 줄 끝 공백 관찰 및 Rust fixture 판정을 대조했다. 통과. |
| 20 | 같은 inline span의 서로 다른 줄 fragment를 하나의 큰 union rect로 합친다 | fragment key가 `(NodeId, line_index)`이며 한 줄 내부 cluster만 union한다. span 2줄 test에서 2개 `20×10px` rect가 유지된다. 통과. |
| 21 | Parley가 예상 밖 InlineBox item을 무시하고 일부 glyph만 반환한다 | `PositionedLayoutItem::InlineBox`는 `UnexpectedInlineBox(paragraph)`로 전체 layout을 실패시킨다. 자동 제거·부분 결과 경로가 없다. 통과. |
| 22 | HostDocument traversal에서 direct text와 span text 순서가 달라진다 | 역순 stack push 후 source order대로 pop하며 text source, inline ancestor, Taffy box child 분리를 fixture test에서 대조한다. 통과. |
| 23 | paragraph가 누락·비-element·display:none·비-Block이거나 보이는 box child가 있다 | HostDocument 변환과 direct `LayoutInput` 검증이 각각 owner, hidden subtree, Block-only, visible child 조건을 확인한다. 알 수 없는 owner와 잘못된 owner ID negative control이 있다. 통과. |
| 24 | 일반 `LayoutStyle::default()`의 Flex를 plain inline span으로 오인하거나 독립 Inline을 Taffy로 보낸다 | Host adapter는 `LayoutDisplay::Inline` 명시값과 나머지 기본 필드를 요구한다. `LayoutDisplay::Inline`인 독립 layout box는 `validate_style`와 Taffy 변환에서 거부한다. Flex regression test가 통과했다. |
| 25 | visible nested/decorated/non-span child를 조용히 평탄화한다 | visible child는 HTML span만 받고 한 단계만 허용한다. border/padding/margin 등 non-default style은 거부하고, nested·visible non-span·Flex display 입력의 negative control을 확인한다. 통과. |
| 26 | 숨긴 span 자손을 수집하거나 style 누락을 조용히 허용한다 | `display:none` subtree는 source/layout collection에서 제외하지만 각 element style은 요구한다. hidden span text 비노출과 missing hidden style 오류 test가 통과했다. |
| 27 | paragraph profile 밖의 visible text node가 기존 HostDocument API에서 성공하거나 숨김 조상 아래 text가 섞인다 | 일반 adapter의 visible-text 거부를 유지하고 새 profile만 처리한 text node ID를 제외한다. 숨김 조상과 `display:none` 자식은 분리한다. malformed UTF-16은 구체 node 오류다. 통과. |
| 28 | measure 오류가 Taffy의 임시 0 크기로 감춰져 부분 frame map이 반환된다 | callback 오류를 `calc_error`에 기록하고 tree 계산 결과를 최종 반환 전에 전파한다. engine 누락·indefinite width·layout mismatch는 typed `LayoutError`다. 통과. |
| 29 | 첫 measure 폭과 최종 content 폭이 다르거나 계산별 revision이 섞인다 | Taffy cache는 normalized width bit별 snapshot을 분리하고 최종 content width와 exact match를 요구한다. `LayoutTextSnapshot`은 input revision과 font registry revision을 보존하고 output/input revision 일치를 test한다. 통과. |
| 30 | 변경 설명이 구현 범위보다 넓거나 증거 파일이 제품 지원처럼 보인다 | `STATUS.md`와 저장소 roadmap HTML은 C15 parent를 미완료로 유지한다. Tailnet preview sync는 merge 뒤 남아 있다. PR 이미지는 Chrome DOM oracle라고 명시하며, runtime·Simulator 앱·glyph ink·border 밖 pixel을 통과했다고 적지 않았다. 통과. |

## 결론과 범위

코드 리뷰에서 수정해야 할 미해결 결함은 찾지 못했다. 구현 근거에 남은 0px 경계, 기존 Flex 기본값으로 span을 평탄화하던 오류, transactional font registration은 코드와 별도 test에서 다시 확인했다. C15 runtime projection, renderer, Android/iOS Simulator 앱, 실제 glyph ink와 부모 경계 밖 pixel은 여전히 검증되지 않았다.

전체 Rspress build는 이 PR diff 시작 시점에 471개 문서의 dead-link 보고로 실패했다. 새 C15 contract/evidence source 자체와 추가한 C15 링크는 해당 error list에 없었다. 이 검토는 전체 문서 link debt를 C15 변경으로 취급하지 않는다.

검토 파일, 3개 index link와 마지막 문서 빌드 출력 hash 갱신을 push한 뒤 원격 diff를 다시 읽어 제품 코드·fixture·실행 결과 변경이 없음을 확인한다. 따라서 30개 행의 판정은 `d81797d3`까지의 구현 diff에 적용되고, 최종 추가 diff는 검토 자료와 문서/evidence 연결만 포함한다.
