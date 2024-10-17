pub enum StatusCode {
    /// ##### Information responses
    /// ### 100 Continue
    /// This interim response indicates that the client should continue the request or ignore the response if the request is already finished.
    Continue = 100,
    /// ##### Information responses
    /// This code is sent in response to an Upgrade request header from the client and indicates the protocol the server is switching to.
    SwitchingProtocols,
    /// ##### Information responses
    /// (WebDAV) This code indicates that the server has received and is processing the request, but no response is available yet.
    Processing,
    /// ##### Information responses
    /// This status code is primarily intended to be used with the Link header, letting the user agent start preloading resources while the server prepares a response or preconnect to an origin from which the page will need resources.
    EarlyHints,
    /// ##### Successful responses
    /// ### 200 Ok
    /// The request succeeded. The result meaning of "success" depends on the HTTP method:
    ///
    /// GET: The resource has been fetched and transmitted in the message body.
    ///
    /// HEAD: The representation headers are included in the response without any message body.
    ///
    /// PUT or POST: The resource describing the result of the action is transmitted in the message body.
    ///
    /// TRACE: The message body contains the request message as received by the server.
    Ok,
    /// ##### Successful responses
    /// The request succeeded, and a new resource was created as a result. This is typically the response sent after POST requests, or some PUT requests.
    Created,
    /// ##### Successful responses
    /// The request has been received but not yet acted upon. It is noncommittal, since there is no way in HTTP to later send an asynchronous response indicating the outcome of the request. It is intended for cases where another process or server handles the request, or for batch processing.
    Accepted,
    /// ##### Successful responses
    /// This response code means the returned metadata is not exactly the same as is available from the origin server, but is collected from a local or a third-party copy. This is mostly used for mirrors or backups of another resource. Except for that specific case, the 200 OK response is preferred to this status.
    NonAuthoritativeInformation,
    /// ##### Successful responses
    /// There is no content to send for this request, but the headers may be useful. The user agent may update its cached headers for this resource with the new ones.
    NoContent,
    /// ##### Successful responses
    /// Tells the user agent to reset the document which sent this request.
    ResetContent,
    /// ##### Successful responses
    /// This response code is used when the Range header is sent from the client to request only part of a resource.
    PartialContent,
    /// ##### Successful responses
    /// (WebDAV) Conveys information about multiple resources, for situations where multiple status codes might be appropriate.
    MultiStatus,
    /// ##### Successful responses
    /// (WebDAV) Used inside a <dav:propstat> response element to avoid repeatedly enumerating the internal members of multiple bindings to the same collection.
    AlreadyReported,
    /// ##### Successful responses
    /// (HTTP Delta encoding) The server has fulfilled a GET request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance.
    IMUsed,
    /// ##### Successful responses
    /// The request has more than one possible response. The user agent or user should choose one of them. (There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick.)
    MultipleChoices,
    /// ##### Redirection responses
    /// The URL of the requested resource has been changed permanently. The new URL is given in the response.
    MovedPermanently,
    /// ##### Redirection responses
    /// This response code means that the URI of requested resource has been changed temporarily. Further changes in the URI might be made in the future. Therefore, this same URI should be used by the client in future requests.
    Found,
    /// ##### Redirection responses
    /// The server sent this response to direct the client to get the requested resource at another URI with a GET request.
    SeeOther,
    /// ##### Redirection responses
    /// This is used for caching purposes. It tells the client that the response has not been modified, so the client can continue to use the same cached version of the response.
    /// ##### Redirection responses
    NotModified,
    /// ##### Redirection responses
    /// Defined in a previous version of the HTTP specification to indicate that a requested response must be accessed by a proxy. It has been deprecated due to security concerns regarding in-band configuration of a proxy.
    UseProxyDeprecated,
    /// ##### Redirection responses
    /// The server sends this response to direct the client to get the requested resource at another URI with the same method that was used in the prior request. This has the same semantics as the 302 Found HTTP response code, with the exception that the user agent must not change the HTTP method used: if a POST was used in the first request, a POST must be used in the second request.
    TemporaryRedirect,
    /// ##### Redirection responses
    /// This means that the resource is now permanently located at another URI, specified by the Location: HTTP Response header. This has the same semantics as the 301 Moved Permanently HTTP response code, with the exception that the user agent must not change the HTTP method used: if a POST was used in the first request, a POST must be used in the second request.
    PermanentRedirect,
    /// ##### Client error responses
    /// ### 400 BadRequest
    /// The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).
    BadRequest,
    /// ##### Client error responses
    /// Although the HTTP standard specifies "unauthorized", semantically this response means "unauthenticated". That is, the client must authenticate itself to get the requested response.
    Unauthorized,
    /// ##### Client error responses
    /// This response code is reserved for future use. The initial aim for creating this code was using it for digital payment systems, however this status code is used very rarely and no standard convention exists.
    PaymentRequiredExperimental,
    /// ##### Client error responses
    /// The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.
    Forbidden,
    /// ##### Client error responses
    /// ### 404 NotFound
    /// The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client. This response code is probably the most well known due to its frequent occurrence on the web.
    NotFound,
    /// ##### Client error responses
    /// The request method is known by the server but is not supported by the target resource. For example, an API may not allow calling DELETE to remove a resource.
    MethodNotAllowed,
    /// ##### Client error responses
    /// This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.
    NotAcceptable,
    /// ##### Client error responses
    /// This is similar to 401 Unauthorized but authentication is needed to be done by a proxy.
    ProxyAuthenticationRequired,
    /// ##### Client error responses
    /// This response is sent on an idle connection by some servers, even without any previous request by the client. It means that the server would like to shut down this unused connection. This response is used much more since some browsers, like Chrome, Firefox 27+, or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also note that some servers merely shut down the connection without sending this message.
    RequestTimeout,
    /// ##### Client error responses
    /// This response is sent when a request conflicts with the current state of the server.
    Conflict,
    /// ##### Client error responses
    /// This response is sent when the requested content has been permanently deleted from server, with no forwarding address. Clients are expected to remove their caches and links to the resource. The HTTP specification intends this status code to be used for "limited-time, promotional services". APIs should not feel compelled to indicate resources that have been deleted with this status code.
    Gone,
    /// ##### Client error responses
    /// Server rejected the request because the Content-Length header field is not defined and the server requires it.
    LengthRequired,
    /// ##### Client error responses
    /// The client has indicated preconditions in its headers which the server does not meet.
    PreconditionFailed,
    /// ##### Client error responses
    /// Request entity is larger than limits defined by server. The server might close the connection or return an Retry-After header field.
    PayloadTooLarge,
    /// ##### Client error responses
    /// The URI requested by the client is longer than the server is willing to interpret.
    URITooLong,
    /// ##### Client error responses
    /// The media format of the requested data is not supported by the server, so the server is rejecting the request.
    UnsupportedMediaType,
    /// ##### Client error responses
    /// The range specified by the Range header field in the request cannot be fulfilled. It's possible that the range is outside the size of the target URI's data.
    RangeNotSatisfiable,
    /// ##### Client error responses
    /// This response code means the expectation indicated by the Expect request header field cannot be met by the server.
    ExpectationFailed,
    /// ##### Client error responses
    /// The request was directed at a server that is not able to produce a response. This can be sent by a server that is not configured to produce responses for the combination of scheme and authority that are included in the request URI.
    MisdirectedRequest,
    /// ##### Client error responses
    /// (WebDAV) The request was well-formed but was unable to be followed due to semantic errors.
    UnprocessableContent ,
    /// ##### Client error responses
    /// (WebDAV) The resource that is being accessed is locked.
    Locked ,
    /// ##### Client error responses
    /// (WebDAV) The request failed due to failure of a previous request.
    FailedDependency ,
    /// ##### Client error responses
    /// Indicates that the server is unwilling to risk processing a request that might be replayed.
    TooEarlyExperimental,
    /// ##### Client error responses
    /// The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol. The server sends an Upgrade header in a 426 response to indicate the required protocol(s).
    UpgradeRequired,
    /// ##### Client error responses
    /// The origin server requires the request to be conditional. This response is intended to prevent the 'lost update' problem, where a client GETs a resource's state, modifies it and PUTs it back to the server, when meanwhile a third party has modified the state on the server, leading to a conflict.
    PreconditionRequired,
    /// ##### Client error responses
    /// The user has sent too many requests in a given amount of time ("rate limiting").
    TooManyRequests,
    /// ##### Client error responses
    /// The server is unwilling to process the request because its header fields are too large. The request may be resubmitted after reducing the size of the request header fields.
    RequestHeaderFieldsTooLarge,
    /// ##### Client error responses
    /// The user agent requested a resource that cannot legally be provided, such as a web page censored by a government.
    UnavailableForLegalReasons,
    /// ##### Server error responses
    /// The server has encountered a situation it does not know how to handle.
    InternalServerError,
    /// ##### Server error responses
    /// The request method is not supported by the server and cannot be handled. The only methods that servers are required to support (and therefore that must not return this code) are GET and HEAD.
    NotImplemented,
    /// ##### Server error responses
    /// This error response means that the server, while working as a gateway to get a response needed to handle the request, got an invalid response.
    BadGateway,
    /// ##### Server error responses
    /// The server is not ready to handle the request. Common causes are a server that is down for maintenance or that is overloaded. Note that together with this response, a user-friendly page explaining the problem should be sent. This response should be used for temporary conditions and the Retry-After HTTP header should, if possible, contain the estimated time before the recovery of the service. The webmaster must also take care about the caching-related headers that are sent along with this response, as these temporary condition responses should usually not be cached.
    ServiceUnavailable,
    /// ##### Server error responses
    /// This error response is given when the server is acting as a gateway and cannot get a response in time.
    GatewayTimeout,
    /// ##### Server error responses
    /// The HTTP version used in the request is not supported by the server.
    HTTPVersionNotSupported,
    /// ##### Server error responses
    /// The server has an internal configuration error: the chosen variant resource is configured to engage in transparent content negotiation itself, and is therefore not a proper end point in the negotiation process.
    VariantAlsoNegotiates,
    /// ##### Server error responses
    /// (WebDAV) The method could not be performed on the resource because the server is unable to store the representation needed to successfully complete the request.
    InsufficientStorage,
    /// ##### Server error responses
    /// (WebDAV) The server detected an infinite loop while processing the request.
    LoopDetected,
    /// ##### Server error responses
    /// Further extensions to the request are required for the server to fulfill it.
    NotExtended,
    /// ##### Server error responses
    /// Indicates that the client needs to authenticate to gain network access.
    NetworkAuthenticationRequired,
}

