
pub mod get {

    use super::build_url;

   pub fn runs(pipline_id: &String) -> String {
        format!("{}?pipelineId={}",build_url("runs"),&pipline_id)
    }
}

fn build_url(endpoint: &str) -> String {
    format!("https://app.wercker.com/api/{}/{}",::API_VERSION, endpoint)
}
