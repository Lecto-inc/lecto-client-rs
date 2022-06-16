use reqwest::Url;

pub fn join_url<T: AsRef<str>>(base_url: &str, paths: &[T]) -> anyhow::Result<Url> {
    let mut url = Url::parse(base_url.strip_suffix('/').unwrap_or(base_url))?;
    paths.iter().for_each(|path| {
        url.path_segments_mut().unwrap().push(path.as_ref());
    });
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("https://hoge.com/api/v1", &["debtors", "hoge.json"], "https://hoge.com/api/v1/debtors/hoge.json")]
    #[case("https://hoge.com/api/v1/", &["debtors", "hoge.json"], "https://hoge.com/api/v1/debtors/hoge.json")]
    fn test_join_url<T: AsRef<str>>(
        #[case] base_url: &str,
        #[case] paths: &[T],
        #[case] expected: &str,
    ) -> anyhow::Result<()> {
        let url = join_url(base_url, paths)?;
        let expect = Url::parse(expected)?;

        assert_eq!(url, expect);

        Ok(())
    }
}
