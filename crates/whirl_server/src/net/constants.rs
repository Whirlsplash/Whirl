// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! Despite `crate::cmd::constants` being ported from a series of constants to
//! a single `enum`, this module may not be ported to one *yet* as Rust does
//! not allows for duplicate discriminant values in `enum`s:
//! <https://doc.rust-lang.org/error-index.html#E0081>.

#![allow(dead_code)]

pub const VAR_PROTOCOL_VERSION: i32 = 24;
pub const STATECMD: i32 = 2;
pub const MAXCMD: i32 = 255;
pub const CURRENT_ROOM: i32 = 253;
pub const CLIENT: i32 = 1;
pub const CO: i32 = 254;
pub const PO: i32 = 255;
pub const VAR_APPNAME: i32 = 1;
pub const VAR_USERNAME: i32 = 2;
pub const VAR_PROTOCOL: i32 = 3;
pub const VAR_ERROR: i32 = 4;
pub const VAR_CHANNEL: i32 = 5;
pub const VAR_BITMAP: i32 = 5;
pub const VAR_PASSWORD: i32 = 6;
pub const VAR_AVATARS: i32 = 7;
pub const VAR_UPDATETIME: i32 = 8;
pub const VAR_CLIENT: i32 = 9;
pub const VAR_SERIAL: i32 = 10;
pub const VAR_EMAIL: i32 = 11;
pub const VAR_LOGONOFF: i32 = 12;
pub const VAR_DURATION: i32 = 13;
pub const VAR_GUEST: i32 = 14;
pub const VAR_SERVERTYPE: i32 = 15;
pub const VAR_VIZCARD: i32 = 16;
pub const VAR_NEW_PASSWD: i32 = 20;
pub const VAR_PRIV: i32 = 22;
pub const VAR_ASLEEP: i32 = 23;
pub const VAR_EXTERNAL_HTTP_SERVER: i32 = 24;
pub const VAR_SCRIPT_SERVER: i32 = 25;
pub const VAR_SMTP_SERVER: i32 = 26;
pub const VAR_MAIL_DOMAIN: i32 = 27;
pub const VAR_NEW_USERNAME: i32 = 28;
pub const VAR_INTERNAL_HTTP_SERVER: i32 = 29;
pub const VAR_INVENTORY: i32 = 32;
pub const ACK: i32 = 0;
pub const NAK_BAD_USER: i32 = 1;
pub const NAK_MAX_ORDINARY: i32 = 2;
pub const NAK_MAX_PRIORITY: i32 = 3;
pub const NAL_BAD_WORLD: i32 = 4;
pub const NAK_FATAIL: i32 = 5;
pub const NAK_BAD_PROTOCOL: i32 = 6;
pub const NAK_BAD_CLIENTSW: i32 = 7;
pub const NAK_BAD_ROOM: i32 = 8;
pub const NAK_BAD_SERIAL: i32 = 9;
pub const NAK_TAKEN_SERIAL: i32 = 10;
pub const NAK_TAKEN_USER: i32 = 11;
pub const NAK_NO_SUCH_USER: i32 = 12;
pub const NAK_BAD_PASSWORD: i32 = 13;
pub const NAK_BAD_ACCOUNT: i32 = 14;
pub const NAK_NOT_LOGGEDON: i32 = 15;
pub const NAK_BAD_IPADDRESS: i32 = 16;
pub const NAK_LOGGEDON: i32 = 17;
pub const NAK_CRYPT_METHOD: i32 = 18;
pub const NAK_CRYPT_ERROR: i32 = 19;
pub const NAK_SESSIONINIT: i32 = 20;
pub const NAK_ROOM_FULL: i32 = 21;
pub const NAK_SHUTDOWN: i32 = 100;
pub const NAK_WRITE_ERROR: i32 = 101;
pub const NAK_READ_ERROR: i32 = 102;
pub const NAK_UNEXPECTED: i32 = 103;
pub const NAK_CONNECTION: i32 = 104;
pub const NAK_IOSTREAMS: i32 = 105;
pub const NAK_TIMEOUT: i32 = 106;
pub const NAK_UNREACHABLE: i32 = 107;
pub const STATUS_CONNECTED: i32 = 200;
pub const STATUS_DETACHING: i32 = 201;
pub const STATUS_WILLRETRY: i32 = 202;
pub const STATUS_DISCONNECTED: i32 = 203;
pub const STATUS_DEAD: i32 = 204;
pub const STATUS_OFFLINE: i32 = 205;
pub const STATUS_GALAXY_ONLINE: i32 = 206;
pub const STATUS_GALAXY_OFFLINE: i32 = 206;
pub const PROPFLAG_BINARY: i32 = 16;
pub const PROPFLAG_FINGER: i32 = 32;
pub const PROPFLAG_AUTOUPDATE: i32 = 64;
pub const PROPFLAG_DBSTORE: i32 = 128;
pub const PROPACCESS_POSSESS: i32 = 1;
pub const PROPACCESS_PRIVATE: i32 = 2;
pub const SERVER_UNKNOWN: i32 = 0;
pub const USER_SERVER_DB: i32 = 1;
pub const USER_SERVER_ANON: i32 = 2;
pub const ROOM_SERVER_US: i32 = 3;
pub const ROOM_SERVER_ANON: i32 = 4;
pub const PRIV_NONE: i32 = 0;
pub const PRIV_BUILD: i32 = 1;
pub const PRIV_BROADCAST: i32 = 2;
pub const PRIV_PROPERTY: i32 = 4;
pub const PRIV_VIP: i32 = 8;
pub const PRIV_VIP2: i32 = 16;
pub const PRIV_SPECIALGUEST: i32 = 64;
