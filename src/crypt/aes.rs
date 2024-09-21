////////////////////////////////////////////////////
//! The purpose of this file is to add functionality
//! for encryption and decryption using:
//!      - AES-256
//!
//! This functionality is what will allow system calls
//! to operate on data.
//!
//! AES reference: https://github.com/kokke/tiny-AES-c/blob/master/aes.c
//!
//! This file provides the following public functionality:
//!
//! mod AES:
//!     const AES_BLOCKLEN: usize - aes block size (256 bit for aes256)
//!     const AES_KEYLEN: usize - size of the encryption/decryption key
//!     const AES_KEYEXPSIZE: usize - size of the round key within AES context
//!
//!     struct AesCtx - an AES context
//!         {
//!         round_key: [usize; AES_KEYEXPSIZE],
//!         iv: [usize; AES_BLOCKLEN],
//!         }
//!         
//!         new(round_key: [usize; AES_KEYEXPSIZE], iv: [usize; AES_BLOCKLEN]) -> Self - constructor
//!         aes_init_ctx(&mut self, key: &[usize]) -> () - initialize an aes context using the key
//!         aes_init_ctx_iv(&mut self, key: &[usize], iv: &[usize]) -> () - initialize the AES context's IV
//!         aes_ctx_set_iv(&mut self, iv: &[usize]) -> () - set the context's iv
//!         aes_encrypt_buffer(&mut self, buf: &mut [usize], length: usize) - encrypt a buffer
//!
//!     test() -> () - run a test function
//!     
//!     
////////////////////////////////////////////////////

pub mod AES {
    ////////////////////////////////////////////////////
    // AES-256 FUNCTIONS

    // number of columns comprising a state in AES. (constant in AES)
    const NB: usize = 4;
    const NK: usize = 8;
    const NR: usize = 14;

    pub const AES_BLOCKLEN: usize = 16;
    pub const AES_KEYLEN: usize = 32;
    pub const AES_KEYEXPSIZE: usize = 240;

