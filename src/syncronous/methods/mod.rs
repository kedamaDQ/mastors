pub mod api;

use crate::{
    Result,
    entities::Entity,
};

pub(crate) use private::{
    FileFormInternal,
    MethodInternal,
    UploadInternal,
};

pub trait Method<'a, E: 'a + Entity>: MethodInternal<'a, E> {
    fn send(&'a self) -> Result<E> {
        self.send_internal()
    }
}

pub(crate) mod private {
    use reqwest::blocking::{
        RequestBuilder,
        Response,
        multipart,
    };
    use serde::Serialize;
    use crate::{
        Connection,
        Error,
        Result,
        entities::Entity,
        utils,
        methods::Method,
    };

    pub trait MethodInternal<'a, E: 'a + Entity>: std::marker::Sized + Serialize {

        const ENDPOINT: &'a str;

        fn connection(&'a self) -> &'a Connection;
    
        fn path(&'a self) -> String {
            Self::ENDPOINT.into()
        }
    
        fn authorization(&'a self) -> Option<&'a str> {
            None
        }
 
        fn send_internal(&self) -> Result<E>;

        fn get(&'a self) -> Result<E> {
            Ok(
                send_request(
                    build_request(self, reqwest::Method::GET)?.query(&self)
                )?
                .json::<E>()?
            )
        }
    
        fn post(&'a self) -> Result<E> {
            Ok(
                send_request(
                    build_request(self, reqwest::Method::POST)?.json(&self)
                )?
                .json::<E>()?
            )
        }

        fn put(&'a self) -> Result<E> {
            Ok(
                send_request(
                    build_request(self, reqwest::Method::PUT)?.json(&self)
                )?
                .json::<E>()?
            )
        }

        fn delete(&'a self) -> Result<E> {
            Ok(
                send_request(
                    build_request(self, reqwest::Method::DELETE)?.json(&self)
                )?
                .json::<E>()?
            )
        }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
    pub struct FileFormInternal<'a> {
        pub form_name: &'a str,
        pub file_name: &'a str,
    }
    
    pub trait UploadInternal<'a, E: 'a + Entity>: Method<'a, E> {
        fn file_form(&self) -> FileFormInternal;
    
        fn text_forms(&self) -> Vec<(&str, String)>;
    
        fn post_with_media(&'a self) -> Result<E> {
            use std::convert::TryFrom;
            use std::fs::File;
            use std::io::prelude::*;
            use multipart::{ Form, Part };
    
            // The documentation of reqwest says that `body() can receive std :: fs :: File`, but isn't code implement From<File>?
            let multipart = self.text_forms().iter().fold(Form::new(), |mp, (name, value)| {
                mp.part((*name).to_owned(), Part::text((*value).to_owned()))
            });
    
            let mut file = File::open(self.file_form().file_name)?;
            let meta = file.metadata()?;
    
            if !meta.is_file() {
                return Err(
                    Error::NotFileError(self.file_form().file_name.to_owned())
                );
            }
    
            if !meta.len() == 0 {
                return Err(
                    Error::BlankFileError(self.file_form().file_name.to_owned())
                );
            }
    
            let mut buf: Vec<u8> = match usize::try_from(meta.len()) {
                Ok(len) => Vec::with_capacity(len),
                Err(_) => Vec::new(),
            };
    
            file.read_to_end(&mut buf)?;
    
            let multipart = multipart.part(
                self.file_form().form_name.to_owned(),
                Part::bytes(buf).file_name(self.file_form().file_name.to_owned())
            );
    
            Ok(
                send_request(
                    build_request(self, reqwest::Method::POST)?.multipart(multipart)
                )?
                .json::<E>()?
            )
        }
    }

    fn build_request<'a, E: Entity + 'a, M: MethodInternal<'a, E>>(
        implementer: &'a M,
        method: reqwest::Method
    ) -> crate::Result<RequestBuilder> {
    
        let mut req = implementer.connection().client().request(
            method,
            implementer.connection().url(&implementer.path())?
        );
    
        if let Some(ac) = implementer.authorization() {
            req = req.bearer_auth(ac);
        }
        Ok(req)
    }
    
    fn send_request(rb: RequestBuilder) -> crate::Result<Response> {
        utils::extract_response(rb.send()?)
    }
}


