#![allow(dead_code)]
#![allow(unused_imports)]

use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry,
    ReplyOpen, ReplyWrite, Request,
};
use libc::{c_int, EEXIST, ENOENT};
use std::env;
use std::ffi::{OsStr, OsString};
use std::io::Write;

const TTL: time::Timespec = time::Timespec { sec: 1, nsec: 0 }; // 1 second
const TIMESPEC: time::Timespec = time::Timespec { sec: 0, nsec: 0 };

fn make_entry(ino: usize) -> FileAttr {
    FileAttr {
        ino: (ino + 10) as u64,
        size: 0,
        blocks: 1,
        atime: TIMESPEC, // 1970-01-01 00:00:00
        mtime: TIMESPEC,
        ctime: TIMESPEC,
        crtime: TIMESPEC,
        kind: FileType::RegularFile,
        perm: 0o644,
        nlink: 1,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    }
}

struct FakeFS {
    files: Vec<OsString>,
}

impl Filesystem for FakeFS {
    fn open(&mut self, _req: &Request, ino: u64, _flags: u32, reply: ReplyOpen) {
        reply.opened(ino, 0);
    }

    fn write(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        data: &[u8],
        _flags: u32,
        reply: ReplyWrite,
    ) {
        std::io::stdout().lock().write_all(data).unwrap();
        reply.written(data.len() as u32);
    }

    fn lookup(&mut self, _req: &Request, _parent: u64, name: &OsStr, reply: ReplyEntry) {
        if let Some(pos) = self.files.iter().position(|f| f == name) {
            reply.entry(&TTL, &make_entry(pos), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn mknod(
        &mut self,
        _req: &Request,
        _parent: u64,
        name: &OsStr,
        _mode: u32,
        _rdev: u32,
        reply: ReplyEntry,
    ) {
        if let Some(_) = self.files.iter().position(|f| f == name) {
            reply.error(EEXIST);
        } else {
            self.files.push(name.to_os_string());
            reply.entry(&TTL, &make_entry(self.files.len() - 1), 0);
        }
    }

    fn rename(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _newparent: u64,
        _newname: &OsStr,
        reply: ReplyEmpty,
    ) {
        reply.ok()
    }
}

fn main() {
    env_logger::init().unwrap();

    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "fsname=ffs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    fuse::mount(FakeFS { files: vec![] }, &mountpoint, &options).unwrap();
}
