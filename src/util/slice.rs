/// Like `slice::array_windows()`, but on stable Rust
pub fn array_windows<T, const N: usize>(iter: &[T]) -> impl Iterator<Item = &[T; N]> {
    iter.windows(N).map(|window| window.try_into().unwrap())
}
