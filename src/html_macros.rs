#![allow(dead_code)]

//! Allows for dominator shorthand

use {
    dominator::{Dom, DomBuilder},
    web_sys::*,
};

macro_rules! el_fns {
    ( $( $el:ident => $tag:ty ),* $(,)? ) => {
    $(
        pub fn $el(f: impl FnOnce(DomBuilder<$tag>) -> DomBuilder<$tag>) -> Dom{
            let builder = DomBuilder::<$tag>::new_html(stringify!($el));
            f(builder).into_dom()
        }
    )*
    };
}

// to unlock more Html{}Element types add it to the features list in:
// (link only works with vscode extension installed called `Isotechnics.commentlinks`) [[rm-applications/utils/wasm_utils/Cargo.toml]]
// Valid list: https://docs.rs/crate/web-sys/0.3.64/features
el_fns! {div => HtmlDivElement,
a => HtmlAnchorElement,
abbr => HtmlElement,
aside => HtmlElement,
button => HtmlButtonElement,
canvas => HtmlCanvasElement,
embed => HtmlEmbedElement,
footer => HtmlElement,
hr => HtmlElement,
i => HtmlElement,
iframe => HtmlIFrameElement,
img => HtmlImageElement,
input => HtmlInputElement,
label => HtmlElement,
li => HtmlElement,
main => HtmlElement,
nav => HtmlElement,
object => HtmlObjectElement,
option => HtmlOptionElement,
p => HtmlParagraphElement,
pre => HtmlElement,
progress => HtmlProgressElement,
select => HtmlSelectElement,
span => HtmlSpanElement,
strong => HtmlElement,
table => HtmlTableElement,
tbody => HtmlTableSectionElement,
textarea => HtmlTextAreaElement,
td => HtmlTableCellElement,
tfoot => HtmlTableSectionElement,
th => HtmlTableCellElement,
thead => HtmlTableSectionElement,
tr => HtmlTableRowElement,
ul => HtmlElement,
h1 => HtmlHeadingElement,
h2 => HtmlHeadingElement,
h3 => HtmlHeadingElement,
h4 => HtmlHeadingElement,
h5 => HtmlHeadingElement,
}
