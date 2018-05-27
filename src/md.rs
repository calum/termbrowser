//! HTML renderer that takes an iterator of events as input.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;

use ansi_term::Style;
use pulldown_cmark::Event::{End, FootnoteReference, HardBreak, Html, InlineHtml, SoftBreak, Start,
                            Text};
use pulldown_cmark::{Event, Tag};

struct Ctx<'b, I> {
    iter: I,
    buf: &'b mut String,
    base_style: Style,
    style_stack: Vec<Style>,
    list_count: Option<usize>,
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> Ctx<'b, I> {
    fn fresh_line(&mut self) {
        if !(self.buf.is_empty() || self.buf.ends_with('\n')) {
            self.buf.push('\n');
        }
    }

    pub fn run(&mut self) {
        let mut numbers = HashMap::new();
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => self.start_tag(tag, &mut numbers),
                End(tag) => self.end_tag(tag),
                Text(text) => self.buf.push_str(&text),
                Html(html) | InlineHtml(html) => self.buf.push_str(&html),
                SoftBreak => self.buf.push('\n'),
                HardBreak => self.buf.push_str("<br />\n"),
                FootnoteReference(_) => unimplemented!(),
            }
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>, numbers: &mut HashMap<Cow<'a, str>, usize>) {
        match tag {
            Tag::Paragraph => {
                self.fresh_line();
            }
            Tag::Rule => {
                self.fresh_line();
                self.buf.push_str("<hr />\n")
            }
            Tag::Header(level) => {
                self.fresh_line();
            }
            Tag::Table(alignments) => {
                unimplemented!();
            }
            Tag::TableHead => {
                unimplemented!();
            }
            Tag::TableRow => {
                unimplemented!();
            }
            Tag::TableCell => {
                unimplemented!();
            }
            Tag::BlockQuote => {
                self.fresh_line();
                self.buf.push_str("\n");
            }
            Tag::CodeBlock(info) => {
                self.fresh_line();
                let lang = info.split(' ').next().unwrap();
                if lang.is_empty() {
                    self.buf.push_str("<code>");
                } else {
                    self.buf.push_str("<code class=\"language-");
                    self.buf.push_str(lang);
                    self.buf.push_str("\">");
                }
            }
            Tag::List(count) => {
                self.fresh_line();
                self.list_count = count;
            }
            Tag::Item => {
                self.fresh_line();
                match self.list_count {
                    Some(c) => {
                        write!(self.buf, "{}. ", c);
                    }
                    None => self.buf.push_str("* "),
                }
            }
            Tag::Emphasis => {
                let new_style = self.style_stack
                    .last()
                    .unwrap_or(&self.base_style)
                    .clone()
                    .italic();
                self.style_stack.push(new_style)
            }
            Tag::Strong => {
                let new_style = self.style_stack
                    .last()
                    .unwrap_or(&self.base_style)
                    .clone()
                    .bold();
                self.style_stack.push(new_style)
            }
            Tag::Code => self.buf.push('\t'),
            Tag::Link(dest, title) => {
                self.buf.push_str("<a href=\"");
                self.buf.push_str(&dest);
                if !title.is_empty() {
                    self.buf.push_str("\" title=\"");
                    self.buf.push_str(&title);
                }
                self.buf.push_str("\">");
            }
            Tag::Image(dest, title) => {
                self.buf.push_str("<img src=\"");
                self.buf.push_str(&dest);
                self.buf.push_str("\" alt=\"");
                self.raw_text(numbers);
                if !title.is_empty() {
                    self.buf.push_str("\" title=\"");
                    self.buf.push_str(&title);
                }
                self.buf.push_str("\" />")
            }
            Tag::FootnoteDefinition(_) => unimplemented!(),
        }
    }

    fn end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.buf.push('\n'),
            Tag::Rule => (),
            Tag::Header(level) => {
                self.buf.push('\n');
            }
            Tag::Table(_) => unimplemented!(),
            Tag::TableHead => unimplemented!(),
            Tag::TableRow => unimplemented!(),
            Tag::TableCell => unimplemented!(),
            Tag::BlockQuote => (),
            Tag::CodeBlock(_) => (),
            Tag::List(Some(_)) => (),
            Tag::List(None) => (),
            Tag::Item => (),
            Tag::Emphasis => {
                self.style_stack.pop();
            }
            Tag::Strong => {
                self.style_stack.pop();
            }
            Tag::Code => (),
            Tag::Link(_, _) => (),
            Tag::Image(_, _) => (),
            Tag::FootnoteDefinition(_) => unimplemented!(),
        }
    }

    // run raw text, consuming end tag
    fn raw_text<'c>(&mut self, numbers: &'c mut HashMap<Cow<'a, str>, usize>) {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Text(text) => self.buf.push_str(&text),
                Html(_) => (),
                InlineHtml(html) => self.buf.push_str(&html),
                SoftBreak | HardBreak => self.buf.push(' '),
                FootnoteReference(_) => unimplemented!(),
            }
        }
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut ascii_buf = String::new();
/// md::push_ascii(&mut ascii_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn push_cli<'a, I: Iterator<Item = Event<'a>>>(buf: &mut String, iter: I) {
    let mut ctx = Ctx {
        iter,
        buf,
        base_style: Style::new(),
        style_stack: Vec::default(),
        list_count: None,
    };
    ctx.run();
}
