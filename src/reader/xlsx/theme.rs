use super::XlsxError;
use crate::xml_read_loop;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};
use structs::drawing::Theme;

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    target: &str,
) -> result::Result<Theme, XlsxError> {
    let r = io::BufReader::new(arv.by_name(&format!("xl/{}", target))?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);

    let mut theme: Theme = Theme::default();

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"a:theme" {
                theme.set_attributes(&mut reader, e);
            }
        },
        Event::Eof => break,
    );

    Ok(theme)
}
