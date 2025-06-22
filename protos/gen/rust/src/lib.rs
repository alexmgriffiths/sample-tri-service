pub mod echo {
    include!("../echo.v1.rs");
    include!("../echo.v1.tonic.rs");
}

pub mod greeter {
    include!("../greeter.v1.rs");
    include!("../greeter.v1.tonic.rs");
}

pub mod time {
    include!("../time.v1.rs");
    include!("../time.v1.tonic.rs");
}