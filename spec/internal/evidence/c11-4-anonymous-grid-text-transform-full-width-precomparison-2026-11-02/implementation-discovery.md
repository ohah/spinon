# `full-width` line-breaking 구현 발견

첫 `spinon-text-layout` feature 실행은 규범 문자열 mapping `AB 12` → `ＡＢ　１２`을 적용했지만, Parley `WordBreak::Normal`은 min-content `10px`, max-content `50px`, min-content 높이 `40px`을 보고했다. line-break opportunity가 full-width 글자 두 개와 digit 두 개 사이에도 생겨 10px line 네 개로 나뉘었다.

Chrome 156은 지원하지 않는 `text-transform:full-width` 선언을 computed `none`으로 돌려주므로 property 자체를 검증할 수 없다. 출력 문자열 `ＡＢ　１２`를 그대로 사용한 candidate에서 Chrome은 `word-break:normal`, Grid min-content track `20px`, 10px line 두 개를 관측했다. Candidate는 wrapping·font metric 관측값이고, 변환 기준은 CSS Text와 Unicode mapping으로 유지한다.

정확한 변환 candidate에 한정한 Parley `WordBreak::KeepAll` 실험은 min-content `20px`, max-content `50px`, min-content 높이 `20px`을 냈고 U+3000 뒤 break opportunity를 보존했다. 계획 보완은 document의 computed `word-break:normal`을 그대로 두고 이 내부 설정을 allowlist source `AB 12`, `AB   12`에만 적용하도록 제안한다. 이 발견은 수정 완료 근거가 아니다. 구현은 published layout의 computed `word-break:normal` 유지, U+0020 source range 보존, candidate 폭·line count 일치를 입증해야 하며 실패하면 해당 profile을 fail-closed한다.
