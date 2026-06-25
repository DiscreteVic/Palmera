

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 2;
const VERSION_REVISION: u32 = 0;

pub fn major() -> u32
{
    VERSION_MAJOR
}

pub fn minor() -> u32
{
    VERSION_MINOR
}

pub fn revision() -> u32
{
    VERSION_REVISION
}

pub fn get_string() -> String
{
    format!("{}.{}.{}", major(), minor(), revision())
}
