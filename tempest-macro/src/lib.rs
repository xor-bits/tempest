use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident,  TokenStream, };
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    LitStr, Result, Token,
};

//

#[derive(Debug)]
struct View {
    tags: Vec<Tag>,
}

impl View {
    fn build(self, builder: &mut PartBuilder) {
        for tag in self.tags {
            tag.build(builder);
        }
    }
}

impl Parse for View {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tags = Vec::new();
        while input.peek(Token![<]) {
            tags.push(input.parse()?);
        }
        Ok(Self { tags })
    }
}

#[derive(Debug)]
enum Tag {
    Block(BlockTag),
    Inline(InlineTag),
}

impl Tag {
    fn build(self, builder: &mut PartBuilder) {
        match self {
            Tag::Block(t) => t.build(builder),
            Tag::Inline(t) => t.build(builder),
        }
    }
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Self> {
        let open: OpenTag = input.parse()?;

        if open.closing.is_some() {
            Ok(Self::Inline(InlineTag { open }))
        } else {
            let mut contents = Vec::new();
            while !(input.peek(Token![<]) && input.peek2(Token![/])) {
                contents.push(input.parse()?);
            }

            Ok(Self::Block(BlockTag {
                open,
                contents,
                close: input.parse()?,
            }))
        }
    }
}

#[derive(Debug)]
struct BlockTag {
    open: OpenTag,
    contents: Vec<Content>,
    close: CloseTag,
}

impl BlockTag {
    fn build(self, builder: &mut PartBuilder) {
        self.open.build(builder);
        for c in self.contents {
            c.build(builder);
        }
        self.close.build(builder);
    }
}

#[derive(Debug)]
enum Content {
    Text(String),
    Param(TokenStream),
    Tag(Tag),
}

impl Content {
    fn build(self, builder: &mut PartBuilder) {
        match self {
            Content::Text(t) => {
                let s = t.as_str();
                builder.push_param(&quote! { #s })
            }
            Content::Param(p) => builder.push_param(&p),
            Content::Tag(t) => t.build(builder),
        }
    }
}

impl Parse for Content {
    fn parse(input: ParseStream) -> Result<Self> {
        // if input.peek(Token![<]) {
        //     Ok(Self::Tag(input.parse()?))
        // } else if input.peek(syn::token::Brace) {
        //     let expr;
        //     braced!(expr in input);
        //     Ok(Self::Param(expr.parse::<TokenStream>()?))
        // } else {
        //     let first = input.parse::<TokenTree>()?.span();
        //     let mut last = first;

        //     while !(input.peek(Token![<]) || input.peek(syn::token::Brace)) {
        //         last = input.parse::<TokenTree>()?.span();
        //     }

        //     first.join(last);

        //     todo!()
        // }

        let look = input.lookahead1();
        if look.peek(Token![<]) {
            Ok(Self::Tag(input.parse()?))
        } else if look.peek(syn::token::Brace) {
            let pasted_item;
            braced!(pasted_item in input);
            let tt = pasted_item.parse::<TokenStream>()?;
            Ok(Self::Param(tt))
        } else if look.peek(syn::LitStr) {
            let str: LitStr = input.parse()?;
            // Ok(Self::Text(format!("\"{}\"", str.value())))
            Ok(Self::Text(str.value()))
        } else if look.peek(syn::Ident) {
            let s: Ident = input.parse()?;
            Ok(Self::Text(s.to_string()))
        } else {
            // accept anything in Content?:
            // let s = input.parse::<TokenTree>()?.to_string();
            // Ok(Self::Text(s))

            Err(look.error())
        }
    }
}

#[derive(Debug)]
struct InlineTag {
    open: OpenTag,
}

impl InlineTag {
    fn build(self, builder: &mut PartBuilder) {
        self.open.build(builder)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct OpenTag {
    beg: Token![<],
    attrs: Option<TagAttrs>,
    closing: Option<Token![/]>,
    end: Token![>],
}

impl OpenTag {
    fn build(self, builder: &mut PartBuilder) {
        builder.push_str("<");
        if let Some(attrs) = self.attrs {
            attrs.build(builder);
        }
        if self.closing.is_some() {
            builder.push_str("/");
        }
        builder.push_str(">");
    }
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let beg = input.parse()?;

        let mut attrs = None;
        if input.peek(syn::Ident) {
            attrs = Some(input.parse()?);
        }

        Ok(Self {
            beg,
            attrs,
            closing: input.parse()?,
            end: input.parse()?,
        })
    }
}

#[allow(unused)]
#[derive(Debug)]
struct CloseTag {
    beg: Token![<],
    closing: Token![/],
    key: Option<Ident>,
    end: Token![>],
}

impl CloseTag {
    fn build(self, builder: &mut PartBuilder) {
        builder.push_str("</");
        if let Some(key) = self.key {
            builder.push_str(key.to_string().as_str());
        }
        builder.push_str(">");
    }
}

impl Parse for CloseTag {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            beg: input.parse()?,
            closing: input.parse()?,
            key: input.parse()?,
            end: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct TagAttrs {
    key: Key,
    others: Vec<TagAttr>,
}

impl TagAttrs {
    fn build(self, builder: &mut PartBuilder) {
        self.key.build(builder);
        for attr in self.others {
            builder.push_str(" ");
            attr.build(builder);
        }
    }
}

impl Parse for TagAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        let mut others = Vec::new();

        while input.peek(syn::Ident) {
            others.push(input.parse()?);
        }

        Ok(Self { key, others })
    }
}

#[derive(Debug)]
struct TagAttr {
    key: Key,
    val: Option<(Token![=], LitStr)>,
}

impl TagAttr {
    fn build(self, builder: &mut PartBuilder) {
        self.key.build(builder);
        if let Some((_, val)) = self.val {
            builder.push_str("=\"");
            builder.push_str(val.value().as_str());
            builder.push_str("\"");
        }
    }
}

impl Parse for TagAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        let mut val = None;
        if input.peek(Token![=]) {
            val = Some((input.parse()?, input.parse()?));
        }

        Ok(Self { key, val })
    }
}

