use std::fmt::Display;
use std::string::ToString;

static CHAR_ARRAY: [u64; 95] = [
    0x0000000000000000, // space
    0x183c3c1818001800, // !
    0x3636000000000000, // "
    0x36367f367f363600, // #
    0x0c3e031e301f0c00, // $
    0x006333180c666300, // %
    0x1c361c6e3b336e00, // &
    0x0606030000000000, // '
    0x180c0606060c1800, // (
    0x060c1818180c0600, // )
    0x00663cff3c660000, // *
    0x000c0c3f0c0c0000, // +
    0x00000000000c0c06, // ,
    0x0000003f00000000, // -
    0x00000000000c0c00, // .
    0x6030180c06030100, // /
    0x3e63737b6f673e00, // 0
    0x0c0e0c0c0c0c3f00, // 1
    0x1e33301c06333f00, // 2
    0x1e33301c30331e00, // 3
    0x383c36337f307800, // 4
    0x3f031f3030331e00, // 5
    0x1c06031f33331e00, // 6
    0x3f3330180c0c0c00, // 7
    0x1e33331e33331e00, // 8
    0x1e33333e30180e00, // 9
    0x000c0c00000c0c00, // :
    0x000c0c00000c0c06, // ;
    0x180c0603060c1800, // <
    0x00003f00003f0000, // =
    0x060c1830180c0600, // >
    0x1e3330180c000c00, // ?
    0x3e637b7b7b031e00, // @
    0x0c1e33333f333300, // A
    0x3f66663e66663f00, // B
    0x3c66030303663c00, // C
    0x1f36666666361f00, // D
    0x7f46161e16467f00, // E
    0x7f46161e16060f00, // F
    0x3c66030373667c00, // G
    0x3333333f33333300, // H
    0x1e0c0c0c0c0c1e00, // I
    0x7830303033331e00, // J
    0x6766361e36666700, // K
    0x0f06060646667f00, // L
    0x63777f7f6b636300, // M
    0x63676f7b73636300, // N
    0x1c36636363361c00, // O
    0x3f66663e06060f00, // P
    0x1e3333333b1e3800, // Q
    0x3f66663e36666700, // R
    0x1e33070e38331e00, // S
    0x3f2d0c0c0c0c1e00, // T
    0x3333333333333f00, // U
    0x33333333331e0c00, // V
    0x6363636b7f776300, // W
    0x6363361c1c366300, // X
    0x3333331e0c0c1e00, // Y
    0x7f6331184c667f00, // Z
    0x1e06060606061e00, // [
    0x03060c1830604000, // \
    0x1e18181818181e00, // ]
    0x081c366300000000, // ^
    0x00000000000000ff, // _
    0x0c0c180000000000, // `
    0x00001e303e336e00, // a
    0x0706063e66663b00, // b
    0x00001e3303331e00, // c
    0x3830303e33336e00, // d
    0x00001e333f031e00, // e
    0x1c36060f06060f00, // f
    0x00006e33333e301f, // g
    0x0706366e66666700, // h
    0x0c000e0c0c0c1e00, // i
    0x300030303033331e, // j
    0x070666361e366700, // k
    0x0e0c0c0c0c0c1e00, // l
    0x0000337f7f6b6300, // m
    0x00001f3333333300, // n
    0x00001e3333331e00, // o
    0x00003b66663e060f, // p
    0x00006e33333e3078, // q
    0x00003b6e66060f00, // r
    0x00003e031e301f00, // s
    0x080c3e0c0c2c1800, // t
    0x0000333333336e00, // u
    0x00003333331e0c00, // v
    0x0000636b7f7f3600, // w
    0x000063361c366300, // x
    0x00003333333e301f, // y
    0x00003f190c263f00, // z
    0x380c0c070c0c3800, // {
    0x1818180018181800, // |
    0x070c0c380c0c0700, // }
    0x6e3b000000000000, // ~
];

fn char_code(c: char) -> Option<u64> {
    if (c as usize) < 0x20 || c as usize > 127 {
        None
    } else {
        Some(CHAR_ARRAY[c as usize - 0x20])
    }
}

fn char_map(c: char) -> Option<[bool; 64]> {
    char_code(c).map(|n| {
        let mut m = [false; 64];

        for shf0 in (0..8).rev() {
            for shf1 in 0..8 {
                let shf = shf0 * 8 + shf1;
                m[shf] = (n >> shf) & 1 == 1;
            }
        }

        m
    })
}

pub struct Moji {
    foreground: String,
    background: String,
    max_chars: usize,
}

impl Moji {
    pub fn new<T, U>(fg: T, bg: Option<U>, max_chars: usize) -> Self
    where
        T: Display,
        U: Display,
    {
        Self {
            foreground: format!(":{}:", fg),
            background: if let Some(bg) = bg {
                format!(":{}:", bg)
            } else {
                "      ".to_string()
            },
            max_chars,
        }
    }

    fn draw_row(&self, maps: &[[bool; 64]]) -> String {
        let mut out = String::new();

        for row in (0..8).rev() {
            for map in maps {
                for column in 0..8 {
                    out += if map[row * 8 + column] {
                        &self.foreground
                    } else {
                        &self.background
                    };
                }
            }

            out += "\n";
        }

        out
    }

    pub fn draw<T: ToString>(&self, s: T) -> String {
        let mut out = String::new();
        let maps = s.to_string().chars().filter_map(char_map).collect::<Vec<_>>();

        if self.max_chars == 0 {
            out += &self.draw_row(&maps);
        } else {
            for maps_ch in maps.chunks(self.max_chars) {
                out += &self.draw_row(maps_ch);
                out += "\n";
            }
        }

        out
    }
}
