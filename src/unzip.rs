use crate::files;
use crate::files::File;
use eyre::{eyre, ContextCompat, Result, WrapErr};
use log::{debug, error};
use normalize_path::NormalizePath;
use std::fmt::Debug;
use std::fs;
use std::io::{Cursor, Read, SeekFrom};
use std::path::Path;

const MAGIC_EOCD: u32 = 0x06054b50_u32;
const MAGIC_CD: u32 = 0x02014b50_u32;
const MAGIC_LFH: u32 = 0x04034b50_u32;
const EOCD_SIZE: usize = 4 + 2 + 2 + 2 + 2 + 4 + 4 + 2;

pub struct ZipFile {
    file: files::File,
    out_dir: String,
}

#[derive(Debug)]
pub struct EndOfCentralDirectory {
    central_directory_entries: u16,
    size_of_central_directory: u32,
    offset_of_central_directory: u32,
    comment: Option<String>,
}

#[derive(Debug)]
pub struct CentralDirectoryEntry {
    crc32: u32,
    comp_size: u32,
    uncomp_size: u32,
    offset: u32,
    name: String,
    extra: Option<Vec<u8>>,
    comment: Option<String>,
}

impl ZipFile {
    pub fn open(in_file_name: &str, out_dir_name: &str) -> Result<Self> {
        let file = files::File::open(in_file_name)
            .wrap_err_with(|| format!("Failed to open {in_file_name:?}"))?;
        let out_dir = String::from(out_dir_name);
        Ok(Self { file, out_dir })
    }

    pub fn find_end_of_central_directory(&mut self) -> Result<EndOfCentralDirectory> {
        let mut ptr = EOCD_SIZE as i64;
        let sf = &mut self.file;

        while ptr > 0 {
            sf.seek(SeekFrom::End(-ptr))
                .wrap_err("Failed to seek to ZIP end")?;
            if sf.read_u32le().wrap_err("Failed to read MAGIC_EOCD")? == MAGIC_EOCD {
                let num_this_disk = sf
                    .read_u16le()
                    .wrap_err("Failed read number of this disk")?;
                if num_this_disk > 1 {
                    return Err(eyre!("Multi-disk unsupported {num_this_disk}"));
                }
                let num_disk_with_cd = sf.read_u16le().wrap_err(
                    "Failed read number of the disk with the start of the central directory",
                )?;
                if num_disk_with_cd > 1 {
                    return Err(eyre!("Multi-disk unsupported {num_disk_with_cd}"));
                }
                let central_directory_entries_this_disk = sf.read_u16le().wrap_err(
                    "Failed read total number of entries in the central directory on this disk",
                )?;
                let central_directory_entries = sf
                    .read_u16le()
                    .wrap_err("Failed read total number of entries in the central directory")?;
                if central_directory_entries_this_disk != central_directory_entries {
                    return Err(eyre!(
                        "Multi-disk unsupported {central_directory_entries_this_disk}!={central_directory_entries}"
                    ));
                }
                let size_of_central_directory = sf
                    .read_u32le()
                    .wrap_err("Failed read size of the central directory")?;
                let offset_of_central_directory = sf
                    .read_u32le()
                    .wrap_err("Failed read offset of start of central directory with respect to the starting disk number")?;

                let comment_length = sf
                    .read_u16le()
                    .wrap_err("Failed read .ZIP file comment length")?;

                let comment = if comment_length == 0 {
                    None
                } else {
                    let comment_bin = sf.read_as_vec(comment_length as usize)?;
                    Some(String::from_utf8(comment_bin).wrap_err("EOCD comment not UTF-8")?)
                };

                let eocd = EndOfCentralDirectory {
                    central_directory_entries,
                    size_of_central_directory,
                    offset_of_central_directory,
                    comment,
                };
                debug!("EOCD record: {eocd:?}");
                return Ok(eocd);
            }
            ptr += 1;
        }

        Err(eyre!("No EndOfCentralDirectory record found"))
    }

