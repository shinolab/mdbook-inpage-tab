mod tab;

use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
    errors::{Error, Result},
};

pub struct InPageTab;

impl InPageTab {
    pub fn new() -> InPageTab {
        InPageTab
    }
}

impl Preprocessor for InPageTab {
    fn name(&self) -> &str {
        "mdbook-inpage-tab"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let re = regex::Regex::new(r"\{\{\s*#tabs\s*\}\}([\s\S]*?)\{\{\s*#endtabs\s*\}\}")
                    .unwrap();
                let mut new = String::with_capacity(chapter.content.len());
                let mut last_match = 0;
                let mut tabs_id = 0;
                for caps in re.captures_iter(&chapter.content) {
                    let m = caps.get(0).unwrap();
                    new.push_str(&chapter.content[last_match..m.start()]);
                    let rep = {
                        let tabs = tab::Tab::parse(caps.get(1).unwrap().as_str());

                        let mut rep = String::new();
                        rep.push_str(r#"<div class="mdbook-inpage-tabs">"#);

                        for tab in &tabs {
                            rep.push_str(&format!(
                                r#"<input id="{0}-tab-{1}" type="radio" class="mdbook-inpage-tab" name="tab-{1}"{2}>
                                <label class="mdbook-inpage-tab-item" num-tabs={3} for="{0}-tab-{1}">{0}</label>"#,
                                tab.name,
                                tabs_id,
                                if tab.id == 0 { " checked" } else { "" },
                                tabs.len()
                            ));
                        }

                        for tab in tabs {
                            rep.push_str(&format!(r#"<div class="mdbook-inpage-tab-content" id={}-content>"#, tab.name));
                            rep.push_str("\n\n");
                            rep.push_str(&tab.contents);
                            rep.push_str("\n\n");
                            rep.push_str("</div>");
                        }

                        rep.push_str("</div>");

                        rep
                    };
                    new.push_str(&rep);
                    last_match = m.end();
                    tabs_id += 1;
                }
                new.push_str(&chapter.content[last_match..]);
                chapter.content = new;
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}
