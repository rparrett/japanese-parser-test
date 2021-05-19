use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{char, line_ending, space0},
    combinator::{map, not, opt},
    error::{Error, ErrorKind},
    multi::{fold_many0, many0, many1, separated_list0},
    sequence::{delimited, pair, tuple},
    tuple, IResult,
};
#[derive(Debug)]
struct TypingTarget {
    displayed_chunks: Vec<String>,
    typed_chunks: Vec<Vec<String>>,
}

static HIRAGANA: &str = "あいぅうえおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもやゆよらりるれろわゐゑをんーっ";
static KATAKANA: &str = "アイウエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヰヱヲンー";
static SUTEGANA: &str = "ァィゥェォャュョぁぃぅぇぉゃゅょ";
static SOKUON: &str = "っッ";
static KANA: &str = "あいぅうえおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもやゆよらりるれろわゐゑをんーアイウエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヰヱヲンーァィゥェォャュョぁぃぅぇぉゃゅょっッ";

fn kana_to_typed_chunks(c: char) -> Option<Vec<String>> {
    match c {
        // hiragana
        'あ' => Some(vec!["a".to_owned()]),
        'い' => Some(vec!["i".to_owned()]),
        'う' => Some(vec!["u".to_owned()]),
        'え' => Some(vec!["e".to_owned()]),
        'か' => Some(vec!["ka".to_owned()]),
        'が' => Some(vec!["ga".to_owned()]),
        'き' => Some(vec!["ki".to_owned()]),
        'ぎ' => Some(vec!["gi".to_owned()]),
        'く' => Some(vec!["ku".to_owned()]),
        'ぐ' => Some(vec!["gu".to_owned()]),
        'け' => Some(vec!["ke".to_owned()]),
        'げ' => Some(vec!["ge".to_owned()]),
        'こ' => Some(vec!["ko".to_owned()]),
        'ご' => Some(vec!["go".to_owned()]),
        'さ' => Some(vec!["sa".to_owned()]),
        'ざ' => Some(vec!["za".to_owned()]),
        'し' => Some(vec!["shi".to_owned(), "si".to_owned()]),
        'じ' => Some(vec!["ji".to_owned()]),
        'す' => Some(vec!["su".to_owned()]),
        'ず' => Some(vec!["zu".to_owned()]),
        'せ' => Some(vec!["se".to_owned()]),
        'ぜ' => Some(vec!["ze".to_owned()]),
        'そ' => Some(vec!["so".to_owned()]),
        'ぞ' => Some(vec!["zo".to_owned()]),
        'た' => Some(vec!["ta".to_owned()]),
        'だ' => Some(vec!["da".to_owned()]),
        'ち' => Some(vec!["chi".to_owned()]),
        'ぢ' => Some(vec!["ji".to_owned()]), // ?
        'つ' => Some(vec!["tsu".to_owned(), "tu".to_owned()]),
        'づ' => Some(vec!["dzu".to_owned(), "du".to_owned()]),
        'て' => Some(vec!["te".to_owned()]),
        'で' => Some(vec!["de".to_owned()]),
        'と' => Some(vec!["to".to_owned()]),
        'ど' => Some(vec!["do".to_owned()]),
        'な' => Some(vec!["na".to_owned()]),
        'に' => Some(vec!["ni".to_owned()]),
        'ぬ' => Some(vec!["nu".to_owned()]),
        'ね' => Some(vec!["ne".to_owned()]),
        'の' => Some(vec!["no".to_owned()]),
        'は' => Some(vec!["ha".to_owned()]),
        'ば' => Some(vec!["ba".to_owned()]),
        'ぱ' => Some(vec!["po".to_owned()]),
        'ひ' => Some(vec!["hi".to_owned()]),
        'び' => Some(vec!["bi".to_owned()]),
        'ぴ' => Some(vec!["po".to_owned()]),
        'ふ' => Some(vec!["fu".to_owned()]),
        'ぶ' => Some(vec!["bu".to_owned()]),
        'ぷ' => Some(vec!["pu".to_owned()]),
        'へ' => Some(vec!["he".to_owned()]),
        'べ' => Some(vec!["be".to_owned()]),
        'ぺ' => Some(vec!["pe".to_owned()]),
        'ほ' => Some(vec!["ho".to_owned()]),
        'ぼ' => Some(vec!["bo".to_owned()]),
        'ぽ' => Some(vec!["po".to_owned()]),
        'ま' => Some(vec!["ma".to_owned()]),
        'み' => Some(vec!["mi".to_owned()]),
        'む' => Some(vec!["mu".to_owned()]),
        'め' => Some(vec!["me".to_owned()]),
        'も' => Some(vec!["mo".to_owned()]),
        'や' => Some(vec!["ya".to_owned()]),
        'ゆ' => Some(vec!["yu".to_owned()]),
        'よ' => Some(vec!["yo".to_owned()]),
        'ら' => Some(vec!["ra".to_owned()]),
        'り' => Some(vec!["ri".to_owned()]),
        'る' => Some(vec!["ru".to_owned()]),
        'れ' => Some(vec!["re".to_owned()]),
        'ろ' => Some(vec!["ro".to_owned()]),
        'わ' => Some(vec!["wa".to_owned()]),
        'ゐ' => Some(vec!["wi".to_owned()]),
        'ゑ' => Some(vec!["we".to_owned()]),
        'を' => Some(vec!["wo".to_owned()]),
        'ん' => Some(vec!["n".to_owned(), "nn".to_owned()]),
        // katakana
        _ => None,
    }
}