impl StatusCode {
    pub fn as_tuple(&self) -> (usize, &'static str) {
        return match self {
            // Information responses
            StatusCode::Continue => (100, "Continue"),
            StatusCode::SwitchingProtocols => (101, "SwitchingProtocols"),
            StatusCode::Processing => (102, "Processing"),
            StatusCode::EarlyHints => (103, "EarlyHints"),

            // Successful responses
            StatusCode::Ok => (200, "Ok"),
            StatusCode::Created => (201, "Created"),
            StatusCode::Accepted => (202, "Accepted"),
            StatusCode::NonAuthoritativeInformation => (203, "NonAuthoritativeInformation"),
            StatusCode::NoContent => (204, "NoContent"),
            StatusCode::ResetContent => (205, "ResetContent"),
            StatusCode::PartialContent => (206, "PartialContent"),
            StatusCode::MultiStatus => (207, "MultiStatus"),
            StatusCode::AlreadyReported => (208, "AlreadyReported"),
            StatusCode::IMUsed => (226, "IMUsed"),

            // Redirection messages
            StatusCode::MultipleChoices => (300, "MultipleChoices"),
            StatusCode::MovedPermanently => (301, "MovedPermanently"),
            StatusCode::Found => (302, "Found"),
            StatusCode::SeeOther => (303, "SeeOther"),
            StatusCode::NotModified => (304, "NotModified"),
            StatusCode::UseProxyDeprecated => (305, "UseProxyDeprecated"),
            StatusCode::TemporaryRedirect => (307, "TemporaryRedirect"),
            StatusCode::PermanentRedirect => (308, "PermanentRedirect"),

            // Client error responses
            StatusCode::BadRequest => (400, "BadRequest"),
            StatusCode::Unauthorized => (401, "Unauthorized"),
            StatusCode::PaymentRequiredExperimental => (402, "PaymentRequiredExperimental"),
            StatusCode::Forbidden => (403, "Forbidden"),
            StatusCode::NotFound => (404, "NotFound"),
            StatusCode::MethodNotAllowed => (405, "MethodNotAllowed"),
            StatusCode::NotAcceptable => (406, "NotAcceptable"),
            StatusCode::ProxyAuthenticationRequired => (407, "ProxyAuthenticationRequired"),
            StatusCode::RequestTimeout => (408, "RequestTimeout"),
            StatusCode::Conflict => (409, "Conflict"),
            StatusCode::Gone => (410, "Gone"),
            StatusCode::LengthRequired => (411, "LengthRequired"),
            StatusCode::PreconditionFailed => (412, "PreconditionFailed"),
            StatusCode::PayloadTooLarge => (413, "PayloadTooLarge"),
            StatusCode::URITooLong => (414, "URITooLong"),
            StatusCode::UnsupportedMediaType => (415, "UnsupportedMediaType"),
            StatusCode::RangeNotSatisfiable => (416, "RangeNotSatisfiable"),
            StatusCode::ExpectationFailed => (417, "ExpectationFailed"),
            StatusCode::MisdirectedRequest => (421, "MisdirectedRequest"),
            StatusCode::UnprocessableContent  => (422, "UnprocessableContent"),
            StatusCode::Locked  => (423, "Locked"),
            StatusCode::FailedDependency  => (424, "FailedDependency"),
            StatusCode::TooEarlyExperimental => (425, "TooEarlyExperimental"),
            StatusCode::UpgradeRequired => (426, "UpgradeRequired"),
            StatusCode::PreconditionRequired => (428, "PreconditionRequired"),
            StatusCode::TooManyRequests => (429, "TooManyRequests"),
            StatusCode::RequestHeaderFieldsTooLarge => (431, "RequestHeaderFieldsTooLarge"),
            StatusCode::UnavailableForLegalReasons => (451, "UnavailableForLegalReasons"),

            // Server error responses
            StatusCode::InternalServerError => (500, "InternalServerError"),
            StatusCode::NotImplemented => (501, "NotImplemented"),
            StatusCode::BadGateway => (502, "BadGateway"),
            StatusCode::ServiceUnavailable => (503, "ServiceUnavailable"),
            StatusCode::GatewayTimeout => (504, "GatewayTimeout"),
            StatusCode::HTTPVersionNotSupported => (505, "HTTPVersionNotSupported"),
            StatusCode::VariantAlsoNegotiates => (506, "VariantAlsoNegotiates"),
            StatusCode::InsufficientStorage => (507, "InsufficientStorage"),
            StatusCode::LoopDetected => (508, "LoopDetected"),
            StatusCode::NotExtended => (510, "NotExtended"),
            StatusCode::NetworkAuthenticationRequired => (511, "NetworkAuthenticationRequired"),
        };
    }
}
