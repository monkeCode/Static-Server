
#[derive(Copy, Clone)]
pub enum HttpCode
{
    Ok=200,
    Created = 201,
    Accepted = 202,
    NonAuthorititaveInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    MultipleChoises = 300,
    MovedPermamently = 301,
    Found = 302,
    SeeOthers = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnvailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505

}

impl TryFrom<u16> for HttpCode {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        unsafe { 
            let res = std::mem::transmute::<u16, HttpCode>(value);
            Ok(res)
        }
    }
}

#[derive(Copy, Clone)]
pub enum ContentType {
    Html,
    Css,
    Js,
    Png,
    Jpg,
    Gif,
    Pdf,
    Txt,
    Svg,
    Xml,
    Tiff,
}

impl ContentType {

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Html => "text/html",
            Self::Css => "text/css",
            Self::Js => "application/javascript",
            Self::Png => "image/png",
            Self::Jpg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::Png => "application/pdf",
            Self::Txt => "text/plain",
            Self::Svg => "image/svg+xml",
            Self::Tiff => "image/tiff",
            Self::Xml => "text/xml",
            //TODO: write all content types
            _ => "application/octet-stream",
        }
    }
    
    pub fn from_str(mime_type:&str) -> Self
    {
        match mime_type {
            "text/html" => ContentType::Html,
            "text/css" => ContentType::Css,
            "application/javascript" => ContentType::Js,
            "image/png" => ContentType::Png,
            "image/jpeg" => ContentType::Jpg,
            "image/gif" => ContentType::Gif,
            "application/pdf" => ContentType::Pdf,
            "text/plain" => ContentType::Txt,
            "image/svg+xml" => ContentType::Svg,
            "image/tiff" => ContentType::Tiff,
            "text/xml" => ContentType::Xml,
            _ => panic!("Not implemented"),
    }
}

    pub fn from_extension(extension: &str) ->Self
    {
            match extension {
                "html" | "htm" => Self::Html,
                "css" => Self::Css,
                "js" => Self::Js,
                "png" => Self::Png,
                "jpg" | "jpeg" => Self::Jpg,
                "gif" => Self::Gif,
                "pdf" => Self::Pdf,
                "txt" => Self::Txt,
                "xml" => Self::Xml,
                "svg" => Self::Svg,
                "tiff" => Self::Tiff,
                //TODO: write all file extensions
                _ => panic!("Type not found"),
            }
        }
}
