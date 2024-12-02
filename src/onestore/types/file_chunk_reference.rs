use crate::errors::Result;
use crate::Reader;

/// A file chunk reference.
/// The two fields can be of variable size.
/// Already defined explicit FCRs are defined below as type aliases.
///
/// See [\[MS-ONESTORE\] 2.2.4].
///
/// [\[MS-ONESTORE\] 2.2.4]: https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-onestore/0d86b13d-d58c-44e8-b931-4728b9d39a4b

// This trait is what makes it possible for us to parse different-sized
// file chunk references. It also makes it possible to easily implement fcrNil and fcrZero,
// as seen in the impl.
trait FcrField {
    fn parse(reader: Reader) -> Self;
    fn is_zero(&self) -> bool;
}

impl FcrField for u32 {
    fn parse(reader: Reader) -> Self {
        reader.get_u32().unwrap()
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl FcrField for u64 {
    fn parse(reader: Reader) -> Self {
        reader.get_u64().unwrap()
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

/// A file chunk reference.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct FileChunkReference<A: FcrField, B: FcrField> {
    /// The offset in the file this FCR refers to, in bytes.
    stp: A,
    /// The size of the referenced data this FCR refers to, in bytes.
    cb: B,
}

/// A file chunk reference, where stp and cb are each 4 bytes long.
pub(crate) type FileChunkReference32 = FileChunkReference<u32, u32>;

/// A file chunk reference, where stp and cb are each 8 bytes long.
pub(crate) type FileChunkReference64 = FileChunkReference<u64, u64>;

/// A file node chunk reference. Both of its fields are of variable size.
pub(crate) type FileNodeChunkReference<A, B> = FileChunkReference<A, B>;

/// A file chunk reference, where the offset is 8 bytes long, the size 4 bytes long.
pub(crate) type FileChunkReference64x32 = FileChunkReference<u64, u32>;

impl<A: FcrField, B: FcrField> FileChunkReference<A, B> {
    pub(crate) fn parse(reader: Reader) -> Result<FileChunkReference<A, B>> {
        let stp: A = <A as FcrField>::parse(reader);
        let cb: B = <B as FcrField>::parse(reader);

        Ok(FileChunkReference { stp, cb })
    }

    pub(crate) fn is_fcr_nil(&self) -> bool {
        self.stp.is_zero()
    }

    pub(crate) fn is_fcr_zero(&self) -> bool {
        self.stp.is_zero() && self.cb.is_zero()
    }
}