#[derive(Debug)]
struct Key {
    first: Ident,
    parts: Vec<(Token![-], Ident)>,
}

impl Key {
    
    fn build(self, builder: &mut PartBuilder) {
        builder.push_str(self.first.to_string().as_str());
        for (_, part) in self.parts {
            builder.push_str("-");
        builder.push_str(part.to_string().as_str());
        }
        
    }
}

impl Parse for Key {
    fn parse(input: ParseStream) -> Result<Self> {
        let first = input.parse()?;
        let mut parts = Vec::new();
        while input.peek(Token![-]) {
            parts.push((input.parse()?, input.parse()?))
            ;
        }

        Ok(Self {first, parts})
    }
}

//

struct PartBuilder {
    str_builder: String,
    stream: TokenStream,
}

impl PartBuilder {
    fn new() -> Self {
        Self {
            str_builder: String::new(),
            stream: TokenStream::new(),
        }
    }

    fn push_str(&mut self, str: &str) {
        self.str_builder.push_str(str);
    }

    fn push_param(&mut self, p: &TokenStream) {
        let static_str = self.str_builder.as_str();
        self.stream.extend(quote! {
            f.write_str(#static_str)?;
            tempest::View::fmt(&(#p), f)?;
        });
        self.str_builder.clear();
    }

    fn finish(mut self) -> TokenStream {
        let static_str = self.str_builder.as_str();
        self.stream.extend(quote! {
            f.write_str(#static_str)?;
        });

        self.stream
    }
}

//

#[proc_macro]
pub fn view(input: TokenStream1) -> TokenStream1 {
    let view = syn::parse_macro_input!(input as View);

    let mut builder = PartBuilder::new();
    view.build(&mut builder);
    let stream = builder.finish();

    quote! {{
        tempest::WrapView(move |f: &mut core::fmt::Formatter| -> core::fmt::Result {
            #stream
            Ok(())
        })
    }}
    .into()
}