    pub fn parse_central_directory(
        &mut self,
        eocd: &EndOfCentralDirectory,
    ) -> Result<Vec<CentralDirectoryEntry>> {
        let mut entries = Vec::with_capacity(eocd.central_directory_entries as usize);
        let sf = &mut self.file;
        sf.seek(SeekFrom::Start(eocd.offset_of_central_directory as u64))?;

        for n in 0..eocd.central_directory_entries {
            if sf.read_u32le().wrap_err("Failed to read MAGIC_CD")? != MAGIC_CD {
                return Err(eyre!("bad CentralDirectoryEntry magic for entry {n} "));
            }

            sf.read_u16le().wrap_err("can't read version made by")?;
            sf.read_u16le()
                .wrap_err("can't read version needed to extract")?;
            sf.read_u16le()
                .wrap_err("can't read general purpose bit flag")?;
            let comp_method = sf.read_u16le().wrap_err("can't read compression method")?;

            if comp_method != 8 && comp_method != 0 {
                return Err(eyre!("Unsupported compression method {comp_method}"));
            }

            sf.read_u16le().wrap_err("can't read last mod file time")?;
            sf.read_u16le().wrap_err("can't read last mod file date")?;

            let crc32 = sf.read_u32le().wrap_err("can't read crc-32")?;

            let comp_size = sf.read_u32le().wrap_err("can't read compressed size")?;
            let uncomp_size = sf.read_u32le().wrap_err("can't read uncompressed size")?;

            let fname_len = sf.read_u16le().wrap_err("can't read file name length")?;
            if fname_len == 0 {
                return Err(eyre!("Missing file name"));
            }
            let extra_len = sf.read_u16le().wrap_err("can't read extra field length")?;
            let comment_len = sf.read_u16le().wrap_err("can't read file comment length")?;

            let disk_start = sf.read_u16le().wrap_err("can't read disk number start")?;
            if disk_start > 1 {
                return Err(eyre!("Multi-disk unsupported"));
            }
            sf.read_u16le()
                .wrap_err("can't read internal file attributes")?;
            sf.read_u32le()
                .wrap_err("can't read external file attributes")?;

            let offset = sf
                .read_u32le()
                .wrap_err("can't read relative offset of local header")?;

            let file_name_bin = sf.read_as_vec(fname_len as usize)?;
            let name = String::from_utf8(file_name_bin).wrap_err("CD file name #{n} UTF-8")?;

            let extra = if extra_len > 0 {
                Some(sf.read_as_vec(extra_len as usize)?)
            } else {
                None
            };

            let comment = if comment_len == 0 {
                None
            } else {
                let comment_bin = sf.read_as_vec(comment_len as usize)?;
                Some(String::from_utf8(comment_bin).wrap_err("CD comment #{n} not UTF-8")?)
            };

            let cd = CentralDirectoryEntry {
                crc32,
                comp_size,
                uncomp_size,
                offset,
                name,
                extra,
                comment,
            };
            debug!("CD#{n}: {cd:?}");
            entries.push(cd);
        }

        Ok(entries)
    }

    pub fn unpack_file(&mut self, cd: &CentralDirectoryEntry) -> Result<()> {
        let name = &cd.name;
        if name.ends_with('/') {
            debug!("Skipping directory {name}");
            return Ok(());
        }

        let sf = &mut self.file;
        sf.seek(SeekFrom::Start(cd.offset as u64))?;

        if sf.read_u32le().wrap_err("Failed to read MAGIC_LFH")? != MAGIC_LFH {
            return Err(eyre!("bad LocalFileHeader magic for entry {name}"));
        }

        sf.read_u16le()
            .wrap_err("can't read version needed to extract")?;
        sf.read_u16le()
            .wrap_err("can't read general purpose bit flag")?;
        let comp_method = sf.read_u16le().wrap_err("can't read compression method")?;

        if comp_method != 8 && comp_method != 0 {
            return Err(eyre!("Unsupported compression method {comp_method}"));
        }

        sf.read_u16le().wrap_err("can't read last mod file time")?;
        sf.read_u16le().wrap_err("can't read last mod file date")?;

        let crc32 = sf.read_u32le().wrap_err("can't read crc-32")?;

        let comp_size = sf.read_u32le().wrap_err("can't read compressed size")?;
        let uncomp_size = sf.read_u32le().wrap_err("can't read uncompressed size")?;

        let fname_len = sf.read_u16le().wrap_err("can't read file name length")?;
        if fname_len == 0 {
            return Err(eyre!("Missing file name"));
        }
        let extra_len = sf.read_u16le().wrap_err("can't read extra field length")?;

        let file_name_bin = sf.read_as_vec(fname_len as usize)?;
        let name = String::from_utf8(file_name_bin).wrap_err("CD file name #{n} UTF-8")?;

        let extra = if extra_len > 0 {
            Some(sf.read_as_vec(extra_len as usize)?)
        } else {
            None
        };

        let payload = sf.read_as_vec(comp_size as usize)?;

        let uncompressed_data = if comp_method == 8 {
            let mut decompressed = vec![0u8; uncomp_size as usize];
            let reader = Cursor::new(payload);
            let mut d = flate2::read::DeflateDecoder::new(reader);
            d.read_exact(&mut decompressed)?;
            decompressed
        } else {
            payload
        };

        // Normalizacja ścieżki pliku (uniknięcie path traversal)

        let name_path = Path::new(name.as_str());
        let normalized = name_path.normalize();

        let base_dir = self.out_dir.as_str();
        let file_name = normalized
            .file_name()
            .wrap_err("Missing name part in {normalized:?}")?
            .to_str()
            .wrap_err("Name is not UTF8")?;
        let file_dir = normalized
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .wrap_err("Path is not UTF8")?;

        let out_dir = format!("{base_dir}/{file_dir}");
        fs::create_dir_all(out_dir).wrap_err("Failed to create directory {out_dir}")?;

        let out_file = format!("{base_dir}/{file_dir}/{file_name}");
        let mut file = File::create(&out_file).wrap_err("Failed to create file {out_file}")?;
        file.write_exact(&uncompressed_data)
            .wrap_err("Failed to write data to {out_file}")?;

        debug!("Created {out_file}");
        Ok(())
    }
}
