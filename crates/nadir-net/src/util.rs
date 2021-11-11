#[derive(Debug, Clone)]
pub enum MaybeFatal<E, F> {
    Recoverable(E),
    Fatal(F),
}

impl<E, F> std::fmt::Display for MaybeFatal<E, F>
where
    E: std::fmt::Display,
    F: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeFatal::Recoverable(e) => e.fmt(f),
            MaybeFatal::Fatal(f_) => f_.fmt(f),
        }
    }
}

impl<E, F> std::error::Error for MaybeFatal<E, F>
where
    E: std::fmt::Debug + std::fmt::Display,
    F: std::fmt::Debug + std::fmt::Display,
{
}

impl<E, F> From<E> for MaybeFatal<E, F> {
    fn from(v: E) -> Self {
        Self::Recoverable(v)
    }
}
