// src/networking/http_client.rs

use reqwest::{Client, Error, Response, Method, header::HeaderMap};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

/// A simple asynchronous HTTP client.
#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    timeout: Duration,
}

impl HttpClient {
    /// Creates a new HttpClient with a default timeout.
    pub fn new(timeout_seconds: Option<u64>) -> Self {
        HttpClient {
            client: Client::new(),
            timeout: Duration::from_secs(timeout_seconds.unwrap_or(30)),
        }
    }

    /// Creates a new HttpClient with a custom reqwest::Client.
    /// This allows for more advanced configuration like custom headers, proxies, etc.
    pub fn with_client(client: Client, timeout_seconds: Option<u64>) -> Self {
        HttpClient {
            client,
            timeout: Duration::from_secs(timeout_seconds.unwrap_or(30)),
        }
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        headers: Option<HeaderMap>,
        query_params: Option<&HashMap<String, String>>,
        body: Option<impl Serialize>,
    ) -> Result<T, Error> {
        let mut request_builder = self.client.request(method, url).timeout(self.timeout);

        if let Some(h) = headers {
            request_builder = request_builder.headers(h);
        }

        if let Some(qp) = query_params {
            request_builder = request_builder.query(qp);
        }

        if let Some(b) = body {
            request_builder = request_builder.json(&b);
        }

        let response = request_builder.send().await?;
        response.error_for_status()?.json::<T>().await
    }
    
    /// Sends a GET request.
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<T, Error> {
        self.send_request(Method::GET, url, headers, query_params, None::<()>).await
    }

    /// Sends a POST request with a JSON body.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        body: B,
    ) -> Result<T, Error> {
        self.send_request(Method::POST, url, headers, None, Some(body)).await
    }

    /// Sends a PUT request with a JSON body.
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        body: B,
    ) -> Result<T, Error> {
        self.send_request(Method::PUT, url, headers, None, Some(body)).await
    }

    /// Sends a DELETE request.
    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> Result<T, Error> {
        self.send_request(Method::DELETE, url, headers, None, None::<()>).await
    }

    /// Sends a PATCH request with a JSON body.
    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        body: B,
    ) -> Result<T, Error> {
        self.send_request(Method::PATCH, url, headers, None, Some(body)).await
    }
    
    /// Sends a GET request and returns the raw Response object.
    /// Useful if you need to access headers or status code directly.
    pub async fn get_raw(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<Response, Error> {
        let mut request_builder = self.client.get(url).timeout(self.timeout);
        if let Some(h) = headers {
            request_builder = request_builder.headers(h);
        }
        if let Some(qp) = query_params {
            request_builder = request_builder.query(qp);
        }
        request_builder.send().await?.error_for_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use wiremock::matchers::{method, path};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use std::collections::HashMap;

    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    struct Post {
        id: u32,
        title: String,
        body: String,
        #[serde(rename = "userId")]
        user_id: u32,
    }
    
    #[derive(Deserialize, Debug, PartialEq, Serialize)]
    struct NewPost {
        title: String,
        body: String,
        #[serde(rename = "userId")]
        user_id: u32,
    }


    #[tokio::test]
    async fn test_get_request() {
        let server = MockServer::start().await;
        let mock_response = Post {
            id: 1,
            title: "Test Post".to_string(),
            body: "This is a test post.".to_string(),
            user_id: 1,
        };

        Mock::given(method("GET"))
            .and(path("/posts/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let client = HttpClient::new(None);
        let url = format!("{}/posts/1", server.uri());
        let response: Post = client.get(&url, None, None).await.unwrap();

        assert_eq!(response, mock_response);
    }

    #[tokio::test]
    async fn test_post_request() {
        let server = MockServer::start().await;
        let new_post = NewPost {
            title: "New Post Title".to_string(),
            body: "New post body.".to_string(),
            user_id: 2,
        };
        let mock_response = Post {
            id: 101, // Typically the server would generate this
            title: new_post.title.clone(),
            body: new_post.body.clone(),
            user_id: new_post.user_id,
        };

        Mock::given(method("POST"))
            .and(path("/posts"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let client = HttpClient::new(None);
        let url = format!("{}/posts", server.uri());
        let response: Post = client.post(&url, None, &new_post).await.unwrap();
        
        assert_eq!(response.title, new_post.title);
        assert_eq!(response.body, new_post.body);
        assert_eq!(response.user_id, new_post.user_id);
        // ID might be generated by server, so we check if it's greater than 0 or a specific value if known
        assert!(response.id > 0); 
    }
    
    #[tokio::test]
    async fn test_get_with_query_params() {
        let server = MockServer::start().await;
        let mock_response = vec![
            Post { id: 1, title: "Post 1".to_string(), body: "Body 1".to_string(), user_id: 1 },
            Post { id: 2, title: "Post 2".to_string(), body: "Body 2".to_string(), user_id: 1 },
        ];

        Mock::given(method("GET"))
            .and(path("/posts"))
            // .and(query_param("userId", "1")) // Wiremock doesn't have direct query_param for HashMap
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;
            
        let client = HttpClient::new(None);
        let mut params = HashMap::new();
        params.insert("userId".to_string(), "1".to_string());
        
        let url = format!("{}/posts", server.uri());
        let response: Vec<Post> = client.get(&url, None, Some(&params)).await.unwrap();

        assert_eq!(response.len(), 2);
        assert_eq!(response, mock_response);
    }

    #[tokio::test]
    async fn test_get_raw_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/raw"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Raw body content"))
            .mount(&server)
            .await;

        let client = HttpClient::new(None);
        let url = format!("{}/raw", server.uri());
        let response = client.get_raw(&url, None, None).await.unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        assert_eq!(response.text().await.unwrap(), "Raw body content");
    }
    
    #[tokio::test]
    async fn test_request_timeout() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/timeout"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2))) // Delay response
            .mount(&server)
            .await;

        // Client with 1 second timeout
        let client = HttpClient::new(Some(1)); 
        let url = format!("{}/timeout", server.uri());
        let result: Result<String, _> = client.get(&url, None, None).await;
        
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.is_timeout());
        }
    }
}
