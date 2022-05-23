use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

#[allow(unused)]
mod intl {

    /// Reference [Pokemon Gen 3 charset](https://bulbapedia.bulbagarden.net/wiki/Character_encoding_(Generation_III))
    pub const CHARSET_TABLE: [[&str; 16]; 16] = [
        [
            " ", "À", "Á", "Â", "Ç", "È", "É", "Ê", "Ë", "Ì", " ", "Î", "Ï", "Ò", "Ó", "Ô",
        ],
        [
            "Œ", "Ù", "Ú", "Û", "Ñ", "ß", "à", "á", " ", "ç", "è", "é", "ê", "ë", "ì", " ",
        ],
        [
            "î", "ï", "ò", "ó", "ô", "œ", "ù", "ú", "û", "ñ", "º", "ª", " ", "&", "+", " ",
        ],
        [
            " ", " ", " ", " ", "Lv", "=", ";", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        ],
        [" "; 16],
        [
            "▯", "¿", "¡", "PK", "MN", "PO", "Ké", " ", " ", " ", "Í", "%", "(", ")", " ", " ",
        ],
        [
            " ", " ", " ", " ", " ", " ", " ", " ", "â", " ", " ", " ", " ", " ", " ", "í",
        ],
        [
            " ", " ", " ", " ", " ", " ", " ", " ", " ", "⬆", "⬇", "⬅", "➡", "*", "*", "*",
        ],
        [
            "*", "*", "*", "*", "ᵉ", "<", ">", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        ],
        [" "; 16],
        [
            "ʳᵉ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "!", "?", ".", "-", "・",
        ],
        [
            "...", "“", "”", "‘", "’", "♂", "♀", " ", ",", "×", "/", "A", "B", "C", "D", "E",
        ],
        [
            "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
        ],
        [
            "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
        ],
        [
            "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "▶",
        ],
        [
            ":", "Ä", "Ö", "Ü", "ä", "ö", "ü", " ", " ", " ", "", "", "", "", "", "",
        ],
    ];
}

#[allow(unused)]
mod jpn {

