use l10nregistry::registry::L10nRegistry;
use l10nregistry::testing::TestFileFetcher;
use unic_langid::LanguageIdentifier;

const FTL_RESOURCE_TOOLKIT: &str = "toolkit/global/textActions.ftl";
const FTL_RESOURCE_BROWSER: &str = "branding/brand.ftl";

#[test]
fn test_generate_sources_for_file() {
    let fetcher = TestFileFetcher::new();
    let en_us: LanguageIdentifier = "en-US".parse().unwrap();

    let fs1 = fetcher.get_test_file_source("toolkit", vec![en_us.clone()], "toolkit/{locale}");
    let fs2 = fetcher.get_test_file_source("browser", vec![en_us.clone()], "browser/{locale}");

    let mut reg = L10nRegistry::default();
    reg.register_sources(vec![fs1, fs2]).unwrap();

    {
        let lock = reg.lock();

        let toolkit = lock.get_source("toolkit").unwrap();
        let browser = lock.get_source("browser").unwrap();

        let mut i = lock.generate_sources_for_file(&en_us, FTL_RESOURCE_TOOLKIT);

        assert_eq!(i.next(), Some(toolkit));
        assert_eq!(i.next(), Some(browser));
        assert_eq!(i.next(), None);

        assert!(browser
            .fetch_file_sync(&en_us, FTL_RESOURCE_TOOLKIT)
            .is_none());

        let mut i = lock.generate_sources_for_file(&en_us, FTL_RESOURCE_TOOLKIT);
        assert_eq!(i.next(), Some(toolkit));
        assert_eq!(i.next(), None);

        assert!(toolkit
            .fetch_file_sync(&en_us, FTL_RESOURCE_TOOLKIT)
            .is_some());

        let mut i = lock.generate_sources_for_file(&en_us, FTL_RESOURCE_TOOLKIT);
        assert_eq!(i.next(), Some(toolkit));
        assert_eq!(i.next(), None);
    }
}

#[test]
fn test_generate_bundles_for_lang_sync() {
    let fetcher = TestFileFetcher::new();
    let en_us: LanguageIdentifier = "en-US".parse().unwrap();
    let fs1 = fetcher.get_test_file_source("toolkit", vec![en_us.clone()], "toolkit/{locale}");
    let fs2 = fetcher.get_test_file_source("browser", vec![en_us.clone()], "browser/{locale}");

    let mut reg = L10nRegistry::default();
    reg.register_sources(vec![fs1, fs2]).unwrap();

    let paths = vec![FTL_RESOURCE_TOOLKIT.into(), FTL_RESOURCE_BROWSER.into()];
    let mut i = reg.generate_bundles_for_lang_sync(en_us.clone(), paths);

    assert!(i.next().is_some());
    assert!(i.next().is_none());
}

#[test]
fn test_generate_bundles_sync() {
    let fetcher = TestFileFetcher::new();
    let en_us: LanguageIdentifier = "en-US".parse().unwrap();
    let fs1 = fetcher.get_test_file_source("toolkit", vec![en_us.clone()], "toolkit/{locale}");
    let fs2 = fetcher.get_test_file_source("browser", vec![en_us.clone()], "browser/{locale}");

    let mut reg = L10nRegistry::default();
    reg.register_sources(vec![fs1, fs2]).unwrap();

    let paths = vec![FTL_RESOURCE_TOOLKIT.into(), FTL_RESOURCE_BROWSER.into()];
    let lang_ids = vec![en_us];
    let mut i = reg.generate_bundles_sync(lang_ids, paths);

    assert!(i.next().is_some());
    assert!(i.next().is_none());
}

#[tokio::test]
async fn test_generate_bundles_for_lang() {
    use futures::stream::StreamExt;

    let fetcher = TestFileFetcher::new();
    let en_us: LanguageIdentifier = "en-US".parse().unwrap();
    let fs1 = fetcher.get_test_file_source("toolkit", vec![en_us.clone()], "toolkit/{locale}");
    let fs2 = fetcher.get_test_file_source("browser", vec![en_us.clone()], "browser/{locale}");

    let mut reg = L10nRegistry::default();
    reg.register_sources(vec![fs1, fs2]).unwrap();

    let paths = vec![FTL_RESOURCE_TOOLKIT.into(), FTL_RESOURCE_BROWSER.into()];
    let mut i = reg.generate_bundles_for_lang(en_us, paths);

    assert!(i.next().await.is_some());
    assert!(i.next().await.is_none());
}

#[tokio::test]
async fn test_generate_bundles() {
    use futures::stream::StreamExt;

    let fetcher = TestFileFetcher::new();
    let en_us: LanguageIdentifier = "en-US".parse().unwrap();
    let fs1 = fetcher.get_test_file_source("toolkit", vec![en_us.clone()], "toolkit/{locale}");
    let fs2 = fetcher.get_test_file_source("browser", vec![en_us.clone()], "browser/{locale}");

    let mut reg = L10nRegistry::default();
    reg.register_sources(vec![fs1, fs2]).unwrap();

    let paths = vec![FTL_RESOURCE_TOOLKIT.into(), FTL_RESOURCE_BROWSER.into()];
    let langs = vec![en_us];
    let mut i = reg.generate_bundles(langs, paths);

    assert!(i.next().await.is_some());
    assert!(i.next().await.is_none());
}
