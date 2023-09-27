// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// This file was generated automatically.
// Please do not modify content of this file directly.
// File generated by using template: "host_toplevel.rs.tpl"
// To regenerate this file follow OpenTitan topgen documentations.

#![allow(dead_code)]

use crate::with_unknown;

with_unknown! {
    pub enum PinmuxPeripheralIn: u32 [default = Self::End] {
        GpioGpio0 = 0,
        GpioGpio1 = 1,
        GpioGpio2 = 2,
        GpioGpio3 = 3,
        GpioGpio4 = 4,
        GpioGpio5 = 5,
        GpioGpio6 = 6,
        GpioGpio7 = 7,
        GpioGpio8 = 8,
        GpioGpio9 = 9,
        GpioGpio10 = 10,
        GpioGpio11 = 11,
        GpioGpio12 = 12,
        GpioGpio13 = 13,
        GpioGpio14 = 14,
        GpioGpio15 = 15,
        GpioGpio16 = 16,
        GpioGpio17 = 17,
        GpioGpio18 = 18,
        GpioGpio19 = 19,
        GpioGpio20 = 20,
        GpioGpio21 = 21,
        GpioGpio22 = 22,
        GpioGpio23 = 23,
        GpioGpio24 = 24,
        GpioGpio25 = 25,
        GpioGpio26 = 26,
        GpioGpio27 = 27,
        GpioGpio28 = 28,
        GpioGpio29 = 29,
        GpioGpio30 = 30,
        GpioGpio31 = 31,
        I2c0Sda = 32,
        I2c0Scl = 33,
        Uart0Rx = 34,
        SpiDeviceTpmCsb = 35,
        End = 36,
    }

    pub enum PinmuxInsel: u32 [default = Self::End] {
        ConstantZero = 0,
        ConstantOne = 1,
        Ioa0 = 2,
        Ioa1 = 3,
        Ioa2 = 4,
        Ioa3 = 5,
        Ioa4 = 6,
        Ioa5 = 7,
        Ioa6 = 8,
        Ioa7 = 9,
        Ioa8 = 10,
        Iob0 = 11,
        Iob1 = 12,
        Iob2 = 13,
        Iob3 = 14,
        Iob4 = 15,
        Iob5 = 16,
        Iob6 = 17,
        Iob7 = 18,
        Iob8 = 19,
        Iob9 = 20,
        Iob10 = 21,
        Iob11 = 22,
        Iob12 = 23,
        Ioc0 = 24,
        Ioc1 = 25,
        Ioc2 = 26,
        Ioc3 = 27,
        Ioc4 = 28,
        Ioc5 = 29,
        Ioc6 = 30,
        Ioc7 = 31,
        Ioc8 = 32,
        Ioc9 = 33,
        Ioc10 = 34,
        Ioc11 = 35,
        Ioc12 = 36,
        Ior0 = 37,
        Ior1 = 38,
        Ior2 = 39,
        Ior3 = 40,
        Ior4 = 41,
        Ior5 = 42,
        Ior6 = 43,
        Ior7 = 44,
        Ior10 = 45,
        Ior11 = 46,
        Ior12 = 47,
        Ior13 = 48,
        End = 49,
    }

    pub enum PinmuxMioOut: u32 [default = Self::End] {
        Ioa0 = 0,
        Ioa1 = 1,
        Ioa2 = 2,
        Ioa3 = 3,
        Ioa4 = 4,
        Ioa5 = 5,
        Ioa6 = 6,
        Ioa7 = 7,
        Ioa8 = 8,
        Iob0 = 9,
        Iob1 = 10,
        Iob2 = 11,
        Iob3 = 12,
        Iob4 = 13,
        Iob5 = 14,
        Iob6 = 15,
        Iob7 = 16,
        Iob8 = 17,
        Iob9 = 18,
        Iob10 = 19,
        Iob11 = 20,
        Iob12 = 21,
        Ioc0 = 22,
        Ioc1 = 23,
        Ioc2 = 24,
        Ioc3 = 25,
        Ioc4 = 26,
        Ioc5 = 27,
        Ioc6 = 28,
        Ioc7 = 29,
        Ioc8 = 30,
        Ioc9 = 31,
        Ioc10 = 32,
        Ioc11 = 33,
        Ioc12 = 34,
        Ior0 = 35,
        Ior1 = 36,
        Ior2 = 37,
        Ior3 = 38,
        Ior4 = 39,
        Ior5 = 40,
        Ior6 = 41,
        Ior7 = 42,
        Ior10 = 43,
        Ior11 = 44,
        Ior12 = 45,
        Ior13 = 46,
        End = 47,
    }

    pub enum PinmuxOutsel: u32 [default = Self::End] {
        ConstantZero = 0,
        ConstantOne = 1,
        ConstantHighZ = 2,
        GpioGpio0 = 3,
        GpioGpio1 = 4,
        GpioGpio2 = 5,
        GpioGpio3 = 6,
        GpioGpio4 = 7,
        GpioGpio5 = 8,
        GpioGpio6 = 9,
        GpioGpio7 = 10,
        GpioGpio8 = 11,
        GpioGpio9 = 12,
        GpioGpio10 = 13,
        GpioGpio11 = 14,
        GpioGpio12 = 15,
        GpioGpio13 = 16,
        GpioGpio14 = 17,
        GpioGpio15 = 18,
        GpioGpio16 = 19,
        GpioGpio17 = 20,
        GpioGpio18 = 21,
        GpioGpio19 = 22,
        GpioGpio20 = 23,
        GpioGpio21 = 24,
        GpioGpio22 = 25,
        GpioGpio23 = 26,
        GpioGpio24 = 27,
        GpioGpio25 = 28,
        GpioGpio26 = 29,
        GpioGpio27 = 30,
        GpioGpio28 = 31,
        GpioGpio29 = 32,
        GpioGpio30 = 33,
        GpioGpio31 = 34,
        I2c0Sda = 35,
        I2c0Scl = 36,
        Uart0Tx = 37,
        SensorCtrlAstDebugOut0 = 38,
        SensorCtrlAstDebugOut1 = 39,
        SensorCtrlAstDebugOut2 = 40,
        SensorCtrlAstDebugOut3 = 41,
        SensorCtrlAstDebugOut4 = 42,
        SensorCtrlAstDebugOut5 = 43,
        SensorCtrlAstDebugOut6 = 44,
        SensorCtrlAstDebugOut7 = 45,
        SensorCtrlAstDebugOut8 = 46,
        OtpCtrlTest0 = 47,
        End = 48,
    }

    pub enum DirectPads: u32 [default = Self::End] {
        SpiHost0Sd0 = 0,
        SpiHost0Sd1 = 1,
        SpiHost0Sd2 = 2,
        SpiHost0Sd3 = 3,
        SpiDeviceSd0 = 4,
        SpiDeviceSd1 = 5,
        SpiDeviceSd2 = 6,
        SpiDeviceSd3 = 7,
        SpiDeviceSck = 8,
        SpiDeviceCsb = 9,
        SpiHost0Sck = 10,
        SpiHost0Csb = 11,
        End = 12,
    }

    pub enum MuxedPads: u32 [default = Self::End] {
        Ioa0 = 0,
        Ioa1 = 1,
        Ioa2 = 2,
        Ioa3 = 3,
        Ioa4 = 4,
        Ioa5 = 5,
        Ioa6 = 6,
        Ioa7 = 7,
        Ioa8 = 8,
        Iob0 = 9,
        Iob1 = 10,
        Iob2 = 11,
        Iob3 = 12,
        Iob4 = 13,
        Iob5 = 14,
        Iob6 = 15,
        Iob7 = 16,
        Iob8 = 17,
        Iob9 = 18,
        Iob10 = 19,
        Iob11 = 20,
        Iob12 = 21,
        Ioc0 = 22,
        Ioc1 = 23,
        Ioc2 = 24,
        Ioc3 = 25,
        Ioc4 = 26,
        Ioc5 = 27,
        Ioc6 = 28,
        Ioc7 = 29,
        Ioc8 = 30,
        Ioc9 = 31,
        Ioc10 = 32,
        Ioc11 = 33,
        Ioc12 = 34,
        Ior0 = 35,
        Ior1 = 36,
        Ior2 = 37,
        Ior3 = 38,
        Ior4 = 39,
        Ior5 = 40,
        Ior6 = 41,
        Ior7 = 42,
        Ior10 = 43,
        Ior11 = 44,
        Ior12 = 45,
        Ior13 = 46,
        End = 47,
    }
}

#[allow(non_camel_case_types)]
pub mod ujson_alias {
    use super::*;
    // Create aliases for the C names of these types so that the ujson
    // created structs can access these structures by their C names.
    pub type pinmux_peripheral_in_t = PinmuxPeripheralIn;
    pub type pinmux_insel_t = PinmuxInsel;
    pub type pinmux_mio_out_t = PinmuxMioOut;
    pub type pinmux_outsel_t = PinmuxOutsel;
}