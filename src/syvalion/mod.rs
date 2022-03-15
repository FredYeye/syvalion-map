mod image;

pub struct Syvalion
{
    rom: Vec<u8>,
}

impl Default for Syvalion
{
    fn default() -> Self
    {
        Self{ rom: std::fs::read("Syvalion (Japan).sfc").expect("Syvalion (Japan).sfc not found!") }
    }
}

impl Syvalion
{
    pub fn print_map_image(&self, chapter: u8)
    {
        let map = self.generate_map(chapter);
        let tile_map = self.generate_tile_map(map, chapter);

        let data = Syvalion::generate_image(tile_map, chapter);
        let chapter_name = format!("chapter_{chapter:02}.png");
        image::save_image(&chapter_name, data, 16 * 16 * 16, 16 * 16 * 16);
    }

    fn generate_map(&self, chapter: u8) -> [u8; 256]
    {
        let mut map = [0x15; 256];

        let mut offset = self.make_u16(0x06B831 + (chapter as u32 - 1) * 2);

        offset += 2; //not sure what the first value is

        let (mut x, mut y) = (1, 0);

        loop
        {
            let screen = self.get_u8(0x060000 | offset as u32);
            let direction = self.get_u8(0x060000 | offset as u32 + 1);

            map[y * 16 + x] = screen;

            match direction & 0b11
            {
                0 => x = (x + 1)           & 0b1111,
                1 => y = y.wrapping_sub(1) & 0b1111,
                2 => x = x.wrapping_sub(1) & 0b1111,
                3 => y = (y + 1)           & 0b1111,

                _ => unreachable!(),
            }

            if direction & 4 != 0
            {
                break;
            }

            offset += 2;
        }

        map
    }

    fn generate_tile_map(&self, map: [u8; 256], chapter: u8) -> [u8; 256 * 256]
    {
        let mut tile_map = [0; 256 * 256];

        let offset =
        {
            let offset1 = self.make_u16(0x06B831 + (chapter as u32 - 1) * 2);
            let offset2 = self.get_u8(0x060000 | offset1 as u32);
            self.make_u24(0x06C7FF + offset2 as u32 * 4)
        };

        for y in 0 .. 16
        {
            for line in 0 .. 16
            {
                for x in 0 .. 16
                {
                    let ofs = x * 16 + line * 256 + y * 256 * 16;
                    tile_map[ofs .. ofs + 16].copy_from_slice(self.get_screen_line(map[y * 16 + x], offset, line as u8));
                }
            }
        }

        tile_map
    }

    fn get_screen_line(&self, screen: u8, base_offset: u32, line_offset: u8) -> &[u8]
    {
        let offset = base_offset + screen as u32 * 0x100 + line_offset as u32 * 0x10;
        &self.rom[Syvalion::snes_to_effective(offset) .. Syvalion::snes_to_effective(offset + 0x10)]
    }

    fn generate_image(tile_map: [u8; 256 * 256], chapter: u8) -> Vec<u8>
    {
        let mut output = vec![0; 4096 * 4096 * 3];

        let chapter_tile_set = [1, 2, 3, 1, 4];

        let tile_name = format!("tile{}.png", chapter_tile_set[chapter as usize - 1]);
        let tiles = image::read_image(&tile_name);

        for y in 0 .. 256
        {
            for x in 0 .. 256
            {
                let offset = y * 256 * 16 * 16 * 3 + x * 16 * 3;
                let tile_offset = tile_map[y * 256 + x];
                let (tx, ty) = (tile_offset & 0b111, tile_offset >> 3);

                for line in 0 .. 16
                {
                    let line_offset = offset + line * 256 * 16 * 3;
                    let asd = tx as usize * 16 * 3 + ty as usize * 8 * 16 * 16 * 3 + line * 128 * 3;

                    output[line_offset .. line_offset + 16 * 3].copy_from_slice(&tiles[asd .. asd + 16 * 3]);
                }
            }
        }

        output
    }

    fn snes_to_effective(address: u32) -> usize
    {
        let bank = (address >> 16) * 0x8000;
        let offset = (address - 0x8000) & 0xFFFF;
        (bank + offset) as usize
    }

    fn get_u8(&self, snes_address: u32) -> u8
    {
        self.rom[Syvalion::snes_to_effective(snes_address)]
    }

    fn make_u16(&self, snes_address: u32) -> u16
    {
        let offset = Syvalion::snes_to_effective(snes_address);
        u16::from_le_bytes
        (
            [
                self.rom[offset    ],
                self.rom[offset + 1],
            ]
        )
    }

    fn make_u24(&self, snes_address: u32) -> u32
    {
        let offset = Syvalion::snes_to_effective(snes_address);
        u32::from_le_bytes
        (
            [
                self.rom[offset    ],
                self.rom[offset + 1],
                self.rom[offset + 2],
                0,
            ]
        )
    }
}
