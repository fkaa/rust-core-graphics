// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

use libc;

pub use self::CGError::*;

// TODO: this is actually a libc::c_float on 32bit
pub type CGFloat = libc::c_double;
pub type CGAffineTransform = ();

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum CGError {
    kCGErrorSuccess = 0,
    kCGErrorFailure = 1000,
    kCGErrorIllegalArgument = 1001,
    kCGErrorInvalidConnection = 1002,
    kCGErrorInvalidContext = 1003,
    kCGErrorCannotComplete = 1004,
    kCGErrorNameTooLong = 1005,
    kCGErrorNotImplemented = 1006,
    kCGErrorRangeCheck = 1007,
    kCGErrorTypeCheck = 1008,
    kCGErrorNoCurrentPoint = 1009,
    kCGErrorInvalidOperation = 1010,
    kCGErrorNoneAvailable = 1011,
    kCGErrorApplicationRequiresNewerSystem = 1015,
    kCGErrorApplicationNotPermittedToExecute = 1016,
    kCGErrorApplicationIncorrectExecutableFormatFound = 1023,
    kCGErrorApplicationIsLaunching = 1024,
    kCGErrorApplicationAlreadyRunning = 1025,
    kCGErrorApplicationCanOnlyBeRunInOneSessionAtATime = 1026,
    kCGErrorClassicApplicationsMustBeLaunchedByClassic = 1027,
    kCGErrorForkFailed = 1028,
    kCGErrorRetryRegistration = 1029,
}

fn cg_desc(err: CGError) -> &'static str {
    match err {
        kCGErrorSuccess => "The requested operation was completed successfully.",
        kCGErrorFailure => "A general failure occurred.",
        kCGErrorIllegalArgument => "One or more of the parameters passed to a function are invalid. Check for NULL pointers.",
        kCGErrorInvalidConnection => "The parameter representing a connection to the window server is invalid.",
        kCGErrorInvalidContext => "The CPSProcessSerNum or context identifier parameter is not valid.",
        kCGErrorCannotComplete => "The requested operation is inappropriate for the parameters passed in, or the current system state.",
        kCGErrorNameTooLong => "A parameter, typically a C string, is too long to be used without truncation.",
        kCGErrorNotImplemented => "Return value from obsolete function stubs present for binary compatibility, but not normally called.",
        kCGErrorRangeCheck => "A parameter passed in has a value that is inappropriate, or which does not map to a useful operation or value.",
        kCGErrorTypeCheck => "A data type or token was encountered that did not match the expected type or token.",
        kCGErrorNoCurrentPoint => "An operation relative to a known point or coordinate could not be done, as there is no known point.",
        kCGErrorInvalidOperation => "The requested operation is not valid for the parameters passed in, or the current system state.",
        kCGErrorNoneAvailable => "The requested operation could not be completed as the indicated resources were not found.",
        kCGErrorApplicationRequiresNewerSystem => "The application requires a newer version of the operating system to run than is currently available.",
        kCGErrorApplicationNotPermittedToExecute => "The application is not permitted to run.",
        kCGErrorApplicationIncorrectExecutableFormatFound => "The application does not have any executable code for the current system.",
        kCGErrorApplicationIsLaunching => "The application is in the process of launching but has not yet checked in with the window server.",
        kCGErrorApplicationAlreadyRunning => "The application being launched was already running and checked in with the window server.",
        kCGErrorApplicationCanOnlyBeRunInOneSessionAtATime => "The application being launched is incompatible with multiple user sessions and is already running in another user session.",
        kCGErrorClassicApplicationsMustBeLaunchedByClassic => "The window server could not launch the application. It must be launched by the Classic environment.",
        kCGErrorForkFailed => "The system was unable to fork a new process to launch the application.",
        kCGErrorRetryRegistration => "The application should retry its registration shortly.",
    }
}


impl CGError {
    pub fn desc(self) -> &'static str {
        cg_desc(self)
    }

    pub fn from_i32(val: i32) -> CGError {
        match val {
            1000 => kCGErrorFailure,
            1001 => kCGErrorIllegalArgument,
            1002 => kCGErrorInvalidConnection,
            1003 => kCGErrorInvalidContext,
            1004 => kCGErrorCannotComplete,
            1005 => kCGErrorNameTooLong,
            1006 => kCGErrorNotImplemented,
            1007 => kCGErrorRangeCheck,
            1008 => kCGErrorTypeCheck,
            1009 => kCGErrorNoCurrentPoint,
            1010 => kCGErrorInvalidOperation,
            1011 => kCGErrorNoneAvailable,
            1015 => kCGErrorApplicationRequiresNewerSystem,
            1016 => kCGErrorApplicationNotPermittedToExecute,
            1023 => kCGErrorApplicationIncorrectExecutableFormatFound,
            1024 => kCGErrorApplicationIsLaunching,
            1025 => kCGErrorApplicationAlreadyRunning,
            1026 => kCGErrorApplicationCanOnlyBeRunInOneSessionAtATime,
            1027 => kCGErrorClassicApplicationsMustBeLaunchedByClassic,
            1028 => kCGErrorForkFailed,
            1029 => kCGErrorRetryRegistration,
            _ => panic!("Invalid error code: {}", val)
        }
    }
}
