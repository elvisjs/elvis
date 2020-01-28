use crate::Error;
// use crate::CSS;
use std::collections::HashMap;

/// Virtual DOM Tree
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tree {
    pub attrs: HashMap<&'static str, &'static str>,
    pub tag: &'static str,
    pub children: Vec<Box<Tree>>,
}

impl Tree {
    /// rescursion deserialize wrapper
    pub fn de(h: &'static str) -> Result<Self, Error> {
        Ok(Self::rde(h)?.0)
    }

    /// Deserialize Tree from html string
    ///
    /// `attrs` field follows MDN doc [HTML attribute refference][1],
    /// all values are `String` in "".
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
    pub fn rde(h: &'static str) -> Result<(Self, Option<parser::Extra>), Error> {
        let mut pos = 0_usize;
        if h.is_empty() {
            return Ok((Tree::default(), None));
        } else if h.find("</").is_none() {
            return Ok((parser::plain(h), None));
        }

        let (tag, attrs) = parser::tag(&h[pos..], &mut pos)?;

        // parse f*cking children
        let mut children: Vec<Box<Tree>> = vec![];
        let mut cext = parser::ch(tag, &h[pos..], &mut children)?;

        // parse parallel children
        pos += cext.pos;
        while !cext.end {
            cext = parser::ch(tag, &h[pos..], &mut children)?;
            pos += cext.pos;
        }

        // communite with child parser
        let mut ext = None;
        if (pos + 1) != h.len() {
            ext = Some(parser::Extra {
                end: false,
                pos: pos,
                tag: cext.tag,
            });
        }

        Ok((
            Tree {
                tag,
                attrs,
                children,
            },
            ext,
        ))
    }
}

/// Parser for html tokenstream
pub mod parser {
    use crate::{Error, Tree};
    use std::collections::HashMap;

    /// Extra html stream
    #[derive(Debug)]
    pub struct Extra {
        pub end: bool,
        pub pos: usize,
        pub tag: &'static str,
    }

    /// process of parsing children
    #[derive(Eq, PartialEq)]
    enum ChildrenProcess {
        BeginTag,
        CloseTag,
        None,
        Plain,
    }

    /// process of parsing tag
    enum TagProcess {
        Attrs,
        Quote,
        None,
        Tag,
    }

    pub fn ch(
        tag: &'static str,
        cht: &'static str,
        children: &mut Vec<Box<Tree>>,
    ) -> Result<Extra, Error> {
        let mut itag = tag;
        let mut process = ChildrenProcess::None;
        let (mut t, mut c) = ((0, 0), (0, 0));
        for (p, q) in cht.chars().enumerate() {
            match q {
                '<' => {
                    if process == ChildrenProcess::Plain {
                        c.1 = p;
                    }

                    process = ChildrenProcess::BeginTag;
                    t.0 = p;
                    t.1 = p;
                }
                '/' => {
                    if &cht[t.0..(t.0 + 1)] == "<" {
                        println!("reach close");
                        process = ChildrenProcess::CloseTag;
                    } else if process != ChildrenProcess::Plain {
                        return Err(Error::DeserializeHtmlError(format!(
                            "children parse failed {}, cht: {}, process: {}",
                            &tag,
                            &cht,
                            &cht[t.0..(t.0 + 1)],
                        )));
                    }
                }
                '>' => {
                    t.1 = p;
                    match process {
                        ChildrenProcess::BeginTag => {
                            println!("reach begin");
                            let (tree, ext) = Tree::rde(&cht[t.0..])?;
                            children.push(Box::new(tree));

                            if let Some(cext) = ext {
                                return Ok(Extra {
                                    end: false,
                                    tag: cext.tag,
                                    pos: cext.pos + t.0,
                                });
                            }
                        }
                        ChildrenProcess::CloseTag => {
                            // verify tag, trim:  "/ tag"
                            itag = &cht[(t.0 + 1)..t.1].trim()[1..].trim()[..];
                            if itag != tag {
                                return Err(Error::DeserializeHtmlError(format!(
                                    "children parse failed {}, cht: {}, close_tag: {}",
                                    &tag, &cht, &itag
                                )));
                            } else if !cht[c.0..c.1].is_empty() {
                                children.push(Box::new(plain(&cht[c.0..c.1])));
                            }

                            return Ok(Extra {
                                end: true,
                                pos: p,
                                tag: itag,
                            });
                        }
                        _ => {
                            // None and Plain
                        }
                    }
                }
                x if !x.is_whitespace() => {
                    match process {
                        ChildrenProcess::None => {
                            process = ChildrenProcess::Plain;
                            c.0 = p;
                            c.1 = p;
                        }
                        ChildrenProcess::Plain => {
                            c.1 = p;
                        }
                        _ => {
                            // tag conditions
                        }
                    }
                }
                _ => {
                    // invalid chars
                }
            }
        }
        Ok(Extra {
            end: true,
            pos: cht.len(),
            tag: itag,
        })
    }

