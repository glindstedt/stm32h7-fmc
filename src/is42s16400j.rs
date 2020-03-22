/// ISI IS42S16400J SDRAM
#[allow(unused)]

/// Speed Grade 7
pub mod is42s16400j_7 {
    use crate::sdram::{FmcSdramConfiguration, FmcSdramTiming, SdramChip};

    const BURST_LENGTH_1: u16 = 0x0000;
    const BURST_LENGTH_2: u16 = 0x0001;
    const BURST_LENGTH_4: u16 = 0x0002;
    const BURST_LENGTH_8: u16 = 0x0004;
    const BURST_TYPE_SEQUENTIAL: u16 = 0x0000;
    const BURST_TYPE_INTERLEAVED: u16 = 0x0008;
    const CAS_LATENCY_2: u16 = 0x0020;
    const CAS_LATENCY_3: u16 = 0x0030;
    const OPERATING_MODE_STANDARD: u16 = 0x0000;
    const WRITEBURST_MODE_PROGRAMMED: u16 = 0x0000;
    const WRITEBURST_MODE_SINGLE: u16 = 0x0200;

    /// Is42s16400j with Speed Grade 7
    ///
    /// Configured with CAS latency 2, limited 100MHz
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Is42s16400j {}

    impl SdramChip for Is42s16400j {
        /// Value of the mode register
        const MODE_REGISTER: u16 = BURST_LENGTH_1
            | BURST_TYPE_SEQUENTIAL
            | CAS_LATENCY_2
            | OPERATING_MODE_STANDARD
            | WRITEBURST_MODE_SINGLE;

        /// Timing Parameters
        const TIMING: FmcSdramTiming = FmcSdramTiming {
            startup_delay_ns: 100_000,    // 100 µs
            max_sd_clock_hz: 100_000_000, // 100 MHz
            refresh_period_ns: 15_625,    // 64ms / (4096 rows) = 15625ns
            mode_register_to_active: 2,   // tMRD = 2 cycles
            exit_self_refresh: 7,         // tXSR = 70ns
            active_to_precharge: 4,       // tRAS = 42ns
            row_cycle: 7,                 // tRC = 63ns
            row_precharge: 2,             // tRP = 15ns
            row_to_column: 2,             // tRCD = 15ns
        };

        /// SDRAM controller configuration
        const CONFIG: FmcSdramConfiguration = FmcSdramConfiguration {
            column_bits: 8,
            row_bits: 12,
            memory_data_width: 16, // 16-bit
            internal_banks: 4,     // 4 internal banks
            cas_latency: 2,        // CAS latency = 2
            write_protection: false,
            sd_clock_divide: 2, // Divide by 2
            read_burst: true,
            read_pipe_delay_cycles: 0,
        };
    }
}
