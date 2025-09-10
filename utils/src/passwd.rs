use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use xenia::{Gid, Uid};

#[allow(unused)]
#[derive(Debug)]
pub struct Passwd<'a> {
    pub name: &'a str,
    pub passwd: &'a str,
    pub uid: Uid,
    pub gid: Gid,
    pub gecos: &'a str,
    pub dir: &'a str,
    pub shell: &'a str,
}

impl Passwd<'_> {
    #[inline]
    pub fn entries() -> Result<PasswdEntries<BufReader<File>>, io::Error> {
        PasswdEntries::new()
    }

    fn from_buf<'a>(buf: &'a str) -> Option<Passwd<'a>> {
        let mut entries = buf.splitn(7, ':');

        let name = entries.next()?;
        let passwd = entries.next()?;

        let uid = entries
            .next()?
            .parse()
            .map(|uid| unsafe { Uid::from_raw(uid) })
            .ok()?;

        let gid = entries
            .next()?
            .parse()
            .map(|gid| unsafe { Gid::from_raw(gid) })
            .ok()?;

        let gecos = entries.next()?;
        let dir = entries.next()?;
        let shell = entries.next()?;

        Some(Passwd {
            name,
            passwd,
            uid,
            gid,
            gecos,
            dir,
            shell,
        })
    }
}

pub struct PasswdEntries<T> {
    reader: T,
    buf: String,
}

impl PasswdEntries<BufReader<File>> {
    #[inline]
    pub fn new() -> Result<Self, io::Error> {
        Self::from_reader(BufReader::new(File::open("/etc/passwd")?))
    }
}

impl<T: BufRead> PasswdEntries<T> {
    pub fn from_reader(reader: T) -> Result<Self, io::Error> {
        Ok(Self {
            reader,
            buf: String::new(),
        })
    }

    pub fn next_entry(&mut self) -> Result<Option<Passwd<'_>>, io::Error> {
        self.buf.clear();

        self.reader.read_line(&mut self.buf)?;

        Ok(Passwd::from_buf(&self.buf))
    }
}
