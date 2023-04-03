use std::collections::HashMap;

pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }

    pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(&mut self) -> Result<Request, String> {
        let Some(url) = self.url.take() else {
            return Err("No url".to_string());
        };
        let method = self.method.take().unwrap_or("GET".to_string());

        Ok(Request {
            url,
            method,
            headers: self.headers.clone(),
            body: self.body.take(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::RequestBuilder;

    #[test]
    fn builder_test() {
        let req = RequestBuilder::default()
            .url("http://some-link.com/task/111")
            .build();

        assert!(req.is_ok());
        assert_eq!("GET", req.unwrap().method);
    }
}
