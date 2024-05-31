use nix::{self, unistd, sys};
use gimli::{self, EndianReader, LittleEndian, EhFrameHdr, EhFrameEntry, EhFrameIter, EhFramePtr, EhFrameUnwindTable};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use nix::Error as NixError;

#[derive(Debug)]
pub enum Error {
    ErrorOpeningFile,
    DwarfFormatError(gimli::read::Error),
}

pub struct DwarfData {
    // Les champs de votre structure DwarfData
    // ...
}

impl DwarfData {
    pub fn from_file(file_path: &str) -> Result<Self, Error> {
        // Le code pour initialiser DwarfData depuis un fichier
        // ...
    }

    pub fn print(&self) {
        // Le code pour afficher les informations de DwarfData
        // ...
    }

    pub fn get_function_and_line_from_addr(&self, addr: u64) -> (Option<&str>, Option<&str>, Option<u64>) {
        // Le code pour obtenir le nom de la fonction et le numéro de ligne à partir de l'adresse
        // ...
    }

    pub fn get_frame_start_address(&self, addr: u64, frame_bottom: u64) -> Option<u64> {
        // Le code pour obtenir l'adresse de début du cadre de pile
        // ...
    }

    pub fn get_function_from_addr(&self, addr: u64) -> Option<&str> {
        // Le code pour obtenir le nom de la fonction à partir de l'adresse
        // ...
    }

    pub fn get_line_from_addr(&self, addr: u64) -> Option<(&str, u64)> {
        // Le code pour obtenir le nom du fichier et le numéro de ligne à partir de l'adresse
        // ...
    }
}