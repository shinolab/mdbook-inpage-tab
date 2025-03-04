#[derive(Debug, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub name: String,
    pub contents: String,
}

impl Tab {
    pub fn parse(content: &str) -> Vec<Self> {
        let re_tab =
            regex::Regex::new(r"\{\{\s*#tab name=(.*?)\s*\}\}([\s\S]*?)\{\{\s*#endtab\s*\}\}")
                .unwrap();
        re_tab
            .captures_iter(content)
            .scan(0, |id, tab| {
                let name = tab.get(1).map(|name| name.as_str().to_string())?;
                let contents = if let Some(contents) = tab.get(2) {
                    contents.as_str().trim().to_string()
                } else {
                    String::new()
                };
                let tab = Tab {
                    id: *id,
                    name,
                    contents,
                };
                *id += 1;
                Some(tab)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let content = r"
{{ #tabs }}
{{ #tab name=tab1 }}
**Tab content 1**
{{ #endtab }}
{{ #tab name=tab2 }}
_Tab content 2-1_
_Tab content 2-2_
{{ #endtab }}
{{ #tab }}
~~Tab content 3~~
{{ #endtab }}
{{ #endtabs }}
";

        let expect = vec![
            Tab {
                id: 0,
                name: "tab1".to_string(),
                contents: "**Tab content 1**".to_string(),
            },
            Tab {
                id: 1,
                name: "tab2".to_string(),
                contents: "_Tab content 2-1_\n_Tab content 2-2_".to_string(),
            },
        ];

        let tabs = Tab::parse(content);
        assert_eq!(tabs, expect);
    }
}
