use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Mods: u32 {
        const NONE           = 0;
        const NO_FAIL        = 1;
        const EASY           = 2;
        const TOUCH_DEVICE   = 4;
        const HIDDEN         = 8;
        const HARD_ROCK      = 16;
        const SUDDEN_DEATH   = 32;
        const DOUBLE_TIME    = 64;
        const RELAX          = 128;
        const HALF_TIME      = 256;
        const NIGHTCORE      = 512; // Only set along with DOUBLE_TIME. i.e: NC only gives 576
        const FLASHLIGHT     = 1024;
        const AUTOPLAY       = 2048;
        const SPUN_OUT       = 4096;
        const RELAX2         = 8192;    // Autopilot
        const PERFECT        = 16384; // Only set along with SUDDEN_DEATH. i.e: PF only gives 16416
        const KEY4           = 32768;
        const KEY5           = 65536;
        const KEY6           = 131072;
        const KEY7           = 262144;
        const KEY8           = 524288;
        const FADE_IN        = 1048576;
        const RANDOM         = 2097152;
        const CINEMA         = 4194304;
        const TARGET         = 8388608;
        const KEY9           = 16777216;
        const KEY_COOP       = 33554432;
        const KEY1           = 67108864;
        const KEY3           = 134217728;
        const KEY2           = 268435456;
        const SCORE_V2       = 536870912;
        const MIRROR         = 1073741824;

        // 复合标志 - 正确的语法
        const KEY_MOD = Self::KEY1.bits() | Self::KEY2.bits() | Self::KEY3.bits() |
                        Self::KEY4.bits() | Self::KEY5.bits() | Self::KEY6.bits() |
                        Self::KEY7.bits() | Self::KEY8.bits() | Self::KEY9.bits() |
                        Self::KEY_COOP.bits();

        const FREE_MOD_ALLOWED = Self::NO_FAIL.bits() | Self::EASY.bits() | Self::HIDDEN.bits() |
                                Self::HARD_ROCK.bits() | Self::SUDDEN_DEATH.bits() |
                                Self::FLASHLIGHT.bits() | Self::FADE_IN.bits() |
                                Self::RELAX.bits() | Self::RELAX2.bits() |
                                Self::SPUN_OUT.bits() | Self::KEY_MOD.bits();

        const SCORE_INCREASE_MODS = Self::HIDDEN.bits() | Self::HARD_ROCK.bits() |
                                   Self::DOUBLE_TIME.bits() | Self::FLASHLIGHT.bits() |
                                   Self::FADE_IN.bits();
    }
}

// Helper methods for checking special combinations
impl Mods {
    pub fn has_nightcore_double_time(&self) -> bool {
        self.contains(Mods::NIGHTCORE) && self.contains(Mods::DOUBLE_TIME)
    }

    pub fn has_perfect_sudden_death(&self) -> bool {
        self.contains(Mods::PERFECT) && self.contains(Mods::SUDDEN_DEATH)
    }
}
