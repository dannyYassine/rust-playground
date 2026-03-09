use std::sync::Arc;

use axum::{
    Router,
    handler::Handler,
    routing::{delete, get, head, options, patch, post, put, trace},
};

use crate::AppState;

pub struct Route {
    router: Router<Arc<AppState>>,
}

impl Route {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    #[allow(dead_code)]
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, put(handler));
        self
    }
    #[allow(dead_code)]
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, patch(handler));
        self
    }
    #[allow(dead_code)]
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, delete(handler));
        self
    }
    #[allow(dead_code)]
    pub fn head<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, head(handler));
        self
    }
    #[allow(dead_code)]
    pub fn options<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, options(handler));
        self
    }
    #[allow(dead_code)]
    pub fn trace<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, Arc<AppState>>,
        T: 'static,
    {
        self.router = self.router.route(path, trace(handler));
        self
    }

    pub fn into_router(self) -> Router<Arc<AppState>> {
        self.router
    }
}
