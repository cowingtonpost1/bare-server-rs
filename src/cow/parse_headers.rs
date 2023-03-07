use std::str::FromStr;

use crate::bare_errors::BareError;
use actix_http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Uri,
};
use actix_web::HttpRequest;
use awc::ClientRequest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct BareRemote {
    pub host: String,
    pub port: i32,
    pub path: String,
    pub protocol: String,
}

pub trait ToUri {
    fn to_uri(&self) -> Option<Uri>;
}

pub trait ApplyHeaders {
    fn apply_headers(&self, _: &mut ClientRequest);
}

impl ApplyHeaders for HeaderMap {
    fn apply_headers(&self, builder: &mut ClientRequest) {
        builder.headers_mut().clear();
        for header in self {
            let (h_name, h_value) = header;
            builder
                .headers_mut()
                .insert(h_name.clone(), h_value.clone());
        }
    }
}

impl ToUri for BareRemote {
    fn to_uri(&self) -> Option<Uri> {
        let uri = Uri::builder()
            .scheme(self.protocol.as_str())
            .authority(self.host.to_owned() + ":" + &(self.port.to_string().to_owned()))
            //.authority(self.host.to_owned())
            .path_and_query(self.path.clone())
            .build();

        println!("{:#?}, Protocol: {}", uri, self.protocol);

        uri.ok()
    }
}

pub struct BareHeaderData {
    pub remote: BareRemote,
    pub headers: HeaderMap,
}

impl Default for BareHeaderData {
    fn default() -> Self {
        BareHeaderData {
            remote: Default::default(),
            headers: HeaderMap::new(),
        }
    }
}

pub fn get_header<'a>(req: &'a HttpRequest, name: &str) -> Option<&'a str> {
    req.headers().get(name)?.to_str().ok()
}

const VALID_PROTOCOLS: [&str; 4] = ["http:", "https:", "ws:", "wss:"];

pub fn parse_headers(req: &HttpRequest) -> Result<BareHeaderData, BareError> {
    let mut remote: BareHeaderData = Default::default();
    for remote_prop in ["host", "port", "protocol", "path"] {
        println!("RemoteProp: {}", &remote_prop);
        let header = "x-bare-".to_owned() + remote_prop;

        if let Some(value) = get_header(&req, &header) {
            println!("Some {}", &header);
            match remote_prop {
                "host" => {
                    remote.remote.host = value.to_owned();
                }
                "path" => {
                    remote.remote.path = value.to_owned();
                }
                "port" => {
                    if let Some(val) = value.to_string().parse::<i32>().ok() {
                        println!("Port {} ok", value);
                        remote.remote.port = val;
                    } else {
                        return Err(BareError {
                            code: "INVALID_BARE_HEADER".to_string(),
                            id: "request.headers.".to_owned() + &header,
                            message: "Header was not a valid integer.".to_string(),
                        });
                    }
                }
                "protocol" => {
                    println!("Header Protocol: {}", value);
                    if !VALID_PROTOCOLS.contains(&value) {
                        return Err(BareError {
                            code: "INVALID_BARE_HEADER".to_string(),
                            id: "request.headers.".to_owned() + &header,
                            message: "Header was invalid".to_string(),
                        });
                    }
                    remote.remote.protocol = value.to_owned();
                    remote.remote.protocol.pop();
                }
                _ => unreachable!(),
            }
        } else {
            println!("Header missing: {}", &header);
            return Err(BareError {
                code: "MISSING_BARE_HEADER".to_string(),
                id: ("request.headers.".to_owned() + &header).to_string(),
                message: "Header was not specified.".to_string(),
            });
        }
    }

    if let Some(value) = get_header(&req, "x-bare-headers") {
        let json: Map<String, Value> = match serde_json::from_str(value).ok() {
            Some(x) => x,
            None => {
                return Err(BareError {
                    code: "INVALID_BARE_HEADER".to_string(),
                    id: "bare.headers.x-bare-forward-headers".to_string(),
                    message: "Header was not an array of Strings.".to_string(),
                })
            }
        };
        println!("Bare Headers: {:#?}", json);

        for (k, v) in json {
            match v {
                Value::String(v) => {
                    remote.headers.insert(
                        HeaderName::from_str(&k).unwrap(),
                        HeaderValue::from_str(&v).unwrap(),
                    );
                    ()
                }
                Value::Array(v) => {
                    let mut array: Vec<String> = vec![];

                    for item in v {
                        if let Value::String(it) = item {
                            array.push(it.to_owned());
                        } else {
                            return Err(BareError {
                                code: "INVALID_BARE_HEADER".to_string(),
                                id: "bare.headers.x-bare-headers".to_string(),
                                message: "Header was not a String.".to_string(),
                            });
                        }
                    }
                    remote.headers.insert(
                        HeaderName::from_str(&k).unwrap(),
                        HeaderValue::from_str(&serde_json::to_string(&array).unwrap()).unwrap(),
                    );
                }
                _ => {
                    return Err(BareError {
                        code: "INVALID_BARE_HEADER".to_string(),
                        id: "bare.headers.x-bare-headers".to_string(),
                        message: "Header was not a String.".to_string(),
                    });
                }
            }
        }
    } else {
        return Err(BareError {
            code: "MISSING_BARE_HEADER".to_string(),
            id: "request.headers.x-bare-headers".to_string(),
            message: "Header was not specified.".to_string(),
        });
    }

    if let Some(value) = get_header(&req, "x-bare-forward-headers") {
        let json: Vec<Value> = match serde_json::from_str(value).ok() {
            Some(n) => n,
            None => {
                return Err(BareError {
                    code: "INVALID_BARE_HEADER".to_string(),
                    id: "bare.headers.x-bare-forward-headers".to_string(),
                    message: "Header was not an array of Strings.".to_string(),
                })
            }
        };
        println!("Forward Headers: {:#?}", json);
        for cow in json {
            if let Value::String(cow) = cow {
                if let Some(req_header) = get_header(&req, &cow) {
                    remote.headers.insert(
                        HeaderName::from_str(&cow).unwrap(),
                        HeaderValue::from_str(&req_header).unwrap(),
                    );
                }
            }
        }
    } else {
        return Err(BareError {
            code: "MISSING_BARE_HEADER".to_string(),
            id: "request.headers.x-bare-forward-headers".to_string(),
            message: "Header was not specified.".to_string(),
        });
    }
    return Ok(remote);
}
