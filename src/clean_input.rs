use ammonia::Builder;
use maplit::hashset;

pub fn sanitize_html(input: &str) -> String {
    
    let cleaner = Builder::default()
        .generic_attributes(hashset!["rel"])
        .link_rel(None)
        .clean(input);
    cleaner.to_string()
}