    /// generate palin text
    pub fn plain(h: &'static str) -> Tree {
        let mut attrs = HashMap::<&'static str, &'static str>::new();
        attrs.insert("text", h);

        Tree {
            tag: "plain",
            attrs: attrs,
            children: vec![],
        }
    }

    /// parse html tag
    pub fn tag(
        h: &'static str,
        pos: &mut usize,
    ) -> Result<(&'static str, HashMap<&'static str, &'static str>), Error> {
        let (mut t, mut k, mut v) = ((0, 0), (0, 0), (0, 0));
        let mut attrs = HashMap::<&'static str, &'static str>::new();
        let mut process = TagProcess::None;
        for (p, q) in h.chars().enumerate() {
            match q {
                '<' => {
                    process = TagProcess::Tag;
                    t.0 = p + 1;
                    t.1 = p + 1;
                }
                '>' => {
                    match process {
                        TagProcess::Tag => t.1 = p,
                        TagProcess::Attrs => {
                            if !&h[k.0..k.1].trim().is_empty() {
                                attrs.insert(&h[k.0..k.1].trim(), &h[v.0..v.1].trim());
                            }
                        }
                        _ => {}
                    }

                    *pos = *pos + p + 1;
                    return Ok((&h[t.0..t.1].trim(), attrs));
                }
                '"' => match process {
                    TagProcess::Quote => {
                        process = TagProcess::Attrs;
                        v.1 = p;
                    }
                    _ => {
                        v.0 = p + 1;
                        v.1 = p + 1;
                        process = TagProcess::Quote;
                    }
                },
                '=' => match process {
                    TagProcess::Attrs => k.1 = p,
                    _ => {
                        return Err(Error::DeserializeHtmlError(format!(
                            "html tag parse failed: {}, html: {}",
                            &h[t.0..t.1],
                            &h
                        )))
                    }
                },
                x if x.is_whitespace() => match process {
                    TagProcess::Tag => {
                        if h[t.0..t.1].trim().is_empty() {
                            t.1 = p;
                        } else {
                            process = TagProcess::Attrs;
                            k.0 = p + 1;
                            k.1 = p + 1;
                        }
                    }
                    TagProcess::Quote => {
                        v.1 = p;
                    }
                    TagProcess::Attrs => {
                        if (k.1 - k.0 != 0) && (v.1 - v.0 != 0) {
                            attrs.insert(&h[k.0..k.1].trim(), &h[v.0..v.1].trim());
                            k.0 = p;
                            k.1 = p;
                        }
                    }
                    _ => {}
                },
                x if !x.is_whitespace() => match process {
                    TagProcess::Tag => {
                        t.1 = p + 1;
                    }
                    TagProcess::Quote => {
                        v.1 = p;
                    }
                    TagProcess::Attrs => {
                        if v.0 == 0 {
                            k.1 = p;
                        } else {
                            v.1 = p;
                        }
                    }
                    _ => {}
                },
                _ => {
                    return Err(Error::DeserializeHtmlError(format!(
                        "html tag parse failed: {}, html: {}, char: {}",
                        &h[t.0..t.1],
                        &h,
                        &q
                    )))
                }
            }
        }

        Err(Error::DeserializeHtmlError(format!(
            "html tag parse failed: {}, html: {}",
            &h[t.0..t.1],
            &h
        )))
    }
}
