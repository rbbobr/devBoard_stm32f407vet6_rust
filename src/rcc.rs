

mod use_rcc{
    use embassy_stm32::Config;
    use embassy_stm32::time::Hertz;

    pub fn rcc_init () -> Config {            
        
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(25_000_000),
                mode: HseMode::Oscillator,
            });
            config.rcc.pll_src = PllSource::HSE;
            config.rcc.pll = Some(Pll {
                prediv: PllPreDiv::DIV25,
                mul: PllMul::MUL336,
                divp: Some(PllPDiv::DIV2), // 25mhz / 25 * 336 / 2 = 168Mhz.
                divq: Some(PllQDiv::DIV7), // 25mhz / 4 * 336 / 7 = 48Mhz.
                divr: None,
            });
            config.rcc.ahb_pre = AHBPrescaler::DIV1;
            config.rcc.apb1_pre = APBPrescaler::DIV4;
            config.rcc.apb2_pre = APBPrescaler::DIV2;
            config.rcc.sys = Sysclk::PLL1_P;
        }
        
        return config;
    }
}