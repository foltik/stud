macro_rules! error {
    ($span:expr, $msg:expr) => {{
        use syn::spanned::Spanned;
        syn::Error::new($span.span(), $msg)
    }};
    ($span:expr, $fmt:expr, $($arg:expr),*) => {{
        use syn::spanned::Spanned;
        syn::Error::new($span.span(), format!($fmt, $($arg),*))
    }};
}
pub(crate) use error;

macro_rules! err {
    ($span:expr, $msg:expr) => {
        Err($crate::utils::error!($span, $msg))
    };
    ($span:expr, $fmt:expr, $($arg:expr),*) => {
        Err($crate::utils::error!($span, $fmt, $($arg),*))
    };
}
pub(crate) use err;

macro_rules! bail {
    ($span:expr, $msg:expr) => {
        return $crate::utils::err!($span, $msg)
    };
    ($span:expr, $fmt:expr, $($arg:expr),*) => {
        return $crate::utils::err!($span, $fmt, $($arg),*)
    };
}
pub(crate) use bail;

macro_rules! expand {
    ($result:expr) => {
        match $result {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    };
}
pub(crate) use expand;