    /// Reference [Pokemon Gen 3 charset](https://bulbapedia.bulbagarden.net/wiki/Character_encoding_(Generation_III))
    pub const CHARSET_TABLE: [[&str; 16]; 16] = [
        [
            " ", "あ", "い", "う", "え", "お", "か", "き", "く", "け", "こ", "さ", "し", "す",
            "せ", "そ",
        ],
        [
            "た", "ち", "つ", "て", "と", "な", "に", "ぬ", "ね", "の", "は", "ひ", "ふ", "へ",
            "ほ", "ま",
        ],
        [
            "み", "む", "め", "も", "や", "ゆ", "よ", "ら", "り", "る", "れ", "ろ", "わ", "を",
            "ん", "ぁ",
        ],
        [
            "ぃ", "ぅ", "ぇ", "ぉ", "ゃ", "ゅ", "ょ", "が", "ぎ", "ぐ", "げ", "ご", "ざ", "じ",
            "ず", "ぜ",
        ],
        [
            "ぞ", "だ", "ぢ", "づ", "で", "ど", "ば", "び", "ぶ", "べ", "ぼ", "ぱ", "ぴ", "ぷ",
            "ぺ", "ぽ",
        ],
        [
            "っ", "ア", "イ", "ウ", "エ", "オ", "カ", "キ", "ク", "ケ", "コ", "サ", "シ", "ス",
            "セ", "ソ",
        ],
        [
            "タ", "チ", "ツ", "テ", "ト", "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ", "ヒ", "フ", "ヘ",
            "ホ", "マ",
        ],
        [
            "ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ", "リ", "ル", "レ", "ロ", "ワ", "ヲ",
            "ン", "ァ",
        ],
        [
            "ィ", "ゥ", "ェ", "ォ", "ャ", "ュ", "ョ", "ガ", "ギ", "グ", "ゲ", "ゴ", "ザ", "ジ",
            "ズ", "ゼ",
        ],
        [
            "ゾ", "ダ", "ヂ", "ヅ", "デ", "ド", "バ", "ビ", "ブ", "ベ", "ボ", "パ", "ピ", "プ",
            "ペ", "ポ",
        ],
        [
            "ッ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "！", "？", "。", "ー", "・",
        ],
        [
            "..", "『", "』", "「", "」", "♂", "♀", "円", ".", "×", "/", "A", "B", "C", "D", "E",
        ],
        [
            "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
        ],
        [
            "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
        ],
        [
            "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "▶",
        ],
        [
            ":", "Ä", "Ö", "Ü", "ä", "ö", "ü", "⬆", "⬇", "⬅", "", "", "", "", "", "",
        ],
    ];
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokeLanguage {
    Japanese = 1,
    English = 2,
    French = 3,
    Italian = 4,
    German = 5,
    Korean = 6,
    Spanish = 7,
}

impl Default for PokeLanguage {
    fn default() -> Self {
        PokeLanguage::English
    }
}

#[derive(Debug)]
pub struct Intl;

#[derive(Debug)]
pub struct Jpn;

pub trait HasCharTable {
    const CHAR_TABLE: [[&'static str; 16]; 16];
}

impl HasCharTable for Intl {
    const CHAR_TABLE: [[&'static str; 16]; 16] = intl::CHARSET_TABLE;
}

impl HasCharTable for Jpn {
    const CHAR_TABLE: [[&'static str; 16]; 16] = jpn::CHARSET_TABLE;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PkStrFFI<T: HasCharTable, const N: usize>(PkString<T, [u8; N]>);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PkString<T: HasCharTable, U> {
    raw_data: U,
    phantom: PhantomData<T>,
}

impl<T: HasCharTable, U> From<U> for PkString<T, U> {
    fn from(data: U) -> Self {
        Self {
            raw_data: data,
            phantom: PhantomData::default(),
        }
    }
}

impl<T, const N: usize> Display for PkStrFFI<T, N>
where
    T: HasCharTable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from(&self.0);
        f.write_str(&s)
    }
}

impl<T, U> Display for PkString<T, U>
where
    T: HasCharTable,
    U: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from(self);
        f.write_str(&s)
    }
}

impl<T, const N: usize> From<[u8; N]> for PkStrFFI<T, N>
where
    T: HasCharTable,
{
    fn from(data: [u8; N]) -> Self {
        Self(PkString::from(data))
    }
}

impl<T, const N: usize> From<PkStrFFI<T, N>> for String
where
    T: HasCharTable,
{
    fn from(data: PkStrFFI<T, N>) -> Self {
        String::from(data.0)
    }
}

impl<T, U> From<PkString<T, U>> for String
where
    T: HasCharTable,
    U: AsRef<[u8]>,
{
    fn from(data: PkString<T, U>) -> Self {
        let mut s = String::with_capacity(data.raw_data.as_ref().len());
        for c in data.raw_data.as_ref() {
            let c = *c;
            if c == 255 {
                break;
            }
            let high = (c & 0x0F) as usize;
            let low = (c >> 4) as usize;
            s.push_str(T::CHAR_TABLE[low][high]);
        }
        s
    }
}

impl<'a, T, U> From<&'a PkString<T, U>> for String
where
    T: HasCharTable,
    U: AsRef<[u8]>,
{
    fn from(data: &'a PkString<T, U>) -> Self {
        let mut s = String::with_capacity(data.raw_data.as_ref().len());
        for c in data.raw_data.as_ref() {
            let c = *c;
            if c == 255 {
                break;
            }
            let high = (c & 0x0F) as usize;
            let low = (c >> 4) as usize;
            s.push_str(T::CHAR_TABLE[low][high]);
        }
        s
    }
}

mod test {
    use crate::charset::{PkStrFFI, Jpn};

    use super::{Intl, PkString};
    use std::mem::size_of;

    #[test]
    fn test_sz() {
        assert_eq!(size_of::<PkStrFFI<Intl, 7>>(), size_of::<[u8; 7]>());
        assert_eq!(size_of::<PkString<Intl, [u8; 7]>>(), size_of::<[u8; 7]>())
    }

    #[test]
    fn parse_pkstring_intl() {
        let bytes = [0xBCu8, 0xCF, 0xBE, 0xC3, 0xFF, 0xFF, 0xFF];
        let s = PkString::<Intl, [u8; 7]>::from(bytes);
        assert_eq!(&format!("{}", s), "BUDI");
        println!("{}", s);
        println!("{:?}", s);
    }

    #[test]
    fn parse_pkstrffi_intl() {
        let bytes = [0xBCu8, 0xCF, 0xBE, 0xC3, 0xFF, 0xFF, 0xFF];
        let s = PkStrFFI::<Intl, 7>::from(bytes);
        assert_eq!(&format!("{}", s), "BUDI");
        println!("{}", s);
        println!("{:?}", s);
    }

    #[test]
    fn parse_pkstrffi_jpn() {
        let bytes = [112u8, 142, 139, 123, 83, 255, 0, 8, 76, 125];
        let s = PkStrFFI::<Jpn, 10>::from(bytes);
        assert_eq!(&format!("{}", s), "ミズゴロウ");
        println!("{}", s);
        println!("{:?}", s);
    }

    #[test]
    fn parse_pkstrffi_transmute() {
        use std::mem::transmute;

        let bytes = [0xBCu8, 0xCF, 0xBE, 0xC3, 0xFF, 0xFF, 0xFF];
        let s = unsafe { transmute::<_, PkStrFFI<Intl, 7>>(bytes) };
        assert_eq!(&format!("{}", s), "BUDI");
        println!("{}", s);
        println!("{:?}", s);
    }
}
