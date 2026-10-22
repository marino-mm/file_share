const SHA256CONSTANT: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

macro_rules! add_mod {
    ($x:expr, $y:expr) => {{
        let (x, y): (u32, u32) = ($x, $y);
        x.wrapping_add(y)
    }};
}

macro_rules! ch {
    ($x:expr, $y:expr, $z:expr) => {{
        let (x, y, z): (u32, u32, u32) = ($x, $y, $z);
        (x & y) ^ (!x & z)
    }};
}

macro_rules! maj {
    ($x:expr, $y:expr, $z:expr) => {{
        let (x, y, z): (u32, u32, u32) = ($x, $y, $z);
        (x & y) ^ (x & z) ^ (y & z)
    }};
}
macro_rules! rotr {
    ($x:expr, $n:expr) => {{
        let x: u32 = $x;
        x.rotate_right($n)
    }};
}

macro_rules! shr {
    ($x:expr, $n:expr) => {{
        let x: u32 = $x;
        x >> $n
    }};
}

macro_rules! sum_0 {
    ($x:expr) => {{
        let x: u32 = $x;
        rotr!(x, 2) ^ rotr!(x, 13) ^ rotr!(x, 22)
    }};
}

macro_rules! sum_1 {
    ($x:expr) => {{
        let x: u32 = $x;
        rotr!(x, 6) ^ rotr!(x, 11) ^ rotr!(x, 25)
    }};
}

macro_rules! sigma_0 {
    ($x:expr) => {{
        let x: u32 = $x;
        rotr!(x, 7) ^ rotr!(x, 18) ^ shr!(x, 3)
    }};
}

macro_rules! sigma_1 {
    ($x:expr) => {{
        let x: u32 = $x;
        rotr!(x, 17) ^ rotr!(x, 19) ^ shr!(x, 10)
    }};
}

pub struct Sha256 {
    hash: [u32; 8],
    l: u64,
    rem: Vec<u8>,
    is_final: bool,
}

impl Sha256 {
    pub fn new() -> Sha256 {
        Sha256 {
            hash: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            l: 0,
            rem: Vec::with_capacity(64),
            is_final: false,
        }
    }

    fn pad_message(&self) -> Vec<u8> {
        let chunk = self.rem.as_slice();
        let mut msg = Vec::with_capacity(128);
        msg.extend_from_slice(chunk);
        msg.push(0x80);
        while msg.len() % 64 != 56 {
            msg.push(0);
        }

        msg.extend_from_slice(&self.l.to_be_bytes());

        msg
    }

    pub fn finalize(&mut self) -> String {
        if !self.is_final {
            self.l += self.rem.len() as u64 * 8;

            let padding = self.pad_message();
            padding.chunks_exact(64).for_each(|chunk| {
                self.hash(chunk);
            });
            self.is_final = true;
        }

        self.hash.iter().map(|x| format!("{:08x}", x)).collect()
    }

    pub fn update(&mut self, stream: &[u8]) {
        let mut new_msg = self.rem.clone();
        new_msg.extend_from_slice(stream);
        self.rem.clear();

        new_msg.chunks(64).for_each(|chunk| {
            if chunk.len() < 64 {
                self.rem = Vec::from(chunk);
            } else {
                self.hash(chunk);
                self.l += 64 * 8;
            }
        })
    }

    fn hash(&mut self, msg_chunk: &[u8]) {
        let mut a = self.hash[0];
        let mut b = self.hash[1];
        let mut c = self.hash[2];
        let mut d = self.hash[3];
        let mut e = self.hash[4];
        let mut f = self.hash[5];
        let mut g = self.hash[6];
        let mut h = self.hash[7];

        let mut w_arr = [0u32; 64];

        let message: [u32; 16] = std::array::from_fn(|i| {
            u32::from_be_bytes(msg_chunk[i * 4..i * 4 + 4].try_into().unwrap())
        });

        for t in 0..64 {
            let w = if t < 16 {
                message[t]
            } else {
                [
                    sigma_1!(w_arr[t - 2]),
                    w_arr[t - 7],
                    sigma_0!(w_arr[t - 15]),
                    w_arr[t - 16],
                ]
                .iter()
                .fold(0u32, |acc, &b| add_mod!(acc, b))
            };
            w_arr[t] = w;

            let t_1 = [h, sum_1!(e), ch!(e, f, g), SHA256CONSTANT[t], w]
                .iter()
                .fold(0u32, |acc, &b| add_mod!(acc, b));

            let t_2 = [sum_0!(a), maj!(a, b, c)]
                .iter()
                .fold(0u32, |acc, &b| add_mod!(acc, b));
            h = g;
            g = f;
            f = e;
            e = add_mod!(d, t_1);
            d = c;
            c = b;
            b = a;
            a = add_mod!(t_1, t_2);
        }

        self.hash[0] = add_mod!(self.hash[0], a);
        self.hash[1] = add_mod!(self.hash[1], b);
        self.hash[2] = add_mod!(self.hash[2], c);
        self.hash[3] = add_mod!(self.hash[3], d);
        self.hash[4] = add_mod!(self.hash[4], e);
        self.hash[5] = add_mod!(self.hash[5], f);
        self.hash[6] = add_mod!(self.hash[6], g);
        self.hash[7] = add_mod!(self.hash[7], h);
    }
}

#[cfg(test)]
mod sha256_test {
    use super::*;

    #[test]
    fn empty() {
        let mut sha = Sha256::new();
        let msg = "";
        sha.update(msg.as_bytes());
        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        )
    }
    #[test]
    fn abc_test() {
        let mut sha = Sha256::new();
        let msg = "abc";
        sha.update(msg.as_bytes());
        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        )
    }

    #[test]
    fn abc_split_test() {
        let mut sha = Sha256::new();
        let msg = "a";
        sha.update(msg.as_bytes());

        let msg = "b";
        sha.update(msg.as_bytes());

        let msg = "c";
        sha.update(msg.as_bytes());

        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        )
    }

    #[test]
    fn hello_test() {
        let mut sha = Sha256::new();
        let msg = "hello";
        sha.update(msg.as_bytes());
        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
        )
    }

    #[test]
    fn lorem_short_test() {
        let mut sha = Sha256::new();
        let msg = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod";
        sha.update(msg.as_bytes());
        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("43f1030606541df5812e4742226d099b2783f92566f502ec962f92a701fd6575")
        )
    }

    #[test]
    fn lorem_long_test() {
        let mut sha = Sha256::new();
        let msg = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem
        ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
        exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
        lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua enim ad minim veniam quis
        nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute
        irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam
        quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis
        aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim
        veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat
        duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad
        minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo
        consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing elit
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim
        ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea
        commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur adipiscing
        elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex
        ea commodo consequat duis aute irure lorem ipsum dolor sit amet consectetur
        adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna
        aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip
        ex ea commodo consequat duis aute irure lorem ipsum dolor sit amet
        consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore
        magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
        aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor sit
        amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et
        dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris
        nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum dolor
        sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore
        et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco
        laboris nisi aliquip ex ea commodo consequat duis aute irure lorem ipsum
        dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation
        ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure lorem";
        sha.update(msg.as_bytes());
        let hash = sha.finalize();
        assert_eq!(
            hash,
            String::from("db8c460d072fdd6028107cd89c885da3f9753a64d41038a483dd5c6b481bc12e")
        )
    }
}
