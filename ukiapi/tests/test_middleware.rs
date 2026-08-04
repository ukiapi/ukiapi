use ukiapi::routing::middleware::MiddlewareExt;
use ukiapi::routing::api::UkiApi;

#[tokio::test]
async fn test_security_headers_middleware() {
    let _app = UkiApi::<()>::new().security_headers();
}
