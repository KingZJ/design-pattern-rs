pub struct Request {
    pub body: String,
}

pub struct Response {
    pub body: String,
}

pub struct IndexController {}

impl IndexController {
    fn index(request: &Request) -> Response {
        Response {
            body: request.body.clone(),
        }
    }
}

pub trait Middleware<Req, Resp> {
    fn handle(&self, request: &mut Req);
}

pub struct Pipeline<Req, Resp> {
    middleware: Box<dyn Middleware<Req, Resp>>,
    next: Option<Box<Pipeline<Req, Resp>>>,
}

impl<Req, Resp> Pipeline<Req, Resp> {
    pub fn new(
        middleware: Box<dyn Middleware<Req, Resp>>,
        next: Option<Box<Pipeline<Req, Resp>>>,
    ) -> Self {
        Self { middleware, next }
    }
}

impl Pipeline<Request, Response> {
    pub fn handle(&self, request: &mut Request) -> Response {
        self.middleware.handle(request);

        self.next
            .as_ref()
            .map(|f| f.handle(request))
            .unwrap_or_else(|| IndexController::index(request))
        // match self.next {
        //     None => IndexController::index(request),
        //     Some(ref next) => next.handle(request),
        // }
    }
}

pub struct LoginMiddleware {}

impl Middleware<Request, Response> for LoginMiddleware {
    fn handle(&self, request: &mut Request) {
        println!("user login is valid");
        request.body = format!("{}-login.valid", request.body);
    }
}

pub struct AuthMiddleware {}

impl Middleware<Request, Response> for AuthMiddleware {
    fn handle(&self, request: &mut Request) {
        println!("user auth is valid");
        request.body = format!("{}-auth.valid", request.body);
    }
}

#[cfg(test)]
mod test {
    use super::{AuthMiddleware, LoginMiddleware, Pipeline, Request};

    #[test]
    fn test() {
        let login = LoginMiddleware {};
        let auth = AuthMiddleware {};

        let pipeline1 = Pipeline::new(Box::new(auth), None);
        let pipeline2 = Pipeline::new(Box::new(login), Some(Box::new(pipeline1)));

        let mut request = Request {
            body: "init".to_string(),
        };
        assert_eq!(
            "init-login.valid-auth.valid",
            pipeline2.handle(&mut request).body
        );
    }
}
