pub trait ToNative {
    type Native;
    fn to_native(&self) -> Self::Native;
}
