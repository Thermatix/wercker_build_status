
pub mod get {

    use super::build_url;

   pub fn runs(pipline_id: &String,author: &String) -> String {
        format!("{}?pipelineId={}&author={}",build_url("runs"),&pipline_id, &author)
    }
}

fn build_url(endpoint: &str) -> String {
    format!("https://app.wercker.com/api/{}/{}",::API_VERSION, endpoint)
}
