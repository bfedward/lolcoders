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
    Hai => "HAI",
    KThxBye => "KTHXBYE",
    Visible => "VISIBLE",
    I => "I",
    Itz => "ITZ",
    Has => "HAS",
    A => "A",
    How => "HOW",
    Iz => "IZ",
    If => "IF",
    U => "U",
    Say => "SAY",
    So => "SO",
    Yr => "YR",
    An => "AN",
    Mkay => "MKAY",
    Troof => "TROOF",
    Yarn => "YARN",
    Numbr => "NUMBR",
    Numbar => "NUMBAR",
    Noob => "NOOB",
    Found => "FOUND",
    Gtfo => "GTFO",
    R => "R",
    Sum => "SUM",
    Diff => "DIFF",
    Produkt => "PRODUKT",
    Quoshunt => "QUOSHUNT",
    Mod => "MOD",
    Biggr => "BIGGR",
    Smallr => "SMALLR",
    Of => "OF",
    Both => "BOTH",
    Either => "EITHER",
    Won => "WON",
    Not => "NOT",
    All => "ALL",
    Any => "ANY",
    Saem => "SAEM",
    Diffrint => "DIFFRINT",
    Btw => "BTW"
}
