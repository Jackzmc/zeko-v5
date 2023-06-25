use std::time::SystemTime;

pub fn make_alphanumeric(str: &str) -> String {
    str.chars().filter(|c| c.is_alphanumeric()).collect()
}
// Old server used milliseconds, so we use that here too:
pub fn get_unix_timestamp() -> std::time::Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}

#[macro_export]
/// Runs your code after Duration has passed
/// # Examples

/// ```
/// timeout!(Duration::from_seconds(15), {
///     println!("Shutting down now...")
/// });
/// ```
///
/// This is the same as:
/// ```
/// tokio::spawn(async move {
///     let mut timer = tokio::time::interval(Duration::from_seconds(15);
///     timer.tick().await;
///     //your function
/// })
/// ```
macro_rules! timeout {
    ( $i:expr, $x:expr) => {
        {
            tokio::spawn(async move {
                tokio::time::sleep($i).await;
                $x;
            })
        }
    };
}

#[macro_export]
/// Creates a timer that calls a function every Duration
/// # Examples

/// ```
/// timer!(update_interval, {
///     let mut lock = manager.lock().await;
/// });
/// ```
///
/// This is the same as:
/// ```
/// tokio::spawn(async move {
///     let mut timer = tokio::time::interval(update_interval);
///     timer.tick().await;
///     loop {
///         timer.tick().await;
///         // your function
///      }
/// })
/// ```
macro_rules! timer {
    ( $i:expr, $x:expr) => {
        {
            tokio::spawn(async move {
                let mut timer = tokio::time::interval($i);
                timer.tick().await;
                loop {
                    timer.tick().await;
                    $x;
                }
            })
        }
    };
}

#[macro_export]
/// Creates a timer that calls a function every Duration with an additional function called on start
///
/// See the [timer!] macro for more information
/// # Examples

/// ```
/// timer_start!(update_interval, {
///      println!("my timer is started!");
/// }, {
///     let mut lock = manager.lock().await;
/// });
/// ```
macro_rules! timer_start {
    ( $i:expr, $s:expr, $x:expr) => {
        {
            tokio::spawn(async move {
                let mut timer = tokio::time::interval($i);
                timer.tick().await;
                $s;
                loop {
                    timer.tick().await;
                    $x;
                }
            })
        }
    };
}