fn is_not_kana_or_open_paren(c: char) -> bool {
    c != '(' && !KANA.contains(c)
}

fn is_hiragana(i: &str) -> nom::IResult<&str, char> {
    nom::character::complete::one_of(HIRAGANA)(i)
}

fn is_sutegana(i: &str) -> nom::IResult<&str, char> {
    nom::character::complete::one_of(SUTEGANA)(i)
}

fn is_sokuon(i: &str) -> nom::IResult<&str, char> {
    nom::character::complete::one_of(SOKUON)(i)
}

fn parenthesized(i: &str) -> nom::IResult<&str, TypingTarget> {
    map(
        many1(pair(
            take_while(is_not_kana_or_open_paren),
            delimited(tag("("), take_while(|c| c != ')'), tag(")")),
        )),
        |things: Vec<(&str, &str)>| {
            let mut typed_chunks = vec![];
            let mut displayed_chunks = vec![];
            for (displayed, typed) in things {
                typed_chunks.push(vec![typed.into()]);
                displayed_chunks.push(displayed.into());
            }
            TypingTarget {
                typed_chunks,
                displayed_chunks,
            }
        },
    )(i)
}

fn japanese(i: &str) -> nom::IResult<&str, Vec<TypingTarget>> {
    many0(alt((kana_chunk, parenthesized)))(i)
}

fn kana_chunk(i: &str) -> nom::IResult<&str, TypingTarget> {
    map(
        many1(tuple((opt(is_sokuon), is_hiragana, opt(is_sutegana)))),
        |things| {
            let mut typed_chunks = vec![];
            let mut displayed_chunks = vec![];

            for (sokuon, kana, _sutegana) in things {
                if let Some(typed) = kana_to_typed_chunks(kana) {
                    if let Some(sokuon) = sokuon {
                        // TODO potentially, the first character of a second/third/other
                        // typed chunk could be different.

                        typed_chunks.push(vec![typed
                            .get(0)
                            .unwrap()
                            .chars()
                            .next()
                            .unwrap()
                            .into()]);
                        displayed_chunks.push(sokuon.into());
                    }
                    typed_chunks.push(typed);
                    displayed_chunks.push(kana.into());
                }
            }

            TypingTarget {
                typed_chunks,
                displayed_chunks,
            }
        },
    )(i)
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", parenthesized("test(what)who(bleh)"));
    println!("{:?}", parenthesized("小(chii)さい"));
    println!("{:?}", kana_chunk("はじめましてとっとりです"));
    println!("--- japanese");
    println!("{:?}", japanese("test"));
    println!("{:?}", japanese("test(what)who(bleh)"));
    println!("{:?}", japanese("小さい"));
    println!("{:?}", japanese("小(chii)さい"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hiragana() {
        assert_eq!(is_hiragana("あ"), Ok(("", 'あ')));
        assert_eq!(is_hiragana("はじめまして。"), Ok(("じめまして。", 'は')));
        assert_eq!(
            is_hiragana("P"),
            Err(nom::Err::Error(Error {
                input: "P",
                code: ErrorKind::OneOf
            }))
        );
    }
}