    // The lookup-tables are marked const so they can be placed in read-only storage instead of RAM
    // The numbers below can be computed dynamically trading ROM for RAM -
    // This can be useful in (embedded) bootloader applications, where ROM is often limited.
    //
    // FIXME: Convert SBOX and RBOX to const fn. Right now the implementation looks too difficult
    // to deal with
    const SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
    const RSBOX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
        0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
        0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
        0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
        0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
        0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
        0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
        0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
        0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
        0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
        0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
        0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ];

    // The round constant word array, Rcon[i], contains the values given by
    // x to the power (i-1) being powers of x (x is denoted as {02}) in the field GF(2^8)
    // FIXME: possibly convert this to a const fn later.
    const RCON: [u8; 11] = [
        0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
    ];

    // state - matrix holding the intermediate results during decryption
    type StateT = [[u8; 4]; 4];

    // SubWord() is a function that takes a four-byte input word and
    // applies the S-box to each of the four bytes to produce an output word.
    // FIXME: when SBOX is turned into a function, simply pass u8 as the argument instead of having
    // to typecast every time
    fn sub_word(tempa: &mut [u8; 4]) {
        tempa[0] = SBOX[tempa[0] as usize];
        tempa[1] = SBOX[tempa[1] as usize];
        tempa[2] = SBOX[tempa[2] as usize];
        tempa[3] = SBOX[tempa[3] as usize];
    }

    // function to produce Nb(Nr+1) round keys. The round keys are used in each round to decrypt the states
    fn key_expansion(round_key: &mut [u8], key: &[u8]) {
        // The first round key is the key itself.
        for i in 0..NK {
            let quad: usize = i << 2;
            round_key[quad] = key[quad];
            round_key[quad + 1] = key[quad + 1];
            round_key[quad + 2] = key[quad + 2];
            round_key[quad + 3] = key[quad + 3];
        }

        // All other round keys are found from teh previous round keys.
        let loop_end = NB * (NR + 1);
        for i in NK..loop_end {
            let k: usize = (i - 1) << 2;

            /* FIXME: use slices for this operation
             * Need to find how to convert slice from [usize] to [usize; 4]
             * when the slice's size is known
            let tempa: [usize; 4] = round_key[k..k+4];
            */
            let mut tempa: [u8; 4] = [
                round_key[k],
                round_key[k + 1],
                round_key[k + 2],
                round_key[k + 3],
            ];

            if i % NK == 0 {
                // This function shifts the 4 bytes in a word to the left once.
                // [a0,a1,a2,a3] becomes [a1,a2,a3,a0]

                // rotate tempa left by 1
                tempa.rotate_left(1);

                // apply the sbox value to each element of tempa
                sub_word(&mut tempa);

                tempa[0] ^= RCON[i / NK];
            }

            if i % NK == 4 {
                // apply the sbox value to each element of tempa
                sub_word(&mut tempa);
            }

            let j: usize = i << 2;
            let l: usize = (i - NK) << 2;

            round_key[j + 0] = round_key[l + 0] ^ tempa[0];
            round_key[j + 1] = round_key[l + 1] ^ tempa[1];
            round_key[j + 2] = round_key[l + 2] ^ tempa[2];
            round_key[j + 3] = round_key[l + 3] ^ tempa[3];
        }
    }

    // This function adds the round key to state.
    // The round key is added to the state by an XOR function.
    fn add_round_key(round: usize, state: &mut StateT, round_key: &[u8]) {
        let round_offset: usize = (round * NB) << 2;

        for i in 0..4 {
            let temp: usize = i * NB + round_offset;
            for j in 0..4 {
                state[i][j] ^= round_key[temp + j];
            }
        }
    }

    // The SubBytes Function Substitutes the values in the
    // state matrix with values in an S-box.
    fn sub_bytes(state: &mut StateT) {
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = SBOX[state[j][i] as usize];
            }
        }
    }

    // The ShiftRows() function shifts the rows in the state to the left.
    // Each row is shifted with different offset.
    // Offset = Row number. So the first row is not shifted.
    fn shift_rows(state: &mut StateT) {
        // Rotate first row 1 column to the left
        let temp: u8 = state[0][1];
        state[0][1] = state[1][1];
        state[1][1] = state[2][1];
        state[2][1] = state[3][1];
        state[3][1] = temp;

        // Rotate second row 2 columns to left
        let temp: u8 = state[0][2];
        state[0][2] = state[2][2];
        state[2][2] = temp;

        let temp: u8 = state[1][2];
        state[1][2] = state[3][2];
        state[3][2] = temp;

        // Rotate third row 3 columns to left
        let temp: u8 = state[0][3];
        state[0][3] = state[3][3];
        state[3][3] = state[2][3];
        state[2][3] = state[1][3];
        state[1][3] = temp;
    }

    fn xtime(x: u8) -> u8 {
        return (x << 1) ^ (((x >> 7) & 1) * 0x1b);
    }

    fn mix_columns(state: &mut StateT) {
        // FIXME: is using let for every single change better than making mut?
        // or is mut specifically if you wish to pass to a function that changes it?
        for i in 0..4 {
            let t: u8 = state[i][0];
            let tmp: u8 = state[i][0] ^ state[i][1] ^ state[i][2] ^ state[i][3];

            let tm: u8 = state[i][0] ^ state[i][1];
            let tm: u8 = xtime(tm);
            state[i][0] ^= tm ^ tmp;

            let tm: u8 = state[i][1] ^ state[i][2];
            let tm: u8 = xtime(tm);
            state[i][1] ^= tm ^ tmp;

            let tm: u8 = state[i][2] ^ state[i][3];
            let tm: u8 = xtime(tm);
            state[i][2] ^= tm ^ tmp;

            let tm: u8 = state[i][3] ^ t;
            let tm: u8 = xtime(tm);
            state[i][3] ^= tm ^ tmp;
        }
    }

    // Multiply is used to multiply numbers in the field GF(2^8)
    // Note: The last call to xtime() is unneeded, but often ends up generating a smaller binary
    //       The compiler seems to be able to vectorize the operation better this way.
    //       See https://github.com/kokke/tiny-AES-c/pull/34
    fn multiply(x: u8, y: u8) -> u8 {
        return ((y & 1) * x)
            ^ ((y >> 1 & 1) * xtime(x))
            ^ ((y >> 2 & 1) * xtime(xtime(x)))
            ^ ((y >> 3 & 1) * xtime(xtime(xtime(x))))
            ^ ((y >> 4 & 1) * xtime(xtime(xtime(xtime(x))))); /* this last call to xtime() can be omitted */
    }

    // MixColumns function mixes the columns of the state matrix.
    // The method used to multiply may be difficult to understand for the inexperienced.
    // Please use the references to gain more information.
    fn inv_mix_columns(state: &mut StateT) {
        for i in 0..4 {
            let a = state[i][0];
            let b = state[i][1];
            let c = state[i][2];
            let d = state[i][3];

            state[i][0] =
                multiply(a, 0x0e) ^ multiply(b, 0x0b) ^ multiply(c, 0x0d) ^ multiply(d, 0x09);
            state[i][1] =
                multiply(a, 0x09) ^ multiply(b, 0x0e) ^ multiply(c, 0x0b) ^ multiply(d, 0x0d);
            state[i][2] =
                multiply(a, 0x0d) ^ multiply(b, 0x09) ^ multiply(c, 0x0e) ^ multiply(d, 0x0b);
            state[i][3] =
                multiply(a, 0x0b) ^ multiply(b, 0x0d) ^ multiply(c, 0x09) ^ multiply(d, 0x0e);
        }
    }

    // The SubBytes Function Substitutes the values in the
    // state matrix with values in an S-box.
    fn inv_sub_bytes(state: &mut StateT) {
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = RSBOX[state[j][i] as usize];
            }
        }
    }

    fn inv_shift_rows(state: &mut StateT) {
        // Rotate first row 1 columns to right
        let temp: u8 = state[3][1];
        state[3][1] = state[2][1];
        state[2][1] = state[1][1];
        state[1][1] = state[0][1];
        state[0][1] = temp;

        // Rotate second row 2 columns to right
        let temp: u8 = state[0][2];
        state[0][2] = state[2][2];
        state[2][2] = temp;

        let temp: u8 = state[1][2];
        state[1][2] = state[3][2];
        state[3][2] = temp;

        // Rotate third row 3 columns to right
        let temp: u8 = state[0][3];
        state[0][3] = state[1][3];
        state[1][3] = state[2][3];
        state[2][3] = state[3][3];
        state[3][3] = temp;
    }

    // Cipher is the main function that encrypts the PlainText.
    fn cipher(state: &mut StateT, round_key: &[u8]) {
        // Add the First round key to the state before starting the rounds.
        add_round_key(0, state, round_key);

        // There will be Nr rounds.
        // The first Nr-1 rounds are identical.
        // These Nr rounds are executed in the loop below.
        // Last one without MixColumns()
        let mut round: usize = 1;
        loop {
            sub_bytes(state);
            shift_rows(state);
            if round == NR {
                break;
            }
            mix_columns(state);
            add_round_key(round, state, round_key);
            round += 1;
        }

        // Add round key to last round
        add_round_key(NR, state, round_key);
    }

    fn inv_cipher(state: &mut StateT, round_key: &[u8]) {
        // Add the First round key to the state before starting the rounds.
        add_round_key(NR, state, round_key);

        // There will be Nr rounds.
        // The first Nr-1 rounds are identical.
        // These Nr rounds are executed in the loop below.
        // Last one without InvMixColumn()
        let mut round: usize = NR - 1;
        loop {
            inv_shift_rows(state);
            inv_sub_bytes(state);
            add_round_key(round, state, round_key);
            if round == 0 {
                break;
            }
            inv_mix_columns(state);
            round -= 1;
        }
    }

    fn xor_with_iv(buf: &mut [u8], iv: &[u8; AES_BLOCKLEN]) {
        for i in 0..AES_BLOCKLEN {
            buf[i] ^= iv[i];
        }
    }

    // FIXME: find a better way to cast from a point in the middle of a buffer to a StateT
    fn buffer_to_statet(buf: &mut [u8], i: usize) -> StateT {
        // i is the offset (just used i to make the code shorter)
        let result: StateT = [
            [buf[i + 0], buf[i + 1], buf[i + 2], buf[i + 3]],
            [buf[i + 4], buf[i + 5], buf[i + 6], buf[i + 7]],
            [buf[i + 8], buf[i + 9], buf[i + 10], buf[i + 11]],
            [buf[i + 12], buf[i + 13], buf[i + 14], buf[i + 15]],
        ];
        return result;
    }

    // Aes context structure
    pub struct AesCtx {
        round_key: [u8; AES_KEYEXPSIZE],
        iv: [u8; AES_BLOCKLEN],
    }

    impl AesCtx {
        pub fn new(round_key: [u8; AES_KEYEXPSIZE], iv: [u8; AES_BLOCKLEN]) -> Self {
            return AesCtx {
                round_key: round_key,
                iv: iv,
            };
        }

        pub fn aes_init_ctx(&mut self, key: &[u8]) {
            key_expansion(&mut self.round_key, key);
        }

        // function to initialize the AES context's IV
        pub fn aes_init_ctx_iv(&mut self, key: &[u8], iv: &[u8]) {
            key_expansion(&mut self.round_key, key);
            // slower equivalent of memcpy (ctx->Iv, iv, AES_BLOCKLEN);
            for i in 0..AES_BLOCKLEN {
                self.iv[i] = iv[i];
            }
        }

        // function to set the context's iv
        pub fn aes_ctx_set_iv(&mut self, iv: &[u8]) {
            for i in 0..AES_BLOCKLEN {
                self.iv[i] = iv[i];
            }
        }

        pub fn aes_encrypt_buffer(&mut self, buf: &mut [u8], length: usize) {
            let mut i: usize = 0;

            while i < length {
                xor_with_iv(buf, &self.iv);
                let mut state: StateT = buffer_to_statet(buf, i);
                cipher(&mut state, &self.round_key);

                // copy the buffer to ctx.iv
                for j in 0..AES_BLOCKLEN {
                    self.iv[j] = buf[i + j];
                }

                i += AES_BLOCKLEN;
            }
        }
    }

    // function for testing purposes
    pub fn test() {
        let mut ctx: AesCtx = AesCtx::new([0; AES_KEYEXPSIZE], [0; AES_BLOCKLEN]);

        let key: [u8; AES_KEYLEN] = [0; AES_KEYLEN];

        // init the iv
        for i in 0..AES_BLOCKLEN {
            ctx.iv[i] = i as u8;
        }

        // init the ctx with round key
        ctx.aes_init_ctx(&key);

        let mut buffer: [u8; AES_BLOCKLEN] = [0; AES_BLOCKLEN];

        println!("Testing AES functionality...");
        println!("RK: {:?}", ctx.round_key);
        println!("IV: {:?}", ctx.iv);

        println!("BUFFER BEFORE: {:?}", buffer);

        ctx.aes_encrypt_buffer(&mut buffer, AES_BLOCKLEN);

        println!("BUFFER AFTER: {:?}", buffer);
    }

    /*
    void AES_CBC_encrypt_buffer(struct AesCtx *ctx, uint8_t* buf, size_t length)
    {
      size_t i;
      uint8_t *Iv = ctx->Iv;
      for (i = 0; i < length; i += AES_BLOCKLEN)
      {
        XorWithIv(buf, Iv);
        Cipher((state_t*)buf, ctx->RoundKey);
        Iv = buf;
        buf += AES_BLOCKLEN;
      }
      /* store Iv in ctx for next call */
      memcpy(ctx->Iv, Iv, AES_BLOCKLEN);
    }



    //==================================================
    // void AES_CBC_decrypt_buffer(struct AesCtx* ctx, uint8_t* buf, size_t length);
    //
    // TODO
    //==================================================

    void AES_CBC_decrypt_buffer(struct AesCtx* ctx, uint8_t* buf, size_t length)
    {
      size_t i;
      uint8_t storeNextIv[AES_BLOCKLEN];
      for (i = 0; i < length; i += AES_BLOCKLEN)
      {
        memcpy(storeNextIv, buf, AES_BLOCKLEN);
        InvCipher((state_t*)buf, ctx->RoundKey);
        XorWithIv(buf, ctx->Iv);
        memcpy(ctx->Iv, storeNextIv, AES_BLOCKLEN);
        buf += AES_BLOCKLEN;
      }

    }

    */
}
