use super::DEFAULT_MESSAGE_LEVEL;
use crate::LatencyUnit;
use http::Response;
use std::time::Duration;
use tracing::Level;
use tracing::Span;

/// Trait used to tell [`Trace`] what to do when a response has been produced.
///
/// [`Trace`]: super::Trace
pub trait OnResponse<B> {
    /// Do the thing.
    ///
    /// `latency` is the duration since the request was received.
    ///
    /// `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure
    /// passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]
    /// that weren't known when the span was created.
    ///
    /// [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html
    /// [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record
    /// [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span);
}

impl<B> OnResponse<B> for () {
    #[inline]
    fn on_response(self, _: &Response<B>, _: Duration, _: &Span) {}
}

impl<B, F> OnResponse<B> for F
where
    F: FnOnce(&Response<B>, Duration, &Span),
{
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
        self(response, latency, span)
    }
}

/// The default [`OnResponse`] implementation used by [`Trace`].
///
/// [`Trace`]: super::Trace
#[derive(Clone, Debug)]
pub struct DefaultOnResponse {
    level: Level,
    latency_unit: LatencyUnit,
    include_headers: bool,
}

impl Default for DefaultOnResponse {
    fn default() -> Self {
        Self {
            level: DEFAULT_MESSAGE_LEVEL,
            latency_unit: LatencyUnit::Millis,
            include_headers: false,
        }
    }
}

impl DefaultOnResponse {
    /// Create a new `DefaultOnResponse`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the [`Level`] used for [tracing events].
    ///
    /// Defaults to [`Level::DEBUG`].
    ///
    /// [tracing events]: https://docs.rs/tracing/latest/tracing/#events
    /// [`Level::DEBUG`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.DEBUG
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Set the [`LatencyUnit`] latencies will be reported in.
    ///
    /// Defaults to [`LatencyUnit::Millis`].
    pub fn latency_unit(mut self, latency_unit: LatencyUnit) -> Self {
        self.latency_unit = latency_unit;
        self
    }

    /// Include response headers on the [`Event`].
    ///
    /// By default headers are not included.
    ///
    /// [`Event`]: tracing::Event
    pub fn include_headers(mut self, include_headers: bool) -> Self {
        self.include_headers = include_headers;
        self
    }
}

// Repeating this pattern match for each case is tedious. So we do it with a quick and
// dirty macro.
//
// Tracing requires all these parts to be declared statically. You cannot easily build
// events dynamically.
#[allow(unused_macros)]
macro_rules! log_pattern_match {
    (
        $this:expr, $res:expr, $latency:expr, $include_headers:expr, [$($level:ident),*]
    ) => {
        match ($this.level, $include_headers, $this.latency_unit, status($res)) {
            $(
                (Level::$level, true, LatencyUnit::Millis, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ms", $latency.as_millis()),
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Millis, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ms", $latency.as_millis()),
                        "finished processing request"
                    );
                }
                (Level::$level, true, LatencyUnit::Millis, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ms", $latency.as_millis()),
                        status = status,
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Millis, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ms", $latency.as_millis()),
                        status = status,
                        "finished processing request"
                    );
                }

                (Level::$level, true, LatencyUnit::Micros, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} μs", $latency.as_micros()),
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Micros, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} μs", $latency.as_micros()),
                        "finished processing request"
                    );
                }
                (Level::$level, true, LatencyUnit::Micros, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} μs", $latency.as_micros()),
                        status = status,
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Micros, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} μs", $latency.as_micros()),
                        status = status,
                        "finished processing request"
                    );
                }

                (Level::$level, true, LatencyUnit::Nanos, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ns", $latency.as_nanos()),
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Nanos, None) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ns", $latency.as_nanos()),
                        "finished processing request"
                    );
                }
                (Level::$level, true, LatencyUnit::Nanos, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ns", $latency.as_nanos()),
                        status = status,
                        response_headers = ?$res.headers(),
                        "finished processing request"
                    );
                }
                (Level::$level, false, LatencyUnit::Nanos, Some(status)) => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ns", $latency.as_nanos()),
                        status = status,
                        "finished processing request"
                    );
                }
            )*
        }
    };
}

impl<B> OnResponse<B> for DefaultOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _: &Span) {
        log_pattern_match!(
            self,
            response,
            latency,
            self.include_headers,
            [ERROR, WARN, INFO, DEBUG, TRACE]
        );
    }
}

fn status<B>(res: &Response<B>) -> Option<i32> {
    use crate::classify::ParsedGrpcStatus;

    let is_grpc = res
        .headers()
        .get(http::header::CONTENT_TYPE)
        .map_or(false, |value| value == "application/grpc");

    if is_grpc {
        match crate::classify::classify_grpc_metadata(res.headers()) {
            ParsedGrpcStatus::Success
            | ParsedGrpcStatus::HeaderNotString
            | ParsedGrpcStatus::HeaderNotInt => Some(0),
            ParsedGrpcStatus::NonSuccess(status) => Some(status.get()),
            // if `grpc-status` is missing then its a streaming response and there is no status
            // _yet_, so its neither success nor error
            ParsedGrpcStatus::GrpcStatusHeaderMissing => None,
        }
    } else {
        Some(res.status().as_u16().into())
    }
}
