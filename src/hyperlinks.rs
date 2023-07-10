extern crate linkify;

use linkify::{LinkFinder, LinkKind};

/// `find_links` will search the source str for any
/// hypertext links (URLs)
pub fn find_links(source: &str) -> Vec<String> {
    let mut link_finder = LinkFinder::new();
    link_finder.kinds(&[LinkKind::Url]);
    let linkify_urls: Vec<_> = link_finder.links(source).collect();
    let mut urls: Vec<String> = linkify_urls
        .iter()
        .map(|link| link.as_str().to_string())
        .collect();
    urls.sort();
    urls.dedup();
    urls
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_one_url() {
        let link = super::find_links("https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)");
        assert_eq!(link.len(), 1);
    }

    #[test]
    fn no_duplicate_urls() {
        let link = super::find_links("example.com, example.com, https://example.com");
        assert_eq!(link.len(), 1)
    }
}
