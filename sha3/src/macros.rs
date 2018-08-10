macro_rules! impl_state {
    ($state:ident, $rate:ident, $padding:ty) => {

        #[allow(non_camel_case_types)]
        #[derive(Clone, Default)]
        pub struct $state {
            state: Sha3State,
            buffer: BlockBuffer<$rate>,
        }

        impl $state {
            fn absorb(&mut self, input: &[u8]) {
                let self_state = &mut self.state;
                self.buffer.input(input, |b| self_state.absorb_block(b));
            }

            fn apply_padding(&mut self) {
                let buf = self.buffer.pad_with::<$padding>()
                    .expect("we never use input_lazy");
                self.state.absorb_block(buf);
            }
        }
    }
}

macro_rules! sha3_impl {
    ($state:ident, $output_size:ident, $rate:ident, $padding:ty) => {

        impl_state!($state, $rate, $padding);

        impl BlockInput for $state {
            type BlockSize = $rate;
        }

        impl Input for $state {
            fn process(&mut self, data: &[u8]) {
                self.absorb(data)
            }
        }

        impl FixedOutput for $state {
            type OutputSize = $output_size;

            fn fixed_result(mut self) -> GenericArray<u8, Self::OutputSize> {
                self.apply_padding();

                let mut out = GenericArray::default();
                let n = out.len();
                self.state.as_bytes(|state| {
                    out.copy_from_slice(&state[..n]);
                });
                out
            }
        }

        impl Reset for $state {
            fn reset(&mut self) -> Self {
                let mut temp = Self::default();
                core::mem::swap(self, &mut temp);
                temp
            }
        }

        impl_opaque_debug!($state);
        impl_write!($state);
    }
}

macro_rules! shake_impl {
    ($state:ident, $rate:ident, $padding:ty) => {
        impl_state!($state, $rate, $padding);

        impl Input for $state {
            fn process(&mut self, data: &[u8]) {
                self.absorb(data)
            }
        }

        impl ExtendableOutput for $state {
            type Reader = Sha3XofReader;

            fn xof_result(mut self) -> Sha3XofReader {
                self.apply_padding();
                let r = $rate::to_usize();
                let res = Sha3XofReader::new(self.state.clone(), r);
                res
            }
        }

        impl Reset for $state {
            fn reset(&mut self) -> Self {
                let mut temp = Self::default();
                core::mem::swap(self, &mut temp);
                temp
            }
        }

        impl_opaque_debug!($state);
        impl_write!($state);
    }
}
