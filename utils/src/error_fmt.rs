pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        write!(f, "\ncaused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
