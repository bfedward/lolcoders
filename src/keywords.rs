macro_rules! define_keywords {
    // compare this to each line in the marco usage below.
    // For Hai => "HAI", Hai is variant:ident and "HAI" is str:literal.
    // The $(...),* syntax means to repeat for everythig in the marco usage below.
    // $(,)? means the last trailing comma is optional.
    ($($variant:ident => $str:literal),* $(,)?) => {
        // This generates a Keyword enum with all the variant:idents.
        // i.e., Hai, KThxBye, Visible and so on.
        #[derive(Debug, Clone, PartialEq)]
        pub enum Keyword {
            // notice the $(...)* syntax again - repeat for all the variant:idents
            $($variant),*
        }

        // this generates an impl of fmt::Display for all the Keywords
        impl std::fmt::Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(Keyword::$variant => $str),*
                };
                write!(f, "{s}")
            }
        }

        // this generates a from_str function for the Keywords.
        // This means we can regognise that "HAI" is Keyword::Hai.
        impl Keyword {
            pub fn from_str(word: &str) -> Option<Self> {
                match word {
                    // repeat for all the keywords
                    $($str => Some(Keyword::$variant)),*,
                    _ => None,
                }
            }

            // makes a static array of all the keywords as strs.
            pub const ALL: &'static [&'static str] = &[
                $($str),*
            ];
        }
    };
}

// when new keywords need added, they just need added in here.
define_keywords! {
    A => "A",
    All => "ALL",
    An => "AN",
    Any => "ANY",
    Biggr => "BIGGR",
    Both => "BOTH",
    Btw => "BTW",
    Can => "CAN",
    Diff => "DIFF",
    Diffrint => "DIFFRINT",
    Either => "EITHER",
    Found => "FOUND",
    Gimmeh => "GIMMEH",
    Gtfo => "GTFO",
    Hai => "HAI",
    Has => "HAS",
    How => "HOW",
    I => "I",
    If => "IF",
    Is => "IS",
    Itz => "ITZ",
    Iz => "IZ",
    KThxBye => "KTHXBYE",
    Maek => "MAEK",
    Mebbe => "MEBBE",
    Mkay => "MKAY",
    Mod => "MOD",
    No => "NO",
    Noob => "NOOB",
    Not => "NOT",
    Now => "NOW",
    Numbar => "NUMBAR",
    Numbr => "NUMBR",
    O => "O",
    Of => "OF",
    Oic => "OIC",
    Omg => "OMG",
    Omgwtf => "OMGWTF",
    Produkt => "PRODUKT",
    Quoshunt => "QUOSHUNT",
    R => "R",
    Rly => "RLY",
    Saem => "SAEM",
    Say => "SAY",
    Smallr => "SMALLR",
    So => "SO",
    Smoosh => "SMOOSH",
    Srs => "SRS",
    Sum => "SUM",
    Troof => "TROOF",
    U => "U",
    Visible => "VISIBLE",
    Wai => "WAI",
    Won => "WON",
    Wtf => "WTF",
    Ya => "YA",
    Yarn => "YARN",
    Yr => "YR",
}
