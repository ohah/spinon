# C15.3 고정 Unicode fixture 글꼴

이 글꼴 파일은 C15.3 Chrome oracle와 이후 opt-in fixture 테스트의 입력이다. 기본 runtime font registry에는 포함하지 않는다. 브라우저 oracle는 명시적인 @font-face URL로만 글꼴을 불러오며 로컬 시스템 글꼴에 의존하지 않는다.

| Fixture 파일 | 원본 저장소와 고정 commit | 원본 버전과 fixture 용도 | SHA-256 |
| --- | --- | --- | --- |
| NotoSansKR-VF.ttf | [notofonts/noto-cjk의 원본 파일](https://github.com/notofonts/noto-cjk/blob/f8d157532fbfaeda587e826d4cd5b21a49186f7c/Sans/Variable/TTF/Subset/NotoSansKR-VF.ttf), commit f8d157532fbfaeda587e826d4cd5b21a49186f7c | Noto Sans KR 2.004; Hangul과 지정 CJK 줄바꿈 문자, weight 축 100–900 중 400 | 9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76 |
| NotoSansArabic-VF.ttf | [google/fonts의 원본 파일](https://github.com/google/fonts/blob/47d54ca1ebb14fd864695f1d39dc734f39017c33/ofl/notosansarabic/NotoSansArabic%5Bwdth%2Cwght%5D.ttf), commit 47d54ca1ebb14fd864695f1d39dc734f39017c33 | Noto Sans Arabic 2.012; Arabic 연결·발음기호와 European digits, weight 400·width 100 | 63111b5b2e074dd48cc67692e0a2726d86ee94c1c37fe8598257b7b4e87e869e |
| NotoEmoji-VF.ttf | [google/fonts의 원본 파일](https://github.com/google/fonts/blob/47d54ca1ebb14fd864695f1d39dc734f39017c33/ofl/notoemoji/NotoEmoji%5Bwght%5D.ttf), commit 47d54ca1ebb14fd864695f1d39dc734f39017c33 | Noto Emoji 3.002; 단색 outline emoji, weight 축 300–700 중 400 | de6c18832938afc99caf132b39d6a30a19bac7f2e812e28db2535b4608d27551 |

각 글꼴과 함께 제공한 *-OFL.txt는 같은 upstream commit에서 가져왔고, 문구와 줄 순서를 보존하면서 원문 줄 끝 공백만 정리했다. [Noto CJK 글꼴 저장소의 라이선스](https://github.com/notofonts/noto-cjk/blob/f8d157532fbfaeda587e826d4cd5b21a49186f7c/Sans/LICENSE)와 [Google Fonts의 개별 라이선스 경로](https://github.com/google/fonts/tree/47d54ca1ebb14fd864695f1d39dc734f39017c33/ofl)를 함께 고정했다. 라이선스는 SIL Open Font License 1.1이며 파일 SHA-256은 다음과 같다.

- NotoSansCJK-OFL.txt: 6a73f9541c2de74158c0e7cf6b0a58ef774f5a780bf191f2d7ec9cc53efe2bf2
- NotoSansArabic-OFL.txt: df5143cdf3380169f2d03bf6d2cd243e85621fd097e608c3607cf7f9c9884ae6
- NotoEmoji-OFL.txt: 79d348cb5161481f603dcda564fb129a9b2c11d7012dcad820af045fed75c020

읽기 전용 fontTools 4.60.2 검사에서 fixture 문자열의 각 codepoint가 지정 글꼴에 존재하는 것을 확인했다. Noto Emoji GSUB는 u1F469 + uni200D + laptop을 technologist 단일 glyph로 매핑한다. 브라우저 fixture는 @font-face 로드 여부와 rendered grapheme range를 별도로 기록한다. 이 검사는 지정한 sample 문자열만 다루며 전체 script coverage나 일반 font fallback을 보장하지 않는다.
